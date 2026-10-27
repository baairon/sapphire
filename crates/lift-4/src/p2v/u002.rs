//! Mechanically lifted functions (scripts/lift_run.py), module u002. Each one is checked against the original
//! in the world harness (tests/world_u002.rs), except f_00030040, which never matched.
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x31ccc size=1046
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_00031ccc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x31ccc;
    loop {
        match pc {
        0x31ccc => {
            // block 31ccc
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31ccc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31ccc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 31ccd mov rbp,rsp
            rbp = rsp;
            // 31cd0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 31cd2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31cd4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 31cd6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 31cd8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31cd9 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 31ce0 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 31ce3 mov r15,rsi
            r15 = rsi;
            // 31ce6 mov r13,rdi
            r13 = rdi;
            // 31ce9 mov r14,QWORD PTR [rip+0x10e6e8] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // 31cf0 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // 31cf3 mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 31cf7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 31cfa mov rdi,r15
            rdi = r15;
            // 31cfd call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31d03 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31d05 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31d07 je 0x3208c
            if fl.e() { pc = 0x3208c; continue; }
            pc = 0x31d0d;
        }
        0x31d0d => {
            // block 31d0d
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31d0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31d0d mov QWORD PTR [rbp-0xc0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (r13) as u64);
            // 31d14 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 31d17 lea rcx,[rip+0xce142] # 0xffe60
            rcx = m.env.data_va(83501);
            // 31d1e mov r14d,DWORD PTR [rcx+rax*4]
            r14 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 31d22 lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 31d29 mov QWORD PTR [rbp-0x88],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0x0u64) as u64);
            // 31d34 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 31d3f mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 31d46 lea r12,[rbp-0x98]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 31d4d lea r13,[rbp-0x9c]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff64);
            pc = 0x31d54;
        }
        0x31d54 => {
            // block 31d54
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31d54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31d54 lea eax,[r14+rbx*1]
            rax = (r14.wrapping_add(rbx)) as u32 as u64;
            // 31d58 mov DWORD PTR [rbp-0x9c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff64), ((rax as u32 as u64)) as u32);
            // 31d5e mov rdi,r12
            rdi = r12;
            // 31d61 mov rsi,r13
            rsi = r13;
            // 31d64 call 0xebb22
            { let r = { let t = m.env.stub_va(0x2ef); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31d69 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 31d6b cmp ebx,0x11
            fl = sub::<32>((rbx as u32 as u64), 0x11u64).1;
            // 31d6e jl 0x31d54
            if fl.l() { pc = 0x31d54; continue; }
            pc = 0x31d70;
        }
        0x31d70 => {
            // block 31d70
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31d70 lea rbx,[rbp-0xb0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31d77 mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 31d82 mov QWORD PTR [rbp-0xb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u64);
            // 31d8d mov QWORD PTR [rbp-0xb8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rbx) as u64);
            // 31d94 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 31d97 mov rax,QWORD PTR [rax+0xc90]
            rax = m.r64(rax.wrapping_add(0xc90)) as u64;
            // 31d9e lea rsi,[rbp-0x98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 31da5 lea rdx,[rbp-0xb8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 31dac mov rdi,r15
            rdi = r15;
            // 31daf call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31db1 mov r14,QWORD PTR [rip+0x10e620] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // 31db8 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // 31dbb mov r15,QWORD PTR [rbp-0xc0]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // 31dc2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31dc4 je 0x31e4f
            if fl.e() { pc = 0x31e4f; continue; }
            pc = 0x31dca;
        }
        0x31dca => {
            // block 31dca
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dca cmp QWORD PTR [rbp-0xa8],0x11
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, 0x11u64).1;
            // 31dd2 jne 0x31e4f
            if fl.ne() { pc = 0x31e4f; continue; }
            pc = 0x31dd4;
        }
        0x31dd4 => {
            // block 31dd4
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dd4 mov rcx,QWORD PTR [rbp-0xb8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // 31ddb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31ddd cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // 31de0 je 0x31e56
            if fl.e() { pc = 0x31e56; continue; }
            pc = 0x31de2;
        }
        0x31de2 => {
            // block 31de2
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31de2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31de2 mov edx,DWORD PTR [rcx+0x20]
            rdx = (m.r32(rcx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 31de5 mov DWORD PTR [rbp+rax*4-0x80],edx
            m.w32(rbp.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff80), ((rdx as u32 as u64)) as u32);
            // 31de9 mov rdx,QWORD PTR [rcx+0x8]
            rdx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 31ded test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 31df0 je 0x31dff
            if fl.e() { pc = 0x31dff; continue; }
            pc = 0x31df2;
        }
        0x31df2 => {
            // block 31df2
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31df2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31df2 mov rcx,rdx
            rcx = rdx;
            // 31df5 mov rdx,QWORD PTR [rcx]
            rdx = m.r64(rcx) as u64;
            // 31df8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 31dfb jne 0x31df2
            if fl.ne() { pc = 0x31df2; continue; }
            pc = 0x31dfd;
        }
        0x31dfd => {
            // block 31dfd
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dfd jmp 0x31e0e
            pc = 0x31e0e; continue;
        }
        0x31dff => {
            // block 31dff
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31dff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31dff mov rdx,rcx
            rdx = rcx;
            pc = 0x31e02;
        }
        0x31e02 => {
            // block 31e02
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e02 mov rcx,QWORD PTR [rdx+0x10]
            rcx = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 31e06 cmp QWORD PTR [rcx],rdx
            fl = sub::<64>(m.r64(rcx) as u64, rdx).1;
            // 31e09 mov rdx,rcx
            rdx = rcx;
            // 31e0c jne 0x31e02
            if fl.ne() { pc = 0x31e02; continue; }
            pc = 0x31e0e;
        }
        0x31e0e => {
            // block 31e0e
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e0e inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 31e11 cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // 31e14 jne 0x31de2
            if fl.ne() { pc = 0x31de2; continue; }
            pc = 0x31e16;
        }
        0x31e16 => {
            // block 31e16
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e16 mov ecx,DWORD PTR [rbp-0x80]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64) as u32 as u64;
            // 31e19 mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            // 31e1c mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 31e1e shr edx,0x4
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x4u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 31e21 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 31e23 shr esi,0xc
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xcu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 31e26 movd xmm0,ecx
            x0 = ((rcx as u32 as u64)) as u32 as u128;
            // 31e2a shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 31e2d movd xmm1,ecx
            x1 = ((rcx as u32 as u64)) as u32 as u128;
            // 31e31 movd xmm2,edx
            x2 = ((rdx as u32 as u64)) as u32 as u128;
            // 31e35 punpckldq xmm2,xmm1
            { let a = x2; let b = x1; x2 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e39 movd xmm1,esi
            x1 = ((rsi as u32 as u64)) as u32 as u128;
            // 31e3d punpckldq xmm0,xmm1
            { let a = x0; let b = x1; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e41 punpckldq xmm0,xmm2
            { let a = x0; let b = x2; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e45 pand xmm0,XMMWORD PTR [rip+0xce003] # 0xffe50
            x0 &= m.r128(m.env.data_va(83485));
            // 31e4d jmp 0x31e5d
            pc = 0x31e5d; continue;
        }
        0x31e4f => {
            // block 31e4f
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e4f xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31e51 jmp 0x32066
            pc = 0x32066; continue;
        }
        0x31e56 => {
            // block 31e56
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e56 pxor xmm0,xmm0
            x0 = 0;
            // 31e5a mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            pc = 0x31e5d;
        }
        0x31e5d => {
            // block 31e5d
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 31e5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31e5d movdqu XMMWORD PTR [r15+0x8],xmm0
            m.w128(r15.wrapping_add(0x8), x0);
            // 31e63 lea rbx,[r15+0x30]
            rbx = r15.wrapping_add(0x30);
            // 31e67 mov rcx,QWORD PTR [r15+0x30]
            rcx = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31e6b mov rcx,QWORD PTR [rcx+0x30]
            rcx = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 31e6f movsx edx,ax
            rdx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // 31e72 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 31e74 mov rdi,rbx
            rdi = rbx;
            // 31e77 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31e79 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31e7c mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31e80 movsx edx,WORD PTR [rbp-0x7a]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff86)) as u64) as u64) as u32 as u64;
            // 31e84 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31e89 mov rdi,rbx
            rdi = rbx;
            // 31e8c call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31e8e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31e91 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31e95 movsx edx,WORD PTR [rbp-0x78]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u64) as u32 as u64;
            // 31e99 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31e9e mov rdi,rbx
            rdi = rbx;
            // 31ea1 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ea3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ea6 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31eaa mov ecx,DWORD PTR [rbp-0x74]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c)) as u64) as u32 as u64;
            // 31ead cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31eb2 mulsd xmm0,QWORD PTR [rip+0xcdf86] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31eba xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 31ebc mov rdi,rbx
            rdi = rbx;
            // 31ebf call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ec1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ec4 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31ec8 mov ecx,DWORD PTR [rbp-0x70]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // 31ecb xorps xmm0,xmm0
            x0 = 0;
            // 31ece cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31ed3 mulsd xmm0,QWORD PTR [rip+0xcdf65] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31edb mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31ee0 mov rdi,rbx
            rdi = rbx;
            // 31ee3 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ee5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ee8 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31eec mov ecx,DWORD PTR [rbp-0x6c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // 31eef xorps xmm0,xmm0
            x0 = 0;
            // 31ef2 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31ef7 mulsd xmm0,QWORD PTR [rip+0xcdf41] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31eff mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // 31f04 mov rdi,rbx
            rdi = rbx;
            // 31f07 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f09 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f0c mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f10 mov ecx,DWORD PTR [rbp-0x68]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // 31f13 xorps xmm0,xmm0
            x0 = 0;
            // 31f16 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f1b mulsd xmm0,QWORD PTR [rip+0xcdf1d] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f23 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31f28 mov rdi,rbx
            rdi = rbx;
            // 31f2b call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f2d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f30 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f34 mov ecx,DWORD PTR [rbp-0x64]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 31f37 xorps xmm0,xmm0
            x0 = 0;
            // 31f3a cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f3f mulsd xmm0,QWORD PTR [rip+0xcdef9] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f47 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 31f4c mov rdi,rbx
            rdi = rbx;
            // 31f4f call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f51 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f54 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f58 mov ecx,DWORD PTR [rbp-0x60]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 31f5b xorps xmm0,xmm0
            x0 = 0;
            // 31f5e cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f63 mulsd xmm0,QWORD PTR [rip+0xcded5] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f6b mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 31f70 mov rdi,rbx
            rdi = rbx;
            // 31f73 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f75 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f78 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f7c mov ecx,DWORD PTR [rbp-0x5c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // 31f7f xorps xmm0,xmm0
            x0 = 0;
            // 31f82 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f87 mulsd xmm0,QWORD PTR [rip+0xcdeb1] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f8f mov esi,0x6
            rsi = (0x6u64) as u32 as u64;
            // 31f94 mov rdi,rbx
            rdi = rbx;
            // 31f97 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f99 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f9c mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31fa0 mov ecx,DWORD PTR [rbp-0x58]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // 31fa3 xorps xmm0,xmm0
            x0 = 0;
            // 31fa6 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31fab mulsd xmm0,QWORD PTR [rip+0xcde8d] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31fb3 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 31fb8 mov rdi,rbx
            rdi = rbx;
            // 31fbb call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31fbd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31fc0 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31fc4 mov ecx,DWORD PTR [rbp-0x54]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64) as u32 as u64;
            // 31fc7 xorps xmm0,xmm0
            x0 = 0;
            // 31fca cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31fcf mulsd xmm0,QWORD PTR [rip+0xcde69] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31fd7 mov esi,0x8
            rsi = (0x8u64) as u32 as u64;
            // 31fdc mov rdi,rbx
            rdi = rbx;
            // 31fdf call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31fe1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31fe4 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31fe8 movsx edx,WORD PTR [rbp-0x50]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64) as u64) as u32 as u64;
            // 31fec mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31ff1 mov rdi,rbx
            rdi = rbx;
            // 31ff4 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ff6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ff9 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31ffd movsx edx,WORD PTR [rbp-0x4e]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb2)) as u64) as u64) as u32 as u64;
            // 32001 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 32006 mov rdi,rbx
            rdi = rbx;
            // 32009 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 3200b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3200e mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 32012 movsx edx,WORD PTR [rbp-0x4c]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u64) as u32 as u64;
            // 32016 mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 3201b mov rdi,rbx
            rdi = rbx;
            // 3201e call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 32020 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 32024 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 32026 shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 32029 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3202c mov DWORD PTR [r15+0x1c],ecx
            m.w32(r15.wrapping_add(0x1c), ((rcx as u32 as u64)) as u32);
            // 32030 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 32032 and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 32035 mov DWORD PTR [r15+0x18],ecx
            m.w32(r15.wrapping_add(0x18), ((rcx as u32 as u64)) as u32);
            // 32039 mov rcx,rax
            rcx = rax;
            // 3203c shr rcx,0x20
            { let (r, f) = shr::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // 32040 mov WORD PTR [r15+0x20],cx
            m.w16(r15.wrapping_add(0x20), ((rcx as u16 as u64)) as u16);
            // 32045 shr rax,0x30
            { let (r, f) = shr::<64>(rax, 0x30u64, fl); rax = r; fl = f; }
            // 32049 mov WORD PTR [r15+0x22],ax
            m.w16(r15.wrapping_add(0x22), ((rax as u16 as u64)) as u16);
            // 3204e mov eax,DWORD PTR [rbp-0x40]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 32051 cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 32056 mulsd xmm0,QWORD PTR [rip+0xcddea] # 0xffe48
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83477)) as u128))).to_bits());
            // 3205e movsd QWORD PTR [r15+0x28],xmm0
            m.w64(r15.wrapping_add(0x28), x0 as u64);
            // 32064 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x32066;
        }
        0x32066 => {
            // block 32066
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 32066 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32066 mov rsi,QWORD PTR [rbp-0xb0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 3206d lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 32074 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 32079 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 32080 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 32087 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            pc = 0x3208c;
        }
        0x3208c => {
            // block 3208c
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 3208c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3208c cmp r14,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r14, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 32090 jne 0x320dc
            if fl.ne() { pc = 0x320dc; continue; }
            pc = 0x32092;
        }
        0x32092 => {
            // block 32092
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 32092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32092 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 32094 add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 3209b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3209c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 3209e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 320a0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 320a2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 320a4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 320a5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x320a6 => {
            // block 320a6
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 320a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 320a6 mov rbx,rax
            rbx = rax;
            pc = 0x320a9;
        }
        0x320a9 => {
            // block 320a9
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 320a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 320a9 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 320b0 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 320b7 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 320bc mov rdi,rbx
            rdi = rbx;
            // 320bf call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 320c4 mov rbx,rax
            rbx = rax;
            // 320c7 mov rsi,QWORD PTR [rbp-0xb0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 320ce lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 320d5 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 320da jmp 0x320a9
            pc = 0x320a9; continue;
        }
        0x320dc => {
            // block 320dc
            #[cfg(feature = "trace")] eprintln!("f_00031ccc pc 320dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 320dc call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe31d0 size=1196
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e31d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe31d0;
    loop {
        match pc {
        0xe31d0 => {
            // block e31d0
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e31d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e31d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e31d1 mov rbp,rsp
            rbp = rsp;
            // e31d4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e31d6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e31d8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e31da push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e31dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e31dd sub rsp,0xf8
            { let (r, f) = sub::<64>(rsp, 0xf8u64);
              rsp = r;
              fl = f; }
            // e31e4 mov rbx,rdx
            rbx = rdx;
            // e31e7 mov QWORD PTR [rbp-0x118],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rsi) as u64);
            // e31ee mov r14,rdi
            r14 = rdi;
            // e31f1 call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e31f6 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // e31f9 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e31fb je 0xe35bb
            if fl.e() { pc = 0xe35bb; continue; }
            pc = 0xe3201;
        }
        0xe3201 => {
            // block e3201
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3201 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3201 add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // e3205 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e3209 mov rsi,rbx
            rsi = rbx;
            // e320c call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3211 lea rsi,[rip+0x4754b] # 0x12a763
            rsi = m.env.data_va(257840);
            // e3218 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e321c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // e3221 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3226 mov QWORD PTR [rbp-0x120],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (r14) as u64);
            // e322d lea rsi,[rip+0x5626f] # 0x1394a3
            rsi = m.env.data_va(318576);
            // e3234 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // e3238 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // e323d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3242 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e3245 jmp 0xe324a
            pc = 0xe324a; continue;
        }
        0xe3247 => {
            // block e3247
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3247 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3247 add r14,rax
            { let (r, f) = add::<64>(r14, rax);
              r14 = r;
              fl = f; }
            pc = 0xe324a;
        }
        0xe324a => {
            // block e324a
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e324a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e324a movzx r8d,BYTE PTR [rbp-0x40]
            r8 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // e324f mov r9b,r8b
            r9 = (r9 & !0xff) | (((r8 as u8 as u64)) as u8 as u64);
            // e3252 and r9b,0x1
            { let r = (r9 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r9 = (r9 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e3256 jne 0xe3264
            if fl.ne() { pc = 0xe3264; continue; }
            pc = 0xe3258;
        }
        0xe3258 => {
            // block e3258
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3258 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3258 mov rsi,r8
            rsi = r8;
            // e325b shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // e325e lea r10,[rbp-0x3f]
            r10 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // e3262 jmp 0xe326c
            pc = 0xe326c; continue;
        }
        0xe3264 => {
            // block e3264
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3264 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3264 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e3268 mov r10,QWORD PTR [rbp-0x30]
            r10 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xe326c;
        }
        0xe326c => {
            // block e326c
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e326c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e326c movzx edx,BYTE PTR [rbp-0x58]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // e3270 mov r11b,dl
            r11 = (r11 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // e3273 and r11b,0x1
            { let r = (r11 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r11 = (r11 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e3277 jne 0xe3285
            if fl.ne() { pc = 0xe3285; continue; }
            pc = 0xe3279;
        }
        0xe3279 => {
            // block e3279
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3279 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3279 mov rdi,rdx
            rdi = rdx;
            // e327c shr rdi,1
            { let (r, f) = shr::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // e327f lea rax,[rbp-0x57]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa9);
            // e3283 jmp 0xe328d
            pc = 0xe328d; continue;
        }
        0xe3285 => {
            // block e3285
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3285 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3285 mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // e3289 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xe328d;
        }
        0xe328d => {
            // block e328d
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e328d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e328d mov rcx,rsi
            rcx = rsi;
            // e3290 sub rcx,r14
            { let (r, f) = sub::<64>(rcx, r14);
              rcx = r;
              fl = f; }
            // e3293 jb 0xe3369
            if fl.b() { pc = 0xe3369; continue; }
            pc = 0xe3299;
        }
        0xe3299 => {
            // block e3299
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3299 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3299 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // e329c jb 0xe3369
            if fl.b() { pc = 0xe3369; continue; }
            pc = 0xe32a2;
        }
        0xe32a2 => {
            // block e32a2
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e32a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e32a2 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e32a5 je 0xe3314
            if fl.e() { pc = 0xe3314; continue; }
            pc = 0xe32a7;
        }
        0xe32a7 => {
            // block e32a7
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e32a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e32a7 lea rbx,[r10+r14*1]
            rbx = r10.wrapping_add(r14);
            // e32ab lea r12,[r10+rsi*1]
            r12 = r10.wrapping_add(rsi);
            // e32af mov rcx,r12
            rcx = r12;
            // e32b2 sub rcx,rbx
            { let (r, f) = sub::<64>(rcx, rbx);
              rcx = r;
              fl = f; }
            // e32b5 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // e32b8 jl 0xe3369
            if fl.l() { pc = 0xe3369; continue; }
            pc = 0xe32be;
        }
        0xe32be => {
            // block e32be
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e32be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e32be mov r15d,0x1
            r15 = (0x1u64) as u32 as u64;
            // e32c4 sub r15,rdi
            { let (r, f) = sub::<64>(r15, rdi);
              r15 = r;
              fl = f; }
            // e32c7 add r15,rsi
            { let (r, f) = add::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // e32ca cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // e32cd je 0xe3369
            if fl.e() { pc = 0xe3369; continue; }
            pc = 0xe32d3;
        }
        0xe32d3 => {
            // block e32d3
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e32d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e32d3 add r15,r10
            { let (r, f) = add::<64>(r15, r10);
              r15 = r;
              fl = f; }
            // e32d6 mov r14b,BYTE PTR [rax]
            r14 = (r14 & !0xff) | ((m.r8(rax) as u64) as u8 as u64);
            pc = 0xe32d9;
        }
        0xe32d9 => {
            // block e32d9
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e32d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e32d9 movzx esi,r14b
            rsi = ((r14 as u8 as u64)) as u32 as u64;
            // e32dd movzx ecx,BYTE PTR [rbx]
            rcx = (m.r8(rbx) as u64) as u32 as u64;
            // e32e0 cmp ecx,esi
            fl = sub::<32>((rcx as u32 as u64), (rsi as u32 as u64)).1;
            // e32e2 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e32e7 jne 0xe32ff
            if fl.ne() { pc = 0xe32ff; continue; }
            pc = 0xe32e9;
        }
        0xe32e9 => {
            // block e32e9
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e32e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e32e9 cmp rdi,rsi
            fl = sub::<64>(rdi, rsi).1;
            // e32ec je 0xe3309
            if fl.e() { pc = 0xe3309; continue; }
            pc = 0xe32ee;
        }
        0xe32ee => {
            // block e32ee
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e32ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e32ee movzx ecx,BYTE PTR [rax+rsi*1]
            rcx = (m.r8(rax.wrapping_add(rsi)) as u64) as u32 as u64;
            // e32f2 movzx r13d,BYTE PTR [rbx+rsi*1]
            r13 = (m.r8(rbx.wrapping_add(rsi)) as u64) as u32 as u64;
            // e32f7 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // e32fa cmp r13d,ecx
            fl = sub::<32>((r13 as u32 as u64), (rcx as u32 as u64)).1;
            // e32fd je 0xe32e9
            if fl.e() { pc = 0xe32e9; continue; }
            pc = 0xe32ff;
        }
        0xe32ff => {
            // block e32ff
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e32ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e32ff inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e3302 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // e3305 jne 0xe32d9
            if fl.ne() { pc = 0xe32d9; continue; }
            pc = 0xe3307;
        }
        0xe3307 => {
            // block e3307
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3307 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3307 jmp 0xe3369
            pc = 0xe3369; continue;
        }
        0xe3309 => {
            // block e3309
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3309 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3309 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // e330c je 0xe3369
            if fl.e() { pc = 0xe3369; continue; }
            pc = 0xe330e;
        }
        0xe330e => {
            // block e330e
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e330e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e330e sub rbx,r10
            { let (r, f) = sub::<64>(rbx, r10);
              rbx = r;
              fl = f; }
            // e3311 mov r14,rbx
            r14 = rbx;
            pc = 0xe3314;
        }
        0xe3314 => {
            // block e3314
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3314 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3314 cmp r14,0xffffffffffffffff
            fl = sub::<64>(r14, 0xffff_ffff_ffff_ffffu64).1;
            // e3318 je 0xe3369
            if fl.e() { pc = 0xe3369; continue; }
            pc = 0xe331a;
        }
        0xe331a => {
            // block e331a
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e331a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e331a test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // e331d jne 0xe3324
            if fl.ne() { pc = 0xe3324; continue; }
            pc = 0xe331f;
        }
        0xe331f => {
            // block e331f
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e331f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e331f shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e3322 jmp 0xe3328
            pc = 0xe3328; continue;
        }
        0xe3324 => {
            // block e3324
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3324 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3324 mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0xe3328;
        }
        0xe3328 => {
            // block e3328
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3328 movzx r8d,BYTE PTR [rbp-0x70]
            r8 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // e332d test r8b,0x1
            fl = logic::<8>((r8 as u8 as u64) & 0x1u64);
            // e3331 jne 0xe333c
            if fl.ne() { pc = 0xe333c; continue; }
            pc = 0xe3333;
        }
        0xe3333 => {
            // block e3333
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3333 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3333 shr r8,1
            { let (r, f) = shr::<64>(r8, 0x1u64, fl); r8 = r; fl = f; }
            // e3336 lea rcx,[rbp-0x6f]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff91);
            // e333a jmp 0xe3344
            pc = 0xe3344; continue;
        }
        0xe333c => {
            // block e333c
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e333c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e333c mov r8,QWORD PTR [rbp-0x68]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e3340 mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            pc = 0xe3344;
        }
        0xe3344 => {
            // block e3344
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3344 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3344 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e3348 mov rsi,r14
            rsi = r14;
            // e334b call 0xec00e
            { let r = { let t = m.env.stub_va(0x7db); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3350 movzx eax,BYTE PTR [rbp-0x70]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // e3354 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e3356 jne 0xe3360
            if fl.ne() { pc = 0xe3360; continue; }
            pc = 0xe3358;
        }
        0xe3358 => {
            // block e3358
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3358 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3358 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // e335b jmp 0xe3247
            pc = 0xe3247; continue;
        }
        0xe3360 => {
            // block e3360
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3360 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3360 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e3364 jmp 0xe3247
            pc = 0xe3247; continue;
        }
        0xe3369 => {
            // block e3369
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3369 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3369 test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // e336c jne 0xe3373
            if fl.ne() { pc = 0xe3373; continue; }
            pc = 0xe336e;
        }
        0xe336e => {
            // block e336e
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e336e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e336e shr r8,1
            { let (r, f) = shr::<64>(r8, 0x1u64, fl); r8 = r; fl = f; }
            // e3371 jmp 0xe3377
            pc = 0xe3377; continue;
        }
        0xe3373 => {
            // block e3373
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3373 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3373 mov r8,QWORD PTR [rbp-0x38]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0xe3377;
        }
        0xe3377 => {
            // block e3377
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3377 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3377 mov rbx,QWORD PTR [rbp-0x120]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // e337e cmp r8d,0xf80
            fl = sub::<32>((r8 as u32 as u64), 0xf80u64).1;
            // e3385 jl 0xe3393
            if fl.l() { pc = 0xe3393; continue; }
            pc = 0xe3387;
        }
        0xe3387 => {
            // block e3387
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3387 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3387 mov DWORD PTR [rbx+0x8],0x1c
            m.w32(rbx.wrapping_add(0x8), (0x1cu64) as u32);
            // e338e jmp 0xe356d
            pc = 0xe356d; continue;
        }
        0xe3393 => {
            // block e3393
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3393 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3393 lea rdi,[rbx+0x34]
            rdi = rbx.wrapping_add(0x34);
            // e3397 mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // e339e test BYTE PTR [rcx],0x1
            fl = logic::<8>(m.r8(rcx) as u64 & 0x1u64);
            // e33a1 jne 0xe33a8
            if fl.ne() { pc = 0xe33a8; continue; }
            pc = 0xe33a3;
        }
        0xe33a3 => {
            // block e33a3
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e33a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e33a3 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // e33a6 jmp 0xe33ac
            pc = 0xe33ac; continue;
        }
        0xe33a8 => {
            // block e33a8
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e33a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e33a8 mov rcx,QWORD PTR [rcx+0x10]
            rcx = m.r64(rcx.wrapping_add(0x10)) as u64;
            pc = 0xe33ac;
        }
        0xe33ac => {
            // block e33ac
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e33ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e33ac test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // e33af lea r8,[rbp-0x3f]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // e33b3 cmovne r8,QWORD PTR [rbp-0x30]
            if fl.ne() { r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // e33b8 lea rsi,[rip+0x560ec] # 0x1394ab
            rsi = m.env.data_va(318584);
            // e33bf mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // e33c4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e33c6 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e33cb mov esi,0xfa
            rsi = (0xfau64) as u32 as u64;
            // e33d0 mov rdi,rbx
            rdi = rbx;
            // e33d3 call 0xe435c
            { let r = { let t = m.env.code_va(0xe435c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e33d8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e33da je 0xe3566
            if fl.e() { pc = 0xe3566; continue; }
            pc = 0xe33e0;
        }
        0xe33e0 => {
            // block e33e0
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e33e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e33e0 xorps xmm0,xmm0
            x0 = 0;
            // e33e3 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // e33ea mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // e33f2 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e33f9 mov rdi,rbx
            rdi = rbx;
            // e33fc call 0xe4a2e
            { let r = { let t = m.env.code_va(0xe4a2e); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3401 xorps xmm0,xmm0
            x0 = 0;
            // e3404 movaps XMMWORD PTR [rbp-0xb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff50), x0);
            // e340b mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // e3416 lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // e341d lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e3424 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3429 lea rsi,[rbp-0xc8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // e3430 lea rdx,[rbp-0xb0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e3437 call 0xe171e
            { let r = { let t = m.env.code_va(0xe171e); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e343c lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // e3443 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3448 mov r14,QWORD PTR [rbp-0xb0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // e344f cmp QWORD PTR [rbp-0xa8],r14
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, r14).1;
            // e3456 je 0xe357e
            if fl.e() { pc = 0xe357e; continue; }
            pc = 0xe345c;
        }
        0xe345c => {
            // block e345c
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e345c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e345c xorps xmm0,xmm0
            x0 = 0;
            // e345f movaps XMMWORD PTR [rbp-0xe0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff20), x0);
            // e3466 mov QWORD PTR [rbp-0xd0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (0x0u64) as u64);
            // e3471 lea rsi,[rip+0x55f9b] # 0x139413
            rsi = m.env.data_va(318432);
            // e3478 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // e347f mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // e3484 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3489 lea rdx,[rbp-0xf8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // e3490 lea rcx,[rbp-0xe0]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e3497 mov rsi,r14
            rsi = r14;
            // e349a call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e349f mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e34a2 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // e34a9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e34ae test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // e34b1 je 0xe3572
            if fl.e() { pc = 0xe3572; continue; }
            pc = 0xe34b7;
        }
        0xe34b7 => {
            // block e34b7
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e34b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e34b7 lea rsi,[rip+0x55f5c] # 0x13941a
            rsi = m.env.data_va(318439);
            // e34be lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e34c5 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e34ca test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e34cc je 0xe35d0
            if fl.e() { pc = 0xe35d0; continue; }
            pc = 0xe34d2;
        }
        0xe34d2 => {
            // block e34d2
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e34d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e34d2 lea rsi,[rip+0x55f44] # 0x13941d
            rsi = m.env.data_va(318442);
            // e34d9 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e34e0 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e34e5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e34e7 jne 0xe3572
            if fl.ne() { pc = 0xe3572; continue; }
            pc = 0xe34ed;
        }
        0xe34ed => {
            // block e34ed
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e34ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e34ed mov r14,QWORD PTR [rbp-0xb0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // e34f4 mov rax,QWORD PTR [rbp-0xa8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // e34fb sub rax,r14
            { let (r, f) = sub::<64>(rax, r14);
              rax = r;
              fl = f; }
            // e34fe sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e3502 movabs rcx,0xaaaaaaaaaaaaaaab
            rcx = 0xaaaa_aaaa_aaaa_aaabu64;
            // e350c imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e3510 cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // e3514 jb 0xe3572
            if fl.b() { pc = 0xe3572; continue; }
            pc = 0xe3516;
        }
        0xe3516 => {
            // block e3516
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3516 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3516 lea rsi,[rip+0x55f05] # 0x139422
            rsi = m.env.data_va(318447);
            // e351d lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // e3524 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e3529 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e352e add r14,0x30
            { let (r, f) = add::<64>(r14, 0x30u64);
              r14 = r;
              fl = f; }
            // e3532 lea rcx,[rbx+0x10]
            rcx = rbx.wrapping_add(0x10);
            // e3536 lea rdx,[rbp-0x110]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // e353d mov rsi,r14
            rsi = r14;
            // e3540 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3545 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // e354c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3551 mov DWORD PTR [rbx+0x8],0x1a
            m.w32(rbx.wrapping_add(0x8), (0x1au64) as u32);
            // e3558 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e355f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3564 jmp 0xe3585
            pc = 0xe3585; continue;
        }
        0xe3566 => {
            // block e3566
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3566 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3566 mov DWORD PTR [rbx+0x8],0x15
            m.w32(rbx.wrapping_add(0x8), (0x15u64) as u32);
            pc = 0xe356d;
        }
        0xe356d => {
            // block e356d
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e356d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e356d xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e3570 jmp 0xe35a0
            pc = 0xe35a0; continue;
        }
        0xe3572 => {
            // block e3572
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3572 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3572 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e3579 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe357e;
        }
        0xe357e => {
            // block e357e
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e357e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e357e mov DWORD PTR [rbx+0x8],0x17
            m.w32(rbx.wrapping_add(0x8), (0x17u64) as u32);
            pc = 0xe3585;
        }
        0xe3585 => {
            // block e3585
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3585 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe3588;
        }
        0xe3588 => {
            // block e3588
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3588 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3588 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e358f call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3594 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e359b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe35a0;
        }
        0xe35a0 => {
            // block e35a0
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e35a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e35a0 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // e35a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e35a9 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e35ad call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e35b2 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e35b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe35bb;
        }
        0xe35bb => {
            // block e35bb
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e35bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e35bb mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e35be add rsp,0xf8
            { let (r, f) = add::<64>(rsp, 0xf8u64);
              rsp = r;
              fl = f; }
            // e35c5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e35c6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e35c8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e35ca pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e35cc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e35ce pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e35cf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe35d0 => {
            // block e35d0
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e35d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e35d0 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e35d7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e35dc mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // e35df jmp 0xe3588
            pc = 0xe3588; continue;
        }
        0xe35e1 => {
            // block e35e1
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e35e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e35e1 mov rbx,rax
            rbx = rax;
            pc = 0xe35e4;
        }
        0xe35e4 => {
            // block e35e4
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e35e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e35e4 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // e35e8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe35ed;
        }
        0xe35ed => {
            // block e35ed
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e35ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e35ed lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e35f1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe35f6;
        }
        0xe35f6 => {
            // block e35f6
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e35f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e35f6 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e35fa call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e35ff mov rdi,rbx
            rdi = rbx;
            // e3602 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3607 mov rbx,rax
            rbx = rax;
            // e360a jmp 0xe35f6
            pc = 0xe35f6; continue;
        }
        0xe360c => {
            // block e360c
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e360c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e360c mov rbx,rax
            rbx = rax;
            // e360f jmp 0xe35ed
            pc = 0xe35ed; continue;
        }
        0xe3611 => {
            // block e3611
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3611 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3611 jmp 0xe35e1
            pc = 0xe35e1; continue;
        }
        0xe3613 => {
            // block e3613
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3613 mov rbx,rax
            rbx = rax;
            // e3616 jmp 0xe365a
            pc = 0xe365a; continue;
        }
        0xe3618 => {
            // block e3618
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3618 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3618 mov rbx,rax
            rbx = rax;
            // e361b jmp 0xe364e
            pc = 0xe364e; continue;
        }
        0xe361d => {
            // block e361d
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e361d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e361d mov rbx,rax
            rbx = rax;
            // e3620 lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // e3627 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e362c jmp 0xe364e
            pc = 0xe364e; continue;
        }
        0xe362e => {
            // block e362e
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e362e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e362e mov rbx,rax
            rbx = rax;
            // e3631 jmp 0xe3642
            pc = 0xe3642; continue;
        }
        0xe3633 => {
            // block e3633
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3633 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3633 mov rbx,rax
            rbx = rax;
            // e3636 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // e363d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe3642;
        }
        0xe3642 => {
            // block e3642
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e3642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3642 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // e3649 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe364e;
        }
        0xe364e => {
            // block e364e
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e364e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e364e lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e3655 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe365a;
        }
        0xe365a => {
            // block e365a
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e365a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e365a lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e3661 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3666 jmp 0xe35e4
            pc = 0xe35e4; continue;
        }
        0xe366b => {
            // block e366b
            #[cfg(feature = "trace")] eprintln!("f_000e31d0 pc e366b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e366b mov rbx,rax
            rbx = rax;
            // e366e lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // e3675 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e367a jmp 0xe3642
            pc = 0xe3642; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f84c size=1220
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f84c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2f84c;
    loop {
        match pc {
        0x2f84c => {
            // block 2f84c
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2f84c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f84c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f84d mov rbp,rsp
            rbp = rsp;
            // 2f850 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2f852 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f854 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2f856 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2f858 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f859 sub rsp,0x108
            { let (r, f) = sub::<64>(rsp, 0x108u64);
              rsp = r;
              fl = f; }
            // 2f860 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // 2f863 mov QWORD PTR [rbp-0xf0],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rdx) as u64);
            // 2f86a mov QWORD PTR [rbp-0xe8],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rsi) as u64);
            // 2f871 mov rbx,rdi
            rbx = rdi;
            // 2f874 mov QWORD PTR [rbp-0xe0],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rbx) as u64);
            // 2f87b cmp esi,0xfffff
            fl = sub::<32>((rsi as u32 as u64), 0xfffffu64).1;
            // 2f881 seta al
            rax = (rax & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // 2f884 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 2f887 lea r15d,[rax+rax*2+0x8]
            r15 = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x8)) as u32 as u64;
            // 2f88c mov DWORD PTR [rbp-0xf4],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff0c), ((r15 as u32 as u64)) as u32);
            // 2f893 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 2f896 cvtsi2sd xmm1,rax
            x1 = set_lo64(x1, (sx::<64>(rax) as f64).to_bits());
            // 2f89b movsd QWORD PTR [rbp-0x120],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), x1 as u64);
            // 2f8a3 movsd xmm0,QWORD PTR [rip+0xd0525] # 0xffdd0
            x0 = m.r64(m.env.data_va(83357)) as u128;
            // 2f8ab mulsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) * f64_of(x1)).to_bits());
            // 2f8af call 0xec1b2
            { let r = { let t = m.env.stub_va(0x97f); m.call_r(t, &[], &[x0 as u64], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2f8b4 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 2f8b9 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 2f8c0 lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // 2f8c4 mov QWORD PTR [rbp-0x100],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r14) as u64);
            // 2f8cb mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f8cf xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 2f8d2 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // 2f8d7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2f8dc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f8df mov rdi,r14
            rdi = r14;
            // 2f8e2 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 2f8e5 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2f8e8 mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f8ec mov esi,0x2880
            rsi = (0x2880u64) as u32 as u64;
            // 2f8f1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2f8f6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f8f9 mov rdi,r14
            rdi = r14;
            // 2f8fc mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 2f8ff mov r15d,r12d
            r15 = ((r12 as u32 as u64)) as u32 as u64;
            // 2f902 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2f905 mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f909 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // 2f90e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2f910 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2f915 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f918 mov rdi,r14
            rdi = r14;
            // 2f91b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2f91e mov rax,QWORD PTR [rbp-0xd8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff28)) as u64;
            // 2f925 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2f927 je 0x2fb8f
            if fl.e() { pc = 0x2fb8f; continue; }
            pc = 0x2f92d;
        }
        0x2f92d => {
            // block 2f92d
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2f92d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f92d lea rax,[rbx+0x8]
            rax = rbx.wrapping_add(0x8);
            // 2f931 mov QWORD PTR [rbp-0xe0],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rbx) as u64);
            // 2f938 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 2f93f mov r12d,0x100
            r12 = (0x100u64) as u32 as u64;
            // 2f945 lea rbx,[rbp-0x50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2f949 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 2f94c mov DWORD PTR [rbp-0x104],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fefc), ((rax as u32 as u64)) as u32);
            // 2f952 mov DWORD PTR [rbp-0x114],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feec), (0x0u64) as u32);
            pc = 0x2f95c;
        }
        0x2f95c => {
            // block 2f95c
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2f95c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f95c xorps xmm0,xmm0
            x0 = 0;
            // 2f95f movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 2f963 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 2f96b mov BYTE PTR [rbp-0x51],0xc
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffaf), (0xcu64) as u8);
            // 2f96f mov rdi,rbx
            rdi = rbx;
            // 2f972 lea rsi,[rbp-0x51]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffaf);
            // 2f976 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2f97b mov rax,QWORD PTR [rbp-0xe8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff18)) as u64;
            // 2f982 lea esi,[rax+r12*1-0x100]
            rsi = (rax.wrapping_add(r12).wrapping_add(0xffff_ffff_ffff_ff00)) as u32 as u64;
            // 2f98a mov rdx,rbx
            rdx = rbx;
            // 2f98d call 0x2fd10
            { let r = { let t = m.env.code_va(0x2fd10); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2f992 cmp r15d,0x100
            fl = sub::<32>((r15 as u32 as u64), 0x100u64).1;
            // 2f999 mov r14d,0x100
            r14 = (0x100u64) as u32 as u64;
            // 2f99f cmovb r14d,r15d
            if fl.b() { r14 = ((r15 as u32 as u64)) as u32 as u64; } else { r14 = r14 as u32 as u64; }
            // 2f9a3 xorps xmm0,xmm0
            x0 = 0;
            // 2f9a6 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // 2f9aa mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 2f9b2 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 2f9b9 mov rsi,rbx
            rsi = rbx;
            // 2f9bc call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2f9c1 lea rbx,[rbp-0xa0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 2f9c8 mov rdi,rbx
            rdi = rbx;
            // 2f9cb lea rsi,[rbp-0x70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 2f9cf call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2f9d4 mov rdi,QWORD PTR [rbp-0xe0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff20)) as u64;
            // 2f9db lea rsi,[rbp-0x88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 2f9e2 mov rdx,rbx
            rdx = rbx;
            // 2f9e5 mov rcx,QWORD PTR [rbp-0xf0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff10)) as u64;
            // 2f9ec mov r8d,r14d
            r8 = ((r14 as u32 as u64)) as u32 as u64;
            // 2f9ef call 0x2fdd8
            { let r = { let t = m.env.code_va(0x2fdd8); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2f9f4 mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 2f9fb test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f9fe je 0x2fa15
            if fl.e() { pc = 0x2fa15; continue; }
            pc = 0x2fa00;
        }
        0x2fa00 => {
            // block 2fa00
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa00 cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 2fa07 je 0x2fa10
            if fl.e() { pc = 0x2fa10; continue; }
            pc = 0x2fa09;
        }
        0x2fa09 => {
            // block 2fa09
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa09 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x2fa10;
        }
        0x2fa10 => {
            // block 2fa10
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa10 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2fa15;
        }
        0x2fa15 => {
            // block 2fa15
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa15 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 2fa1c test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2fa1f je 0x2fa30
            if fl.e() { pc = 0x2fa30; continue; }
            pc = 0x2fa21;
        }
        0x2fa21 => {
            // block 2fa21
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa21 cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 2fa25 je 0x2fa2b
            if fl.e() { pc = 0x2fa2b; continue; }
            pc = 0x2fa27;
        }
        0x2fa27 => {
            // block 2fa27
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa27 mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x2fa2b;
        }
        0x2fa2b => {
            // block 2fa2b
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa2b call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2fa30;
        }
        0x2fa30 => {
            // block 2fa30
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa30 cmp DWORD PTR [rbp-0x104],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fefc)) as u64, 0x0u64).1;
            // 2fa37 je 0x2fb0b
            if fl.e() { pc = 0x2fb0b; continue; }
            pc = 0x2fa3d;
        }
        0x2fa3d => {
            // block 2fa3d
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa3d mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 2fa44 mov al,BYTE PTR [rax]
            rax = (rax & !0xff) | ((m.r8(rax) as u64) as u8 as u64);
            // 2fa46 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2fa48 je 0x2fb0b
            if fl.e() { pc = 0x2fb0b; continue; }
            pc = 0x2fa4e;
        }
        0x2fa4e => {
            // block 2fa4e
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa4e mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 2fa53 lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 2fa5a lea rsi,[rip+0xfb172] # 0x12abd3
            rsi = m.env.data_va(258976);
            // 2fa61 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fa66 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 2fa69 cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 2fa6e divsd xmm0,QWORD PTR [rbp-0x120]
            x0 = set_lo64(x0, (f64_of(x0) / f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u128))).to_bits());
            // 2fa76 mulsd xmm0,QWORD PTR [rip+0xd035a] # 0xffdd8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83365)) as u128))).to_bits());
            // 2fa7e cvttsd2si rbx,xmm0
            rbx = cvtt64(f64_of(x0)) as u64;
            // 2fa83 cmp ebx,DWORD PTR [rbp-0x114]
            fl = sub::<32>((rbx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_feec)) as u64).1;
            // 2fa89 je 0x2faff
            if fl.e() { pc = 0x2faff; continue; }
            pc = 0x2fa8b;
        }
        0x2fa8b => {
            // block 2fa8b
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa8b movzx edx,BYTE PTR [rbp-0xb8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // 2fa92 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 2fa95 jne 0x2faa3
            if fl.ne() { pc = 0x2faa3; continue; }
            pc = 0x2fa97;
        }
        0x2fa97 => {
            // block 2fa97
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fa97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fa97 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2fa9a lea rsi,[rbp-0xb7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff49);
            // 2faa1 jmp 0x2fab1
            pc = 0x2fab1; continue;
        }
        0x2faa3 => {
            // block 2faa3
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2faa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2faa3 mov rdx,QWORD PTR [rbp-0xb0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 2faaa mov rsi,QWORD PTR [rbp-0xa8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            pc = 0x2fab1;
        }
        0x2fab1 => {
            // block 2fab1
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fab1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fab1 mov rdi,QWORD PTR [rip+0x1106d0] # 0x140188
            rdi = m.r64(m.env.data_va(346453)) as u64;
            // 2fab8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fabd mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 2fac2 mov rdi,rax
            rdi = rax;
            // 2fac5 lea rsi,[rip+0xfb11c] # 0x12abe8
            rsi = m.env.data_va(258997);
            // 2facc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fad1 mov rdi,rax
            rdi = rax;
            // 2fad4 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 2fad6 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fadb mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 2fae0 mov rdi,rax
            rdi = rax;
            // 2fae3 lea rsi,[rip+0xfb108] # 0x12abf2
            rsi = m.env.data_va(259007);
            // 2faea call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2faef mov rdi,rax
            rdi = rax;
            // 2faf2 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2faf7 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 2faf9 mov DWORD PTR [rbp-0x114],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feec), ((rax as u32 as u64)) as u32);
            pc = 0x2faff;
        }
        0x2faff => {
            // block 2faff
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2faff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2faff lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 2fb06 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2fb0b;
        }
        0x2fb0b => {
            // block 2fb0b
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fb0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fb0b mov rbx,QWORD PTR [rbp-0x100]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff00)) as u64;
            // 2fb12 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2fb15 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 2fb19 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // 2fb1e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2fb23 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2fb26 mov rdi,rbx
            rdi = rbx;
            // 2fb29 mov edx,DWORD PTR [rbp-0xf4]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff0c)) as u64) as u32 as u64;
            // 2fb2f call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fb31 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2fb34 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 2fb38 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // 2fb3d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2fb42 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2fb45 mov rdi,rbx
            rdi = rbx;
            // 2fb48 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 2fb4b call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fb4d mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 2fb51 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2fb54 je 0x2fb65
            if fl.e() { pc = 0x2fb65; continue; }
            pc = 0x2fb56;
        }
        0x2fb56 => {
            // block 2fb56
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fb56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fb56 cmp QWORD PTR [rbp-0x48],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, rdi).1;
            // 2fb5a je 0x2fb60
            if fl.e() { pc = 0x2fb60; continue; }
            pc = 0x2fb5c;
        }
        0x2fb5c => {
            // block 2fb5c
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fb5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fb5c mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            pc = 0x2fb60;
        }
        0x2fb60 => {
            // block 2fb60
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fb60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fb60 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2fb65;
        }
        0x2fb65 => {
            // block 2fb65
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fb65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fb65 sub r15d,r14d
            { let (r, f) = sub::<32>((r15 as u32 as u64), (r14 as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // 2fb68 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // 2fb6b add r12d,0x100
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x100u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 2fb72 mov rax,QWORD PTR [rbp-0xd8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff28)) as u64;
            // 2fb79 cmp r13d,eax
            fl = sub::<32>((r13 as u32 as u64), (rax as u32 as u64)).1;
            // 2fb7c lea rbx,[rbp-0x50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2fb80 jb 0x2f95c
            if fl.b() { pc = 0x2f95c; continue; }
            pc = 0x2fb86;
        }
        0x2fb86 => {
            // block 2fb86
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fb86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fb86 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 2fb8d jmp 0x2fb96
            pc = 0x2fb96; continue;
        }
        0x2fb8f => {
            // block 2fb8f
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fb8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fb8f add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2fb93 mov rax,rbx
            rax = rbx;
            pc = 0x2fb96;
        }
        0x2fb96 => {
            // block 2fb96
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fb96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fb96 cmp BYTE PTR [rax],0x0
            fl = sub::<8>(m.r8(rax) as u64, 0x0u64).1;
            // 2fb99 je 0x2fc60
            if fl.e() { pc = 0x2fc60; continue; }
            pc = 0x2fb9f;
        }
        0x2fb9f => {
            // block 2fb9f
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fb9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fb9f lea rsi,[rip+0xfb02d] # 0x12abd3
            rsi = m.env.data_va(258976);
            // 2fba6 lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // 2fbad mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 2fbb2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fbb7 movzx edx,BYTE PTR [rbp-0xd0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff30)) as u64) as u32 as u64;
            // 2fbbe test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 2fbc1 jne 0x2fbcf
            if fl.ne() { pc = 0x2fbcf; continue; }
            pc = 0x2fbc3;
        }
        0x2fbc3 => {
            // block 2fbc3
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fbc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fbc3 lea rsi,[rbp-0xcf]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff31);
            // 2fbca shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2fbcd jmp 0x2fbdd
            pc = 0x2fbdd; continue;
        }
        0x2fbcf => {
            // block 2fbcf
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fbcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fbcf mov rdx,QWORD PTR [rbp-0xc8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff38)) as u64;
            // 2fbd6 mov rsi,QWORD PTR [rbp-0xc0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            pc = 0x2fbdd;
        }
        0x2fbdd => {
            // block 2fbdd
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fbdd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fbdd mov rdi,QWORD PTR [rip+0x1105a4] # 0x140188
            rdi = m.r64(m.env.data_va(346453)) as u64;
            // 2fbe4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fbe9 lea rsi,[rip+0xfb008] # 0x12abf8
            rsi = m.env.data_va(259013);
            // 2fbf0 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 2fbf5 mov rdi,rax
            rdi = rax;
            // 2fbf8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fbfd mov rbx,rax
            rbx = rax;
            // 2fc00 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2fc03 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2fc07 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 2fc0a lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2fc0e call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fc13 mov rsi,QWORD PTR [rip+0x110576] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2fc1a lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2fc1e call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fc23 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2fc26 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2fc2a mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2fc2f mov rdi,rax
            rdi = rax;
            // 2fc32 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fc34 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2fc37 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2fc3b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fc40 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // 2fc44 mov rdi,rbx
            rdi = rbx;
            // 2fc47 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fc4c mov rdi,rbx
            rdi = rbx;
            // 2fc4f call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fc54 lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // 2fc5b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2fc60;
        }
        0x2fc60 => {
            // block 2fc60
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fc60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fc60 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2fc62 add rsp,0x108
            { let (r, f) = add::<64>(rsp, 0x108u64);
              rsp = r;
              fl = f; }
            // 2fc69 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2fc6a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2fc6c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2fc6e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2fc70 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2fc72 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2fc73 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2fc74 => {
            // block 2fc74
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fc74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fc74 jmp 0x2fc76
            pc = 0x2fc76; continue;
        }
        0x2fc76 => {
            // block 2fc76
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fc76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fc76 mov rbx,rax
            rbx = rax;
            // 2fc79 jmp 0x2fcd0
            pc = 0x2fcd0; continue;
        }
        0x2fc7b => {
            // block 2fc7b
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fc7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fc7b mov rbx,rax
            rbx = rax;
            // 2fc7e lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 2fc85 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fc8a jmp 0x2fcd0
            pc = 0x2fcd0; continue;
        }
        0x2fc8c => {
            // block 2fc8c
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fc8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fc8c mov rbx,rax
            rbx = rax;
            // 2fc8f jmp 0x2fcb5
            pc = 0x2fcb5; continue;
        }
        0x2fc91 => {
            // block 2fc91
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fc91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fc91 mov rbx,rax
            rbx = rax;
            // 2fc94 mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 2fc9b test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2fc9e je 0x2fcb5
            if fl.e() { pc = 0x2fcb5; continue; }
            pc = 0x2fca0;
        }
        0x2fca0 => {
            // block 2fca0
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fca0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fca0 cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 2fca7 je 0x2fcb0
            if fl.e() { pc = 0x2fcb0; continue; }
            pc = 0x2fca9;
        }
        0x2fca9 => {
            // block 2fca9
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fca9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fca9 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x2fcb0;
        }
        0x2fcb0 => {
            // block 2fcb0
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fcb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fcb0 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2fcb5;
        }
        0x2fcb5 => {
            // block 2fcb5
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fcb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fcb5 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 2fcbc test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2fcbf je 0x2fcd0
            if fl.e() { pc = 0x2fcd0; continue; }
            pc = 0x2fcc1;
        }
        0x2fcc1 => {
            // block 2fcc1
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fcc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fcc1 cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 2fcc5 je 0x2fccb
            if fl.e() { pc = 0x2fccb; continue; }
            pc = 0x2fcc7;
        }
        0x2fcc7 => {
            // block 2fcc7
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fcc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fcc7 mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x2fccb;
        }
        0x2fccb => {
            // block 2fccb
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fccb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fccb call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2fcd0;
        }
        0x2fcd0 => {
            // block 2fcd0
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fcd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fcd0 mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 2fcd4 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2fcd7 je 0x2fce8
            if fl.e() { pc = 0x2fce8; continue; }
            pc = 0x2fcd9;
        }
        0x2fcd9 => {
            // block 2fcd9
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fcd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fcd9 cmp QWORD PTR [rbp-0x48],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, rdi).1;
            // 2fcdd je 0x2fce3
            if fl.e() { pc = 0x2fce3; continue; }
            pc = 0x2fcdf;
        }
        0x2fcdf => {
            // block 2fcdf
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fcdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fcdf mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            pc = 0x2fce3;
        }
        0x2fce3 => {
            // block 2fce3
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fce3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fce3 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2fce8;
        }
        0x2fce8 => {
            // block 2fce8
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fce8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fce8 mov rdi,rbx
            rdi = rbx;
            // 2fceb call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fcf0 mov rbx,rax
            rbx = rax;
            // 2fcf3 jmp 0x2fd01
            pc = 0x2fd01; continue;
        }
        0x2fcf5 => {
            // block 2fcf5
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fcf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fcf5 mov rbx,rax
            rbx = rax;
            // 2fcf8 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2fcfc call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2fd01;
        }
        0x2fd01 => {
            // block 2fd01
            #[cfg(feature = "trace")] eprintln!("f_0002f84c pc 2fd01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fd01 lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // 2fd08 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2fd0d jmp 0x2fce8
            pc = 0x2fce8; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: unproven, tier P2 @0x30040 size=1570
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00030040<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x30040;
    loop {
        match pc {
        0x30040 => {
            // block 30040
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30040 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30040 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 30041 mov rbp,rsp
            rbp = rsp;
            // 30044 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 30046 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 30048 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 3004a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 3004c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3004d sub rsp,0x128
            { let (r, f) = sub::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // 30054 mov r14,rdx
            r14 = rdx;
            // 30057 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 3005a mov r12,rdi
            r12 = rdi;
            // 3005d cmp r15d,0xfffff
            fl = sub::<32>((r15 as u32 as u64), 0xfffffu64).1;
            // 30064 seta al
            rax = (rax & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // 30067 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 3006a lea ebx,[rax+rax*2+0x7]
            rbx = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x7)) as u32 as u64;
            // 3006e mov DWORD PTR [rbp-0x124],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fedc), ((rbx as u32 as u64)) as u32);
            // 30074 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 30076 mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 3007a sub rdx,QWORD PTR [r14]
            { let (r, f) = sub::<64>(rdx, m.r64(r14) as u64);
              rdx = r;
              fl = f; }
            // 3007d cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 30080 cmovbe edx,ecx
            if fl.be() { rdx = ((rcx as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // 30083 mov QWORD PTR [rbp-0x120],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rdx) as u64);
            // 3008a xorps xmm0,xmm0
            x0 = 0;
            // 3008d movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 30091 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 30099 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 3009b cvtsi2sd xmm1,rax
            x1 = set_lo64(x1, (sx::<64>(rax) as f64).to_bits());
            // 300a0 movsd QWORD PTR [rbp-0x148],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), x1 as u64);
            // 300a8 movsd xmm0,QWORD PTR [rip+0xcfd20] # 0xffdd0
            x0 = m.r64(m.env.data_va(83357)) as u128;
            // 300b0 mulsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) * f64_of(x1)).to_bits());
            // 300b4 call 0xec1b2
            { let r = { let t = m.env.stub_va(0x97f); m.call_r(t, &[], &[x0 as u64], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 300b9 movsd QWORD PTR [rbp-0x100],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), x0 as u64);
            // 300c1 lea rdi,[r12+0x18]
            rdi = r12.wrapping_add(0x18);
            // 300c6 mov QWORD PTR [rbp-0x118],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rdi) as u64);
            // 300cd mov rax,QWORD PTR [r12+0x18]
            rax = m.r64(r12.wrapping_add(0x18)) as u64;
            // 300d2 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 300d6 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // 300db mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 300e0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 300e3 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 300e5 mov rbx,rdi
            rbx = rdi;
            // 300e8 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 300ea mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 300ed mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 300f1 mov esi,0x2880
            rsi = (0x2880u64) as u32 as u64;
            // 300f6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 300fb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 300fe mov rdi,rbx
            rdi = rbx;
            // 30101 mov rdx,QWORD PTR [rbp-0x120]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 30108 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 3010a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3010d mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 30111 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // 30116 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 30118 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3011d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 30120 mov rdi,rbx
            rdi = rbx;
            // 30123 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30125 cvttsd2si rax,QWORD PTR [rbp-0x100]
            rax = cvtt64(f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff00)) as u128))) as u64;
            // 3012e mov QWORD PTR [rbp-0x130],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rax) as u64);
            // 30135 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 30137 je 0x30473
            if fl.e() { pc = 0x30473; continue; }
            pc = 0x3013d;
        }
        0x3013d => {
            // block 3013d
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3013d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3013d lea rax,[r12+0x8]
            rax = r12.wrapping_add(0x8);
            // 30142 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // 30149 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 3014b lea r13,[rbp-0xc1]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff3f);
            // 30152 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 30154 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 3015b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3015d mov QWORD PTR [rbp-0x140],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rax) as u64);
            pc = 0x30164;
        }
        0x30164 => {
            // block 30164
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30164 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30164 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 30169 mov rdi,r12
            rdi = r12;
            // 3016c call 0x30662
            { let r = { let t = m.env.code_va(0x30662); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30171 mov QWORD PTR [rbp-0x108],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rbx) as u64);
            // 30178 xorpd xmm0,xmm0
            x0 = 0;
            // 3017c movapd XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // 30181 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 30189 mov BYTE PTR [rbp-0x71],0x12
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff8f), (0x12u64) as u8);
            // 3018d lea rbx,[rbp-0x70]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 30191 mov rdi,rbx
            rdi = rbx;
            // 30194 lea rsi,[rbp-0x71]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8f);
            // 30198 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 3019d mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 301a0 mov rdx,rbx
            rdx = rbx;
            // 301a3 call 0x2fd10
            { let r = { let t = m.env.code_va(0x2fd10); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 301a8 mov DWORD PTR [rbp-0x110],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef0), ((r15 as u32 as u64)) as u32);
            // 301af xorpd xmm0,xmm0
            x0 = 0;
            // 301b3 movapd XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 301bb mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // 301c3 mov rbx,QWORD PTR [rbp-0x100]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff00)) as u64;
            // 301ca xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x301cd;
        }
        0x301cd => {
            // block 301cd
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 301cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 301cd mov ebx,ebx
            rbx = ((rbx as u32 as u64)) as u32 as u64;
            // 301cf mov rsi,QWORD PTR [r14]
            rsi = m.r64(r14) as u64;
            // 301d2 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 301d6 sub rax,rsi
            { let (r, f) = sub::<64>(rax, rsi);
              rax = r;
              fl = f; }
            // 301d9 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 301dc jae 0x30215
            if fl.ae() { pc = 0x30215; continue; }
            pc = 0x301de;
        }
        0x301de => {
            // block 301de
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 301de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 301de add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 301e1 mov rax,QWORD PTR [rbp-0x88]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 301e8 cmp rax,QWORD PTR [rbp-0x80]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64).1;
            // 301ec je 0x301fb
            if fl.e() { pc = 0x301fb; continue; }
            pc = 0x301ee;
        }
        0x301ee => {
            // block 301ee
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 301ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 301ee mov cl,BYTE PTR [rsi]
            rcx = (rcx & !0xff) | ((m.r8(rsi) as u64) as u8 as u64);
            // 301f0 mov BYTE PTR [rax],cl
            m.w8(rax, ((rcx as u8 as u64)) as u8);
            // 301f2 inc QWORD PTR [rbp-0x88]
            { let (r, f) = inc::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64, fl); m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (r) as u64); fl = f; }
            // 301f9 jmp 0x30207
            pc = 0x30207; continue;
        }
        0x301fb => {
            // block 301fb
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 301fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 301fb lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 30202 call 0xebb0a
            { let r = { let t = m.env.stub_va(0x2d7); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x30207;
        }
        0x30207 => {
            // block 30207
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30207 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30207 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 3020a inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 3020c cmp r15,0x100
            fl = sub::<64>(r15, 0x100u64).1;
            // 30213 jb 0x301cd
            if fl.b() { pc = 0x301cd; continue; }
            pc = 0x30215;
        }
        0x30215 => {
            // block 30215
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30215 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30215 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 3021c lea rsi,[rbp-0x70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 30220 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30225 mov r15,QWORD PTR [rbp-0x118]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 3022c lea rbx,[rbp-0xc0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 30233 mov rdi,rbx
            rdi = rbx;
            // 30236 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 3023d call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30242 mov r8d,DWORD PTR [rbp-0x88]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // 30249 sub r8d,DWORD PTR [rbp-0x90]
            { let (r, f) = sub::<32>((r8 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 30250 mov rdi,r12
            rdi = r12;
            // 30253 lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 3025a mov rdx,rbx
            rdx = rbx;
            // 3025d lea rcx,[rbp-0x50]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 30261 call 0x2fdd8
            { let r = { let t = m.env.code_va(0x2fdd8); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30266 mov rdi,QWORD PTR [rbp-0xc0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // 3026d test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 30270 je 0x30287
            if fl.e() { pc = 0x30287; continue; }
            pc = 0x30272;
        }
        0x30272 => {
            // block 30272
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30272 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30272 cmp QWORD PTR [rbp-0xb8],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64, rdi).1;
            // 30279 je 0x30282
            if fl.e() { pc = 0x30282; continue; }
            pc = 0x3027b;
        }
        0x3027b => {
            // block 3027b
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3027b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3027b mov QWORD PTR [rbp-0xb8],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rdi) as u64);
            pc = 0x30282;
        }
        0x30282 => {
            // block 30282
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30282 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x30287;
        }
        0x30287 => {
            // block 30287
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30287 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30287 mov rdi,QWORD PTR [rbp-0xa8]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // 3028e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 30291 je 0x302a8
            if fl.e() { pc = 0x302a8; continue; }
            pc = 0x30293;
        }
        0x30293 => {
            // block 30293
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30293 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30293 cmp QWORD PTR [rbp-0xa0],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64, rdi).1;
            // 3029a je 0x302a3
            if fl.e() { pc = 0x302a3; continue; }
            pc = 0x3029c;
        }
        0x3029c => {
            // block 3029c
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3029c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3029c mov QWORD PTR [rbp-0xa0],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (rdi) as u64);
            pc = 0x302a3;
        }
        0x302a3 => {
            // block 302a3
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 302a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 302a3 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x302a8;
        }
        0x302a8 => {
            // block 302a8
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 302a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 302a8 mov BYTE PTR [rbp-0xc1],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff3f), (0x0u64) as u8);
            pc = 0x302af;
        }
        0x302af => {
            // block 302af
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 302af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 302af mov rdi,r12
            rdi = r12;
            // 302b2 mov rsi,r13
            rsi = r13;
            // 302b5 call 0x2f626
            { let r = { let t = m.env.code_va(0x2f626); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 302ba test BYTE PTR [rbp-0xc1],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff3f)) as u64 & 0x1u64);
            // 302c1 jne 0x302af
            if fl.ne() { pc = 0x302af; continue; }
            pc = 0x302c3;
        }
        0x302c3 => {
            // block 302c3
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 302c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 302c3 mov rax,QWORD PTR [rbp-0x100]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff00)) as u64;
            // 302ca add eax,0x100
            { let (r, f) = add::<32>((rax as u32 as u64), 0x100u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 302cf mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 302d6 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 302dd test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 302df je 0x303c2
            if fl.e() { pc = 0x303c2; continue; }
            pc = 0x302e5;
        }
        0x302e5 => {
            // block 302e5
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 302e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 302e5 mov rax,QWORD PTR [rbp-0x138]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 302ec mov al,BYTE PTR [rax]
            rax = (rax & !0xff) | ((m.r8(rax) as u64) as u8 as u64);
            // 302ee test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 302f0 je 0x303c2
            if fl.e() { pc = 0x303c2; continue; }
            pc = 0x302f6;
        }
        0x302f6 => {
            // block 302f6
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 302f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 302f6 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 302fb lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 30302 lea rsi,[rip+0xfa8d1] # 0x12abda
            rsi = m.env.data_va(258983);
            // 30309 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 3030e mov rax,QWORD PTR [rbp-0x100]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff00)) as u64;
            // 30315 mov eax,eax
            rax = ((rax as u32 as u64)) as u32 as u64;
            // 30317 cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 3031c divsd xmm0,QWORD PTR [rbp-0x148]
            x0 = set_lo64(x0, (f64_of(x0) / f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u128))).to_bits());
            // 30324 mulsd xmm0,QWORD PTR [rip+0xcfaac] # 0xffdd8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83365)) as u128))).to_bits());
            // 3032c cvttsd2si rbx,xmm0
            rbx = cvtt64(f64_of(x0)) as u64;
            // 30331 mov rax,QWORD PTR [rbp-0x140]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 30338 cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // 3033a je 0x303af
            if fl.e() { pc = 0x303af; continue; }
            pc = 0x3033c;
        }
        0x3033c => {
            // block 3033c
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3033c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3033c movzx edx,BYTE PTR [rbp-0xe0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff20)) as u64) as u32 as u64;
            // 30343 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 30346 jne 0x30354
            if fl.ne() { pc = 0x30354; continue; }
            pc = 0x30348;
        }
        0x30348 => {
            // block 30348
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30348 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30348 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 3034b lea rsi,[rbp-0xdf]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff21);
            // 30352 jmp 0x30362
            pc = 0x30362; continue;
        }
        0x30354 => {
            // block 30354
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30354 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30354 mov rdx,QWORD PTR [rbp-0xd8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff28)) as u64;
            // 3035b mov rsi,QWORD PTR [rbp-0xd0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff30)) as u64;
            pc = 0x30362;
        }
        0x30362 => {
            // block 30362
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30362 mov rdi,QWORD PTR [rip+0x10fe1f] # 0x140188
            rdi = m.r64(m.env.data_va(346453)) as u64;
            // 30369 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 3036e mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 30373 mov rdi,rax
            rdi = rax;
            // 30376 lea rsi,[rip+0xfa86b] # 0x12abe8
            rsi = m.env.data_va(258997);
            // 3037d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30382 mov rdi,rax
            rdi = rax;
            // 30385 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 30387 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 3038c mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 30391 mov rdi,rax
            rdi = rax;
            // 30394 lea rsi,[rip+0xfa857] # 0x12abf2
            rsi = m.env.data_va(259007);
            // 3039b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 303a0 mov rdi,rax
            rdi = rax;
            // 303a3 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 303a8 mov QWORD PTR [rbp-0x140],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rbx) as u64);
            pc = 0x303af;
        }
        0x303af => {
            // block 303af
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 303af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 303af lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 303b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 303bb mov r15,QWORD PTR [rbp-0x118]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            pc = 0x303c2;
        }
        0x303c2 => {
            // block 303c2
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 303c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 303c2 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 303c5 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 303c9 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // 303ce mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 303d3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 303d6 mov rdi,r15
            rdi = r15;
            // 303d9 mov edx,DWORD PTR [rbp-0x124]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fedc)) as u64) as u32 as u64;
            // 303df call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 303e1 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 303e4 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 303e8 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // 303ed mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 303f2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 303f5 mov rdi,r15
            rdi = r15;
            // 303f8 mov rdx,QWORD PTR [rbp-0x100]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff00)) as u64;
            // 303ff call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30401 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 30403 mov rdi,r12
            rdi = r12;
            // 30406 call 0x30662
            { let r = { let t = m.env.code_va(0x30662); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 3040b mov rdi,QWORD PTR [rbp-0x90]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 30412 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 30415 mov rbx,QWORD PTR [rbp-0x108]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef8)) as u64;
            // 3041c je 0x30433
            if fl.e() { pc = 0x30433; continue; }
            pc = 0x3041e;
        }
        0x3041e => {
            // block 3041e
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3041e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3041e cmp QWORD PTR [rbp-0x88],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64, rdi).1;
            // 30425 je 0x3042e
            if fl.e() { pc = 0x3042e; continue; }
            pc = 0x30427;
        }
        0x30427 => {
            // block 30427
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30427 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30427 mov QWORD PTR [rbp-0x88],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdi) as u64);
            pc = 0x3042e;
        }
        0x3042e => {
            // block 3042e
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3042e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3042e call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x30433;
        }
        0x30433 => {
            // block 30433
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30433 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30433 mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 30437 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3043a mov r15d,DWORD PTR [rbp-0x110]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64) as u32 as u64;
            // 30441 je 0x30452
            if fl.e() { pc = 0x30452; continue; }
            pc = 0x30443;
        }
        0x30443 => {
            // block 30443
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30443 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30443 cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 30447 je 0x3044d
            if fl.e() { pc = 0x3044d; continue; }
            pc = 0x30449;
        }
        0x30449 => {
            // block 30449
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30449 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30449 mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x3044d;
        }
        0x3044d => {
            // block 3044d
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3044d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3044d call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x30452;
        }
        0x30452 => {
            // block 30452
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30452 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30452 add r15d,0x100
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x100u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 30459 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 3045b mov rax,QWORD PTR [rbp-0x130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 30462 cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // 30464 jb 0x30164
            if fl.b() { pc = 0x30164; continue; }
            pc = 0x3046a;
        }
        0x3046a => {
            // block 3046a
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3046a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3046a mov r12,QWORD PTR [rbp-0x138]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 30471 jmp 0x30477
            pc = 0x30477; continue;
        }
        0x30473 => {
            // block 30473
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30473 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30473 add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            pc = 0x30477;
        }
        0x30477 => {
            // block 30477
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30477 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30477 cmp BYTE PTR [r12],0x0
            fl = sub::<8>(m.r8(r12) as u64, 0x0u64).1;
            // 3047c je 0x30543
            if fl.e() { pc = 0x30543; continue; }
            pc = 0x30482;
        }
        0x30482 => {
            // block 30482
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30482 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30482 lea rsi,[rip+0xfa751] # 0x12abda
            rsi = m.env.data_va(258983);
            // 30489 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 30490 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 30495 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 3049a movzx edx,BYTE PTR [rbp-0xf8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff08)) as u64) as u32 as u64;
            // 304a1 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 304a4 jne 0x304b2
            if fl.ne() { pc = 0x304b2; continue; }
            pc = 0x304a6;
        }
        0x304a6 => {
            // block 304a6
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 304a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 304a6 lea rsi,[rbp-0xf7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff09);
            // 304ad shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 304b0 jmp 0x304c0
            pc = 0x304c0; continue;
        }
        0x304b2 => {
            // block 304b2
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 304b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 304b2 mov rdx,QWORD PTR [rbp-0xf0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff10)) as u64;
            // 304b9 mov rsi,QWORD PTR [rbp-0xe8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff18)) as u64;
            pc = 0x304c0;
        }
        0x304c0 => {
            // block 304c0
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 304c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 304c0 mov rdi,QWORD PTR [rip+0x10fcc1] # 0x140188
            rdi = m.r64(m.env.data_va(346453)) as u64;
            // 304c7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 304cc lea rsi,[rip+0xfa725] # 0x12abf8
            rsi = m.env.data_va(259013);
            // 304d3 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 304d8 mov rdi,rax
            rdi = rax;
            // 304db call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 304e0 mov rbx,rax
            rbx = rax;
            // 304e3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 304e6 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 304ea add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 304ed lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 304f1 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 304f6 mov rsi,QWORD PTR [rip+0x10fc93] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 304fd lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 30501 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30506 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 30509 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 3050d mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 30512 mov rdi,rax
            rdi = rax;
            // 30515 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30517 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 3051a lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 3051e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30523 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // 30527 mov rdi,rbx
            rdi = rbx;
            // 3052a call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 3052f mov rdi,rbx
            rdi = rbx;
            // 30532 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30537 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 3053e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x30543;
        }
        0x30543 => {
            // block 30543
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30543 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30543 mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 30547 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3054a je 0x3055b
            if fl.e() { pc = 0x3055b; continue; }
            pc = 0x3054c;
        }
        0x3054c => {
            // block 3054c
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3054c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3054c cmp QWORD PTR [rbp-0x48],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, rdi).1;
            // 30550 je 0x30556
            if fl.e() { pc = 0x30556; continue; }
            pc = 0x30552;
        }
        0x30552 => {
            // block 30552
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30552 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30552 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            pc = 0x30556;
        }
        0x30556 => {
            // block 30556
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30556 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x3055b;
        }
        0x3055b => {
            // block 3055b
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3055b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3055b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 3055d add rsp,0x128
            { let (r, f) = add::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // 30564 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 30565 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 30567 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 30569 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3056b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3056d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3056e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x3056f => {
            // block 3056f
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3056f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3056f mov rbx,rax
            rbx = rax;
            pc = 0x30572;
        }
        0x30572 => {
            // block 30572
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30572 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30572 mov rdi,QWORD PTR [rbp-0x90]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 30579 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3057c je 0x30593
            if fl.e() { pc = 0x30593; continue; }
            pc = 0x3057e;
        }
        0x3057e => {
            // block 3057e
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3057e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3057e cmp QWORD PTR [rbp-0x88],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64, rdi).1;
            // 30585 je 0x3058e
            if fl.e() { pc = 0x3058e; continue; }
            pc = 0x30587;
        }
        0x30587 => {
            // block 30587
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30587 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30587 mov QWORD PTR [rbp-0x88],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdi) as u64);
            pc = 0x3058e;
        }
        0x3058e => {
            // block 3058e
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3058e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3058e call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x30593;
        }
        0x30593 => {
            // block 30593
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30593 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30593 mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 30597 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3059a je 0x305ab
            if fl.e() { pc = 0x305ab; continue; }
            pc = 0x3059c;
        }
        0x3059c => {
            // block 3059c
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3059c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3059c cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 305a0 je 0x305a6
            if fl.e() { pc = 0x305a6; continue; }
            pc = 0x305a2;
        }
        0x305a2 => {
            // block 305a2
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305a2 mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x305a6;
        }
        0x305a6 => {
            // block 305a6
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305a6 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x305ab;
        }
        0x305ab => {
            // block 305ab
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305ab mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 305af test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 305b2 je 0x305c3
            if fl.e() { pc = 0x305c3; continue; }
            pc = 0x305b4;
        }
        0x305b4 => {
            // block 305b4
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305b4 cmp QWORD PTR [rbp-0x48],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, rdi).1;
            // 305b8 je 0x305be
            if fl.e() { pc = 0x305be; continue; }
            pc = 0x305ba;
        }
        0x305ba => {
            // block 305ba
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305ba mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            pc = 0x305be;
        }
        0x305be => {
            // block 305be
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305be call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x305c3;
        }
        0x305c3 => {
            // block 305c3
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305c3 mov rdi,rbx
            rdi = rbx;
            // 305c6 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 305cb jmp 0x3056f
            pc = 0x3056f; continue;
        }
        0x305cd => {
            // block 305cd
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305cd jmp 0x3056f
            pc = 0x3056f; continue;
        }
        0x305cf => {
            // block 305cf
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305cf mov rbx,rax
            rbx = rax;
            // 305d2 jmp 0x30593
            pc = 0x30593; continue;
        }
        0x305d4 => {
            // block 305d4
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305d4 mov rbx,rax
            rbx = rax;
            // 305d7 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 305de call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 305e3 jmp 0x30572
            pc = 0x30572; continue;
        }
        0x305e5 => {
            // block 305e5
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305e5 mov rbx,rax
            rbx = rax;
            // 305e8 jmp 0x305ab
            pc = 0x305ab; continue;
        }
        0x305ea => {
            // block 305ea
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305ea mov rbx,rax
            rbx = rax;
            // 305ed jmp 0x30613
            pc = 0x30613; continue;
        }
        0x305ef => {
            // block 305ef
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305ef mov rbx,rax
            rbx = rax;
            // 305f2 mov rdi,QWORD PTR [rbp-0xc0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // 305f9 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 305fc je 0x30613
            if fl.e() { pc = 0x30613; continue; }
            pc = 0x305fe;
        }
        0x305fe => {
            // block 305fe
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 305fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 305fe cmp QWORD PTR [rbp-0xb8],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64, rdi).1;
            // 30605 je 0x3060e
            if fl.e() { pc = 0x3060e; continue; }
            pc = 0x30607;
        }
        0x30607 => {
            // block 30607
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30607 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30607 mov QWORD PTR [rbp-0xb8],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rdi) as u64);
            pc = 0x3060e;
        }
        0x3060e => {
            // block 3060e
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3060e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3060e call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x30613;
        }
        0x30613 => {
            // block 30613
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30613 mov rdi,QWORD PTR [rbp-0xa8]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // 3061a test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3061d je 0x30572
            if fl.e() { pc = 0x30572; continue; }
            pc = 0x30623;
        }
        0x30623 => {
            // block 30623
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30623 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30623 cmp QWORD PTR [rbp-0xa0],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64, rdi).1;
            // 3062a je 0x30633
            if fl.e() { pc = 0x30633; continue; }
            pc = 0x3062c;
        }
        0x3062c => {
            // block 3062c
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3062c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3062c mov QWORD PTR [rbp-0xa0],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (rdi) as u64);
            pc = 0x30633;
        }
        0x30633 => {
            // block 30633
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30633 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30633 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 30638 jmp 0x30572
            pc = 0x30572; continue;
        }
        0x3063d => {
            // block 3063d
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3063d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3063d jmp 0x305e5
            pc = 0x305e5; continue;
        }
        0x3063f => {
            // block 3063f
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 3063f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3063f mov rbx,rax
            rbx = rax;
            // 30642 jmp 0x30650
            pc = 0x30650; continue;
        }
        0x30644 => {
            // block 30644
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30644 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30644 mov rbx,rax
            rbx = rax;
            // 30647 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 3064b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x30650;
        }
        0x30650 => {
            // block 30650
            #[cfg(feature = "trace")] eprintln!("f_00030040 pc 30650 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30650 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // 30657 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 3065c jmp 0x305ab
            pc = 0x305ab; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x161b4 size=1596
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000161b4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x161b4;
    loop {
        match pc {
        0x161b4 => {
            // block 161b4
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 161b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 161b4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 161b5 mov rbp,rsp
            rbp = rsp;
            // 161b8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 161ba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 161bc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 161be push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 161c0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 161c1 sub rsp,0x128
            { let (r, f) = sub::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // 161c8 mov r15,rsi
            r15 = rsi;
            // 161cb mov r12,rdi
            r12 = rdi;
            // 161ce mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 161d3 jmp 0x16220
            pc = 0x16220; continue;
        }
        0x161d5 => {
            // block 161d5
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 161d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 161d5 lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 161dc mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 161e1 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 161e5 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 161ea lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 161ee call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 161f3 lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 161f7 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 161fc lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 16200 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16205 lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 1620c call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16211 add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 16218 mov rdi,rbx
            rdi = rbx;
            // 1621b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16220;
        }
        0x16220 => {
            // block 16220
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16220 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16220 mov rbx,QWORD PTR [r12+0x10]
            rbx = m.r64(r12.wrapping_add(0x10)) as u64;
            // 16225 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 16228 jne 0x161d5
            if fl.ne() { pc = 0x161d5; continue; }
            pc = 0x1622a;
        }
        0x1622a => {
            // block 1622a
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1622a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1622a mov QWORD PTR [rbp-0x128],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (r12) as u64);
            // 16231 mov QWORD PTR [rbp-0x130],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (r15) as u64);
            // 16238 mov r12,QWORD PTR [r15+0x8]
            r12 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1623c cmp r12,QWORD PTR [r15+0x10]
            fl = sub::<64>(r12, m.r64(r15.wrapping_add(0x10)) as u64).1;
            // 16240 je 0x16714
            if fl.e() { pc = 0x16714; continue; }
            pc = 0x16246;
        }
        0x16246 => {
            // block 16246
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16246 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16246 mov rax,QWORD PTR [rbp-0x128]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 1624d lea rax,[rax+0x8]
            rax = rax.wrapping_add(0x8);
            // 16251 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 16258 lea r14,[rbp-0x108]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 1625f lea r15,[rbp-0xc0]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            pc = 0x16266;
        }
        0x16266 => {
            // block 16266
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16266 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16266 mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 1626e mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 16276 mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // 1627d mov QWORD PTR [r15+0x70],0x0
            m.w64(r15.wrapping_add(0x70), (0x0u64) as u64);
            // 16285 mov QWORD PTR [r15+0x68],0x0
            m.w64(r15.wrapping_add(0x68), (0x0u64) as u64);
            // 1628d mov QWORD PTR [r15+0x60],0x0
            m.w64(r15.wrapping_add(0x60), (0x0u64) as u64);
            // 16295 mov QWORD PTR [r15+0x58],0x0
            m.w64(r15.wrapping_add(0x58), (0x0u64) as u64);
            // 1629d mov QWORD PTR [r15+0x50],0x0
            m.w64(r15.wrapping_add(0x50), (0x0u64) as u64);
            // 162a5 mov QWORD PTR [r15+0x48],0x0
            m.w64(r15.wrapping_add(0x48), (0x0u64) as u64);
            // 162ad mov QWORD PTR [r15+0x40],0x0
            m.w64(r15.wrapping_add(0x40), (0x0u64) as u64);
            // 162b5 mov QWORD PTR [r15+0x38],0x0
            m.w64(r15.wrapping_add(0x38), (0x0u64) as u64);
            // 162bd mov QWORD PTR [r15+0x30],0x0
            m.w64(r15.wrapping_add(0x30), (0x0u64) as u64);
            // 162c5 mov QWORD PTR [r15+0x28],0x0
            m.w64(r15.wrapping_add(0x28), (0x0u64) as u64);
            // 162cd mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            // 162d5 mov QWORD PTR [r15+0x18],0x0
            m.w64(r15.wrapping_add(0x18), (0x0u64) as u64);
            // 162dd mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // 162e5 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 162ed mov QWORD PTR [r15],0x0
            m.w64(r15, (0x0u64) as u64);
            // 162f4 mov eax,DWORD PTR [r12+0x4]
            rax = (m.r32(r12.wrapping_add(0x4)) as u64) as u32 as u64;
            // 162f9 mov DWORD PTR [rbp-0x11c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee4), ((rax as u32 as u64)) as u32);
            // 162ff mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // 16303 mov DWORD PTR [rbp-0x120],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee0), ((rax as u32 as u64)) as u32);
            // 16309 mov eax,DWORD PTR [r12+0x8]
            rax = (m.r32(r12.wrapping_add(0x8)) as u64) as u32 as u64;
            // 1630e mov DWORD PTR [rbp-0x118],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee8), ((rax as u32 as u64)) as u32);
            // 16314 lea rsi,[r12+0x18]
            rsi = r12.wrapping_add(0x18);
            // 16319 mov rdi,r14
            rdi = r14;
            // 1631c call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16321 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // 16326 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 1632d mov rcx,QWORD PTR [rbp-0xc0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // 16334 mov rax,QWORD PTR [rbp-0xb8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // 1633b cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 1633e je 0x1635b
            if fl.e() { pc = 0x1635b; continue; }
            pc = 0x16340;
        }
        0x16340 => {
            // block 16340
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16340 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16340 lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 16344 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 16347 not rdx
            { let v = !rdx; rdx = v; }
            // 1634a and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 1634e add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 16351 mov QWORD PTR [rbp-0xb8],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rdx) as u64);
            // 16358 mov rcx,rdx
            rcx = rdx;
            pc = 0x1635b;
        }
        0x1635b => {
            // block 1635b
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1635b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1635b mov rbx,QWORD PTR [r12+0x60]
            rbx = m.r64(r12.wrapping_add(0x60)) as u64;
            // 16360 cmp rbx,QWORD PTR [r12+0x68]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x68)) as u64).1;
            // 16365 lea r13,[rbp-0xa8]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1636c jne 0x16377
            if fl.ne() { pc = 0x16377; continue; }
            pc = 0x1636e;
        }
        0x1636e => {
            // block 1636e
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1636e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1636e jmp 0x163a7
            pc = 0x163a7; continue;
        }
        0x16370 => {
            // block 16370
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16370 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16370 mov rcx,QWORD PTR [rbp-0xb8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            pc = 0x16377;
        }
        0x16377 => {
            // block 16377
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16377 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16377 cmp rcx,QWORD PTR [rbp-0xb0]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64).1;
            // 1637e je 0x16391
            if fl.e() { pc = 0x16391; continue; }
            pc = 0x16380;
        }
        0x16380 => {
            // block 16380
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16380 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16380 mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // 16382 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // 16384 add rcx,0x4
            { let (r, f) = add::<64>(rcx, 0x4u64);
              rcx = r;
              fl = f; }
            // 16388 mov QWORD PTR [rbp-0xb8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rcx) as u64);
            // 1638f jmp 0x1639c
            pc = 0x1639c; continue;
        }
        0x16391 => {
            // block 16391
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16391 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16391 mov rdi,r15
            rdi = r15;
            // 16394 mov rsi,rbx
            rsi = rbx;
            // 16397 call 0xeba0e
            { let r = { let t = m.env.stub_va(0x1db); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1639c;
        }
        0x1639c => {
            // block 1639c
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1639c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1639c add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 163a0 cmp rbx,QWORD PTR [r12+0x68]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x68)) as u64).1;
            // 163a5 jne 0x16370
            if fl.ne() { pc = 0x16370; continue; }
            pc = 0x163a7;
        }
        0x163a7 => {
            // block 163a7
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 163a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 163a7 mov rax,QWORD PTR [rbp-0xa8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // 163ae mov rcx,QWORD PTR [rbp-0xa0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 163b5 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 163b8 je 0x163d5
            if fl.e() { pc = 0x163d5; continue; }
            pc = 0x163ba;
        }
        0x163ba => {
            // block 163ba
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 163ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 163ba lea rdx,[rcx-0x4]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffc);
            // 163be sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 163c1 not rdx
            { let v = !rdx; rdx = v; }
            // 163c4 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 163c8 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 163cb mov QWORD PTR [rbp-0xa0],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (rdx) as u64);
            // 163d2 mov rax,rdx
            rax = rdx;
            pc = 0x163d5;
        }
        0x163d5 => {
            // block 163d5
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 163d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 163d5 mov rbx,QWORD PTR [r12+0x78]
            rbx = m.r64(r12.wrapping_add(0x78)) as u64;
            // 163da cmp rbx,QWORD PTR [r12+0x80]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x80)) as u64).1;
            // 163e2 jne 0x163ed
            if fl.ne() { pc = 0x163ed; continue; }
            pc = 0x163e4;
        }
        0x163e4 => {
            // block 163e4
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 163e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 163e4 jmp 0x16420
            pc = 0x16420; continue;
        }
        0x163e6 => {
            // block 163e6
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 163e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 163e6 mov rax,QWORD PTR [rbp-0xa0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            pc = 0x163ed;
        }
        0x163ed => {
            // block 163ed
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 163ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 163ed cmp rax,QWORD PTR [rbp-0x98]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64).1;
            // 163f4 je 0x16407
            if fl.e() { pc = 0x16407; continue; }
            pc = 0x163f6;
        }
        0x163f6 => {
            // block 163f6
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 163f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 163f6 mov ecx,DWORD PTR [rbx]
            rcx = (m.r32(rbx) as u64) as u32 as u64;
            // 163f8 mov DWORD PTR [rax],ecx
            m.w32(rax, ((rcx as u32 as u64)) as u32);
            // 163fa add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 163fe mov QWORD PTR [rbp-0xa0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (rax) as u64);
            // 16405 jmp 0x16412
            pc = 0x16412; continue;
        }
        0x16407 => {
            // block 16407
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16407 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16407 mov rdi,r13
            rdi = r13;
            // 1640a mov rsi,rbx
            rsi = rbx;
            // 1640d call 0xeba56
            { let r = { let t = m.env.stub_va(0x223); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16412;
        }
        0x16412 => {
            // block 16412
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16412 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16412 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 16416 cmp rbx,QWORD PTR [r12+0x80]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x80)) as u64).1;
            // 1641e jne 0x163e6
            if fl.ne() { pc = 0x163e6; continue; }
            pc = 0x16420;
        }
        0x16420 => {
            // block 16420
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16420 mov rax,QWORD PTR [rbp-0x90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 16427 mov rcx,QWORD PTR [rbp-0x88]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 1642e cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 16431 je 0x1644e
            if fl.e() { pc = 0x1644e; continue; }
            pc = 0x16433;
        }
        0x16433 => {
            // block 16433
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16433 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16433 lea rdx,[rcx-0x4]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffc);
            // 16437 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 1643a not rdx
            { let v = !rdx; rdx = v; }
            // 1643d and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 16441 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 16444 mov QWORD PTR [rbp-0x88],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdx) as u64);
            // 1644b mov rax,rdx
            rax = rdx;
            pc = 0x1644e;
        }
        0x1644e => {
            // block 1644e
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1644e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1644e mov rbx,QWORD PTR [r12+0x90]
            rbx = m.r64(r12.wrapping_add(0x90)) as u64;
            // 16456 cmp rbx,QWORD PTR [r12+0x98]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x98)) as u64).1;
            // 1645e jne 0x16469
            if fl.ne() { pc = 0x16469; continue; }
            pc = 0x16460;
        }
        0x16460 => {
            // block 16460
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16460 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16460 jmp 0x1649d
            pc = 0x1649d; continue;
        }
        0x16462 => {
            // block 16462
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16462 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16462 mov rax,QWORD PTR [rbp-0x88]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0x16469;
        }
        0x16469 => {
            // block 16469
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16469 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16469 cmp rax,QWORD PTR [rbp-0x80]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64).1;
            // 1646d je 0x16480
            if fl.e() { pc = 0x16480; continue; }
            pc = 0x1646f;
        }
        0x1646f => {
            // block 1646f
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1646f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1646f mov ecx,DWORD PTR [rbx]
            rcx = (m.r32(rbx) as u64) as u32 as u64;
            // 16471 mov DWORD PTR [rax],ecx
            m.w32(rax, ((rcx as u32 as u64)) as u32);
            // 16473 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 16477 mov QWORD PTR [rbp-0x88],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rax) as u64);
            // 1647e jmp 0x1648f
            pc = 0x1648f; continue;
        }
        0x16480 => {
            // block 16480
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16480 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16480 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 16487 mov rsi,rbx
            rsi = rbx;
            // 1648a call 0xeba32
            { let r = { let t = m.env.stub_va(0x1ff); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1648f;
        }
        0x1648f => {
            // block 1648f
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1648f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1648f add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 16493 cmp rbx,QWORD PTR [r12+0x98]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x98)) as u64).1;
            // 1649b jne 0x16462
            if fl.ne() { pc = 0x16462; continue; }
            pc = 0x1649d;
        }
        0x1649d => {
            // block 1649d
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1649d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1649d mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 164a1 mov rcx,QWORD PTR [rbp-0x70]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 164a5 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 164a8 je 0x164c2
            if fl.e() { pc = 0x164c2; continue; }
            pc = 0x164aa;
        }
        0x164aa => {
            // block 164aa
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 164aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 164aa lea rdx,[rcx-0x4]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffc);
            // 164ae sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 164b1 not rdx
            { let v = !rdx; rdx = v; }
            // 164b4 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 164b8 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 164bb mov QWORD PTR [rbp-0x70],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rdx) as u64);
            // 164bf mov rax,rdx
            rax = rdx;
            pc = 0x164c2;
        }
        0x164c2 => {
            // block 164c2
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 164c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 164c2 mov rbx,QWORD PTR [r12+0xa8]
            rbx = m.r64(r12.wrapping_add(0xa8)) as u64;
            // 164ca cmp rbx,QWORD PTR [r12+0xb0]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0xb0)) as u64).1;
            // 164d2 jne 0x164da
            if fl.ne() { pc = 0x164da; continue; }
            pc = 0x164d4;
        }
        0x164d4 => {
            // block 164d4
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 164d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 164d4 jmp 0x16508
            pc = 0x16508; continue;
        }
        0x164d6 => {
            // block 164d6
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 164d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 164d6 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x164da;
        }
        0x164da => {
            // block 164da
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 164da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 164da cmp rax,QWORD PTR [rbp-0x68]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 164de je 0x164ee
            if fl.e() { pc = 0x164ee; continue; }
            pc = 0x164e0;
        }
        0x164e0 => {
            // block 164e0
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 164e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 164e0 mov ecx,DWORD PTR [rbx]
            rcx = (m.r32(rbx) as u64) as u32 as u64;
            // 164e2 mov DWORD PTR [rax],ecx
            m.w32(rax, ((rcx as u32 as u64)) as u32);
            // 164e4 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 164e8 mov QWORD PTR [rbp-0x70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rax) as u64);
            // 164ec jmp 0x164fa
            pc = 0x164fa; continue;
        }
        0x164ee => {
            // block 164ee
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 164ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 164ee lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 164f2 mov rsi,rbx
            rsi = rbx;
            // 164f5 call 0xeb9ea
            { let r = { let t = m.env.stub_va(0x1b7); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x164fa;
        }
        0x164fa => {
            // block 164fa
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 164fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 164fa add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 164fe cmp rbx,QWORD PTR [r12+0xb0]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0xb0)) as u64).1;
            // 16506 jne 0x164d6
            if fl.ne() { pc = 0x164d6; continue; }
            pc = 0x16508;
        }
        0x16508 => {
            // block 16508
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16508 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16508 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1650c mov rcx,QWORD PTR [rbp-0x58]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 16510 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 16513 je 0x1652d
            if fl.e() { pc = 0x1652d; continue; }
            pc = 0x16515;
        }
        0x16515 => {
            // block 16515
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16515 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16515 lea rdx,[rcx-0x4]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffc);
            // 16519 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 1651c not rdx
            { let v = !rdx; rdx = v; }
            // 1651f and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 16523 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 16526 mov QWORD PTR [rbp-0x58],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rdx) as u64);
            // 1652a mov rax,rdx
            rax = rdx;
            pc = 0x1652d;
        }
        0x1652d => {
            // block 1652d
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1652d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1652d mov rbx,QWORD PTR [r12+0xc0]
            rbx = m.r64(r12.wrapping_add(0xc0)) as u64;
            // 16535 cmp rbx,QWORD PTR [r12+0xc8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0xc8)) as u64).1;
            // 1653d jne 0x16545
            if fl.ne() { pc = 0x16545; continue; }
            pc = 0x1653f;
        }
        0x1653f => {
            // block 1653f
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1653f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1653f jmp 0x16573
            pc = 0x16573; continue;
        }
        0x16541 => {
            // block 16541
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16541 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16541 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x16545;
        }
        0x16545 => {
            // block 16545
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16545 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16545 cmp rax,QWORD PTR [rbp-0x50]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64).1;
            // 16549 je 0x16559
            if fl.e() { pc = 0x16559; continue; }
            pc = 0x1654b;
        }
        0x1654b => {
            // block 1654b
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1654b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1654b mov ecx,DWORD PTR [rbx]
            rcx = (m.r32(rbx) as u64) as u32 as u64;
            // 1654d mov DWORD PTR [rax],ecx
            m.w32(rax, ((rcx as u32 as u64)) as u32);
            // 1654f add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 16553 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 16557 jmp 0x16565
            pc = 0x16565; continue;
        }
        0x16559 => {
            // block 16559
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16559 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16559 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1655d mov rsi,rbx
            rsi = rbx;
            // 16560 call 0xeb9ea
            { let r = { let t = m.env.stub_va(0x1b7); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16565;
        }
        0x16565 => {
            // block 16565
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16565 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16565 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 16569 cmp rbx,QWORD PTR [r12+0xc8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0xc8)) as u64).1;
            // 16571 jne 0x16541
            if fl.ne() { pc = 0x16541; continue; }
            pc = 0x16573;
        }
        0x16573 => {
            // block 16573
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16573 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16573 mov rax,QWORD PTR [rbp-0x128]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 1657a mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // 1657e cmp r13,QWORD PTR [rax+0x18]
            fl = sub::<64>(r13, m.r64(rax.wrapping_add(0x18)) as u64).1;
            // 16582 je 0x166af
            if fl.e() { pc = 0x166af; continue; }
            pc = 0x16588;
        }
        0x16588 => {
            // block 16588
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16588 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16588 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 1658f mov QWORD PTR [r13+0x10],rax
            m.w64(r13.wrapping_add(0x10), (rax) as u64);
            // 16593 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 1659a mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 165a1 mov QWORD PTR [r13+0x8],rcx
            m.w64(r13.wrapping_add(0x8), (rcx) as u64);
            // 165a5 mov QWORD PTR [r13+0x0],rax
            m.w64(r13, (rax) as u64);
            // 165a9 lea rbx,[r13+0x18]
            rbx = r13.wrapping_add(0x18);
            // 165ad mov rdi,rbx
            rdi = rbx;
            // 165b0 mov rsi,r14
            rsi = r14;
            // 165b3 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 165b8 lea rax,[rbp-0xf0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 165bf mov rcx,rax
            rcx = rax;
            // 165c2 mov ax,WORD PTR [rcx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rcx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 165c6 mov WORD PTR [r13+0x5c],ax
            m.w16(r13.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 165cb mov eax,DWORD PTR [rcx+0x28]
            rax = (m.r32(rcx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 165ce mov DWORD PTR [r13+0x58],eax
            m.w32(r13.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 165d2 mov rax,QWORD PTR [rcx+0x20]
            rax = m.r64(rcx.wrapping_add(0x20)) as u64;
            // 165d6 mov QWORD PTR [r13+0x50],rax
            m.w64(r13.wrapping_add(0x50), (rax) as u64);
            // 165da mov rax,QWORD PTR [rcx+0x18]
            rax = m.r64(rcx.wrapping_add(0x18)) as u64;
            // 165de mov QWORD PTR [r13+0x48],rax
            m.w64(r13.wrapping_add(0x48), (rax) as u64);
            // 165e2 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 165e6 mov QWORD PTR [r13+0x40],rax
            m.w64(r13.wrapping_add(0x40), (rax) as u64);
            // 165ea mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 165ed mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 165f1 mov QWORD PTR [r13+0x38],rcx
            m.w64(r13.wrapping_add(0x38), (rcx) as u64);
            // 165f5 mov QWORD PTR [r13+0x30],rax
            m.w64(r13.wrapping_add(0x30), (rax) as u64);
            // 165f9 lea rdi,[r13+0x60]
            rdi = r13.wrapping_add(0x60);
            // 165fd mov QWORD PTR [rbp-0x140],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rdi) as u64);
            // 16604 mov rsi,r15
            rsi = r15;
            // 16607 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1660c lea r14,[r13+0x78]
            r14 = r13.wrapping_add(0x78);
            // 16610 mov rdi,r14
            rdi = r14;
            // 16613 lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1661a call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1661f mov QWORD PTR [rbp-0x138],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rbx) as u64);
            // 16626 lea r15,[r13+0x90]
            r15 = r13.wrapping_add(0x90);
            // 1662d mov rdi,r15
            rdi = r15;
            // 16630 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 16637 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1663c lea rbx,[r13+0xa8]
            rbx = r13.wrapping_add(0xa8);
            // 16643 mov rdi,rbx
            rdi = rbx;
            // 16646 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1664a call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1664f lea rdi,[r13+0xc0]
            rdi = r13.wrapping_add(0xc0);
            // 16656 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1665a call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1665f lea rax,[rbp-0x48]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 16663 mov rcx,rax
            rcx = rax;
            // 16666 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 16669 mov DWORD PTR [r13+0xf0],eax
            m.w32(r13.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 16670 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 16674 mov QWORD PTR [r13+0xe8],rax
            m.w64(r13.wrapping_add(0xe8), (rax) as u64);
            // 1667b mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 1667e mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 16682 mov QWORD PTR [r13+0xe0],rcx
            m.w64(r13.wrapping_add(0xe0), (rcx) as u64);
            // 16689 mov QWORD PTR [r13+0xd8],rax
            m.w64(r13.wrapping_add(0xd8), (rax) as u64);
            // 16690 mov rax,QWORD PTR [rbp-0x128]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 16697 add QWORD PTR [rax+0x10],0xf8
            { let (r, f) = add::<64>(m.r64(rax.wrapping_add(0x10)) as u64, 0xf8u64);
              m.w64(rax.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 1669f lea r14,[rbp-0x108]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 166a6 lea r15,[rbp-0xc0]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 166ad jmp 0x166c2
            pc = 0x166c2; continue;
        }
        0x166af => {
            // block 166af
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 166af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 166af mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 166b6 lea rsi,[rbp-0x120]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 166bd call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x166c2;
        }
        0x166c2 => {
            // block 166c2
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 166c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 166c2 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 166c6 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 166cb lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 166cf call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 166d4 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 166db call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 166e0 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 166e7 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 166ec mov rdi,r15
            rdi = r15;
            // 166ef call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 166f4 mov rdi,r14
            rdi = r14;
            // 166f7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 166fc add r12,0xf8
            { let (r, f) = add::<64>(r12, 0xf8u64);
              r12 = r;
              fl = f; }
            // 16703 mov rax,QWORD PTR [rbp-0x130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1670a cmp r12,QWORD PTR [rax+0x10]
            fl = sub::<64>(r12, m.r64(rax.wrapping_add(0x10)) as u64).1;
            // 1670e jne 0x16266
            if fl.ne() { pc = 0x16266; continue; }
            pc = 0x16714;
        }
        0x16714 => {
            // block 16714
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16714 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16714 add rsp,0x128
            { let (r, f) = add::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // 1671b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1671c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1671e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 16720 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 16722 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 16724 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 16725 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x16726 => {
            // block 16726
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16726 mov r12,rax
            r12 = rax;
            // 16729 jmp 0x167b1
            pc = 0x167b1; continue;
        }
        0x1672e => {
            // block 1672e
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1672e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1672e mov r12,rax
            r12 = rax;
            // 16731 jmp 0x167b1
            pc = 0x167b1; continue;
        }
        0x16733 => {
            // block 16733
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16733 mov r12,rax
            r12 = rax;
            // 16736 jmp 0x167b1
            pc = 0x167b1; continue;
        }
        0x16738 => {
            // block 16738
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16738 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16738 mov r12,rax
            r12 = rax;
            // 1673b jmp 0x167b1
            pc = 0x167b1; continue;
        }
        0x1673d => {
            // block 1673d
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1673d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1673d mov r12,rax
            r12 = rax;
            // 16740 jmp 0x167b1
            pc = 0x167b1; continue;
        }
        0x16742 => {
            // block 16742
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16742 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16742 mov r12,rax
            r12 = rax;
            // 16745 jmp 0x167aa
            pc = 0x167aa; continue;
        }
        0x16747 => {
            // block 16747
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16747 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16747 mov QWORD PTR [rbp-0x138],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rbx) as u64);
            // 1674e mov r12,rax
            r12 = rax;
            // 16751 jmp 0x16790
            pc = 0x16790; continue;
        }
        0x16753 => {
            // block 16753
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16753 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16753 mov QWORD PTR [rbp-0x138],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rbx) as u64);
            // 1675a mov r12,rax
            r12 = rax;
            // 1675d jmp 0x16784
            pc = 0x16784; continue;
        }
        0x1675f => {
            // block 1675f
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1675f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1675f mov r12,rax
            r12 = rax;
            // 16762 jmp 0x1677c
            pc = 0x1677c; continue;
        }
        0x16764 => {
            // block 16764
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16764 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16764 mov r12,rax
            r12 = rax;
            // 16767 jmp 0x16774
            pc = 0x16774; continue;
        }
        0x16769 => {
            // block 16769
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16769 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16769 mov r12,rax
            r12 = rax;
            // 1676c mov rdi,rbx
            rdi = rbx;
            // 1676f call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16774;
        }
        0x16774 => {
            // block 16774
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16774 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16774 mov rdi,r15
            rdi = r15;
            // 16777 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1677c;
        }
        0x1677c => {
            // block 1677c
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 1677c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1677c mov rdi,r14
            rdi = r14;
            // 1677f call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16784;
        }
        0x16784 => {
            // block 16784
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16784 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16784 mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 1678b call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16790;
        }
        0x16790 => {
            // block 16790
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 16790 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16790 mov rdi,QWORD PTR [rbp-0x138]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 16797 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1679c lea r14,[rbp-0x108]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 167a3 lea r15,[rbp-0xc0]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            pc = 0x167aa;
        }
        0x167aa => {
            // block 167aa
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 167aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 167aa lea r13,[rbp-0xa8]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            pc = 0x167b1;
        }
        0x167b1 => {
            // block 167b1
            #[cfg(feature = "trace")] eprintln!("f_000161b4 pc 167b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 167b1 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 167b5 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 167ba lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 167be call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 167c3 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 167ca call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 167cf mov rdi,r13
            rdi = r13;
            // 167d2 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 167d7 mov rdi,r15
            rdi = r15;
            // 167da call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 167df mov rdi,r14
            rdi = r14;
            // 167e2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 167e7 mov rdi,r12
            rdi = r12;
            // 167ea call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x17542 size=1626
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00017542<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x17542;
    loop {
        match pc {
        0x17542 => {
            // block 17542
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17542 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17542 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 17543 mov rbp,rsp
            rbp = rsp;
            // 17546 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 17548 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1754a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1754c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1754e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1754f sub rsp,0x1d8
            { let (r, f) = sub::<64>(rsp, 0x1d8u64);
              rsp = r;
              fl = f; }
            // 17556 mov r15,rsi
            r15 = rsi;
            // 17559 mov r13,rdi
            r13 = rdi;
            // 1755c movzx eax,BYTE PTR [r13+0x0]
            rax = (m.r8(r13) as u64) as u32 as u64;
            // 17561 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17563 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 17566 jne 0x17570
            if fl.ne() { pc = 0x17570; continue; }
            pc = 0x17568;
        }
        0x17568 => {
            // block 17568
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17568 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17568 mov rdx,rax
            rdx = rax;
            // 1756b shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 1756e jmp 0x17574
            pc = 0x17574; continue;
        }
        0x17570 => {
            // block 17570
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17570 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17570 mov rdx,QWORD PTR [r13+0x8]
            rdx = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0x17574;
        }
        0x17574 => {
            // block 17574
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17574 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17574 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 17577 je 0x17585
            if fl.e() { pc = 0x17585; continue; }
            pc = 0x17579;
        }
        0x17579 => {
            // block 17579
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17579 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17579 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 1757b jne 0x1758c
            if fl.ne() { pc = 0x1758c; continue; }
            pc = 0x1757d;
        }
        0x1757d => {
            // block 1757d
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1757d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1757d mov rdx,rax
            rdx = rax;
            // 17580 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 17583 jmp 0x17590
            pc = 0x17590; continue;
        }
        0x17585 => {
            // block 17585
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17585 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 17587 jmp 0x17aa2
            pc = 0x17aa2; continue;
        }
        0x1758c => {
            // block 1758c
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1758c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1758c mov rdx,QWORD PTR [r13+0x8]
            rdx = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0x17590;
        }
        0x17590 => {
            // block 17590
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17590 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17590 cmp rdx,0x2
            fl = sub::<64>(rdx, 0x2u64).1;
            // 17594 ja 0x175ff
            if fl.a() { pc = 0x175ff; continue; }
            pc = 0x17596;
        }
        0x17596 => {
            // block 17596
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17596 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17596 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 17598 jne 0x175a0
            if fl.ne() { pc = 0x175a0; continue; }
            pc = 0x1759a;
        }
        0x1759a => {
            // block 1759a
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1759a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1759a lea rdx,[r13+0x1]
            rdx = r13.wrapping_add(0x1);
            // 1759e jmp 0x175a4
            pc = 0x175a4; continue;
        }
        0x175a0 => {
            // block 175a0
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175a0 mov rdx,QWORD PTR [r13+0x10]
            rdx = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0x175a4;
        }
        0x175a4 => {
            // block 175a4
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175a4 movsx rdx,BYTE PTR [rdx]
            rdx = sx::<8>(m.r8(rdx) as u64) as u64;
            // 175a8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 175ab js 0x175bf
            if fl.s() { pc = 0x175bf; continue; }
            pc = 0x175ad;
        }
        0x175ad => {
            // block 175ad
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175ad mov rsi,QWORD PTR [rip+0x128bbc] # 0x140170
            rsi = m.r64(m.env.data_va(346429)) as u64;
            // 175b4 test BYTE PTR [rsi+rdx*4+0x3d],0x4
            fl = logic::<8>(m.r8(rsi.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x3d)) as u64 & 0x4u64);
            // 175b9 jne 0x17736
            if fl.ne() { pc = 0x17736; continue; }
            pc = 0x175bf;
        }
        0x175bf => {
            // block 175bf
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175bf test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 175c1 jne 0x175cb
            if fl.ne() { pc = 0x175cb; continue; }
            pc = 0x175c3;
        }
        0x175c3 => {
            // block 175c3
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175c3 mov rdx,rax
            rdx = rax;
            // 175c6 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 175c9 jmp 0x175cf
            pc = 0x175cf; continue;
        }
        0x175cb => {
            // block 175cb
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175cb mov rdx,QWORD PTR [r13+0x8]
            rdx = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0x175cf;
        }
        0x175cf => {
            // block 175cf
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175cf cmp rdx,0x2
            fl = sub::<64>(rdx, 0x2u64).1;
            // 175d3 jb 0x175ff
            if fl.b() { pc = 0x175ff; continue; }
            pc = 0x175d5;
        }
        0x175d5 => {
            // block 175d5
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175d5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 175d7 jne 0x175df
            if fl.ne() { pc = 0x175df; continue; }
            pc = 0x175d9;
        }
        0x175d9 => {
            // block 175d9
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175d9 lea rdx,[r13+0x1]
            rdx = r13.wrapping_add(0x1);
            // 175dd jmp 0x175e3
            pc = 0x175e3; continue;
        }
        0x175df => {
            // block 175df
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175df mov rdx,QWORD PTR [r13+0x10]
            rdx = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0x175e3;
        }
        0x175e3 => {
            // block 175e3
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175e3 movsx rdx,BYTE PTR [rdx+0x1]
            rdx = sx::<8>(m.r8(rdx.wrapping_add(0x1)) as u64) as u64;
            // 175e8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 175eb js 0x175ff
            if fl.s() { pc = 0x175ff; continue; }
            pc = 0x175ed;
        }
        0x175ed => {
            // block 175ed
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175ed mov rsi,QWORD PTR [rip+0x128b7c] # 0x140170
            rsi = m.r64(m.env.data_va(346429)) as u64;
            // 175f4 test BYTE PTR [rsi+rdx*4+0x3d],0x4
            fl = logic::<8>(m.r8(rsi.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x3d)) as u64 & 0x4u64);
            // 175f9 jne 0x17736
            if fl.ne() { pc = 0x17736; continue; }
            pc = 0x175ff;
        }
        0x175ff => {
            // block 175ff
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 175ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 175ff test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 17601 jne 0x17608
            if fl.ne() { pc = 0x17608; continue; }
            pc = 0x17603;
        }
        0x17603 => {
            // block 17603
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17603 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17603 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 17606 jmp 0x1760c
            pc = 0x1760c; continue;
        }
        0x17608 => {
            // block 17608
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17608 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17608 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0x1760c;
        }
        0x1760c => {
            // block 1760c
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1760c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1760c cmp rax,0x8
            fl = sub::<64>(rax, 0x8u64).1;
            // 17610 jne 0x178d8
            if fl.ne() { pc = 0x178d8; continue; }
            pc = 0x17616;
        }
        0x17616 => {
            // block 17616
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17616 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17616 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 1761d mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 17622 call 0x16080
            { let r = { let t = m.env.code_va(0x16080); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17627 mov rbx,QWORD PTR [rbp-0x158]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 1762e mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 17631 cmp rbx,QWORD PTR [rbp-0x150]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64).1;
            // 17638 je 0x178b5
            if fl.e() { pc = 0x178b5; continue; }
            pc = 0x1763e;
        }
        0x1763e => {
            // block 1763e
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1763e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1763e mov QWORD PTR [rbp-0x1f8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe08), (r15) as u64);
            // 17645 lea r14,[rbp-0x177]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fe89);
            // 1764c lea r12,[r13+0x1]
            r12 = r13.wrapping_add(0x1);
            pc = 0x17650;
        }
        0x17650 => {
            // block 17650
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17650 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17650 mov rsi,QWORD PTR [rbx+0x10]
            rsi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 17654 lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 1765b call 0xb799c
            { let r = { let t = m.env.code_va(0xb799c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17660 movzx edx,BYTE PTR [rbp-0x178]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64) as u32 as u64;
            // 17667 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 17669 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 1766b jne 0x17672
            if fl.ne() { pc = 0x17672; continue; }
            pc = 0x1766d;
        }
        0x1766d => {
            // block 1766d
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1766d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1766d shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 17670 jmp 0x17679
            pc = 0x17679; continue;
        }
        0x17672 => {
            // block 17672
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17672 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17672 mov rdx,QWORD PTR [rbp-0x170]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            pc = 0x17679;
        }
        0x17679 => {
            // block 17679
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17679 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17679 movzx esi,BYTE PTR [r13+0x0]
            rsi = (m.r8(r13) as u64) as u32 as u64;
            // 1767e mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 17681 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 17684 jne 0x1768b
            if fl.ne() { pc = 0x1768b; continue; }
            pc = 0x17686;
        }
        0x17686 => {
            // block 17686
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17686 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17686 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 17689 jmp 0x1768f
            pc = 0x1768f; continue;
        }
        0x1768b => {
            // block 1768b
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1768b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1768b mov rsi,QWORD PTR [r13+0x8]
            rsi = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0x1768f;
        }
        0x1768f => {
            // block 1768f
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1768f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1768f cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 17692 jne 0x176d3
            if fl.ne() { pc = 0x176d3; continue; }
            pc = 0x17694;
        }
        0x17694 => {
            // block 17694
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17694 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17694 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 17696 mov rdi,QWORD PTR [rbp-0x168]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 1769d cmove rdi,r14
            if fl.e() { rdi = r14; }
            // 176a1 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 176a3 mov rsi,r12
            rsi = r12;
            // 176a6 je 0x176ac
            if fl.e() { pc = 0x176ac; continue; }
            pc = 0x176a8;
        }
        0x176a8 => {
            // block 176a8
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 176a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 176a8 mov rsi,QWORD PTR [r13+0x10]
            rsi = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0x176ac;
        }
        0x176ac => {
            // block 176ac
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 176ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 176ac test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 176ae jne 0x176e1
            if fl.ne() { pc = 0x176e1; continue; }
            pc = 0x176b0;
        }
        0x176b0 => {
            // block 176b0
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 176b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 176b0 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 176b3 je 0x1787f
            if fl.e() { pc = 0x1787f; continue; }
            pc = 0x176b9;
        }
        0x176b9 => {
            // block 176b9
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 176b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 176b9 movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 176bc movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 176bf cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 176c1 jne 0x176d3
            if fl.ne() { pc = 0x176d3; continue; }
            pc = 0x176c3;
        }
        0x176c3 => {
            // block 176c3
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 176c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 176c3 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 176c6 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 176c9 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 176cc jne 0x176b9
            if fl.ne() { pc = 0x176b9; continue; }
            pc = 0x176ce;
        }
        0x176ce => {
            // block 176ce
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 176ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 176ce jmp 0x1787f
            pc = 0x1787f; continue;
        }
        0x176d3 => {
            // block 176d3
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 176d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 176d3 lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 176da call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 176df jmp 0x17713
            pc = 0x17713; continue;
        }
        0x176e1 => {
            // block 176e1
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 176e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 176e1 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 176e4 je 0x1787f
            if fl.e() { pc = 0x1787f; continue; }
            pc = 0x176ea;
        }
        0x176ea => {
            // block 176ea
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 176ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 176ea call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 176ef mov r15,r14
            r15 = r14;
            // 176f2 mov r14,r13
            r14 = r13;
            // 176f5 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 176f8 lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 176ff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17704 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 17707 mov r13,r14
            r13 = r14;
            // 1770a mov r14,r15
            r14 = r15;
            // 1770d je 0x1788b
            if fl.e() { pc = 0x1788b; continue; }
            pc = 0x17713;
        }
        0x17713 => {
            // block 17713
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17713 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17713 add rbx,0xf8
            { let (r, f) = add::<64>(rbx, 0xf8u64);
              rbx = r;
              fl = f; }
            // 1771a cmp rbx,QWORD PTR [rbp-0x150]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64).1;
            // 17721 jne 0x17650
            if fl.ne() { pc = 0x17650; continue; }
            pc = 0x17727;
        }
        0x17727 => {
            // block 17727
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17727 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17727 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 1772a mov r15,QWORD PTR [rbp-0x1f8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe08)) as u64;
            // 17731 jmp 0x178b5
            pc = 0x178b5; continue;
        }
        0x17736 => {
            // block 17736
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17736 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17736 mov QWORD PTR [rbp-0x200],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe00), (r13) as u64);
            // 1773d mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 17744 lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // 1774b lea rbx,[rbp-0x130]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 17752 mov rax,QWORD PTR [rip+0x128a6f] # 0x1401c8
            rax = m.r64(m.env.data_va(346517)) as u64;
            // 17759 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 1775d mov QWORD PTR [rbp-0x140],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rcx) as u64);
            // 17764 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 17768 mov QWORD PTR [rbp-0xc8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (rax) as u64);
            // 1776f mov QWORD PTR [rbp-0x138],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (0x0u64) as u64);
            // 1777a mov rsi,rbx
            rsi = rbx;
            // 1777d call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17782 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 1778a mov DWORD PTR [rbp-0x38],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0xffffffffu64) as u32);
            // 17791 mov r14,QWORD PTR [rip+0x128c20] # 0x1403b8
            r14 = m.r64(m.env.data_va(347013)) as u64;
            // 17798 lea r12,[r14+0x18]
            r12 = r14.wrapping_add(0x18);
            // 1779c mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            // 177a3 add r14,0x40
            { let (r, f) = add::<64>(r14, 0x40u64);
              r14 = r;
              fl = f; }
            // 177a7 mov QWORD PTR [rbp-0xc8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (r14) as u64);
            // 177ae mov rdi,rbx
            rdi = rbx;
            // 177b1 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 177b6 mov rdi,rbx
            rdi = rbx;
            // 177b9 mov r13,QWORD PTR [rip+0x128be8] # 0x1403a8
            r13 = m.r64(m.env.data_va(346997)) as u64;
            // 177c0 add r13,0x10
            { let (r, f) = add::<64>(r13, 0x10u64);
              r13 = r;
              fl = f; }
            // 177c4 mov QWORD PTR [rbp-0x130],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (r13) as u64);
            // 177cb mov QWORD PTR [rbp-0xd8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (0x0u64) as u64);
            // 177d6 mov QWORD PTR [rbp-0xe0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x0u64) as u64);
            // 177e1 mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // 177ec mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // 177f7 mov DWORD PTR [rbp-0xd0],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (0x8u64) as u32);
            // 17801 mov rsi,QWORD PTR [rbp-0x200]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe00)) as u64;
            // 17808 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1780d lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 17814 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 17818 call 0xec05c
            { let r = { let t = m.env.stub_va(0x829); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1781d mov edi,DWORD PTR [rbp-0x2c]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 17820 mov rsi,r15
            rsi = r15;
            // 17823 call 0x17202
            { let r = { let t = m.env.code_va(0x17202); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17828 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1782a mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            // 17831 mov QWORD PTR [rbp-0xc8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (r14) as u64);
            // 17838 mov QWORD PTR [rbp-0x130],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (r13) as u64);
            // 1783f lea rdi,[rbp-0xf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 17846 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1784b lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 17852 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17857 mov rsi,QWORD PTR [rip+0x1289a2] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // 1785e add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 17862 lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 17869 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1786e lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // 17875 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1787a jmp 0x17aa2
            pc = 0x17aa2; continue;
        }
        0x1787f => {
            // block 1787f
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1787f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1787f lea rdi,[rbp-0x178]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe88);
            // 17886 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1788b;
        }
        0x1788b => {
            // block 1788b
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1788b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1788b mov r15,QWORD PTR [rbp-0x1f8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe08)) as u64;
            // 17892 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 17895 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 17899 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 1789c movzx esi,WORD PTR [rbx+0x4]
            rsi = (m.r16(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 178a0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 178a2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 178a4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 178a7 mov rdi,r15
            rdi = r15;
            // 178aa call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 178ac mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 178ae jmp 0x178b5
            pc = 0x178b5; continue;
        }
        0x178b0 => {
            // block 178b0
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 178b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 178b0 jmp 0x17af4
            pc = 0x17af4; continue;
        }
        0x178b5 => {
            // block 178b5
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 178b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 178b5 lea rax,[rip+0x129b5c] # 0x141418
            rax = m.env.data_va(351205);
            // 178bc mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // 178c3 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 178ca call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 178cf test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 178d2 je 0x17aa2
            if fl.e() { pc = 0x17aa2; continue; }
            pc = 0x178d8;
        }
        0x178d8 => {
            // block 178d8
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 178d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 178d8 mov QWORD PTR [rbp-0x200],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe00), (r13) as u64);
            // 178df mov QWORD PTR [rbp-0x1f8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe08), (r15) as u64);
            // 178e6 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 178ed mov rsi,r13
            rsi = r13;
            // 178f0 call 0x174d0
            { let r = { let t = m.env.code_va(0x174d0); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 178f5 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 178fc call 0x63dd3
            { let r = { let t = m.env.code_va(0x63dd3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17901 mov r14,QWORD PTR [rbp-0x1a8]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 17908 lea r13,[rbp-0x1a0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 1790f cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 17912 je 0x17a34
            if fl.e() { pc = 0x17a34; continue; }
            pc = 0x17918;
        }
        0x17918 => {
            // block 17918
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17918 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17918 lea r15,[rbp-0x1d8]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fe28);
            // 1791f lea r12,[rbp-0x1c0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            pc = 0x17926;
        }
        0x17926 => {
            // block 17926
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17926 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17926 mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 1792a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1792c mov rdi,r15
            rdi = r15;
            // 1792f call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17934 mov rdi,r12
            rdi = r12;
            // 17937 mov rsi,r15
            rsi = r15;
            // 1793a call 0x174d0
            { let r = { let t = m.env.code_va(0x174d0); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1793f mov rdi,r15
            rdi = r15;
            // 17942 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17947 movzx r11d,BYTE PTR [rbp-0x1c0]
            r11 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64) as u32 as u64;
            // 1794f test r11b,0x1
            fl = logic::<8>((r11 as u8 as u64) & 0x1u64);
            // 17953 jne 0x17961
            if fl.ne() { pc = 0x17961; continue; }
            pc = 0x17955;
        }
        0x17955 => {
            // block 17955
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17955 shr r11,1
            { let (r, f) = shr::<64>(r11, 0x1u64, fl); r11 = r; fl = f; }
            // 17958 lea r9,[rbp-0x1bf]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_fe41);
            // 1795f jmp 0x1796f
            pc = 0x1796f; continue;
        }
        0x17961 => {
            // block 17961
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17961 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17961 mov r11,QWORD PTR [rbp-0x1b8]
            r11 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 17968 mov r9,QWORD PTR [rbp-0x1b0]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            pc = 0x1796f;
        }
        0x1796f => {
            // block 1796f
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1796f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1796f movzx edx,BYTE PTR [rbp-0x190]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64) as u32 as u64;
            // 17976 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 17979 jne 0x17987
            if fl.ne() { pc = 0x17987; continue; }
            pc = 0x1797b;
        }
        0x1797b => {
            // block 1797b
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1797b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1797b shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 1797e lea rsi,[rbp-0x18f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe71);
            // 17985 jmp 0x17995
            pc = 0x17995; continue;
        }
        0x17987 => {
            // block 17987
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17987 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17987 mov rdx,QWORD PTR [rbp-0x188]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 1798e mov rsi,QWORD PTR [rbp-0x180]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            pc = 0x17995;
        }
        0x17995 => {
            // block 17995
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17995 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17995 cmp r11,rdx
            fl = sub::<64>(r11, rdx).1;
            // 17998 jb 0x179fe
            if fl.b() { pc = 0x179fe; continue; }
            pc = 0x1799a;
        }
        0x1799a => {
            // block 1799a
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 1799a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1799a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 1799d je 0x17a65
            if fl.e() { pc = 0x17a65; continue; }
            pc = 0x179a3;
        }
        0x179a3 => {
            // block 179a3
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179a3 cmp r11,rdx
            fl = sub::<64>(r11, rdx).1;
            // 179a6 jl 0x179fe
            if fl.l() { pc = 0x179fe; continue; }
            pc = 0x179a8;
        }
        0x179a8 => {
            // block 179a8
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179a8 mov r10d,0x1
            r10 = (0x1u64) as u32 as u64;
            // 179ae sub r10,rdx
            { let (r, f) = sub::<64>(r10, rdx);
              r10 = r;
              fl = f; }
            // 179b1 add r10,r11
            { let (r, f) = add::<64>(r10, r11);
              r10 = r;
              fl = f; }
            // 179b4 je 0x179fe
            if fl.e() { pc = 0x179fe; continue; }
            pc = 0x179b6;
        }
        0x179b6 => {
            // block 179b6
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179b6 add r11,r9
            { let (r, f) = add::<64>(r11, r9);
              r11 = r;
              fl = f; }
            // 179b9 add r10,r9
            { let (r, f) = add::<64>(r10, r9);
              r10 = r;
              fl = f; }
            // 179bc mov r8b,BYTE PTR [rsi]
            r8 = (r8 & !0xff) | ((m.r8(rsi) as u64) as u8 as u64);
            // 179bf mov rcx,r9
            rcx = r9;
            pc = 0x179c2;
        }
        0x179c2 => {
            // block 179c2
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179c2 movzx ebx,r8b
            rbx = ((r8 as u8 as u64)) as u32 as u64;
            // 179c6 movzx edi,BYTE PTR [rcx]
            rdi = (m.r8(rcx) as u64) as u32 as u64;
            // 179c9 cmp edi,ebx
            fl = sub::<32>((rdi as u32 as u64), (rbx as u32 as u64)).1;
            // 179cb mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 179d0 jne 0x179e6
            if fl.ne() { pc = 0x179e6; continue; }
            pc = 0x179d2;
        }
        0x179d2 => {
            // block 179d2
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179d2 cmp rdx,rbx
            fl = sub::<64>(rdx, rbx).1;
            // 179d5 je 0x179f0
            if fl.e() { pc = 0x179f0; continue; }
            pc = 0x179d7;
        }
        0x179d7 => {
            // block 179d7
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179d7 movzx edi,BYTE PTR [rsi+rbx*1]
            rdi = (m.r8(rsi.wrapping_add(rbx)) as u64) as u32 as u64;
            // 179db movzx eax,BYTE PTR [rcx+rbx*1]
            rax = (m.r8(rcx.wrapping_add(rbx)) as u64) as u32 as u64;
            // 179df inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 179e2 cmp eax,edi
            fl = sub::<32>((rax as u32 as u64), (rdi as u32 as u64)).1;
            // 179e4 je 0x179d2
            if fl.e() { pc = 0x179d2; continue; }
            pc = 0x179e6;
        }
        0x179e6 => {
            // block 179e6
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179e6 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 179e9 cmp rcx,r10
            fl = sub::<64>(rcx, r10).1;
            // 179ec jne 0x179c2
            if fl.ne() { pc = 0x179c2; continue; }
            pc = 0x179ee;
        }
        0x179ee => {
            // block 179ee
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179ee jmp 0x179fe
            pc = 0x179fe; continue;
        }
        0x179f0 => {
            // block 179f0
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179f0 cmp rcx,r11
            fl = sub::<64>(rcx, r11).1;
            // 179f3 je 0x179fe
            if fl.e() { pc = 0x179fe; continue; }
            pc = 0x179f5;
        }
        0x179f5 => {
            // block 179f5
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179f5 sub rcx,r9
            { let (r, f) = sub::<64>(rcx, r9);
              rcx = r;
              fl = f; }
            // 179f8 cmp rcx,0xffffffffffffffff
            fl = sub::<64>(rcx, 0xffff_ffff_ffff_ffffu64).1;
            // 179fc jne 0x17a65
            if fl.ne() { pc = 0x17a65; continue; }
            pc = 0x179fe;
        }
        0x179fe => {
            // block 179fe
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 179fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 179fe mov rdi,r12
            rdi = r12;
            // 17a01 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17a06 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 17a0a test rax,rax
            fl = logic::<64>(rax & rax);
            // 17a0d je 0x17a1c
            if fl.e() { pc = 0x17a1c; continue; }
            pc = 0x17a0f;
        }
        0x17a0f => {
            // block 17a0f
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17a0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17a0f mov r14,rax
            r14 = rax;
            // 17a12 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 17a15 test rax,rax
            fl = logic::<64>(rax & rax);
            // 17a18 jne 0x17a0f
            if fl.ne() { pc = 0x17a0f; continue; }
            pc = 0x17a1a;
        }
        0x17a1a => {
            // block 17a1a
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17a1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17a1a jmp 0x17a2b
            pc = 0x17a2b; continue;
        }
        0x17a1c => {
            // block 17a1c
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17a1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17a1c mov rax,r14
            rax = r14;
            pc = 0x17a1f;
        }
        0x17a1f => {
            // block 17a1f
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17a1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17a1f mov r14,QWORD PTR [rax+0x10]
            r14 = m.r64(rax.wrapping_add(0x10)) as u64;
            // 17a23 cmp QWORD PTR [r14],rax
            fl = sub::<64>(m.r64(r14) as u64, rax).1;
            // 17a26 mov rax,r14
            rax = r14;
            // 17a29 jne 0x17a1f
            if fl.ne() { pc = 0x17a1f; continue; }
            pc = 0x17a2b;
        }
        0x17a2b => {
            // block 17a2b
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17a2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17a2b cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 17a2e jne 0x17926
            if fl.ne() { pc = 0x17926; continue; }
            pc = 0x17a34;
        }
        0x17a34 => {
            // block 17a34
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17a34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17a34 lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 17a3b mov rsi,QWORD PTR [rbp-0x200]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe00)) as u64;
            // 17a42 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17a47 lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 17a4e mov rsi,QWORD PTR [rbp-0x1f8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe08)) as u64;
            // 17a55 call 0x17332
            { let r = { let t = m.env.code_va(0x17332); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17a5a mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17a5c lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 17a63 jmp 0x17a7e
            pc = 0x17a7e; continue;
        }
        0x17a65 => {
            // block 17a65
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17a65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17a65 mov edi,DWORD PTR [r14+0x1c]
            rdi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 17a69 mov rsi,QWORD PTR [rbp-0x1f8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe08)) as u64;
            // 17a70 call 0x1729c
            { let r = { let t = m.env.code_va(0x1729c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17a75 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17a77 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            pc = 0x17a7e;
        }
        0x17a7e => {
            // block 17a7e
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17a7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17a7e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17a83 mov rsi,QWORD PTR [rbp-0x1a0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 17a8a lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 17a91 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17a96 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 17a9d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17aa2;
        }
        0x17aa2 => {
            // block 17aa2
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17aa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17aa2 and bl,0x1
            { let r = (rbx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rbx = (rbx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 17aa5 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 17aa7 add rsp,0x1d8
            { let (r, f) = add::<64>(rsp, 0x1d8u64);
              rsp = r;
              fl = f; }
            // 17aae pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 17aaf pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 17ab1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 17ab3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 17ab5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 17ab7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17ab8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x17ab9 => {
            // block 17ab9
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17ab9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17ab9 mov rbx,rax
            rbx = rax;
            // 17abc jmp 0x17acd
            pc = 0x17acd; continue;
        }
        0x17abe => {
            // block 17abe
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17abe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17abe mov rbx,rax
            rbx = rax;
            // 17ac1 lea rdi,[rbp-0x1d8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe28);
            pc = 0x17ac8;
        }
        0x17ac8 => {
            // block 17ac8
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17ac8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17ac8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17acd;
        }
        0x17acd => {
            // block 17acd
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17acd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17acd mov rsi,QWORD PTR [rbp-0x1a0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 17ad4 lea rdi,[rbp-0x1a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // 17adb call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17ae0;
        }
        0x17ae0 => {
            // block 17ae0
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17ae0 lea rdi,[rbp-0x190]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe70);
            // 17ae7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17aec;
        }
        0x17aec => {
            // block 17aec
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17aec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17aec mov rdi,rbx
            rdi = rbx;
            // 17aef call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17af4;
        }
        0x17af4 => {
            // block 17af4
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17af4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17af4 mov rbx,rax
            rbx = rax;
            // 17af7 lea rax,[rip+0x12991a] # 0x141418
            rax = m.env.data_va(351205);
            // 17afe mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // 17b05 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 17b0c call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17b11 jmp 0x17aec
            pc = 0x17aec; continue;
        }
        0x17b13 => {
            // block 17b13
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b13 mov rbx,rax
            rbx = rax;
            // 17b16 jmp 0x17ae0
            pc = 0x17ae0; continue;
        }
        0x17b18 => {
            // block 17b18
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b18 jmp 0x17ab9
            pc = 0x17ab9; continue;
        }
        0x17b1a => {
            // block 17b1a
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b1a mov rbx,rax
            rbx = rax;
            // 17b1d lea rdi,[rbp-0x1f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe10);
            // 17b24 jmp 0x17ac8
            pc = 0x17ac8; continue;
        }
        0x17b26 => {
            // block 17b26
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b26 mov rbx,rax
            rbx = rax;
            // 17b29 mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            // 17b30 mov QWORD PTR [rbp-0xc8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (r14) as u64);
            // 17b37 mov QWORD PTR [rbp-0x130],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (r13) as u64);
            // 17b3e jmp 0x17b4d
            pc = 0x17b4d; continue;
        }
        0x17b40 => {
            // block 17b40
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b40 mov rbx,rax
            rbx = rax;
            // 17b43 jmp 0x17b7c
            pc = 0x17b7c; continue;
        }
        0x17b45 => {
            // block 17b45
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b45 mov rbx,rax
            rbx = rax;
            // 17b48 jmp 0x17b65
            pc = 0x17b65; continue;
        }
        0x17b4a => {
            // block 17b4a
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b4a mov rbx,rax
            rbx = rax;
            pc = 0x17b4d;
        }
        0x17b4d => {
            // block 17b4d
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b4d lea rdi,[rbp-0xf0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 17b54 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17b59 lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 17b60 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17b65;
        }
        0x17b65 => {
            // block 17b65
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b65 mov rsi,QWORD PTR [rip+0x128694] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // 17b6c add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 17b70 lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 17b77 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17b7c;
        }
        0x17b7c => {
            // block 17b7c
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b7c lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // 17b83 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17b88 jmp 0x17aec
            pc = 0x17aec; continue;
        }
        0x17b8d => {
            // block 17b8d
            #[cfg(feature = "trace")] eprintln!("f_00017542 pc 17b8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b8d mov rbx,rax
            rbx = rax;
            // 17b90 lea rdi,[rbp-0x1c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // 17b97 jmp 0x17ac8
            pc = 0x17ac8; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x26172 size=1682
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00026172<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x26172;
    loop {
        match pc {
        0x26172 => {
            // block 26172
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26172 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26172 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 26173 mov rbp,rsp
            rbp = rsp;
            // 26176 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 26178 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 26179 sub rsp,0x150
            { let (r, f) = sub::<64>(rsp, 0x150u64);
              rsp = r;
              fl = f; }
            // 26180 mov r14,rsi
            r14 = rsi;
            // 26183 mov rbx,rdi
            rbx = rdi;
            // 26186 lea rsi,[rip+0x1042c9] # 0x12a456
            rsi = m.env.data_va(257059);
            // 2618d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 26192 mov rdi,r14
            rdi = r14;
            // 26195 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2619a cmp DWORD PTR [rbx],0x4e545632
            fl = sub::<32>(m.r32(rbx) as u64, 0x4e545632u64).1;
            // 261a0 jne 0x261e2
            if fl.ne() { pc = 0x261e2; continue; }
            pc = 0x261a2;
        }
        0x261a2 => {
            // block 261a2
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 261a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 261a2 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 261a6 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 261ab mov edx,0x4e
            rdx = (0x4eu64) as u32 as u64;
            // 261b0 call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 261b5 movsx edx,BYTE PTR [rbx+0x2]
            rdx = (sx::<8>(m.r8(rbx.wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // 261b9 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 261c0 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 261c5 call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 261ca movzx edx,BYTE PTR [rbp-0x88]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // 261d1 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 261d4 jne 0x2622f
            if fl.ne() { pc = 0x2622f; continue; }
            pc = 0x261d6;
        }
        0x261d6 => {
            // block 261d6
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 261d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 261d6 lea rsi,[rbp-0x87]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff79);
            // 261dd shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 261e0 jmp 0x26237
            pc = 0x26237; continue;
        }
        0x261e2 => {
            // block 261e2
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 261e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 261e2 lea rsi,[rip+0x10426f] # 0x12a458
            rsi = m.env.data_va(257061);
            // 261e9 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 261ee mov rdi,r14
            rdi = r14;
            // 261f1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 261f6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 261f9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 261fd mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26201 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26204 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26207 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2620b mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 2620d mov rdi,rax
            rdi = rax;
            // 26210 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26215 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26218 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2621c mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26220 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26223 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26226 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2622a jmp 0x263bb
            pc = 0x263bb; continue;
        }
        0x2622f => {
            // block 2622f
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2622f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2622f mov rdx,QWORD PTR [rbp-0x80]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 26233 mov rsi,QWORD PTR [rbp-0x78]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            pc = 0x26237;
        }
        0x26237 => {
            // block 26237
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26237 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26237 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 2623b call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26240 mov rcx,QWORD PTR [rax+0x10]
            rcx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 26244 mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 26248 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2624b mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2624f mov QWORD PTR [rbp-0x50],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdx) as u64);
            // 26253 mov QWORD PTR [rbp-0x58],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rcx) as u64);
            // 26257 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2625f mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 26267 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 2626e movsx edx,BYTE PTR [rbx+0x1]
            rdx = (sx::<8>(m.r8(rbx.wrapping_add(0x1)) as u64) as u64) as u32 as u64;
            // 26272 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 26279 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 2627e call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26283 movzx edx,BYTE PTR [rbp-0xa0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64) as u32 as u64;
            // 2628a test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 2628d jne 0x2629b
            if fl.ne() { pc = 0x2629b; continue; }
            pc = 0x2628f;
        }
        0x2628f => {
            // block 2628f
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2628f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2628f lea rsi,[rbp-0x9f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff61);
            // 26296 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 26299 jmp 0x262a9
            pc = 0x262a9; continue;
        }
        0x2629b => {
            // block 2629b
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2629b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2629b mov rdx,QWORD PTR [rbp-0x98]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 262a2 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            pc = 0x262a9;
        }
        0x262a9 => {
            // block 262a9
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 262a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 262a9 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 262ad call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 262b2 mov rcx,QWORD PTR [rax+0x10]
            rcx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 262b6 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // 262ba mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 262bd mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 262c1 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 262c5 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 262c9 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 262d1 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 262d9 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 262e0 movsx edx,BYTE PTR [rbx]
            rdx = (sx::<8>(m.r8(rbx) as u64) as u64) as u32 as u64;
            // 262e3 lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 262ea mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 262ef call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 262f4 movzx edx,BYTE PTR [rbp-0xb8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // 262fb test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 262fe jne 0x2630c
            if fl.ne() { pc = 0x2630c; continue; }
            pc = 0x26300;
        }
        0x26300 => {
            // block 26300
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26300 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26300 lea rsi,[rbp-0xb7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff49);
            // 26307 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2630a jmp 0x2631a
            pc = 0x2631a; continue;
        }
        0x2630c => {
            // block 2630c
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2630c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2630c mov rdx,QWORD PTR [rbp-0xb0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 26313 mov rsi,QWORD PTR [rbp-0xa8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            pc = 0x2631a;
        }
        0x2631a => {
            // block 2631a
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2631a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2631a lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2631e call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26323 mov rcx,QWORD PTR [rax+0x10]
            rcx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 26327 mov QWORD PTR [rbp-0x18],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (rcx) as u64);
            // 2632b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2632e mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 26332 mov QWORD PTR [rbp-0x20],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (rdx) as u64);
            // 26336 mov QWORD PTR [rbp-0x28],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rcx) as u64);
            // 2633a mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 26342 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2634a mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 26351 movzx edx,BYTE PTR [rbp-0x28]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 26355 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 26358 jne 0x26363
            if fl.ne() { pc = 0x26363; continue; }
            pc = 0x2635a;
        }
        0x2635a => {
            // block 2635a
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2635a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2635a lea rsi,[rbp-0x27]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd9);
            // 2635e shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 26361 jmp 0x2636b
            pc = 0x2636b; continue;
        }
        0x26363 => {
            // block 26363
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26363 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26363 mov rdx,QWORD PTR [rbp-0x20]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 26367 mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            pc = 0x2636b;
        }
        0x2636b => {
            // block 2636b
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2636b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2636b mov rdi,r14
            rdi = r14;
            // 2636e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26373 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 26377 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2637c lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 26383 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26388 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2638c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26391 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 26398 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2639d lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 263a1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 263a6 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 263ad call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 263b2 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 263b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x263bb;
        }
        0x263bb => {
            // block 263bb
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 263bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 263bb mov edx,DWORD PTR [rbx+0x4]
            rdx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 263be cmp edx,0x72656751
            fl = sub::<32>((rdx as u32 as u64), 0x72656751u64).1;
            // 263c4 jg 0x263d3
            if fl.g() { pc = 0x263d3; continue; }
            pc = 0x263c6;
        }
        0x263c6 => {
            // block 263c6
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 263c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 263c6 cmp edx,0x626e6b53
            fl = sub::<32>((rdx as u32 as u64), 0x626e6b53u64).1;
            // 263cc je 0x26446
            if fl.e() { pc = 0x26446; continue; }
            pc = 0x263ce;
        }
        0x263ce => {
            // block 263ce
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 263ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 263ce jmp 0x26487
            pc = 0x26487; continue;
        }
        0x263d3 => {
            // block 263d3
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 263d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 263d3 cmp edx,0x7461736a
            fl = sub::<32>((rdx as u32 as u64), 0x7461736au64).1;
            // 263d9 jg 0x26400
            if fl.g() { pc = 0x26400; continue; }
            pc = 0x263db;
        }
        0x263db => {
            // block 263db
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 263db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 263db cmp edx,0x73746d6f
            fl = sub::<32>((rdx as u32 as u64), 0x73746d6fu64).1;
            // 263e1 jg 0x2641a
            if fl.g() { pc = 0x2641a; continue; }
            pc = 0x263e3;
        }
        0x263e3 => {
            // block 263e3
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 263e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 263e3 cmp edx,0x73646952
            fl = sub::<32>((rdx as u32 as u64), 0x73646952u64).1;
            // 263e9 jg 0x26436
            if fl.g() { pc = 0x26436; continue; }
            pc = 0x263eb;
        }
        0x263eb => {
            // block 263eb
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 263eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 263eb cmp edx,0x72656752
            fl = sub::<32>((rdx as u32 as u64), 0x72656752u64).1;
            // 263f1 je 0x26446
            if fl.e() { pc = 0x26446; continue; }
            pc = 0x263f3;
        }
        0x263f3 => {
            // block 263f3
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 263f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 263f3 cmp edx,0x72656757
            fl = sub::<32>((rdx as u32 as u64), 0x72656757u64).1;
            // 263f9 je 0x26446
            if fl.e() { pc = 0x26446; continue; }
            pc = 0x263fb;
        }
        0x263fb => {
            // block 263fb
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 263fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 263fb jmp 0x26487
            pc = 0x26487; continue;
        }
        0x26400 => {
            // block 26400
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26400 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26400 cmp edx,0x78666571
            fl = sub::<32>((rdx as u32 as u64), 0x78666571u64).1;
            // 26406 jg 0x26424
            if fl.g() { pc = 0x26424; continue; }
            pc = 0x26408;
        }
        0x26408 => {
            // block 26408
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26408 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26408 cmp edx,0x7461736b
            fl = sub::<32>((rdx as u32 as u64), 0x7461736bu64).1;
            // 2640e je 0x26446
            if fl.e() { pc = 0x26446; continue; }
            pc = 0x26410;
        }
        0x26410 => {
            // block 26410
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26410 cmp edx,0x76646174
            fl = sub::<32>((rdx as u32 as u64), 0x76646174u64).1;
            // 26416 je 0x26446
            if fl.e() { pc = 0x26446; continue; }
            pc = 0x26418;
        }
        0x26418 => {
            // block 26418
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26418 jmp 0x26487
            pc = 0x26487; continue;
        }
        0x2641a => {
            // block 2641a
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2641a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2641a cmp edx,0x73746d70
            fl = sub::<32>((rdx as u32 as u64), 0x73746d70u64).1;
            // 26420 je 0x26446
            if fl.e() { pc = 0x26446; continue; }
            pc = 0x26422;
        }
        0x26422 => {
            // block 26422
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26422 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26422 jmp 0x26487
            pc = 0x26487; continue;
        }
        0x26424 => {
            // block 26424
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26424 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26424 cmp edx,0x78666572
            fl = sub::<32>((rdx as u32 as u64), 0x78666572u64).1;
            // 2642a je 0x26446
            if fl.e() { pc = 0x26446; continue; }
            pc = 0x2642c;
        }
        0x2642c => {
            // block 2642c
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2642c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2642c cmp edx,0x78667374
            fl = sub::<32>((rdx as u32 as u64), 0x78667374u64).1;
            // 26432 jne 0x26487
            if fl.ne() { pc = 0x26487; continue; }
            pc = 0x26434;
        }
        0x26434 => {
            // block 26434
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26434 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26434 jmp 0x26446
            pc = 0x26446; continue;
        }
        0x26436 => {
            // block 26436
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26436 cmp edx,0x73646953
            fl = sub::<32>((rdx as u32 as u64), 0x73646953u64).1;
            // 2643c je 0x26446
            if fl.e() { pc = 0x26446; continue; }
            pc = 0x2643e;
        }
        0x2643e => {
            // block 2643e
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2643e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2643e cmp edx,0x73746174
            fl = sub::<32>((rdx as u32 as u64), 0x73746174u64).1;
            // 26444 jne 0x26487
            if fl.ne() { pc = 0x26487; continue; }
            pc = 0x26446;
        }
        0x26446 => {
            // block 26446
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26446 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26446 sar edx,0x18
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x18u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 26449 lea rdi,[rbp-0x118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 26450 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 26455 call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2645a movsx edx,BYTE PTR [rbx+0x6]
            rdx = (sx::<8>(m.r8(rbx.wrapping_add(0x6)) as u64) as u64) as u32 as u64;
            // 2645e lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 26465 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 2646a call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2646f movzx edx,BYTE PTR [rbp-0x130]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64) as u32 as u64;
            // 26476 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 26479 jne 0x264d5
            if fl.ne() { pc = 0x264d5; continue; }
            pc = 0x2647b;
        }
        0x2647b => {
            // block 2647b
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2647b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2647b lea rsi,[rbp-0x12f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fed1);
            // 26482 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 26485 jmp 0x264e3
            pc = 0x264e3; continue;
        }
        0x26487 => {
            // block 26487
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26487 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26487 lea rsi,[rip+0x103fcf] # 0x12a45d
            rsi = m.env.data_va(257066);
            // 2648e mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 26493 mov rdi,r14
            rdi = r14;
            // 26496 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2649b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2649e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 264a2 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 264a6 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 264a9 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 264ac mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 264b0 mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 264b3 mov rdi,rax
            rdi = rax;
            // 264b6 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 264bb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 264be mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 264c2 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 264c6 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 264c9 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 264cc mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 264d0 jmp 0x266a4
            pc = 0x266a4; continue;
        }
        0x264d5 => {
            // block 264d5
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 264d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 264d5 mov rdx,QWORD PTR [rbp-0x128]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 264dc mov rsi,QWORD PTR [rbp-0x120]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            pc = 0x264e3;
        }
        0x264e3 => {
            // block 264e3
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 264e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 264e3 lea rdi,[rbp-0x118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 264ea call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 264ef mov rcx,QWORD PTR [rax+0x10]
            rcx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 264f3 mov QWORD PTR [rbp-0xf0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rcx) as u64);
            // 264fa mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 264fd mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 26501 mov QWORD PTR [rbp-0xf8],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rdx) as u64);
            // 26508 mov QWORD PTR [rbp-0x100],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rcx) as u64);
            // 2650f mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 26517 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2651f mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 26526 movsx edx,BYTE PTR [rbx+0x5]
            rdx = (sx::<8>(m.r8(rbx.wrapping_add(0x5)) as u64) as u64) as u32 as u64;
            // 2652a lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 26531 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 26536 call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2653b movzx edx,BYTE PTR [rbp-0x148]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64) as u32 as u64;
            // 26542 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 26545 jne 0x26553
            if fl.ne() { pc = 0x26553; continue; }
            pc = 0x26547;
        }
        0x26547 => {
            // block 26547
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26547 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26547 lea rsi,[rbp-0x147]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb9);
            // 2654e shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 26551 jmp 0x26561
            pc = 0x26561; continue;
        }
        0x26553 => {
            // block 26553
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26553 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26553 mov rdx,QWORD PTR [rbp-0x140]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 2655a mov rsi,QWORD PTR [rbp-0x138]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            pc = 0x26561;
        }
        0x26561 => {
            // block 26561
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26561 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26561 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 26568 call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2656d mov rcx,QWORD PTR [rax+0x10]
            rcx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 26571 mov QWORD PTR [rbp-0xd8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rcx) as u64);
            // 26578 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2657b mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2657f mov QWORD PTR [rbp-0xe0],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rdx) as u64);
            // 26586 mov QWORD PTR [rbp-0xe8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rcx) as u64);
            // 2658d mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 26595 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2659d mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 265a4 movsx edx,BYTE PTR [rbx+0x4]
            rdx = (sx::<8>(m.r8(rbx.wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // 265a8 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 265af mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 265b4 call 0xebff0
            { let r = { let t = m.env.stub_va(0x7bd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 265b9 movzx edx,BYTE PTR [rbp-0x160]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64) as u32 as u64;
            // 265c0 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 265c3 jne 0x265d1
            if fl.ne() { pc = 0x265d1; continue; }
            pc = 0x265c5;
        }
        0x265c5 => {
            // block 265c5
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 265c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 265c5 lea rsi,[rbp-0x15f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea1);
            // 265cc shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 265cf jmp 0x265df
            pc = 0x265df; continue;
        }
        0x265d1 => {
            // block 265d1
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 265d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 265d1 mov rdx,QWORD PTR [rbp-0x158]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 265d8 mov rsi,QWORD PTR [rbp-0x150]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            pc = 0x265df;
        }
        0x265df => {
            // block 265df
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 265df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 265df lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 265e6 call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 265eb mov rcx,QWORD PTR [rax+0x10]
            rcx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 265ef mov QWORD PTR [rbp-0xc0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rcx) as u64);
            // 265f6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 265f9 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 265fd mov QWORD PTR [rbp-0xc8],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (rdx) as u64);
            // 26604 mov QWORD PTR [rbp-0xd0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rcx) as u64);
            // 2660b mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 26613 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2661b mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 26622 movzx edx,BYTE PTR [rbp-0xd0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff30)) as u64) as u32 as u64;
            // 26629 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 2662c jne 0x2663a
            if fl.ne() { pc = 0x2663a; continue; }
            pc = 0x2662e;
        }
        0x2662e => {
            // block 2662e
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2662e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2662e lea rsi,[rbp-0xcf]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff31);
            // 26635 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 26638 jmp 0x26648
            pc = 0x26648; continue;
        }
        0x2663a => {
            // block 2663a
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2663a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2663a mov rdx,QWORD PTR [rbp-0xc8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff38)) as u64;
            // 26641 mov rsi,QWORD PTR [rbp-0xc0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            pc = 0x26648;
        }
        0x26648 => {
            // block 26648
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26648 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26648 mov rdi,r14
            rdi = r14;
            // 2664b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26650 lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // 26657 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2665c lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 26663 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26668 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 2666f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26674 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 2667b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26680 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 26687 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2668c lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 26693 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26698 lea rdi,[rbp-0x118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 2669f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x266a4;
        }
        0x266a4 => {
            // block 266a4
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 266a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 266a4 lea rsi,[rip+0x103db8] # 0x12a463
            rsi = m.env.data_va(257072);
            // 266ab mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 266b0 mov rdi,r14
            rdi = r14;
            // 266b3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 266b8 mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 266bb mov rdi,rax
            rdi = rax;
            // 266be call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 266c3 lea rsi,[rip+0x103d9c] # 0x12a466
            rsi = m.env.data_va(257075);
            // 266ca mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 266cf mov rdi,rax
            rdi = rax;
            // 266d2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 266d7 mov esi,DWORD PTR [rbx+0xc]
            rsi = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // 266da mov rdi,rax
            rdi = rax;
            // 266dd call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 266e2 lea rsi,[rip+0x103d84] # 0x12a46d
            rsi = m.env.data_va(257082);
            // 266e9 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 266ee mov rdi,rax
            rdi = rax;
            // 266f1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 266f6 mov esi,DWORD PTR [rbx+0x10]
            rsi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 266f9 mov rdi,rax
            rdi = rax;
            // 266fc call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26701 lea rsi,[rip+0x103d4c] # 0x12a454
            rsi = m.env.data_va(257057);
            // 26708 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2670d mov rdi,r14
            rdi = r14;
            // 26710 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26715 add rsp,0x150
            { let (r, f) = add::<64>(rsp, 0x150u64);
              rsp = r;
              fl = f; }
            // 2671c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2671d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2671f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 26720 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x26721 => {
            // block 26721
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26721 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26721 mov rbx,rax
            rbx = rax;
            // 26724 jmp 0x2678a
            pc = 0x2678a; continue;
        }
        0x26726 => {
            // block 26726
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26726 mov rbx,rax
            rbx = rax;
            // 26729 jmp 0x2677e
            pc = 0x2677e; continue;
        }
        0x2672b => {
            // block 2672b
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2672b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2672b mov rbx,rax
            rbx = rax;
            // 2672e jmp 0x26772
            pc = 0x26772; continue;
        }
        0x26730 => {
            // block 26730
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26730 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26730 mov rbx,rax
            rbx = rax;
            // 26733 jmp 0x26766
            pc = 0x26766; continue;
        }
        0x26735 => {
            // block 26735
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26735 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26735 mov rbx,rax
            rbx = rax;
            // 26738 jmp 0x2675a
            pc = 0x2675a; continue;
        }
        0x2673a => {
            // block 2673a
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2673a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2673a mov rbx,rax
            rbx = rax;
            // 2673d jmp 0x2674e
            pc = 0x2674e; continue;
        }
        0x2673f => {
            // block 2673f
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2673f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2673f mov rbx,rax
            rbx = rax;
            // 26742 lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // 26749 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2674e;
        }
        0x2674e => {
            // block 2674e
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2674e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2674e lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 26755 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2675a;
        }
        0x2675a => {
            // block 2675a
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2675a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2675a lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 26761 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26766;
        }
        0x26766 => {
            // block 26766
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26766 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26766 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 2676d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x26772;
        }
        0x26772 => {
            // block 26772
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26772 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26772 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 26779 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2677e;
        }
        0x2677e => {
            // block 2677e
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2677e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2677e lea rdi,[rbp-0x130]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 26785 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2678a;
        }
        0x2678a => {
            // block 2678a
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2678a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2678a lea rdi,[rbp-0x118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 26791 jmp 0x267f7
            pc = 0x267f7; continue;
        }
        0x26793 => {
            // block 26793
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26793 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26793 mov rbx,rax
            rbx = rax;
            // 26796 jmp 0x267f3
            pc = 0x267f3; continue;
        }
        0x26798 => {
            // block 26798
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 26798 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26798 mov rbx,rax
            rbx = rax;
            // 2679b jmp 0x267e7
            pc = 0x267e7; continue;
        }
        0x2679d => {
            // block 2679d
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 2679d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2679d mov rbx,rax
            rbx = rax;
            // 267a0 jmp 0x267de
            pc = 0x267de; continue;
        }
        0x267a2 => {
            // block 267a2
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267a2 mov rbx,rax
            rbx = rax;
            // 267a5 jmp 0x267d2
            pc = 0x267d2; continue;
        }
        0x267a7 => {
            // block 267a7
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267a7 mov rbx,rax
            rbx = rax;
            // 267aa jmp 0x267c9
            pc = 0x267c9; continue;
        }
        0x267ac => {
            // block 267ac
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267ac mov rbx,rax
            rbx = rax;
            // 267af jmp 0x267bd
            pc = 0x267bd; continue;
        }
        0x267b1 => {
            // block 267b1
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267b1 mov rbx,rax
            rbx = rax;
            // 267b4 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 267b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x267bd;
        }
        0x267bd => {
            // block 267bd
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267bd lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 267c4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x267c9;
        }
        0x267c9 => {
            // block 267c9
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267c9 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 267cd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x267d2;
        }
        0x267d2 => {
            // block 267d2
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267d2 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 267d9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x267de;
        }
        0x267de => {
            // block 267de
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267de lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 267e2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x267e7;
        }
        0x267e7 => {
            // block 267e7
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267e7 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 267ee call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x267f3;
        }
        0x267f3 => {
            // block 267f3
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267f3 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            pc = 0x267f7;
        }
        0x267f7 => {
            // block 267f7
            #[cfg(feature = "trace")] eprintln!("f_00026172 pc 267f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267f7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 267fc mov rdi,rbx
            rdi = rbx;
            // 267ff call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xde198 size=1690
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000de198<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xde198;
    loop {
        match pc {
        0xde198 => {
            // block de198
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de198 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de198 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // de199 mov rbp,rsp
            rbp = rsp;
            // de19c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // de19e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // de1a0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // de1a2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // de1a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // de1a5 sub rsp,0xa0e8
            { let (r, f) = sub::<64>(rsp, 0xa0e8u64);
              rsp = r;
              fl = f; }
            // de1ac mov rbx,rdi
            rbx = rdi;
            // de1af mov rax,QWORD PTR [rip+0x62222] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // de1b6 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // de1b9 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // de1bd mov QWORD PTR [rbp-0xa038],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5fc8), (0x0u64) as u64);
            // de1c8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de1cb lea rsi,[rbp-0xa03c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fc4);
            // de1d2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // de1d4 call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de1da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de1dd lea rsi,[rbp-0xa040]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fc0);
            // de1e4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // de1e6 mov rdi,rbx
            rdi = rbx;
            // de1e9 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de1ef mov r15d,DWORD PTR [rbp-0xa040]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5fc0)) as u64) as u32 as u64;
            // de1f6 cmp r15d,0xa
            fl = sub::<32>((r15 as u32 as u64), 0xau64).1;
            // de1fa sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // de1fe mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de201 lea rsi,[rbp-0xa041]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fbf);
            // de208 lea rdx,[rbp-0xa042]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5fbe);
            // de20f xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // de211 mov rdi,rbx
            rdi = rbx;
            // de214 call QWORD PTR [rax+0x5e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x5e0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de21a mov esi,DWORD PTR [rbp-0xa03c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5fc4)) as u64) as u32 as u64;
            // de220 mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // de226 movzx r9d,BYTE PTR [rbp-0xa042]
            r9 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_5fbe)) as u64) as u32 as u64;
            // de22e movzx ecx,BYTE PTR [rbp-0xa041]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_5fbf)) as u64) as u32 as u64;
            // de235 movzx r8d,r14b
            r8 = ((r14 as u8 as u64)) as u32 as u64;
            // de239 lea rdi,[rbp-0xa080]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5f80);
            // de240 call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de245 cmp r15d,0xa
            fl = sub::<32>((r15 as u32 as u64), 0xau64).1;
            // de249 mov eax,DWORD PTR [rbp-0xa07c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f84)) as u64) as u32 as u64;
            // de24f mov DWORD PTR [rbp-0xa084],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c), ((rax as u32 as u64)) as u32);
            // de255 mov eax,DWORD PTR [rbp-0xa078]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f88)) as u64) as u32 as u64;
            // de25b mov DWORD PTR [rbp-0xa088],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f78), ((rax as u32 as u64)) as u32);
            // de261 mov eax,DWORD PTR [rbp-0xa080]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f80)) as u64) as u32 as u64;
            // de267 mov DWORD PTR [rbp-0xa0f0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f10), ((rax as u32 as u64)) as u32);
            // de26d mov r14d,DWORD PTR [rbp-0xa074]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f8c)) as u64) as u32 as u64;
            // de274 jne 0xde290
            if fl.ne() { pc = 0xde290; continue; }
            pc = 0xde276;
        }
        0xde276 => {
            // block de276
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de276 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de279 lea rsi,[rbp-0xa084]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5f7c);
            // de280 lea rdx,[rbp-0xa088]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5f78);
            // de287 mov rdi,rbx
            rdi = rbx;
            // de28a call QWORD PTR [rax+0x1a00]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0xde290;
        }
        0xde290 => {
            // block de290
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de290 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de290 mov r12,QWORD PTR [rbx+0x280]
            r12 = m.r64(rbx.wrapping_add(0x280)) as u64;
            // de297 mov r13,rbx
            r13 = rbx;
            // de29a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // de29c mov QWORD PTR [rbp-0xa0f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f08), (rax) as u64);
            // de2a3 test r12,r12
            fl = logic::<64>(r12 & r12);
            // de2a6 jne 0xde2f8
            if fl.ne() { pc = 0xde2f8; continue; }
            pc = 0xde2a8;
        }
        0xde2a8 => {
            // block de2a8
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de2a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de2a8 cmp BYTE PTR [r13+0x279],0x0
            fl = sub::<8>(m.r8(r13.wrapping_add(0x279)) as u64, 0x0u64).1;
            // de2b0 je 0xde2cd
            if fl.e() { pc = 0xde2cd; continue; }
            pc = 0xde2b2;
        }
        0xde2b2 => {
            // block de2b2
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de2b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de2b2 mov edi,DWORD PTR [rbp-0xa088]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de2b8 imul edi,DWORD PTR [rbp-0xa0f0]
            { let (r, f) = imul::<32>((rdi as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f10)) as u64); rdi = (r) as u32 as u64; fl = f; }
            // de2bf shl rdi,0x2
            { let (r, f) = shl::<64>(rdi, 0x2u64, fl); rdi = r; fl = f; }
            // de2c3 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de2c8 mov r12,rax
            r12 = rax;
            // de2cb jmp 0xde2f1
            pc = 0xde2f1; continue;
        }
        0xde2cd => {
            // block de2cd
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de2cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de2cd mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // de2d1 mov esi,DWORD PTR [r13+0x250]
            rsi = (m.r32(r13.wrapping_add(0x250)) as u64) as u32 as u64;
            // de2d8 lea rdx,[rbp-0xa038]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5fc8);
            // de2df mov rdi,r13
            rdi = r13;
            // de2e2 call QWORD PTR [rax+0xa10]
            { let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de2e8 mov r12,QWORD PTR [rbp-0xa038]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5fc8)) as u64;
            // de2ef xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xde2f1;
        }
        0xde2f1 => {
            // block de2f1
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de2f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de2f1 mov QWORD PTR [rbp-0xa0f8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f08), (rax) as u64);
            pc = 0xde2f8;
        }
        0xde2f8 => {
            // block de2f8
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de2f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de2f8 lea rdi,[rbp-0xa030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // de2ff mov esi,0x800
            rsi = (0x800u64) as u32 as u64;
            // de304 call 0x5932c
            { let r = { let t = m.env.code_va(0x5932c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de309 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // de30c mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // de30f je 0xde364
            if fl.e() { pc = 0xde364; continue; }
            pc = 0xde311;
        }
        0xde311 => {
            // block de311
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de311 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de311 lea r14,[rbp-0xa030]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // de318 lea r15,[rbp-0x8030]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // de31f mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // de321 mov DWORD PTR [rbp-0xa098],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f68), ((rax as u32 as u64)) as u32);
            pc = 0xde327;
        }
        0xde327 => {
            // block de327
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de327 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de327 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // de32b mov ecx,DWORD PTR [rbp-0xa084]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c)) as u64) as u32 as u64;
            // de331 mov rdi,r13
            rdi = r13;
            // de334 mov rsi,r14
            rsi = r14;
            // de337 mov rdx,r15
            rdx = r15;
            // de33a call QWORD PTR [rax+0x19e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de340 mov edx,DWORD PTR [rbp-0xa088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de346 shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // de349 mov rdi,r12
            rdi = r12;
            // de34c mov rsi,r15
            rsi = r15;
            // de34f call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de354 mov eax,DWORD PTR [rbp-0xa088]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de35a lea r12,[r12+rax*4]
            r12 = r12.wrapping_add(rax.wrapping_mul(4));
            // de35e dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // de360 jne 0xde327
            if fl.ne() { pc = 0xde327; continue; }
            pc = 0xde362;
        }
        0xde362 => {
            // block de362
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de362 jmp 0xde36a
            pc = 0xde36a; continue;
        }
        0xde364 => {
            // block de364
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de364 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de364 mov DWORD PTR [rbp-0xa098],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f68), ((rax as u32 as u64)) as u32);
            pc = 0xde36a;
        }
        0xde36a => {
            // block de36a
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de36a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de36a mov r14,r13
            r14 = r13;
            // de36d mov ecx,DWORD PTR [rbp-0xa084]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c)) as u64) as u32 as u64;
            // de373 mov eax,0xcccccccd
            rax = (0xcccccccdu64) as u32 as u64;
            // de378 mov edx,DWORD PTR [rbp-0xa0f0]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f10)) as u64) as u32 as u64;
            // de37e sub edx,DWORD PTR [rbp-0xa098]
            { let (r, f) = sub::<32>((rdx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f68)) as u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // de384 mov DWORD PTR [rbp-0xa0d4],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f2c), ((rdx as u32 as u64)) as u32);
            // de38a je 0xde72f
            if fl.e() { pc = 0xde72f; continue; }
            pc = 0xde390;
        }
        0xde390 => {
            // block de390
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de390 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de390 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // de394 shr rcx,0x23
            { let (r, f) = shr::<64>(rcx, 0x23u64, fl); rcx = r; fl = f; }
            // de398 mov QWORD PTR [rbp-0xa0a0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f60), (rcx) as u64);
            // de39f mov eax,DWORD PTR [rbp-0xa0d4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f2c)) as u64) as u32 as u64;
            // de3a5 mov ecx,0xcccccccd
            rcx = (0xcccccccdu64) as u32 as u64;
            // de3aa imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // de3ae shr rcx,0x23
            { let (r, f) = shr::<64>(rcx, 0x23u64, fl); rcx = r; fl = f; }
            // de3b2 mov QWORD PTR [rbp-0xa0e0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f20), (rcx) as u64);
            // de3b9 lea edx,[rcx-0x1]
            rdx = (rcx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // de3bc mov DWORD PTR [rbp-0xa0ec],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f14), ((rdx as u32 as u64)) as u32);
            // de3c2 lea ecx,[rcx-0x2]
            rcx = (rcx.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // de3c5 mov DWORD PTR [rbp-0xa0fc],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f04), ((rcx as u32 as u64)) as u32);
            // de3cb cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // de3d0 movsd QWORD PTR [rbp-0xa0e8],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f18), x0 as u64);
            // de3d8 movsd xmm0,QWORD PTR [rip+0x24ba8] # 0x102f88
            x0 = m.r64(m.env.data_va(96085)) as u128;
            // de3e0 movsd QWORD PTR [rbp-0xa0a8],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f58), x0 as u64);
            // de3e8 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // de3ea xorpd xmm0,xmm0
            x0 = 0;
            // de3ee xorpd xmm1,xmm1
            x1 = 0;
            // de3f2 movsd QWORD PTR [rbp-0xa0d0],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f30), x1 as u64);
            // de3fa jmp 0xde470
            pc = 0xde470; continue;
        }
        0xde3fc => {
            // block de3fc
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de3fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de3fc mov QWORD PTR [rbp-0xa0b8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f48), (r12) as u64);
            // de403 cmp edx,0x2
            fl = sub::<32>((rdx as u32 as u64), 0x2u64).1;
            // de406 movzx eax,WORD PTR [r8+0x270]
            rax = (m.r16(r8.wrapping_add(0x270)) as u64) as u32 as u64;
            // de40e je 0xde443
            if fl.e() { pc = 0xde443; continue; }
            pc = 0xde410;
        }
        0xde410 => {
            // block de410
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de410 cmp edx,DWORD PTR [rbp-0xa0fc]
            fl = sub::<32>((rdx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f04)) as u64).1;
            // de416 je 0xde443
            if fl.e() { pc = 0xde443; continue; }
            pc = 0xde418;
        }
        0xde418 => {
            // block de418
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de418 movsd QWORD PTR [rbp-0xa0c8],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f38), x0 as u64);
            // de420 mov DWORD PTR [rbp-0xa0bc],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f44), ((rsi as u32 as u64)) as u32);
            // de426 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // de428 mov r9w,0x1f6
            r9 = (r9 & !0xffff) | ((0x1f6u64) as u16 as u64);
            // de42d mov ax,0x40
            rax = (rax & !0xffff) | ((0x40u64) as u16 as u64);
            // de431 cmovne r9w,ax
            if fl.ne() { r9 = (r9 & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            // de436 mov si,0x200
            rsi = (rsi & !0xffff) | ((0x200u64) as u16 as u64);
            // de43a mov di,0x200
            rdi = (rdi & !0xffff) | ((0x200u64) as u16 as u64);
            // de43e jmp 0xde52f
            pc = 0xde52f; continue;
        }
        0xde443 => {
            // block de443
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de443 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de443 movsd QWORD PTR [rbp-0xa0c8],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f38), x0 as u64);
            // de44b mov DWORD PTR [rbp-0xa0bc],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f44), ((rsi as u32 as u64)) as u32);
            // de451 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // de453 mov r9w,0x1f6
            r9 = (r9 & !0xffff) | ((0x1f6u64) as u16 as u64);
            // de458 mov cx,0x104
            rcx = (rcx & !0xffff) | ((0x104u64) as u16 as u64);
            // de45c cmovne r9w,cx
            if fl.ne() { r9 = (r9 & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            // de461 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // de463 mov di,0x200
            rdi = (rdi & !0xffff) | ((0x200u64) as u16 as u64);
            // de467 mov cx,0x26c
            rcx = (rcx & !0xffff) | ((0x26cu64) as u16 as u64);
            // de46b jmp 0xde521
            pc = 0xde521; continue;
        }
        0xde470 => {
            // block de470
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de470 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de470 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // de472 je 0xde490
            if fl.e() { pc = 0xde490; continue; }
            pc = 0xde474;
        }
        0xde474 => {
            // block de474
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de474 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de474 movapd xmm1,xmm0
            x1 = x0;
            // de478 divsd xmm1,QWORD PTR [rbp-0xa0e8]
            x1 = set_lo64(x1, (f64_of(x1) / f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f18)) as u128))).to_bits());
            // de480 mulsd xmm1,QWORD PTR [rip+0x24b08] # 0x102f90
            x1 = set_lo64(x1, (f64_of(x1) * f64_of((m.r64(m.env.data_va(96093)) as u128))).to_bits());
            // de488 movsd QWORD PTR [rbp-0xa0d0],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f30), x1 as u64);
            pc = 0xde490;
        }
        0xde490 => {
            // block de490
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de490 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de490 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // de492 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // de494 mov rcx,QWORD PTR [rbp-0xa0e0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f20)) as u64;
            // de49b div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // de49d cmp edx,0x1
            fl = sub::<32>((rdx as u32 as u64), 0x1u64).1;
            // de4a0 mov r8,r14
            r8 = r14;
            // de4a3 je 0xde4ea
            if fl.e() { pc = 0xde4ea; continue; }
            pc = 0xde4a5;
        }
        0xde4a5 => {
            // block de4a5
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de4a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de4a5 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // de4a7 jne 0xde4de
            if fl.ne() { pc = 0xde4de; continue; }
            pc = 0xde4a9;
        }
        0xde4a9 => {
            // block de4a9
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de4a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de4a9 movsd QWORD PTR [rbp-0xa0c8],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f38), x0 as u64);
            // de4b1 mov DWORD PTR [rbp-0xa0bc],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f44), ((rsi as u32 as u64)) as u32);
            // de4b7 mov QWORD PTR [rbp-0xa0b8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f48), (r12) as u64);
            // de4be movzx eax,WORD PTR [r8+0x270]
            rax = (m.r16(r8.wrapping_add(0x270)) as u64) as u32 as u64;
            // de4c6 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // de4c8 mov r9w,0x1f6
            r9 = (r9 & !0xffff) | ((0x1f6u64) as u16 as u64);
            // de4cd mov cx,0x3ac
            rcx = (rcx & !0xffff) | ((0x3acu64) as u16 as u64);
            // de4d1 cmovne r9w,cx
            if fl.ne() { r9 = (r9 & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            // de4d6 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // de4d8 mov di,0x200
            rdi = (rdi & !0xffff) | ((0x200u64) as u16 as u64);
            // de4dc jmp 0xde521
            pc = 0xde521; continue;
        }
        0xde4de => {
            // block de4de
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de4de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de4de cmp edx,DWORD PTR [rbp-0xa0ec]
            fl = sub::<32>((rdx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f14)) as u64).1;
            // de4e4 jne 0xde3fc
            if fl.ne() { pc = 0xde3fc; continue; }
            pc = 0xde4ea;
        }
        0xde4ea => {
            // block de4ea
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de4ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de4ea movsd QWORD PTR [rbp-0xa0c8],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f38), x0 as u64);
            // de4f2 mov DWORD PTR [rbp-0xa0bc],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f44), ((rsi as u32 as u64)) as u32);
            // de4f8 mov QWORD PTR [rbp-0xa0b8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f48), (r12) as u64);
            // de4ff movzx eax,WORD PTR [r8+0x270]
            rax = (m.r16(r8.wrapping_add(0x270)) as u64) as u32 as u64;
            // de507 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // de509 mov r9w,0x1f6
            r9 = (r9 & !0xffff) | ((0x1f6u64) as u16 as u64);
            // de50e mov cx,0x2e4
            rcx = (rcx & !0xffff) | ((0x2e4u64) as u16 as u64);
            // de512 cmovne r9w,cx
            if fl.ne() { r9 = (r9 & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            // de517 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // de519 mov di,0x200
            rdi = (rdi & !0xffff) | ((0x200u64) as u16 as u64);
            // de51d mov cx,0x352
            rcx = (rcx & !0xffff) | ((0x352u64) as u16 as u64);
            pc = 0xde521;
        }
        0xde521 => {
            // block de521
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de521 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de521 cmovne di,cx
            if fl.ne() { rdi = (rdi & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            // de525 test al,0x2
            fl = logic::<8>((rax as u8 as u64) & 0x2u64);
            // de527 mov si,0x200
            rsi = (rsi & !0xffff) | ((0x200u64) as u16 as u64);
            // de52b cmovne si,cx
            if fl.ne() { rsi = (rsi & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            pc = 0xde52f;
        }
        0xde52f => {
            // block de52f
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de52f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de52f mov WORD PTR [rbp-0xa0ae],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_5f52), ((rsi as u16 as u64)) as u16);
            // de536 mov WORD PTR [rbp-0xa0ac],di
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_5f54), ((rdi as u16 as u64)) as u16);
            // de53d mov WORD PTR [rbp-0xa0aa],r9w
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_5f56), ((r9 as u16 as u64)) as u16);
            // de545 mov eax,DWORD PTR [rbp-0xa084]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c)) as u64) as u32 as u64;
            // de54b mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // de550 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // de553 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // de555 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // de55a je 0xde6ae
            if fl.e() { pc = 0xde6ae; continue; }
            pc = 0xde560;
        }
        0xde560 => {
            // block de560
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de560 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // de563 mov eax,eax
            rax = ((rax as u32 as u64)) as u32 as u64;
            // de565 mov rdx,QWORD PTR [rbp-0xa0a0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f60)) as u64;
            // de56c cmp r12d,edx
            fl = sub::<32>((r12 as u32 as u64), (rdx as u32 as u64)).1;
            // de56f mov r15w,r9w
            r15 = (r15 & !0xffff) | (((r9 as u16 as u64)) as u16 as u64);
            // de573 mov r13w,si
            r13 = (r13 & !0xffff) | (((rsi as u16 as u64)) as u16 as u64);
            // de577 mov r14w,di
            r14 = (r14 & !0xffff) | (((rdi as u16 as u64)) as u16 as u64);
            // de57b jbe 0xde66c
            if fl.be() { pc = 0xde66c; continue; }
            pc = 0xde581;
        }
        0xde581 => {
            // block de581
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de581 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de581 cvtsi2sd xmm1,rcx
            x1 = set_lo64(x1, (sx::<64>(rcx) as f64).to_bits());
            // de586 mulsd xmm1,QWORD PTR [rip+0x24a02] # 0x102f90
            x1 = set_lo64(x1, (f64_of(x1) * f64_of((m.r64(m.env.data_va(96093)) as u128))).to_bits());
            // de58e cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // de593 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // de597 movzx eax,WORD PTR [r8+0x270]
            rax = (m.r16(r8.wrapping_add(0x270)) as u64) as u32 as u64;
            // de59f test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // de5a1 jne 0xde5b3
            if fl.ne() { pc = 0xde5b3; continue; }
            pc = 0xde5a3;
        }
        0xde5a3 => {
            // block de5a3
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de5a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de5a3 movsd QWORD PTR [rbp-0xa098],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f68), x1 as u64);
            // de5ab mov r15d,0x1f6
            r15 = (0x1f6u64) as u32 as u64;
            // de5b1 jmp 0xde5e6
            pc = 0xde5e6; continue;
        }
        0xde5b3 => {
            // block de5b3
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de5b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de5b3 mov rax,QWORD PTR [r8]
            rax = m.r64(r8) as u64;
            // de5b6 movsd xmm0,QWORD PTR [rbp-0xa0a8]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f58)) as u128;
            // de5be addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // de5c2 movsd QWORD PTR [rbp-0xa098],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f68), x1 as u64);
            // de5ca xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // de5cc mov r14,r8
            r14 = r8;
            // de5cf mov rdi,r14
            rdi = r14;
            // de5d2 call QWORD PTR [rax+0x19d8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19d8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de5d8 mov r8,r14
            r8 = r14;
            // de5db mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // de5de movzx eax,WORD PTR [r8+0x270]
            rax = (m.r16(r8.wrapping_add(0x270)) as u64) as u32 as u64;
            pc = 0xde5e6;
        }
        0xde5e6 => {
            // block de5e6
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de5e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de5e6 mov r13d,0x200
            r13 = (0x200u64) as u32 as u64;
            // de5ec test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // de5ee mov r14d,0x200
            r14 = (0x200u64) as u32 as u64;
            // de5f4 je 0xde628
            if fl.e() { pc = 0xde628; continue; }
            pc = 0xde5f6;
        }
        0xde5f6 => {
            // block de5f6
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de5f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de5f6 mov rax,QWORD PTR [r8]
            rax = m.r64(r8) as u64;
            // de5f9 movsd xmm0,QWORD PTR [rbp-0xa0a8]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f58)) as u128;
            // de601 addsd xmm0,QWORD PTR [rbp-0xa098]
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f68)) as u128))).to_bits());
            // de609 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // de60e mov r14,r8
            r14 = r8;
            // de611 mov rdi,r14
            rdi = r14;
            // de614 call QWORD PTR [rax+0x19d8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19d8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de61a mov r8,r14
            r8 = r14;
            // de61d mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // de620 movzx eax,WORD PTR [r8+0x270]
            rax = (m.r16(r8.wrapping_add(0x270)) as u64) as u32 as u64;
            pc = 0xde628;
        }
        0xde628 => {
            // block de628
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de628 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de628 test al,0x2
            fl = logic::<8>((rax as u8 as u64) & 0x2u64);
            // de62a movsd xmm0,QWORD PTR [rbp-0xa098]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f68)) as u128;
            // de632 je 0xde656
            if fl.e() { pc = 0xde656; continue; }
            pc = 0xde634;
        }
        0xde634 => {
            // block de634
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de634 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de634 mov rax,QWORD PTR [r8]
            rax = m.r64(r8) as u64;
            // de637 addsd xmm0,QWORD PTR [rbp-0xa0a8]
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f58)) as u128))).to_bits());
            // de63f mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // de644 mov r13,r8
            r13 = r8;
            // de647 mov rdi,r13
            rdi = r13;
            // de64a call QWORD PTR [rax+0x19d8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19d8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de650 mov r8,r13
            r8 = r13;
            // de653 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xde656;
        }
        0xde656 => {
            // block de656
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de656 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de656 mov r9w,WORD PTR [rbp-0xa0aa]
            r9 = (r9 & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_5f56)) as u64) as u16 as u64);
            // de65e mov di,WORD PTR [rbp-0xa0ac]
            rdi = (rdi & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_5f54)) as u64) as u16 as u64);
            // de665 mov si,WORD PTR [rbp-0xa0ae]
            rsi = (rsi & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_5f52)) as u64) as u16 as u64);
            pc = 0xde66c;
        }
        0xde66c => {
            // block de66c
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de66c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de66c mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // de66e mov WORD PTR [rbp+rax*2-0xa030],r15w
            m.w16(rbp.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((r15 as u16 as u64)) as u16);
            // de677 test r12b,0x1
            fl = logic::<8>((r12 as u8 as u64) & 0x1u64);
            // de67b jne 0xde697
            if fl.ne() { pc = 0xde697; continue; }
            pc = 0xde67d;
        }
        0xde67d => {
            // block de67d
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de67d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de67d lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // de680 mov eax,eax
            rax = ((rax as u32 as u64)) as u32 as u64;
            // de682 mov WORD PTR [rbp+rax*2-0xa030],r13w
            m.w16(rbp.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((r13 as u16 as u64)) as u16);
            // de68b lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // de68e mov WORD PTR [rbp+rax*2-0xa030],r14w
            m.w16(rbp.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((r14 as u16 as u64)) as u16);
            pc = 0xde697;
        }
        0xde697 => {
            // block de697
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de697 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de697 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // de69a mov eax,DWORD PTR [rbp-0xa084]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c)) as u64) as u32 as u64;
            // de6a0 add ebx,0x2
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x2u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // de6a3 cmp r12d,eax
            fl = sub::<32>((r12 as u32 as u64), (rax as u32 as u64)).1;
            // de6a6 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // de6a8 jb 0xde560
            if fl.b() { pc = 0xde560; continue; }
            pc = 0xde6ae;
        }
        0xde6ae => {
            // block de6ae
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de6ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de6ae mov rax,QWORD PTR [r8]
            rax = m.r64(r8) as u64;
            // de6b1 mov rdi,r8
            rdi = r8;
            // de6b4 mov r14,rdi
            r14 = rdi;
            // de6b7 lea rsi,[rbp-0xa030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // de6be lea rbx,[rbp-0x8030]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // de6c5 mov rdx,rbx
            rdx = rbx;
            // de6c8 call QWORD PTR [rax+0x19e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de6ce mov edx,DWORD PTR [rbp-0xa088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de6d4 shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // de6d7 mov r12,QWORD PTR [rbp-0xa0b8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f48)) as u64;
            // de6de mov rdi,r12
            rdi = r12;
            // de6e1 mov rsi,rbx
            rsi = rbx;
            // de6e4 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de6e9 mov eax,DWORD PTR [rbp-0xa088]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de6ef lea r12,[r12+rax*4]
            r12 = r12.wrapping_add(rax.wrapping_mul(4));
            // de6f3 movsd xmm0,QWORD PTR [rbp-0xa0a8]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f58)) as u128;
            // de6fb addsd xmm0,QWORD PTR [rbp-0xa0d0]
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f30)) as u128))).to_bits());
            // de703 movsd QWORD PTR [rbp-0xa0a8],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f58), x0 as u64);
            // de70b movsd xmm0,QWORD PTR [rbp-0xa0c8]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f38)) as u128;
            // de713 addsd xmm0,QWORD PTR [rip+0x2146d] # 0xffb88
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(82773)) as u128))).to_bits());
            // de71b mov esi,DWORD PTR [rbp-0xa0bc]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f44)) as u64) as u32 as u64;
            // de721 inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // de723 cmp esi,DWORD PTR [rbp-0xa0d4]
            fl = sub::<32>((rsi as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f2c)) as u64).1;
            // de729 jne 0xde470
            if fl.ne() { pc = 0xde470; continue; }
            pc = 0xde72f;
        }
        0xde72f => {
            // block de72f
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de72f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de72f mov r15,QWORD PTR [rbp-0xa0f8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f08)) as u64;
            // de736 test r15,r15
            fl = logic::<64>(r15 & r15);
            // de739 mov rbx,r14
            rbx = r14;
            // de73c je 0xde7b4
            if fl.e() { pc = 0xde7b4; continue; }
            pc = 0xde73e;
        }
        0xde73e => {
            // block de73e
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de73e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de73e mov al,BYTE PTR [rbx+0x279]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x279)) as u64) as u8 as u64);
            // de744 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // de746 je 0xde7b4
            if fl.e() { pc = 0xde7b4; continue; }
            pc = 0xde748;
        }
        0xde748 => {
            // block de748
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de748 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de748 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de74b mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // de751 lea rdx,[rbp-0xa08c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5f74);
            // de758 mov rdi,rbx
            rdi = rbx;
            // de75b call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de761 mov r14d,DWORD PTR [rbp-0xa08c]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f74)) as u64) as u32 as u64;
            // de768 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de76b mov rdi,rbx
            rdi = rbx;
            // de76e call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de774 cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // de777 jbe 0xde787
            if fl.be() { pc = 0xde787; continue; }
            pc = 0xde779;
        }
        0xde779 => {
            // block de779
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de779 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de779 mov DWORD PTR [rbp-0xa08c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f74), (0x0u64) as u32);
            // de783 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // de785 jmp 0xde78d
            pc = 0xde78d; continue;
        }
        0xde787 => {
            // block de787
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de787 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de787 mov esi,DWORD PTR [rbp-0xa08c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f74)) as u64) as u32 as u64;
            pc = 0xde78d;
        }
        0xde78d => {
            // block de78d
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de78d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de78d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de790 mov ecx,DWORD PTR [rbp-0xa0f0]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f10)) as u64) as u32 as u64;
            // de796 imul ecx,DWORD PTR [rbp-0xa088]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // de79d shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // de7a0 mov rdi,rbx
            rdi = rbx;
            // de7a3 mov rdx,r15
            rdx = r15;
            // de7a6 call QWORD PTR [rax+0x2c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de7ac mov rdi,r15
            rdi = r15;
            // de7af call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0xde7b4;
        }
        0xde7b4 => {
            // block de7b4
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de7b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de7b4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de7b7 movzx esi,BYTE PTR [rbx+0x278]
            rsi = (m.r8(rbx.wrapping_add(0x278)) as u64) as u32 as u64;
            // de7be mov rdi,rbx
            rdi = rbx;
            // de7c1 call QWORD PTR [rax+0x570]
            { let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de7c7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de7ca mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // de7d0 mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // de7d6 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // de7db mov rdi,rbx
            rdi = rbx;
            // de7de call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de7e4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de7e7 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // de7ed xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // de7ef mov rdi,rbx
            rdi = rbx;
            // de7f2 call QWORD PTR [rax+0x4f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de7f8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de7fb mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // de801 mov rdi,rbx
            rdi = rbx;
            // de804 call QWORD PTR [rax+0x618]
            { let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // de80a mov rax,QWORD PTR [rip+0x61bc7] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // de811 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // de814 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // de818 jne 0xde82c
            if fl.ne() { pc = 0xde82c; continue; }
            pc = 0xde81a;
        }
        0xde81a => {
            // block de81a
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de81a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de81a add rsp,0xa0e8
            { let (r, f) = add::<64>(rsp, 0xa0e8u64);
              rsp = r;
              fl = f; }
            // de821 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // de822 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // de824 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // de826 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // de828 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // de82a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // de82b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xde82c => {
            // block de82c
            #[cfg(feature = "trace")] eprintln!("f_000de198 pc de82c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de82c call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
