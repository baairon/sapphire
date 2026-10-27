//! Mechanically lifted functions (scripts/lift_run.py), module u001. Each one is checked against the original
//! in the world harness (tests/world_u001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xdf804 size=818
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000df804<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdf804;
    loop {
        match pc {
        0xdf804 => {
            // block df804
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df804 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df804 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df805 mov rbp,rsp
            rbp = rsp;
            // df808 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // df80a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // df80c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // df80e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // df810 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df811 sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // df818 mov DWORD PTR [rbp-0xa8],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), ((r8 as u32 as u64)) as u32);
            // df81f mov DWORD PTR [rbp-0x84],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c), ((rcx as u32 as u64)) as u32);
            // df825 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // df828 mov rbx,rsi
            rbx = rsi;
            // df82b mov r14,rdi
            r14 = rdi;
            // df82e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // df836 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // df839 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // df83e call QWORD PTR [rax+0x1908]
            { let r = { let t = m.r64(rax.wrapping_add(0x1908)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df844 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // df847 lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // df84b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // df84d mov rdi,r14
            rdi = r14;
            // df850 call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df856 mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // df859 mov edx,DWORD PTR [r14+0x274]
            rdx = (m.r32(r14.wrapping_add(0x274)) as u64) as u32 as u64;
            // df860 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // df864 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // df866 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df869 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // df86c call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df871 mov eax,DWORD PTR [rbp-0x6c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // df874 mov DWORD PTR [rbp-0x74],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), ((rax as u32 as u64)) as u32);
            // df877 mov eax,DWORD PTR [rbp-0x68]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // df87a mov DWORD PTR [rbp-0x78],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff88), ((rax as u32 as u64)) as u32);
            // df87d mov r15d,DWORD PTR [rbp-0x70]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // df881 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // df884 lea rsi,[rbp-0x7c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff84);
            // df888 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // df88a mov rdi,r14
            rdi = r14;
            // df88d call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df893 cmp DWORD PTR [rbp-0x7c],0xa
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64, 0xau64).1;
            // df897 jne 0xdf8ad
            if fl.ne() { pc = 0xdf8ad; continue; }
            pc = 0xdf899;
        }
        0xdf899 => {
            // block df899
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df899 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df899 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // df89c lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df8a0 lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df8a4 mov rdi,r14
            rdi = r14;
            // df8a7 call QWORD PTR [rax+0x1a00]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdf8ad;
        }
        0xdf8ad => {
            // block df8ad
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df8ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df8ad mov r13,QWORD PTR [r14+0x280]
            r13 = m.r64(r14.wrapping_add(0x280)) as u64;
            // df8b4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // df8b6 mov QWORD PTR [rbp-0xb0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (rax) as u64);
            // df8bd test r13,r13
            fl = logic::<64>(r13 & r13);
            // df8c0 jne 0xdf90c
            if fl.ne() { pc = 0xdf90c; continue; }
            pc = 0xdf8c2;
        }
        0xdf8c2 => {
            // block df8c2
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df8c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df8c2 cmp BYTE PTR [r14+0x279],0x0
            fl = sub::<8>(m.r8(r14.wrapping_add(0x279)) as u64, 0x0u64).1;
            // df8ca je 0xdf8e8
            if fl.e() { pc = 0xdf8e8; continue; }
            pc = 0xdf8cc;
        }
        0xdf8cc => {
            // block df8cc
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df8cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df8cc mov edi,DWORD PTR [rbp-0x78]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // df8cf imul edi,r15d
            { let (r, f) = imul::<32>((rdi as u32 as u64), (r15 as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // df8d3 shl rdi,0x2
            { let (r, f) = shl::<64>(rdi, 0x2u64, fl); rdi = r; fl = f; }
            // df8d7 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df8dc mov r13,rax
            r13 = rax;
            // df8df mov QWORD PTR [rbp-0xb0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (r13) as u64);
            // df8e6 jmp 0xdf90c
            pc = 0xdf90c; continue;
        }
        0xdf8e8 => {
            // block df8e8
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df8e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df8e8 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // df8eb mov esi,DWORD PTR [r14+0x250]
            rsi = (m.r32(r14.wrapping_add(0x250)) as u64) as u32 as u64;
            // df8f2 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // df8f6 mov rdi,r14
            rdi = r14;
            // df8f9 call QWORD PTR [rax+0xa10]
            { let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df8ff mov r13,QWORD PTR [rbp-0x30]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // df903 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // df905 mov QWORD PTR [rbp-0xb0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (rax) as u64);
            pc = 0xdf90c;
        }
        0xdf90c => {
            // block df90c
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df90c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df90c mov QWORD PTR [rbp-0xb8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (r14) as u64);
            // df913 mov eax,DWORD PTR [rbp-0x74]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c)) as u64) as u32 as u64;
            // df916 cmp eax,r12d
            fl = sub::<32>((rax as u32 as u64), (r12 as u32 as u64)).1;
            // df919 cmovb r12d,eax
            if fl.b() { r12 = ((rax as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // df91d mov QWORD PTR [rbp-0x90],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (r12) as u64);
            // df924 mov edx,DWORD PTR [rbp-0x84]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c)) as u64) as u32 as u64;
            // df92a cmp r15d,edx
            fl = sub::<32>((r15 as u32 as u64), (rdx as u32 as u64)).1;
            // df92d cmovb edx,r15d
            if fl.b() { rdx = ((r15 as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // df931 mov rcx,r12
            rcx = r12;
            // df934 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // df937 sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // df939 mov r14d,0x0
            r14 = (0x0u64) as u32 as u64;
            // df93f mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // df944 jbe 0xdf959
            if fl.be() { pc = 0xdf959; continue; }
            pc = 0xdf946;
        }
        0xdf946 => {
            // block df946
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df946 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df946 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // df949 shr r14d,1
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x1u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // df94c sub eax,r14d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r14 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // df94f shl r14d,0x2
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // df953 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // df956 mov rcx,rax
            rcx = rax;
            pc = 0xdf959;
        }
        0xdf959 => {
            // block df959
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df959 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df959 mov QWORD PTR [rbp-0x98],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rcx) as u64);
            // df960 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // df963 mov DWORD PTR [rbp-0xbc],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff44), ((r15 as u32 as u64)) as u32);
            // df96a sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // df96c mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // df96f mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // df974 jbe 0xdf981
            if fl.be() { pc = 0xdf981; continue; }
            pc = 0xdf976;
        }
        0xdf976 => {
            // block df976
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df976 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // df979 shr r12d,1
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x1u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // df97c sub eax,r12d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r12 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // df97f mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xdf981;
        }
        0xdf981 => {
            // block df981
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df981 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df981 mov QWORD PTR [rbp-0xc8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (rcx) as u64);
            // df988 mov esi,DWORD PTR [rbp-0x78]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // df98b imul esi,r12d
            { let (r, f) = imul::<32>((rsi as u32 as u64), (r12 as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // df98f shl esi,0x2
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x2u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // df992 mov rdi,r13
            rdi = r13;
            // df995 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df99a mov esi,DWORD PTR [rbp-0x78]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // df99d imul r12d,esi
            { let (r, f) = imul::<32>((r12 as u32 as u64), (rsi as u32 as u64)); r12 = (r) as u32 as u64; fl = f; }
            // df9a1 lea r12,[r13+r12*4+0x0]
            r12 = r13.wrapping_add(r12.wrapping_mul(4));
            // df9a6 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // df9a9 je 0xdfa3c
            if fl.e() { pc = 0xdfa3c; continue; }
            pc = 0xdf9af;
        }
        0xdf9af => {
            // block df9af
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df9af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df9af mov rax,QWORD PTR [rbp-0x90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // df9b6 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // df9b9 mov QWORD PTR [rbp-0x90],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (rax) as u64);
            // df9c0 lea rax,[r14+rax*1]
            rax = r14.wrapping_add(rax);
            // df9c4 mov QWORD PTR [rbp-0xa0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (rax) as u64);
            // df9cb mov eax,DWORD PTR [rbp-0xa8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64) as u32 as u64;
            // df9d1 mov QWORD PTR [rbp-0xa8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rax) as u64);
            // df9d8 mov DWORD PTR [rbp-0x84],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c), ((r15 as u32 as u64)) as u32);
            // df9df mov r13w,0x1
            r13 = (r13 & !0xffff) | ((0x1u64) as u16 as u64);
            // df9e4 mov r15,r12
            r15 = r12;
            pc = 0xdf9e7;
        }
        0xdf9e7 => {
            // block df9e7
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc df9e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df9e7 mov rdi,r12
            rdi = r12;
            // df9ea mov rsi,r14
            rsi = r14;
            // df9ed call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df9f2 lea rdi,[r12+r14*1]
            rdi = r12.wrapping_add(r14);
            // df9f6 mov rsi,rbx
            rsi = rbx;
            // df9f9 mov rdx,QWORD PTR [rbp-0x90]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // dfa00 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfa05 add r12,QWORD PTR [rbp-0xa0]
            { let (r, f) = add::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64);
              r12 = r;
              fl = f; }
            // dfa0c mov rdi,r12
            rdi = r12;
            // dfa0f mov rsi,QWORD PTR [rbp-0x98]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // dfa16 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfa1b mov esi,DWORD PTR [rbp-0x78]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // dfa1e lea r12,[r15+rsi*4]
            r12 = r15.wrapping_add(rsi.wrapping_mul(4));
            // dfa22 add rbx,QWORD PTR [rbp-0xa8]
            { let (r, f) = add::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64);
              rbx = r;
              fl = f; }
            // dfa29 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // dfa2d lea r13d,[r13+0x1]
            r13 = (r13.wrapping_add(0x1)) as u32 as u64;
            // dfa31 cmp eax,DWORD PTR [rbp-0x84]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c)) as u64).1;
            // dfa37 mov r15,r12
            r15 = r12;
            // dfa3a jb 0xdf9e7
            if fl.b() { pc = 0xdf9e7; continue; }
            pc = 0xdfa3c;
        }
        0xdfa3c => {
            // block dfa3c
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfa3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfa3c mov rax,QWORD PTR [rbp-0xc8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff38)) as u64;
            // dfa43 imul esi,eax
            { let (r, f) = imul::<32>((rsi as u32 as u64), (rax as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // dfa46 mov rdi,r12
            rdi = r12;
            // dfa49 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfa4e mov r12,QWORD PTR [rbp-0xb0]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // dfa55 test r12,r12
            fl = logic::<64>(r12 & r12);
            // dfa58 mov rbx,QWORD PTR [rbp-0xb8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // dfa5f mov r15d,DWORD PTR [rbp-0xbc]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff44)) as u64) as u32 as u64;
            // dfa66 je 0xdface
            if fl.e() { pc = 0xdface; continue; }
            pc = 0xdfa68;
        }
        0xdfa68 => {
            // block dfa68
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfa68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfa68 mov al,BYTE PTR [rbx+0x279]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x279)) as u64) as u8 as u64);
            // dfa6e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dfa70 je 0xdface
            if fl.e() { pc = 0xdface; continue; }
            pc = 0xdfa72;
        }
        0xdfa72 => {
            // block dfa72
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfa72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfa72 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfa75 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // dfa7b lea rdx,[rbp-0x80]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // dfa7f mov rdi,rbx
            rdi = rbx;
            // dfa82 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfa88 mov r14d,DWORD PTR [rbp-0x80]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64) as u32 as u64;
            // dfa8c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfa8f mov rdi,rbx
            rdi = rbx;
            // dfa92 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfa98 cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // dfa9b jbe 0xdfaa8
            if fl.be() { pc = 0xdfaa8; continue; }
            pc = 0xdfa9d;
        }
        0xdfa9d => {
            // block dfa9d
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfa9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfa9d mov DWORD PTR [rbp-0x80],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u32);
            // dfaa4 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dfaa6 jmp 0xdfaab
            pc = 0xdfaab; continue;
        }
        0xdfaa8 => {
            // block dfaa8
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfaa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfaa8 mov esi,DWORD PTR [rbp-0x80]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64) as u32 as u64;
            pc = 0xdfaab;
        }
        0xdfaab => {
            // block dfaab
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dfaab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfaab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfaae imul r15d,DWORD PTR [rbp-0x78]
            { let (r, f) = imul::<32>((r15 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64); r15 = (r) as u32 as u64; fl = f; }
            // dfab3 shl r15d,0x2
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x2u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // dfab7 mov rdi,rbx
            rdi = rbx;
            // dfaba mov rdx,r12
            rdx = r12;
            // dfabd mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // dfac0 call QWORD PTR [rax+0x2c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfac6 mov rdi,r12
            rdi = r12;
            // dfac9 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdface;
        }
        0xdface => {
            // block dface
            #[cfg(feature = "trace")] eprintln!("f_000df804 pc dface rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dface mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfad1 movzx esi,BYTE PTR [rbx+0x278]
            rsi = (m.r8(rbx.wrapping_add(0x278)) as u64) as u32 as u64;
            // dfad8 mov rdi,rbx
            rdi = rbx;
            // dfadb call QWORD PTR [rax+0x570]
            { let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfae1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfae4 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // dfaea mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // dfaef mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dfaf4 mov rdi,rbx
            rdi = rbx;
            // dfaf7 call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfafd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfb00 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // dfb06 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfb08 mov rdi,rbx
            rdi = rbx;
            // dfb0b call QWORD PTR [rax+0x4f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfb11 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfb14 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // dfb1a mov rdi,rbx
            rdi = rbx;
            // dfb1d call QWORD PTR [rax+0x618]
            { let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfb23 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // dfb2a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dfb2b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dfb2d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dfb2f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dfb31 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dfb33 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dfb34 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe00a1 size=867
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e00a1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe00a1;
    loop {
        match pc {
        0xe00a1 => {
            // block e00a1
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e00a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e00a1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e00a2 mov rbp,rsp
            rbp = rsp;
            // e00a5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e00a7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e00a9 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e00ab push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e00ad push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e00ae sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // e00b2 mov DWORD PTR [rbp-0x38],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rcx as u32 as u64)) as u32);
            // e00b5 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // e00b7 mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // e00bb mov r14,rdi
            r14 = rdi;
            // e00be lea eax,[rbx+0x4]
            rax = (rbx.wrapping_add(0x4)) as u32 as u64;
            // e00c1 movsxd rdi,eax
            rdi = sx::<32>((rax as u32 as u64)) as u64;
            // e00c4 add rdi,rdi
            { let (r, f) = add::<64>(rdi, rdi);
              rdi = r;
              fl = f; }
            // e00c7 mov r15,0xffffffffffffffff
            r15 = 0xffff_ffff_ffff_ffffu64;
            // e00ce cmovb rdi,r15
            if fl.b() { rdi = r15; }
            // e00d2 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e00d7 mov r13,rax
            r13 = rax;
            // e00da mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e00dc shr eax,0x1f
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e00df add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e00e1 sar eax,1
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e00e3 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e00e7 lea eax,[rax+0x4]
            rax = (rax.wrapping_add(0x4)) as u32 as u64;
            // e00ea movsxd r12,eax
            r12 = sx::<32>((rax as u32 as u64)) as u64;
            // e00ed add r12,r12
            { let (r, f) = add::<64>(r12, r12);
              r12 = r;
              fl = f; }
            // e00f0 cmovb r12,r15
            if fl.b() { r12 = r15; }
            // e00f4 mov rdi,r12
            rdi = r12;
            // e00f7 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e00fc mov r15,rax
            r15 = rax;
            // e00ff mov rdi,r12
            rdi = r12;
            // e0102 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0107 mov r12,rax
            r12 = rax;
            // e010a test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // e010c jle 0xe0159
            if fl.le() { pc = 0xe0159; continue; }
            pc = 0xe010e;
        }
        0xe010e => {
            // block e010e
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e010e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e010e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0110 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe0112;
        }
        0xe0112 => {
            // block e0112
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0112 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0112 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // e0114 shr esi,0x1f
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1fu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e0117 add esi,ecx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rcx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0119 sar esi,1
            { let (r, f) = sar::<32>((rsi as u32 as u64), 0x1u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e011b inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e011d test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // e0120 movsxd rdx,eax
            rdx = sx::<32>((rax as u32 as u64)) as u64;
            // e0123 movzx edx,WORD PTR [r14+rdx*2]
            rdx = (m.r16(r14.wrapping_add(rdx.wrapping_mul(2))) as u64) as u32 as u64;
            // e0128 movsxd rsi,esi
            rsi = sx::<32>((rsi as u32 as u64)) as u64;
            // e012b jne 0xe0134
            if fl.ne() { pc = 0xe0134; continue; }
            pc = 0xe012d;
        }
        0xe012d => {
            // block e012d
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e012d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e012d mov WORD PTR [r15+rsi*2],dx
            m.w16(r15.wrapping_add(rsi.wrapping_mul(2)), ((rdx as u16 as u64)) as u16);
            // e0132 jmp 0xe0139
            pc = 0xe0139; continue;
        }
        0xe0134 => {
            // block e0134
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0134 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0134 mov WORD PTR [r12+rsi*2],dx
            m.w16(r12.wrapping_add(rsi.wrapping_mul(2)), ((rdx as u16 as u64)) as u16);
            pc = 0xe0139;
        }
        0xe0139 => {
            // block e0139
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0139 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0139 lea edx,[rax+0x1]
            rdx = (rax.wrapping_add(0x1)) as u32 as u64;
            // e013c movsxd rdx,edx
            rdx = sx::<32>((rdx as u32 as u64)) as u64;
            // e013f mov dx,WORD PTR [r14+rdx*2]
            rdx = (rdx & !0xffff) | ((m.r16(r14.wrapping_add(rdx.wrapping_mul(2))) as u64) as u16 as u64);
            // e0144 mov WORD PTR [r13+rax*1+0x2],dx
            m.w16(r13.wrapping_add(rax).wrapping_add(0x2), ((rdx as u16 as u64)) as u16);
            // e014a inc ecx
            { let (r, f) = inc::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // e014c add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // e0150 cmp ebx,ecx
            fl = sub::<32>((rbx as u32 as u64), (rcx as u32 as u64)).1;
            // e0152 jne 0xe0112
            if fl.ne() { pc = 0xe0112; continue; }
            pc = 0xe0154;
        }
        0xe0154 => {
            // block e0154
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0154 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0154 movzx eax,WORD PTR [r13+0x2]
            rax = (m.r16(r13.wrapping_add(0x2)) as u64) as u32 as u64;
            pc = 0xe0159;
        }
        0xe0159 => {
            // block e0159
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0159 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0159 mov WORD PTR [r13+0x0],ax
            m.w16(r13, ((rax as u16 as u64)) as u16);
            // e015e movsxd rax,ebx
            rax = sx::<32>((rbx as u32 as u64)) as u64;
            // e0161 mov ax,WORD PTR [r13+rax*2+0x0]
            rax = (rax & !0xffff) | ((m.r16(r13.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // e0167 lea ecx,[rbx+0x2]
            rcx = (rbx.wrapping_add(0x2)) as u32 as u64;
            // e016a movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e016d mov WORD PTR [r13+rcx*2+0x0],ax
            m.w16(r13.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e0173 lea ecx,[rbx+0x1]
            rcx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // e0176 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e0179 mov WORD PTR [r13+rcx*2+0x0],ax
            m.w16(r13.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e017f shl ebx,0x10
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0182 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e0184 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // e0185 mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // e0188 idiv ecx
            { let (q, r) = idiv::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // e018a mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // e018d test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // e018f mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // e0193 jle 0xe0244
            if fl.le() { pc = 0xe0244; continue; }
            pc = 0xe0199;
        }
        0xe0199 => {
            // block e0199
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0199 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0199 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // e019e xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e01a1 lea r9,[rip+0x47e78] # 0x128020
            r9 = m.env.data_va(247789);
            // e01a8 mov r8w,0x4
            r8 = (r8 & !0xffff) | ((0x4u64) as u16 as u64);
            // e01ad mov r10d,DWORD PTR [rbp-0x38]
            r10 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            pc = 0xe01b1;
        }
        0xe01b1 => {
            // block e01b1
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e01b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e01b1 mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // e01b4 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e01b7 mov edx,r11d
            rdx = ((r11 as u32 as u64)) as u32 as u64;
            // e01ba shr edx,0xb
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e01bd and edx,0x1f
            { let r = (rdx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e01c0 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e01c2 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e01c4 movsx ebx,WORD PTR [r13+rax*2-0x2]
            rbx = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e01ca mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // e01cf sub edi,edx
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rdx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e01d1 imul ebx,DWORD PTR [r9+rdi*4]
            { let (r, f) = imul::<32>((rbx as u32 as u64), m.r32(r9.wrapping_add(rdi.wrapping_mul(4))) as u64); rbx = (r) as u32 as u64; fl = f; }
            // e01d6 movsx edi,WORD PTR [r13+rax*2+0x0]
            rdi = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e01dc mov esi,0x40
            rsi = (0x40u64) as u32 as u64;
            // e01e1 sub esi,edx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e01e3 imul edi,DWORD PTR [r9+rsi*4]
            { let (r, f) = imul::<32>((rdi as u32 as u64), m.r32(r9.wrapping_add(rsi.wrapping_mul(4))) as u64); rdi = (r) as u32 as u64; fl = f; }
            // e01e8 add edi,ebx
            { let (r, f) = add::<32>((rdi as u32 as u64), (rbx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e01ea movsx esi,WORD PTR [r13+rax*2+0x2]
            rsi = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e01f0 mov ebx,0x60
            rbx = (0x60u64) as u32 as u64;
            // e01f5 sub ebx,edx
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rdx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e01f7 imul esi,DWORD PTR [r9+rbx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rbx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // e01fc add esi,edi
            { let (r, f) = add::<32>((rsi as u32 as u64), (rdi as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e01fe movsx eax,WORD PTR [r13+rax*2+0x4]
            rax = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e0204 mov edi,0x80
            rdi = (0x80u64) as u32 as u64;
            // e0209 sub edi,edx
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rdx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e020b imul eax,DWORD PTR [r9+rdi*4]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(r9.wrapping_add(rdi.wrapping_mul(4))) as u64); rax = (r) as u32 as u64; fl = f; }
            // e0210 add eax,esi
            { let (r, f) = add::<32>((rax as u32 as u64), (rsi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e0212 mov bx,0x3fb
            rbx = (rbx & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e0216 cmp eax,0x3fbffff
            fl = sub::<32>((rax as u32 as u64), 0x3fbffffu64).1;
            // e021b ja 0xe022c
            if fl.a() { pc = 0xe022c; continue; }
            pc = 0xe021d;
        }
        0xe021d => {
            // block e021d
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e021d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e021d mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e021f shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0222 cmp eax,0x40000
            fl = sub::<32>((rax as u32 as u64), 0x40000u64).1;
            // e0227 cmovb bx,r8w
            if fl.b() { rbx = (rbx & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            pc = 0xe022c;
        }
        0xe022c => {
            // block e022c
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e022c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e022c movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e022f mov WORD PTR [r14+rcx*2],bx
            m.w16(r14.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16 as u64)) as u16);
            // e0234 add ecx,0x2
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x2u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0237 add r11d,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e023b dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // e023e jne 0xe01b1
            if fl.ne() { pc = 0xe01b1; continue; }
            pc = 0xe0244;
        }
        0xe0244 => {
            // block e0244
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0244 mov QWORD PTR [rbp-0x60],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (r13) as u64);
            // e0248 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // e024c mov ax,WORD PTR [r15+0x2]
            rax = (rax & !0xffff) | ((m.r16(r15.wrapping_add(0x2)) as u64) as u16 as u64);
            // e0251 mov WORD PTR [r15],ax
            m.w16(r15, ((rax as u16 as u64)) as u16);
            // e0255 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e0259 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // e025c mov cx,WORD PTR [r15+rax*2]
            rcx = (rcx & !0xffff) | ((m.r16(r15.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // e0261 lea edx,[rsi+0x2]
            rdx = (rsi.wrapping_add(0x2)) as u32 as u64;
            // e0264 movsxd rdx,edx
            rdx = sx::<32>((rdx as u32 as u64)) as u64;
            // e0267 mov WORD PTR [r15+rdx*2],cx
            m.w16(r15.wrapping_add(rdx.wrapping_mul(2)), ((rcx as u16 as u64)) as u16);
            // e026c inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e026e movsxd rsi,esi
            rsi = sx::<32>((rsi as u32 as u64)) as u64;
            // e0271 mov WORD PTR [r15+rsi*2],cx
            m.w16(r15.wrapping_add(rsi.wrapping_mul(2)), ((rcx as u16 as u64)) as u16);
            // e0276 mov QWORD PTR [rbp-0x50],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r12) as u64);
            // e027a mov cx,WORD PTR [r12+0x2]
            rcx = (rcx & !0xffff) | ((m.r16(r12.wrapping_add(0x2)) as u64) as u16 as u64);
            // e0280 mov WORD PTR [r12],cx
            m.w16(r12, ((rcx as u16 as u64)) as u16);
            // e0285 mov ax,WORD PTR [r12+rax*2]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // e028a mov WORD PTR [r12+rdx*2],ax
            m.w16(r12.wrapping_add(rdx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e028f mov WORD PTR [r12+rsi*2],ax
            m.w16(r12.wrapping_add(rsi.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e0294 mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // e0297 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // e029a jl 0xe03db
            if fl.l() { pc = 0xe03db; continue; }
            pc = 0xe02a0;
        }
        0xe02a0 => {
            // block e02a0
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e02a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e02a0 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e02a2 shr eax,0x1f
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e02a5 add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e02a7 sar eax,1
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e02a9 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e02ab mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e02af mov r12d,0x2
            r12 = (0x2u64) as u32 as u64;
            // e02b5 xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e02b8 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe02bb;
        }
        0xe02bb => {
            // block e02bb
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e02bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e02bb mov r13,r14
            r13 = r14;
            // e02be mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // e02c1 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e02c4 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // e02c7 shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e02ca and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e02cd inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e02cf movsxd r14,eax
            r14 = sx::<32>((rax as u32 as u64)) as u64;
            // e02d2 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e02d6 movsx eax,WORD PTR [rsi+r14*2-0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e02dc mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // e02e1 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e02e3 lea rdx,[rip+0x47d36] # 0x128020
            rdx = m.env.data_va(247789);
            // e02ea mov rdi,rdx
            rdi = rdx;
            // e02ed mov ecx,DWORD PTR [rdi+rcx*4]
            rcx = (m.r32(rdi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e02f0 mov DWORD PTR [rbp-0x38],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rcx as u32 as u64)) as u32);
            // e02f3 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e02f6 movsx ecx,WORD PTR [rsi+r14*2]
            rcx = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e02fb mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // e0300 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0302 mov edx,DWORD PTR [rdi+rdx*4]
            rdx = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // e0305 mov DWORD PTR [rbp-0x40],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), ((rdx as u32 as u64)) as u32);
            // e0308 imul ecx,edx
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rdx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // e030b add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e030d movsx eax,WORD PTR [rsi+r14*2+0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e0313 mov edx,0x60
            rdx = (0x60u64) as u32 as u64;
            // e0318 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e031a mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // e031e imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e0322 add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e0324 movsx r10d,WORD PTR [rsi+r14*2+0x4]
            r10 = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e032a mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // e032f sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0331 mov r9d,DWORD PTR [rdi+rcx*4]
            r9 = (m.r32(rdi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e0335 imul r10d,r9d
            { let (r, f) = imul::<32>((r10 as u32 as u64), (r9 as u32 as u64)); r10 = (r) as u32 as u64; fl = f; }
            // e0339 add r10d,eax
            { let (r, f) = add::<32>((r10 as u32 as u64), (rax as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // e033c mov bx,0x3fb
            rbx = (rbx & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e0340 cmp r10d,0x3fbffff
            fl = sub::<32>((r10 as u32 as u64), 0x3fbffffu64).1;
            // e0347 mov ax,0x3fb
            rax = (rax & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e034b ja 0xe0362
            if fl.a() { pc = 0xe0362; continue; }
            pc = 0xe034d;
        }
        0xe034d => {
            // block e034d
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e034d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e034d mov eax,r10d
            rax = ((r10 as u32 as u64)) as u32 as u64;
            // e0350 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0353 cmp r10d,0x40000
            fl = sub::<32>((r10 as u32 as u64), 0x40000u64).1;
            // e035a mov cx,0x4
            rcx = (rcx & !0xffff) | ((0x4u64) as u16 as u64);
            // e035e cmovb ax,cx
            if fl.b() { rax = (rax & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            pc = 0xe0362;
        }
        0xe0362 => {
            // block e0362
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e0362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0362 lea ecx,[r12-0x2]
            rcx = (r12.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // e0367 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e036a mov WORD PTR [r13+rcx*2+0x0],ax
            m.w16(r13.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e0370 mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // e0374 movsx eax,WORD PTR [rsi+r14*2-0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e037a imul eax,DWORD PTR [rbp-0x38]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64); rax = (r) as u32 as u64; fl = f; }
            // e037e movsx ecx,WORD PTR [rsi+r14*2]
            rcx = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e0383 imul ecx,DWORD PTR [rbp-0x40]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // e0387 add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0389 movsx edx,WORD PTR [rsi+r14*2+0x2]
            rdx = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e038f imul edx,r8d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r8 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // e0393 add edx,ecx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0395 movsx eax,WORD PTR [rsi+r14*2+0x4]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e039b imul eax,r9d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r9 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e039f add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e03a1 cmp eax,0x3fbffff
            fl = sub::<32>((rax as u32 as u64), 0x3fbffffu64).1;
            // e03a6 ja 0xe03ba
            if fl.a() { pc = 0xe03ba; continue; }
            pc = 0xe03a8;
        }
        0xe03a8 => {
            // block e03a8
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e03a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e03a8 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e03aa shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e03ad cmp eax,0x40000
            fl = sub::<32>((rax as u32 as u64), 0x40000u64).1;
            // e03b2 mov ax,0x4
            rax = (rax & !0xffff) | ((0x4u64) as u16 as u64);
            // e03b6 cmovb bx,ax
            if fl.b() { rbx = (rbx & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            pc = 0xe03ba;
        }
        0xe03ba => {
            // block e03ba
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e03ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e03ba movsxd rax,r12d
            rax = sx::<32>((r12 as u32 as u64)) as u64;
            // e03bd mov WORD PTR [r13+rax*2+0x0],bx
            m.w16(r13.wrapping_add(rax.wrapping_mul(2)), ((rbx as u16 as u64)) as u16);
            // e03c3 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // e03c6 add r12d,0x4
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x4u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e03ca add r11d,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e03ce cmp r15,QWORD PTR [rbp-0x58]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64).1;
            // e03d2 mov r14,r13
            r14 = r13;
            // e03d5 jl 0xe02bb
            if fl.l() { pc = 0xe02bb; continue; }
            pc = 0xe03db;
        }
        0xe03db => {
            // block e03db
            #[cfg(feature = "trace")] eprintln!("f_000e00a1 pc e03db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e03db mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e03df call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e03e4 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e03e8 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e03ed mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // e03f1 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // e03f5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e03f6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e03f8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e03fa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e03fc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e03fe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e03ff jmp 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd582c size=900
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000d582c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd582c;
    loop {
        match pc {
        0xd582c => {
            // block d582c
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d582c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d582c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d582d mov rbp,rsp
            rbp = rsp;
            // d5830 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d5832 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d5834 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d5836 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d5838 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5839 sub rsp,0x348
            { let (r, f) = sub::<64>(rsp, 0x348u64);
              rsp = r;
              fl = f; }
            // d5840 mov r14,rsi
            r14 = rsi;
            // d5843 mov rbx,rdi
            rbx = rdi;
            // d5846 mov r13,QWORD PTR [rip+0x6ab8b] # 0x1403d8
            r13 = m.r64(m.env.data_va(347045)) as u64;
            // d584d mov r13,QWORD PTR [r13+0x0]
            r13 = m.r64(r13) as u64;
            // d5851 mov QWORD PTR [rbp-0x30],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r13) as u64);
            // d5855 mov edi,0x100
            rdi = (0x100u64) as u32 as u64;
            // d585a call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d585f mov QWORD PTR [rbp-0x360],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca0), (rax) as u64);
            // d5866 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d5869 call 0xcdbbf
            { let r = { let t = m.env.code_va(0xcdbbf); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d586e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5870 je 0xd589f
            if fl.e() { pc = 0xd589f; continue; }
            pc = 0xd5872;
        }
        0xd5872 => {
            // block d5872
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5872 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5872 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5875 lea rdx,[rbp-0x32c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fcd4);
            // d587c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d587f mov esi,0x40002
            rsi = (0x40002u64) as u32 as u64;
            // d5884 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5889 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d588c mov rdi,rbx
            rdi = rbx;
            // d588f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5892 cmp DWORD PTR [rbp-0x32c],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcd4)) as u64, 0x1u64).1;
            // d5899 jne 0xd5b68
            if fl.ne() { pc = 0xd5b68; continue; }
            pc = 0xd589f;
        }
        0xd589f => {
            // block d589f
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d589f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d589f mov QWORD PTR [rbp-0x368],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fc98), (r14) as u64);
            // d58a6 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d58a9 call 0xcdb57
            { let r = { let t = m.env.code_va(0xcdb57); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58ae test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d58b0 jne 0xd58ca
            if fl.ne() { pc = 0xd58ca; continue; }
            pc = 0xd58b2;
        }
        0xd58b2 => {
            // block d58b2
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d58b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d58b2 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d58b5 call 0xcdbbf
            { let r = { let t = m.env.code_va(0xcdbbf); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58ba test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d58bc jne 0xd58ca
            if fl.ne() { pc = 0xd58ca; continue; }
            pc = 0xd58be;
        }
        0xd58be => {
            // block d58be
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d58be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d58be mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d58c1 call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58c6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d58c8 je 0xd5949
            if fl.e() { pc = 0xd5949; continue; }
            pc = 0xd58ca;
        }
        0xd58ca => {
            // block d58ca
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d58ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d58ca mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d58cd mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d58d2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d58d4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d58d9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d58dc mov rdi,rbx
            rdi = rbx;
            // d58df call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58e2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d58e5 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d58ea mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // d58ef mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d58f4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d58f7 mov rdi,rbx
            rdi = rbx;
            // d58fa call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58fd mov r14d,0x3e8
            r14 = (0x3e8u64) as u32 as u64;
            // d5903 lea r12,[rbp-0x330]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fcd0);
            // d590a mov r15d,0x100
            r15 = (0x100u64) as u32 as u64;
            pc = 0xd5910;
        }
        0xd5910 => {
            // block d5910
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5910 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5910 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5913 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d5918 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d591d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5920 mov rdi,rbx
            rdi = rbx;
            // d5923 mov rdx,r12
            rdx = r12;
            // d5926 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5929 mov eax,DWORD PTR [rbp-0x330]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64) as u32 as u64;
            // d592f and eax,r15d
            { let r = (rax as u32 as u64) & (r15 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d5932 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d5934 shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // d5937 sub r14d,ecx
            { let (r, f) = sub::<32>((r14 as u32 as u64), (rcx as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // d593a je 0xd5940
            if fl.e() { pc = 0xd5940; continue; }
            pc = 0xd593c;
        }
        0xd593c => {
            // block d593c
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d593c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d593c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d593e jne 0xd5910
            if fl.ne() { pc = 0xd5910; continue; }
            pc = 0xd5940;
        }
        0xd5940 => {
            // block d5940
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5940 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5940 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // d5943 je 0xd5a60
            if fl.e() { pc = 0xd5a60; continue; }
            pc = 0xd5949;
        }
        0xd5949 => {
            // block d5949
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5949 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d594c lea r12,[rbp-0x334]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fccc);
            // d5953 mov r15d,0x100
            r15 = (0x100u64) as u32 as u64;
            // d5959 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd595b;
        }
        0xd595b => {
            // block d595b
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d595b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d595b mov QWORD PTR [rbp-0x358],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fca8), (rdx) as u64);
            // d5962 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5965 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d596a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d596f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5972 mov rdi,rbx
            rdi = rbx;
            // d5975 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5978 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d597b mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d5980 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d5985 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d598a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d598d mov rdi,rbx
            rdi = rbx;
            // d5990 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5993 mov r13d,0x3e8
            r13 = (0x3e8u64) as u32 as u64;
            pc = 0xd5999;
        }
        0xd5999 => {
            // block d5999
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5999 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5999 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d599c mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d59a1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d59a6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d59a9 mov rdi,rbx
            rdi = rbx;
            // d59ac mov rdx,r12
            rdx = r12;
            // d59af call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d59b2 mov eax,DWORD PTR [rbp-0x334]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fccc)) as u64) as u32 as u64;
            // d59b8 and eax,r15d
            { let r = (rax as u32 as u64) & (r15 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d59bb mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d59bd shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // d59c0 sub r13d,ecx
            { let (r, f) = sub::<32>((r13 as u32 as u64), (rcx as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // d59c3 je 0xd59c9
            if fl.e() { pc = 0xd59c9; continue; }
            pc = 0xd59c5;
        }
        0xd59c5 => {
            // block d59c5
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d59c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d59c5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d59c7 jne 0xd5999
            if fl.ne() { pc = 0xd5999; continue; }
            pc = 0xd59c9;
        }
        0xd59c9 => {
            // block d59c9
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d59c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d59c9 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // d59cc je 0xd5a4d
            if fl.e() { pc = 0xd5a4d; continue; }
            pc = 0xd59ce;
        }
        0xd59ce => {
            // block d59ce
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d59ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d59ce mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d59d1 mov rcx,QWORD PTR [rbp-0x360]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fca0)) as u64;
            // d59d8 lea rdx,[rcx+r14*4]
            rdx = rcx.wrapping_add(r14.wrapping_mul(4));
            // d59dc mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d59e1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d59e6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d59e9 mov rdi,rbx
            rdi = rbx;
            // d59ec call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d59ef inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // d59f2 mov rdx,QWORD PTR [rbp-0x358]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fca8)) as u64;
            // d59f9 add edx,0x4
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x4u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d59fc cmp r14,0x40
            fl = sub::<64>(r14, 0x40u64).1;
            // d5a00 jb 0xd595b
            if fl.b() { pc = 0xd595b; continue; }
            pc = 0xd5a06;
        }
        0xd5a06 => {
            // block d5a06
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a06 lea rbx,[rbp-0x328]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // d5a0d mov rdi,rbx
            rdi = rbx;
            // d5a10 call 0x12b8c
            { let r = { let t = m.env.code_va(0x12b8c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5a15 lea rdi,[rbp-0x350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // d5a1c mov rsi,rbx
            rsi = rbx;
            // d5a1f mov rdx,QWORD PTR [rbp-0x360]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fca0)) as u64;
            // d5a26 call 0x12e88
            { let r = { let t = m.env.code_va(0x12e88); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5a2b mov rbx,QWORD PTR [rbp-0x368]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fc98)) as u64;
            // d5a32 movzx eax,BYTE PTR [rbp-0x350]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fcb0)) as u64) as u32 as u64;
            // d5a39 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5a3b mov r13,QWORD PTR [rip+0x6a996] # 0x1403d8
            r13 = m.r64(m.env.data_va(347045)) as u64;
            // d5a42 mov r13,QWORD PTR [r13+0x0]
            r13 = m.r64(r13) as u64;
            // d5a46 jne 0xd5a68
            if fl.ne() { pc = 0xd5a68; continue; }
            pc = 0xd5a48;
        }
        0xd5a48 => {
            // block d5a48
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a48 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // d5a4b jmp 0xd5a6f
            pc = 0xd5a6f; continue;
        }
        0xd5a4d => {
            // block d5a4d
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a4d xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d5a50 mov r13,QWORD PTR [rip+0x6a981] # 0x1403d8
            r13 = m.r64(m.env.data_va(347045)) as u64;
            // d5a57 mov r13,QWORD PTR [r13+0x0]
            r13 = m.r64(r13) as u64;
            // d5a5b jmp 0xd5b68
            pc = 0xd5b68; continue;
        }
        0xd5a60 => {
            // block d5a60
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a60 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d5a63 jmp 0xd5b68
            pc = 0xd5b68; continue;
        }
        0xd5a68 => {
            // block d5a68
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a68 mov rax,QWORD PTR [rbp-0x348]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcb8)) as u64;
            pc = 0xd5a6f;
        }
        0xd5a6f => {
            // block d5a6f
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a6f test rax,rax
            fl = logic::<64>(rax & rax);
            // d5a72 jne 0xd5b26
            if fl.ne() { pc = 0xd5b26; continue; }
            pc = 0xd5a78;
        }
        0xd5a78 => {
            // block d5a78
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a78 lea rdi,[rbx+0x10]
            rdi = rbx.wrapping_add(0x10);
            // d5a7c test BYTE PTR [rbp-0xc0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64 & 0x1u64);
            // d5a83 jne 0xd5a91
            if fl.ne() { pc = 0xd5a91; continue; }
            pc = 0xd5a85;
        }
        0xd5a85 => {
            // block d5a85
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a85 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // d5a8c inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // d5a8f jmp 0xd5a98
            pc = 0xd5a98; continue;
        }
        0xd5a91 => {
            // block d5a91
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a91 mov rsi,QWORD PTR [rbp-0xb0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            pc = 0xd5a98;
        }
        0xd5a98 => {
            // block d5a98
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5a98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5a98 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // d5a9d call 0xec2cc
            { let r = { let t = m.env.stub_va(0xa99); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5aa2 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // d5aa6 test BYTE PTR [rbp-0xa8],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64 & 0x1u64);
            // d5aad jne 0xd5abb
            if fl.ne() { pc = 0xd5abb; continue; }
            pc = 0xd5aaf;
        }
        0xd5aaf => {
            // block d5aaf
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5aaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5aaf lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d5ab6 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // d5ab9 jmp 0xd5ac2
            pc = 0xd5ac2; continue;
        }
        0xd5abb => {
            // block d5abb
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5abb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5abb mov rsi,QWORD PTR [rbp-0x98]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            pc = 0xd5ac2;
        }
        0xd5ac2 => {
            // block d5ac2
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5ac2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ac2 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // d5ac7 call 0xec2cc
            { let r = { let t = m.env.stub_va(0xa99); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5acc lea rdi,[rbx+0x30]
            rdi = rbx.wrapping_add(0x30);
            // d5ad0 test BYTE PTR [rbp-0x90],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64 & 0x1u64);
            // d5ad7 jne 0xd5ae5
            if fl.ne() { pc = 0xd5ae5; continue; }
            pc = 0xd5ad9;
        }
        0xd5ad9 => {
            // block d5ad9
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5ad9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ad9 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d5ae0 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // d5ae3 jmp 0xd5ae9
            pc = 0xd5ae9; continue;
        }
        0xd5ae5 => {
            // block d5ae5
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5ae5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ae5 mov rsi,QWORD PTR [rbp-0x80]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            pc = 0xd5ae9;
        }
        0xd5ae9 => {
            // block d5ae9
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5ae9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ae9 mov edx,0x64
            rdx = (0x64u64) as u32 as u64;
            // d5aee call 0xec2cc
            { let r = { let t = m.env.stub_va(0xa99); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5af3 lea rdi,[rbx+0x98]
            rdi = rbx.wrapping_add(0x98);
            // d5afa test BYTE PTR [rbp-0x78],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64 & 0x1u64);
            // d5afe jne 0xd5b09
            if fl.ne() { pc = 0xd5b09; continue; }
            pc = 0xd5b00;
        }
        0xd5b00 => {
            // block d5b00
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b00 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d5b04 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // d5b07 jmp 0xd5b0d
            pc = 0xd5b0d; continue;
        }
        0xd5b09 => {
            // block d5b09
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b09 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0xd5b0d;
        }
        0xd5b0d => {
            // block d5b0d
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b0d mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // d5b12 call 0xec2cc
            { let r = { let t = m.env.stub_va(0xa99); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5b17 lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // d5b1e call 0x1369c
            { let r = { let t = m.env.code_va(0x1369c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5b23 mov DWORD PTR [rbx+0xc],eax
            m.w32(rbx.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            pc = 0xd5b26;
        }
        0xd5b26 => {
            // block d5b26
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b26 mov rdi,QWORD PTR [rbp-0x360]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fca0)) as u64;
            // d5b2d call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5b32 movzx eax,BYTE PTR [rbp-0x350]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fcb0)) as u64) as u32 as u64;
            // d5b39 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5b3b jne 0xd5b42
            if fl.ne() { pc = 0xd5b42; continue; }
            pc = 0xd5b3d;
        }
        0xd5b3d => {
            // block d5b3d
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b3d shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // d5b40 jmp 0xd5b49
            pc = 0xd5b49; continue;
        }
        0xd5b42 => {
            // block d5b42
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b42 mov rax,QWORD PTR [rbp-0x348]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcb8)) as u64;
            pc = 0xd5b49;
        }
        0xd5b49 => {
            // block d5b49
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b49 test rax,rax
            fl = logic::<64>(rax & rax);
            // d5b4c sete r12b
            r12 = (r12 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d5b50 lea rdi,[rbp-0x350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // d5b57 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5b5c lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // d5b63 call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd5b68;
        }
        0xd5b68 => {
            // block d5b68
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b68 cmp r13,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r13, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // d5b6c jne 0xd5b83
            if fl.ne() { pc = 0xd5b83; continue; }
            pc = 0xd5b6e;
        }
        0xd5b6e => {
            // block d5b6e
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b6e mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // d5b71 add rsp,0x348
            { let (r, f) = add::<64>(rsp, 0x348u64);
              rsp = r;
              fl = f; }
            // d5b78 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5b79 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d5b7b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d5b7d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d5b7f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d5b81 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5b82 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd5b83 => {
            // block d5b83
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b83 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0xd5b8d => {
            // block d5b8d
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b8d mov rbx,rax
            rbx = rax;
            // d5b90 lea rdi,[rbp-0x350]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb0);
            // d5b97 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd5b9c;
        }
        0xd5b9c => {
            // block d5b9c
            #[cfg(feature = "trace")] eprintln!("f_000d582c pc d5b9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5b9c lea rdi,[rbp-0x328]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcd8);
            // d5ba3 call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5ba8 mov rdi,rbx
            rdi = rbx;
            // d5bab call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdda0e size=930
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000dda0e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdda0e;
    loop {
        match pc {
        0xdda0e => {
            // block dda0e
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc dda0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dda0e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dda0f mov rbp,rsp
            rbp = rsp;
            // dda12 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dda14 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dda16 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dda18 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dda1a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dda1b sub rsp,0x100a8
            { let (r, f) = sub::<64>(rsp, 0x100a8u64);
              rsp = r;
              fl = f; }
            // dda22 mov r14,rdi
            r14 = rdi;
            // dda25 mov rax,QWORD PTR [rip+0x629ac] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // dda2c mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dda2f mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // dda33 mov QWORD PTR [rbp-0x10038],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ffc8), (0x0u64) as u64);
            // dda3e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dda41 lea rsi,[rbp-0x1003c]
            rsi = rbp.wrapping_add(0xffff_ffff_fffe_ffc4);
            // dda48 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // dda4b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dda4d call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dda53 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dda56 lea rsi,[rbp-0x10040]
            rsi = rbp.wrapping_add(0xffff_ffff_fffe_ffc0);
            // dda5d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dda5f mov rdi,r14
            rdi = r14;
            // dda62 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dda68 cmp DWORD PTR [rbp-0x10040],0xa
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ffc0)) as u64, 0xau64).1;
            // dda6f sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // dda72 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dda75 lea rsi,[rbp-0x10041]
            rsi = rbp.wrapping_add(0xffff_ffff_fffe_ffbf);
            // dda7c lea rdx,[rbp-0x10042]
            rdx = rbp.wrapping_add(0xffff_ffff_fffe_ffbe);
            // dda83 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dda85 mov rdi,r14
            rdi = r14;
            // dda88 call QWORD PTR [rax+0x5e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x5e0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dda8e mov esi,DWORD PTR [rbp-0x1003c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ffc4)) as u64) as u32 as u64;
            // dda94 mov edx,DWORD PTR [r14+0x274]
            rdx = (m.r32(r14.wrapping_add(0x274)) as u64) as u32 as u64;
            // dda9b movzx r9d,BYTE PTR [rbp-0x10042]
            r9 = (m.r8(rbp.wrapping_add(0xffff_ffff_fffe_ffbe)) as u64) as u32 as u64;
            // ddaa3 movzx ecx,BYTE PTR [rbp-0x10041]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_fffe_ffbf)) as u64) as u32 as u64;
            // ddaaa movzx r8d,bl
            r8 = ((rbx as u8 as u64)) as u32 as u64;
            // ddaae lea rdi,[rbp-0x10080]
            rdi = rbp.wrapping_add(0xffff_ffff_fffe_ff80);
            // ddab5 call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddaba mov eax,DWORD PTR [rbp-0x10080]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff80)) as u64) as u32 as u64;
            // ddac0 mov DWORD PTR [rbp-0x1009c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_fffe_ff64), ((rax as u32 as u64)) as u32);
            // ddac6 mov r13d,DWORD PTR [rbp-0x1007c]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff84)) as u64) as u32 as u64;
            // ddacd mov eax,DWORD PTR [rbp-0x10078]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff88)) as u64) as u32 as u64;
            // ddad3 mov QWORD PTR [rbp-0x10098],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ff68), (rax) as u64);
            // ddada mov ebx,DWORD PTR [rbp-0x10074]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff8c)) as u64) as u32 as u64;
            // ddae0 mov r12,QWORD PTR [r14+0x280]
            r12 = m.r64(r14.wrapping_add(0x280)) as u64;
            // ddae7 test r12,r12
            fl = logic::<64>(r12 & r12);
            // ddaea jne 0xddb3d
            if fl.ne() { pc = 0xddb3d; continue; }
            pc = 0xddaec;
        }
        0xddaec => {
            // block ddaec
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddaec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddaec cmp BYTE PTR [r14+0x279],0x0
            fl = sub::<8>(m.r8(r14.wrapping_add(0x279)) as u64, 0x0u64).1;
            // ddaf4 je 0xddb19
            if fl.e() { pc = 0xddb19; continue; }
            pc = 0xddaf6;
        }
        0xddaf6 => {
            // block ddaf6
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddaf6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddaf6 mov eax,DWORD PTR [rbp-0x1009c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff64)) as u64) as u32 as u64;
            // ddafc mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // ddafe mov rax,QWORD PTR [rbp-0x10098]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff68)) as u64;
            // ddb05 imul edi,eax
            { let (r, f) = imul::<32>((rdi as u32 as u64), (rax as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // ddb08 shl rdi,0x2
            { let (r, f) = shl::<64>(rdi, 0x2u64, fl); rdi = r; fl = f; }
            // ddb0c call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddb11 mov r12,rax
            r12 = rax;
            // ddb14 mov r15,r12
            r15 = r12;
            // ddb17 jmp 0xddb3d
            pc = 0xddb3d; continue;
        }
        0xddb19 => {
            // block ddb19
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddb19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddb19 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ddb1c mov esi,DWORD PTR [r14+0x250]
            rsi = (m.r32(r14.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddb23 lea rdx,[rbp-0x10038]
            rdx = rbp.wrapping_add(0xffff_ffff_fffe_ffc8);
            // ddb2a mov rdi,r14
            rdi = r14;
            // ddb2d call QWORD PTR [rax+0xa10]
            { let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddb33 mov r12,QWORD PTR [rbp-0x10038]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ffc8)) as u64;
            // ddb3a xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xddb3d;
        }
        0xddb3d => {
            // block ddb3d
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddb3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddb3d mov DWORD PTR [rbp-0x100bc],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_fffe_ff44), ((r13 as u32 as u64)) as u32);
            // ddb44 mov QWORD PTR [rbp-0x100b8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ff48), (r15) as u64);
            // ddb4b mov QWORD PTR [rbp-0x100b0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ff50), (r14) as u64);
            // ddb52 lea r14,[rbp-0x8030]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // ddb59 mov esi,0x4000
            rsi = (0x4000u64) as u32 as u64;
            // ddb5e mov rdi,r14
            rdi = r14;
            // ddb61 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddb66 lea rsi,[rbp-0x10030]
            rsi = rbp.wrapping_add(0xffff_ffff_fffe_ffd0);
            // ddb6d mov rdi,r14
            rdi = r14;
            // ddb70 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // ddb73 call 0x67628
            { let r = { let t = m.env.code_va(0x67628); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddb78 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // ddb7a je 0xddbd2
            if fl.e() { pc = 0xddbd2; continue; }
            pc = 0xddb7c;
        }
        0xddb7c => {
            // block ddb7c
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddb7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddb7c mov rcx,QWORD PTR [rbp-0x10098]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff68)) as u64;
            // ddb83 lea r14d,[rcx*4+0x0]
            r14 = (rcx.wrapping_mul(4)) as u32 as u64;
            // ddb8b lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // ddb8e lea rax,[rax*4+0x4]
            rax = rax.wrapping_mul(4).wrapping_add(0x4);
            // ddb96 imul rax,rcx
            { let (r, f) = imul::<64>(rax, rcx); rax = r; fl = f; }
            // ddb9a add rax,r12
            { let (r, f) = add::<64>(rax, r12);
              rax = r;
              fl = f; }
            // ddb9d mov QWORD PTR [rbp-0x10090],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ff70), (rax) as u64);
            // ddba4 lea r13,[rcx*4+0x0]
            r13 = rcx.wrapping_mul(4);
            // ddbac mov r15d,ebx
            r15 = ((rbx as u32 as u64)) as u32 as u64;
            pc = 0xddbaf;
        }
        0xddbaf => {
            // block ddbaf
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddbaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddbaf mov rdi,r12
            rdi = r12;
            // ddbb2 lea rsi,[rbp-0x8030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // ddbb9 mov rdx,r14
            rdx = r14;
            // ddbbc call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddbc1 add r12,r13
            { let (r, f) = add::<64>(r12, r13);
              r12 = r;
              fl = f; }
            // ddbc4 dec r15d
            { let (r, f) = dec::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // ddbc7 jne 0xddbaf
            if fl.ne() { pc = 0xddbaf; continue; }
            pc = 0xddbc9;
        }
        0xddbc9 => {
            // block ddbc9
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddbc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddbc9 mov r12,QWORD PTR [rbp-0x10090]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff70)) as u64;
            // ddbd0 jmp 0xddbd4
            pc = 0xddbd4; continue;
        }
        0xddbd2 => {
            // block ddbd2
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddbd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddbd2 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xddbd4;
        }
        0xddbd4 => {
            // block ddbd4
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddbd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddbd4 cmp ebx,DWORD PTR [rbp-0x1009c]
            fl = sub::<32>((rbx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff64)) as u64).1;
            // ddbda mov r13,rbx
            r13 = rbx;
            // ddbdd mov ebx,DWORD PTR [rbp-0x100bc]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff44)) as u64) as u32 as u64;
            // ddbe3 jae 0xddca6
            if fl.ae() { pc = 0xddca6; continue; }
            pc = 0xddbe9;
        }
        0xddbe9 => {
            // block ddbe9
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddbe9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddbe9 mov rax,QWORD PTR [rbp-0x10098]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff68)) as u64;
            // ddbf0 lea eax,[rax*4+0x0]
            rax = (rax.wrapping_mul(4)) as u32 as u64;
            // ddbf7 mov QWORD PTR [rbp-0x100a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_fffe_ff58), (rax) as u64);
            // ddbfe lea eax,[r13*8+0x0]
            rax = (r13.wrapping_mul(8)) as u32 as u64;
            // ddc06 lea r14,[rbp-0x10030]
            r14 = rbp.wrapping_add(0xffff_ffff_fffe_ffd0);
            // ddc0d xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xddc10;
        }
        0xddc10 => {
            // block ddc10
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddc10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddc10 mov DWORD PTR [rbp-0x10090],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_fffe_ff70), ((rax as u32 as u64)) as u32);
            // ddc16 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // ddc18 mov r8d,ebx
            r8 = ((rbx as u32 as u64)) as u32 as u64;
            // ddc1b lea rcx,[rbp-0x802a]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_7fd6);
            // ddc22 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // ddc24 je 0xddc5f
            if fl.e() { pc = 0xddc5f; continue; }
            pc = 0xddc26;
        }
        0xddc26 => {
            // block ddc26
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddc26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddc26 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // ddc28 and esi,0xfffe
            { let r = (rsi as u32 as u64) & 0xfffeu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ddc2e cmp esi,0x4095
            fl = sub::<32>((rsi as u32 as u64), 0x4095u64).1;
            // ddc34 movzx esi,ax
            rsi = ((rax as u16 as u64)) as u32 as u64;
            // ddc37 cmova esi,r15d
            if fl.a() { rsi = ((r15 as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // ddc3b mov edi,0x4095
            rdi = (0x4095u64) as u32 as u64;
            // ddc40 sub edi,esi
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rsi as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // ddc42 mov WORD PTR [rcx],di
            m.w16(rcx, ((rdi as u16 as u64)) as u16);
            // ddc45 shl esi,0x4
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x4u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // ddc48 mov WORD PTR [rcx-0x2],si
            m.w16(rcx.wrapping_add(0xffff_ffff_ffff_fffe), ((rsi as u16 as u64)) as u16);
            // ddc4c mov WORD PTR [rcx-0x6],si
            m.w16(rcx.wrapping_add(0xffff_ffff_ffff_fffa), ((rsi as u16 as u64)) as u16);
            // ddc50 mov WORD PTR [rcx-0x4],si
            m.w16(rcx.wrapping_add(0xffff_ffff_ffff_fffc), ((rsi as u16 as u64)) as u16);
            // ddc54 add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // ddc58 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // ddc5a dec r8d
            { let (r, f) = dec::<32>((r8 as u32 as u64), fl); r8 = (r) as u32 as u64; fl = f; }
            // ddc5d jne 0xddc26
            if fl.ne() { pc = 0xddc26; continue; }
            pc = 0xddc5f;
        }
        0xddc5f => {
            // block ddc5f
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddc5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddc5f lea rdi,[rbp-0x8030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // ddc66 mov rsi,r14
            rsi = r14;
            // ddc69 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // ddc6b call 0x67628
            { let r = { let t = m.env.code_va(0x67628); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddc70 mov rdi,r12
            rdi = r12;
            // ddc73 mov rsi,r14
            rsi = r14;
            // ddc76 mov rdx,QWORD PTR [rbp-0x100a8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff58)) as u64;
            // ddc7d call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddc82 mov rax,QWORD PTR [rbp-0x10098]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff68)) as u64;
            // ddc89 lea r12,[r12+rax*4]
            r12 = r12.wrapping_add(rax.wrapping_mul(4));
            // ddc8d inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // ddc90 mov eax,DWORD PTR [rbp-0x10090]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff70)) as u64) as u32 as u64;
            // ddc96 add eax,0x8
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // ddc99 cmp r13d,DWORD PTR [rbp-0x1009c]
            fl = sub::<32>((r13 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff64)) as u64).1;
            // ddca0 jne 0xddc10
            if fl.ne() { pc = 0xddc10; continue; }
            pc = 0xddca6;
        }
        0xddca6 => {
            // block ddca6
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddca6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddca6 mov r15,QWORD PTR [rbp-0x100b8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff48)) as u64;
            // ddcad test r15,r15
            fl = logic::<64>(r15 & r15);
            // ddcb0 mov rbx,QWORD PTR [rbp-0x100b0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff50)) as u64;
            // ddcb7 je 0xddd32
            if fl.e() { pc = 0xddd32; continue; }
            pc = 0xddcb9;
        }
        0xddcb9 => {
            // block ddcb9
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddcb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddcb9 mov al,BYTE PTR [rbx+0x279]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x279)) as u64) as u8 as u64);
            // ddcbf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // ddcc1 je 0xddd32
            if fl.e() { pc = 0xddd32; continue; }
            pc = 0xddcc3;
        }
        0xddcc3 => {
            // block ddcc3
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddcc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddcc3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddcc6 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddccc lea rdx,[rbp-0x10084]
            rdx = rbp.wrapping_add(0xffff_ffff_fffe_ff7c);
            // ddcd3 mov rdi,rbx
            rdi = rbx;
            // ddcd6 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddcdc mov r14d,DWORD PTR [rbp-0x10084]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff7c)) as u64) as u32 as u64;
            // ddce3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddce6 mov rdi,rbx
            rdi = rbx;
            // ddce9 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddcef cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // ddcf2 jbe 0xddd02
            if fl.be() { pc = 0xddd02; continue; }
            pc = 0xddcf4;
        }
        0xddcf4 => {
            // block ddcf4
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddcf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddcf4 mov DWORD PTR [rbp-0x10084],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_fffe_ff7c), (0x0u64) as u32);
            // ddcfe xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ddd00 jmp 0xddd08
            pc = 0xddd08; continue;
        }
        0xddd02 => {
            // block ddd02
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddd02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddd02 mov esi,DWORD PTR [rbp-0x10084]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff7c)) as u64) as u32 as u64;
            pc = 0xddd08;
        }
        0xddd08 => {
            // block ddd08
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddd08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddd08 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddd0b mov ecx,DWORD PTR [rbp-0x1009c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_fffe_ff64)) as u64) as u32 as u64;
            // ddd11 mov rdx,QWORD PTR [rbp-0x10098]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_fffe_ff68)) as u64;
            // ddd18 imul ecx,edx
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rdx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // ddd1b shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // ddd1e mov rdi,rbx
            rdi = rbx;
            // ddd21 mov rdx,r15
            rdx = r15;
            // ddd24 call QWORD PTR [rax+0x2c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddd2a mov rdi,r15
            rdi = r15;
            // ddd2d call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xddd32;
        }
        0xddd32 => {
            // block ddd32
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddd32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddd32 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddd35 movzx esi,BYTE PTR [rbx+0x278]
            rsi = (m.r8(rbx.wrapping_add(0x278)) as u64) as u32 as u64;
            // ddd3c mov rdi,rbx
            rdi = rbx;
            // ddd3f call QWORD PTR [rax+0x570]
            { let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddd45 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddd48 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddd4e mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // ddd54 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // ddd59 mov rdi,rbx
            rdi = rbx;
            // ddd5c call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddd62 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddd65 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddd6b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ddd6d mov rdi,rbx
            rdi = rbx;
            // ddd70 call QWORD PTR [rax+0x4f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddd76 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ddd79 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddd7f mov rdi,rbx
            rdi = rbx;
            // ddd82 call QWORD PTR [rax+0x618]
            { let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddd88 mov rax,QWORD PTR [rip+0x62649] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // ddd8f mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // ddd92 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // ddd96 jne 0xdddaa
            if fl.ne() { pc = 0xdddaa; continue; }
            pc = 0xddd98;
        }
        0xddd98 => {
            // block ddd98
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc ddd98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddd98 add rsp,0x100a8
            { let (r, f) = add::<64>(rsp, 0x100a8u64);
              rsp = r;
              fl = f; }
            // ddd9f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ddda0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ddda2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ddda4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ddda6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ddda8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ddda9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdddaa => {
            // block dddaa
            #[cfg(feature = "trace")] eprintln!("f_000dda0e pc dddaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dddaa call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdc4d6 size=960
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000dc4d6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdc4d6;
    loop {
        match pc {
        0xdc4d6 => {
            // block dc4d6
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc4d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc4d6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc4d7 mov rbp,rsp
            rbp = rsp;
            // dc4da push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dc4dc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dc4de push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dc4e0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dc4e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dc4e3 sub rsp,0xa0a8
            { let (r, f) = sub::<64>(rsp, 0xa0a8u64);
              rsp = r;
              fl = f; }
            // dc4ea mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // dc4ec mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // dc4ef mov r14,rdi
            r14 = rdi;
            // dc4f2 mov rax,QWORD PTR [rip+0x63edf] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // dc4f9 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dc4fc mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // dc500 lea r15,[rbp-0xa030]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dc507 mov esi,0x800
            rsi = (0x800u64) as u32 as u64;
            // dc50c mov rdi,r15
            rdi = r15;
            // dc50f call 0x5932c
            { let r = { let t = m.env.code_va(0x5932c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc514 lea rsi,[rbp-0x8030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // dc51b mov edx,0x800
            rdx = (0x800u64) as u32 as u64;
            // dc520 mov rdi,r15
            rdi = r15;
            // dc523 call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc528 lea rdi,[rbp-0xa070]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5f90);
            // dc52f xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // dc532 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dc534 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dc536 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dc539 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // dc53c mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // dc53e call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc543 mov r15d,DWORD PTR [rbp-0xa068]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f98)) as u64) as u32 as u64;
            // dc54a cmp ebx,0x4
            fl = sub::<32>((rbx as u32 as u64), 0x4u64).1;
            // dc54d cmove ebx,r13d
            if fl.e() { rbx = ((r13 as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // dc551 cmp ebx,0x4
            fl = sub::<32>((rbx as u32 as u64), 0x4u64).1;
            // dc554 jae 0xdc656
            if fl.ae() { pc = 0xdc656; continue; }
            pc = 0xdc55a;
        }
        0xdc55a => {
            // block dc55a
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc55a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc55a mov DWORD PTR [rbp-0xa088],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f78), ((rbx as u32 as u64)) as u32);
            // dc560 mov eax,DWORD PTR [rbp-0xa06c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f94)) as u64) as u32 as u64;
            // dc566 mov DWORD PTR [rbp-0xa0ac],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f54), ((rax as u32 as u64)) as u32);
            // dc56c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // dc56e je 0xdc662
            if fl.e() { pc = 0xdc662; continue; }
            pc = 0xdc574;
        }
        0xdc574 => {
            // block dc574
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc574 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc574 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // dc577 je 0xdc662
            if fl.e() { pc = 0xdc662; continue; }
            pc = 0xdc57d;
        }
        0xdc57d => {
            // block dc57d
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc57d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc57d mov QWORD PTR [rbp-0xa090],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f70), (r12) as u64);
            // dc584 mov r13d,DWORD PTR [rbp-0xa064]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f9c)) as u64) as u32 as u64;
            // dc58b movsxd rbx,DWORD PTR [rbp-0xa088]
            rbx = sx::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u64;
            // dc592 mov r12,QWORD PTR [r14+0x280]
            r12 = m.r64(r14.wrapping_add(0x280)) as u64;
            // dc599 mov QWORD PTR [rbp-0xa0a8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f58), (r14) as u64);
            // dc5a0 lea rdi,[rbp-0xa030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dc5a7 mov esi,0x800
            rsi = (0x800u64) as u32 as u64;
            // dc5ac call 0x5932c
            { let r = { let t = m.env.code_va(0x5932c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc5b1 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // dc5b4 je 0xdc67a
            if fl.e() { pc = 0xdc67a; continue; }
            pc = 0xdc5ba;
        }
        0xdc5ba => {
            // block dc5ba
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc5ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc5ba mov QWORD PTR [rbp-0xa098],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f68), (rbx) as u64);
            // dc5c1 lea eax,[r15*4+0x0]
            rax = (r15.wrapping_mul(4)) as u32 as u64;
            // dc5c9 mov QWORD PTR [rbp-0xa0a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f60), (rax) as u64);
            // dc5d0 mov QWORD PTR [rbp-0xa0c0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f40), (r15) as u64);
            // dc5d7 shl r15,0x2
            { let (r, f) = shl::<64>(r15, 0x2u64, fl); r15 = r; fl = f; }
            // dc5db mov QWORD PTR [rbp-0xa080],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f80), (r15) as u64);
            // dc5e2 lea r15,[rbp-0x8030]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // dc5e9 mov QWORD PTR [rbp-0xa0d0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f30), (r12) as u64);
            // dc5f0 mov r14d,r13d
            r14 = ((r13 as u32 as u64)) as u32 as u64;
            // dc5f3 mov DWORD PTR [rbp-0xa0c4],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f3c), ((r13 as u32 as u64)) as u32);
            // dc5fa mov rbx,QWORD PTR [rbp-0xa0a8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f58)) as u64;
            // dc601 mov r13d,DWORD PTR [rbp-0xa0ac]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f54)) as u64) as u32 as u64;
            pc = 0xdc608;
        }
        0xdc608 => {
            // block dc608
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc608 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc608 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dc60b mov rdi,rbx
            rdi = rbx;
            // dc60e lea rsi,[rbp-0xa030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dc615 mov rdx,r15
            rdx = r15;
            // dc618 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // dc61b call QWORD PTR [rax+0x19e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc621 mov rdi,r12
            rdi = r12;
            // dc624 mov rsi,r15
            rsi = r15;
            // dc627 mov rdx,QWORD PTR [rbp-0xa0a0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f60)) as u64;
            // dc62e call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc633 add r12,QWORD PTR [rbp-0xa080]
            { let (r, f) = add::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f80)) as u64);
              r12 = r;
              fl = f; }
            // dc63a dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // dc63d jne 0xdc608
            if fl.ne() { pc = 0xdc608; continue; }
            pc = 0xdc63f;
        }
        0xdc63f => {
            // block dc63f
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc63f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc63f mov r13,QWORD PTR [rbp-0xa0a0]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f60)) as u64;
            // dc646 mov r15,QWORD PTR [rbp-0xa0c0]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f40)) as u64;
            // dc64d mov rbx,QWORD PTR [rbp-0xa098]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f68)) as u64;
            // dc654 jmp 0xdc690
            pc = 0xdc690; continue;
        }
        0xdc656 => {
            // block dc656
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc656 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc656 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dc659 lea rsi,[rip+0x5cbee] # 0x13924e
            rsi = m.env.data_va(317979);
            // dc660 jmp 0xdc66c
            pc = 0xdc66c; continue;
        }
        0xdc662 => {
            // block dc662
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc662 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dc665 lea rsi,[rip+0x5cc17] # 0x139283
            rsi = m.env.data_va(318032);
            pc = 0xdc66c;
        }
        0xdc66c => {
            // block dc66c
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc66c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc66c mov rdi,r14
            rdi = r14;
            // dc66f call QWORD PTR [rax+0x170]
            { let r = { let t = m.r64(rax.wrapping_add(0x170)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc675 jmp 0xdc86f
            pc = 0xdc86f; continue;
        }
        0xdc67a => {
            // block dc67a
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc67a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc67a mov QWORD PTR [rbp-0xa0d0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f30), (r12) as u64);
            // dc681 mov DWORD PTR [rbp-0xa0c4],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f3c), ((r13 as u32 as u64)) as u32);
            // dc688 lea r13d,[r15*4+0x0]
            r13 = (r15.wrapping_mul(4)) as u32 as u64;
            pc = 0xdc690;
        }
        0xdc690 => {
            // block dc690
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc690 mov QWORD PTR [rbp-0xa0a0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f60), (r13) as u64);
            // dc697 mov QWORD PTR [rbp-0xa0c0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f40), (r15) as u64);
            // dc69e lea rax,[rip+0x4b92b] # 0x127fd0
            rax = m.env.data_va(247709);
            // dc6a5 mov rax,QWORD PTR [rax+rbx*8]
            rax = m.r64(rax.wrapping_add(rbx.wrapping_mul(8))) as u64;
            // dc6a9 mov QWORD PTR [rbp-0xa0b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f48), (rax) as u64);
            // dc6b0 mov edx,DWORD PTR [rbp-0xa088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // dc6b6 mov rax,r15
            rax = r15;
            // dc6b9 shl rax,0x2
            { let (r, f) = shl::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // dc6bd mov QWORD PTR [rbp-0xa080],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f80), (rax) as u64);
            // dc6c4 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dc6c6 imul rax,QWORD PTR [rbp-0xa090],0x608
            { let (r, f) = imul::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f70)) as u64, 0x608u64); rax = r; fl = f; }
            // dc6d1 lea rcx,[rip+0xe10c8] # 0x1bd7a0
            rcx = m.env.data_va(860013);
            // dc6d8 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // dc6db shl rdx,0x7
            { let (r, f) = shl::<64>(rdx, 0x7u64, fl); rdx = r; fl = f; }
            // dc6df add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // dc6e2 mov QWORD PTR [rbp-0xa090],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f70), (rdx) as u64);
            // dc6e9 lea rbx,[rbp-0x8030]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            pc = 0xdc6f0;
        }
        0xdc6f0 => {
            // block dc6f0
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc6f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc6f0 mov QWORD PTR [rbp-0xa088],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f78), (rsi) as u64);
            // dc6f7 mov rax,rsi
            rax = rsi;
            // dc6fa shl rax,0x4
            { let (r, f) = shl::<64>(rax, 0x4u64, fl); rax = r; fl = f; }
            // dc6fe mov rcx,QWORD PTR [rbp-0xa090]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f70)) as u64;
            // dc705 mov rsi,QWORD PTR [rax+rcx*1+0x10]
            rsi = m.r64(rax.wrapping_add(rcx).wrapping_add(0x10)) as u64;
            // dc70a test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // dc70d je 0xdc85b
            if fl.e() { pc = 0xdc85b; continue; }
            pc = 0xdc713;
        }
        0xdc713 => {
            // block dc713
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc713 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc713 mov rcx,QWORD PTR [rbp-0xa090]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f70)) as u64;
            // dc71a mov edx,DWORD PTR [rax+rcx*1+0x8]
            rdx = (m.r32(rax.wrapping_add(rcx).wrapping_add(0x8)) as u64) as u32 as u64;
            // dc71e mov DWORD PTR [rbp-0xa098],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f68), ((rdx as u32 as u64)) as u32);
            // dc724 mov r14d,DWORD PTR [rax+rcx*1+0xc]
            r14 = (m.r32(rax.wrapping_add(rcx).wrapping_add(0xc)) as u64) as u32 as u64;
            // dc729 mov rdi,rbx
            rdi = rbx;
            // dc72c mov rdx,QWORD PTR [rbp-0xa0b8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f48)) as u64;
            // dc733 mov r12,rbx
            r12 = rbx;
            // dc736 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc73b mov r15d,DWORD PTR [rbp-0xa0ac]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f54)) as u64) as u32 as u64;
            // dc742 cmp r15d,0x800
            fl = sub::<32>((r15 as u32 as u64), 0x800u64).1;
            // dc749 jne 0xdc75c
            if fl.ne() { pc = 0xdc75c; continue; }
            pc = 0xdc74b;
        }
        0xdc74b => {
            // block dc74b
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc74b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc74b mov edx,0x780
            rdx = (0x780u64) as u32 as u64;
            // dc750 mov rdi,r12
            rdi = r12;
            // dc753 lea rsi,[rbp-0xa030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dc75a jmp 0xdc769
            pc = 0xdc769; continue;
        }
        0xdc75c => {
            // block dc75c
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc75c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc75c mov rdi,r12
            rdi = r12;
            // dc75f lea rsi,[rbp-0xa030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dc766 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            pc = 0xdc769;
        }
        0xdc769 => {
            // block dc769
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc769 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc769 call 0x58fd6
            { let r = { let t = m.env.code_va(0x58fd6); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc76e mov r13,QWORD PTR [rbp-0xa0a8]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f58)) as u64;
            // dc775 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // dc77a sub ebx,DWORD PTR [rbp-0xa098]
            { let (r, f) = sub::<32>((rbx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f68)) as u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // dc780 movzx eax,WORD PTR [r13+0x270]
            rax = (m.r16(r13.wrapping_add(0x270)) as u64) as u32 as u64;
            // dc788 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // dc78b je 0xdc79f
            if fl.e() { pc = 0xdc79f; continue; }
            pc = 0xdc78d;
        }
        0xdc78d => {
            // block dc78d
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc78d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc78d movzx esi,ax
            rsi = ((rax as u16 as u64)) as u32 as u64;
            // dc790 lea rdi,[rbp-0xa030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dc797 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // dc79a call 0x591eb
            { let r = { let t = m.env.code_va(0x591eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdc79f;
        }
        0xdc79f => {
            // block dc79f
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc79f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc79f add ebx,r14d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r14 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // dc7a2 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // dc7a6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dc7a8 mov rdi,r13
            rdi = r13;
            // dc7ab lea rsi,[rbp-0xa071]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5f8f);
            // dc7b2 call QWORD PTR [rax+0x410]
            { let r = { let t = m.r64(rax.wrapping_add(0x410)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc7b8 cmp DWORD PTR [r13+0x274],0x0
            fl = sub::<32>(m.r32(r13.wrapping_add(0x274)) as u64, 0x0u64).1;
            // dc7c0 je 0xdc7df
            if fl.e() { pc = 0xdc7df; continue; }
            pc = 0xdc7c2;
        }
        0xdc7c2 => {
            // block dc7c2
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc7c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc7c2 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // dc7c6 mov rdi,r13
            rdi = r13;
            // dc7c9 mov QWORD PTR [rbp-0xa0a8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f58), (r13) as u64);
            // dc7d0 lea rsi,[rip+0x5caed] # 0x1392c4
            rsi = m.env.data_va(318097);
            // dc7d7 call QWORD PTR [rax+0x170]
            { let r = { let t = m.r64(rax.wrapping_add(0x170)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc7dd jmp 0xdc7f8
            pc = 0xdc7f8; continue;
        }
        0xdc7df => {
            // block dc7df
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc7df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc7df mov QWORD PTR [rbp-0xa0a8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f58), (r13) as u64);
            // dc7e6 lea rdi,[rbp-0xa030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dc7ed mov rsi,r12
            rsi = r12;
            // dc7f0 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // dc7f3 call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdc7f8;
        }
        0xdc7f8 => {
            // block dc7f8
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc7f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc7f8 and ebx,0xffff
            { let r = (rbx as u32 as u64) & 0xffffu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // dc7fe mov r13,QWORD PTR [rbp-0xa0a0]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f60)) as u64;
            // dc805 mov r14d,ebx
            r14 = ((rbx as u32 as u64)) as u32 as u64;
            // dc808 mov rbx,r12
            rbx = r12;
            // dc80b je 0xdc85b
            if fl.e() { pc = 0xdc85b; continue; }
            pc = 0xdc80d;
        }
        0xdc80d => {
            // block dc80d
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc80d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc80d mov eax,DWORD PTR [rbp-0xa098]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f68)) as u64) as u32 as u64;
            // dc813 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // dc816 add eax,DWORD PTR [rbp-0xa0c4]
            { let (r, f) = add::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f3c)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // dc81c mov rcx,QWORD PTR [rbp-0xa0c0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f40)) as u64;
            // dc823 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dc826 mov rcx,QWORD PTR [rbp-0xa0d0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f30)) as u64;
            // dc82d lea r12,[rcx+rax*4]
            r12 = rcx.wrapping_add(rax.wrapping_mul(4));
            // dc831 mov r15w,0x1
            r15 = (r15 & !0xffff) | ((0x1u64) as u16 as u64);
            pc = 0xdc836;
        }
        0xdc836 => {
            // block dc836
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc836 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc836 mov rdi,r12
            rdi = r12;
            // dc839 mov rsi,rbx
            rsi = rbx;
            // dc83c mov rdx,r13
            rdx = r13;
            // dc83f call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc844 mov rcx,QWORD PTR [rbp-0xa080]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f80)) as u64;
            // dc84b movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // dc84f lea r15d,[r15+0x1]
            r15 = (r15.wrapping_add(0x1)) as u32 as u64;
            // dc853 add r12,rcx
            { let (r, f) = add::<64>(r12, rcx);
              r12 = r;
              fl = f; }
            // dc856 cmp eax,r14d
            fl = sub::<32>((rax as u32 as u64), (r14 as u32 as u64)).1;
            // dc859 jb 0xdc836
            if fl.b() { pc = 0xdc836; continue; }
            pc = 0xdc85b;
        }
        0xdc85b => {
            // block dc85b
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc85b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc85b mov rsi,QWORD PTR [rbp-0xa088]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64;
            // dc862 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // dc865 cmp rsi,0x8
            fl = sub::<64>(rsi, 0x8u64).1;
            // dc869 jne 0xdc6f0
            if fl.ne() { pc = 0xdc6f0; continue; }
            pc = 0xdc86f;
        }
        0xdc86f => {
            // block dc86f
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc86f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc86f mov rax,QWORD PTR [rip+0x63b62] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // dc876 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dc879 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // dc87d jne 0xdc891
            if fl.ne() { pc = 0xdc891; continue; }
            pc = 0xdc87f;
        }
        0xdc87f => {
            // block dc87f
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc87f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc87f add rsp,0xa0a8
            { let (r, f) = add::<64>(rsp, 0xa0a8u64);
              rsp = r;
              fl = f; }
            // dc886 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc887 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dc889 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dc88b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc88d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dc88f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc890 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdc891 => {
            // block dc891
            #[cfg(feature = "trace")] eprintln!("f_000dc4d6 pc dc891 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc891 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb232 size=982
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eb232<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeb232;
    loop {
        match pc {
        0xeb232 => {
            // block eb232
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb232 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb232 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb233 mov rbp,rsp
            rbp = rsp;
            // eb236 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb238 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb23a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // eb23c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eb23e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb23f sub rsp,0x308
            { let (r, f) = sub::<64>(rsp, 0x308u64);
              rsp = r;
              fl = f; }
            // eb246 mov r13,rsi
            r13 = rsi;
            // eb249 mov rbx,rdi
            rbx = rdi;
            // eb24c lea r14,[rbp-0x2b0]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb253 mov rdi,r14
            rdi = r14;
            // eb256 call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb25b mov edi,DWORD PTR [rbx+0x5c]
            rdi = (m.r32(rbx.wrapping_add(0x5c)) as u64) as u32 as u64;
            // eb25e mov rsi,r14
            rsi = r14;
            // eb261 call 0x17202
            { let r = { let t = m.env.code_va(0x17202); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb266 mov ebx,0xffffffff
            rbx = (0xffffffffu64) as u32 as u64;
            // eb26b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb26d je 0xeb5ca
            if fl.e() { pc = 0xeb5ca; continue; }
            pc = 0xeb273;
        }
        0xeb273 => {
            // block eb273
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb273 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb273 mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // eb27a mov rax,QWORD PTR [rax+0x3e8]
            rax = m.r64(rax.wrapping_add(0x3e8)) as u64;
            // eb281 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb288 lea rsi,[rbp-0x2b4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd4c);
            // eb28f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eb291 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb293 mov esi,DWORD PTR [rbp-0x2b4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64) as u32 as u64;
            // eb299 lea rdi,[rbp-0x2f0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd10);
            // eb2a0 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // eb2a5 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eb2a7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // eb2aa xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // eb2ad call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb2b2 mov eax,DWORD PTR [rbp-0x2f0]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd10)) as u64) as u32 as u64;
            // eb2b8 mov r15d,DWORD PTR [rbp-0x2ec]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd14)) as u64) as u32 as u64;
            // eb2bf mov r12d,DWORD PTR [rbp-0x2e8]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64) as u32 as u64;
            // eb2c6 shl r12d,0x2
            { let (r, f) = shl::<32>((r12 as u32 as u64), 0x2u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // eb2ca xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // eb2cc xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // eb2cf test r13,r13
            fl = logic::<64>(r13 & r13);
            // eb2d2 je 0xeb399
            if fl.e() { pc = 0xeb399; continue; }
            pc = 0xeb2d8;
        }
        0xeb2d8 => {
            // block eb2d8
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb2d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb2d8 mov QWORD PTR [rbp-0x320],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce0), (rax) as u64);
            // eb2df mov DWORD PTR [rbp-0x324],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc), ((r12 as u32 as u64)) as u32);
            // eb2e6 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb2ea mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // eb2ee mov rdi,r13
            rdi = r13;
            // eb2f1 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb2f3 mov QWORD PTR [rbp-0x310],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0), (rax) as u64);
            // eb2fa mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb2fe mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // eb302 mov rdi,r13
            rdi = r13;
            // eb305 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb307 mov DWORD PTR [rbp-0x2f8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd08), ((rax as u32 as u64)) as u32);
            // eb30d mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb311 mov rax,QWORD PTR [rax+0x40]
            rax = m.r64(rax.wrapping_add(0x40)) as u64;
            // eb315 mov rdi,r13
            rdi = r13;
            // eb318 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb31a mov DWORD PTR [rbp-0x300],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd00), ((rax as u32 as u64)) as u32);
            // eb320 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb324 mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // eb328 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eb32a mov rdi,r13
            rdi = r13;
            // eb32d call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb32f mov rbx,rax
            rbx = rax;
            // eb332 mov rax,QWORD PTR [rbp-0x320]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // eb339 cmp DWORD PTR [rbp-0x2f8],eax
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd08)) as u64, (rax as u32 as u64)).1;
            // eb33f setne r12b
            r12 = (r12 & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // eb343 mov rax,QWORD PTR [rbp-0x310]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0)) as u64;
            // eb34a cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // eb34d setne r14b
            r14 = (r14 & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // eb351 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb355 mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // eb359 mov rdi,r13
            rdi = r13;
            // eb35c call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb35e or r14b,r12b
            { let r = (r14 as u8 as u64) | (r12 as u8 as u64); let f = logic::<8>(r);
              r14 = (r14 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // eb361 mov cl,0x1
            rcx = (rcx & !0xff) | ((0x1u64) as u8 as u64);
            // eb363 mov QWORD PTR [rbp-0x318],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rcx) as u64);
            // eb36a cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // eb36d je 0xeb389
            if fl.e() { pc = 0xeb389; continue; }
            pc = 0xeb36f;
        }
        0xeb36f => {
            // block eb36f
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb36f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb36f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb373 mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // eb377 mov rdi,r13
            rdi = r13;
            // eb37a call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb37c cmp eax,0xa
            fl = sub::<32>((rax as u32 as u64), 0xau64).1;
            // eb37f sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // eb382 mov QWORD PTR [rbp-0x318],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rax) as u64);
            pc = 0xeb389;
        }
        0xeb389 => {
            // block eb389
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb389 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb389 mov r12d,DWORD PTR [rbp-0x324]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc)) as u64) as u32 as u64;
            // eb390 mov rcx,QWORD PTR [rbp-0x310]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0)) as u64;
            // eb397 jmp 0xeb3bb
            pc = 0xeb3bb; continue;
        }
        0xeb399 => {
            // block eb399
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb399 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb399 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // eb39c mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // eb39e mov DWORD PTR [rbp-0x2f8],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd08), ((rdx as u32 as u64)) as u32);
            // eb3a4 mov QWORD PTR [rbp-0x320],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce0), (rax) as u64);
            // eb3ab mov DWORD PTR [rbp-0x300],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd00), ((r12 as u32 as u64)) as u32);
            // eb3b2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // eb3b4 mov QWORD PTR [rbp-0x318],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce8), (rax) as u64);
            pc = 0xeb3bb;
        }
        0xeb3bb => {
            // block eb3bb
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3bb mov QWORD PTR [rbp-0x310],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0), (rcx) as u64);
            // eb3c2 lea r15d,[rcx*4+0x0]
            r15 = (rcx.wrapping_mul(4)) as u32 as u64;
            // eb3ca test r13,r13
            fl = logic::<64>(r13 & r13);
            // eb3cd sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // eb3d0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // eb3d2 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // eb3d5 jne 0xeb401
            if fl.ne() { pc = 0xeb401; continue; }
            pc = 0xeb3d7;
        }
        0xeb3d7 => {
            // block eb3d7
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3d7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // eb3d9 jne 0xeb401
            if fl.ne() { pc = 0xeb401; continue; }
            pc = 0xeb3db;
        }
        0xeb3db => {
            // block eb3db
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3db mov rcx,QWORD PTR [rbp-0x318]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce8)) as u64;
            // eb3e2 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // eb3e4 jne 0xeb401
            if fl.ne() { pc = 0xeb401; continue; }
            pc = 0xeb3e6;
        }
        0xeb3e6 => {
            // block eb3e6
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3e6 cmp DWORD PTR [rbp-0x300],r12d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64, (r12 as u32 as u64)).1;
            // eb3ed jne 0xeb401
            if fl.ne() { pc = 0xeb401; continue; }
            pc = 0xeb3ef;
        }
        0xeb3ef => {
            // block eb3ef
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3ef cmp DWORD PTR [rbp-0x300],r15d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64, (r15 as u32 as u64)).1;
            // eb3f6 jne 0xeb401
            if fl.ne() { pc = 0xeb401; continue; }
            pc = 0xeb3f8;
        }
        0xeb3f8 => {
            // block eb3f8
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb3f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb3f8 cmp DWORD PTR [rbp-0x300],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64, 0x0u64).1;
            // eb3ff jns 0xeb413
            if fl.ns() { pc = 0xeb413; continue; }
            pc = 0xeb401;
        }
        0xeb401 => {
            // block eb401
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb401 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb401 mov rax,QWORD PTR [rbp-0x320]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // eb408 movsxd rdi,eax
            rdi = sx::<32>((rax as u32 as u64)) as u64;
            // eb40b movsxd rsi,r12d
            rsi = sx::<32>((r12 as u32 as u64)) as u64;
            // eb40e call 0xec1ac
            { let r = { let t = m.env.stub_va(0x979); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb413;
        }
        0xeb413 => {
            // block eb413
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb413 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb413 mov DWORD PTR [rbp-0x324],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc), ((r12 as u32 as u64)) as u32);
            // eb41a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // eb41d mov r12,rax
            r12 = rax;
            // eb420 cmove rbx,r12
            if fl.e() { rbx = r12; }
            // eb424 test r13,r13
            fl = logic::<64>(r13 & r13);
            // eb427 je 0xeb436
            if fl.e() { pc = 0xeb436; continue; }
            pc = 0xeb429;
        }
        0xeb429 => {
            // block eb429
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb429 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb429 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb42d mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // eb431 mov rdi,r13
            rdi = r13;
            // eb434 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb436;
        }
        0xeb436 => {
            // block eb436
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb436 test r12,r12
            fl = logic::<64>(r12 & r12);
            // eb439 je 0xeb579
            if fl.e() { pc = 0xeb579; continue; }
            pc = 0xeb43f;
        }
        0xeb43f => {
            // block eb43f
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb43f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb43f cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // eb442 je 0xeb579
            if fl.e() { pc = 0xeb579; continue; }
            pc = 0xeb448;
        }
        0xeb448 => {
            // block eb448
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb448 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb448 mov rax,QWORD PTR [rbp-0x320]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // eb44f lea eax,[rax-0x1]
            rax = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // eb452 mov ecx,DWORD PTR [rbp-0x2f8]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd08)) as u64) as u32 as u64;
            // eb458 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // eb45a cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // eb45c cmovg ecx,eax
            if fl.g() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // eb45f mov DWORD PTR [rbp-0x2f8],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd08), ((rcx as u32 as u64)) as u32);
            // eb465 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // eb467 mov r14,r12
            r14 = r12;
            // eb46a js 0xeb57c
            if fl.s() { pc = 0xeb57c; continue; }
            pc = 0xeb470;
        }
        0xeb470 => {
            // block eb470
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb470 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb470 movsxd rax,DWORD PTR [rbp-0x300]
            rax = sx::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64) as u64;
            // eb477 mov QWORD PTR [rbp-0x300],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd00), (rax) as u64);
            // eb47e movsxd rax,DWORD PTR [rbp-0x324]
            rax = sx::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc)) as u64) as u64;
            // eb485 mov QWORD PTR [rbp-0x308],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcf8), (rax) as u64);
            // eb48c movsxd r14,r15d
            r14 = sx::<32>((r15 as u32 as u64)) as u64;
            // eb48f mov r15,r12
            r15 = r12;
            // eb492 mov QWORD PTR [rbp-0x330],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0), (r15) as u64);
            // eb499 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xeb49c;
        }
        0xeb49c => {
            // block eb49c
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb49c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb49c test r13,r13
            fl = logic::<64>(r13 & r13);
            // eb49f je 0xeb4f4
            if fl.e() { pc = 0xeb4f4; continue; }
            pc = 0xeb4a1;
        }
        0xeb4a1 => {
            // block eb4a1
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4a1 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // eb4a5 mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // eb4a9 mov rdi,r13
            rdi = r13;
            // eb4ac call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb4ae cmp eax,0xa
            fl = sub::<32>((rax as u32 as u64), 0xau64).1;
            // eb4b1 jne 0xeb4f4
            if fl.ne() { pc = 0xeb4f4; continue; }
            pc = 0xeb4b3;
        }
        0xeb4b3 => {
            // block eb4b3
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4b3 mov rax,QWORD PTR [rbp-0x310]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0)) as u64;
            // eb4ba test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // eb4bc mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // eb4c1 jle 0xeb54a
            if fl.le() { pc = 0xeb54a; continue; }
            pc = 0xeb4c7;
        }
        0xeb4c7 => {
            // block eb4c7
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4c7 mov cl,BYTE PTR [rbx+rax*1+0x3]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x3)) as u64) as u8 as u64);
            // eb4cb mov BYTE PTR [r15+rax*1],cl
            m.w8(r15.wrapping_add(rax), ((rcx as u8 as u64)) as u8);
            // eb4cf mov cl,BYTE PTR [rbx+rax*1]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax)) as u64) as u8 as u64);
            // eb4d2 mov BYTE PTR [r15+rax*1+0x1],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x1), ((rcx as u8 as u64)) as u8);
            // eb4d7 mov cl,BYTE PTR [rbx+rax*1+0x1]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x1)) as u64) as u8 as u64);
            // eb4db mov BYTE PTR [r15+rax*1+0x2],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x2), ((rcx as u8 as u64)) as u8);
            // eb4e0 mov cl,BYTE PTR [rbx+rax*1+0x2]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x2)) as u64) as u8 as u64);
            // eb4e4 mov BYTE PTR [r15+rax*1+0x3],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x3), ((rcx as u8 as u64)) as u8);
            // eb4e9 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // eb4ed cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // eb4f0 jl 0xeb4c7
            if fl.l() { pc = 0xeb4c7; continue; }
            pc = 0xeb4f2;
        }
        0xeb4f2 => {
            // block eb4f2
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4f2 jmp 0xeb54a
            pc = 0xeb54a; continue;
        }
        0xeb4f4 => {
            // block eb4f4
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4f4 mov rax,QWORD PTR [rbp-0x318]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce8)) as u64;
            // eb4fb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb4fd je 0xeb53c
            if fl.e() { pc = 0xeb53c; continue; }
            pc = 0xeb4ff;
        }
        0xeb4ff => {
            // block eb4ff
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb4ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb4ff mov rax,QWORD PTR [rbp-0x310]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcf0)) as u64;
            // eb506 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // eb508 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // eb50d jle 0xeb54a
            if fl.le() { pc = 0xeb54a; continue; }
            pc = 0xeb50f;
        }
        0xeb50f => {
            // block eb50f
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb50f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb50f mov cl,BYTE PTR [rbx+rax*1+0x3]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x3)) as u64) as u8 as u64);
            // eb513 mov BYTE PTR [r15+rax*1],cl
            m.w8(r15.wrapping_add(rax), ((rcx as u8 as u64)) as u8);
            // eb517 mov cl,BYTE PTR [rbx+rax*1+0x2]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x2)) as u64) as u8 as u64);
            // eb51b mov BYTE PTR [r15+rax*1+0x1],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x1), ((rcx as u8 as u64)) as u8);
            // eb520 mov cl,BYTE PTR [rbx+rax*1+0x1]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax).wrapping_add(0x1)) as u64) as u8 as u64);
            // eb524 mov BYTE PTR [r15+rax*1+0x2],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x2), ((rcx as u8 as u64)) as u8);
            // eb529 mov cl,BYTE PTR [rbx+rax*1]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(rax)) as u64) as u8 as u64);
            // eb52c mov BYTE PTR [r15+rax*1+0x3],cl
            m.w8(r15.wrapping_add(rax).wrapping_add(0x3), ((rcx as u8 as u64)) as u8);
            // eb531 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // eb535 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // eb538 jl 0xeb50f
            if fl.l() { pc = 0xeb50f; continue; }
            pc = 0xeb53a;
        }
        0xeb53a => {
            // block eb53a
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb53a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb53a jmp 0xeb54a
            pc = 0xeb54a; continue;
        }
        0xeb53c => {
            // block eb53c
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb53c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb53c mov rdi,r15
            rdi = r15;
            // eb53f mov rsi,rbx
            rsi = rbx;
            // eb542 mov rdx,r14
            rdx = r14;
            // eb545 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb54a;
        }
        0xeb54a => {
            // block eb54a
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb54a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb54a add rbx,QWORD PTR [rbp-0x300]
            { let (r, f) = add::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd00)) as u64);
              rbx = r;
              fl = f; }
            // eb551 add r15,QWORD PTR [rbp-0x308]
            { let (r, f) = add::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcf8)) as u64);
              r15 = r;
              fl = f; }
            // eb558 cmp r12d,DWORD PTR [rbp-0x2f8]
            fl = sub::<32>((r12 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd08)) as u64).1;
            // eb55f lea eax,[r12+0x1]
            rax = (r12.wrapping_add(0x1)) as u32 as u64;
            // eb564 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // eb567 jl 0xeb49c
            if fl.l() { pc = 0xeb49c; continue; }
            pc = 0xeb56d;
        }
        0xeb56d => {
            // block eb56d
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb56d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb56d mov r12,QWORD PTR [rbp-0x330]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // eb574 mov r14,r12
            r14 = r12;
            // eb577 jmp 0xeb57c
            pc = 0xeb57c; continue;
        }
        0xeb579 => {
            // block eb579
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb579 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb579 mov r14,rbx
            r14 = rbx;
            pc = 0xeb57c;
        }
        0xeb57c => {
            // block eb57c
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb57c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb57c mov DWORD PTR [rbp-0x2f4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd0c), (0x0u64) as u32);
            // eb586 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb58d lea rdx,[rbp-0x2f4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fd0c);
            // eb594 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eb596 call 0x2b74
            { let r = { let t = m.env.code_va(0x2b74); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb59b mov esi,DWORD PTR [rbp-0x2f4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd0c)) as u64) as u32 as u64;
            // eb5a1 mov ecx,DWORD PTR [rbp-0x324]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fcdc)) as u64) as u32 as u64;
            // eb5a7 mov rax,QWORD PTR [rbp-0x320]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fce0)) as u64;
            // eb5ae imul ecx,eax
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rax as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // eb5b1 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb5b8 mov rdx,r14
            rdx = r14;
            // eb5bb call 0x44d68
            { let r = { let t = m.env.code_va(0x44d68); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb5c0 mov rdi,r12
            rdi = r12;
            // eb5c3 call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb5c8 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xeb5ca;
        }
        0xeb5ca => {
            // block eb5ca
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb5ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb5ca lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb5d1 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb5d6 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // eb5d8 add rsp,0x308
            { let (r, f) = add::<64>(rsp, 0x308u64);
              rsp = r;
              fl = f; }
            // eb5df pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb5e0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eb5e2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eb5e4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb5e6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb5e8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb5e9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeb5ea => {
            // block eb5ea
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb5ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb5ea jmp 0xeb5ec
            pc = 0xeb5ec; continue;
        }
        0xeb5ec => {
            // block eb5ec
            #[cfg(feature = "trace")] eprintln!("f_000eb232 pc eb5ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb5ec mov rbx,rax
            rbx = rax;
            // eb5ef lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eb5f6 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb5fb mov rdi,rbx
            rdi = rbx;
            // eb5fe call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
