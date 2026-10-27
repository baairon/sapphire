//! Mechanically lifted functions (scripts/lift_run.py), module q001. Each one is checked against the original
//! in the world harness (tests/world_q001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x2156 size=254
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002156<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2156;
    loop {
        match pc {
        0x2156 => {
            // block 2156
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2156 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2156 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2157 mov rbp,rsp
            rbp = rsp;
            // 215a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 215c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 215e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2160 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2161 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2165 mov r14,rdx
            r14 = rdx;
            // 2168 mov r15,rsi
            r15 = rsi;
            // 216b mov r12,rdi
            r12 = rdi;
            // 216e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2172 call QWORD PTR [rax+0x418]
            { let r = { let t = m.r64(rax.wrapping_add(0x418)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2178 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 217a je 0x21c6
            if fl.e() { pc = 0x21c6; continue; }
            pc = 0x217c;
        }
        0x217c => {
            // block 217c
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 217c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 217c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2180 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 2184 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 2186 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 218b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2190 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2193 mov rdi,r12
            rdi = r12;
            // 2196 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2199 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 219b je 0x2244
            if fl.e() { pc = 0x2244; continue; }
            pc = 0x21a1;
        }
        0x21a1 => {
            // block 21a1
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21a1 mov eax,0x300000
            rax = (0x300000u64) as u32 as u64;
            // 21a6 and eax,DWORD PTR [rbp-0x24]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 21a9 mov DWORD PTR [rbp-0x24],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rax as u32 as u64)) as u32);
            // 21ac cmp eax,0x300000
            fl = sub::<32>((rax as u32 as u64), 0x300000u64).1;
            // 21b1 je 0x21ca
            if fl.e() { pc = 0x21ca; continue; }
            pc = 0x21b3;
        }
        0x21b3 => {
            // block 21b3
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21b3 cmp eax,0x200000
            fl = sub::<32>((rax as u32 as u64), 0x200000u64).1;
            // 21b8 jne 0x21d3
            if fl.ne() { pc = 0x21d3; continue; }
            pc = 0x21ba;
        }
        0x21ba => {
            // block 21ba
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21ba mov eax,0x800000
            rax = (0x800000u64) as u32 as u64;
            // 21bf mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 21c4 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21c6 => {
            // block 21c6
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21c6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 21c8 jmp 0x2244
            pc = 0x2244; continue;
        }
        0x21ca => {
            // block 21ca
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21ca mov eax,0x1000000
            rax = (0x1000000u64) as u32 as u64;
            // 21cf xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 21d1 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21d3 => {
            // block 21d3
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21d3 cmp eax,0x100000
            fl = sub::<32>((rax as u32 as u64), 0x100000u64).1;
            // 21d8 jne 0x21e6
            if fl.ne() { pc = 0x21e6; continue; }
            pc = 0x21da;
        }
        0x21da => {
            // block 21da
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21da mov eax,0x400000
            rax = (0x400000u64) as u32 as u64;
            // 21df mov ebx,0x2
            rbx = (0x2u64) as u32 as u64;
            // 21e4 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21e6 => {
            // block 21e6
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21e6 mov eax,0x200000
            rax = (0x200000u64) as u32 as u64;
            // 21eb mov ebx,0x4
            rbx = (0x4u64) as u32 as u64;
            pc = 0x21f0;
        }
        0x21f0 => {
            // block 21f0
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21f0 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 21f3 je 0x21f8
            if fl.e() { pc = 0x21f8; continue; }
            pc = 0x21f5;
        }
        0x21f5 => {
            // block 21f5
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21f5 mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            pc = 0x21f8;
        }
        0x21f8 => {
            // block 21f8
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21f8 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 21fb je 0x2215
            if fl.e() { pc = 0x2215; continue; }
            pc = 0x21fd;
        }
        0x21fd => {
            // block 21fd
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21fd mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 2202 call 0xd19e0
            { let r = { let t = m.env.code_va(0xd19e0); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2207 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 2209 je 0x2210
            if fl.e() { pc = 0x2210; continue; }
            pc = 0x220b;
        }
        0x220b => {
            // block 220b
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 220b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 220b imul eax,ebx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rbx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 220e jmp 0x2212
            pc = 0x2212; continue;
        }
        0x2210 => {
            // block 2210
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2210 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2210 shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0x2212;
        }
        0x2212 => {
            // block 2212
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2212 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2212 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            pc = 0x2215;
        }
        0x2215 => {
            // block 2215
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2215 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2215 mov DWORD PTR [rbp-0x28],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x12u64) as u32);
            // 221c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2220 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2224 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2226 mov rdi,r12
            rdi = r12;
            // 2229 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 222f mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 2232 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2235 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // 2237 cmp eax,0x10
            fl = sub::<32>((rax as u32 as u64), 0x10u64).1;
            // 223a jne 0x2244
            if fl.ne() { pc = 0x2244; continue; }
            pc = 0x223c;
        }
        0x223c => {
            // block 223c
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 223c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 223c shl DWORD PTR [r15],0x2
            { let (r, f) = shl::<32>(m.r32(r15) as u64, 0x2u64, fl); m.w32(r15, (r) as u32); fl = f; }
            // 2240 shr DWORD PTR [r14],0x2
            { let (r, f) = shr::<32>(m.r32(r14) as u64, 0x2u64, fl); m.w32(r14, (r) as u32); fl = f; }
            pc = 0x2244;
        }
        0x2244 => {
            // block 2244
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2244 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 2246 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 224a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 224b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 224d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 224f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2251 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2252 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4982 size=300
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004982<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4982;
    loop {
        match pc {
        0x4982 => {
            // block 4982
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4982 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4982 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4983 mov rbp,rsp
            rbp = rsp;
            // 4986 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4988 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 498a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 498c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 498e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 498f sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4993 mov r12,rcx
            r12 = rcx;
            // 4996 mov r15,rdx
            r15 = rdx;
            // 4999 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 499b mov r14,rdi
            r14 = rdi;
            // 499e mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 49a5 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 49ac mov DWORD PTR [rbp-0x34],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0x0u64) as u32);
            // 49b3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49b6 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49bc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 49be je 0x49c8
            if fl.e() { pc = 0x49c8; continue; }
            pc = 0x49c0;
        }
        0x49c0 => {
            // block 49c0
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 49c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49c0 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 49c3 jmp 0x4a9b
            pc = 0x4a9b; continue;
        }
        0x49c8 => {
            // block 49c8
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 49c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49c8 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 49cc mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49cf mov ebx,ebx
            rbx = ((rbx as u32 as u64)) as u32 as u64;
            // 49d1 lea rcx,[rip+0xfa798] # 0xff170
            rcx = m.env.data_va(80189);
            // 49d8 mov r13d,DWORD PTR [rcx+rbx*4]
            r13 = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 49dc lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 49e0 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 49e3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 49e8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 49eb mov rdi,r14
            rdi = r14;
            // 49ee mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 49f1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49f4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 49f6 je 0x4a9b
            if fl.e() { pc = 0x4a9b; continue; }
            pc = 0x49fc;
        }
        0x49fc => {
            // block 49fc
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 49fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49fc lea rax,[rip+0xfa79d] # 0xff1a0
            rax = m.env.data_va(80237);
            // 4a03 mov eax,DWORD PTR [rax+rbx*4]
            rax = (m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a06 test DWORD PTR [rbp-0x2c],eax
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & (rax as u32 as u64));
            // 4a09 je 0x4a31
            if fl.e() { pc = 0x4a31; continue; }
            pc = 0x4a0b;
        }
        0x4a0b => {
            // block 4a0b
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a0b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a0e lea rcx,[rip+0xfa7bb] # 0xff1d0
            rcx = m.env.data_va(80285);
            // 4a15 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a18 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 4a1c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4a1f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4a24 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4a27 mov rdi,r14
            rdi = r14;
            // 4a2a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a2d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4a2f je 0x4a9b
            if fl.e() { pc = 0x4a9b; continue; }
            pc = 0x4a31;
        }
        0x4a31 => {
            // block 4a31
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a31 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a34 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 4a38 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4a3b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4a40 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4a43 mov rdi,r14
            rdi = r14;
            // 4a46 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4a49 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a4c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4a4e je 0x4a9b
            if fl.e() { pc = 0x4a9b; continue; }
            pc = 0x4a50;
        }
        0x4a50 => {
            // block 4a50
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a50 lea rax,[rip+0xfa7a9] # 0xff200
            rax = m.env.data_va(80333);
            // 4a57 mov eax,DWORD PTR [rax+rbx*4]
            rax = (m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a5a test DWORD PTR [rbp-0x2c],eax
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & (rax as u32 as u64));
            // 4a5d je 0x4a85
            if fl.e() { pc = 0x4a85; continue; }
            pc = 0x4a5f;
        }
        0x4a5f => {
            // block 4a5f
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a5f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a62 lea rcx,[rip+0xfa7c7] # 0xff230
            rcx = m.env.data_va(80381);
            // 4a69 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a6c lea rdx,[rbp-0x34]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 4a70 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4a73 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4a78 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4a7b mov rdi,r14
            rdi = r14;
            // 4a7e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a81 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4a83 je 0x4a9b
            if fl.e() { pc = 0x4a9b; continue; }
            pc = 0x4a85;
        }
        0x4a85 => {
            // block 4a85
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a85 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 4a88 bswap eax
            rax = (rax as u32).swap_bytes() as u64;
            // 4a8a mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            // 4a8d mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // 4a90 bswap eax
            rax = (rax as u32).swap_bytes() as u64;
            // 4a92 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 4a96 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // 4a98 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x4a9b;
        }
        0x4a9b => {
            // block 4a9b
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a9b mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 4a9e add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4aa2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4aa3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4aa5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4aa7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4aa9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4aab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4aac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x9046 size=260
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00009046<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x9046;
    loop {
        match pc {
        0x9046 => {
            // block 9046
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 9046 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9046 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 9047 mov rbp,rsp
            rbp = rsp;
            // 904a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 904c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 904e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 9050 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 9052 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9053 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 9057 mov r14,rdi
            r14 = rdi;
            // 905a lea rsi,[rip+0x120f42] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 9061 lea rbx,[rbp-0x58]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9065 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 906a mov rdi,rbx
            rdi = rbx;
            // 906d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9072 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9076 mov rsi,rbx
            rsi = rbx;
            // 9079 call 0x68488
            { let r = { let t = m.env.code_va(0x68488); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 907e lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9082 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9087 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // 908b call 0xceac8
            { let r = { let t = m.env.code_va(0xceac8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9090 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 9093 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 9097 lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 909b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 909e cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 90a1 je 0x90f7
            if fl.e() { pc = 0x90f7; continue; }
            pc = 0x90a3;
        }
        0x90a3 => {
            // block 90a3
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90a3 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x90a6;
        }
        0x90a6 => {
            // block 90a6
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90a6 mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 90a9 cmp esi,r12d
            fl = sub::<32>((rsi as u32 as u64), (r12 as u32 as u64)).1;
            // 90ac ja 0x90cd
            if fl.a() { pc = 0x90cd; continue; }
            pc = 0x90ae;
        }
        0x90ae => {
            // block 90ae
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90ae mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 90b1 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 90b5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 90b7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 90bc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 90bf mov rdi,r14
            rdi = r14;
            // 90c2 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 90c4 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 90c7 xor eax,0x1
            { let r = (rax as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 90ca add r13d,eax
            { let (r, f) = add::<32>((r13 as u32 as u64), (rax as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x90cd;
        }
        0x90cd => {
            // block 90cd
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90cd mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 90d1 test rax,rax
            fl = logic::<64>(rax & rax);
            // 90d4 je 0x90e3
            if fl.e() { pc = 0x90e3; continue; }
            pc = 0x90d6;
        }
        0x90d6 => {
            // block 90d6
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90d6 mov rbx,rax
            rbx = rax;
            // 90d9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 90dc test rax,rax
            fl = logic::<64>(rax & rax);
            // 90df jne 0x90d6
            if fl.ne() { pc = 0x90d6; continue; }
            pc = 0x90e1;
        }
        0x90e1 => {
            // block 90e1
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90e1 jmp 0x90f2
            pc = 0x90f2; continue;
        }
        0x90e3 => {
            // block 90e3
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90e3 mov rax,rbx
            rax = rbx;
            pc = 0x90e6;
        }
        0x90e6 => {
            // block 90e6
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90e6 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 90ea cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 90ed mov rax,rbx
            rax = rbx;
            // 90f0 jne 0x90e6
            if fl.ne() { pc = 0x90e6; continue; }
            pc = 0x90f2;
        }
        0x90f2 => {
            // block 90f2
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90f2 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 90f5 jne 0x90a6
            if fl.ne() { pc = 0x90a6; continue; }
            pc = 0x90f7;
        }
        0x90f7 => {
            // block 90f7
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90f7 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 90fa sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 90fd mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 9101 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9105 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 910a mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 910c add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 9110 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9111 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 9113 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 9115 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9117 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 9119 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 911a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x911b => {
            // block 911b
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 911b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 911b mov rbx,rax
            rbx = rax;
            pc = 0x911e;
        }
        0x911e => {
            // block 911e
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 911e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 911e mov rsi,QWORD PTR [r15]
            rsi = m.r64(r15) as u64;
            // 9121 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9125 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x912a;
        }
        0x912a => {
            // block 912a
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 912a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 912a mov rdi,rbx
            rdi = rbx;
            // 912d call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9132 mov rbx,rax
            rbx = rax;
            // 9135 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9139 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 913e jmp 0x912a
            pc = 0x912a; continue;
        }
        0x9140 => {
            // block 9140
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 9140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9140 mov rbx,rax
            rbx = rax;
            // 9143 lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 9147 jmp 0x911e
            pc = 0x911e; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x117fa size=280
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000117fa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x117fa;
    loop {
        match pc {
        0x117fa => {
            // block 117fa
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 117fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 117fa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 117fb mov rbp,rsp
            rbp = rsp;
            // 117fe push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11800 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11802 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 11804 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 11806 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11807 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 11808 mov r13,rdi
            r13 = rdi;
            // 1180b mov rax,QWORD PTR [rip+0x12ea06] # 0x140218
            rax = m.r64(m.env.data_va(346597)) as u64;
            // 11812 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 11816 mov QWORD PTR [r13+0x0],rax
            m.w64(r13, (rax) as u64);
            // 1181a mov QWORD PTR [r13+0x8],rsi
            m.w64(r13.wrapping_add(0x8), (rsi) as u64);
            // 1181e mov DWORD PTR [r13+0x10],0x1
            m.w32(r13.wrapping_add(0x10), (0x1u64) as u32);
            // 11826 mov QWORD PTR [r13+0x20],0x0
            m.w64(r13.wrapping_add(0x20), (0x0u64) as u64);
            // 1182e mov QWORD PTR [r13+0x18],0x0
            m.w64(r13.wrapping_add(0x18), (0x0u64) as u64);
            // 11836 mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 11839 mov rdi,rsi
            rdi = rsi;
            // 1183c call QWORD PTR [rax+0x50]
            { let r = { let t = m.r64(rax.wrapping_add(0x50)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1183f cmp eax,0xa
            fl = sub::<32>((rax as u32 as u64), 0xau64).1;
            // 11842 jne 0x11902
            if fl.ne() { pc = 0x11902; continue; }
            pc = 0x11848;
        }
        0x11848 => {
            // block 11848
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 11848 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11848 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1184c mov rdi,r13
            rdi = r13;
            // 1184f call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11852 mov rcx,rax
            rcx = rax;
            // 11855 neg rcx
            { let (r, f) = neg::<64>(rcx); rcx = r; fl = f; }
            // 11858 cmovl rcx,rax
            if fl.l() { rcx = rax; }
            // 1185c mov QWORD PTR [r13+0x20],rcx
            m.w64(r13.wrapping_add(0x20), (rcx) as u64);
            // 11860 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 11864 mov rdi,r13
            rdi = r13;
            // 11867 call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1186a imul rax,QWORD PTR [r13+0x20]
            { let (r, f) = imul::<64>(rax, m.r64(r13.wrapping_add(0x20)) as u64); rax = r; fl = f; }
            // 1186f mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // 11874 mul rcx
            { let (lo, hi, f) = mul1::<64>(rax, rcx); rax = lo; rdx = hi; fl = f; }
            // 11877 mov rdi,0xffffffffffffffff
            rdi = 0xffff_ffff_ffff_ffffu64;
            // 1187e cmovno rdi,rax
            if fl.no() { rdi = rax; }
            // 11882 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11887 mov QWORD PTR [r13+0x18],rax
            m.w64(r13.wrapping_add(0x18), (rax) as u64);
            // 1188b mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1188f mov rdi,r13
            rdi = r13;
            // 11892 call QWORD PTR [rax+0x68]
            { let r = { let t = m.r64(rax.wrapping_add(0x68)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11895 mov r14,rax
            r14 = rax;
            // 11898 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1189c mov r15,QWORD PTR [r13+0x18]
            r15 = m.r64(r13.wrapping_add(0x18)) as u64;
            // 118a0 mov rdi,r13
            rdi = r13;
            // 118a3 call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 118a6 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 118a9 test rax,rax
            fl = logic::<64>(rax & rax);
            // 118ac jle 0x11902
            if fl.le() { pc = 0x11902; continue; }
            pc = 0x118ae;
        }
        0x118ae => {
            // block 118ae
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 118ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 118ae mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 118b2 mov rdi,r13
            rdi = r13;
            // 118b5 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 118b8 test rax,rax
            fl = logic::<64>(rax & rax);
            // 118bb mov ebx,0x0
            rbx = (0x0u64) as u32 as u64;
            // 118c0 jle 0x118df
            if fl.le() { pc = 0x118df; continue; }
            pc = 0x118c2;
        }
        0x118c2 => {
            // block 118c2
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 118c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 118c2 mov eax,DWORD PTR [r14+rbx*4]
            rax = (m.r32(r14.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 118c6 rol eax,0x8
            { let (r, f) = rol::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 118c9 mov DWORD PTR [r15+rbx*4],eax
            m.w32(r15.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 118cd inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 118d0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 118d4 mov rdi,r13
            rdi = r13;
            // 118d7 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 118da cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 118dd jl 0x118c2
            if fl.l() { pc = 0x118c2; continue; }
            pc = 0x118df;
        }
        0x118df => {
            // block 118df
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 118df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 118df mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 118e3 mov rdi,r13
            rdi = r13;
            // 118e6 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 118e9 add r14,rax
            { let (r, f) = add::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // 118ec add r15,QWORD PTR [r13+0x20]
            { let (r, f) = add::<64>(r15, m.r64(r13.wrapping_add(0x20)) as u64);
              r15 = r;
              fl = f; }
            // 118f0 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 118f3 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 118f7 mov rdi,r13
            rdi = r13;
            // 118fa call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 118fd cmp r12,rax
            fl = sub::<64>(r12, rax).1;
            // 11900 jl 0x118ae
            if fl.l() { pc = 0x118ae; continue; }
            pc = 0x11902;
        }
        0x11902 => {
            // block 11902
            #[cfg(feature = "trace")] eprintln!("f_000117fa pc 11902 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11902 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 11906 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11907 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11909 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1190b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1190d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1190f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11910 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x123fc size=273
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000123fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x123fc;
    loop {
        match pc {
        0x123fc => {
            // block 123fc
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 123fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 123fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 123fd mov rbp,rsp
            rbp = rsp;
            // 12400 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12402 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12404 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12406 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12408 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12409 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 1240d mov rbx,rdx
            rbx = rdx;
            // 12410 mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // 12414 mov r15,rdi
            r15 = rdi;
            // 12417 cmp r15,rsi
            fl = sub::<64>(r15, rsi).1;
            // 1241a je 0x124d1
            if fl.e() { pc = 0x124d1; continue; }
            pc = 0x12420;
        }
        0x12420 => {
            // block 12420
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12420 mov rdi,rbx
            rdi = rbx;
            // 12423 mov rsi,r15
            rsi = r15;
            // 12426 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1242b lea rcx,[r15+0x38]
            rcx = r15.wrapping_add(0x38);
            // 1242f mov r13d,0x1
            r13 = (0x1u64) as u32 as u64;
            // 12435 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 12438 cmp rcx,QWORD PTR [rbp-0x40]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64).1;
            // 1243c je 0x124d1
            if fl.e() { pc = 0x124d1; continue; }
            pc = 0x12442;
        }
        0x12442 => {
            // block 12442
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12442 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12442 mov r14,rbx
            r14 = rbx;
            pc = 0x12445;
        }
        0x12445 => {
            // block 12445
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12445 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12445 lea rdi,[r14+0x38]
            rdi = r14.wrapping_add(0x38);
            // 12449 mov eax,DWORD PTR [r15+0x50]
            rax = (m.r32(r15.wrapping_add(0x50)) as u64) as u32 as u64;
            // 1244d cmp eax,DWORD PTR [r14+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x18)) as u64).1;
            // 12451 jge 0x1249e
            if fl.ge() { pc = 0x1249e; continue; }
            pc = 0x12453;
        }
        0x12453 => {
            // block 12453
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12453 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12453 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // 12457 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 1245b mov rsi,r14
            rsi = r14;
            // 1245e call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12463 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // 12466 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 12469 mov r14,r12
            r14 = r12;
            // 1246c mov rdi,rbx
            rdi = rbx;
            // 1246f je 0x12493
            if fl.e() { pc = 0x12493; continue; }
            pc = 0x12471;
        }
        0x12471 => {
            // block 12471
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12471 lea rdi,[rbx+r14*1]
            rdi = rbx.wrapping_add(r14);
            // 12475 mov eax,DWORD PTR [r15+0x50]
            rax = (m.r32(r15.wrapping_add(0x50)) as u64) as u32 as u64;
            // 12479 cmp eax,DWORD PTR [rbx+r14*1-0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbx.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // 1247e jge 0x12493
            if fl.ge() { pc = 0x12493; continue; }
            pc = 0x12480;
        }
        0x12480 => {
            // block 12480
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12480 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12480 lea rsi,[rbx+r14*1-0x38]
            rsi = rbx.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_ffc8);
            // 12485 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1248a add r14,0xffffffffffffffc8
            { let (r, f) = add::<64>(r14, 0xffff_ffff_ffff_ffc8u64);
              r14 = r;
              fl = f; }
            // 1248e jne 0x12471
            if fl.ne() { pc = 0x12471; continue; }
            pc = 0x12490;
        }
        0x12490 => {
            // block 12490
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12490 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12490 mov rdi,rbx
            rdi = rbx;
            pc = 0x12493;
        }
        0x12493 => {
            // block 12493
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12493 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12493 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 12497 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1249c jmp 0x124b1
            pc = 0x124b1; continue;
        }
        0x1249e => {
            // block 1249e
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 1249e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1249e mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 124a2 mov rsi,rcx
            rsi = rcx;
            // 124a5 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // 124a9 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 124ae inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            pc = 0x124b1;
        }
        0x124b1 => {
            // block 124b1
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124b1 mov rax,r15
            rax = r15;
            // 124b4 add rax,0x70
            { let (r, f) = add::<64>(rax, 0x70u64);
              rax = r;
              fl = f; }
            // 124b8 add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 124bc cmp rax,QWORD PTR [rbp-0x40]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64).1;
            // 124c0 mov r15,QWORD PTR [rbp-0x30]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 124c4 mov rcx,rax
            rcx = rax;
            // 124c7 mov r14,QWORD PTR [rbp-0x38]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 124cb jne 0x12445
            if fl.ne() { pc = 0x12445; continue; }
            pc = 0x124d1;
        }
        0x124d1 => {
            // block 124d1
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124d1 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 124d5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 124d6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 124d8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 124da pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 124dc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 124de pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 124df ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x124e0 => {
            // block 124e0
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124e0 jmp 0x124e2
            pc = 0x124e2; continue;
        }
        0x124e2 => {
            // block 124e2
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124e2 mov r14,rax
            r14 = rax;
            // 124e5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 124e8 je 0x12505
            if fl.e() { pc = 0x12505; continue; }
            pc = 0x124ea;
        }
        0x124ea => {
            // block 124ea
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124ea test r13,r13
            fl = logic::<64>(r13 & r13);
            // 124ed je 0x12505
            if fl.e() { pc = 0x12505; continue; }
            pc = 0x124ef;
        }
        0x124ef => {
            // block 124ef
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 124ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 124ef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 124f2 mov rdi,rbx
            rdi = rbx;
            // 124f5 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 124f7 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 124fb dec r13
            { let (r, f) = dec::<64>(r13, fl); r13 = r; fl = f; }
            // 124fe jne 0x124ef
            if fl.ne() { pc = 0x124ef; continue; }
            pc = 0x12500;
        }
        0x12500 => {
            // block 12500
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12500 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12500 jmp 0x12505
            pc = 0x12505; continue;
        }
        0x12502 => {
            // block 12502
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12502 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12502 mov r14,rax
            r14 = rax;
            pc = 0x12505;
        }
        0x12505 => {
            // block 12505
            #[cfg(feature = "trace")] eprintln!("f_000123fc pc 12505 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12505 mov rdi,r14
            rdi = r14;
            // 12508 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1250d size=271
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001250d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1250d;
    loop {
        match pc {
        0x1250d => {
            // block 1250d
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1250d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1250d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1250e mov rbp,rsp
            rbp = rsp;
            // 12511 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12513 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12515 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12517 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12519 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1251a sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 1251e mov rbx,r8
            rbx = r8;
            // 12521 mov r13,rcx
            r13 = rcx;
            // 12524 mov QWORD PTR [rbp-0x38],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r13) as u64);
            // 12528 mov r12,rdx
            r12 = rdx;
            // 1252b mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // 1252f mov rcx,rdi
            rcx = rdi;
            // 12532 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 12535 mov r14,rbx
            r14 = rbx;
            // 12538 cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // 1253b je 0x12589
            if fl.e() { pc = 0x12589; continue; }
            pc = 0x1253d;
        }
        0x1253d => {
            // block 1253d
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1253d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1253d cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 12540 je 0x125ab
            if fl.e() { pc = 0x125ab; continue; }
            pc = 0x12542;
        }
        0x12542 => {
            // block 12542
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12542 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12542 mov eax,DWORD PTR [r12+0x18]
            rax = (m.r32(r12.wrapping_add(0x18)) as u64) as u32 as u64;
            // 12547 cmp eax,DWORD PTR [rcx+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx.wrapping_add(0x18)) as u64).1;
            // 1254a jge 0x12563
            if fl.ge() { pc = 0x12563; continue; }
            pc = 0x1254c;
        }
        0x1254c => {
            // block 1254c
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1254c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1254c mov r13,rcx
            r13 = rcx;
            // 1254f mov rdi,r14
            rdi = r14;
            // 12552 mov rsi,r12
            rsi = r12;
            // 12555 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1255a add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 1255e mov rcx,r13
            rcx = r13;
            // 12561 jmp 0x12578
            pc = 0x12578; continue;
        }
        0x12563 => {
            // block 12563
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12563 mov rdi,r14
            rdi = r14;
            // 12566 mov rsi,rcx
            rsi = rcx;
            // 12569 mov r13,rcx
            r13 = rcx;
            // 1256c call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12571 mov rcx,r13
            rcx = r13;
            // 12574 add rcx,0x38
            { let (r, f) = add::<64>(rcx, 0x38u64);
              rcx = r;
              fl = f; }
            pc = 0x12578;
        }
        0x12578 => {
            // block 12578
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12578 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12578 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 1257b add r14,0x38
            { let (r, f) = add::<64>(r14, 0x38u64);
              r14 = r;
              fl = f; }
            // 1257f cmp rcx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 12583 mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 12587 jne 0x1253d
            if fl.ne() { pc = 0x1253d; continue; }
            pc = 0x12589;
        }
        0x12589 => {
            // block 12589
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12589 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12589 cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 1258c je 0x125dd
            if fl.e() { pc = 0x125dd; continue; }
            pc = 0x1258e;
        }
        0x1258e => {
            // block 1258e
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1258e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1258e mov rdi,r14
            rdi = r14;
            // 12591 mov rsi,r12
            rsi = r12;
            // 12594 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12599 add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 1259d add r14,0x38
            { let (r, f) = add::<64>(r14, 0x38u64);
              r14 = r;
              fl = f; }
            // 125a1 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 125a4 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 125a7 jne 0x1258e
            if fl.ne() { pc = 0x1258e; continue; }
            pc = 0x125a9;
        }
        0x125a9 => {
            // block 125a9
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125a9 jmp 0x125dd
            pc = 0x125dd; continue;
        }
        0x125ab => {
            // block 125ab
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125ab cmp rcx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 125af je 0x125dd
            if fl.e() { pc = 0x125dd; continue; }
            pc = 0x125b1;
        }
        0x125b1 => {
            // block 125b1
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125b1 sub QWORD PTR [rbp-0x30],rcx
            { let (r, f) = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, rcx);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r) as u64);
              fl = f; }
            // 125b5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 125b8 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x125bb;
        }
        0x125bb => {
            // block 125bb
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125bb mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 125bf lea rdi,[r14+r13*1]
            rdi = r14.wrapping_add(r13);
            // 125c3 lea rsi,[rcx+r13*1]
            rsi = rcx.wrapping_add(r13);
            // 125c7 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 125cc add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 125d0 dec r12
            { let (r, f) = dec::<64>(r12, fl); r12 = r; fl = f; }
            // 125d3 cmp QWORD PTR [rbp-0x30],r13
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, r13).1;
            // 125d7 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 125db jne 0x125bb
            if fl.ne() { pc = 0x125bb; continue; }
            pc = 0x125dd;
        }
        0x125dd => {
            // block 125dd
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125dd add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 125e1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 125e2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 125e4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 125e6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 125e8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 125ea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 125eb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x125ec => {
            // block 125ec
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125ec jmp 0x125ee
            pc = 0x125ee; continue;
        }
        0x125ee => {
            // block 125ee
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125ee mov r14,rax
            r14 = rax;
            pc = 0x125f1;
        }
        0x125f1 => {
            // block 125f1
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125f1 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 125f4 je 0x1260c
            if fl.e() { pc = 0x1260c; continue; }
            pc = 0x125f6;
        }
        0x125f6 => {
            // block 125f6
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125f6 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 125f9 je 0x1260c
            if fl.e() { pc = 0x1260c; continue; }
            pc = 0x125fb;
        }
        0x125fb => {
            // block 125fb
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125fb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 125fe mov rdi,rbx
            rdi = rbx;
            // 12601 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12603 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12607 dec r15
            { let (r, f) = dec::<64>(r15, fl); r15 = r; fl = f; }
            // 1260a jne 0x125fb
            if fl.ne() { pc = 0x125fb; continue; }
            pc = 0x1260c;
        }
        0x1260c => {
            // block 1260c
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1260c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1260c mov rdi,r14
            rdi = r14;
            // 1260f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12614 mov r14,rax
            r14 = rax;
            // 12617 sub r15,r12
            { let (r, f) = sub::<64>(r15, r12);
              r15 = r;
              fl = f; }
            // 1261a jmp 0x125f1
            pc = 0x125f1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x12824 size=272
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012824<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x12824;
    loop {
        match pc {
        0x12824 => {
            // block 12824
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12824 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12824 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12825 mov rbp,rsp
            rbp = rsp;
            // 12828 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1282a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1282c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1282e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12830 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12831 sub rsp,0x88
            { let (r, f) = sub::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // 12838 mov r14,rdx
            r14 = rdx;
            // 1283b mov r15,rsi
            r15 = rsi;
            // 1283e mov r13,rdi
            r13 = rdi;
            // 12841 lea r12,[rbp-0x60]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 12845 mov rbx,r15
            rbx = r15;
            // 12848 jmp 0x12851
            pc = 0x12851; continue;
        }
        0x1284a => {
            // block 1284a
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 1284a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1284a cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 1284d cmove r15,rbx
            if fl.e() { r15 = rbx; }
            pc = 0x12851;
        }
        0x12851 => {
            // block 12851
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12851 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12851 mov rdi,r12
            rdi = r12;
            // 12854 mov rsi,r13
            rsi = r13;
            // 12857 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1285c mov rdi,r13
            rdi = r13;
            // 1285f mov rsi,rbx
            rsi = rbx;
            // 12862 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12867 mov rdi,rbx
            rdi = rbx;
            // 1286a mov rsi,r12
            rsi = r12;
            // 1286d call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12872 mov rdi,r12
            rdi = r12;
            // 12875 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1287a add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 1287e add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12882 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 12885 jne 0x1284a
            if fl.ne() { pc = 0x1284a; continue; }
            pc = 0x12887;
        }
        0x12887 => {
            // block 12887
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12887 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12887 mov QWORD PTR [rbp-0xa0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (r14) as u64);
            // 1288e cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 12891 je 0x128fe
            if fl.e() { pc = 0x128fe; continue; }
            pc = 0x12893;
        }
        0x12893 => {
            // block 12893
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12893 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12893 lea r14,[rbp-0x98]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 1289a mov rbx,r13
            rbx = r13;
            // 1289d mov r12,r15
            r12 = r15;
            // 128a0 jmp 0x128b1
            pc = 0x128b1; continue;
        }
        0x128a2 => {
            // block 128a2
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128a2 cmp QWORD PTR [rbp-0xa8],rbx
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, rbx).1;
            // 128a9 cmove r15,r12
            if fl.e() { r15 = r12; }
            // 128ad add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            pc = 0x128b1;
        }
        0x128b1 => {
            // block 128b1
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128b1 lea rax,[r15-0x38]
            rax = r15.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 128b5 mov QWORD PTR [rbp-0xa8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rax) as u64);
            pc = 0x128bc;
        }
        0x128bc => {
            // block 128bc
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128bc mov rdi,r14
            rdi = r14;
            // 128bf mov rsi,rbx
            rsi = rbx;
            // 128c2 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 128c7 mov rdi,rbx
            rdi = rbx;
            // 128ca mov rsi,r12
            rsi = r12;
            // 128cd call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 128d2 mov rdi,r12
            rdi = r12;
            // 128d5 mov rsi,r14
            rsi = r14;
            // 128d8 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 128dd mov rdi,r14
            rdi = r14;
            // 128e0 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 128e5 add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 128e9 cmp r12,QWORD PTR [rbp-0xa0]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64).1;
            // 128f0 jne 0x128a2
            if fl.ne() { pc = 0x128a2; continue; }
            pc = 0x128f2;
        }
        0x128f2 => {
            // block 128f2
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128f2 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 128f6 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 128f9 mov r12,r15
            r12 = r15;
            // 128fc jne 0x128bc
            if fl.ne() { pc = 0x128bc; continue; }
            pc = 0x128fe;
        }
        0x128fe => {
            // block 128fe
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128fe mov rax,r13
            rax = r13;
            // 12901 add rsp,0x88
            { let (r, f) = add::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // 12908 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 12909 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1290b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1290d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1290f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12911 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12912 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x12913 => {
            // block 12913
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12913 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12913 mov rbx,rax
            rbx = rax;
            // 12916 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1291a jmp 0x12926
            pc = 0x12926; continue;
        }
        0x1291c => {
            // block 1291c
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 1291c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1291c mov rbx,rax
            rbx = rax;
            // 1291f lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            pc = 0x12926;
        }
        0x12926 => {
            // block 12926
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12926 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12926 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1292b mov rdi,rbx
            rdi = rbx;
            // 1292e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14f08 size=288
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014f08<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x14f08;
    loop {
        match pc {
        0x14f08 => {
            // block 14f08
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14f08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14f08 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14f09 mov rbp,rsp
            rbp = rsp;
            // 14f0c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 14f0e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14f0f mov r14,rsi
            r14 = rsi;
            // 14f12 mov rbx,rdi
            rbx = rdi;
            // 14f15 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 14f18 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14f1b mov rsi,QWORD PTR [rip+0x12b276] # 0x140198
            rsi = m.r64(m.env.data_va(346469)) as u64;
            // 14f22 mov rdi,r14
            rdi = r14;
            // 14f25 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14f2a mov QWORD PTR [rbx+0x80],rax
            m.w64(rbx.wrapping_add(0x80), (rax) as u64);
            // 14f31 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 14f34 movzx r14d,BYTE PTR [rbx+0x192]
            r14 = (m.r8(rbx.wrapping_add(0x192)) as u64) as u32 as u64;
            // 14f3c mov rdi,rax
            rdi = rax;
            // 14f3f call QWORD PTR [rcx+0x38]
            { let r = { let t = m.r64(rcx.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14f42 mov BYTE PTR [rbx+0x192],al
            m.w8(rbx.wrapping_add(0x192), ((rax as u8 as u64)) as u8);
            // 14f48 movzx ecx,al
            rcx = ((rax as u8 as u64)) as u32 as u64;
            // 14f4b cmp r14d,ecx
            fl = sub::<32>((r14 as u32 as u64), (rcx as u32 as u64)).1;
            // 14f4e je 0x15023
            if fl.e() { pc = 0x15023; continue; }
            pc = 0x14f54;
        }
        0x14f54 => {
            // block 14f54
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14f54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14f54 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 14f5c mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 14f64 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 14f6c mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 14f74 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 14f7c mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 14f84 mov cl,BYTE PTR [rbx+0x190]
            rcx = (rcx & !0xff) | ((m.r8(rbx.wrapping_add(0x190)) as u64) as u8 as u64);
            // 14f8a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 14f8c je 0x14fd5
            if fl.e() { pc = 0x14fd5; continue; }
            pc = 0x14f8e;
        }
        0x14f8e => {
            // block 14f8e
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14f8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14f8e test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 14f90 je 0x14fa0
            if fl.e() { pc = 0x14fa0; continue; }
            pc = 0x14f92;
        }
        0x14f92 => {
            // block 14f92
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14f92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14f92 mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 14f96 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 14f99 je 0x14fa0
            if fl.e() { pc = 0x14fa0; continue; }
            pc = 0x14f9b;
        }
        0x14f9b => {
            // block 14f9b
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14f9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14f9b call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x14fa0;
        }
        0x14fa0 => {
            // block 14fa0
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14fa0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14fa0 mov al,BYTE PTR [rbx+0x191]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x191)) as u64) as u8 as u64);
            // 14fa6 mov BYTE PTR [rbx+0x190],al
            m.w8(rbx.wrapping_add(0x190), ((rax as u8 as u64)) as u8);
            // 14fac mov rax,QWORD PTR [rbx+0x70]
            rax = m.r64(rbx.wrapping_add(0x70)) as u64;
            // 14fb0 mov QWORD PTR [rbx+0x60],rax
            m.w64(rbx.wrapping_add(0x60), (rax) as u64);
            // 14fb4 mov rax,QWORD PTR [rbx+0x68]
            rax = m.r64(rbx.wrapping_add(0x68)) as u64;
            // 14fb8 mov QWORD PTR [rbx+0x40],rax
            m.w64(rbx.wrapping_add(0x40), (rax) as u64);
            // 14fbc mov BYTE PTR [rbx+0x191],0x0
            m.w8(rbx.wrapping_add(0x191), (0x0u64) as u8);
            // 14fc3 mov QWORD PTR [rbx+0x70],0x0
            m.w64(rbx.wrapping_add(0x70), (0x0u64) as u64);
            // 14fcb mov QWORD PTR [rbx+0x68],0x0
            m.w64(rbx.wrapping_add(0x68), (0x0u64) as u64);
            // 14fd3 jmp 0x15023
            pc = 0x15023; continue;
        }
        0x14fd5 => {
            // block 14fd5
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14fd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14fd5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 14fd7 jne 0x1500b
            if fl.ne() { pc = 0x1500b; continue; }
            pc = 0x14fd9;
        }
        0x14fd9 => {
            // block 14fd9
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14fd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14fd9 mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 14fdd lea rcx,[rbx+0x58]
            rcx = rbx.wrapping_add(0x58);
            // 14fe1 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 14fe4 je 0x1500b
            if fl.e() { pc = 0x1500b; continue; }
            pc = 0x14fe6;
        }
        0x14fe6 => {
            // block 14fe6
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 14fe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14fe6 mov rdi,QWORD PTR [rbx+0x60]
            rdi = m.r64(rbx.wrapping_add(0x60)) as u64;
            // 14fea mov QWORD PTR [rbx+0x70],rdi
            m.w64(rbx.wrapping_add(0x70), (rdi) as u64);
            // 14fee mov QWORD PTR [rbx+0x68],rax
            m.w64(rbx.wrapping_add(0x68), (rax) as u64);
            // 14ff2 mov BYTE PTR [rbx+0x191],0x0
            m.w8(rbx.wrapping_add(0x191), (0x0u64) as u8);
            // 14ff9 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14ffe mov QWORD PTR [rbx+0x40],rax
            m.w64(rbx.wrapping_add(0x40), (rax) as u64);
            // 15002 mov BYTE PTR [rbx+0x190],0x1
            m.w8(rbx.wrapping_add(0x190), (0x1u64) as u8);
            // 15009 jmp 0x15023
            pc = 0x15023; continue;
        }
        0x1500b => {
            // block 1500b
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 1500b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1500b mov rdi,QWORD PTR [rbx+0x60]
            rdi = m.r64(rbx.wrapping_add(0x60)) as u64;
            // 1500f mov QWORD PTR [rbx+0x70],rdi
            m.w64(rbx.wrapping_add(0x70), (rdi) as u64);
            // 15013 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15018 mov QWORD PTR [rbx+0x68],rax
            m.w64(rbx.wrapping_add(0x68), (rax) as u64);
            // 1501c mov BYTE PTR [rbx+0x191],0x1
            m.w8(rbx.wrapping_add(0x191), (0x1u64) as u8);
            pc = 0x15023;
        }
        0x15023 => {
            // block 15023
            #[cfg(feature = "trace")] eprintln!("f_00014f08 pc 15023 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15023 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15024 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15026 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15027 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15e10 size=268
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00015e10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15e10;
    loop {
        match pc {
        0x15e10 => {
            // block 15e10
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15e11 mov rbp,rsp
            rbp = rsp;
            // 15e14 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15e16 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15e18 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15e1a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15e1c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15e1d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 15e21 mov r14,rsi
            r14 = rsi;
            // 15e24 mov r12,rdi
            r12 = rdi;
            // 15e27 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 15e2b mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 15e30 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 15e33 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 15e36 jns 0x15e44
            if fl.ns() { pc = 0x15e44; continue; }
            pc = 0x15e38;
        }
        0x15e38 => {
            // block 15e38
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e38 mov rdi,r12
            rdi = r12;
            // 15e3b call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x15e44 => {
            // block 15e44
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e44 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 15e49 sub r15,rcx
            { let (r, f) = sub::<64>(r15, rcx);
              r15 = r;
              fl = f; }
            // 15e4c movabs rax,0x3fffffffffffffff
            rax = 0x3fff_ffff_ffff_ffffu64;
            // 15e56 cmp r15,rax
            fl = sub::<64>(r15, rax).1;
            // 15e59 jae 0x15e87
            if fl.ae() { pc = 0x15e87; continue; }
            pc = 0x15e5b;
        }
        0x15e5b => {
            // block 15e5b
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e5b mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 15e5f add r15,r15
            { let (r, f) = add::<64>(r15, r15);
              r15 = r;
              fl = f; }
            // 15e62 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 15e65 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 15e69 mov r13,QWORD PTR [r12+0x8]
            r13 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 15e6e mov rbx,r13
            rbx = r13;
            // 15e71 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 15e74 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 15e78 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 15e7b test r15,r15
            fl = logic::<64>(r15 & r15);
            // 15e7e mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 15e83 jne 0x15ea4
            if fl.ne() { pc = 0x15ea4; continue; }
            pc = 0x15e85;
        }
        0x15e85 => {
            // block 15e85
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e85 jmp 0x15eaf
            pc = 0x15eaf; continue;
        }
        0x15e87 => {
            // block 15e87
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e87 mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 15e8b mov r13,QWORD PTR [r12+0x8]
            r13 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 15e90 mov rbx,r13
            rbx = r13;
            // 15e93 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 15e96 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 15e9a movabs r15,0x7fffffffffffffff
            r15 = 0x7fff_ffff_ffff_ffffu64;
            pc = 0x15ea4;
        }
        0x15ea4 => {
            // block 15ea4
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15ea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ea4 mov rdi,r15
            rdi = r15;
            // 15ea7 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15eac mov r14,r15
            r14 = r15;
            pc = 0x15eaf;
        }
        0x15eaf => {
            // block 15eaf
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15eaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15eaf add r14,rax
            { let (r, f) = add::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // 15eb2 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 15eb6 mov cl,BYTE PTR [rcx]
            rcx = (rcx & !0xff) | ((m.r8(rcx) as u64) as u8 as u64);
            // 15eb8 mov BYTE PTR [rax+rbx*1],cl
            m.w8(rax.wrapping_add(rbx), ((rcx as u8 as u64)) as u8);
            // 15ebb lea rcx,[rax+rbx*1+0x1]
            rcx = rax.wrapping_add(rbx).wrapping_add(0x1);
            // 15ec0 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // 15ec4 mov r15,QWORD PTR [rbp-0x38]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 15ec8 sub r13,r15
            { let (r, f) = sub::<64>(r13, r15);
              r13 = r;
              fl = f; }
            // 15ecb sub rbx,r13
            { let (r, f) = sub::<64>(rbx, r13);
              rbx = r;
              fl = f; }
            // 15ece add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 15ed1 mov rdi,rbx
            rdi = rbx;
            // 15ed4 mov rsi,r15
            rsi = r15;
            // 15ed7 mov rdx,r13
            rdx = r13;
            // 15eda call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15edf mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 15ee3 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 15ee7 mov QWORD PTR [r12+0x8],rax
            m.w64(r12.wrapping_add(0x8), (rax) as u64);
            // 15eec mov QWORD PTR [r12+0x10],r14
            m.w64(r12.wrapping_add(0x10), (r14) as u64);
            // 15ef1 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 15ef4 je 0x15f0c
            if fl.e() { pc = 0x15f0c; continue; }
            pc = 0x15ef6;
        }
        0x15ef6 => {
            // block 15ef6
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15ef6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ef6 mov rdi,r15
            rdi = r15;
            // 15ef9 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 15efd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15efe pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15f00 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15f02 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15f04 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15f06 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15f07 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x15f0c => {
            // block 15f0c
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15f0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f0c add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 15f10 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15f11 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15f13 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15f15 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15f17 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15f19 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15f1a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18b50 size=253
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018b50<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x18b50;
    loop {
        match pc {
        0x18b50 => {
            // block 18b50
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b50 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18b51 mov rbp,rsp
            rbp = rsp;
            // 18b54 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18b56 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18b58 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 18b5a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 18b5c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18b5d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 18b5e mov rbx,rsi
            rbx = rsi;
            // 18b61 mov r14,rdi
            r14 = rdi;
            // 18b64 mov r13,QWORD PTR [rbx]
            r13 = m.r64(rbx) as u64;
            // 18b67 cmp r13,QWORD PTR [rbx+0x8]
            fl = sub::<64>(r13, m.r64(rbx.wrapping_add(0x8)) as u64).1;
            // 18b6b je 0x18c3b
            if fl.e() { pc = 0x18c3b; continue; }
            pc = 0x18b71;
        }
        0x18b71 => {
            // block 18b71
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b71 lea r12,[rip+0x111741] # 0x12a2b9
            r12 = m.env.data_va(256646);
            // 18b78 lea r15,[rip+0x11172f] # 0x12a2ae
            r15 = m.env.data_va(256635);
            pc = 0x18b7f;
        }
        0x18b7f => {
            // block 18b7f
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b7f mov eax,DWORD PTR [r13+0x0]
            rax = (m.r32(r13) as u64) as u32 as u64;
            // 18b83 cmp eax,0x6e6f696d
            fl = sub::<32>((rax as u32 as u64), 0x6e6f696du64).1;
            // 18b88 jg 0x18ba9
            if fl.g() { pc = 0x18ba9; continue; }
            pc = 0x18b8a;
        }
        0x18b8a => {
            // block 18b8a
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b8a cmp eax,0x69616c66
            fl = sub::<32>((rax as u32 as u64), 0x69616c66u64).1;
            // 18b8f jg 0x18bbd
            if fl.g() { pc = 0x18bbd; continue; }
            pc = 0x18b91;
        }
        0x18b91 => {
            // block 18b91
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b91 cmp eax,0x69616474
            fl = sub::<32>((rax as u32 as u64), 0x69616474u64).1;
            // 18b96 jne 0x18bed
            if fl.ne() { pc = 0x18bed; continue; }
            pc = 0x18b98;
        }
        0x18b98 => {
            // block 18b98
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b98 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 18b9d mov rdi,r14
            rdi = r14;
            // 18ba0 lea rsi,[rip+0x1116f9] # 0x12a2a0
            rsi = m.env.data_va(256621);
            // 18ba7 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18ba9 => {
            // block 18ba9
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18ba9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18ba9 cmp eax,0x6e6f696e
            fl = sub::<32>((rax as u32 as u64), 0x6e6f696eu64).1;
            // 18bae jne 0x18bd5
            if fl.ne() { pc = 0x18bd5; continue; }
            pc = 0x18bb0;
        }
        0x18bb0 => {
            // block 18bb0
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bb0 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 18bb5 mov rdi,r14
            rdi = r14;
            // 18bb8 mov rsi,r15
            rsi = r15;
            // 18bbb jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bbd => {
            // block 18bbd
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bbd cmp eax,0x69616c67
            fl = sub::<32>((rax as u32 as u64), 0x69616c67u64).1;
            // 18bc2 jne 0x18c05
            if fl.ne() { pc = 0x18c05; continue; }
            pc = 0x18bc4;
        }
        0x18bc4 => {
            // block 18bc4
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bc4 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 18bc9 mov rdi,r14
            rdi = r14;
            // 18bcc lea rsi,[rip+0x1116d3] # 0x12a2a6
            rsi = m.env.data_va(256627);
            // 18bd3 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bd5 => {
            // block 18bd5
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bd5 cmp eax,0x6f757420
            fl = sub::<32>((rax as u32 as u64), 0x6f757420u64).1;
            // 18bda jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18bdc;
        }
        0x18bdc => {
            // block 18bdc
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bdc mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18be1 mov rdi,r14
            rdi = r14;
            // 18be4 lea rsi,[rip+0x1116c9] # 0x12a2b4
            rsi = m.env.data_va(256641);
            // 18beb jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bed => {
            // block 18bed
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bed cmp eax,0x69616573
            fl = sub::<32>((rax as u32 as u64), 0x69616573u64).1;
            // 18bf2 jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18bf4;
        }
        0x18bf4 => {
            // block 18bf4
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bf4 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18bf9 mov rdi,r14
            rdi = r14;
            // 18bfc lea rsi,[rip+0x111698] # 0x12a29b
            rsi = m.env.data_va(256616);
            // 18c03 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18c05 => {
            // block 18c05
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c05 cmp eax,0x69736469
            fl = sub::<32>((rax as u32 as u64), 0x69736469u64).1;
            // 18c0a jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18c0c;
        }
        0x18c0c => {
            // block 18c0c
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c0c mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18c11 mov rdi,r14
            rdi = r14;
            // 18c14 lea rsi,[rip+0x11167b] # 0x12a296
            rsi = m.env.data_va(256611);
            // 18c1b jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18c1d => {
            // block 18c1d
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c1d mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18c22 mov rdi,r14
            rdi = r14;
            // 18c25 mov rsi,r12
            rsi = r12;
            pc = 0x18c28;
        }
        0x18c28 => {
            // block 18c28
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c28 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18c2d add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // 18c31 cmp r13,QWORD PTR [rbx+0x8]
            fl = sub::<64>(r13, m.r64(rbx.wrapping_add(0x8)) as u64).1;
            // 18c35 jne 0x18b7f
            if fl.ne() { pc = 0x18b7f; continue; }
            pc = 0x18c3b;
        }
        0x18c3b => {
            // block 18c3b
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c3b mov rax,r14
            rax = r14;
            // 18c3e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 18c42 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18c43 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18c45 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 18c47 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18c49 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18c4b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18c4c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19a7a size=294
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00019a7a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x19a7a;
    loop {
        match pc {
        0x19a7a => {
            // block 19a7a
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19a7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a7a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19a7b mov rbp,rsp
            rbp = rsp;
            // 19a7e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19a80 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19a82 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 19a84 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 19a86 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19a87 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19a88 mov r12,rdx
            r12 = rdx;
            // 19a8b mov r15,rdi
            r15 = rdi;
            // 19a8e mov r14,r12
            r14 = r12;
            // 19a91 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 19a94 mov rbx,r14
            rbx = r14;
            // 19a97 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19a9b mov r13,QWORD PTR [r15]
            r13 = m.r64(r15) as u64;
            // 19a9e mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19aa2 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19aa5 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19aa9 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19aac jbe 0x19af3
            if fl.be() { pc = 0x19af3; continue; }
            pc = 0x19aae;
        }
        0x19aae => {
            // block 19aae
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19aae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19aae mov r12,rsi
            r12 = rsi;
            // 19ab1 mov rdi,r15
            rdi = r15;
            // 19ab4 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19ab9 mov rax,rbx
            rax = rbx;
            // 19abc shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19ac0 je 0x19aca
            if fl.e() { pc = 0x19aca; continue; }
            pc = 0x19ac2;
        }
        0x19ac2 => {
            // block 19ac2
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19ac2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ac2 mov rdi,r15
            rdi = r15;
            // 19ac5 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x19aca => {
            // block 19aca
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19aca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19aca mov rsi,QWORD PTR [r15+0x10]
            rsi = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19ace sub rsi,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rsi, m.r64(r15) as u64);
              rsi = r;
              fl = f; }
            // 19ad1 mov rax,rsi
            rax = rsi;
            // 19ad4 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19ad8 movabs rcx,0x1ffffffffffffffe
            rcx = 0x1fff_ffff_ffff_fffeu64;
            // 19ae2 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 19ae5 ja 0x19b0c
            if fl.a() { pc = 0x19b0c; continue; }
            pc = 0x19ae7;
        }
        0x19ae7 => {
            // block 19ae7
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19ae7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ae7 sar rsi,1
            { let (r, f) = sar::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 19aea cmp rsi,rbx
            fl = sub::<64>(rsi, rbx).1;
            // 19aed cmovb rsi,rbx
            if fl.b() { rsi = rbx; }
            // 19af1 jmp 0x19b16
            pc = 0x19b16; continue;
        }
        0x19af3 => {
            // block 19af3
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19af3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19af3 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19af7 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19afa sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19afe cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19b01 jbe 0x19b33
            if fl.be() { pc = 0x19b33; continue; }
            pc = 0x19b03;
        }
        0x19b03 => {
            // block 19b03
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b03 lea rbx,[rsi+rax*4]
            rbx = rsi.wrapping_add(rax.wrapping_mul(4));
            // 19b07 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 19b0a jmp 0x19b39
            pc = 0x19b39; continue;
        }
        0x19b0c => {
            // block 19b0c
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b0c movabs rsi,0x3fffffffffffffff
            rsi = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x19b16;
        }
        0x19b16 => {
            // block 19b16
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b16 mov rdi,r15
            rdi = r15;
            // 19b19 call 0xeba20
            { let r = { let t = m.env.stub_va(0x1ed); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19b1e mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19b22 mov rsi,r12
            rsi = r12;
            // 19b25 mov rdx,r14
            rdx = r14;
            // 19b28 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19b2d shl rbx,0x2
            { let (r, f) = shl::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19b31 jmp 0x19b6a
            pc = 0x19b6a; continue;
        }
        0x19b33 => {
            // block 19b33
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b33 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 19b36 mov rbx,r12
            rbx = r12;
            pc = 0x19b39;
        }
        0x19b39 => {
            // block 19b39
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b39 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 19b3d sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 19b40 mov rdi,r13
            rdi = r13;
            // 19b43 mov rdx,rbx
            rdx = rbx;
            // 19b46 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19b4b mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19b4f test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 19b52 je 0x19b70
            if fl.e() { pc = 0x19b70; continue; }
            pc = 0x19b54;
        }
        0x19b54 => {
            // block 19b54
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b54 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 19b58 sub r12,rsi
            { let (r, f) = sub::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 19b5b mov rbx,r12
            rbx = r12;
            // 19b5e and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19b62 mov rdx,r12
            rdx = r12;
            // 19b65 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x19b6a;
        }
        0x19b6a => {
            // block 19b6a
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b6a add QWORD PTR [r15+0x8],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rbx);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 19b6e jmp 0x19b91
            pc = 0x19b91; continue;
        }
        0x19b70 => {
            // block 19b70
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b70 and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19b74 add r13,rbx
            { let (r, f) = add::<64>(r13, rbx);
              r13 = r;
              fl = f; }
            // 19b77 cmp rdi,r13
            fl = sub::<64>(rdi, r13).1;
            // 19b7a je 0x19b91
            if fl.e() { pc = 0x19b91; continue; }
            pc = 0x19b7c;
        }
        0x19b7c => {
            // block 19b7c
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b7c lea rax,[rdi-0x4]
            rax = rdi.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19b80 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19b83 not rax
            { let v = !rax; rax = v; }
            // 19b86 and rax,0xfffffffffffffffc
            { let r = rax & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 19b8a add rax,rdi
            { let (r, f) = add::<64>(rax, rdi);
              rax = r;
              fl = f; }
            // 19b8d mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            pc = 0x19b91;
        }
        0x19b91 => {
            // block 19b91
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b91 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19b95 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19b96 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 19b98 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 19b9a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19b9c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 19b9e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19b9f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19dea size=294
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00019dea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x19dea;
    loop {
        match pc {
        0x19dea => {
            // block 19dea
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19dea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19dea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19deb mov rbp,rsp
            rbp = rsp;
            // 19dee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19df0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19df2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 19df4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 19df6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19df7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19df8 mov r12,rdx
            r12 = rdx;
            // 19dfb mov r15,rdi
            r15 = rdi;
            // 19dfe mov r14,r12
            r14 = r12;
            // 19e01 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 19e04 mov rbx,r14
            rbx = r14;
            // 19e07 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19e0b mov r13,QWORD PTR [r15]
            r13 = m.r64(r15) as u64;
            // 19e0e mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19e12 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19e15 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19e19 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19e1c jbe 0x19e63
            if fl.be() { pc = 0x19e63; continue; }
            pc = 0x19e1e;
        }
        0x19e1e => {
            // block 19e1e
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e1e mov r12,rsi
            r12 = rsi;
            // 19e21 mov rdi,r15
            rdi = r15;
            // 19e24 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19e29 mov rax,rbx
            rax = rbx;
            // 19e2c shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19e30 je 0x19e3a
            if fl.e() { pc = 0x19e3a; continue; }
            pc = 0x19e32;
        }
        0x19e32 => {
            // block 19e32
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e32 mov rdi,r15
            rdi = r15;
            // 19e35 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x19e3a => {
            // block 19e3a
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e3a mov rsi,QWORD PTR [r15+0x10]
            rsi = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19e3e sub rsi,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rsi, m.r64(r15) as u64);
              rsi = r;
              fl = f; }
            // 19e41 mov rax,rsi
            rax = rsi;
            // 19e44 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19e48 movabs rcx,0x1ffffffffffffffe
            rcx = 0x1fff_ffff_ffff_fffeu64;
            // 19e52 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 19e55 ja 0x19e7c
            if fl.a() { pc = 0x19e7c; continue; }
            pc = 0x19e57;
        }
        0x19e57 => {
            // block 19e57
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e57 sar rsi,1
            { let (r, f) = sar::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 19e5a cmp rsi,rbx
            fl = sub::<64>(rsi, rbx).1;
            // 19e5d cmovb rsi,rbx
            if fl.b() { rsi = rbx; }
            // 19e61 jmp 0x19e86
            pc = 0x19e86; continue;
        }
        0x19e63 => {
            // block 19e63
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e63 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19e67 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19e6a sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19e6e cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19e71 jbe 0x19ea3
            if fl.be() { pc = 0x19ea3; continue; }
            pc = 0x19e73;
        }
        0x19e73 => {
            // block 19e73
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e73 lea rbx,[rsi+rax*4]
            rbx = rsi.wrapping_add(rax.wrapping_mul(4));
            // 19e77 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 19e7a jmp 0x19ea9
            pc = 0x19ea9; continue;
        }
        0x19e7c => {
            // block 19e7c
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e7c movabs rsi,0x3fffffffffffffff
            rsi = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x19e86;
        }
        0x19e86 => {
            // block 19e86
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e86 mov rdi,r15
            rdi = r15;
            // 19e89 call 0xeba44
            { let r = { let t = m.env.stub_va(0x211); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19e8e mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19e92 mov rsi,r12
            rsi = r12;
            // 19e95 mov rdx,r14
            rdx = r14;
            // 19e98 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19e9d shl rbx,0x2
            { let (r, f) = shl::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19ea1 jmp 0x19eda
            pc = 0x19eda; continue;
        }
        0x19ea3 => {
            // block 19ea3
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ea3 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 19ea6 mov rbx,r12
            rbx = r12;
            pc = 0x19ea9;
        }
        0x19ea9 => {
            // block 19ea9
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ea9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ea9 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 19ead sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 19eb0 mov rdi,r13
            rdi = r13;
            // 19eb3 mov rdx,rbx
            rdx = rbx;
            // 19eb6 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19ebb mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19ebf test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 19ec2 je 0x19ee0
            if fl.e() { pc = 0x19ee0; continue; }
            pc = 0x19ec4;
        }
        0x19ec4 => {
            // block 19ec4
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ec4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ec4 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 19ec8 sub r12,rsi
            { let (r, f) = sub::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 19ecb mov rbx,r12
            rbx = r12;
            // 19ece and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19ed2 mov rdx,r12
            rdx = r12;
            // 19ed5 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x19eda;
        }
        0x19eda => {
            // block 19eda
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19eda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19eda add QWORD PTR [r15+0x8],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rbx);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 19ede jmp 0x19f01
            pc = 0x19f01; continue;
        }
        0x19ee0 => {
            // block 19ee0
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ee0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ee0 and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19ee4 add r13,rbx
            { let (r, f) = add::<64>(r13, rbx);
              r13 = r;
              fl = f; }
            // 19ee7 cmp rdi,r13
            fl = sub::<64>(rdi, r13).1;
            // 19eea je 0x19f01
            if fl.e() { pc = 0x19f01; continue; }
            pc = 0x19eec;
        }
        0x19eec => {
            // block 19eec
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19eec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19eec lea rax,[rdi-0x4]
            rax = rdi.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19ef0 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19ef3 not rax
            { let v = !rax; rax = v; }
            // 19ef6 and rax,0xfffffffffffffffc
            { let r = rax & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 19efa add rax,rdi
            { let (r, f) = add::<64>(rax, rdi);
              rax = r;
              fl = f; }
            // 19efd mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            pc = 0x19f01;
        }
        0x19f01 => {
            // block 19f01
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19f01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f01 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19f05 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19f06 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 19f08 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 19f0a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19f0c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 19f0e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19f0f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2893b size=276
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002893b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2893b;
    loop {
        match pc {
        0x2893b => {
            // block 2893b
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 2893b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2893b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2893c mov rbp,rsp
            rbp = rsp;
            // 2893f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28941 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28943 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 28945 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 28947 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28948 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2894c mov r12,rsi
            r12 = rsi;
            // 2894f mov r14,rdi
            r14 = rdi;
            // 28952 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 28956 mov rsi,QWORD PTR [r12+0x10]
            rsi = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2895b call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28960 cmp QWORD PTR [r12+0x10],0x1
            fl = sub::<64>(m.r64(r12.wrapping_add(0x10)) as u64, 0x1u64).1;
            // 28966 lea rcx,[rip+0x101ea8] # 0x12a815
            rcx = m.env.data_va(258018);
            // 2896d lea rsi,[rip+0x101eae] # 0x12a822
            rsi = m.env.data_va(258031);
            // 28974 cmove rsi,rcx
            if fl.e() { rsi = rcx; }
            // 28978 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 2897b movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // 2897e or rdx,0xc
            { let r = rdx | 0xcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 28982 mov rdi,rax
            rdi = rax;
            // 28985 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2898a add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // 2898e cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28991 je 0x28a29
            if fl.e() { pc = 0x28a29; continue; }
            pc = 0x28997;
        }
        0x28997 => {
            // block 28997
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 28997 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28997 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2899b lea r13,[rip+0x101afb] # 0x12a49d
            r13 = m.env.data_va(257130);
            pc = 0x289a2;
        }
        0x289a2 => {
            // block 289a2
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289a2 mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 289a5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 289a7 mov rdi,r15
            rdi = r15;
            // 289aa call 0x5b5c9
            { let r = { let t = m.env.code_va(0x5b5c9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 289af movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 289b3 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 289b6 jne 0x289c1
            if fl.ne() { pc = 0x289c1; continue; }
            pc = 0x289b8;
        }
        0x289b8 => {
            // block 289b8
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289b8 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 289bb lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 289bf jmp 0x289c9
            pc = 0x289c9; continue;
        }
        0x289c1 => {
            // block 289c1
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289c1 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 289c5 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x289c9;
        }
        0x289c9 => {
            // block 289c9
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289c9 mov rdi,r14
            rdi = r14;
            // 289cc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 289d1 mov rdi,r15
            rdi = r15;
            // 289d4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 289d9 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 289dd test rax,rax
            fl = logic::<64>(rax & rax);
            // 289e0 je 0x289ef
            if fl.e() { pc = 0x289ef; continue; }
            pc = 0x289e2;
        }
        0x289e2 => {
            // block 289e2
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289e2 mov rbx,rax
            rbx = rax;
            // 289e5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 289e8 test rax,rax
            fl = logic::<64>(rax & rax);
            // 289eb jne 0x289e2
            if fl.ne() { pc = 0x289e2; continue; }
            pc = 0x289ed;
        }
        0x289ed => {
            // block 289ed
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289ed jmp 0x289fe
            pc = 0x289fe; continue;
        }
        0x289ef => {
            // block 289ef
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289ef mov rax,rbx
            rax = rbx;
            pc = 0x289f2;
        }
        0x289f2 => {
            // block 289f2
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289f2 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 289f6 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 289f9 mov rax,rbx
            rax = rbx;
            // 289fc jne 0x289f2
            if fl.ne() { pc = 0x289f2; continue; }
            pc = 0x289fe;
        }
        0x289fe => {
            // block 289fe
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289fe cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28a01 mov rsi,r13
            rsi = r13;
            // 28a04 lea rax,[rip+0x1102bd] # 0x138cc8
            rax = m.env.data_va(316565);
            // 28a0b cmove rsi,rax
            if fl.e() { rsi = rax; }
            // 28a0f setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 28a12 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 28a15 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 28a18 mov rdi,r14
            rdi = r14;
            // 28a1b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28a20 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28a23 jne 0x289a2
            if fl.ne() { pc = 0x289a2; continue; }
            pc = 0x28a29;
        }
        0x28a29 => {
            // block 28a29
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 28a29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a29 mov rax,r14
            rax = r14;
            // 28a2c add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 28a30 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28a31 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 28a33 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 28a35 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28a37 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28a39 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28a3a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x28a3b => {
            // block 28a3b
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 28a3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a3b mov rbx,rax
            rbx = rax;
            // 28a3e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 28a42 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28a47 mov rdi,rbx
            rdi = rbx;
            // 28a4a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29800 size=296
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029800<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x29800;
    loop {
        match pc {
        0x29800 => {
            // block 29800
            #[cfg(feature = "trace")] eprintln!("f_00029800 pc 29800 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29800 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29801 mov rbp,rsp
            rbp = rsp;
            // 29804 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 29806 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29807 mov r14,rsi
            r14 = rsi;
            // 2980a mov rbx,rdi
            rbx = rdi;
            // 2980d cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 29810 je 0x2991f
            if fl.e() { pc = 0x2991f; continue; }
            pc = 0x29816;
        }
        0x29816 => {
            // block 29816
            #[cfg(feature = "trace")] eprintln!("f_00029800 pc 29816 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29816 lea rdi,[rbx+0x50]
            rdi = rbx.wrapping_add(0x50);
            // 2981a lea rsi,[r14+0x50]
            rsi = r14.wrapping_add(0x50);
            // 2981e call 0x28e5e
            { let r = { let t = m.env.code_va(0x28e5e); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29823 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 29827 mov QWORD PTR [rbx+0x18],rax
            m.w64(rbx.wrapping_add(0x18), (rax) as u64);
            // 2982b mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 2982f mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 29833 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 29836 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 2983a mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 2983e mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 29841 mov rax,QWORD PTR [r14+0x20]
            rax = m.r64(r14.wrapping_add(0x20)) as u64;
            // 29845 mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // 29849 mov eax,DWORD PTR [r14+0x28]
            rax = (m.r32(r14.wrapping_add(0x28)) as u64) as u32 as u64;
            // 2984d mov DWORD PTR [rbx+0x28],eax
            m.w32(rbx.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 29850 mov rax,QWORD PTR [r14+0x2c]
            rax = m.r64(r14.wrapping_add(0x2c)) as u64;
            // 29854 mov QWORD PTR [rbx+0x2c],rax
            m.w64(rbx.wrapping_add(0x2c), (rax) as u64);
            // 29858 movups xmm0,XMMWORD PTR [r14+0x34]
            x0 = m.r128(r14.wrapping_add(0x34));
            // 2985d movups XMMWORD PTR [rbx+0x34],xmm0
            m.w128(rbx.wrapping_add(0x34), x0);
            // 29861 mov eax,DWORD PTR [r14+0x44]
            rax = (m.r32(r14.wrapping_add(0x44)) as u64) as u32 as u64;
            // 29865 mov DWORD PTR [rbx+0x44],eax
            m.w32(rbx.wrapping_add(0x44), ((rax as u32 as u64)) as u32);
            // 29868 mov eax,DWORD PTR [r14+0x48]
            rax = (m.r32(r14.wrapping_add(0x48)) as u64) as u32 as u64;
            // 2986c mov DWORD PTR [rbx+0x48],eax
            m.w32(rbx.wrapping_add(0x48), ((rax as u32 as u64)) as u32);
            // 2986f mov eax,DWORD PTR [r14+0x4c]
            rax = (m.r32(r14.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 29873 mov DWORD PTR [rbx+0x4c],eax
            m.w32(rbx.wrapping_add(0x4c), ((rax as u32 as u64)) as u32);
            // 29876 mov rax,QWORD PTR [r14+0x78]
            rax = m.r64(r14.wrapping_add(0x78)) as u64;
            // 2987a mov QWORD PTR [rbx+0x78],rax
            m.w64(rbx.wrapping_add(0x78), (rax) as u64);
            // 2987e mov eax,DWORD PTR [r14+0x80]
            rax = (m.r32(r14.wrapping_add(0x80)) as u64) as u32 as u64;
            // 29885 mov DWORD PTR [rbx+0x80],eax
            m.w32(rbx.wrapping_add(0x80), ((rax as u32 as u64)) as u32);
            // 2988b movups xmm0,XMMWORD PTR [r14+0x84]
            x0 = m.r128(r14.wrapping_add(0x84));
            // 29893 movups XMMWORD PTR [rbx+0x84],xmm0
            m.w128(rbx.wrapping_add(0x84), x0);
            // 2989a mov eax,DWORD PTR [r14+0x94]
            rax = (m.r32(r14.wrapping_add(0x94)) as u64) as u32 as u64;
            // 298a1 mov DWORD PTR [rbx+0x94],eax
            m.w32(rbx.wrapping_add(0x94), ((rax as u32 as u64)) as u32);
            // 298a7 mov eax,DWORD PTR [r14+0x98]
            rax = (m.r32(r14.wrapping_add(0x98)) as u64) as u32 as u64;
            // 298ae mov DWORD PTR [rbx+0x98],eax
            m.w32(rbx.wrapping_add(0x98), ((rax as u32 as u64)) as u32);
            // 298b4 mov eax,DWORD PTR [r14+0x9c]
            rax = (m.r32(r14.wrapping_add(0x9c)) as u64) as u32 as u64;
            // 298bb mov DWORD PTR [rbx+0x9c],eax
            m.w32(rbx.wrapping_add(0x9c), ((rax as u32 as u64)) as u32);
            // 298c1 mov eax,DWORD PTR [r14+0xa0]
            rax = (m.r32(r14.wrapping_add(0xa0)) as u64) as u32 as u64;
            // 298c8 mov DWORD PTR [rbx+0xa0],eax
            m.w32(rbx.wrapping_add(0xa0), ((rax as u32 as u64)) as u32);
            // 298ce mov eax,DWORD PTR [r14+0xa4]
            rax = (m.r32(r14.wrapping_add(0xa4)) as u64) as u32 as u64;
            // 298d5 mov DWORD PTR [rbx+0xa4],eax
            m.w32(rbx.wrapping_add(0xa4), ((rax as u32 as u64)) as u32);
            // 298db mov rax,QWORD PTR [r14+0xa8]
            rax = m.r64(r14.wrapping_add(0xa8)) as u64;
            // 298e2 mov QWORD PTR [rbx+0xa8],rax
            m.w64(rbx.wrapping_add(0xa8), (rax) as u64);
            // 298e9 mov eax,DWORD PTR [r14+0xb0]
            rax = (m.r32(r14.wrapping_add(0xb0)) as u64) as u32 as u64;
            // 298f0 mov DWORD PTR [rbx+0xb0],eax
            m.w32(rbx.wrapping_add(0xb0), ((rax as u32 as u64)) as u32);
            // 298f6 mov eax,DWORD PTR [r14+0xbc]
            rax = (m.r32(r14.wrapping_add(0xbc)) as u64) as u32 as u64;
            // 298fd mov DWORD PTR [rbx+0xbc],eax
            m.w32(rbx.wrapping_add(0xbc), ((rax as u32 as u64)) as u32);
            // 29903 mov rax,QWORD PTR [r14+0xb4]
            rax = m.r64(r14.wrapping_add(0xb4)) as u64;
            // 2990a mov QWORD PTR [rbx+0xb4],rax
            m.w64(rbx.wrapping_add(0xb4), (rax) as u64);
            // 29911 mov rax,QWORD PTR [r14+0xc0]
            rax = m.r64(r14.wrapping_add(0xc0)) as u64;
            // 29918 mov QWORD PTR [rbx+0xc0],rax
            m.w64(rbx.wrapping_add(0xc0), (rax) as u64);
            pc = 0x2991f;
        }
        0x2991f => {
            // block 2991f
            #[cfg(feature = "trace")] eprintln!("f_00029800 pc 2991f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2991f mov rax,rbx
            rax = rbx;
            // 29922 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29923 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 29925 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29926 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x33a14 size=244
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00033a14<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x33a14;
    loop {
        match pc {
        0x33a14 => {
            // block 33a14
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a14 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 33a15 mov rbp,rsp
            rbp = rsp;
            // 33a18 mov r8,rdx
            r8 = rdx;
            // 33a1b mov BYTE PTR [rcx],0x0
            m.w8(rcx, (0x0u64) as u8);
            // 33a1e mov DWORD PTR [r8],0x0
            m.w32(r8, (0x0u64) as u32);
            // 33a25 mov edx,DWORD PTR [rdi]
            rdx = (m.r32(rdi) as u64) as u32 as u64;
            // 33a27 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 33a29 je 0x33a7b
            if fl.e() { pc = 0x33a7b; continue; }
            pc = 0x33a2b;
        }
        0x33a2b => {
            // block 33a2b
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a2b cmp DWORD PTR [rdi+0x4],0x0
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 33a2f je 0x33a7b
            if fl.e() { pc = 0x33a7b; continue; }
            pc = 0x33a31;
        }
        0x33a31 => {
            // block 33a31
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a31 cmp WORD PTR [rdi+0x34],0x0
            fl = sub::<16>(m.r16(rdi.wrapping_add(0x34)) as u64, 0x0u64).1;
            // 33a36 je 0x33a7b
            if fl.e() { pc = 0x33a7b; continue; }
            pc = 0x33a38;
        }
        0x33a38 => {
            // block 33a38
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a38 cmp DWORD PTR [rdi+0x24],0x0
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 33a3c je 0x33a7b
            if fl.e() { pc = 0x33a7b; continue; }
            pc = 0x33a3e;
        }
        0x33a3e => {
            // block 33a3e
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a3e mov r9d,DWORD PTR [rdi+0x10]
            r9 = (m.r32(rdi.wrapping_add(0x10)) as u64) as u32 as u64;
            // 33a42 cmp r9,0xb
            fl = sub::<64>(r9, 0xbu64).1;
            // 33a46 ja 0x33a7b
            if fl.a() { pc = 0x33a7b; continue; }
            pc = 0x33a48;
        }
        0x33a48 => {
            // block 33a48
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a48 mov edi,DWORD PTR [rdi+0x1c]
            rdi = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 33a4b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33a4d cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // 33a50 ja 0x33a7d
            if fl.a() { pc = 0x33a7d; continue; }
            pc = 0x33a52;
        }
        0x33a52 => {
            // block 33a52
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a52 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // 33a54 jbe 0x33a7d
            if fl.be() { pc = 0x33a7d; continue; }
            pc = 0x33a56;
        }
        0x33a56 => {
            // block 33a56
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a56 lea eax,[r9-0x1]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33a5a cmp eax,0xb
            fl = sub::<32>((rax as u32 as u64), 0xbu64).1;
            // 33a5d jae 0x33a69
            if fl.ae() { pc = 0x33a69; continue; }
            pc = 0x33a5f;
        }
        0x33a5f => {
            // block 33a5f
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a5f mov edx,0x7a9
            rdx = (0x7a9u64) as u32 as u64;
            // 33a64 bt edx,eax
            fl.cf = bit::<32>((rdx as u32 as u64), (rax as u32 as u64));
            // 33a67 jb 0x33a7f
            if fl.b() { pc = 0x33a7f; continue; }
            pc = 0x33a69;
        }
        0x33a69 => {
            // block 33a69
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a69 cmp r9d,0x2
            fl = sub::<32>((r9 as u32 as u64), 0x2u64).1;
            // 33a6d sete dl
            rdx = (rdx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 33a70 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 33a72 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33a75 xor al,dl
            { let r = (rax as u8 as u64) ^ (rdx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 33a77 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // 33a79 jmp 0x33a81
            pc = 0x33a81; continue;
        }
        0x33a7b => {
            // block 33a7b
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a7b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x33a7d;
        }
        0x33a7d => {
            // block 33a7d
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a7d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 33a7e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x33a7f => {
            // block 33a7f
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a7f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x33a81;
        }
        0x33a81 => {
            // block 33a81
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a81 lea ecx,[r9-0x1]
            rcx = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33a85 cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 33a88 jae 0x33a9a
            if fl.ae() { pc = 0x33a9a; continue; }
            pc = 0x33a8a;
        }
        0x33a8a => {
            // block 33a8a
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a8a mov r10d,0x1
            r10 = (0x1u64) as u32 as u64;
            // 33a90 mov edx,0x3a9
            rdx = (0x3a9u64) as u32 as u64;
            // 33a95 bt edx,ecx
            fl.cf = bit::<32>((rdx as u32 as u64), (rcx as u32 as u64));
            // 33a98 jb 0x33aa8
            if fl.b() { pc = 0x33aa8; continue; }
            pc = 0x33a9a;
        }
        0x33a9a => {
            // block 33a9a
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a9a cmp r9d,0xb
            fl = sub::<32>((r9 as u32 as u64), 0xbu64).1;
            // 33a9e setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 33aa1 movzx r10d,cl
            r10 = ((rcx as u8 as u64)) as u32 as u64;
            // 33aa5 inc r10d
            { let (r, f) = inc::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            pc = 0x33aa8;
        }
        0x33aa8 => {
            // block 33aa8
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33aa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33aa8 cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // 33aab je 0x33ac4
            if fl.e() { pc = 0x33ac4; continue; }
            pc = 0x33aad;
        }
        0x33aad => {
            // block 33aad
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33aad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33aad cmp edi,0x1
            fl = sub::<32>((rdi as u32 as u64), 0x1u64).1;
            // 33ab0 jne 0x33ad6
            if fl.ne() { pc = 0x33ad6; continue; }
            pc = 0x33ab2;
        }
        0x33ab2 => {
            // block 33ab2
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33ab2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ab2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 33ab4 lea rax,[rip+0xcc495] # 0xfff50
            rax = m.env.data_va(83741);
            // 33abb lea rcx,[rip+0xcc44e] # 0xfff10
            rcx = m.env.data_va(83677);
            // 33ac2 jmp 0x33aec
            pc = 0x33aec; continue;
        }
        0x33ac4 => {
            // block 33ac4
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33ac4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ac4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 33ac6 lea rax,[rip+0xcc503] # 0xfffd0
            rax = m.env.data_va(83869);
            // 33acd lea rcx,[rip+0xcc4bc] # 0xfff90
            rcx = m.env.data_va(83805);
            // 33ad4 jmp 0x33aec
            pc = 0x33aec; continue;
        }
        0x33ad6 => {
            // block 33ad6
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33ad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ad6 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 33ad8 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // 33ada jne 0x33af4
            if fl.ne() { pc = 0x33af4; continue; }
            pc = 0x33adc;
        }
        0x33adc => {
            // block 33adc
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33adc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33adc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 33ade lea rax,[rip+0xcc3eb] # 0xffed0
            rax = m.env.data_va(83613);
            // 33ae5 lea rcx,[rip+0xcc3a4] # 0xffe90
            rcx = m.env.data_va(83549);
            pc = 0x33aec;
        }
        0x33aec => {
            // block 33aec
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33aec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33aec cmove rcx,rax
            if fl.e() { rcx = rax; }
            // 33af0 mov ecx,DWORD PTR [rcx+r9*4]
            rcx = (m.r32(rcx.wrapping_add(r9.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x33af4;
        }
        0x33af4 => {
            // block 33af4
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33af4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33af4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33af6 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 33af8 div r10d
            { let (q, r) = div::<32>(rdx, rax, (r10 as u32 as u64)); rax = q; rdx = r; }
            // 33afb add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 33afd mov DWORD PTR [r8],eax
            m.w32(r8, ((rax as u32 as u64)) as u32);
            // 33b00 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 33b02 jmp 0x33a7d
            pc = 0x33a7d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x441ca size=252
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000441ca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x441ca;
    loop {
        match pc {
        0x441ca => {
            // block 441ca
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 441ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 441ca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 441cb mov rbp,rsp
            rbp = rsp;
            // 441ce push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 441d0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 441d2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 441d4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 441d6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 441d7 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 441db mov r15,rsi
            r15 = rsi;
            // 441de mov r14,rdi
            r14 = rdi;
            // 441e1 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 441e8 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 441ef lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // 441f3 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 441f7 mov rdi,r15
            rdi = r15;
            // 441fa call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 441ff mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // 44207 mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // 4420a mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 44212 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // 44216 call 0xcb77f
            { let r = { let t = m.env.code_va(0xcb77f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4421b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4421d je 0x442b1
            if fl.e() { pc = 0x442b1; continue; }
            pc = 0x44223;
        }
        0x44223 => {
            // block 44223
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 44223 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44223 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 44226 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 4422a xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 4422d mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // 44232 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44237 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4423a mov rdi,r14
            rdi = r14;
            // 4423d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44240 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44242 je 0x442b4
            if fl.e() { pc = 0x442b4; continue; }
            pc = 0x44244;
        }
        0x44244 => {
            // block 44244
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 44244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44244 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 44247 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 4424b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 4424e mov esi,0x19
            rsi = (0x19u64) as u32 as u64;
            // 44253 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44258 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4425b mov rdi,r14
            rdi = r14;
            // 4425e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44261 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44263 je 0x442b4
            if fl.e() { pc = 0x442b4; continue; }
            pc = 0x44265;
        }
        0x44265 => {
            // block 44265
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 44265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44265 movzx eax,BYTE PTR [rbp-0x29]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd7)) as u64) as u32 as u64;
            // 44269 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4426c movzx r12d,BYTE PTR [rbp-0x2d]
            r12 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3)) as u64) as u32 as u64;
            // 44271 and r12d,0xf0
            { let r = (r12 as u32 as u64) & 0xf0u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 44278 or r12d,eax
            { let r = (r12 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4427b mov DWORD PTR [rbp-0x34],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0x0u64) as u32);
            // 44282 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 44287 lea r14,[rbp-0x34]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            pc = 0x4428b;
        }
        0x4428b => {
            // block 4428b
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 4428b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4428b lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 4428e bt r12d,eax
            fl.cf = bit::<32>((r12 as u32 as u64), (rax as u32 as u64));
            // 44292 jb 0x4429f
            if fl.b() { pc = 0x4429f; continue; }
            pc = 0x44294;
        }
        0x44294 => {
            // block 44294
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 44294 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44294 mov rdi,r15
            rdi = r15;
            // 44297 mov rsi,r14
            rsi = r14;
            // 4429a call 0xebb52
            { let r = { let t = m.env.stub_va(0x31f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4429f;
        }
        0x4429f => {
            // block 4429f
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 4429f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4429f mov DWORD PTR [rbp-0x34],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rbx as u32 as u64)) as u32);
            // 442a2 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 442a5 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 442a8 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 442ab mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 442ad jl 0x4428b
            if fl.l() { pc = 0x4428b; continue; }
            pc = 0x442af;
        }
        0x442af => {
            // block 442af
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 442af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 442af jmp 0x442b4
            pc = 0x442b4; continue;
        }
        0x442b1 => {
            // block 442b1
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 442b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 442b1 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x442b4;
        }
        0x442b4 => {
            // block 442b4
            #[cfg(feature = "trace")] eprintln!("f_000441ca pc 442b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 442b4 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 442b7 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 442bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 442bc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 442be pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 442c0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 442c2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 442c4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 442c5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x593a3 size=264
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000593a3<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x593a3;
    loop {
        match pc {
        0x593a3 => {
            // block 593a3
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 593a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 593a3 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 593a4 mov rbp,rsp
            rbp = rsp;
            // 593a7 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 593a9 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 593ab push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 593ad push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 593ae sub rsp,0x2050
            { let (r, f) = sub::<64>(rsp, 0x2050u64);
              rsp = r;
              fl = f; }
            // 593b5 mov r14,rcx
            r14 = rcx;
            // 593b8 mov r12,rdi
            r12 = rdi;
            // 593bb mov al,BYTE PTR [rbp+0x10]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0x10)) as u64) as u8 as u64);
            // 593be mov r15,QWORD PTR [rip+0xe7013] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 593c5 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 593c8 mov QWORD PTR [rbp-0x28],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (r15) as u64);
            // 593cc movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 593cf movzx ecx,r8b
            rcx = ((r8 as u8 as u64)) as u32 as u64;
            // 593d3 movzx r8d,r9b
            r8 = ((r9 as u8 as u64)) as u32 as u64;
            // 593d7 lea rdi,[rbp-0x2070]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df90);
            // 593de mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // 593e1 call 0x32c5c
            { let r = { let t = m.env.code_va(0x32c5c); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 593e6 mov edx,DWORD PTR [rbp-0x206c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df94)) as u64) as u32 as u64;
            // 593ec test dx,dx
            fl = logic::<16>((rdx as u16 as u64) & (rdx as u16 as u64));
            // 593ef je 0x5943b
            if fl.e() { pc = 0x5943b; continue; }
            pc = 0x593f1;
        }
        0x593f1 => {
            // block 593f1
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 593f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 593f1 movabs rax,0xffffffffffff
            rax = 0x0_ffff_ffff_ffffu64;
            // 593fb and rax,r14
            { let r = rax & r14; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 593fe mov rcx,rax
            rcx = rax;
            // 59401 shr rcx,0x10
            { let (r, f) = shr::<64>(rcx, 0x10u64, fl); rcx = r; fl = f; }
            // 59405 shr rax,0x20
            { let (r, f) = shr::<64>(rax, 0x20u64, fl); rax = r; fl = f; }
            // 59409 movzx esi,dx
            rsi = ((rdx as u16 as u64)) as u32 as u64;
            // 5940c add rsi,rsi
            { let (r, f) = add::<64>(rsi, rsi);
              rsi = r;
              fl = f; }
            // 5940f xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0x59411;
        }
        0x59411 => {
            // block 59411
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59411 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59411 mov WORD PTR [rbp+rdi*2-0x2030],r14w
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd0), ((r14 as u16 as u64)) as u16);
            // 5941a mov WORD PTR [rbp+rdi*2-0x202e],cx
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd2), ((rcx as u16 as u64)) as u16);
            // 59422 mov WORD PTR [rbp+rdi*2-0x202c],ax
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd4), ((rax as u16 as u64)) as u16);
            // 5942a mov WORD PTR [rbp+rdi*2-0x202a],cx
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd6), ((rcx as u16 as u64)) as u16);
            // 59432 add rdi,0x4
            { let (r, f) = add::<64>(rdi, 0x4u64);
              rdi = r;
              fl = f; }
            // 59436 cmp rdi,rsi
            fl = sub::<64>(rdi, rsi).1;
            // 59439 jl 0x59411
            if fl.l() { pc = 0x59411; continue; }
            pc = 0x5943b;
        }
        0x5943b => {
            // block 5943b
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 5943b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5943b cmp DWORD PTR [rbp-0x2070],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df90)) as u64, 0x0u64).1;
            // 59442 je 0x59490
            if fl.e() { pc = 0x59490; continue; }
            pc = 0x59444;
        }
        0x59444 => {
            // block 59444
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59444 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59444 lea rdi,[rbp-0x2030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            // 5944b mov rsi,r12
            rsi = r12;
            // 5944e call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59453 cmp DWORD PTR [rbp-0x2070],0x2
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df90)) as u64, 0x2u64).1;
            // 5945a jb 0x59490
            if fl.b() { pc = 0x59490; continue; }
            pc = 0x5945c;
        }
        0x5945c => {
            // block 5945c
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 5945c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5945c mov bx,0x2
            rbx = (rbx & !0xffff) | ((0x2u64) as u16 as u64);
            // 59460 lea r14,[rbp-0x2030]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            pc = 0x59467;
        }
        0x59467 => {
            // block 59467
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59467 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59467 mov edx,DWORD PTR [rbp-0x206c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df94)) as u64) as u32 as u64;
            // 5946d mov eax,DWORD PTR [rbp-0x2068]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df98)) as u64) as u32 as u64;
            // 59473 lea r12,[r12+rax*4]
            r12 = r12.wrapping_add(rax.wrapping_mul(4));
            // 59477 mov rdi,r14
            rdi = r14;
            // 5947a mov rsi,r12
            rsi = r12;
            // 5947d call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59482 movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // 59485 lea ebx,[rbx+0x1]
            rbx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 59488 cmp eax,DWORD PTR [rbp-0x2070]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df90)) as u64).1;
            // 5948e jb 0x59467
            if fl.b() { pc = 0x59467; continue; }
            pc = 0x59490;
        }
        0x59490 => {
            // block 59490
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59490 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59490 cmp r15,QWORD PTR [rbp-0x28]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64).1;
            // 59494 jne 0x594a6
            if fl.ne() { pc = 0x594a6; continue; }
            pc = 0x59496;
        }
        0x59496 => {
            // block 59496
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59496 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59496 add rsp,0x2050
            { let (r, f) = add::<64>(rsp, 0x2050u64);
              rsp = r;
              fl = f; }
            // 5949d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5949e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 594a0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 594a2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 594a4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 594a5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x594a6 => {
            // block 594a6
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 594a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 594a6 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x632dc size=291
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000632dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x632dc;
    loop {
        match pc {
        0x632dc => {
            // block 632dc
            #[cfg(feature = "trace")] eprintln!("f_000632dc pc 632dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 632dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 632dd mov rbp,rsp
            rbp = rsp;
            // 632e0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 632e1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 632e2 mov rbx,rsi
            rbx = rsi;
            // 632e5 lea rsi,[rip+0xce4d3] # 0x1317bf
            rsi = m.env.data_va(286604);
            // 632ec mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 632f1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 632f6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 632f9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 632fd mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 63301 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 63304 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 63307 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 6330b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 6330e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 63312 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 6331b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 6331e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 63322 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 6332d mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 6332f mov rdi,rax
            rdi = rax;
            // 63332 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63337 lea rsi,[rip+0xce488] # 0x1317c6
            rsi = m.env.data_va(286611);
            // 6333e mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 63343 mov rdi,rax
            rdi = rax;
            // 63346 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6334b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 6334e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 63352 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 63356 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 63359 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6335c mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 63360 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 63363 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 63367 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 63370 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 63373 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 63377 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 63382 mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 63385 mov rdi,rax
            rdi = rax;
            // 63388 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6338d lea rsi,[rip+0xce439] # 0x1317cd
            rsi = m.env.data_va(286618);
            // 63394 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 63399 mov rdi,rax
            rdi = rax;
            // 6339c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 633a1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 633a4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 633a8 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 633ac and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 633af or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 633b2 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 633b6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 633b9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 633bd mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 633c6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 633c9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 633cd mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 633d8 mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 633db mov rdi,rax
            rdi = rax;
            // 633de call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 633e3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 633e6 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 633ea mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 633ee and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 633f1 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 633f4 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 633f8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 633fc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 633fd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 633fe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6911e size=296
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006911e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6911e;
    loop {
        match pc {
        0x6911e => {
            // block 6911e
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6911e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6911e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6911f mov rbp,rsp
            rbp = rsp;
            // 69122 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 69124 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 69126 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 69128 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 6912a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6912b sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 6912f mov r15,rsi
            r15 = rsi;
            // 69132 mov r14,rdi
            r14 = rdi;
            // 69135 cmp QWORD PTR [r15+0x1d8],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x1d8)) as u64, 0x0u64).1;
            // 6913d jne 0x691b5
            if fl.ne() { pc = 0x691b5; continue; }
            pc = 0x6913f;
        }
        0x6913f => {
            // block 6913f
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6913f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6913f mov r13,QWORD PTR [r15+0x198]
            r13 = m.r64(r15.wrapping_add(0x198)) as u64;
            // 69146 lea rax,[r15+0x1a0]
            rax = r15.wrapping_add(0x1a0);
            // 6914d mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 69151 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 69154 je 0x691b5
            if fl.e() { pc = 0x691b5; continue; }
            pc = 0x69156;
        }
        0x69156 => {
            // block 69156
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69156 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69156 lea r12,[r15+0x1c8]
            r12 = r15.wrapping_add(0x1c8);
            // 6915d lea rax,[r15+0x1d0]
            rax = r15.wrapping_add(0x1d0);
            // 69164 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            pc = 0x69168;
        }
        0x69168 => {
            // block 69168
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69168 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69168 lea rbx,[r13+0x20]
            rbx = r13.wrapping_add(0x20);
            // 6916c mov rdi,r12
            rdi = r12;
            // 6916f mov rsi,rbx
            rsi = rbx;
            // 69172 call 0xebcba
            { let r = { let t = m.env.stub_va(0x487); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69177 cmp rax,QWORD PTR [rbp-0x38]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64).1;
            // 6917b jne 0x69188
            if fl.ne() { pc = 0x69188; continue; }
            pc = 0x6917d;
        }
        0x6917d => {
            // block 6917d
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6917d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6917d mov rdi,r12
            rdi = r12;
            // 69180 mov rsi,rbx
            rsi = rbx;
            // 69183 call 0xebca8
            { let r = { let t = m.env.stub_va(0x475); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x69188;
        }
        0x69188 => {
            // block 69188
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69188 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69188 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 6918c test rax,rax
            fl = logic::<64>(rax & rax);
            // 6918f je 0x6919f
            if fl.e() { pc = 0x6919f; continue; }
            pc = 0x69191;
        }
        0x69191 => {
            // block 69191
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69191 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69191 mov r13,rax
            r13 = rax;
            // 69194 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 69198 test rax,rax
            fl = logic::<64>(rax & rax);
            // 6919b jne 0x69191
            if fl.ne() { pc = 0x69191; continue; }
            pc = 0x6919d;
        }
        0x6919d => {
            // block 6919d
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6919d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6919d jmp 0x691af
            pc = 0x691af; continue;
        }
        0x6919f => {
            // block 6919f
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6919f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6919f mov rax,r13
            rax = r13;
            pc = 0x691a2;
        }
        0x691a2 => {
            // block 691a2
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691a2 mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // 691a6 cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // 691aa mov rax,r13
            rax = r13;
            // 691ad jne 0x691a2
            if fl.ne() { pc = 0x691a2; continue; }
            pc = 0x691af;
        }
        0x691af => {
            // block 691af
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691af cmp r13,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r13, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 691b3 jne 0x69168
            if fl.ne() { pc = 0x69168; continue; }
            pc = 0x691b5;
        }
        0x691b5 => {
            // block 691b5
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691b5 lea r12,[r14+0x8]
            r12 = r14.wrapping_add(0x8);
            // 691b9 mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 691c1 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 691c9 mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // 691d0 mov QWORD PTR [r14],r12
            m.w64(r14, (r12) as u64);
            // 691d3 mov rbx,QWORD PTR [r15+0x1c8]
            rbx = m.r64(r15.wrapping_add(0x1c8)) as u64;
            // 691da add r15,0x1d0
            { let (r, f) = add::<64>(r15, 0x1d0u64);
              r15 = r;
              fl = f; }
            // 691e1 jmp 0x69217
            pc = 0x69217; continue;
        }
        0x691e3 => {
            // block 691e3
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691e3 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // 691e7 mov rdi,r14
            rdi = r14;
            // 691ea mov rsi,r12
            rsi = r12;
            // 691ed call 0xebca2
            { let r = { let t = m.env.stub_va(0x46f); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 691f2 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 691f6 test rax,rax
            fl = logic::<64>(rax & rax);
            // 691f9 je 0x69208
            if fl.e() { pc = 0x69208; continue; }
            pc = 0x691fb;
        }
        0x691fb => {
            // block 691fb
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691fb mov rbx,rax
            rbx = rax;
            // 691fe mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 69201 test rax,rax
            fl = logic::<64>(rax & rax);
            // 69204 jne 0x691fb
            if fl.ne() { pc = 0x691fb; continue; }
            pc = 0x69206;
        }
        0x69206 => {
            // block 69206
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69206 jmp 0x69217
            pc = 0x69217; continue;
        }
        0x69208 => {
            // block 69208
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69208 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69208 mov rax,rbx
            rax = rbx;
            pc = 0x6920b;
        }
        0x6920b => {
            // block 6920b
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6920b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6920b mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 6920f cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 69212 mov rax,rbx
            rax = rbx;
            // 69215 jne 0x6920b
            if fl.ne() { pc = 0x6920b; continue; }
            pc = 0x69217;
        }
        0x69217 => {
            // block 69217
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69217 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69217 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 6921a jne 0x691e3
            if fl.ne() { pc = 0x691e3; continue; }
            pc = 0x6921c;
        }
        0x6921c => {
            // block 6921c
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6921c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6921c mov rax,r14
            rax = r14;
            // 6921f add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 69223 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 69224 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 69226 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 69228 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6922a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6922c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6922d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6922e => {
            // block 6922e
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6922e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6922e mov rbx,rax
            rbx = rax;
            // 69231 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 69235 mov rdi,r14
            rdi = r14;
            // 69238 call 0xebcc0
            { let r = { let t = m.env.stub_va(0x48d); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6923d mov rdi,rbx
            rdi = rbx;
            // 69240 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x72d00 size=292
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00072d00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x72d00;
    loop {
        match pc {
        0x72d00 => {
            // block 72d00
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 72d01 mov rbp,rsp
            rbp = rsp;
            // 72d04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 72d06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 72d08 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 72d0a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 72d0c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 72d0d sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 72d11 mov rbx,rdx
            rbx = rdx;
            // 72d14 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 72d17 mov r15,rdi
            r15 = rdi;
            // 72d1a mov DWORD PTR [rbp-0x2c],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r14 as u32 as u64)) as u32);
            // 72d1e movzx eax,BYTE PTR [rbx]
            rax = (m.r8(rbx) as u64) as u32 as u64;
            // 72d21 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 72d23 jne 0x72d2a
            if fl.ne() { pc = 0x72d2a; continue; }
            pc = 0x72d25;
        }
        0x72d25 => {
            // block 72d25
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d25 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 72d28 jmp 0x72d2e
            pc = 0x72d2e; continue;
        }
        0x72d2a => {
            // block 72d2a
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d2a mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0x72d2e;
        }
        0x72d2e => {
            // block 72d2e
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d2e test rax,rax
            fl = logic::<64>(rax & rax);
            // 72d31 je 0x72dec
            if fl.e() { pc = 0x72dec; continue; }
            pc = 0x72d37;
        }
        0x72d37 => {
            // block 72d37
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d37 lea r12,[r15+0x168]
            r12 = r15.wrapping_add(0x168);
            // 72d3e lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 72d42 mov rdi,r12
            rdi = r12;
            // 72d45 call 0xebc66
            { let r = { let t = m.env.stub_va(0x433); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 72d4a lea rcx,[r15+0x170]
            rcx = r15.wrapping_add(0x170);
            // 72d51 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 72d54 jne 0x72dec
            if fl.ne() { pc = 0x72dec; continue; }
            pc = 0x72d5a;
        }
        0x72d5a => {
            // block 72d5a
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d5a mov DWORD PTR [rbp-0x50],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), ((r14 as u32 as u64)) as u32);
            // 72d5e lea r13,[rbp-0x48]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 72d62 mov rdi,r13
            rdi = r13;
            // 72d65 mov rsi,rbx
            rsi = rbx;
            // 72d68 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 72d6d lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 72d71 mov rdi,r12
            rdi = r12;
            // 72d74 call 0xebc54
            { let r = { let t = m.env.stub_va(0x421); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 72d79 mov rdi,r13
            rdi = r13;
            // 72d7c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 72d81 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 72d88 mov rsi,rbx
            rsi = rbx;
            // 72d8b call 0xb5094
            { let r = { let t = m.env.code_va(0xb5094); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 72d90 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 72d94 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 72d98 mov rax,QWORD PTR [rbp-0x90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 72d9f mov rcx,QWORD PTR [rbp-0x88]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 72da6 mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 72daa mov QWORD PTR [rbp-0x70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rax) as u64);
            // 72dae xorps xmm0,xmm0
            x0 = 0;
            // 72db1 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 72db8 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // 72dc0 mov DWORD PTR [rbp-0x58],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((r14 as u32 as u64)) as u32);
            // 72dc4 add r15,0x1b0
            { let (r, f) = add::<64>(r15, 0x1b0u64);
              r15 = r;
              fl = f; }
            // 72dcb lea rsi,[rbp-0x70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 72dcf mov rdi,r15
            rdi = r15;
            // 72dd2 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 72dd7 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 72ddb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 72de0 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 72de7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x72dec;
        }
        0x72dec => {
            // block 72dec
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72dec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72dec add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 72df0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 72df1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 72df3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 72df5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 72df7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 72df9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 72dfa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x72dfb => {
            // block 72dfb
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72dfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72dfb mov rbx,rax
            rbx = rax;
            // 72dfe mov rdi,r13
            rdi = r13;
            // 72e01 jmp 0x72e16
            pc = 0x72e16; continue;
        }
        0x72e03 => {
            // block 72e03
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72e03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72e03 mov rbx,rax
            rbx = rax;
            // 72e06 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 72e0a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 72e0f lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            pc = 0x72e16;
        }
        0x72e16 => {
            // block 72e16
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72e16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72e16 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 72e1b mov rdi,rbx
            rdi = rbx;
            // 72e1e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8d6b size=270
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8d6b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb8d6b;
    loop {
        match pc {
        0xb8d6b => {
            // block b8d6b
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8d6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8d6b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8d6c mov rbp,rsp
            rbp = rsp;
            // b8d6f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b8d71 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8d73 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b8d75 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b8d77 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8d78 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b8d7c mov r12,rsi
            r12 = rsi;
            // b8d7f mov r15,rdi
            r15 = rdi;
            // b8d82 cmp QWORD PTR [r12+0x10],0x0
            fl = sub::<64>(m.r64(r12.wrapping_add(0x10)) as u64, 0x0u64).1;
            // b8d88 je 0xb8e3f
            if fl.e() { pc = 0xb8e3f; continue; }
            pc = 0xb8d8e;
        }
        0xb8d8e => {
            // block b8d8e
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8d8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8d8e mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8d92 lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // b8d97 cmp rcx,r13
            fl = sub::<64>(rcx, r13).1;
            // b8d9a je 0xb8e53
            if fl.e() { pc = 0xb8e53; continue; }
            pc = 0xb8da0;
        }
        0xb8da0 => {
            // block b8da0
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8da0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8da0 mov r14,rcx
            r14 = rcx;
            // b8da3 jmp 0xb8dac
            pc = 0xb8dac; continue;
        }
        0xb8da5 => {
            // block b8da5
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8da5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8da5 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8da9 mov r14,rax
            r14 = rax;
            pc = 0xb8dac;
        }
        0xb8dac => {
            // block b8dac
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8dac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8dac cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // b8daf setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // b8db2 lea rsi,[rip+0x7ff0f] # 0x138cc8
            rsi = m.env.data_va(316565);
            // b8db9 lea rcx,[rip+0x716dd] # 0x12a49d
            rcx = m.env.data_va(257130);
            // b8dc0 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b8dc4 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // b8dc7 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // b8dca mov rdi,r15
            rdi = r15;
            // b8dcd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8dd2 mov rbx,rax
            rbx = rax;
            // b8dd5 mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b8dd9 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b8dde lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8de2 call 0x60c44
            { let r = { let t = m.env.code_va(0x60c44); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8de7 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // b8deb test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b8dee jne 0xb8df9
            if fl.ne() { pc = 0xb8df9; continue; }
            pc = 0xb8df0;
        }
        0xb8df0 => {
            // block b8df0
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8df0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8df0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b8df3 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // b8df7 jmp 0xb8e01
            pc = 0xb8e01; continue;
        }
        0xb8df9 => {
            // block b8df9
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8df9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8df9 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b8dfd mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xb8e01;
        }
        0xb8e01 => {
            // block b8e01
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e01 mov rdi,rbx
            rdi = rbx;
            // b8e04 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8e09 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8e0d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8e12 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // b8e16 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b8e19 je 0xb8e28
            if fl.e() { pc = 0xb8e28; continue; }
            pc = 0xb8e1b;
        }
        0xb8e1b => {
            // block b8e1b
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e1b mov rax,rcx
            rax = rcx;
            // b8e1e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b8e21 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b8e24 jne 0xb8e1b
            if fl.ne() { pc = 0xb8e1b; continue; }
            pc = 0xb8e26;
        }
        0xb8e26 => {
            // block b8e26
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e26 jmp 0xb8e34
            pc = 0xb8e34; continue;
        }
        0xb8e28 => {
            // block b8e28
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e28 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // b8e2c cmp QWORD PTR [rax],r14
            fl = sub::<64>(m.r64(rax) as u64, r14).1;
            // b8e2f mov r14,rax
            r14 = rax;
            // b8e32 jne 0xb8e28
            if fl.ne() { pc = 0xb8e28; continue; }
            pc = 0xb8e34;
        }
        0xb8e34 => {
            // block b8e34
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e34 cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // b8e37 jne 0xb8da5
            if fl.ne() { pc = 0xb8da5; continue; }
            pc = 0xb8e3d;
        }
        0xb8e3d => {
            // block b8e3d
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e3d jmp 0xb8e53
            pc = 0xb8e53; continue;
        }
        0xb8e3f => {
            // block b8e3f
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e3f lea rsi,[rip+0x7efb7] # 0x137dfd
            rsi = m.env.data_va(312778);
            // b8e46 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // b8e4b mov rdi,r15
            rdi = r15;
            // b8e4e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb8e53;
        }
        0xb8e53 => {
            // block b8e53
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e53 mov rax,r15
            rax = r15;
            // b8e56 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b8e5a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8e5b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b8e5d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b8e5f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8e61 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b8e63 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8e64 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb8e65 => {
            // block b8e65
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e65 mov rbx,rax
            rbx = rax;
            // b8e68 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8e6c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8e71 mov rdi,rbx
            rdi = rbx;
            // b8e74 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9113 size=283
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9113<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9113;
    loop {
        match pc {
        0xb9113 => {
            // block b9113
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9113 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9113 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9114 mov rbp,rsp
            rbp = rsp;
            // b9117 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b9119 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b911b push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b911d push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b911f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9120 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b9121 mov r14,rsi
            r14 = rsi;
            // b9124 mov r13,rdi
            r13 = rdi;
            // b9127 mov rbx,QWORD PTR [r14]
            rbx = m.r64(r14) as u64;
            // b912a add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // b912e cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // b9131 je 0xb921c
            if fl.e() { pc = 0xb921c; continue; }
            pc = 0xb9137;
        }
        0xb9137 => {
            // block b9137
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9137 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9137 lea r15,[rip+0x71337] # 0x12a475
            r15 = m.env.data_va(257090);
            // b913e jmp 0xb9157
            pc = 0xb9157; continue;
        }
        0xb9140 => {
            // block b9140
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9140 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // b9145 mov rdi,r13
            rdi = r13;
            // b9148 lea rsi,[rip+0x7134e] # 0x12a49d
            rsi = m.env.data_va(257130);
            // b914f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9154 mov rbx,r12
            rbx = r12;
            pc = 0xb9157;
        }
        0xb9157 => {
            // block b9157
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9157 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9157 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // b915c mov rdi,r13
            rdi = r13;
            // b915f mov rsi,r15
            rsi = r15;
            // b9162 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9167 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b916a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b916e mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b9172 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b9175 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b9178 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b917c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b917f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b9183 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b918b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b918e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b9192 mov QWORD PTR [rcx+rax*1+0x18],0x2
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x2u64) as u64);
            // b919b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b919e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b91a2 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b91ad movzx esi,BYTE PTR [rbx+0x19]
            rsi = (m.r8(rbx.wrapping_add(0x19)) as u64) as u32 as u64;
            // b91b1 mov rdi,rax
            rdi = rax;
            // b91b4 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b91b9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b91bc mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b91c0 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b91c4 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b91c7 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b91ca mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b91ce mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b91d1 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b91d5 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // b91e0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b91e3 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b91e7 and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b91ef mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b91f3 test rax,rax
            fl = logic::<64>(rax & rax);
            // b91f6 je 0xb9206
            if fl.e() { pc = 0xb9206; continue; }
            pc = 0xb91f8;
        }
        0xb91f8 => {
            // block b91f8
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b91f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b91f8 mov r12,rax
            r12 = rax;
            // b91fb mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b91ff test rax,rax
            fl = logic::<64>(rax & rax);
            // b9202 jne 0xb91f8
            if fl.ne() { pc = 0xb91f8; continue; }
            pc = 0xb9204;
        }
        0xb9204 => {
            // block b9204
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9204 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9204 jmp 0xb9213
            pc = 0xb9213; continue;
        }
        0xb9206 => {
            // block b9206
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9206 mov r12,QWORD PTR [rbx+0x10]
            r12 = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b920a cmp QWORD PTR [r12],rbx
            fl = sub::<64>(m.r64(r12) as u64, rbx).1;
            // b920e mov rbx,r12
            rbx = r12;
            // b9211 jne 0xb9206
            if fl.ne() { pc = 0xb9206; continue; }
            pc = 0xb9213;
        }
        0xb9213 => {
            // block b9213
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9213 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9213 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // b9216 jne 0xb9140
            if fl.ne() { pc = 0xb9140; continue; }
            pc = 0xb921c;
        }
        0xb921c => {
            // block b921c
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b921c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b921c mov rax,r13
            rax = r13;
            // b921f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b9223 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9224 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b9226 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b9228 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b922a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b922c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b922d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9356 size=244
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9356<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9356;
    loop {
        match pc {
        0xb9356 => {
            // block b9356
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9356 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9356 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9357 mov rbp,rsp
            rbp = rsp;
            // b935a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b935c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b935e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b9360 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b9362 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9363 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b9367 mov r13,rdx
            r13 = rdx;
            // b936a mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // b936d mov r14,rdi
            r14 = rdi;
            // b9370 lea rbx,[r13+0x8]
            rbx = r13.wrapping_add(0x8);
            // b9374 mov rsi,QWORD PTR [r13+0x8]
            rsi = m.r64(r13.wrapping_add(0x8)) as u64;
            // b9378 mov rdi,r13
            rdi = r13;
            // b937b call 0xebcf0
            { let r = { let t = m.env.stub_va(0x4bd); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9380 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // b9388 mov QWORD PTR [r13+0x0],rbx
            m.w64(r13, (rbx) as u64);
            // b938c mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // b9394 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9398 call 0xcbc54
            { let r = { let t = m.env.code_va(0xcbc54); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b939d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b939f je 0xb9438
            if fl.e() { pc = 0xb9438; continue; }
            pc = 0xb93a5;
        }
        0xb93a5 => {
            // block b93a5
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93a5 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b93a9 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b93ae test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b93b0 je 0xb9438
            if fl.e() { pc = 0xb9438; continue; }
            pc = 0xb93b6;
        }
        0xb93b6 => {
            // block b93b6
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93b6 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b93ba call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b93bf xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // b93c2 movzx ecx,r12w
            rcx = ((r12 as u16 as u64)) as u32 as u64;
            // b93c6 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b93c9 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b93cb mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // b93d0 jbe 0xb943a
            if fl.be() { pc = 0xb943a; continue; }
            pc = 0xb93d2;
        }
        0xb93d2 => {
            // block b93d2
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93d2 lea rax,[rip+0x495d7] # 0x1029b0
            rax = m.env.data_va(94589);
            // b93d9 mov eax,DWORD PTR [rax+rcx*4]
            rax = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b93dc mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b93e0 lea r12,[rbp-0x2d]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd3);
            pc = 0xb93e4;
        }
        0xb93e4 => {
            // block b93e4
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93e4 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // b93eb mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b93ee mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b93f2 lea esi,[r15+rcx*1]
            rsi = (r15.wrapping_add(rcx)) as u32 as u64;
            // b93f6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b93f8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b93fd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9400 mov rdi,r14
            rdi = r14;
            // b9403 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // b9407 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb940a;
        }
        0xb940a => {
            // block b940a
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b940a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b940a mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // b940d mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // b940f shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // b9411 mov BYTE PTR [rbp-0x2d],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3), ((rax as u8 as u64)) as u8);
            // b9414 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b9416 je 0xb9423
            if fl.e() { pc = 0xb9423; continue; }
            pc = 0xb9418;
        }
        0xb9418 => {
            // block b9418
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9418 mov rdi,r13
            rdi = r13;
            // b941b mov rsi,r12
            rsi = r12;
            // b941e call 0xebce4
            { let r = { let t = m.env.stub_va(0x4b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb9423;
        }
        0xb9423 => {
            // block b9423
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9423 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9423 add ebx,0x8
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x8u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b9426 cmp ebx,0x20
            fl = sub::<32>((rbx as u32 as u64), 0x20u64).1;
            // b9429 jne 0xb940a
            if fl.ne() { pc = 0xb940a; continue; }
            pc = 0xb942b;
        }
        0xb942b => {
            // block b942b
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b942b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b942b inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // b942e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // b9430 cmp r15d,0x5
            fl = sub::<32>((r15 as u32 as u64), 0x5u64).1;
            // b9434 jne 0xb93e4
            if fl.ne() { pc = 0xb93e4; continue; }
            pc = 0xb9436;
        }
        0xb9436 => {
            // block b9436
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9436 jmp 0xb943a
            pc = 0xb943a; continue;
        }
        0xb9438 => {
            // block b9438
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9438 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb943a;
        }
        0xb943a => {
            // block b943a
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b943a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b943a add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b943e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b943f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b9441 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b9443 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9445 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9447 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9448 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb944a size=242
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b944a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb944a;
    loop {
        match pc {
        0xb944a => {
            // block b944a
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b944a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b944a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b944b mov rbp,rsp
            rbp = rsp;
            // b944e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b9450 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b9452 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b9454 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b9456 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9457 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b9458 mov r13,rdx
            r13 = rdx;
            // b945b mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // b945e mov r14,rdi
            r14 = rdi;
            // b9461 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9465 call 0xcbc54
            { let r = { let t = m.env.code_va(0xcbc54); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b946a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b946c je 0xb952a
            if fl.e() { pc = 0xb952a; continue; }
            pc = 0xb9472;
        }
        0xb9472 => {
            // block b9472
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9472 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9472 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9476 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b947b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b947d je 0xb952a
            if fl.e() { pc = 0xb952a; continue; }
            pc = 0xb9483;
        }
        0xb9483 => {
            // block b9483
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9483 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9487 call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b948c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // b948f movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // b9493 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b9496 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b9498 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // b949d jbe 0xb952c
            if fl.be() { pc = 0xb952c; continue; }
            pc = 0xb94a3;
        }
        0xb94a3 => {
            // block b94a3
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94a3 lea rax,[rip+0x49506] # 0x1029b0
            rax = m.env.data_va(94589);
            // b94aa mov r15d,DWORD PTR [rax+rcx*4]
            r15 = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b94ae mov rbx,QWORD PTR [r13+0x0]
            rbx = m.r64(r13) as u64;
            // b94b2 add r13,0x8
            { let (r, f) = add::<64>(r13, 0x8u64);
              r13 = r;
              fl = f; }
            pc = 0xb94b6;
        }
        0xb94b6 => {
            // block b94b6
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94b6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b94b8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xb94ba;
        }
        0xb94ba => {
            // block b94ba
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94ba cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // b94bd je 0xb94d5
            if fl.e() { pc = 0xb94d5; continue; }
            pc = 0xb94bf;
        }
        0xb94bf => {
            // block b94bf
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94bf mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b94c3 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b94c6 je 0xb94dc
            if fl.e() { pc = 0xb94dc; continue; }
            pc = 0xb94c8;
        }
        0xb94c8 => {
            // block b94c8
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94c8 mov rcx,rsi
            rcx = rsi;
            // b94cb mov rsi,QWORD PTR [rcx]
            rsi = m.r64(rcx) as u64;
            // b94ce test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b94d1 jne 0xb94c8
            if fl.ne() { pc = 0xb94c8; continue; }
            pc = 0xb94d3;
        }
        0xb94d3 => {
            // block b94d3
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94d3 jmp 0xb94eb
            pc = 0xb94eb; continue;
        }
        0xb94d5 => {
            // block b94d5
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94d5 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b94d7 mov rbx,r13
            rbx = r13;
            // b94da jmp 0xb94f2
            pc = 0xb94f2; continue;
        }
        0xb94dc => {
            // block b94dc
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94dc mov rsi,rbx
            rsi = rbx;
            pc = 0xb94df;
        }
        0xb94df => {
            // block b94df
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94df mov rcx,QWORD PTR [rsi+0x10]
            rcx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // b94e3 cmp QWORD PTR [rcx],rsi
            fl = sub::<64>(m.r64(rcx) as u64, rsi).1;
            // b94e6 mov rsi,rcx
            rsi = rcx;
            // b94e9 jne 0xb94df
            if fl.ne() { pc = 0xb94df; continue; }
            pc = 0xb94eb;
        }
        0xb94eb => {
            // block b94eb
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94eb mov sil,BYTE PTR [rbx+0x19]
            rsi = (rsi & !0xff) | ((m.r8(rbx.wrapping_add(0x19)) as u64) as u8 as u64);
            // b94ef mov rbx,rcx
            rbx = rcx;
            pc = 0xb94f2;
        }
        0xb94f2 => {
            // block b94f2
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94f2 movzx esi,sil
            rsi = ((rsi as u8 as u64)) as u32 as u64;
            // b94f6 lea ecx,[rax*8+0x0]
            rcx = (rax.wrapping_mul(8)) as u32 as u64;
            // b94fd shl esi,cl
            { let (r, f) = shl::<32>((rsi as u32 as u64), (rcx as u8 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // b94ff or edx,esi
            { let r = (rdx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b9501 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // b9503 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // b9506 jne 0xb94ba
            if fl.ne() { pc = 0xb94ba; continue; }
            pc = 0xb9508;
        }
        0xb9508 => {
            // block b9508
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9508 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9508 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b950b lea esi,[r12+r15*1]
            rsi = (r12.wrapping_add(r15)) as u32 as u64;
            // b950f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b9514 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9517 mov rdi,r14
            rdi = r14;
            // b951a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b951d inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // b9520 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // b9522 cmp r12d,0x5
            fl = sub::<32>((r12 as u32 as u64), 0x5u64).1;
            // b9526 jne 0xb94b6
            if fl.ne() { pc = 0xb94b6; continue; }
            pc = 0xb9528;
        }
        0xb9528 => {
            // block b9528
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9528 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9528 jmp 0xb952c
            pc = 0xb952c; continue;
        }
        0xb952a => {
            // block b952a
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b952a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b952a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb952c;
        }
        0xb952c => {
            // block b952c
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b952c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b952c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b9530 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9531 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b9533 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b9535 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9537 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9539 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b953a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8598 size=273
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8598<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc8598;
    loop {
        match pc {
        0xc8598 => {
            // block c8598
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c8598 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8598 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8599 mov rbp,rsp
            rbp = rsp;
            // c859c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c859e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c85a0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c85a1 mov ax,WORD PTR [rdi]
            rax = (rax & !0xffff) | ((m.r16(rdi) as u64) as u16 as u64);
            // c85a4 mov WORD PTR [rdi-0x2],ax
            m.w16(rdi.wrapping_add(0xffff_ffff_ffff_fffe), ((rax as u16 as u64)) as u16);
            // c85a8 lea eax,[r8-0x1]
            rax = (r8.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // c85ac cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // c85ae mov ax,WORD PTR [rdi+rax*2]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // c85b2 lea ebx,[r8+0x1]
            rbx = (r8.wrapping_add(0x1)) as u32 as u64;
            // c85b6 movsxd rbx,ebx
            rbx = sx::<32>((rbx as u32 as u64)) as u64;
            // c85b9 mov WORD PTR [rdi+rbx*2],ax
            m.w16(rdi.wrapping_add(rbx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // c85bd movsxd rbx,r8d
            rbx = sx::<32>((r8 as u32 as u64)) as u64;
            // c85c0 mov WORD PTR [rdi+rbx*2],ax
            m.w16(rdi.wrapping_add(rbx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // c85c4 imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // c85c8 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // c85ca cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c85cb idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // c85cd mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // c85d0 imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c85d4 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c85d6 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c85d7 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // c85d9 mov r10d,eax
            r10 = ((rax as u32 as u64)) as u32 as u64;
            // c85dc cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // c85df jge 0xc86a2
            if fl.ge() { pc = 0xc86a2; continue; }
            pc = 0xc85e5;
        }
        0xc85e5 => {
            // block c85e5
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c85e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c85e5 shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // c85e9 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // c85ec cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c85ed idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // c85f0 sub r10d,r11d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // c85f3 movsxd rcx,r11d
            rcx = sx::<32>((r11 as u32 as u64)) as u64;
            // c85f6 lea r14,[rsi+rcx*2]
            r14 = rsi.wrapping_add(rcx.wrapping_mul(2));
            // c85fa imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            // c85fe lea r9,[rip+0x80dbb] # 0x1493c0
            r9 = m.env.data_va(383885);
            // c8605 mov r8w,0x4
            r8 = (r8 & !0xffff) | ((0x4u64) as u16 as u64);
            pc = 0xc860a;
        }
        0xc860a => {
            // block c860a
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c860a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c860a mov edx,r11d
            rdx = ((r11 as u32 as u64)) as u32 as u64;
            // c860d sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // c8610 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // c8613 shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c8616 and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c8619 movsxd r15,edx
            r15 = sx::<32>((rdx as u32 as u64)) as u64;
            // c861c movsx esi,WORD PTR [rdi+r15*2-0x2]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // c8622 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // c8627 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8629 imul esi,DWORD PTR [r9+rcx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rcx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // c862e movsx ecx,WORD PTR [rdi+r15*2]
            rcx = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // c8633 mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // c8638 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c863a imul ecx,DWORD PTR [r9+rdx*4]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(r9.wrapping_add(rdx.wrapping_mul(4))) as u64); rcx = (r) as u32 as u64; fl = f; }
            // c863f add ecx,esi
            { let (r, f) = add::<32>((rcx as u32 as u64), (rsi as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8641 movsx edx,WORD PTR [rdi+r15*2+0x2]
            rdx = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // c8647 mov esi,0x60
            rsi = (0x60u64) as u32 as u64;
            // c864c sub esi,ebx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // c864e imul edx,DWORD PTR [r9+rsi*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r9.wrapping_add(rsi.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // c8653 add edx,ecx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8655 movsx esi,WORD PTR [rdi+r15*2+0x4]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // c865b mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // c8660 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8662 imul esi,DWORD PTR [r9+rcx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rcx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // c8667 add esi,edx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rdx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // c8669 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // c866b and edx,0xffff0000
            { let r = (rdx as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8671 mov bx,0x3fb
            rbx = (rbx & !0xffff) | ((0x3fbu64) as u16 as u64);
            // c8675 cmp edx,0x3fb0000
            fl = sub::<32>((rdx as u32 as u64), 0x3fb0000u64).1;
            // c867b jg 0xc868e
            if fl.g() { pc = 0xc868e; continue; }
            pc = 0xc867d;
        }
        0xc867d => {
            // block c867d
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c867d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c867d sar esi,0x10
            { let (r, f) = sar::<32>((rsi as u32 as u64), 0x10u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // c8680 cmp edx,0x40000
            fl = sub::<32>((rdx as u32 as u64), 0x40000u64).1;
            // c8686 cmovl si,r8w
            if fl.l() { rsi = (rsi & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            // c868b mov bx,si
            rbx = (rbx & !0xffff) | (((rsi as u16 as u64)) as u16 as u64);
            pc = 0xc868e;
        }
        0xc868e => {
            // block c868e
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c868e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c868e mov WORD PTR [r14],bx
            m.w16(r14, ((rbx as u16 as u64)) as u16);
            // c8692 add r14,0x2
            { let (r, f) = add::<64>(r14, 0x2u64);
              r14 = r;
              fl = f; }
            // c8696 add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c8699 dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // c869c jne 0xc860a
            if fl.ne() { pc = 0xc860a; continue; }
            pc = 0xc86a2;
        }
        0xc86a2 => {
            // block c86a2
            #[cfg(feature = "trace")] eprintln!("f_000c8598 pc c86a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c86a2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c86a3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c86a5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c86a7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c86a8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdfc90 size=292
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000dfc90<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdfc90;
    loop {
        match pc {
        0xdfc90 => {
            // block dfc90
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfc90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfc90 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dfc91 mov rbp,rsp
            rbp = rsp;
            // dfc94 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dfc96 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dfc98 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dfc9a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dfc9c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dfc9d sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // dfca1 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // dfca5 mov r12,rdi
            r12 = rdi;
            // dfca8 mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // dfcac mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // dfcb1 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // dfcb4 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // dfcb8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // dfcbb mov rax,rbx
            rax = rbx;
            // dfcbe shr rax,0x3d
            { let (r, f) = shr::<64>(rax, 0x3du64, fl); rax = r; fl = f; }
            // dfcc2 je 0xdfcd0
            if fl.e() { pc = 0xdfcd0; continue; }
            pc = 0xdfcc4;
        }
        0xdfcc4 => {
            // block dfcc4
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfcc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfcc4 mov rdi,r12
            rdi = r12;
            // dfcc7 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0xdfcd0 => {
            // block dfcd0
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfcd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfcd0 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // dfcd5 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // dfcda mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // dfcde sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // dfce1 mov rcx,r15
            rcx = r15;
            // dfce4 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // dfce8 movabs rdx,0xffffffffffffffe
            rdx = 0xfff_ffff_ffff_fffeu64;
            // dfcf2 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // dfcf5 ja 0xdfd04
            if fl.a() { pc = 0xdfd04; continue; }
            pc = 0xdfcf7;
        }
        0xdfcf7 => {
            // block dfcf7
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfcf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfcf7 sar r15,0x2
            { let (r, f) = sar::<64>(r15, 0x2u64, fl); r15 = r; fl = f; }
            // dfcfb cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // dfcfe cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // dfd02 jmp 0xdfd0e
            pc = 0xdfd0e; continue;
        }
        0xdfd04 => {
            // block dfd04
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfd04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfd04 movabs r15,0x1fffffffffffffff
            r15 = 0x1fff_ffff_ffff_ffffu64;
            pc = 0xdfd0e;
        }
        0xdfd0e => {
            // block dfd0e
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfd0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfd0e mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // dfd13 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // dfd17 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // dfd1a mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // dfd1e mov r13,r14
            r13 = r14;
            // dfd21 sar r13,0x3
            { let (r, f) = sar::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // dfd25 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // dfd2d mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // dfd31 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // dfd33 test r15,r15
            fl = logic::<64>(r15 & r15);
            // dfd36 je 0xdfd48
            if fl.e() { pc = 0xdfd48; continue; }
            pc = 0xdfd38;
        }
        0xdfd38 => {
            // block dfd38
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfd38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfd38 lea rdi,[r15*8+0x0]
            rdi = r15.wrapping_mul(8);
            // dfd40 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfd45 mov rbx,rax
            rbx = rax;
            pc = 0xdfd48;
        }
        0xdfd48 => {
            // block dfd48
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfd48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfd48 lea rax,[rbx+r15*8]
            rax = rbx.wrapping_add(r15.wrapping_mul(8));
            // dfd4c mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // dfd50 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // dfd54 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dfd57 mov QWORD PTR [rbx+r13*8],rax
            m.w64(rbx.wrapping_add(r13.wrapping_mul(8)), (rax) as u64);
            // dfd5b lea r13,[rbx+r13*8+0x8]
            r13 = rbx.wrapping_add(r13.wrapping_mul(8)).wrapping_add(0x8);
            // dfd60 mov rdi,rbx
            rdi = rbx;
            // dfd63 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // dfd67 mov rsi,r15
            rsi = r15;
            // dfd6a mov rdx,r14
            rdx = r14;
            // dfd6d call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfd72 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // dfd76 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // dfd7a mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // dfd7f mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // dfd83 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // dfd87 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // dfd8b mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // dfd90 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // dfd94 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // dfd98 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // dfd9c lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // dfda0 call 0xebedc
            { let r = { let t = m.env.stub_va(0x6a9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfda5 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // dfda9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dfdaa pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dfdac pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dfdae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dfdb0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dfdb2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dfdb3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdff97 size=266
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dff97<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdff97;
    loop {
        match pc {
        0xdff97 => {
            // block dff97
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc dff97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dff97 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dff98 mov rbp,rsp
            rbp = rsp;
            // dff9b push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dff9d push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dff9f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dffa0 mov ax,WORD PTR [rdi]
            rax = (rax & !0xffff) | ((m.r16(rdi) as u64) as u16 as u64);
            // dffa3 mov WORD PTR [rdi-0x2],ax
            m.w16(rdi.wrapping_add(0xffff_ffff_ffff_fffe), ((rax as u16 as u64)) as u16);
            // dffa7 lea eax,[r8-0x1]
            rax = (r8.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // dffab cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // dffad mov r10w,WORD PTR [rdi+rax*2]
            r10 = (r10 & !0xffff) | ((m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // dffb2 lea eax,[r8+0x1]
            rax = (r8.wrapping_add(0x1)) as u32 as u64;
            // dffb6 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // dffb8 mov WORD PTR [rdi+rax*2],r10w
            m.w16(rdi.wrapping_add(rax.wrapping_mul(2)), ((r10 as u16 as u64)) as u16);
            // dffbd movsxd rbx,r8d
            rbx = sx::<32>((r8 as u32 as u64)) as u64;
            // dffc0 mov WORD PTR [rdi+rbx*2],r10w
            m.w16(rdi.wrapping_add(rbx.wrapping_mul(2)), ((r10 as u16 as u64)) as u16);
            // dffc5 imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dffc9 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // dffcb cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dffcc idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // dffce mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // dffd1 imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // dffd5 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // dffd7 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dffd8 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // dffda mov r10d,eax
            r10 = ((rax as u32 as u64)) as u32 as u64;
            // dffdd cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // dffe0 jge 0xe009a
            if fl.ge() { pc = 0xe009a; continue; }
            pc = 0xdffe6;
        }
        0xdffe6 => {
            // block dffe6
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc dffe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dffe6 shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // dffea mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // dffed cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dffee idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // dfff1 sub r10d,r11d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // dfff4 movsxd rcx,r11d
            rcx = sx::<32>((r11 as u32 as u64)) as u64;
            // dfff7 lea r14,[rsi+rcx*2]
            r14 = rsi.wrapping_add(rcx.wrapping_mul(2));
            // dfffb imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            // dffff lea r9,[rip+0x4801a] # 0x128020
            r9 = m.env.data_va(247789);
            // e0006 mov r8w,0x4
            r8 = (r8 & !0xffff) | ((0x4u64) as u16 as u64);
            pc = 0xe000b;
        }
        0xe000b => {
            // block e000b
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc e000b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e000b mov edx,r11d
            rdx = ((r11 as u32 as u64)) as u32 as u64;
            // e000e sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e0011 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // e0014 shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0017 and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e001a movsxd r15,edx
            r15 = sx::<32>((rdx as u32 as u64)) as u64;
            // e001d movsx esi,WORD PTR [rdi+r15*2-0x2]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e0023 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // e0028 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e002a imul esi,DWORD PTR [r9+rcx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rcx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // e002f movsx ecx,WORD PTR [rdi+r15*2]
            rcx = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e0034 mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // e0039 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e003b imul ecx,DWORD PTR [r9+rdx*4]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(r9.wrapping_add(rdx.wrapping_mul(4))) as u64); rcx = (r) as u32 as u64; fl = f; }
            // e0040 add ecx,esi
            { let (r, f) = add::<32>((rcx as u32 as u64), (rsi as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0042 movsx edx,WORD PTR [rdi+r15*2+0x2]
            rdx = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e0048 mov esi,0x60
            rsi = (0x60u64) as u32 as u64;
            // e004d sub esi,ebx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e004f imul edx,DWORD PTR [r9+rsi*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r9.wrapping_add(rsi.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // e0054 add edx,ecx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0056 movsx esi,WORD PTR [rdi+r15*2+0x4]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r15.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e005c mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // e0061 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0063 imul esi,DWORD PTR [r9+rcx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rcx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // e0068 add esi,edx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rdx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e006a mov dx,0x3fb
            rdx = (rdx & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e006e cmp esi,0x3fbffff
            fl = sub::<32>((rsi as u32 as u64), 0x3fbffffu64).1;
            // e0074 ja 0xe0086
            if fl.a() { pc = 0xe0086; continue; }
            pc = 0xe0076;
        }
        0xe0076 => {
            // block e0076
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc e0076 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0076 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // e0078 shr edx,0x10
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e007b cmp esi,0x40000
            fl = sub::<32>((rsi as u32 as u64), 0x40000u64).1;
            // e0081 cmovb dx,r8w
            if fl.b() { rdx = (rdx & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            pc = 0xe0086;
        }
        0xe0086 => {
            // block e0086
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc e0086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0086 mov WORD PTR [r14],dx
            m.w16(r14, ((rdx as u16 as u64)) as u16);
            // e008a add r14,0x2
            { let (r, f) = add::<64>(r14, 0x2u64);
              r14 = r;
              fl = f; }
            // e008e add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // e0091 dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // e0094 jne 0xe000b
            if fl.ne() { pc = 0xe000b; continue; }
            pc = 0xe009a;
        }
        0xe009a => {
            // block e009a
            #[cfg(feature = "trace")] eprintln!("f_000dff97 pc e009a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e009a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e009b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e009d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e009f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e00a0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe196e size=280
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e196e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe196e;
    loop {
        match pc {
        0xe196e => {
            // block e196e
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e196e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e196e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e196f mov rbp,rsp
            rbp = rsp;
            // e1972 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e1974 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1976 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1977 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e197b mov r14,rcx
            r14 = rcx;
            // e197e mov r15,rsi
            r15 = rsi;
            // e1981 lea rax,[rip+0x492d3] # 0x12ac5b
            rax = m.env.data_va(259112);
            // e1988 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e198c mov rsi,rdx
            rsi = rdx;
            // e198f mov rdx,rax
            rdx = rax;
            // e1992 call 0xebf06
            { let r = { let t = m.env.stub_va(0x6d3); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1997 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // e199b test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e199d jne 0xe19a8
            if fl.ne() { pc = 0xe19a8; continue; }
            pc = 0xe199f;
        }
        0xe199f => {
            // block e199f
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e199f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e199f lea r8,[r15+0x1]
            r8 = r15.wrapping_add(0x1);
            // e19a3 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // e19a6 jmp 0xe19b0
            pc = 0xe19b0; continue;
        }
        0xe19a8 => {
            // block e19a8
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19a8 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // e19ac mov r8,QWORD PTR [r15+0x10]
            r8 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0xe19b0;
        }
        0xe19b0 => {
            // block e19b0
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19b0 movzx edx,BYTE PTR [rbp-0x30]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e19b4 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // e19b7 jne 0xe19c2
            if fl.ne() { pc = 0xe19c2; continue; }
            pc = 0xe19b9;
        }
        0xe19b9 => {
            // block e19b9
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19b9 lea r11,[rbp-0x2f]
            r11 = rbp.wrapping_add(0xffff_ffff_ffff_ffd1);
            // e19bd shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e19c0 jmp 0xe19ca
            pc = 0xe19ca; continue;
        }
        0xe19c2 => {
            // block e19c2
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19c2 mov rdx,QWORD PTR [rbp-0x28]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // e19c6 mov r11,QWORD PTR [rbp-0x20]
            r11 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            pc = 0xe19ca;
        }
        0xe19ca => {
            // block e19ca
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19ca cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // e19cd jb 0xe1a22
            if fl.b() { pc = 0xe1a22; continue; }
            pc = 0xe19cf;
        }
        0xe19cf => {
            // block e19cf
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19cf test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e19d2 je 0xe1a32
            if fl.e() { pc = 0xe1a32; continue; }
            pc = 0xe19d4;
        }
        0xe19d4 => {
            // block e19d4
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19d4 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // e19d7 jl 0xe1a22
            if fl.l() { pc = 0xe1a22; continue; }
            pc = 0xe19d9;
        }
        0xe19d9 => {
            // block e19d9
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19d9 mov r10d,0x1
            r10 = (0x1u64) as u32 as u64;
            // e19df sub r10,rdx
            { let (r, f) = sub::<64>(r10, rdx);
              r10 = r;
              fl = f; }
            // e19e2 add r10,rax
            { let (r, f) = add::<64>(r10, rax);
              r10 = r;
              fl = f; }
            // e19e5 je 0xe1a22
            if fl.e() { pc = 0xe1a22; continue; }
            pc = 0xe19e7;
        }
        0xe19e7 => {
            // block e19e7
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19e7 add rax,r8
            { let (r, f) = add::<64>(rax, r8);
              rax = r;
              fl = f; }
            // e19ea add r10,r8
            { let (r, f) = add::<64>(r10, r8);
              r10 = r;
              fl = f; }
            // e19ed movzx r9d,BYTE PTR [r11]
            r9 = (m.r8(r11) as u64) as u32 as u64;
            // e19f1 mov rcx,r8
            rcx = r8;
            pc = 0xe19f4;
        }
        0xe19f4 => {
            // block e19f4
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e19f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e19f4 movzx ebx,BYTE PTR [rcx]
            rbx = (m.r8(rcx) as u64) as u32 as u64;
            // e19f7 cmp ebx,r9d
            fl = sub::<32>((rbx as u32 as u64), (r9 as u32 as u64)).1;
            // e19fa mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // e19ff jne 0xe1a16
            if fl.ne() { pc = 0xe1a16; continue; }
            pc = 0xe1a01;
        }
        0xe1a01 => {
            // block e1a01
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a01 cmp rdx,rbx
            fl = sub::<64>(rdx, rbx).1;
            // e1a04 je 0xe1a26
            if fl.e() { pc = 0xe1a26; continue; }
            pc = 0xe1a06;
        }
        0xe1a06 => {
            // block e1a06
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a06 movzx edi,BYTE PTR [r11+rbx*1]
            rdi = (m.r8(r11.wrapping_add(rbx)) as u64) as u32 as u64;
            // e1a0b movzx esi,BYTE PTR [rcx+rbx*1]
            rsi = (m.r8(rcx.wrapping_add(rbx)) as u64) as u32 as u64;
            // e1a0f inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e1a12 cmp esi,edi
            fl = sub::<32>((rsi as u32 as u64), (rdi as u32 as u64)).1;
            // e1a14 je 0xe1a01
            if fl.e() { pc = 0xe1a01; continue; }
            pc = 0xe1a16;
        }
        0xe1a16 => {
            // block e1a16
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a16 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // e1a19 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1a1b cmp rcx,r10
            fl = sub::<64>(rcx, r10).1;
            // e1a1e jne 0xe19f4
            if fl.ne() { pc = 0xe19f4; continue; }
            pc = 0xe1a20;
        }
        0xe1a20 => {
            // block e1a20
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a20 jmp 0xe1a5b
            pc = 0xe1a5b; continue;
        }
        0xe1a22 => {
            // block e1a22
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a22 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1a24 jmp 0xe1a5b
            pc = 0xe1a5b; continue;
        }
        0xe1a26 => {
            // block e1a26
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a26 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1a28 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // e1a2b je 0xe1a5b
            if fl.e() { pc = 0xe1a5b; continue; }
            pc = 0xe1a2d;
        }
        0xe1a2d => {
            // block e1a2d
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a2d cmp rcx,r8
            fl = sub::<64>(rcx, r8).1;
            // e1a30 jne 0xe1a5b
            if fl.ne() { pc = 0xe1a5b; continue; }
            pc = 0xe1a32;
        }
        0xe1a32 => {
            // block e1a32
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a32 mov rdi,r14
            rdi = r14;
            // e1a35 mov rsi,r15
            rsi = r15;
            // e1a38 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1a3d movzx edx,BYTE PTR [rbp-0x30]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e1a41 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // e1a44 jne 0xe1a4b
            if fl.ne() { pc = 0xe1a4b; continue; }
            pc = 0xe1a46;
        }
        0xe1a46 => {
            // block e1a46
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a46 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e1a49 jmp 0xe1a4f
            pc = 0xe1a4f; continue;
        }
        0xe1a4b => {
            // block e1a4b
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a4b mov rdx,QWORD PTR [rbp-0x28]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            pc = 0xe1a4f;
        }
        0xe1a4f => {
            // block e1a4f
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a4f mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // e1a51 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e1a53 mov rdi,r14
            rdi = r14;
            // e1a56 call 0xebfde
            { let r = { let t = m.env.stub_va(0x7ab); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1a5b;
        }
        0xe1a5b => {
            // block e1a5b
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a5b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e1a5f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1a64 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e1a66 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e1a6a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e1a6b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e1a6d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e1a6f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1a70 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe1a71 => {
            // block e1a71
            #[cfg(feature = "trace")] eprintln!("f_000e196e pc e1a71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a71 mov rbx,rax
            rbx = rax;
            // e1a74 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e1a78 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1a7d mov rdi,rbx
            rdi = rbx;
            // e1a80 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2346 size=288
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2346<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2346;
    loop {
        match pc {
        0xe2346 => {
            // block e2346
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2346 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2346 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2347 mov rbp,rsp
            rbp = rsp;
            // e234a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e234c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e234e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e2350 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2352 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2353 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e2357 mov r13,r9
            r13 = r9;
            // e235a mov ebx,r8d
            rbx = ((r8 as u32 as u64)) as u32 as u64;
            // e235d mov DWORD PTR [rbp-0x6c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rcx as u32 as u64)) as u32);
            // e2360 mov DWORD PTR [rbp-0x5c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rdx as u32 as u64)) as u32);
            // e2363 mov QWORD PTR [rbp-0x68],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rsi) as u64);
            // e2367 mov r15,rdi
            r15 = rdi;
            // e236a call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e236f mov cl,0x1
            rcx = (rcx & !0xff) | ((0x1u64) as u8 as u64);
            // e2371 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e2373 je 0xe242c
            if fl.e() { pc = 0xe242c; continue; }
            pc = 0xe2379;
        }
        0xe2379 => {
            // block e2379
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2379 mov rdi,r15
            rdi = r15;
            // e237c call 0xe494e
            { let r = { let t = m.env.code_va(0xe494e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2381 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2383 je 0xe242a
            if fl.e() { pc = 0xe242a; continue; }
            pc = 0xe2389;
        }
        0xe2389 => {
            // block e2389
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2389 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2389 mov DWORD PTR [rbp-0x70],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), ((rbx as u32 as u64)) as u32);
            // e238c mov r12d,0xffffffe2
            r12 = (0xffffffe2u64) as u32 as u64;
            // e2392 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2396 lea r14,[rbp-0x58]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0xe239a;
        }
        0xe239a => {
            // block e239a
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e239a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e239a mov rdi,rbx
            rdi = rbx;
            // e239d mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e23a1 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23a6 mov rdi,r15
            rdi = r15;
            // e23a9 mov rsi,rbx
            rsi = rbx;
            // e23ac mov edx,DWORD PTR [rbp-0x5c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // e23af call 0xe2466
            { let r = { let t = m.env.code_va(0xe2466); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23b4 mov rdi,rbx
            rdi = rbx;
            // e23b7 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e23b9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23be test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e23c0 je 0xe242a
            if fl.e() { pc = 0xe242a; continue; }
            pc = 0xe23c2;
        }
        0xe23c2 => {
            // block e23c2
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e23c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e23c2 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e23c5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e23c8 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e23ca mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // e23cf call QWORD PTR [rax+0xbd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23d5 mov rdi,r14
            rdi = r14;
            // e23d8 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e23dc call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23e1 mov rdi,r15
            rdi = r15;
            // e23e4 mov rsi,r14
            rsi = r14;
            // e23e7 mov edx,DWORD PTR [rbp-0x5c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // e23ea mov ecx,DWORD PTR [rbp-0x6c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // e23ed mov r8d,DWORD PTR [rbp-0x70]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // e23f1 mov r9,r13
            r9 = r13;
            // e23f4 mov rbx,r13
            rbx = r13;
            // e23f7 call 0xe26e8
            { let r = { let t = m.env.code_va(0xe26e8); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23fc mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // e23ff mov rdi,r14
            rdi = r14;
            // e2402 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2407 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // e240a je 0xe2422
            if fl.e() { pc = 0xe2422; continue; }
            pc = 0xe240c;
        }
        0xe240c => {
            // block e240c
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e240c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e240c cmp r13d,0x1
            fl = sub::<32>((r13 as u32 as u64), 0x1u64).1;
            // e2410 je 0xe243d
            if fl.e() { pc = 0xe243d; continue; }
            pc = 0xe2412;
        }
        0xe2412 => {
            // block e2412
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2412 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2412 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // e2415 mov r13,rbx
            r13 = rbx;
            // e2418 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e241c jne 0xe239a
            if fl.ne() { pc = 0xe239a; continue; }
            pc = 0xe2422;
        }
        0xe2422 => {
            // block e2422
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2422 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2422 mov rdi,r15
            rdi = r15;
            // e2425 call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe242a;
        }
        0xe242a => {
            // block e242a
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e242a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e242a xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe242c;
        }
        0xe242c => {
            // block e242c
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e242c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e242c mov al,cl
            rax = (rax & !0xff) | (((rcx as u8 as u64)) as u8 as u64);
            // e242e add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e2432 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2433 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2435 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e2437 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2439 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e243b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e243c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe243d => {
            // block e243d
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e243d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e243d mov rdi,r15
            rdi = r15;
            // e2440 call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2445 mov cl,0x1
            rcx = (rcx & !0xff) | ((0x1u64) as u8 as u64);
            // e2447 jmp 0xe242c
            pc = 0xe242c; continue;
        }
        0xe2449 => {
            // block e2449
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2449 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2449 mov rbx,rax
            rbx = rax;
            // e244c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2450 jmp 0xe2459
            pc = 0xe2459; continue;
        }
        0xe2452 => {
            // block e2452
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2452 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2452 mov rbx,rax
            rbx = rax;
            // e2455 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0xe2459;
        }
        0xe2459 => {
            // block e2459
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2459 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2459 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e245e mov rdi,rbx
            rdi = rbx;
            // e2461 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb608 size=280
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000eb608<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeb608;
    loop {
        match pc {
        0xeb608 => {
            // block eb608
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb608 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb608 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb609 mov rbp,rsp
            rbp = rsp;
            // eb60c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb60e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb610 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // eb612 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eb614 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb615 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // eb619 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // eb61d mov r15,rdi
            r15 = rdi;
            // eb620 movabs r12,0x492492492492492
            r12 = 0x492_4924_9249_2492u64;
            // eb62a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // eb62d mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // eb631 sub rbx,rax
            { let (r, f) = sub::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // eb634 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // eb638 movabs r13,0x6db6db6db6db6db7
            r13 = 0x6db6_db6d_b6db_6db7u64;
            // eb642 imul rbx,r13
            { let (r, f) = imul::<64>(rbx, r13); rbx = r; fl = f; }
            // eb646 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // eb649 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // eb64c jbe 0xeb659
            if fl.be() { pc = 0xeb659; continue; }
            pc = 0xeb64e;
        }
        0xeb64e => {
            // block eb64e
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb64e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb64e mov rdi,r15
            rdi = r15;
            // eb651 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0xeb659 => {
            // block eb659
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb659 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb659 lea rcx,[r15+0x10]
            rcx = r15.wrapping_add(0x10);
            // eb65d mov rdx,QWORD PTR [r15+0x10]
            rdx = m.r64(r15.wrapping_add(0x10)) as u64;
            // eb661 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // eb664 sar rdx,0x3
            { let (r, f) = sar::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // eb668 imul rdx,r13
            { let (r, f) = imul::<64>(rdx, r13); rdx = r; fl = f; }
            // eb66c movabs rsi,0x249249249249248
            rsi = 0x249_2492_4924_9248u64;
            // eb676 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // eb679 ja 0xeb688
            if fl.a() { pc = 0xeb688; continue; }
            pc = 0xeb67b;
        }
        0xeb67b => {
            // block eb67b
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb67b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb67b add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // eb67e cmp rdx,rbx
            fl = sub::<64>(rdx, rbx).1;
            // eb681 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // eb685 mov r12,rdx
            r12 = rdx;
            pc = 0xeb688;
        }
        0xeb688 => {
            // block eb688
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb688 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb688 mov r14,QWORD PTR [r15+0x8]
            r14 = m.r64(r15.wrapping_add(0x8)) as u64;
            // eb68c sub r14,rax
            { let (r, f) = sub::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // eb68f sar r14,0x3
            { let (r, f) = sar::<64>(r14, 0x3u64, fl); r14 = r; fl = f; }
            // eb693 imul r14,r13
            { let (r, f) = imul::<64>(r14, r13); r14 = r; fl = f; }
            // eb697 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // eb69f mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // eb6a3 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // eb6a5 test r12,r12
            fl = logic::<64>(r12 & r12);
            // eb6a8 je 0xeb6b6
            if fl.e() { pc = 0xeb6b6; continue; }
            pc = 0xeb6aa;
        }
        0xeb6aa => {
            // block eb6aa
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb6aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb6aa imul rdi,r12,0x38
            { let (r, f) = imul::<64>(r12, 0x38u64); rdi = r; fl = f; }
            // eb6ae call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb6b3 mov rbx,rax
            rbx = rax;
            pc = 0xeb6b6;
        }
        0xeb6b6 => {
            // block eb6b6
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb6b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb6b6 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // eb6ba imul r14,r14,0x38
            { let (r, f) = imul::<64>(r14, 0x38u64); r14 = r; fl = f; }
            // eb6be lea rdi,[rbx+r14*1]
            rdi = rbx.wrapping_add(r14);
            // eb6c2 mov QWORD PTR [rbp-0x40],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdi) as u64);
            // eb6c6 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // eb6ca imul rax,r12,0x38
            { let (r, f) = imul::<64>(r12, 0x38u64); rax = r; fl = f; }
            // eb6ce add rax,rbx
            { let (r, f) = add::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // eb6d1 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // eb6d5 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // eb6d9 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb6de lea rax,[r14+rbx*1+0x38]
            rax = r14.wrapping_add(rbx).wrapping_add(0x38);
            // eb6e3 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // eb6e7 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // eb6eb mov rdi,r15
            rdi = r15;
            // eb6ee call 0xebf30
            { let r = { let t = m.env.stub_va(0x6fd); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb6f3 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // eb6f7 call 0xeb8ac
            { let r = { let t = m.env.stub_va(0x79); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb6fc add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // eb700 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb701 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eb703 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eb705 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb707 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb709 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb70a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeb70b => {
            // block eb70b
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb70b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb70b mov rbx,rax
            rbx = rax;
            // eb70e lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // eb712 call 0xeb8ac
            { let r = { let t = m.env.stub_va(0x79); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb717 mov rdi,rbx
            rdi = rbx;
            // eb71a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
