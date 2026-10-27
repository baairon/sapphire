//! Mechanically lifted functions (scripts/lift_run.py), module r001. Each one is checked against the original
//! in the world harness (tests/world_r001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x914a size=410
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000914a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x914a;
    loop {
        match pc {
        0x914a => {
            // block 914a
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 914a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 914a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 914b mov rbp,rsp
            rbp = rsp;
            // 914e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9150 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9152 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 9154 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 9156 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9157 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 915b mov r14,rsi
            r14 = rsi;
            // 915e mov rbx,rdi
            rbx = rdi;
            // 9161 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 9164 mov rdi,r14
            rdi = r14;
            // 9167 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 916a lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 916e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 9176 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 917e mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 9182 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 9185 mov rax,QWORD PTR [rax+0x198]
            rax = m.r64(rax.wrapping_add(0x198)) as u64;
            // 918c mov rdi,rbx
            rdi = rbx;
            // 918f call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9191 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9195 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 9197 call 0xbc038
            { let r = { let t = m.env.code_va(0xbc038); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 919c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 919e je 0x9295
            if fl.e() { pc = 0x9295; continue; }
            pc = 0x91a4;
        }
        0x91a4 => {
            // block 91a4
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 91a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 91a4 mov r12,QWORD PTR [rbp-0x40]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 91a8 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 91ab lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 91af cmp r12,rax
            fl = sub::<64>(r12, rax).1;
            // 91b2 je 0x9298
            if fl.e() { pc = 0x9298; continue; }
            pc = 0x91b8;
        }
        0x91b8 => {
            // block 91b8
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 91b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 91b8 lea r15,[rbp-0x50]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 91bc lea r13,[rbp-0x58]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0x91c0;
        }
        0x91c0 => {
            // block 91c0
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 91c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 91c0 mov edi,DWORD PTR [r12+0x1c]
            rdi = (m.r32(r12.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 91c5 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 91cd mov QWORD PTR [r15],0x0
            m.w64(r15, (0x0u64) as u64);
            // 91d4 mov QWORD PTR [rbp-0x58],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r15) as u64);
            // 91d8 mov rsi,r13
            rsi = r13;
            // 91db call 0xbc13c
            { let r = { let t = m.env.code_va(0xbc13c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 91e0 mov r13,QWORD PTR [rbp-0x58]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 91e4 jmp 0x9246
            pc = 0x9246; continue;
        }
        0x91e6 => {
            // block 91e6
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 91e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 91e6 mov DWORD PTR [rbp-0x5c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), (0x0u64) as u32);
            // 91ed mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 91f0 mov rax,QWORD PTR [rax+0x10c8]
            rax = m.r64(rax.wrapping_add(0x10c8)) as u64;
            // 91f7 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 91fb mov rdi,rbx
            rdi = rbx;
            // 91fe lea rdx,[rbp-0x5c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa4);
            // 9202 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9204 mov edx,DWORD PTR [rbp-0x5c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // 9207 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 9209 je 0x921f
            if fl.e() { pc = 0x921f; continue; }
            pc = 0x920b;
        }
        0x920b => {
            // block 920b
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 920b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 920b xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 920d jne 0x921f
            if fl.ne() { pc = 0x921f; continue; }
            pc = 0x920f;
        }
        0x920f => {
            // block 920f
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 920f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 920f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 9212 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 9216 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 921a mov rdi,r14
            rdi = r14;
            // 921d call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x921f;
        }
        0x921f => {
            // block 921f
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 921f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 921f mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 9223 test rax,rax
            fl = logic::<64>(rax & rax);
            // 9226 je 0x9236
            if fl.e() { pc = 0x9236; continue; }
            pc = 0x9228;
        }
        0x9228 => {
            // block 9228
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9228 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9228 mov r13,rax
            r13 = rax;
            // 922b mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 922f test rax,rax
            fl = logic::<64>(rax & rax);
            // 9232 jne 0x9228
            if fl.ne() { pc = 0x9228; continue; }
            pc = 0x9234;
        }
        0x9234 => {
            // block 9234
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9234 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9234 jmp 0x9246
            pc = 0x9246; continue;
        }
        0x9236 => {
            // block 9236
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9236 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9236 mov rax,r13
            rax = r13;
            pc = 0x9239;
        }
        0x9239 => {
            // block 9239
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9239 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9239 mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // 923d cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // 9241 mov rax,r13
            rax = r13;
            // 9244 jne 0x9239
            if fl.ne() { pc = 0x9239; continue; }
            pc = 0x9246;
        }
        0x9246 => {
            // block 9246
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9246 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9246 cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 9249 jne 0x91e6
            if fl.ne() { pc = 0x91e6; continue; }
            pc = 0x924b;
        }
        0x924b => {
            // block 924b
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 924b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 924b mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 924f lea r13,[rbp-0x58]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9253 mov rdi,r13
            rdi = r13;
            // 9256 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 925b mov rax,QWORD PTR [r12+0x8]
            rax = m.r64(r12.wrapping_add(0x8)) as u64;
            // 9260 test rax,rax
            fl = logic::<64>(rax & rax);
            // 9263 je 0x9273
            if fl.e() { pc = 0x9273; continue; }
            pc = 0x9265;
        }
        0x9265 => {
            // block 9265
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9265 mov r12,rax
            r12 = rax;
            // 9268 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 926c test rax,rax
            fl = logic::<64>(rax & rax);
            // 926f jne 0x9265
            if fl.ne() { pc = 0x9265; continue; }
            pc = 0x9271;
        }
        0x9271 => {
            // block 9271
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9271 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9271 jmp 0x9283
            pc = 0x9283; continue;
        }
        0x9273 => {
            // block 9273
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9273 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9273 mov rax,r12
            rax = r12;
            pc = 0x9276;
        }
        0x9276 => {
            // block 9276
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9276 mov r12,QWORD PTR [rax+0x10]
            r12 = m.r64(rax.wrapping_add(0x10)) as u64;
            // 927a cmp QWORD PTR [r12],rax
            fl = sub::<64>(m.r64(r12) as u64, rax).1;
            // 927e mov rax,r12
            rax = r12;
            // 9281 jne 0x9276
            if fl.ne() { pc = 0x9276; continue; }
            pc = 0x9283;
        }
        0x9283 => {
            // block 9283
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9283 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9283 lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 9287 cmp r12,rax
            fl = sub::<64>(r12, rax).1;
            // 928a jne 0x91c0
            if fl.ne() { pc = 0x91c0; continue; }
            pc = 0x9290;
        }
        0x9290 => {
            // block 9290
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9290 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9290 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 9293 jmp 0x9298
            pc = 0x9298; continue;
        }
        0x9295 => {
            // block 9295
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9295 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9295 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x9298;
        }
        0x9298 => {
            // block 9298
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9298 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9298 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 929c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 92a0 call 0xebd80
            { let r = { let t = m.env.stub_va(0x54d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 92a5 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 92a8 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 92ac pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 92ad pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 92af pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 92b1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 92b3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 92b5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 92b6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x92b7 => {
            // block 92b7
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 92b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 92b7 mov rbx,rax
            rbx = rax;
            // 92ba mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 92be lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 92c2 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x92c7;
        }
        0x92c7 => {
            // block 92c7
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 92c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 92c7 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 92cb lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 92cf call 0xebd80
            { let r = { let t = m.env.stub_va(0x54d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 92d4 mov rdi,rbx
            rdi = rbx;
            // 92d7 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 92dc jmp 0x92b7
            pc = 0x92b7; continue;
        }
        0x92de => {
            // block 92de
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 92de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 92de mov rbx,rax
            rbx = rax;
            // 92e1 jmp 0x92c7
            pc = 0x92c7; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x92e4 size=484
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000092e4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x92e4;
    loop {
        match pc {
        0x92e4 => {
            // block 92e4
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 92e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 92e4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 92e5 mov rbp,rsp
            rbp = rsp;
            // 92e8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 92ea push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 92ec push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 92ee push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 92f0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 92f1 sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 92f5 mov QWORD PTR [rbp-0x88],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdx) as u64);
            // 92fc mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 92ff mov r14,rdi
            r14 = rdi;
            // 9302 xorps xmm0,xmm0
            x0 = 0;
            // 9305 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 9309 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 930d movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 9311 mov rax,QWORD PTR [rdx]
            rax = m.r64(rdx) as u64;
            // 9314 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 9318 mov rdi,rdx
            rdi = rdx;
            // 931b call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 931d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 9320 mov rax,QWORD PTR [rax+0x1d28]
            rax = m.r64(rax.wrapping_add(0x1d28)) as u64;
            // 9327 mov rdi,r14
            rdi = r14;
            // 932a mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 932d call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 932f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 9331 je 0x933a
            if fl.e() { pc = 0x933a; continue; }
            pc = 0x9333;
        }
        0x9333 => {
            // block 9333
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9333 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9333 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 9335 jmp 0x9485
            pc = 0x9485; continue;
        }
        0x933a => {
            // block 933a
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 933a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 933a mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 933d lea rcx,[rip+0xf636c] # 0xff6b0
            rcx = m.env.data_va(81533);
            // 9344 lea rsi,[rcx+rax*4]
            rsi = rcx.wrapping_add(rax.wrapping_mul(4));
            // 9348 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 934c call 0xeb882
            { let r = { let t = m.env.stub_va(0x4f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9351 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // 9353 cmp QWORD PTR [rbp-0x38],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, 0x0u64).1;
            // 9358 je 0x9485
            if fl.e() { pc = 0x9485; continue; }
            pc = 0x935e;
        }
        0x935e => {
            // block 935e
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 935e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 935e lea r13,[rbp-0x78]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 9362 lea r12,[rbp-0x60]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            pc = 0x9366;
        }
        0x9366 => {
            // block 9366
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9366 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9366 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 936a mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 936e mov rdx,rcx
            rdx = rcx;
            // 9371 shr rdx,0xa
            { let (r, f) = shr::<64>(rdx, 0xau64, fl); rdx = r; fl = f; }
            // 9375 mov rax,QWORD PTR [rax+rdx*8]
            rax = m.r64(rax.wrapping_add(rdx.wrapping_mul(8))) as u64;
            // 9379 and rcx,0x3ff
            { let r = rcx & 0x3ffu64; let f = logic::<64>(r);
              rcx = r;
              fl = f; }
            // 9380 mov r15d,DWORD PTR [rax+rcx*4]
            r15 = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 9384 mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 938b mov DWORD PTR [rbp-0x68],0x6c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0x6cu64) as u32);
            // 9392 mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 939a mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 93a2 mov QWORD PTR [rbp-0x80],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (r13) as u64);
            // 93a6 mov rdi,r12
            rdi = r12;
            // 93a9 call 0xeb87c
            { let r = { let t = m.env.stub_va(0x49); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 93ae cmp r15d,0xffffffff
            fl = sub::<32>((r15 as u32 as u64), 0xffffffffu64).1;
            // 93b2 je 0x946b
            if fl.e() { pc = 0x946b; continue; }
            pc = 0x93b8;
        }
        0x93b8 => {
            // block 93b8
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 93b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 93b8 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 93bb mov rax,QWORD PTR [rax+0x10c8]
            rax = m.r64(rax.wrapping_add(0x10c8)) as u64;
            // 93c2 mov rdi,r14
            rdi = r14;
            // 93c5 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 93c8 lea rdx,[rbp-0x64]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 93cc call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 93ce test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 93d0 je 0x946b
            if fl.e() { pc = 0x946b; continue; }
            pc = 0x93d6;
        }
        0x93d6 => {
            // block 93d6
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 93d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 93d6 mov edx,DWORD PTR [rbp-0x64]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 93d9 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 93db je 0x946b
            if fl.e() { pc = 0x946b; continue; }
            pc = 0x93e1;
        }
        0x93e1 => {
            // block 93e1
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 93e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 93e1 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 93e8 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 93eb mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 93ef mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 93f2 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 93f4 mov edi,DWORD PTR [rbp-0x64]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 93f7 lea rsi,[rbp-0x68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 93fb call 0xbc0fc
            { let r = { let t = m.env.code_va(0xbc0fc); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9400 cmp DWORD PTR [rbp-0x68],0x6b
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, 0x6bu64).1;
            // 9404 ja 0x946b
            if fl.a() { pc = 0x946b; continue; }
            pc = 0x9406;
        }
        0x9406 => {
            // block 9406
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9406 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9406 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 9409 mov rax,QWORD PTR [rax+0x198]
            rax = m.r64(rax.wrapping_add(0x198)) as u64;
            // 9410 mov rdi,r14
            rdi = r14;
            // 9413 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9415 mov esi,DWORD PTR [rbp-0x68]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // 9418 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 941a call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 941f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 9421 je 0x946b
            if fl.e() { pc = 0x946b; continue; }
            pc = 0x9423;
        }
        0x9423 => {
            // block 9423
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9423 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9423 mov edi,DWORD PTR [rbp-0x68]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // 9426 lea rsi,[rbp-0x80]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 942a call 0xbc13c
            { let r = { let t = m.env.code_va(0xbc13c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 942f mov rbx,QWORD PTR [rbp-0x80]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 9433 jmp 0x9466
            pc = 0x9466; continue;
        }
        0x9435 => {
            // block 9435
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9435 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9435 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 9439 mov rdi,r12
            rdi = r12;
            // 943c call 0xeb882
            { let r = { let t = m.env.stub_va(0x4f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9441 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 9445 test rax,rax
            fl = logic::<64>(rax & rax);
            // 9448 je 0x9457
            if fl.e() { pc = 0x9457; continue; }
            pc = 0x944a;
        }
        0x944a => {
            // block 944a
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 944a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 944a mov rbx,rax
            rbx = rax;
            // 944d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 9450 test rax,rax
            fl = logic::<64>(rax & rax);
            // 9453 jne 0x944a
            if fl.ne() { pc = 0x944a; continue; }
            pc = 0x9455;
        }
        0x9455 => {
            // block 9455
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9455 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9455 jmp 0x9466
            pc = 0x9466; continue;
        }
        0x9457 => {
            // block 9457
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9457 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9457 mov rax,rbx
            rax = rbx;
            pc = 0x945a;
        }
        0x945a => {
            // block 945a
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 945a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 945a mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 945e cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 9461 mov rax,rbx
            rax = rbx;
            // 9464 jne 0x945a
            if fl.ne() { pc = 0x945a; continue; }
            pc = 0x9466;
        }
        0x9466 => {
            // block 9466
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9466 cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // 9469 jne 0x9435
            if fl.ne() { pc = 0x9435; continue; }
            pc = 0x946b;
        }
        0x946b => {
            // block 946b
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 946b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 946b mov rsi,QWORD PTR [rbp-0x78]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 946f lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 9473 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9478 cmp QWORD PTR [rbp-0x38],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, 0x0u64).1;
            // 947d jne 0x9366
            if fl.ne() { pc = 0x9366; continue; }
            pc = 0x9483;
        }
        0x9483 => {
            // block 9483
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9483 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x9485;
        }
        0x9485 => {
            // block 9485
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 9485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9485 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 9489 call 0xeb83a
            { let r = { let t = m.env.stub_va(0x7); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 948e mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 9490 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 9494 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9495 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 9497 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 9499 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 949b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 949d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 949e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x949f => {
            // block 949f
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 949f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 949f jmp 0x94a1
            pc = 0x94a1; continue;
        }
        0x94a1 => {
            // block 94a1
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 94a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 94a1 mov rbx,rax
            rbx = rax;
            // 94a4 mov rsi,QWORD PTR [rbp-0x78]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 94a8 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 94ac call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x94b1;
        }
        0x94b1 => {
            // block 94b1
            #[cfg(feature = "trace")] eprintln!("f_000092e4 pc 94b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 94b1 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 94b5 call 0xeb83a
            { let r = { let t = m.env.stub_va(0x7); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 94ba mov rdi,rbx
            rdi = rbx;
            // 94bd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 94c2 mov rbx,rax
            rbx = rax;
            // 94c5 jmp 0x94b1
            pc = 0x94b1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xed4c size=380
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ed4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xed4c;
    loop {
        match pc {
        0xed4c => {
            // block ed4c
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ed4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ed4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ed4d mov rbp,rsp
            rbp = rsp;
            // ed50 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ed52 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ed54 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ed56 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ed58 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ed59 sub rsp,0xb8
            { let (r, f) = sub::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // ed60 mov r15,rdx
            r15 = rdx;
            // ed63 mov r13,rsi
            r13 = rsi;
            // ed66 mov rbx,rdi
            rbx = rdi;
            // ed69 lea r14,[r15+0x8]
            r14 = r15.wrapping_add(0x8);
            // ed6d mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // ed71 mov rdi,r15
            rdi = r15;
            // ed74 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ed79 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // ed81 mov QWORD PTR [r15],r14
            m.w64(r15, (r14) as u64);
            // ed84 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // ed8c cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // ed90 je 0xedd5
            if fl.e() { pc = 0xedd5; continue; }
            pc = 0xed92;
        }
        0xed92 => {
            // block ed92
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ed92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ed92 lea r12,[rbp-0xd0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // ed99 mov rdi,r12
            rdi = r12;
            // ed9c mov rsi,r13
            rsi = r13;
            // ed9f call 0x2b982
            { let r = { let t = m.env.code_va(0x2b982); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eda4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // eda7 mov rax,QWORD PTR [rax+0xd0]
            rax = m.r64(rax.wrapping_add(0xd0)) as u64;
            // edae mov rdi,rbx
            rdi = rbx;
            // edb1 mov rsi,r12
            rsi = r12;
            // edb4 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // edb6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // edb8 je 0xeddc
            if fl.e() { pc = 0xeddc; continue; }
            pc = 0xedba;
        }
        0xedba => {
            // block edba
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc edba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // edba lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // edc1 mov rsi,r15
            rsi = r15;
            // edc4 call 0x2bbc2
            { let r = { let t = m.env.code_va(0x2bbc2); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // edc9 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // edcb jmp 0xee6c
            pc = 0xee6c; continue;
        }
        0xedd0 => {
            // block edd0
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc edd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // edd0 jmp 0xee9e
            pc = 0xee9e; continue;
        }
        0xedd5 => {
            // block edd5
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc edd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // edd5 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // edd7 jmp 0xee8a
            pc = 0xee8a; continue;
        }
        0xeddc => {
            // block eddc
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc eddc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eddc mov r14,QWORD PTR [r13+0x0]
            r14 = m.r64(r13) as u64;
            // ede0 mov QWORD PTR [rbp-0xe0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r13) as u64);
            // ede7 lea r13,[r13+0x8]
            r13 = r13.wrapping_add(0x8);
            // edeb jmp 0xee55
            pc = 0xee55; continue;
        }
        0xeded => {
            // block eded
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc eded rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eded mov DWORD PTR [rbp-0xd4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c), (0x0u64) as u32);
            // edf7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // edfa mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // edfe mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // ee02 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ee07 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ee0a mov rdi,rbx
            rdi = rbx;
            // ee0d lea rdx,[rbp-0xd4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff2c);
            // ee14 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ee16 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // ee18 je 0xee30
            if fl.e() { pc = 0xee30; continue; }
            pc = 0xee1a;
        }
        0xee1a => {
            // block ee1a
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee1a lea rsi,[r14+0x1c]
            rsi = r14.wrapping_add(0x1c);
            // ee1e mov r12d,DWORD PTR [rbp-0xd4]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c)) as u64) as u32 as u64;
            // ee25 mov rdi,r15
            rdi = r15;
            // ee28 call 0xeb870
            { let r = { let t = m.env.stub_va(0x3d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ee2d mov DWORD PTR [rax],r12d
            m.w32(rax, ((r12 as u32 as u64)) as u32);
            pc = 0xee30;
        }
        0xee30 => {
            // block ee30
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee30 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // ee34 test rax,rax
            fl = logic::<64>(rax & rax);
            // ee37 je 0xee46
            if fl.e() { pc = 0xee46; continue; }
            pc = 0xee39;
        }
        0xee39 => {
            // block ee39
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee39 mov r14,rax
            r14 = rax;
            // ee3c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ee3f test rax,rax
            fl = logic::<64>(rax & rax);
            // ee42 jne 0xee39
            if fl.ne() { pc = 0xee39; continue; }
            pc = 0xee44;
        }
        0xee44 => {
            // block ee44
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee44 jmp 0xee55
            pc = 0xee55; continue;
        }
        0xee46 => {
            // block ee46
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee46 mov rax,r14
            rax = r14;
            pc = 0xee49;
        }
        0xee49 => {
            // block ee49
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee49 mov r14,QWORD PTR [rax+0x10]
            r14 = m.r64(rax.wrapping_add(0x10)) as u64;
            // ee4d cmp QWORD PTR [r14],rax
            fl = sub::<64>(m.r64(r14) as u64, rax).1;
            // ee50 mov rax,r14
            rax = r14;
            // ee53 jne 0xee49
            if fl.ne() { pc = 0xee49; continue; }
            pc = 0xee55;
        }
        0xee55 => {
            // block ee55
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee55 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // ee58 jne 0xeded
            if fl.ne() { pc = 0xeded; continue; }
            pc = 0xee5a;
        }
        0xee5a => {
            // block ee5a
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee5a mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // ee5e mov rcx,QWORD PTR [rbp-0xe0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff20)) as u64;
            // ee65 cmp rax,QWORD PTR [rcx+0x10]
            fl = sub::<64>(rax, m.r64(rcx.wrapping_add(0x10)) as u64).1;
            // ee69 sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0xee6c;
        }
        0xee6c => {
            // block ee6c
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee6c lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // ee70 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ee75 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // ee79 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ee7e lea rdi,[rbp-0xac]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff54);
            // ee85 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xee8a;
        }
        0xee8a => {
            // block ee8a
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee8a mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // ee8c add rsp,0xb8
            { let (r, f) = add::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // ee93 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ee94 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ee96 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ee98 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ee9a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ee9c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ee9d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xee9e => {
            // block ee9e
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee9e mov rbx,rax
            rbx = rax;
            // eea1 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // eea5 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eeaa lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // eeae call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eeb3 lea rdi,[rbp-0xac]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff54);
            // eeba call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eebf mov rdi,rbx
            rdi = rbx;
            // eec2 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10b02 size=372
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010b02<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x10b02;
    loop {
        match pc {
        0x10b02 => {
            // block 10b02
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10b02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10b02 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10b03 mov rbp,rsp
            rbp = rsp;
            // 10b06 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10b08 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10b0a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 10b0c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10b0e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10b0f sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10b13 mov r15,rdi
            r15 = rdi;
            // 10b16 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10b1a mov r13,QWORD PTR [r15+0x18]
            r13 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10b1e cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 10b21 jne 0x10c5b
            if fl.ne() { pc = 0x10c5b; continue; }
            pc = 0x10b27;
        }
        0x10b27 => {
            // block 10b27
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10b27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10b27 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 10b2b mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 10b2e mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10b32 mov rax,rbx
            rax = rbx;
            // 10b35 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 10b38 jbe 0x10b8d
            if fl.be() { pc = 0x10b8d; continue; }
            pc = 0x10b3a;
        }
        0x10b3a => {
            // block 10b3a
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10b3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10b3a sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10b3e lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 10b42 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 10b46 lea r14,[rax+rcx*1+0x1]
            r14 = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 10b4b sar r14,1
            { let (r, f) = sar::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 10b4e lea rax,[r14*8+0x0]
            rax = r14.wrapping_mul(8);
            // 10b56 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10b5a mov rdi,rbx
            rdi = rbx;
            // 10b5d sub rdi,rax
            { let (r, f) = sub::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 10b60 sub r12,rbx
            { let (r, f) = sub::<64>(r12, rbx);
              r12 = r;
              fl = f; }
            // 10b63 mov r13,r12
            r13 = r12;
            // 10b66 shr r13,0x3
            { let (r, f) = shr::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // 10b6a mov rsi,rbx
            rsi = rbx;
            // 10b6d mov rdx,r12
            rdx = r12;
            // 10b70 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10b75 sub r13,r14
            { let (r, f) = sub::<64>(r13, r14);
              r13 = r;
              fl = f; }
            // 10b78 lea r12,[rbx+r13*8]
            r12 = rbx.wrapping_add(r13.wrapping_mul(8));
            // 10b7c mov QWORD PTR [r15+0x10],r12
            m.w64(r15.wrapping_add(0x10), (r12) as u64);
            // 10b80 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 10b84 sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10b88 jmp 0x10c57
            pc = 0x10c57; continue;
        }
        0x10b8d => {
            // block 10b8d
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10b8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10b8d mov rax,r12
            rax = r12;
            // 10b90 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 10b93 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10b97 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 10b9d cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 10ba1 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 10bab and rax,r14
            { let r = rax & r14; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 10bae mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10bb2 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 10bb6 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 10bba mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 10bbe lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 10bc6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10bcb mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 10bcf mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 10bd3 lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 10bd7 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10bdb mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 10bdf lea rdx,[rax+r14*8]
            rdx = rax.wrapping_add(r14.wrapping_mul(8));
            // 10be3 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 10be7 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 10bea je 0x10c24
            if fl.e() { pc = 0x10c24; continue; }
            pc = 0x10bec;
        }
        0x10bec => {
            // block 10bec
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10bec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10bec mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 10bef mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 10bf2 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 10bf6 add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 10bfa mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10bfe add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 10c02 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 10c05 jne 0x10bec
            if fl.ne() { pc = 0x10bec; continue; }
            pc = 0x10c07;
        }
        0x10c07 => {
            // block 10c07
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10c07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c07 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // 10c0a mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10c0e mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 10c12 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 10c16 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10c1a mov r13,QWORD PTR [r15+0x18]
            r13 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10c1e mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10c22 jmp 0x10c2b
            pc = 0x10c2b; continue;
        }
        0x10c24 => {
            // block 10c24
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10c24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c24 mov rsi,rcx
            rsi = rcx;
            // 10c27 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0x10c2b;
        }
        0x10c2b => {
            // block 10c2b
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10c2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c2b mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 10c2e mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 10c32 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 10c36 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 10c3a mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 10c3e mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 10c42 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 10c46 mov QWORD PTR [rbp-0x38],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r13) as u64);
            // 10c4a lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10c4e call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10c53 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x10c57;
        }
        0x10c57 => {
            // block 10c57
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10c57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c57 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x10c5b;
        }
        0x10c5b => {
            // block 10c5b
            #[cfg(feature = "trace")] eprintln!("f_00010b02 pc 10c5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10c5b mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 10c5e mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 10c62 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10c67 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10c6b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10c6c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 10c6e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 10c70 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10c72 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10c74 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10c75 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10de4 size=372
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010de4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x10de4;
    loop {
        match pc {
        0x10de4 => {
            // block 10de4
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10de4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10de4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10de5 mov rbp,rsp
            rbp = rsp;
            // 10de8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10dea push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10dec push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 10dee push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10df0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10df1 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10df5 mov r15,rdi
            r15 = rdi;
            // 10df8 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10dfc mov r13,QWORD PTR [r15+0x18]
            r13 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10e00 cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 10e03 jne 0x10f3d
            if fl.ne() { pc = 0x10f3d; continue; }
            pc = 0x10e09;
        }
        0x10e09 => {
            // block 10e09
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10e09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e09 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 10e0d mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 10e10 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10e14 mov rax,rbx
            rax = rbx;
            // 10e17 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 10e1a jbe 0x10e6f
            if fl.be() { pc = 0x10e6f; continue; }
            pc = 0x10e1c;
        }
        0x10e1c => {
            // block 10e1c
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10e1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e1c sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10e20 lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 10e24 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 10e28 lea r14,[rax+rcx*1+0x1]
            r14 = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 10e2d sar r14,1
            { let (r, f) = sar::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 10e30 lea rax,[r14*8+0x0]
            rax = r14.wrapping_mul(8);
            // 10e38 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10e3c mov rdi,rbx
            rdi = rbx;
            // 10e3f sub rdi,rax
            { let (r, f) = sub::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 10e42 sub r12,rbx
            { let (r, f) = sub::<64>(r12, rbx);
              r12 = r;
              fl = f; }
            // 10e45 mov r13,r12
            r13 = r12;
            // 10e48 shr r13,0x3
            { let (r, f) = shr::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // 10e4c mov rsi,rbx
            rsi = rbx;
            // 10e4f mov rdx,r12
            rdx = r12;
            // 10e52 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10e57 sub r13,r14
            { let (r, f) = sub::<64>(r13, r14);
              r13 = r;
              fl = f; }
            // 10e5a lea r12,[rbx+r13*8]
            r12 = rbx.wrapping_add(r13.wrapping_mul(8));
            // 10e5e mov QWORD PTR [r15+0x10],r12
            m.w64(r15.wrapping_add(0x10), (r12) as u64);
            // 10e62 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 10e66 sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10e6a jmp 0x10f39
            pc = 0x10f39; continue;
        }
        0x10e6f => {
            // block 10e6f
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10e6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e6f mov rax,r12
            rax = r12;
            // 10e72 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 10e75 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10e79 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 10e7f cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 10e83 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 10e8d and rax,r14
            { let r = rax & r14; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 10e90 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10e94 mov rax,QWORD PTR [r15+0x20]
            rax = m.r64(r15.wrapping_add(0x20)) as u64;
            // 10e98 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 10e9c mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 10ea0 lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 10ea8 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10ead mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 10eb1 mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 10eb5 lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 10eb9 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10ebd mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 10ec1 lea rdx,[rax+r14*8]
            rdx = rax.wrapping_add(r14.wrapping_mul(8));
            // 10ec5 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 10ec9 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 10ecc je 0x10f06
            if fl.e() { pc = 0x10f06; continue; }
            pc = 0x10ece;
        }
        0x10ece => {
            // block 10ece
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10ece rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10ece mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 10ed1 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 10ed4 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 10ed8 add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 10edc mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10ee0 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 10ee4 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 10ee7 jne 0x10ece
            if fl.ne() { pc = 0x10ece; continue; }
            pc = 0x10ee9;
        }
        0x10ee9 => {
            // block 10ee9
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10ee9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10ee9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // 10eec mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10ef0 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 10ef4 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 10ef8 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10efc mov r13,QWORD PTR [r15+0x18]
            r13 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10f00 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10f04 jmp 0x10f0d
            pc = 0x10f0d; continue;
        }
        0x10f06 => {
            // block 10f06
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f06 mov rsi,rcx
            rsi = rcx;
            // 10f09 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0x10f0d;
        }
        0x10f0d => {
            // block 10f0d
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f0d mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 10f10 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 10f14 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 10f18 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 10f1c mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 10f20 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 10f24 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 10f28 mov QWORD PTR [rbp-0x38],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r13) as u64);
            // 10f2c lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10f30 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10f35 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x10f39;
        }
        0x10f39 => {
            // block 10f39
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f39 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x10f3d;
        }
        0x10f3d => {
            // block 10f3d
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f3d mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 10f40 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 10f44 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10f49 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10f4d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10f4e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 10f50 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 10f52 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10f54 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10f56 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10f57 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1582c size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001582c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1582c;
    loop {
        match pc {
        0x1582c => {
            // block 1582c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1582c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1582c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1582d mov rbp,rsp
            rbp = rsp;
            // 15830 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15832 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15834 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15836 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15838 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15839 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 1583d mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 15841 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 15844 je 0x15980
            if fl.e() { pc = 0x15980; continue; }
            pc = 0x1584a;
        }
        0x1584a => {
            // block 1584a
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1584a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1584a mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1584e movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // 15851 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 15854 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15858 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 1585b mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1585f mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 15863 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 15867 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 1586b mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 1586f lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // 15873 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 15877 jmp 0x1587c
            pc = 0x1587c; continue;
        }
        0x15879 => {
            // block 15879
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15879 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15879 mov rbx,rax
            rbx = rax;
            pc = 0x1587c;
        }
        0x1587c => {
            // block 1587c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1587c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1587c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 1587f mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 15883 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 15888 movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 1588c mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1588f and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15893 jne 0x1589d
            if fl.ne() { pc = 0x1589d; continue; }
            pc = 0x15895;
        }
        0x15895 => {
            // block 15895
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15895 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15895 mov r14,rax
            r14 = rax;
            // 15898 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 1589b jmp 0x158a1
            pc = 0x158a1; continue;
        }
        0x1589d => {
            // block 1589d
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1589d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1589d mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x158a1;
        }
        0x158a1 => {
            // block 158a1
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158a1 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // 158a5 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 158a8 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 158ac cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 158b1 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 158b4 jne 0x158c0
            if fl.ne() { pc = 0x158c0; continue; }
            pc = 0x158b6;
        }
        0x158b6 => {
            // block 158b6
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158b6 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // 158ba mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 158be jmp 0x158c8
            pc = 0x158c8; continue;
        }
        0x158c0 => {
            // block 158c0
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158c0 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 158c4 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x158c8;
        }
        0x158c8 => {
            // block 158c8
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158c8 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 158cb mov rdx,r13
            rdx = r13;
            // 158ce cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // 158d2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 158d5 je 0x158ec
            if fl.e() { pc = 0x158ec; continue; }
            pc = 0x158d7;
        }
        0x158d7 => {
            // block 158d7
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158d7 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 158db call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 158e0 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 158e2 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 158e6 je 0x158ec
            if fl.e() { pc = 0x158ec; continue; }
            pc = 0x158e8;
        }
        0x158e8 => {
            // block 158e8
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158e8 jns 0x158fe
            if fl.ns() { pc = 0x158fe; continue; }
            pc = 0x158ea;
        }
        0x158ea => {
            // block 158ea
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158ea jmp 0x158f1
            pc = 0x158f1; continue;
        }
        0x158ec => {
            // block 158ec
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158ec cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 158ef jae 0x158fe
            if fl.ae() { pc = 0x158fe; continue; }
            pc = 0x158f1;
        }
        0x158f1 => {
            // block 158f1
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158f1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 158f4 test rax,rax
            fl = logic::<64>(rax & rax);
            // 158f7 jne 0x15879
            if fl.ne() { pc = 0x15879; continue; }
            pc = 0x158f9;
        }
        0x158f9 => {
            // block 158f9
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158f9 jmp 0x15995
            pc = 0x15995; continue;
        }
        0x158fe => {
            // block 158fe
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158fe test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 15901 jne 0x1590b
            if fl.ne() { pc = 0x1590b; continue; }
            pc = 0x15903;
        }
        0x15903 => {
            // block 15903
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15903 mov r14,rbx
            r14 = rbx;
            // 15906 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 15909 jmp 0x15912
            pc = 0x15912; continue;
        }
        0x1590b => {
            // block 1590b
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1590b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1590b mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 1590f mov r14,rbx
            r14 = rbx;
            pc = 0x15912;
        }
        0x15912 => {
            // block 15912
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15912 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15912 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 15916 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 15919 mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 1591d cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 15922 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 15925 jne 0x1592c
            if fl.ne() { pc = 0x1592c; continue; }
            pc = 0x15927;
        }
        0x15927 => {
            // block 15927
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15927 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15927 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 1592a jmp 0x15930
            pc = 0x15930; continue;
        }
        0x1592c => {
            // block 1592c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1592c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1592c mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0x15930;
        }
        0x15930 => {
            // block 15930
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15930 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15930 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 15933 mov rdx,rax
            rdx = rax;
            // 15936 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // 1593a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 1593d je 0x1595e
            if fl.e() { pc = 0x1595e; continue; }
            pc = 0x1593f;
        }
        0x1593f => {
            // block 1593f
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1593f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1593f test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 15942 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 15946 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 1594b mov r12,rax
            r12 = rax;
            // 1594e call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15953 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 15955 mov rax,r12
            rax = r12;
            // 15958 je 0x1595e
            if fl.e() { pc = 0x1595e; continue; }
            pc = 0x1595a;
        }
        0x1595a => {
            // block 1595a
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595a js 0x15963
            if fl.s() { pc = 0x15963; continue; }
            pc = 0x1595c;
        }
        0x1595c => {
            // block 1595c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595c jmp 0x1598c
            pc = 0x1598c; continue;
        }
        0x1595e => {
            // block 1595e
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595e cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 15961 jae 0x1598c
            if fl.ae() { pc = 0x1598c; continue; }
            pc = 0x15963;
        }
        0x15963 => {
            // block 15963
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15963 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 15967 test rax,rax
            fl = logic::<64>(rax & rax);
            // 1596a jne 0x15879
            if fl.ne() { pc = 0x15879; continue; }
            pc = 0x15970;
        }
        0x15970 => {
            // block 15970
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15970 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15970 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15974 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 15977 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 1597b mov rax,r14
            rax = r14;
            // 1597e jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x15980 => {
            // block 15980
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15980 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15980 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 15984 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 15987 mov rax,rdi
            rax = rdi;
            // 1598a jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x1598c => {
            // block 1598c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1598c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1598c mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15990 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 15993 jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x15995 => {
            // block 15995
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15995 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15995 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15999 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // 1599c mov rax,rbx
            rax = rbx;
            pc = 0x1599f;
        }
        0x1599f => {
            // block 1599f
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1599f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1599f add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 159a3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 159a4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 159a6 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 159a8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 159aa pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 159ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 159ad ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x17062 size=416
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00017062<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x17062;
    loop {
        match pc {
        0x17062 => {
            // block 17062
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 17062 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17062 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 17063 mov rbp,rsp
            rbp = rsp;
            // 17066 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 17068 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1706a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1706c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1706e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1706f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 17070 mov r15,rdx
            r15 = rdx;
            // 17073 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 17076 mov rbx,rdi
            rbx = rdi;
            // 17079 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 1707b je 0x17086
            if fl.e() { pc = 0x17086; continue; }
            pc = 0x1707d;
        }
        0x1707d => {
            // block 1707d
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 1707d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1707d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17080 mov rdi,rbx
            rdi = rbx;
            // 17083 call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17086;
        }
        0x17086 => {
            // block 17086
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 17086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17086 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17089 mov rdi,rbx
            rdi = rbx;
            // 1708c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1708f mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 17092 mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            // 17095 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 17099 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // 1709c sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 170a0 movabs rdx,0xef7bdef7bdef7bdf
            rdx = 0xef7b_def7_bdef_7bdfu64;
            // 170aa imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // 170ae cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 170b1 jae 0x171f1
            if fl.ae() { pc = 0x171f1; continue; }
            pc = 0x170b7;
        }
        0x170b7 => {
            // block 170b7
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 170b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 170b7 imul r12,rcx,0xf8
            { let (r, f) = imul::<64>(rcx, 0xf8u64); r12 = r; fl = f; }
            // 170be lea r13,[rbx+r12*1]
            r13 = rbx.wrapping_add(r12);
            // 170c2 mov rax,QWORD PTR [rbx+r12*1+0x10]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x10)) as u64;
            // 170c7 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 170cb mov rax,QWORD PTR [rbx+r12*1]
            rax = m.r64(rbx.wrapping_add(r12)) as u64;
            // 170cf mov rcx,QWORD PTR [rbx+r12*1+0x8]
            rcx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x8)) as u64;
            // 170d4 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 170d8 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 170db lea rdi,[r15+0x18]
            rdi = r15.wrapping_add(0x18);
            // 170df lea rsi,[rbx+r12*1+0x18]
            rsi = rbx.wrapping_add(r12).wrapping_add(0x18);
            // 170e4 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 170e9 mov ax,WORD PTR [rbx+r12*1+0x5c]
            rax = (rax & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x5c)) as u64) as u16 as u64);
            // 170ef mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 170f4 mov eax,DWORD PTR [rbx+r12*1+0x58]
            rax = (m.r32(rbx.wrapping_add(r12).wrapping_add(0x58)) as u64) as u32 as u64;
            // 170f9 mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 170fd mov rax,QWORD PTR [rbx+r12*1+0x50]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x50)) as u64;
            // 17102 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 17106 mov rax,QWORD PTR [rbx+r12*1+0x48]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x48)) as u64;
            // 1710b mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 1710f mov rax,QWORD PTR [rbx+r12*1+0x40]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x40)) as u64;
            // 17114 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 17118 mov rax,QWORD PTR [rbx+r12*1+0x30]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x30)) as u64;
            // 1711d mov rcx,QWORD PTR [rbx+r12*1+0x38]
            rcx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x38)) as u64;
            // 17122 mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 17126 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 1712a cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 1712d je 0x171ac
            if fl.e() { pc = 0x171ac; continue; }
            pc = 0x1712f;
        }
        0x1712f => {
            // block 1712f
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 1712f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1712f lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 17133 mov rsi,QWORD PTR [rbx+r12*1+0x60]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0x60)) as u64;
            // 17138 mov rdx,QWORD PTR [rbx+r12*1+0x68]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x68)) as u64;
            // 1713d call 0xeba1a
            { let r = { let t = m.env.stub_va(0x1e7); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17142 lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 17146 mov rsi,QWORD PTR [rbx+r12*1+0x78]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0x78)) as u64;
            // 1714b mov rdx,QWORD PTR [rbx+r12*1+0x80]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x80)) as u64;
            // 17153 call 0xeba62
            { let r = { let t = m.env.stub_va(0x22f); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17158 lea rdi,[r15+0x90]
            rdi = r15.wrapping_add(0x90);
            // 1715f mov rsi,QWORD PTR [rbx+r12*1+0x90]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0x90)) as u64;
            // 17167 mov rdx,QWORD PTR [rbx+r12*1+0x98]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x98)) as u64;
            // 1716f call 0xeba3e
            { let r = { let t = m.env.stub_va(0x20b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17174 lea rdi,[r15+0xa8]
            rdi = r15.wrapping_add(0xa8);
            // 1717b mov rsi,QWORD PTR [rbx+r12*1+0xa8]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0xa8)) as u64;
            // 17183 mov rdx,QWORD PTR [rbx+r12*1+0xb0]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0xb0)) as u64;
            // 1718b call 0xeb9f6
            { let r = { let t = m.env.stub_va(0x1c3); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17190 lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 17197 mov rsi,QWORD PTR [rbx+r12*1+0xc0]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0xc0)) as u64;
            // 1719f mov rdx,QWORD PTR [rbx+r12*1+0xc8]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0xc8)) as u64;
            // 171a7 call 0xeb9f6
            { let r = { let t = m.env.stub_va(0x1c3); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x171ac;
        }
        0x171ac => {
            // block 171ac
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 171ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 171ac mov eax,DWORD PTR [rbx+r12*1+0xf0]
            rax = (m.r32(rbx.wrapping_add(r12).wrapping_add(0xf0)) as u64) as u32 as u64;
            // 171b4 mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 171bb mov rax,QWORD PTR [rbx+r12*1+0xe8]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0xe8)) as u64;
            // 171c3 mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 171ca mov rax,QWORD PTR [rbx+r12*1+0xd8]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0xd8)) as u64;
            // 171d2 mov rcx,QWORD PTR [rbx+r12*1+0xe0]
            rcx = m.r64(rbx.wrapping_add(r12).wrapping_add(0xe0)) as u64;
            // 171da mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 171e1 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 171e8 cmp DWORD PTR [r15+0x4],r14d
            fl = sub::<32>(m.r32(r15.wrapping_add(0x4)) as u64, (r14 as u32 as u64)).1;
            // 171ec sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 171ef jmp 0x171f3
            pc = 0x171f3; continue;
        }
        0x171f1 => {
            // block 171f1
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 171f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 171f1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x171f3;
        }
        0x171f3 => {
            // block 171f3
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 171f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 171f3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 171f7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 171f8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 171fa pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 171fc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 171fe pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 17200 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17201 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2c238 size=382
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002c238<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2c238;
    loop {
        match pc {
        0x2c238 => {
            // block 2c238
            #[cfg(feature = "trace")] eprintln!("f_0002c238 pc 2c238 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c238 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2c239 mov rbp,rsp
            rbp = rsp;
            // 2c23c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2c23e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2c23f sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 2c243 mov rbx,rsi
            rbx = rsi;
            // 2c246 mov r14,rdi
            r14 = rdi;
            // 2c249 mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 2c24b lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2c24f call 0x60623
            { let r = { let t = m.env.code_va(0x60623); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c254 lea rsi,[rip+0xfe1fb] # 0x12a456
            rsi = m.env.data_va(257059);
            // 2c25b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c260 mov rdi,r14
            rdi = r14;
            // 2c263 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c268 movzx edx,BYTE PTR [rbp-0x28]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 2c26c test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 2c26f jne 0x2c27a
            if fl.ne() { pc = 0x2c27a; continue; }
            pc = 0x2c271;
        }
        0x2c271 => {
            // block 2c271
            #[cfg(feature = "trace")] eprintln!("f_0002c238 pc 2c271 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c271 lea rsi,[rbp-0x27]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd9);
            // 2c275 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2c278 jmp 0x2c282
            pc = 0x2c282; continue;
        }
        0x2c27a => {
            // block 2c27a
            #[cfg(feature = "trace")] eprintln!("f_0002c238 pc 2c27a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c27a mov rdx,QWORD PTR [rbp-0x20]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 2c27e mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            pc = 0x2c282;
        }
        0x2c282 => {
            // block 2c282
            #[cfg(feature = "trace")] eprintln!("f_0002c238 pc 2c282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c282 mov rdi,rax
            rdi = rax;
            // 2c285 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c28a lea rsi,[rip+0xfe6f5] # 0x12a986
            rsi = m.env.data_va(258387);
            // 2c291 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c296 mov rdi,rax
            rdi = rax;
            // 2c299 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c29e mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 2c2a0 mov rdi,rax
            rdi = rax;
            // 2c2a3 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c2a8 lea rsi,[rip+0xfe6d9] # 0x12a988
            rsi = m.env.data_va(258389);
            // 2c2af mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 2c2b4 mov rdi,rax
            rdi = rax;
            // 2c2b7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c2bc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c2bf mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c2c3 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2c2c7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c2ca or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c2cd mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2c2d1 mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 2c2d4 mov rdi,rax
            rdi = rax;
            // 2c2d7 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c2dc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c2df mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c2e3 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2c2e7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c2ea or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c2ed mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2c2f1 lea rsi,[rip+0xfe699] # 0x12a991
            rsi = m.env.data_va(258398);
            // 2c2f8 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c2fd mov rdi,rax
            rdi = rax;
            // 2c300 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c305 mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 2c308 mov rdi,rax
            rdi = rax;
            // 2c30b call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c310 lea rsi,[rip+0xfe67c] # 0x12a993
            rsi = m.env.data_va(258400);
            // 2c317 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 2c31c mov rdi,rax
            rdi = rax;
            // 2c31f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c324 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c327 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c32b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2c32f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c332 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c335 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2c339 mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 2c33c mov rdi,rax
            rdi = rax;
            // 2c33f call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c344 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c347 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c34b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2c34f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c352 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c355 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2c359 lea rsi,[rip+0xfe63c] # 0x12a99c
            rsi = m.env.data_va(258409);
            // 2c360 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 2c365 mov rdi,rax
            rdi = rax;
            // 2c368 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c36d mov esi,DWORD PTR [rbx+0xc]
            rsi = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // 2c370 mov rdi,rax
            rdi = rax;
            // 2c373 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c378 lea rsi,[rip+0xfe0d5] # 0x12a454
            rsi = m.env.data_va(257057);
            // 2c37f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c384 mov rdi,rax
            rdi = rax;
            // 2c387 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c38c lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2c390 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c395 mov rax,r14
            rax = r14;
            // 2c398 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 2c39c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2c39d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2c39f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2c3a0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2c3a1 => {
            // block 2c3a1
            #[cfg(feature = "trace")] eprintln!("f_0002c238 pc 2c3a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c3a1 mov rbx,rax
            rbx = rax;
            // 2c3a4 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2c3a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c3ad mov rdi,rbx
            rdi = rbx;
            // 2c3b0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e5b4 size=442
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e5b4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e5b4;
    loop {
        match pc {
        0x2e5b4 => {
            // block 2e5b4
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5b4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e5b5 mov rbp,rsp
            rbp = rsp;
            // 2e5b8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e5ba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e5bc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2e5be push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e5c0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e5c1 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2e5c5 mov r15,rdx
            r15 = rdx;
            // 2e5c8 mov rbx,rsi
            rbx = rsi;
            // 2e5cb mov r13,rdi
            r13 = rdi;
            // 2e5ce test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2e5d1 je 0x2e5dd
            if fl.e() { pc = 0x2e5dd; continue; }
            pc = 0x2e5d3;
        }
        0x2e5d3 => {
            // block 2e5d3
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5d3 mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // 2e5d8 cmp BYTE PTR [rbx],0x0
            fl = sub::<8>(m.r8(rbx) as u64, 0x0u64).1;
            // 2e5db jne 0x2e5e2
            if fl.ne() { pc = 0x2e5e2; continue; }
            pc = 0x2e5dd;
        }
        0x2e5dd => {
            // block 2e5dd
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5dd mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            pc = 0x2e5e2;
        }
        0x2e5e2 => {
            // block 2e5e2
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5e2 mov DWORD PTR [rbp-0x54],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((rax as u32 as u64)) as u32);
            // 2e5e5 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 2e5e8 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2e5eb je 0x2e632
            if fl.e() { pc = 0x2e632; continue; }
            pc = 0x2e5ed;
        }
        0x2e5ed => {
            // block 2e5ed
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5ed cmp BYTE PTR [r15],0x0
            fl = sub::<8>(m.r8(r15) as u64, 0x0u64).1;
            // 2e5f1 je 0x2e632
            if fl.e() { pc = 0x2e632; continue; }
            pc = 0x2e5f3;
        }
        0x2e5f3 => {
            // block 2e5f3
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5f3 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2e5f6 je 0x2e6c8
            if fl.e() { pc = 0x2e6c8; continue; }
            pc = 0x2e5fc;
        }
        0x2e5fc => {
            // block 2e5fc
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5fc cmp BYTE PTR [rbx],0x0
            fl = sub::<8>(m.r8(rbx) as u64, 0x0u64).1;
            // 2e5ff je 0x2e6c8
            if fl.e() { pc = 0x2e6c8; continue; }
            pc = 0x2e605;
        }
        0x2e605 => {
            // block 2e605
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e605 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e605 lea rsi,[rip+0xfc574] # 0x12ab80
            rsi = m.env.data_va(258893);
            // 2e60c mov rdi,rbx
            rdi = rbx;
            // 2e60f call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e614 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e616 jne 0x2e62f
            if fl.ne() { pc = 0x2e62f; continue; }
            pc = 0x2e618;
        }
        0x2e618 => {
            // block 2e618
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e618 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e618 lea rsi,[rip+0xfc561] # 0x12ab80
            rsi = m.env.data_va(258893);
            // 2e61f mov rdi,r15
            rdi = r15;
            // 2e622 call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e627 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e629 je 0x2e6c8
            if fl.e() { pc = 0x2e6c8; continue; }
            pc = 0x2e62f;
        }
        0x2e62f => {
            // block 2e62f
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e62f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e62f xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x2e632;
        }
        0x2e632 => {
            // block 2e632
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e632 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e632 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 2e636 mov rdi,r13
            rdi = r13;
            // 2e639 mov rsi,rbx
            rsi = rbx;
            // 2e63c call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e642 mov r12,rax
            r12 = rax;
            pc = 0x2e645;
        }
        0x2e645 => {
            // block 2e645
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e645 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 2e648 je 0x2e753
            if fl.e() { pc = 0x2e753; continue; }
            pc = 0x2e64e;
        }
        0x2e64e => {
            // block 2e64e
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e64e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e64e lea rbx,[r13+0x28]
            rbx = r13.wrapping_add(0x28);
            // 2e652 lea rsi,[rip+0x10a66f] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 2e659 mov rdi,rbx
            rdi = rbx;
            // 2e65c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e661 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 2e664 je 0x2e67f
            if fl.e() { pc = 0x2e67f; continue; }
            pc = 0x2e666;
        }
        0x2e666 => {
            // block 2e666
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e666 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e666 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 2e66a mov rdi,r13
            rdi = r13;
            // 2e66d call QWORD PTR [rax+0x88]
            { let r = { let t = m.r64(rax.wrapping_add(0x88)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e673 mov DWORD PTR [rbp-0x54],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), (0x0u64) as u32);
            // 2e67a jmp 0x2e753
            pc = 0x2e753; continue;
        }
        0x2e67f => {
            // block 2e67f
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e67f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e67f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2e683 mov rdi,r12
            rdi = r12;
            // 2e686 mov rsi,r15
            rsi = r15;
            // 2e689 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e68c mov QWORD PTR [r13+0x48],rax
            m.w64(r13.wrapping_add(0x48), (rax) as u64);
            // 2e690 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 2e693 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2e696 je 0x2e745
            if fl.e() { pc = 0x2e745; continue; }
            pc = 0x2e69c;
        }
        0x2e69c => {
            // block 2e69c
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e69c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e69c mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 2e6a0 mov rdi,r12
            rdi = r12;
            // 2e6a3 mov rsi,rax
            rsi = rax;
            // 2e6a6 call QWORD PTR [rcx+0x40]
            { let r = { let t = m.r64(rcx.wrapping_add(0x40)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e6a9 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e6ab je 0x2e745
            if fl.e() { pc = 0x2e745; continue; }
            pc = 0x2e6b1;
        }
        0x2e6b1 => {
            // block 2e6b1
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6b1 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2e6b5 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 2e6b7 call 0xec11c
            { let r = { let t = m.env.stub_va(0x8e9); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e6bc test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // 2e6bf jne 0x2e6f7
            if fl.ne() { pc = 0x2e6f7; continue; }
            pc = 0x2e6c1;
        }
        0x2e6c1 => {
            // block 2e6c1
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6c1 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // 2e6c6 jmp 0x2e706
            pc = 0x2e706; continue;
        }
        0x2e6c8 => {
            // block 2e6c8
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6c8 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 2e6d0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 2e6d4 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2e6d8 mov rdi,r13
            rdi = r13;
            // 2e6db call QWORD PTR [rax+0x10]
            { let r = { let t = m.r64(rax.wrapping_add(0x10)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e6de test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e6e0 jle 0x2e753
            if fl.le() { pc = 0x2e753; continue; }
            pc = 0x2e6e2;
        }
        0x2e6e2 => {
            // block 2e6e2
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6e2 mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 2e6e6 cmp QWORD PTR [r13+0x18],rax
            fl = sub::<64>(m.r64(r13.wrapping_add(0x18)) as u64, rax).1;
            // 2e6ea je 0x2e753
            if fl.e() { pc = 0x2e753; continue; }
            pc = 0x2e6ec;
        }
        0x2e6ec => {
            // block 2e6ec
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6ec mov r12,QWORD PTR [rax]
            r12 = m.r64(rax) as u64;
            // 2e6ef xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 2e6f2 jmp 0x2e645
            pc = 0x2e645; continue;
        }
        0x2e6f7 => {
            // block 2e6f7
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6f7 mov rax,QWORD PTR [r13+0x38]
            rax = m.r64(r13.wrapping_add(0x38)) as u64;
            // 2e6fb mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 2e6fe mov QWORD PTR [r13+0x30],0x0
            m.w64(r13.wrapping_add(0x30), (0x0u64) as u64);
            pc = 0x2e706;
        }
        0x2e706 => {
            // block 2e706
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e706 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e706 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2e708 mov rdi,rbx
            rdi = rbx;
            // 2e70b call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e710 mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2e714 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 2e718 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 2e71c mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2e720 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 2e724 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 2e727 xorps xmm0,xmm0
            x0 = 0;
            // 2e72a movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 2e72e mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 2e736 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2e73a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e73f mov r14d,0x7d0
            r14 = (0x7d0u64) as u32 as u64;
            pc = 0x2e745;
        }
        0x2e745 => {
            // block 2e745
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e745 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e745 mov DWORD PTR [rbp-0x54],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((r14 as u32 as u64)) as u32);
            // 2e749 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2e74d mov rdi,r12
            rdi = r12;
            // 2e750 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e753;
        }
        0x2e753 => {
            // block 2e753
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e753 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e753 mov eax,DWORD PTR [rbp-0x54]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64) as u32 as u64;
            // 2e756 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2e75a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e75b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e75d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2e75f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e761 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e763 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e764 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2e765 => {
            // block 2e765
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e765 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e765 mov rdi,rax
            rdi = rax;
            // 2e768 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e7ee size=396
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e7ee<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e7ee;
    loop {
        match pc {
        0x2e7ee => {
            // block 2e7ee
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e7ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7ee push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e7ef mov rbp,rsp
            rbp = rsp;
            // 2e7f2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e7f4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e7f6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2e7f8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e7fa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e7fb sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 2e7ff mov r14,rcx
            r14 = rcx;
            // 2e802 mov r13d,edx
            r13 = ((rdx as u32 as u64)) as u32 as u64;
            // 2e805 mov r12,rsi
            r12 = rsi;
            // 2e808 mov r15,rdi
            r15 = rdi;
            // 2e80b mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 2e810 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2e813 je 0x2e955
            if fl.e() { pc = 0x2e955; continue; }
            pc = 0x2e819;
        }
        0x2e819 => {
            // block 2e819
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e819 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e819 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e81c mov rdi,r15
            rdi = r15;
            // 2e81f mov rsi,r12
            rsi = r12;
            // 2e822 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e828 mov ebx,0x2
            rbx = (0x2u64) as u32 as u64;
            // 2e82d test rax,rax
            fl = logic::<64>(rax & rax);
            // 2e830 je 0x2e955
            if fl.e() { pc = 0x2e955; continue; }
            pc = 0x2e836;
        }
        0x2e836 => {
            // block 2e836
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e836 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e836 mov QWORD PTR [rbp-0x48],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r14) as u64);
            // 2e83a mov rdi,r12
            rdi = r12;
            // 2e83d call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e842 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2e846 mov rdi,rbx
            rdi = rbx;
            // 2e849 mov rsi,r12
            rsi = r12;
            // 2e84c mov rdx,rax
            rdx = rax;
            // 2e84f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e854 lea rsi,[rip+0xfc32d] # 0x12ab88
            rsi = m.env.data_va(258901);
            // 2e85b mov rdi,rbx
            rdi = rbx;
            // 2e85e call 0xebff6
            { let r = { let t = m.env.stub_va(0x7c3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e863 cmp r13d,0x4
            fl = sub::<32>((r13 as u32 as u64), 0x4u64).1;
            // 2e867 jae 0x2e8b8
            if fl.ae() { pc = 0x2e8b8; continue; }
            pc = 0x2e869;
        }
        0x2e869 => {
            // block 2e869
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e869 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e869 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e86c mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // 2e870 mov rdi,r15
            rdi = r15;
            // 2e873 mov rsi,r12
            rsi = r12;
            // 2e876 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e878 mov DWORD PTR [rbp-0x4c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((rax as u32 as u64)) as u32);
            // 2e87b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e87e mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // 2e882 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 2e885 mov rdi,r15
            rdi = r15;
            // 2e888 mov rsi,r12
            rsi = r12;
            // 2e88b call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e88d mov DWORD PTR [rbp-0x50],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), ((rax as u32 as u64)) as u32);
            // 2e890 movsxd rax,r13d
            rax = sx::<32>((r13 as u32 as u64)) as u64;
            // 2e893 lea rcx,[rip+0xd1516] # 0xffdb0
            rcx = m.env.data_va(83325);
            // 2e89a mov r13d,DWORD PTR [rcx+rax*4]
            r13 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 2e89e lea rax,[rip+0xd149b] # 0xffd40
            rax = m.env.data_va(83213);
            pc = 0x2e8a5;
        }
        0x2e8a5 => {
            // block 2e8a5
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8a5 cmp DWORD PTR [rax+r14*8],r13d
            fl = sub::<32>(m.r32(rax.wrapping_add(r14.wrapping_mul(8))) as u64, (r13 as u32 as u64)).1;
            // 2e8a9 je 0x2e8cb
            if fl.e() { pc = 0x2e8cb; continue; }
            pc = 0x2e8ab;
        }
        0x2e8ab => {
            // block 2e8ab
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8ab inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 2e8ae xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 2e8b0 cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // 2e8b4 jbe 0x2e8a5
            if fl.be() { pc = 0x2e8a5; continue; }
            pc = 0x2e8b6;
        }
        0x2e8b6 => {
            // block 2e8b6
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8b6 jmp 0x2e8d0
            pc = 0x2e8d0; continue;
        }
        0x2e8b8 => {
            // block 2e8b8
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8b8 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2e8bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e8c1 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 2e8c6 jmp 0x2e955
            pc = 0x2e955; continue;
        }
        0x2e8cb => {
            // block 2e8cb
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8cb mov ebx,DWORD PTR [rax+r14*8+0x4]
            rbx = (m.r32(rax.wrapping_add(r14.wrapping_mul(8)).wrapping_add(0x4)) as u64) as u32 as u64;
            pc = 0x2e8d0;
        }
        0x2e8d0 => {
            // block 2e8d0
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e8d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e8d0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e8d3 mov rax,QWORD PTR [rax+0x58]
            rax = m.r64(rax.wrapping_add(0x58)) as u64;
            // 2e8d7 mov rdi,r15
            rdi = r15;
            // 2e8da mov rsi,r12
            rsi = r12;
            // 2e8dd call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e8df mov edx,DWORD PTR [rbp-0x4c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u32 as u64;
            // 2e8e2 imul ebx,edx
            { let (r, f) = imul::<32>((rbx as u32 as u64), (rdx as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // 2e8e5 mov r8d,ebx
            r8 = ((rbx as u32 as u64)) as u32 as u64;
            // 2e8e8 sar r8d,0x1f
            { let (r, f) = sar::<32>((r8 as u32 as u64), 0x1fu64, fl); r8 = (r) as u32 as u64; fl = f; }
            // 2e8ec shr r8d,0x1d
            { let (r, f) = shr::<32>((r8 as u32 as u64), 0x1du64, fl); r8 = (r) as u32 as u64; fl = f; }
            // 2e8f0 add r8d,ebx
            { let (r, f) = add::<32>((r8 as u32 as u64), (rbx as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2e8f3 sar r8d,0x3
            { let (r, f) = sar::<32>((r8 as u32 as u64), 0x3u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // 2e8f7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e8fa mov rax,QWORD PTR [rax+0x60]
            rax = m.r64(rax.wrapping_add(0x60)) as u64;
            // 2e8fe test BYTE PTR [rbp-0x40],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64 & 0x1u64);
            // 2e902 jne 0x2e90a
            if fl.ne() { pc = 0x2e90a; continue; }
            pc = 0x2e904;
        }
        0x2e904 => {
            // block 2e904
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e904 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e904 lea r9,[rbp-0x3f]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 2e908 jmp 0x2e90e
            pc = 0x2e90e; continue;
        }
        0x2e90a => {
            // block 2e90a
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e90a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e90a mov r9,QWORD PTR [rbp-0x30]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x2e90e;
        }
        0x2e90e => {
            // block 2e90e
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e90e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e90e mov DWORD PTR [rsp+0x8],r13d
            m.w32(rsp.wrapping_add(0x8), ((r13 as u32 as u64)) as u32);
            // 2e913 mov DWORD PTR [rsp+0x18],0x0
            m.w32(rsp.wrapping_add(0x18), (0x0u64) as u32);
            // 2e91b mov DWORD PTR [rsp+0x10],0x0
            m.w32(rsp.wrapping_add(0x10), (0x0u64) as u32);
            // 2e923 mov DWORD PTR [rsp],0x1
            m.w32(rsp, (0x1u64) as u32);
            // 2e92a movsd xmm1,QWORD PTR [rip+0xd1256] # 0xffb88
            x1 = m.r64(m.env.data_va(82773)) as u128;
            // 2e932 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2e934 mov rdi,r15
            rdi = r15;
            // 2e937 mov ecx,DWORD PTR [rbp-0x50]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64) as u32 as u64;
            // 2e93a call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e93c mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2e940 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 2e943 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2e946 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 2e949 movzx ebx,al
            rbx = ((rax as u8 as u64)) as u32 as u64;
            // 2e94c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2e950 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2e955;
        }
        0x2e955 => {
            // block 2e955
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e955 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 2e957 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 2e95b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e95c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e95e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2e960 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e962 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e964 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e965 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2e966 => {
            // block 2e966
            #[cfg(feature = "trace")] eprintln!("f_0002e7ee pc 2e966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e966 mov rbx,rax
            rbx = rax;
            // 2e969 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2e96d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e972 mov rdi,rbx
            rdi = rbx;
            // 2e975 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f626 size=428
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f626<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2f626;
    loop {
        match pc {
        0x2f626 => {
            // block 2f626
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f626 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f626 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f627 mov rbp,rsp
            rbp = rsp;
            // 2f62a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f62c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f62d sub rsp,0x90
            { let (r, f) = sub::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // 2f634 mov r14,rsi
            r14 = rsi;
            // 2f637 mov rbx,rdi
            rbx = rdi;
            // 2f63a xorps xmm0,xmm0
            x0 = 0;
            // 2f63d movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // 2f641 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 2f649 mov BYTE PTR [rbp-0x31],0x5
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), (0x5u64) as u8);
            // 2f64d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2f651 lea rsi,[rbp-0x31]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            // 2f655 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f65a xorps xmm0,xmm0
            x0 = 0;
            // 2f65d movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 2f661 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 2f669 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // 2f66d mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 2f675 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 2f67c lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2f680 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f685 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 2f68c lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2f690 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f695 lea rsi,[rbp-0x88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 2f69c lea rdx,[rbp-0xa0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 2f6a3 lea rcx,[rbp-0x70]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 2f6a7 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 2f6ad mov rdi,rbx
            rdi = rbx;
            // 2f6b0 call 0x2fdd8
            { let r = { let t = m.env.code_va(0x2fdd8); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f6b5 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2f6b7 mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 2f6be test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f6c1 je 0x2f6d8
            if fl.e() { pc = 0x2f6d8; continue; }
            pc = 0x2f6c3;
        }
        0x2f6c3 => {
            // block 2f6c3
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6c3 cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 2f6ca je 0x2f6d3
            if fl.e() { pc = 0x2f6d3; continue; }
            pc = 0x2f6cc;
        }
        0x2f6cc => {
            // block 2f6cc
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6cc mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x2f6d3;
        }
        0x2f6d3 => {
            // block 2f6d3
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6d3 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f6d8;
        }
        0x2f6d8 => {
            // block 2f6d8
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6d8 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 2f6df test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f6e2 je 0x2f6f3
            if fl.e() { pc = 0x2f6f3; continue; }
            pc = 0x2f6e4;
        }
        0x2f6e4 => {
            // block 2f6e4
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6e4 cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 2f6e8 je 0x2f6ee
            if fl.e() { pc = 0x2f6ee; continue; }
            pc = 0x2f6ea;
        }
        0x2f6ea => {
            // block 2f6ea
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6ea mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x2f6ee;
        }
        0x2f6ee => {
            // block 2f6ee
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6ee call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f6f3;
        }
        0x2f6f3 => {
            // block 2f6f3
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6f3 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // 2f6f5 je 0x2f70b
            if fl.e() { pc = 0x2f70b; continue; }
            pc = 0x2f6f7;
        }
        0x2f6f7 => {
            // block 2f6f7
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f6f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f6f7 mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 2f6fb mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 2f6ff cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 2f702 je 0x2f70f
            if fl.e() { pc = 0x2f70f; continue; }
            pc = 0x2f704;
        }
        0x2f704 => {
            // block 2f704
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f704 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f704 mov al,BYTE PTR [rdi]
            rax = (rax & !0xff) | ((m.r8(rdi) as u64) as u8 as u64);
            // 2f706 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // 2f709 jmp 0x2f717
            pc = 0x2f717; continue;
        }
        0x2f70b => {
            // block 2f70b
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f70b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f70b mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x2f70f;
        }
        0x2f70f => {
            // block 2f70f
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f70f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f70f test rax,rax
            fl = logic::<64>(rax & rax);
            // 2f712 mov rdi,rax
            rdi = rax;
            // 2f715 je 0x2f726
            if fl.e() { pc = 0x2f726; continue; }
            pc = 0x2f717;
        }
        0x2f717 => {
            // block 2f717
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f717 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f717 cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 2f71b je 0x2f721
            if fl.e() { pc = 0x2f721; continue; }
            pc = 0x2f71d;
        }
        0x2f71d => {
            // block 2f71d
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f71d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f71d mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x2f721;
        }
        0x2f721 => {
            // block 2f721
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f721 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f721 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f726;
        }
        0x2f726 => {
            // block 2f726
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f726 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 2f72a test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f72d je 0x2f73e
            if fl.e() { pc = 0x2f73e; continue; }
            pc = 0x2f72f;
        }
        0x2f72f => {
            // block 2f72f
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f72f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f72f cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 2f733 je 0x2f739
            if fl.e() { pc = 0x2f739; continue; }
            pc = 0x2f735;
        }
        0x2f735 => {
            // block 2f735
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f735 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f735 mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x2f739;
        }
        0x2f739 => {
            // block 2f739
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f739 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f739 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f73e;
        }
        0x2f73e => {
            // block 2f73e
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f73e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f73e mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 2f740 add rsp,0x90
            { let (r, f) = add::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // 2f747 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f748 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f74a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f74b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2f74c => {
            // block 2f74c
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f74c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f74c mov rbx,rax
            rbx = rax;
            // 2f74f jmp 0x2f7b2
            pc = 0x2f7b2; continue;
        }
        0x2f751 => {
            // block 2f751
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f751 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f751 mov rbx,rax
            rbx = rax;
            // 2f754 jmp 0x2f79a
            pc = 0x2f79a; continue;
        }
        0x2f756 => {
            // block 2f756
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f756 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f756 mov rbx,rax
            rbx = rax;
            // 2f759 jmp 0x2f77f
            pc = 0x2f77f; continue;
        }
        0x2f75b => {
            // block 2f75b
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f75b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f75b mov rbx,rax
            rbx = rax;
            // 2f75e mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 2f765 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f768 je 0x2f77f
            if fl.e() { pc = 0x2f77f; continue; }
            pc = 0x2f76a;
        }
        0x2f76a => {
            // block 2f76a
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f76a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f76a cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 2f771 je 0x2f77a
            if fl.e() { pc = 0x2f77a; continue; }
            pc = 0x2f773;
        }
        0x2f773 => {
            // block 2f773
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f773 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f773 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x2f77a;
        }
        0x2f77a => {
            // block 2f77a
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f77a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f77a call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f77f;
        }
        0x2f77f => {
            // block 2f77f
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f77f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f77f mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 2f786 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f789 je 0x2f79a
            if fl.e() { pc = 0x2f79a; continue; }
            pc = 0x2f78b;
        }
        0x2f78b => {
            // block 2f78b
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f78b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f78b cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 2f78f je 0x2f795
            if fl.e() { pc = 0x2f795; continue; }
            pc = 0x2f791;
        }
        0x2f791 => {
            // block 2f791
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f791 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f791 mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x2f795;
        }
        0x2f795 => {
            // block 2f795
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f795 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f795 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f79a;
        }
        0x2f79a => {
            // block 2f79a
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f79a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f79a mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 2f79e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f7a1 je 0x2f7b2
            if fl.e() { pc = 0x2f7b2; continue; }
            pc = 0x2f7a3;
        }
        0x2f7a3 => {
            // block 2f7a3
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7a3 cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 2f7a7 je 0x2f7ad
            if fl.e() { pc = 0x2f7ad; continue; }
            pc = 0x2f7a9;
        }
        0x2f7a9 => {
            // block 2f7a9
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7a9 mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x2f7ad;
        }
        0x2f7ad => {
            // block 2f7ad
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7ad call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f7b2;
        }
        0x2f7b2 => {
            // block 2f7b2
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7b2 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 2f7b6 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f7b9 je 0x2f7ca
            if fl.e() { pc = 0x2f7ca; continue; }
            pc = 0x2f7bb;
        }
        0x2f7bb => {
            // block 2f7bb
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7bb cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 2f7bf je 0x2f7c5
            if fl.e() { pc = 0x2f7c5; continue; }
            pc = 0x2f7c1;
        }
        0x2f7c1 => {
            // block 2f7c1
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7c1 mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x2f7c5;
        }
        0x2f7c5 => {
            // block 2f7c5
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7c5 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f7ca;
        }
        0x2f7ca => {
            // block 2f7ca
            #[cfg(feature = "trace")] eprintln!("f_0002f626 pc 2f7ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f7ca mov rdi,rbx
            rdi = rbx;
            // 2f7cd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x311bc size=428
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000311bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x311bc;
    loop {
        match pc {
        0x311bc => {
            // block 311bc
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 311bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 311bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 311bd mov rbp,rsp
            rbp = rsp;
            // 311c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 311c2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 311c3 sub rsp,0x90
            { let (r, f) = sub::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // 311ca mov r14,rsi
            r14 = rsi;
            // 311cd mov rbx,rdi
            rbx = rdi;
            // 311d0 xorps xmm0,xmm0
            x0 = 0;
            // 311d3 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // 311d7 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 311df mov BYTE PTR [rbp-0x31],0x16
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), (0x16u64) as u8);
            // 311e3 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 311e7 lea rsi,[rbp-0x31]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            // 311eb call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 311f0 xorps xmm0,xmm0
            x0 = 0;
            // 311f3 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 311f7 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 311ff movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // 31203 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 3120b lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 31212 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 31216 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3121b lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 31222 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 31226 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3122b lea rsi,[rbp-0x88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 31232 lea rdx,[rbp-0xa0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 31239 lea rcx,[rbp-0x70]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 3123d mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 31243 mov rdi,rbx
            rdi = rbx;
            // 31246 call 0x2fdd8
            { let r = { let t = m.env.code_va(0x2fdd8); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3124b mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 3124d mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 31254 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31257 je 0x3126e
            if fl.e() { pc = 0x3126e; continue; }
            pc = 0x31259;
        }
        0x31259 => {
            // block 31259
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31259 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31259 cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 31260 je 0x31269
            if fl.e() { pc = 0x31269; continue; }
            pc = 0x31262;
        }
        0x31262 => {
            // block 31262
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31262 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31262 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x31269;
        }
        0x31269 => {
            // block 31269
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31269 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x3126e;
        }
        0x3126e => {
            // block 3126e
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3126e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3126e mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 31275 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31278 je 0x31289
            if fl.e() { pc = 0x31289; continue; }
            pc = 0x3127a;
        }
        0x3127a => {
            // block 3127a
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3127a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3127a cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 3127e je 0x31284
            if fl.e() { pc = 0x31284; continue; }
            pc = 0x31280;
        }
        0x31280 => {
            // block 31280
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31280 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31280 mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x31284;
        }
        0x31284 => {
            // block 31284
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31284 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31284 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31289;
        }
        0x31289 => {
            // block 31289
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31289 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31289 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // 3128b je 0x312a1
            if fl.e() { pc = 0x312a1; continue; }
            pc = 0x3128d;
        }
        0x3128d => {
            // block 3128d
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3128d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3128d mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 31291 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 31295 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 31298 je 0x312a5
            if fl.e() { pc = 0x312a5; continue; }
            pc = 0x3129a;
        }
        0x3129a => {
            // block 3129a
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3129a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3129a mov al,BYTE PTR [rdi]
            rax = (rax & !0xff) | ((m.r8(rdi) as u64) as u8 as u64);
            // 3129c mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // 3129f jmp 0x312ad
            pc = 0x312ad; continue;
        }
        0x312a1 => {
            // block 312a1
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312a1 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x312a5;
        }
        0x312a5 => {
            // block 312a5
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312a5 test rax,rax
            fl = logic::<64>(rax & rax);
            // 312a8 mov rdi,rax
            rdi = rax;
            // 312ab je 0x312bc
            if fl.e() { pc = 0x312bc; continue; }
            pc = 0x312ad;
        }
        0x312ad => {
            // block 312ad
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312ad cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 312b1 je 0x312b7
            if fl.e() { pc = 0x312b7; continue; }
            pc = 0x312b3;
        }
        0x312b3 => {
            // block 312b3
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312b3 mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x312b7;
        }
        0x312b7 => {
            // block 312b7
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312b7 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x312bc;
        }
        0x312bc => {
            // block 312bc
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312bc mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 312c0 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 312c3 je 0x312d4
            if fl.e() { pc = 0x312d4; continue; }
            pc = 0x312c5;
        }
        0x312c5 => {
            // block 312c5
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312c5 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 312c9 je 0x312cf
            if fl.e() { pc = 0x312cf; continue; }
            pc = 0x312cb;
        }
        0x312cb => {
            // block 312cb
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312cb mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x312cf;
        }
        0x312cf => {
            // block 312cf
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312cf call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x312d4;
        }
        0x312d4 => {
            // block 312d4
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312d4 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 312d6 add rsp,0x90
            { let (r, f) = add::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // 312dd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 312de pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 312e0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 312e1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x312e2 => {
            // block 312e2
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312e2 mov rbx,rax
            rbx = rax;
            // 312e5 jmp 0x31348
            pc = 0x31348; continue;
        }
        0x312e7 => {
            // block 312e7
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312e7 mov rbx,rax
            rbx = rax;
            // 312ea jmp 0x31330
            pc = 0x31330; continue;
        }
        0x312ec => {
            // block 312ec
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312ec mov rbx,rax
            rbx = rax;
            // 312ef jmp 0x31315
            pc = 0x31315; continue;
        }
        0x312f1 => {
            // block 312f1
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312f1 mov rbx,rax
            rbx = rax;
            // 312f4 mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 312fb test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 312fe je 0x31315
            if fl.e() { pc = 0x31315; continue; }
            pc = 0x31300;
        }
        0x31300 => {
            // block 31300
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31300 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31300 cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 31307 je 0x31310
            if fl.e() { pc = 0x31310; continue; }
            pc = 0x31309;
        }
        0x31309 => {
            // block 31309
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31309 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31309 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x31310;
        }
        0x31310 => {
            // block 31310
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31310 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31310 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31315;
        }
        0x31315 => {
            // block 31315
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31315 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31315 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 3131c test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3131f je 0x31330
            if fl.e() { pc = 0x31330; continue; }
            pc = 0x31321;
        }
        0x31321 => {
            // block 31321
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31321 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31321 cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 31325 je 0x3132b
            if fl.e() { pc = 0x3132b; continue; }
            pc = 0x31327;
        }
        0x31327 => {
            // block 31327
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31327 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31327 mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x3132b;
        }
        0x3132b => {
            // block 3132b
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3132b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3132b call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31330;
        }
        0x31330 => {
            // block 31330
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31330 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31330 mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 31334 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31337 je 0x31348
            if fl.e() { pc = 0x31348; continue; }
            pc = 0x31339;
        }
        0x31339 => {
            // block 31339
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31339 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31339 cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 3133d je 0x31343
            if fl.e() { pc = 0x31343; continue; }
            pc = 0x3133f;
        }
        0x3133f => {
            // block 3133f
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3133f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3133f mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x31343;
        }
        0x31343 => {
            // block 31343
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31343 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31343 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31348;
        }
        0x31348 => {
            // block 31348
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31348 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31348 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 3134c test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3134f je 0x31360
            if fl.e() { pc = 0x31360; continue; }
            pc = 0x31351;
        }
        0x31351 => {
            // block 31351
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31351 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31351 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 31355 je 0x3135b
            if fl.e() { pc = 0x3135b; continue; }
            pc = 0x31357;
        }
        0x31357 => {
            // block 31357
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31357 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31357 mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x3135b;
        }
        0x3135b => {
            // block 3135b
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3135b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3135b call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31360;
        }
        0x31360 => {
            // block 31360
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31360 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31360 mov rdi,rbx
            rdi = rbx;
            // 31363 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x323b2 size=454
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000323b2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x323b2;
    loop {
        match pc {
        0x323b2 => {
            // block 323b2
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 323b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 323b2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 323b3 mov rbp,rsp
            rbp = rsp;
            // 323b6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 323b7 mov QWORD PTR [rdi+0x8],0x0
            m.w64(rdi.wrapping_add(0x8), (0x0u64) as u64);
            // 323bf mov QWORD PTR [rdi],0x0
            m.w64(rdi, (0x0u64) as u64);
            // 323c6 mov DWORD PTR [rdi+0x10],0xc
            m.w32(rdi.wrapping_add(0x10), (0xcu64) as u32);
            // 323cd mov DWORD PTR [rdi+0x14],0x0
            m.w32(rdi.wrapping_add(0x14), (0x0u64) as u32);
            // 323d4 mov DWORD PTR [rdi+0x18],0x20
            m.w32(rdi.wrapping_add(0x18), (0x20u64) as u32);
            // 323db mov DWORD PTR [rdi+0x1c],0x3
            m.w32(rdi.wrapping_add(0x1c), (0x3u64) as u32);
            // 323e2 mov BYTE PTR [rdi+0x20],0x0
            m.w8(rdi.wrapping_add(0x20), (0x0u64) as u8);
            // 323e6 mov WORD PTR [rdi+0x34],0x0
            m.w16(rdi.wrapping_add(0x34), (0x0u64) as u16);
            // 323ec mov QWORD PTR [rdi+0x2c],0x0
            m.w64(rdi.wrapping_add(0x2c), (0x0u64) as u64);
            // 323f4 mov QWORD PTR [rdi+0x24],0x0
            m.w64(rdi.wrapping_add(0x24), (0x0u64) as u64);
            // 323fc mov DWORD PTR [rdi+0x38],0x12
            m.w32(rdi.wrapping_add(0x38), (0x12u64) as u32);
            // 32403 cmp esi,0xb
            fl = sub::<32>((rsi as u32 as u64), 0xbu64).1;
            // 32406 ja 0x32555
            if fl.a() { pc = 0x32555; continue; }
            pc = 0x3240c;
        }
        0x3240c => {
            // block 3240c
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3240c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3240c cmp edx,0x1f
            fl = sub::<32>((rdx as u32 as u64), 0x1fu64).1;
            // 3240f ja 0x32555
            if fl.a() { pc = 0x32555; continue; }
            pc = 0x32415;
        }
        0x32415 => {
            // block 32415
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32415 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32415 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 32418 ja 0x32555
            if fl.a() { pc = 0x32555; continue; }
            pc = 0x3241e;
        }
        0x3241e => {
            // block 3241e
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3241e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3241e cmp edx,0x17
            fl = sub::<32>((rdx as u32 as u64), 0x17u64).1;
            // 32421 sete r11b
            r11 = (r11 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32425 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 32427 and eax,0xffffffef
            { let r = (rax as u32 as u64) & 0xffffffefu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3242a cmp eax,0xa
            fl = sub::<32>((rax as u32 as u64), 0xau64).1;
            // 3242d sete r8b
            r8 = (r8 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32431 lea eax,[rdx-0x1b]
            rax = (rdx.wrapping_add(0xffff_ffff_ffff_ffe5)) as u32 as u64;
            // 32434 cmp eax,0x5
            fl = sub::<32>((rax as u32 as u64), 0x5u64).1;
            // 32437 setb r9b
            r9 = (r9 & !0xff) | (((fl.b() as u64)) as u8 as u64);
            // 3243b lea r10d,[rcx-0x1]
            r10 = (rcx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 3243f cmp r10d,0x1
            fl = sub::<32>((r10 as u32 as u64), 0x1u64).1;
            // 32443 ja 0x32451
            if fl.a() { pc = 0x32451; continue; }
            pc = 0x32445;
        }
        0x32445 => {
            // block 32445
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32445 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32445 or r11b,r8b
            { let r = (r11 as u8 as u64) | (r8 as u8 as u64); let f = logic::<8>(r);
              r11 = (r11 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32448 or r9b,r11b
            { let r = (r9 as u8 as u64) | (r11 as u8 as u64); let f = logic::<8>(r);
              r9 = (r9 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3244b jne 0x32555
            if fl.ne() { pc = 0x32555; continue; }
            pc = 0x32451;
        }
        0x32451 => {
            // block 32451
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32451 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32451 mov r8d,edx
            r8 = ((rdx as u32 as u64)) as u32 as u64;
            // 32454 mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 32457 imul r11,r9,0x780
            { let (r, f) = imul::<64>(r9, 0x780u64); r11 = r; fl = f; }
            // 3245e lea rbx,[rip+0x19020b] # 0x1c2670
            rbx = m.env.data_va(880189);
            // 32465 add rbx,r11
            { let (r, f) = add::<64>(rbx, r11);
              rbx = r;
              fl = f; }
            // 32468 imul r8,r8,0x3c
            { let (r, f) = imul::<64>(r8, 0x3cu64); r8 = r; fl = f; }
            // 3246c mov eax,DWORD PTR [r8+rbx*1+0x38]
            rax = (m.r32(r8.wrapping_add(rbx).wrapping_add(0x38)) as u64) as u32 as u64;
            // 32471 mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32474 mov rax,QWORD PTR [r8+rbx*1+0x30]
            rax = m.r64(r8.wrapping_add(rbx).wrapping_add(0x30)) as u64;
            // 32479 mov QWORD PTR [rdi+0x30],rax
            m.w64(rdi.wrapping_add(0x30), (rax) as u64);
            // 3247d mov rax,QWORD PTR [r8+rbx*1+0x28]
            rax = m.r64(r8.wrapping_add(rbx).wrapping_add(0x28)) as u64;
            // 32482 mov QWORD PTR [rdi+0x28],rax
            m.w64(rdi.wrapping_add(0x28), (rax) as u64);
            // 32486 mov rax,QWORD PTR [r8+rbx*1+0x20]
            rax = m.r64(r8.wrapping_add(rbx).wrapping_add(0x20)) as u64;
            // 3248b mov QWORD PTR [rdi+0x20],rax
            m.w64(rdi.wrapping_add(0x20), (rax) as u64);
            // 3248f mov rax,QWORD PTR [r8+rbx*1+0x18]
            rax = m.r64(r8.wrapping_add(rbx).wrapping_add(0x18)) as u64;
            // 32494 mov QWORD PTR [rdi+0x18],rax
            m.w64(rdi.wrapping_add(0x18), (rax) as u64);
            // 32498 mov rax,QWORD PTR [r8+rbx*1+0x10]
            rax = m.r64(r8.wrapping_add(rbx).wrapping_add(0x10)) as u64;
            // 3249d mov QWORD PTR [rdi+0x10],rax
            m.w64(rdi.wrapping_add(0x10), (rax) as u64);
            // 324a1 mov rax,QWORD PTR [r8+rbx*1]
            rax = m.r64(r8.wrapping_add(rbx)) as u64;
            // 324a5 mov rbx,QWORD PTR [r8+rbx*1+0x8]
            rbx = m.r64(r8.wrapping_add(rbx).wrapping_add(0x8)) as u64;
            // 324aa mov QWORD PTR [rdi+0x8],rbx
            m.w64(rdi.wrapping_add(0x8), (rbx) as u64);
            // 324ae mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 324b1 mov DWORD PTR [rdi+0x10],esi
            m.w32(rdi.wrapping_add(0x10), ((rsi as u32 as u64)) as u32);
            // 324b4 mov DWORD PTR [rdi+0x18],edx
            m.w32(rdi.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // 324b7 mov DWORD PTR [rdi+0x1c],ecx
            m.w32(rdi.wrapping_add(0x1c), ((rcx as u32 as u64)) as u32);
            // 324ba mov r8d,DWORD PTR [rdi]
            r8 = (m.r32(rdi) as u64) as u32 as u64;
            // 324bd cmp r10d,0x1
            fl = sub::<32>((r10 as u32 as u64), 0x1u64).1;
            // 324c1 ja 0x3253a
            if fl.a() { pc = 0x3253a; continue; }
            pc = 0x324c3;
        }
        0x324c3 => {
            // block 324c3
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 324c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 324c3 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 324c6 jbe 0x324d0
            if fl.be() { pc = 0x324d0; continue; }
            pc = 0x324c8;
        }
        0x324c8 => {
            // block 324c8
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 324c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 324c8 mov DWORD PTR [rdi],r8d
            m.w32(rdi, ((r8 as u32 as u64)) as u32);
            // 324cb mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // 324ce jmp 0x3252f
            pc = 0x3252f; continue;
        }
        0x324d0 => {
            // block 324d0
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 324d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 324d0 lea rax,[rip+0x81] # 0x32558
            rax = m.env.code_va(0x32558);
            // 324d7 movsxd rsi,DWORD PTR [rax+r9*4]
            rsi = sx::<32>(m.r32(rax.wrapping_add(r9.wrapping_mul(4))) as u64) as u64;
            // 324db add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 324de jmp rsi
            pc = (rsi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x324e0 => {
            // block 324e0
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 324e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 324e0 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 324e3 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 324e6 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 324e9 lea eax,[rax+rax*1+0x458]
            rax = (rax.wrapping_add(rax).wrapping_add(0x458)) as u32 as u64;
            // 324f0 mov DWORD PTR [rdi],eax
            m.w32(rdi, ((rax as u32 as u64)) as u32);
            // 324f2 jmp 0x3252f
            pc = 0x3252f; continue;
        }
        0x324f4 => {
            // block 324f4
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 324f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 324f4 mov DWORD PTR [rdi],0x2e4
            m.w32(rdi, (0x2e4u64) as u32);
            // 324fa mov eax,0x2e4
            rax = (0x2e4u64) as u32 as u64;
            // 324ff jmp 0x3252f
            pc = 0x3252f; continue;
        }
        0x32501 => {
            // block 32501
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32501 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32501 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 32504 mov ecx,0x202
            rcx = (0x202u64) as u32 as u64;
            // 32509 mov eax,0x1fc
            rax = (0x1fcu64) as u32 as u64;
            // 3250e jmp 0x3251d
            pc = 0x3251d; continue;
        }
        0x32510 => {
            // block 32510
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32510 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32510 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 32513 mov ecx,0x264
            rcx = (0x264u64) as u32 as u64;
            // 32518 mov eax,0x256
            rax = (0x256u64) as u32 as u64;
            pc = 0x3251d;
        }
        0x3251d => {
            // block 3251d
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3251d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3251d cmove eax,ecx
            if fl.e() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 32520 mov DWORD PTR [rdi],eax
            m.w32(rdi, ((rax as u32 as u64)) as u32);
            // 32522 jmp 0x3252f
            pc = 0x3252f; continue;
        }
        0x32524 => {
            // block 32524
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32524 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32524 mov DWORD PTR [rdi],0x634
            m.w32(rdi, (0x634u64) as u32);
            // 3252a mov eax,0x634
            rax = (0x634u64) as u32 as u64;
            pc = 0x3252f;
        }
        0x3252f => {
            // block 3252f
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3252f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3252f mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 32531 sub ecx,r8d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r8 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // 32534 mov DWORD PTR [rdi+0xc],ecx
            m.w32(rdi.wrapping_add(0xc), ((rcx as u32 as u64)) as u32);
            // 32537 mov r8d,eax
            r8 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x3253a;
        }
        0x3253a => {
            // block 3253a
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3253a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3253a test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // 3253d je 0x32555
            if fl.e() { pc = 0x32555; continue; }
            pc = 0x3253f;
        }
        0x3253f => {
            // block 3253f
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3253f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3253f cmp edx,0x1f
            fl = sub::<32>((rdx as u32 as u64), 0x1fu64).1;
            // 32542 ja 0x32555
            if fl.a() { pc = 0x32555; continue; }
            pc = 0x32544;
        }
        0x32544 => {
            // block 32544
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32544 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32544 mov eax,0xfc800400
            rax = (0xfc800400u64) as u32 as u64;
            // 32549 bt eax,edx
            fl.cf = bit::<32>((rax as u32 as u64), (rdx as u32 as u64));
            // 3254c jae 0x32555
            if fl.ae() { pc = 0x32555; continue; }
            pc = 0x3254e;
        }
        0x3254e => {
            // block 3254e
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 3254e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3254e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3254f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32550 jmp 0x325ca
            { let r = { let t = m.env.code_va(0x325ca); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x32555 => {
            // block 32555
            #[cfg(feature = "trace")] eprintln!("f_000323b2 pc 32555 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32555 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32556 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32557 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x332f0 size=410
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000332f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x332f0;
    loop {
        match pc {
        0x332f0 => {
            // block 332f0
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 332f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 332f1 mov rbp,rsp
            rbp = rsp;
            // 332f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 332f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 332f8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 332fa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 332fc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 332fd sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 33301 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 33305 mov r9,rdx
            r9 = rdx;
            // 33308 mov QWORD PTR [rbp-0x48],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r9) as u64);
            // 3330c mov r13,rdi
            r13 = rdi;
            // 3330f mov rcx,QWORD PTR [rsi]
            rcx = m.r64(rsi) as u64;
            // 33312 mov rbx,QWORD PTR [rsi+0x8]
            rbx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 33316 cmp rbx,rcx
            fl = sub::<64>(rbx, rcx).1;
            // 33319 je 0x33330
            if fl.e() { pc = 0x33330; continue; }
            pc = 0x3331b;
        }
        0x3331b => {
            // block 3331b
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3331b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3331b lea rdx,[rbx-0x4]
            rdx = rbx.wrapping_add(0xffff_ffff_ffff_fffc);
            // 3331f sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 33322 not rdx
            { let v = !rdx; rdx = v; }
            // 33325 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 33329 add rdx,rbx
            { let (r, f) = add::<64>(rdx, rbx);
              rdx = r;
              fl = f; }
            // 3332c mov QWORD PTR [rsi+0x8],rdx
            m.w64(rsi.wrapping_add(0x8), (rdx) as u64);
            pc = 0x33330;
        }
        0x33330 => {
            // block 33330
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33330 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33330 mov edx,DWORD PTR [r13+0x0]
            rdx = (m.r32(r13) as u64) as u32 as u64;
            // 33334 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 33336 je 0x33478
            if fl.e() { pc = 0x33478; continue; }
            pc = 0x3333c;
        }
        0x3333c => {
            // block 3333c
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3333c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3333c cmp DWORD PTR [r13+0x4],0x0
            fl = sub::<32>(m.r32(r13.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 33341 je 0x33478
            if fl.e() { pc = 0x33478; continue; }
            pc = 0x33347;
        }
        0x33347 => {
            // block 33347
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33347 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33347 mov r14w,WORD PTR [r13+0x34]
            r14 = (r14 & !0xffff) | ((m.r16(r13.wrapping_add(0x34)) as u64) as u16 as u64);
            // 3334c test r14w,r14w
            fl = logic::<16>((r14 as u16 as u64) & (r14 as u16 as u64));
            // 33350 je 0x33478
            if fl.e() { pc = 0x33478; continue; }
            pc = 0x33356;
        }
        0x33356 => {
            // block 33356
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33356 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33356 mov ecx,DWORD PTR [r13+0x24]
            rcx = (m.r32(r13.wrapping_add(0x24)) as u64) as u32 as u64;
            // 3335a test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 3335c je 0x33478
            if fl.e() { pc = 0x33478; continue; }
            pc = 0x33362;
        }
        0x33362 => {
            // block 33362
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33362 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33364 test r9,r9
            fl = logic::<64>(r9 & r9);
            // 33367 je 0x3347a
            if fl.e() { pc = 0x3347a; continue; }
            pc = 0x3336d;
        }
        0x3336d => {
            // block 3336d
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3336d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3336d cmp QWORD PTR [rbp-0x40],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64, 0x0u64).1;
            // 33372 je 0x3347a
            if fl.e() { pc = 0x3347a; continue; }
            pc = 0x33378;
        }
        0x33378 => {
            // block 33378
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33378 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33378 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 3337a add ebx,0xffffffff
            { let (r, f) = add::<32>((rbx as u32 as u64), 0xffffffffu64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 3337d jae 0x33478
            if fl.ae() { pc = 0x33478; continue; }
            pc = 0x33383;
        }
        0x33383 => {
            // block 33383
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33383 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33383 imul ecx,ebx
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rbx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // 33386 add rcx,r9
            { let (r, f) = add::<64>(rcx, r9);
              rcx = r;
              fl = f; }
            // 33389 je 0x33478
            if fl.e() { pc = 0x33478; continue; }
            pc = 0x3338f;
        }
        0x3338f => {
            // block 3338f
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3338f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3338f mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 33391 cmp r9,QWORD PTR [rbp-0x40]
            fl = sub::<64>(r9, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64).1;
            // 33395 je 0x3347a
            if fl.e() { pc = 0x3347a; continue; }
            pc = 0x3339b;
        }
        0x3339b => {
            // block 3339b
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3339b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3339b mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 3339f test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // 333a2 cmove r8d,edx
            if fl.e() { r8 = ((rdx as u32 as u64)) as u32 as u64; } else { r8 = r8 as u32 as u64; }
            // 333a6 cmp r8d,edx
            fl = sub::<32>((r8 as u32 as u64), (rdx as u32 as u64)).1;
            // 333a9 cmova r8d,edx
            if fl.a() { r8 = ((rdx as u32 as u64)) as u32 as u64; } else { r8 = r8 as u32 as u64; }
            // 333ad mov DWORD PTR [rbp-0x4c],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((r8 as u32 as u64)) as u32);
            // 333b1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x333b3;
        }
        0x333b3 => {
            // block 333b3
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 333b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 333b3 cmp DWORD PTR [rbp-0x4c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64, 0x0u64).1;
            // 333b7 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 333be je 0x3345d
            if fl.e() { pc = 0x3345d; continue; }
            pc = 0x333c4;
        }
        0x333c4 => {
            // block 333c4
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 333c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 333c4 movzx r12d,si
            r12 = ((rsi as u16 as u64)) as u32 as u64;
            // 333c8 mov DWORD PTR [rbp-0x5c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rsi as u32 as u64)) as u32);
            // 333cb mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            pc = 0x333d0;
        }
        0x333d0 => {
            // block 333d0
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 333d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 333d0 lea r15d,[rbx-0x1]
            r15 = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 333d4 mov rdi,r13
            rdi = r13;
            // 333d7 mov rsi,r9
            rsi = r9;
            // 333da mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 333dd mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // 333e0 call 0x330d0
            { let r = { let t = m.env.code_va(0x330d0); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 333e5 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 333e9 mov rdi,r13
            rdi = r13;
            // 333ec mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 333f0 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 333f3 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // 333f6 call 0x330d0
            { let r = { let t = m.env.code_va(0x330d0); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 333fb xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 333fd movzx edx,r14w
            rdx = ((r14 as u16 as u64)) as u32 as u64;
            // 33401 cmp edx,r12d
            fl = sub::<32>((rdx as u32 as u64), (r12 as u32 as u64)).1;
            // 33404 jbe 0x3340b
            if fl.be() { pc = 0x3340b; continue; }
            pc = 0x33406;
        }
        0x33406 => {
            // block 33406
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33406 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33406 mov ecx,DWORD PTR [r13+r12*4+0x24]
            rcx = (m.r32(r13.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x24)) as u64) as u32 as u64;
            pc = 0x3340b;
        }
        0x3340b => {
            // block 3340b
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3340b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3340b mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 3340d mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 33411 mov rsi,rax
            rsi = rax;
            // 33414 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33419 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 3341b je 0x33441
            if fl.e() { pc = 0x33441; continue; }
            pc = 0x3341d;
        }
        0x3341d => {
            // block 3341d
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3341d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3341d mov rdi,QWORD PTR [rbp-0x58]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 33421 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 33425 cmp rax,QWORD PTR [rdi+0x10]
            fl = sub::<64>(rax, m.r64(rdi.wrapping_add(0x10)) as u64).1;
            // 33429 je 0x33438
            if fl.e() { pc = 0x33438; continue; }
            pc = 0x3342b;
        }
        0x3342b => {
            // block 3342b
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3342b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3342b mov DWORD PTR [rax],r15d
            m.w32(rax, ((r15 as u32 as u64)) as u32);
            // 3342e add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 33432 mov QWORD PTR [rdi+0x8],rax
            m.w64(rdi.wrapping_add(0x8), (rax) as u64);
            // 33436 jmp 0x33441
            pc = 0x33441; continue;
        }
        0x33438 => {
            // block 33438
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33438 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 3343c call 0xebb3a
            { let r = { let t = m.env.stub_va(0x307); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x33441;
        }
        0x33441 => {
            // block 33441
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33441 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33441 mov DWORD PTR [rbp-0x2c],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rbx as u32 as u64)) as u32);
            // 33444 movzx r14d,WORD PTR [r13+0x34]
            r14 = (m.r16(r13.wrapping_add(0x34)) as u64) as u32 as u64;
            // 33449 cmp ebx,DWORD PTR [rbp-0x4c]
            fl = sub::<32>((rbx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64).1;
            // 3344c lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 3344f mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 33451 mov r9,QWORD PTR [rbp-0x48]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 33455 jb 0x333d0
            if fl.b() { pc = 0x333d0; continue; }
            pc = 0x3345b;
        }
        0x3345b => {
            // block 3345b
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3345b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3345b jmp 0x33460
            pc = 0x33460; continue;
        }
        0x3345d => {
            // block 3345d
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3345d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3345d mov DWORD PTR [rbp-0x5c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rsi as u32 as u64)) as u32);
            pc = 0x33460;
        }
        0x33460 => {
            // block 33460
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33460 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33460 mov esi,DWORD PTR [rbp-0x5c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // 33463 inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 33465 movzx edx,si
            rdx = ((rsi as u16 as u64)) as u32 as u64;
            // 33468 movzx ecx,r14w
            rcx = ((r14 as u16 as u64)) as u32 as u64;
            // 3346c cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 3346e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 33470 jb 0x333b3
            if fl.b() { pc = 0x333b3; continue; }
            pc = 0x33476;
        }
        0x33476 => {
            // block 33476
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33476 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33476 jmp 0x3347a
            pc = 0x3347a; continue;
        }
        0x33478 => {
            // block 33478
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 33478 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33478 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x3347a;
        }
        0x3347a => {
            // block 3347a
            #[cfg(feature = "trace")] eprintln!("f_000332f0 pc 3347a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3347a add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 3347e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3347f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 33481 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 33483 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 33485 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 33487 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 33488 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x33b08 size=458
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00033b08<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x33b08;
    loop {
        match pc {
        0x33b08 => {
            // block 33b08
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b08 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 33b09 mov rbp,rsp
            rbp = rsp;
            // 33b0c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 33b0e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 33b0f mov DWORD PTR [rdx],0xffffffff
            m.w32(rdx, (0xffffffffu64) as u32);
            // 33b15 cmp DWORD PTR [rdi],0x0
            fl = sub::<32>(m.r32(rdi) as u64, 0x0u64).1;
            // 33b18 je 0x33c93
            if fl.e() { pc = 0x33c93; continue; }
            pc = 0x33b1e;
        }
        0x33b1e => {
            // block 33b1e
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b1e cmp DWORD PTR [rdi+0x4],0x0
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 33b22 je 0x33c93
            if fl.e() { pc = 0x33c93; continue; }
            pc = 0x33b28;
        }
        0x33b28 => {
            // block 33b28
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b28 cmp WORD PTR [rdi+0x34],0x0
            fl = sub::<16>(m.r16(rdi.wrapping_add(0x34)) as u64, 0x0u64).1;
            // 33b2d je 0x33c93
            if fl.e() { pc = 0x33c93; continue; }
            pc = 0x33b33;
        }
        0x33b33 => {
            // block 33b33
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b33 cmp DWORD PTR [rdi+0x24],0x0
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 33b37 je 0x33c93
            if fl.e() { pc = 0x33c93; continue; }
            pc = 0x33b3d;
        }
        0x33b3d => {
            // block 33b3d
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b3d mov ecx,DWORD PTR [rdi+0x10]
            rcx = (m.r32(rdi.wrapping_add(0x10)) as u64) as u32 as u64;
            // 33b40 cmp rcx,0xb
            fl = sub::<64>(rcx, 0xbu64).1;
            // 33b44 ja 0x33c93
            if fl.a() { pc = 0x33c93; continue; }
            pc = 0x33b4a;
        }
        0x33b4a => {
            // block 33b4a
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b4a mov r10d,DWORD PTR [rdi+0x1c]
            r10 = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 33b4e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33b50 cmp r10d,0x2
            fl = sub::<32>((r10 as u32 as u64), 0x2u64).1;
            // 33b54 ja 0x33c95
            if fl.a() { pc = 0x33c95; continue; }
            pc = 0x33b5a;
        }
        0x33b5a => {
            // block 33b5a
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b5a cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 33b5d sete r8b
            r8 = (r8 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 33b61 setne r9b
            r9 = (r9 & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 33b65 cmp r10d,0x2
            fl = sub::<32>((r10 as u32 as u64), 0x2u64).1;
            // 33b69 je 0x33b85
            if fl.e() { pc = 0x33b85; continue; }
            pc = 0x33b6b;
        }
        0x33b6b => {
            // block 33b6b
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b6b cmp r10d,0x1
            fl = sub::<32>((r10 as u32 as u64), 0x1u64).1;
            // 33b6f jne 0x33b99
            if fl.ne() { pc = 0x33b99; continue; }
            pc = 0x33b71;
        }
        0x33b71 => {
            // block 33b71
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b71 lea rax,[rip+0xcc3d8] # 0xfff50
            rax = m.env.data_va(83741);
            // 33b78 lea rax,[rax+rcx*4]
            rax = rax.wrapping_add(rcx.wrapping_mul(4));
            // 33b7c lea rbx,[rip+0xcc38d] # 0xfff10
            rbx = m.env.data_va(83677);
            // 33b83 jmp 0x33bb6
            pc = 0x33bb6; continue;
        }
        0x33b85 => {
            // block 33b85
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b85 lea rax,[rip+0xcc444] # 0xfffd0
            rax = m.env.data_va(83869);
            // 33b8c lea rax,[rax+rcx*4]
            rax = rax.wrapping_add(rcx.wrapping_mul(4));
            // 33b90 lea rbx,[rip+0xcc3f9] # 0xfff90
            rbx = m.env.data_va(83805);
            // 33b97 jmp 0x33bb6
            pc = 0x33bb6; continue;
        }
        0x33b99 => {
            // block 33b99
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b99 test r10d,r10d
            fl = logic::<32>((r10 as u32 as u64) & (r10 as u32 as u64));
            // 33b9c mov r10d,0x0
            r10 = (0x0u64) as u32 as u64;
            // 33ba2 jne 0x33bc7
            if fl.ne() { pc = 0x33bc7; continue; }
            pc = 0x33ba4;
        }
        0x33ba4 => {
            // block 33ba4
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33ba4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ba4 lea rax,[rip+0xcc325] # 0xffed0
            rax = m.env.data_va(83613);
            // 33bab lea rax,[rax+rcx*4]
            rax = rax.wrapping_add(rcx.wrapping_mul(4));
            // 33baf lea rbx,[rip+0xcc2da] # 0xffe90
            rbx = m.env.data_va(83549);
            pc = 0x33bb6;
        }
        0x33bb6 => {
            // block 33bb6
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bb6 lea r10,[rbx+rcx*4]
            r10 = rbx.wrapping_add(rcx.wrapping_mul(4));
            // 33bba mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 33bbc cmp eax,esi
            fl = sub::<32>((rax as u32 as u64), (rsi as u32 as u64)).1;
            // 33bbe ja 0x33c93
            if fl.a() { pc = 0x33c93; continue; }
            pc = 0x33bc4;
        }
        0x33bc4 => {
            // block 33bc4
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bc4 mov r10d,DWORD PTR [r10]
            r10 = (m.r32(r10) as u64) as u32 as u64;
            pc = 0x33bc7;
        }
        0x33bc7 => {
            // block 33bc7
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bc7 lea rbx,[rip+0xcc442] # 0x100010
            rbx = m.env.data_va(83933);
            // 33bce mov r11d,DWORD PTR [rbx+rcx*4]
            r11 = (m.r32(rbx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 33bd2 cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 33bd5 ja 0x33bf7
            if fl.a() { pc = 0x33bf7; continue; }
            pc = 0x33bd7;
        }
        0x33bd7 => {
            // block 33bd7
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bd7 mov ebx,0x752
            rbx = (0x752u64) as u32 as u64;
            // 33bdc bt ebx,ecx
            fl.cf = bit::<32>((rbx as u32 as u64), (rcx as u32 as u64));
            // 33bdf jae 0x33bf7
            if fl.ae() { pc = 0x33bf7; continue; }
            pc = 0x33be1;
        }
        0x33be1 => {
            // block 33be1
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33be1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33be1 mov ebx,0x5f
            rbx = (0x5fu64) as u32 as u64;
            // 33be6 bt rbx,rcx
            fl.cf = bit::<64>(rbx, rcx);
            // 33bea jae 0x33c13
            if fl.ae() { pc = 0x33c13; continue; }
            pc = 0x33bec;
        }
        0x33bec => {
            // block 33bec
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bec cmp r11d,esi
            fl = sub::<32>((r11 as u32 as u64), (rsi as u32 as u64)).1;
            // 33bef jb 0x33c93
            if fl.b() { pc = 0x33c93; continue; }
            pc = 0x33bf5;
        }
        0x33bf5 => {
            // block 33bf5
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bf5 jmp 0x33c13
            pc = 0x33c13; continue;
        }
        0x33bf7 => {
            // block 33bf7
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bf7 mov ebx,0x5f
            rbx = (0x5fu64) as u32 as u64;
            // 33bfc shr rbx,cl
            { let (r, f) = shr::<64>(rbx, (rcx as u8 as u64), fl); rbx = r; fl = f; }
            // 33bff cmp r11d,esi
            fl = sub::<32>((r11 as u32 as u64), (rsi as u32 as u64)).1;
            // 33c02 jae 0x33c13
            if fl.ae() { pc = 0x33c13; continue; }
            pc = 0x33c04;
        }
        0x33c04 => {
            // block 33c04
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c04 and rbx,0x1
            { let r = rbx & 0x1u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 33c08 je 0x33c13
            if fl.e() { pc = 0x33c13; continue; }
            pc = 0x33c0a;
        }
        0x33c0a => {
            // block 33c0a
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c0a cmp ecx,0xb
            fl = sub::<32>((rcx as u32 as u64), 0xbu64).1;
            // 33c0d je 0x33c93
            if fl.e() { pc = 0x33c93; continue; }
            pc = 0x33c13;
        }
        0x33c13 => {
            // block 33c13
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c13 lea r14d,[rcx-0x1]
            r14 = (rcx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33c17 cmp r14d,0xa
            fl = sub::<32>((r14 as u32 as u64), 0xau64).1;
            // 33c1b jae 0x33c28
            if fl.ae() { pc = 0x33c28; continue; }
            pc = 0x33c1d;
        }
        0x33c1d => {
            // block 33c1d
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c1d mov ebx,0x3a9
            rbx = (0x3a9u64) as u32 as u64;
            // 33c22 bt ebx,r14d
            fl.cf = bit::<32>((rbx as u32 as u64), (r14 as u32 as u64));
            // 33c26 jb 0x33c53
            if fl.b() { pc = 0x33c53; continue; }
            pc = 0x33c28;
        }
        0x33c28 => {
            // block 33c28
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c28 mov ebx,0x5f
            rbx = (0x5fu64) as u32 as u64;
            // 33c2d shr rbx,cl
            { let (r, f) = shr::<64>(rbx, (rcx as u8 as u64), fl); rbx = r; fl = f; }
            // 33c30 cmp r11d,esi
            fl = sub::<32>((r11 as u32 as u64), (rsi as u32 as u64)).1;
            // 33c33 mov r11d,ecx
            r11 = ((rcx as u32 as u64)) as u32 as u64;
            // 33c36 jae 0x33c61
            if fl.ae() { pc = 0x33c61; continue; }
            pc = 0x33c38;
        }
        0x33c38 => {
            // block 33c38
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c38 cmp r10d,esi
            fl = sub::<32>((r10 as u32 as u64), (rsi as u32 as u64)).1;
            // 33c3b mov r11d,ecx
            r11 = ((rcx as u32 as u64)) as u32 as u64;
            // 33c3e jbe 0x33c61
            if fl.be() { pc = 0x33c61; continue; }
            pc = 0x33c40;
        }
        0x33c40 => {
            // block 33c40
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c40 and rbx,0x1
            { let r = rbx & 0x1u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 33c44 mov r11d,ecx
            r11 = ((rcx as u32 as u64)) as u32 as u64;
            // 33c47 je 0x33c61
            if fl.e() { pc = 0x33c61; continue; }
            pc = 0x33c49;
        }
        0x33c49 => {
            // block 33c49
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c49 cmp ecx,0xb
            fl = sub::<32>((rcx as u32 as u64), 0xbu64).1;
            // 33c4c mov r11d,ecx
            r11 = ((rcx as u32 as u64)) as u32 as u64;
            // 33c4f jne 0x33c93
            if fl.ne() { pc = 0x33c93; continue; }
            pc = 0x33c51;
        }
        0x33c51 => {
            // block 33c51
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c51 jmp 0x33c61
            pc = 0x33c61; continue;
        }
        0x33c53 => {
            // block 33c53
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c53 movsxd r11,r14d
            r11 = sx::<32>((r14 as u32 as u64)) as u64;
            // 33c56 lea rbx,[rip+0xcc433] # 0x100090
            rbx = m.env.data_va(84061);
            // 33c5d mov r11d,DWORD PTR [rbx+r11*4]
            r11 = (m.r32(rbx.wrapping_add(r11.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x33c61;
        }
        0x33c61 => {
            // block 33c61
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c61 cmp r11d,0xa
            fl = sub::<32>((r11 as u32 as u64), 0xau64).1;
            // 33c65 ja 0x33c72
            if fl.a() { pc = 0x33c72; continue; }
            pc = 0x33c67;
        }
        0x33c67 => {
            // block 33c67
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c67 mov ebx,0x752
            rbx = (0x752u64) as u32 as u64;
            // 33c6c bt ebx,r11d
            fl.cf = bit::<32>((rbx as u32 as u64), (r11 as u32 as u64));
            // 33c70 jb 0x33c9a
            if fl.b() { pc = 0x33c9a; continue; }
            pc = 0x33c72;
        }
        0x33c72 => {
            // block 33c72
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c72 lea r14,[rip+0xcc3d7] # 0x100050
            r14 = m.env.data_va(83997);
            // 33c79 mov ebx,0x5f
            rbx = (0x5fu64) as u32 as u64;
            // 33c7e shr rbx,cl
            { let (r, f) = shr::<64>(rbx, (rcx as u8 as u64), fl); rbx = r; fl = f; }
            // 33c81 cmp DWORD PTR [r14+rcx*4],esi
            fl = sub::<32>(m.r32(r14.wrapping_add(rcx.wrapping_mul(4))) as u64, (rsi as u32 as u64)).1;
            // 33c85 jae 0x33c9a
            if fl.ae() { pc = 0x33c9a; continue; }
            pc = 0x33c87;
        }
        0x33c87 => {
            // block 33c87
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c87 and rbx,0x1
            { let r = rbx & 0x1u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 33c8b je 0x33c9a
            if fl.e() { pc = 0x33c9a; continue; }
            pc = 0x33c8d;
        }
        0x33c8d => {
            // block 33c8d
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c8d cmp r11d,0xb
            fl = sub::<32>((r11 as u32 as u64), 0xbu64).1;
            // 33c91 je 0x33c9a
            if fl.e() { pc = 0x33c9a; continue; }
            pc = 0x33c93;
        }
        0x33c93 => {
            // block 33c93
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c93 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x33c95;
        }
        0x33c95 => {
            // block 33c95
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c95 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 33c96 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 33c98 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 33c99 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x33c9a => {
            // block 33c9a
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c9a cmp ecx,0xb
            fl = sub::<32>((rcx as u32 as u64), 0xbu64).1;
            // 33c9d ja 0x33cad
            if fl.a() { pc = 0x33cad; continue; }
            pc = 0x33c9f;
        }
        0x33c9f => {
            // block 33c9f
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c9f mov ebx,0xf52
            rbx = (0xf52u64) as u32 as u64;
            // 33ca4 bt ebx,ecx
            fl.cf = bit::<32>((rbx as u32 as u64), (rcx as u32 as u64));
            // 33ca7 jae 0x33cad
            if fl.ae() { pc = 0x33cad; continue; }
            pc = 0x33ca9;
        }
        0x33ca9 => {
            // block 33ca9
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33ca9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ca9 sub esi,eax
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rax as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cab jmp 0x33cc9
            pc = 0x33cc9; continue;
        }
        0x33cad => {
            // block 33cad
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33cad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33cad cmp r10d,esi
            fl = sub::<32>((r10 as u32 as u64), (rsi as u32 as u64)).1;
            // 33cb0 jbe 0x33cbe
            if fl.be() { pc = 0x33cbe; continue; }
            pc = 0x33cb2;
        }
        0x33cb2 => {
            // block 33cb2
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33cb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33cb2 movzx ecx,r8b
            rcx = ((r8 as u8 as u64)) as u32 as u64;
            // 33cb6 sub esi,eax
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rax as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cb8 add esi,esi
            { let (r, f) = add::<32>((rsi as u32 as u64), (rsi as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cba or esi,ecx
            { let r = (rsi as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cbc jmp 0x33cc9
            pc = 0x33cc9; continue;
        }
        0x33cbe => {
            // block 33cbe
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33cbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33cbe movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 33cc2 sub esi,r10d
            { let (r, f) = sub::<32>((rsi as u32 as u64), (r10 as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cc5 add esi,esi
            { let (r, f) = add::<32>((rsi as u32 as u64), (rsi as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cc7 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x33cc9;
        }
        0x33cc9 => {
            // block 33cc9
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33cc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33cc9 mov DWORD PTR [rdx],esi
            m.w32(rdx, ((rsi as u32 as u64)) as u32);
            // 33ccb cmp esi,DWORD PTR [rdi]
            fl = sub::<32>((rsi as u32 as u64), m.r32(rdi) as u64).1;
            // 33ccd setb al
            rax = (rax & !0xff) | (((fl.b() as u64)) as u8 as u64);
            // 33cd0 jmp 0x33c95
            pc = 0x33c95; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6537c size=394
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006537c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6537c;
    loop {
        match pc {
        0x6537c => {
            // block 6537c
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6537c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6537c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6537d mov rbp,rsp
            rbp = rsp;
            // 65380 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 65382 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 65384 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 65386 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65388 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65389 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 6538d mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 65391 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 65394 je 0x654d0
            if fl.e() { pc = 0x654d0; continue; }
            pc = 0x6539a;
        }
        0x6539a => {
            // block 6539a
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6539a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6539a mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 6539e movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // 653a1 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 653a4 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 653a8 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 653ab mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 653af mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 653b3 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 653b7 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 653bb mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 653bf lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // 653c3 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 653c7 jmp 0x653cc
            pc = 0x653cc; continue;
        }
        0x653c9 => {
            // block 653c9
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 653c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 653c9 mov rbx,rax
            rbx = rax;
            pc = 0x653cc;
        }
        0x653cc => {
            // block 653cc
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 653cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 653cc test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 653cf mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 653d3 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 653d8 movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 653dc mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 653df and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 653e3 jne 0x653ed
            if fl.ne() { pc = 0x653ed; continue; }
            pc = 0x653e5;
        }
        0x653e5 => {
            // block 653e5
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 653e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 653e5 mov r14,rax
            r14 = rax;
            // 653e8 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 653eb jmp 0x653f1
            pc = 0x653f1; continue;
        }
        0x653ed => {
            // block 653ed
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 653ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 653ed mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x653f1;
        }
        0x653f1 => {
            // block 653f1
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 653f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 653f1 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // 653f5 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 653f8 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 653fc cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 65401 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 65404 jne 0x65410
            if fl.ne() { pc = 0x65410; continue; }
            pc = 0x65406;
        }
        0x65406 => {
            // block 65406
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65406 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65406 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // 6540a mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 6540e jmp 0x65418
            pc = 0x65418; continue;
        }
        0x65410 => {
            // block 65410
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65410 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 65414 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x65418;
        }
        0x65418 => {
            // block 65418
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65418 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 6541b mov rdx,r13
            rdx = r13;
            // 6541e cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // 65422 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 65425 je 0x6543c
            if fl.e() { pc = 0x6543c; continue; }
            pc = 0x65427;
        }
        0x65427 => {
            // block 65427
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65427 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65427 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 6542b call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65430 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65432 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 65436 je 0x6543c
            if fl.e() { pc = 0x6543c; continue; }
            pc = 0x65438;
        }
        0x65438 => {
            // block 65438
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65438 jns 0x6544e
            if fl.ns() { pc = 0x6544e; continue; }
            pc = 0x6543a;
        }
        0x6543a => {
            // block 6543a
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6543a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6543a jmp 0x65441
            pc = 0x65441; continue;
        }
        0x6543c => {
            // block 6543c
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6543c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6543c cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 6543f jae 0x6544e
            if fl.ae() { pc = 0x6544e; continue; }
            pc = 0x65441;
        }
        0x65441 => {
            // block 65441
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65441 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65441 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 65444 test rax,rax
            fl = logic::<64>(rax & rax);
            // 65447 jne 0x653c9
            if fl.ne() { pc = 0x653c9; continue; }
            pc = 0x65449;
        }
        0x65449 => {
            // block 65449
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65449 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65449 jmp 0x654e5
            pc = 0x654e5; continue;
        }
        0x6544e => {
            // block 6544e
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6544e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6544e test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 65451 jne 0x6545b
            if fl.ne() { pc = 0x6545b; continue; }
            pc = 0x65453;
        }
        0x65453 => {
            // block 65453
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65453 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65453 mov r14,rbx
            r14 = rbx;
            // 65456 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 65459 jmp 0x65462
            pc = 0x65462; continue;
        }
        0x6545b => {
            // block 6545b
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6545b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6545b mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 6545f mov r14,rbx
            r14 = rbx;
            pc = 0x65462;
        }
        0x65462 => {
            // block 65462
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65462 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65462 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 65466 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 65469 mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 6546d cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 65472 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 65475 jne 0x6547c
            if fl.ne() { pc = 0x6547c; continue; }
            pc = 0x65477;
        }
        0x65477 => {
            // block 65477
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65477 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65477 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 6547a jmp 0x65480
            pc = 0x65480; continue;
        }
        0x6547c => {
            // block 6547c
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6547c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6547c mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0x65480;
        }
        0x65480 => {
            // block 65480
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65480 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65480 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 65483 mov rdx,rax
            rdx = rax;
            // 65486 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // 6548a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 6548d je 0x654ae
            if fl.e() { pc = 0x654ae; continue; }
            pc = 0x6548f;
        }
        0x6548f => {
            // block 6548f
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6548f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6548f test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 65492 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 65496 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 6549b mov r12,rax
            r12 = rax;
            // 6549e call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 654a3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 654a5 mov rax,r12
            rax = r12;
            // 654a8 je 0x654ae
            if fl.e() { pc = 0x654ae; continue; }
            pc = 0x654aa;
        }
        0x654aa => {
            // block 654aa
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654aa js 0x654b3
            if fl.s() { pc = 0x654b3; continue; }
            pc = 0x654ac;
        }
        0x654ac => {
            // block 654ac
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654ac jmp 0x654dc
            pc = 0x654dc; continue;
        }
        0x654ae => {
            // block 654ae
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654ae cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 654b1 jae 0x654dc
            if fl.ae() { pc = 0x654dc; continue; }
            pc = 0x654b3;
        }
        0x654b3 => {
            // block 654b3
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654b3 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 654b7 test rax,rax
            fl = logic::<64>(rax & rax);
            // 654ba jne 0x653c9
            if fl.ne() { pc = 0x653c9; continue; }
            pc = 0x654c0;
        }
        0x654c0 => {
            // block 654c0
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654c0 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 654c4 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 654c7 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 654cb mov rax,r14
            rax = r14;
            // 654ce jmp 0x654ef
            pc = 0x654ef; continue;
        }
        0x654d0 => {
            // block 654d0
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654d0 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 654d4 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 654d7 mov rax,rdi
            rax = rdi;
            // 654da jmp 0x654ef
            pc = 0x654ef; continue;
        }
        0x654dc => {
            // block 654dc
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654dc mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 654e0 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 654e3 jmp 0x654ef
            pc = 0x654ef; continue;
        }
        0x654e5 => {
            // block 654e5
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654e5 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 654e9 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // 654ec mov rax,rbx
            rax = rbx;
            pc = 0x654ef;
        }
        0x654ef => {
            // block 654ef
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654ef add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 654f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 654f4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 654f6 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 654f8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 654fa pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 654fc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 654fd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5f52 size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5f52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5f52;
    loop {
        match pc {
        0xb5f52 => {
            // block b5f52
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5f52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5f52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5f53 mov rbp,rsp
            rbp = rsp;
            // b5f56 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5f58 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5f5a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5f5c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5f5e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5f5f sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b5f63 mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b5f67 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5f6a je 0xb60a6
            if fl.e() { pc = 0xb60a6; continue; }
            pc = 0xb5f70;
        }
        0xb5f70 => {
            // block b5f70
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5f70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5f70 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // b5f74 movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // b5f77 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b5f7a and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5f7e shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // b5f81 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b5f85 mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // b5f89 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b5f8d mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // b5f91 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // b5f95 lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // b5f99 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // b5f9d jmp 0xb5fa2
            pc = 0xb5fa2; continue;
        }
        0xb5f9f => {
            // block b5f9f
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5f9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5f9f mov rbx,rax
            rbx = rax;
            pc = 0xb5fa2;
        }
        0xb5fa2 => {
            // block b5fa2
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fa2 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b5fa5 mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b5fa9 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // b5fae movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b5fb2 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b5fb5 and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5fb9 jne 0xb5fc3
            if fl.ne() { pc = 0xb5fc3; continue; }
            pc = 0xb5fbb;
        }
        0xb5fbb => {
            // block b5fbb
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fbb mov r14,rax
            r14 = rax;
            // b5fbe shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b5fc1 jmp 0xb5fc7
            pc = 0xb5fc7; continue;
        }
        0xb5fc3 => {
            // block b5fc3
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fc3 mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xb5fc7;
        }
        0xb5fc7 => {
            // block b5fc7
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fc7 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // b5fcb test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b5fce mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b5fd2 cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // b5fd7 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b5fda jne 0xb5fe6
            if fl.ne() { pc = 0xb5fe6; continue; }
            pc = 0xb5fdc;
        }
        0xb5fdc => {
            // block b5fdc
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fdc lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // b5fe0 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // b5fe4 jmp 0xb5fee
            pc = 0xb5fee; continue;
        }
        0xb5fe6 => {
            // block b5fe6
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fe6 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // b5fea mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xb5fee;
        }
        0xb5fee => {
            // block b5fee
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fee cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // b5ff1 mov rdx,r13
            rdx = r13;
            // b5ff4 cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // b5ff8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5ffb je 0xb6012
            if fl.e() { pc = 0xb6012; continue; }
            pc = 0xb5ffd;
        }
        0xb5ffd => {
            // block b5ffd
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5ffd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ffd mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // b6001 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6006 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b6008 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // b600c je 0xb6012
            if fl.e() { pc = 0xb6012; continue; }
            pc = 0xb600e;
        }
        0xb600e => {
            // block b600e
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b600e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b600e jns 0xb6024
            if fl.ns() { pc = 0xb6024; continue; }
            pc = 0xb6010;
        }
        0xb6010 => {
            // block b6010
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6010 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6010 jmp 0xb6017
            pc = 0xb6017; continue;
        }
        0xb6012 => {
            // block b6012
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6012 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // b6015 jae 0xb6024
            if fl.ae() { pc = 0xb6024; continue; }
            pc = 0xb6017;
        }
        0xb6017 => {
            // block b6017
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6017 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6017 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b601a test rax,rax
            fl = logic::<64>(rax & rax);
            // b601d jne 0xb5f9f
            if fl.ne() { pc = 0xb5f9f; continue; }
            pc = 0xb601f;
        }
        0xb601f => {
            // block b601f
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b601f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b601f jmp 0xb60bb
            pc = 0xb60bb; continue;
        }
        0xb6024 => {
            // block b6024
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6024 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6024 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b6027 jne 0xb6031
            if fl.ne() { pc = 0xb6031; continue; }
            pc = 0xb6029;
        }
        0xb6029 => {
            // block b6029
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6029 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6029 mov r14,rbx
            r14 = rbx;
            // b602c shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // b602f jmp 0xb6038
            pc = 0xb6038; continue;
        }
        0xb6031 => {
            // block b6031
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6031 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6031 mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // b6035 mov r14,rbx
            r14 = rbx;
            pc = 0xb6038;
        }
        0xb6038 => {
            // block b6038
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6038 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6038 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b603c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b603f mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b6043 cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // b6048 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b604b jne 0xb6052
            if fl.ne() { pc = 0xb6052; continue; }
            pc = 0xb604d;
        }
        0xb604d => {
            // block b604d
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b604d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b604d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // b6050 jmp 0xb6056
            pc = 0xb6056; continue;
        }
        0xb6052 => {
            // block b6052
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6052 mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0xb6056;
        }
        0xb6056 => {
            // block b6056
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6056 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6056 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // b6059 mov rdx,rax
            rdx = rax;
            // b605c cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // b6060 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b6063 je 0xb6084
            if fl.e() { pc = 0xb6084; continue; }
            pc = 0xb6065;
        }
        0xb6065 => {
            // block b6065
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6065 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6065 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b6068 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b606c cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // b6071 mov r12,rax
            r12 = rax;
            // b6074 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6079 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b607b mov rax,r12
            rax = r12;
            // b607e je 0xb6084
            if fl.e() { pc = 0xb6084; continue; }
            pc = 0xb6080;
        }
        0xb6080 => {
            // block b6080
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6080 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6080 js 0xb6089
            if fl.s() { pc = 0xb6089; continue; }
            pc = 0xb6082;
        }
        0xb6082 => {
            // block b6082
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6082 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6082 jmp 0xb60b2
            pc = 0xb60b2; continue;
        }
        0xb6084 => {
            // block b6084
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6084 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6084 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // b6087 jae 0xb60b2
            if fl.ae() { pc = 0xb60b2; continue; }
            pc = 0xb6089;
        }
        0xb6089 => {
            // block b6089
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6089 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6089 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // b608d test rax,rax
            fl = logic::<64>(rax & rax);
            // b6090 jne 0xb5f9f
            if fl.ne() { pc = 0xb5f9f; continue; }
            pc = 0xb6096;
        }
        0xb6096 => {
            // block b6096
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6096 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6096 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // b609a mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // b609d add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // b60a1 mov rax,r14
            rax = r14;
            // b60a4 jmp 0xb60c5
            pc = 0xb60c5; continue;
        }
        0xb60a6 => {
            // block b60a6
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b60a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b60a6 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b60aa mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // b60ad mov rax,rdi
            rax = rdi;
            // b60b0 jmp 0xb60c5
            pc = 0xb60c5; continue;
        }
        0xb60b2 => {
            // block b60b2
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b60b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b60b2 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // b60b6 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // b60b9 jmp 0xb60c5
            pc = 0xb60c5; continue;
        }
        0xb60bb => {
            // block b60bb
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b60bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b60bb mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // b60bf mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // b60c2 mov rax,rbx
            rax = rbx;
            pc = 0xb60c5;
        }
        0xb60c5 => {
            // block b60c5
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b60c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b60c5 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b60c9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b60ca pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b60cc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b60ce pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b60d0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b60d2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b60d3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7a88 size=450
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7a88<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7a88;
    loop {
        match pc {
        0xb7a88 => {
            // block b7a88
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7a88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a88 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7a89 mov rbp,rsp
            rbp = rsp;
            // b7a8c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7a8e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7a8f sub rsp,0x60
            { let (r, f) = sub::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // b7a93 mov rbx,rsi
            rbx = rsi;
            // b7a96 mov r14,rdi
            r14 = rdi;
            // b7a99 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b7a9c call QWORD PTR [rax+0x1d0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7aa2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b7aa6 mov rsi,rax
            rsi = rax;
            // b7aa9 call 0xb799c
            { let r = { let t = m.env.code_va(0xb799c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7aae test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // b7ab1 jne 0xb7aba
            if fl.ne() { pc = 0xb7aba; continue; }
            pc = 0xb7ab3;
        }
        0xb7ab3 => {
            // block b7ab3
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7ab3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ab3 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // b7ab8 jmp 0xb7ac9
            pc = 0xb7ac9; continue;
        }
        0xb7aba => {
            // block b7aba
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7aba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7aba mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b7abe mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7ac1 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7ac9;
        }
        0xb7ac9 => {
            // block b7ac9
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7ac9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ac9 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b7acb mov rdi,rbx
            rdi = rbx;
            // b7ace call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7ad3 mov rax,QWORD PTR [rbp-0x20]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // b7ad7 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // b7adb mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b7adf mov rcx,QWORD PTR [rbp-0x28]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b7ae3 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // b7ae7 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b7aea xorps xmm0,xmm0
            x0 = 0;
            // b7aed movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // b7af1 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // b7af9 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b7afd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7b02 movzx eax,BYTE PTR [rbx]
            rax = (m.r8(rbx) as u64) as u32 as u64;
            // b7b05 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // b7b07 jne 0xb7b0e
            if fl.ne() { pc = 0xb7b0e; continue; }
            pc = 0xb7b09;
        }
        0xb7b09 => {
            // block b7b09
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b09 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // b7b0c jmp 0xb7b12
            pc = 0xb7b12; continue;
        }
        0xb7b0e => {
            // block b7b0e
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b0e mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0xb7b12;
        }
        0xb7b12 => {
            // block b7b12
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b12 test rax,rax
            fl = logic::<64>(rax & rax);
            // b7b15 je 0xb7b49
            if fl.e() { pc = 0xb7b49; continue; }
            pc = 0xb7b17;
        }
        0xb7b17 => {
            // block b7b17
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b17 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b7b1a mov rdi,r14
            rdi = r14;
            // b7b1d call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7b23 cmp eax,0x10710800
            fl = sub::<32>((rax as u32 as u64), 0x10710800u64).1;
            // b7b28 jne 0xb7b60
            if fl.ne() { pc = 0xb7b60; continue; }
            pc = 0xb7b2a;
        }
        0xb7b2a => {
            // block b7b2a
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b2a lea rsi,[rip+0x802ca] # 0x137dfb
            rsi = m.env.data_va(312776);
            // b7b31 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b7b35 mov rdx,rbx
            rdx = rbx;
            // b7b38 call 0xebcf6
            { let r = { let t = m.env.stub_va(0x4c3); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7b3d test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // b7b40 jne 0xb7b90
            if fl.ne() { pc = 0xb7b90; continue; }
            pc = 0xb7b42;
        }
        0xb7b42 => {
            // block b7b42
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b42 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // b7b47 jmp 0xb7b9f
            pc = 0xb7b9f; continue;
        }
        0xb7b49 => {
            // block b7b49
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b49 lea rsi,[rip+0x802a2] # 0x137df2
            rsi = m.env.data_va(312767);
            // b7b50 mov rdi,rbx
            rdi = rbx;
            // b7b53 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7b58 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b7b5b jmp 0xb7c25
            pc = 0xb7c25; continue;
        }
        0xb7b60 => {
            // block b7b60
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b60 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b7b63 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // b7b66 cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // b7b6b jne 0xb7c25
            if fl.ne() { pc = 0xb7c25; continue; }
            pc = 0xb7b71;
        }
        0xb7b71 => {
            // block b7b71
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b71 lea rsi,[rip+0x7d2e8] # 0x134e60
            rsi = m.env.data_va(300589);
            // b7b78 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b7b7c mov rdx,rbx
            rdx = rbx;
            // b7b7f call 0xebcf6
            { let r = { let t = m.env.stub_va(0x4c3); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7b84 test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // b7b87 jne 0xb7bdd
            if fl.ne() { pc = 0xb7bdd; continue; }
            pc = 0xb7b89;
        }
        0xb7b89 => {
            // block b7b89
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b89 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // b7b8e jmp 0xb7bec
            pc = 0xb7bec; continue;
        }
        0xb7b90 => {
            // block b7b90
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b90 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b7b94 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7b97 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7b9f;
        }
        0xb7b9f => {
            // block b7b9f
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b9f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b7ba1 mov rdi,rbx
            rdi = rbx;
            // b7ba4 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7ba9 mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b7bad mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // b7bb1 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // b7bb5 mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b7bb9 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // b7bbd mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b7bc0 xorps xmm0,xmm0
            x0 = 0;
            // b7bc3 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // b7bc7 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // b7bcf lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b7bd3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7bd8 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // b7bdb jmp 0xb7c25
            pc = 0xb7c25; continue;
        }
        0xb7bdd => {
            // block b7bdd
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7bdd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7bdd mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b7be1 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7be4 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7bec;
        }
        0xb7bec => {
            // block b7bec
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7bec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7bec xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b7bee mov rdi,rbx
            rdi = rbx;
            // b7bf1 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7bf6 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // b7bfa mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // b7bfe mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // b7c02 mov rcx,QWORD PTR [rbp-0x68]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // b7c06 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // b7c0a mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b7c0d xorps xmm0,xmm0
            x0 = 0;
            // b7c10 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // b7c14 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // b7c1c lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b7c20 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb7c25;
        }
        0xb7c25 => {
            // block b7c25
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7c25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c25 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b7c28 add rsp,0x60
            { let (r, f) = add::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // b7c2c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7c2d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7c2f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7c30 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb7c31 => {
            // block b7c31
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7c31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c31 mov rdi,rax
            rdi = rax;
            // b7c34 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7c39 mov rdi,rax
            rdi = rax;
            // b7c3c call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7c41 mov rdi,rax
            rdi = rax;
            // b7c44 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbaafa size=414
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000baafa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbaafa;
    loop {
        match pc {
        0xbaafa => {
            // block baafa
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc baafa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baafa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // baafb mov rbp,rsp
            rbp = rsp;
            // baafe push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bab00 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bab02 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bab04 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bab06 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bab07 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bab0b mov r13,rsi
            r13 = rsi;
            // bab0e mov r14,rdi
            r14 = rdi;
            // bab11 mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // bab15 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bab19 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bab1c je 0xbab33
            if fl.e() { pc = 0xbab33; continue; }
            pc = 0xbab1e;
        }
        0xbab1e => {
            // block bab1e
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab1e lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bab22 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bab25 not rdx
            { let v = !rdx; rdx = v; }
            // bab28 and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bab2c add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bab2f mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            pc = 0xbab33;
        }
        0xbab33 => {
            // block bab33
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab33 mov r15,QWORD PTR [r14+0x8]
            r15 = m.r64(r14.wrapping_add(0x8)) as u64;
            // bab37 add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // bab3b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // bab3d cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // bab40 je 0xbac80
            if fl.e() { pc = 0xbac80; continue; }
            pc = 0xbab46;
        }
        0xbab46 => {
            // block bab46
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab46 mov edi,DWORD PTR [r15+0x1c]
            rdi = (m.r32(r15.wrapping_add(0x1c)) as u64) as u32 as u64;
            // bab4a mov r12d,DWORD PTR [r15+0x20]
            r12 = (m.r32(r15.wrapping_add(0x20)) as u64) as u32 as u64;
            // bab4e mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // bab55 mov DWORD PTR [rbp-0x30],0x3e7
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x3e7u64) as u32);
            // bab5c lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // bab60 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // bab64 call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bab69 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // bab6c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // bab6e sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // bab71 mov edx,DWORD PTR [rbp-0x30]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // bab74 cmp rdx,0x3
            fl = sub::<64>(rdx, 0x3u64).1;
            // bab78 ja 0xbac03
            if fl.a() { pc = 0xbac03; continue; }
            pc = 0xbab7e;
        }
        0xbab7e => {
            // block bab7e
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab7e xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // bab80 or al,bl
            { let r = (rax as u8 as u64) | (rbx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // bab82 jne 0xbac03
            if fl.ne() { pc = 0xbac03; continue; }
            pc = 0xbab84;
        }
        0xbab84 => {
            // block bab84
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab84 lea rax,[rip+0x47e75] # 0x102a00
            rax = m.env.data_va(94669);
            // bab8b mov eax,DWORD PTR [rax+rdx*4]
            rax = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // bab8e lea rsi,[rip+0x47e7b] # 0x102a10
            rsi = m.env.data_va(94685);
            // bab95 mov edx,DWORD PTR [rsi+rdx*4]
            rdx = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // bab98 mov DWORD PTR [rbp-0x40],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), ((rcx as u32 as u64)) as u32);
            // bab9b mov DWORD PTR [rbp-0x3c],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((r12 as u32 as u64)) as u32);
            // bab9f mov DWORD PTR [rbp-0x38],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rax as u32 as u64)) as u32);
            // baba2 mov DWORD PTR [rbp-0x34],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rdx as u32 as u64)) as u32);
            // baba5 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // baba9 cmp rax,QWORD PTR [r13+0x10]
            fl = sub::<64>(rax, m.r64(r13.wrapping_add(0x10)) as u64).1;
            // babad je 0xbabc5
            if fl.e() { pc = 0xbabc5; continue; }
            pc = 0xbabaf;
        }
        0xbabaf => {
            // block babaf
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babaf mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // babb3 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // babb7 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // babbb mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // babbe add QWORD PTR [r13+0x8],0x10
            { let (r, f) = add::<64>(m.r64(r13.wrapping_add(0x8)) as u64, 0x10u64);
              m.w64(r13.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // babc3 jmp 0xbabd1
            pc = 0xbabd1; continue;
        }
        0xbabc5 => {
            // block babc5
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babc5 mov rdi,r13
            rdi = r13;
            // babc8 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // babcc call 0xebb2e
            { let r = { let t = m.env.stub_va(0x2fb); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbabd1;
        }
        0xbabd1 => {
            // block babd1
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babd1 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // babd5 test rax,rax
            fl = logic::<64>(rax & rax);
            // babd8 je 0xbabe7
            if fl.e() { pc = 0xbabe7; continue; }
            pc = 0xbabda;
        }
        0xbabda => {
            // block babda
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babda mov r15,rax
            r15 = rax;
            // babdd mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // babe0 test rax,rax
            fl = logic::<64>(rax & rax);
            // babe3 jne 0xbabda
            if fl.ne() { pc = 0xbabda; continue; }
            pc = 0xbabe5;
        }
        0xbabe5 => {
            // block babe5
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babe5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babe5 jmp 0xbabf6
            pc = 0xbabf6; continue;
        }
        0xbabe7 => {
            // block babe7
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babe7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babe7 mov rax,r15
            rax = r15;
            pc = 0xbabea;
        }
        0xbabea => {
            // block babea
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babea mov r15,QWORD PTR [rax+0x10]
            r15 = m.r64(rax.wrapping_add(0x10)) as u64;
            // babee cmp QWORD PTR [r15],rax
            fl = sub::<64>(m.r64(r15) as u64, rax).1;
            // babf1 mov rax,r15
            rax = r15;
            // babf4 jne 0xbabea
            if fl.ne() { pc = 0xbabea; continue; }
            pc = 0xbabf6;
        }
        0xbabf6 => {
            // block babf6
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babf6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babf6 cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // babf9 jne 0xbab46
            if fl.ne() { pc = 0xbab46; continue; }
            pc = 0xbabff;
        }
        0xbabff => {
            // block babff
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babff mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // bac01 jmp 0xbac80
            pc = 0xbac80; continue;
        }
        0xbac03 => {
            // block bac03
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac03 mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // bac07 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bac0b cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bac0e je 0xbac7e
            if fl.e() { pc = 0xbac7e; continue; }
            pc = 0xbac10;
        }
        0xbac10 => {
            // block bac10
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac10 lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bac14 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bac17 not rdx
            { let v = !rdx; rdx = v; }
            // bac1a and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bac1e add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bac21 mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            // bac25 jmp 0xbac7e
            pc = 0xbac7e; continue;
        }
        0xbac27 => {
            // block bac27
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac27 mov r14,rax
            r14 = rax;
            // bac2a cmp edx,0x1
            fl = sub::<32>((rdx as u32 as u64), 0x1u64).1;
            // bac2d jne 0xbac8f
            if fl.ne() { pc = 0xbac8f; continue; }
            pc = 0xbac2f;
        }
        0xbac2f => {
            // block bac2f
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac2f mov rdi,r14
            rdi = r14;
            // bac32 call 0xec164
            { let r = { let t = m.env.stub_va(0x931); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bac37 mov rax,QWORD PTR [rip+0x8578a] # 0x1403c8
            rax = m.r64(m.env.data_va(347029)) as u64;
            // bac3e add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // bac42 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // bac46 mov rdi,r14
            rdi = r14;
            // bac49 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bac4e mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // bac52 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bac56 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bac59 je 0xbac70
            if fl.e() { pc = 0xbac70; continue; }
            pc = 0xbac5b;
        }
        0xbac5b => {
            // block bac5b
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac5b lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bac5f sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bac62 not rdx
            { let v = !rdx; rdx = v; }
            // bac65 and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bac69 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bac6c mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            pc = 0xbac70;
        }
        0xbac70 => {
            // block bac70
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac70 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // bac74 call 0xec128
            { let r = { let t = m.env.stub_va(0x8f5); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bac79 call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbac7e;
        }
        0xbac7e => {
            // block bac7e
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac7e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xbac80;
        }
        0xbac80 => {
            // block bac80
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac80 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bac84 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bac85 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bac87 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bac89 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bac8b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bac8d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bac8e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbac8f => {
            // block bac8f
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac8f mov rdi,r14
            rdi = r14;
            // bac92 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbe30 size=400
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbe30<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbe30;
    loop {
        match pc {
        0xbbe30 => {
            // block bbe30
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe30 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbe31 mov rbp,rsp
            rbp = rsp;
            // bbe34 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bbe36 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bbe38 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bbe3a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bbe3c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bbe3d sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bbe41 mov r15,rsi
            r15 = rsi;
            // bbe44 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // bbe48 movzx ecx,BYTE PTR [r15]
            rcx = (m.r8(r15) as u64) as u32 as u64;
            // bbe4c test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // bbe4f jne 0xbbe5a
            if fl.ne() { pc = 0xbbe5a; continue; }
            pc = 0xbbe51;
        }
        0xbbe51 => {
            // block bbe51
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe51 lea r14,[r15+0x1]
            r14 = r15.wrapping_add(0x1);
            // bbe55 shr rcx,1
            { let (r, f) = shr::<64>(rcx, 0x1u64, fl); rcx = r; fl = f; }
            // bbe58 jmp 0xbbe62
            pc = 0xbbe62; continue;
        }
        0xbbe5a => {
            // block bbe5a
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe5a mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // bbe5e mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0xbbe62;
        }
        0xbbe62 => {
            // block bbe62
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe62 movzx r13d,BYTE PTR [rip+0x10c726] # 0x1c8590
            r13 = (m.r8(m.env.data_va(904541)) as u64) as u32 as u64;
            // bbe6a test r13b,0x1
            fl = logic::<8>((r13 as u8 as u64) & 0x1u64);
            // bbe6e jne 0xbbe7c
            if fl.ne() { pc = 0xbbe7c; continue; }
            pc = 0xbbe70;
        }
        0xbbe70 => {
            // block bbe70
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe70 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // bbe73 lea r12,[rip+0x10c717] # 0x1c8591
            r12 = m.env.data_va(904542);
            // bbe7a jmp 0xbbe8a
            pc = 0xbbe8a; continue;
        }
        0xbbe7c => {
            // block bbe7c
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe7c mov r12,QWORD PTR [rip+0x10c71d] # 0x1c85a0
            r12 = m.r64(m.env.data_va(904557)) as u64;
            // bbe83 mov r13,QWORD PTR [rip+0x10c70e] # 0x1c8598
            r13 = m.r64(m.env.data_va(904549)) as u64;
            pc = 0xbbe8a;
        }
        0xbbe8a => {
            // block bbe8a
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe8a mov rbx,rcx
            rbx = rcx;
            // bbe8d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // bbe90 je 0xbbed5
            if fl.e() { pc = 0xbbed5; continue; }
            pc = 0xbbe92;
        }
        0xbbe92 => {
            // block bbe92
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe92 test r13,r13
            fl = logic::<64>(r13 & r13);
            // bbe95 je 0xbbeb1
            if fl.e() { pc = 0xbbeb1; continue; }
            pc = 0xbbe97;
        }
        0xbbe97 => {
            // block bbe97
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe97 movzx esi,BYTE PTR [r14+rbx*1-0x1]
            rsi = (m.r8(r14.wrapping_add(rbx).wrapping_add(0xffff_ffff_ffff_ffff)) as u64) as u32 as u64;
            // bbe9d mov rdi,r12
            rdi = r12;
            // bbea0 mov rdx,r13
            rdx = r13;
            // bbea3 call 0xec212
            { let r = { let t = m.env.stub_va(0x9df); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbea8 lea rcx,[rbx-0x1]
            rcx = rbx.wrapping_add(0xffff_ffff_ffff_ffff);
            // bbeac test rax,rax
            fl = logic::<64>(rax & rax);
            // bbeaf jne 0xbbe8a
            if fl.ne() { pc = 0xbbe8a; continue; }
            pc = 0xbbeb1;
        }
        0xbbeb1 => {
            // block bbeb1
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbeb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbeb1 lea rcx,[r14+rbx*1-0x1]
            rcx = r14.wrapping_add(rbx).wrapping_add(0xffff_ffff_ffff_ffff);
            // bbeb6 sub rcx,r14
            { let (r, f) = sub::<64>(rcx, r14);
              rcx = r;
              fl = f; }
            // bbeb9 cmp rcx,0xffffffffffffffff
            fl = sub::<64>(rcx, 0xffff_ffff_ffff_ffffu64).1;
            // bbebd je 0xbbed5
            if fl.e() { pc = 0xbbed5; continue; }
            pc = 0xbbebf;
        }
        0xbbebf => {
            // block bbebf
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbebf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbebf inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // bbec2 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bbec6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bbec8 mov rsi,r15
            rsi = r15;
            // bbecb mov r8,r15
            r8 = r15;
            // bbece call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbed3 jmp 0xbbee7
            pc = 0xbbee7; continue;
        }
        0xbbed5 => {
            // block bbed5
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbed5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbed5 lea rsi,[rip+0x7cdec] # 0x138cc8
            rsi = m.env.data_va(316565);
            // bbedc lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bbee0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bbee2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbbee7;
        }
        0xbbee7 => {
            // block bbee7
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbee7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbee7 movzx r13d,BYTE PTR [rbp-0x40]
            r13 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // bbeec test r13b,0x1
            fl = logic::<8>((r13 as u8 as u64) & 0x1u64);
            // bbef0 jne 0xbbefb
            if fl.ne() { pc = 0xbbefb; continue; }
            pc = 0xbbef2;
        }
        0xbbef2 => {
            // block bbef2
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbef2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbef2 lea r14,[rbp-0x3f]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // bbef6 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // bbef9 jmp 0xbbf03
            pc = 0xbbf03; continue;
        }
        0xbbefb => {
            // block bbefb
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbefb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbefb mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bbeff mov r14,QWORD PTR [rbp-0x30]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xbbf03;
        }
        0xbbf03 => {
            // block bbf03
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf03 movzx r12d,BYTE PTR [rip+0x10c685] # 0x1c8590
            r12 = (m.r8(m.env.data_va(904541)) as u64) as u32 as u64;
            // bbf0b test r12b,0x1
            fl = logic::<8>((r12 as u8 as u64) & 0x1u64);
            // bbf0f jne 0xbbf1d
            if fl.ne() { pc = 0xbbf1d; continue; }
            pc = 0xbbf11;
        }
        0xbbf11 => {
            // block bbf11
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf11 shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // bbf14 lea r15,[rip+0x10c676] # 0x1c8591
            r15 = m.env.data_va(904542);
            // bbf1b jmp 0xbbf2b
            pc = 0xbbf2b; continue;
        }
        0xbbf1d => {
            // block bbf1d
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf1d mov r15,QWORD PTR [rip+0x10c67c] # 0x1c85a0
            r15 = m.r64(m.env.data_va(904557)) as u64;
            // bbf24 mov r12,QWORD PTR [rip+0x10c66d] # 0x1c8598
            r12 = m.r64(m.env.data_va(904549)) as u64;
            pc = 0xbbf2b;
        }
        0xbbf2b => {
            // block bbf2b
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf2b test r13,r13
            fl = logic::<64>(r13 & r13);
            // bbf2e je 0xbbf7b
            if fl.e() { pc = 0xbbf7b; continue; }
            pc = 0xbbf30;
        }
        0xbbf30 => {
            // block bbf30
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf30 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xbbf32;
        }
        0xbbf32 => {
            // block bbf32
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf32 test r12,r12
            fl = logic::<64>(r12 & r12);
            // bbf35 je 0xbbf56
            if fl.e() { pc = 0xbbf56; continue; }
            pc = 0xbbf37;
        }
        0xbbf37 => {
            // block bbf37
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf37 movzx esi,BYTE PTR [r14+rbx*1]
            rsi = (m.r8(r14.wrapping_add(rbx)) as u64) as u32 as u64;
            // bbf3c mov rdi,r15
            rdi = r15;
            // bbf3f mov rdx,r12
            rdx = r12;
            // bbf42 call 0xec212
            { let r = { let t = m.env.stub_va(0x9df); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbf47 test rax,rax
            fl = logic::<64>(rax & rax);
            // bbf4a je 0xbbf56
            if fl.e() { pc = 0xbbf56; continue; }
            pc = 0xbbf4c;
        }
        0xbbf4c => {
            // block bbf4c
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf4c inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // bbf4f cmp r13,rbx
            fl = sub::<64>(r13, rbx).1;
            // bbf52 jne 0xbbf32
            if fl.ne() { pc = 0xbbf32; continue; }
            pc = 0xbbf54;
        }
        0xbbf54 => {
            // block bbf54
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf54 jmp 0xbbf7b
            pc = 0xbbf7b; continue;
        }
        0xbbf56 => {
            // block bbf56
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf56 cmp rbx,0xffffffffffffffff
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffffu64).1;
            // bbf5a je 0xbbf7b
            if fl.e() { pc = 0xbbf7b; continue; }
            pc = 0xbbf5c;
        }
        0xbbf5c => {
            // block bbf5c
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf5c lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bbf60 mov rcx,0xffffffffffffffff
            rcx = 0xffff_ffff_ffff_ffffu64;
            // bbf67 mov r14,QWORD PTR [rbp-0x48]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // bbf6b mov rdi,r14
            rdi = r14;
            // bbf6e mov rdx,rbx
            rdx = rbx;
            // bbf71 mov r8,rsi
            r8 = rsi;
            // bbf74 call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbf79 jmp 0xbbf90
            pc = 0xbbf90; continue;
        }
        0xbbf7b => {
            // block bbf7b
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf7b lea rsi,[rip+0x7cd46] # 0x138cc8
            rsi = m.env.data_va(316565);
            // bbf82 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bbf84 mov r14,QWORD PTR [rbp-0x48]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // bbf88 mov rdi,r14
            rdi = r14;
            // bbf8b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbbf90;
        }
        0xbbf90 => {
            // block bbf90
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf90 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bbf94 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbf99 mov rax,r14
            rax = r14;
            // bbf9c add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bbfa0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bbfa1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bbfa3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bbfa5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bbfa7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bbfa9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbfaa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbbfab => {
            // block bbfab
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbfab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbfab mov rbx,rax
            rbx = rax;
            // bbfae lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bbfb2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbfb7 mov rdi,rbx
            rdi = rbx;
            // bbfba call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc51ba size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c51ba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc51ba;
    loop {
        match pc {
        0xc51ba => {
            // block c51ba
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c51ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c51ba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c51bb mov rbp,rsp
            rbp = rsp;
            // c51be push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c51c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c51c2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c51c4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c51c6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c51c7 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c51cb mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c51cf test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c51d2 je 0xc530e
            if fl.e() { pc = 0xc530e; continue; }
            pc = 0xc51d8;
        }
        0xc51d8 => {
            // block c51d8
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c51d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c51d8 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // c51dc movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // c51df mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c51e2 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c51e6 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // c51e9 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // c51ed mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // c51f1 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // c51f5 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // c51f9 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // c51fd lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // c5201 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // c5205 jmp 0xc520a
            pc = 0xc520a; continue;
        }
        0xc5207 => {
            // block c5207
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5207 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5207 mov rbx,rax
            rbx = rax;
            pc = 0xc520a;
        }
        0xc520a => {
            // block c520a
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c520a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c520a test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c520d mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c5211 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // c5216 movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // c521a mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c521d and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c5221 jne 0xc522b
            if fl.ne() { pc = 0xc522b; continue; }
            pc = 0xc5223;
        }
        0xc5223 => {
            // block c5223
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5223 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5223 mov r14,rax
            r14 = rax;
            // c5226 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // c5229 jmp 0xc522f
            pc = 0xc522f; continue;
        }
        0xc522b => {
            // block c522b
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c522b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c522b mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xc522f;
        }
        0xc522f => {
            // block c522f
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c522f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c522f lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // c5233 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c5236 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c523a cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // c523f test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c5242 jne 0xc524e
            if fl.ne() { pc = 0xc524e; continue; }
            pc = 0xc5244;
        }
        0xc5244 => {
            // block c5244
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5244 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // c5248 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // c524c jmp 0xc5256
            pc = 0xc5256; continue;
        }
        0xc524e => {
            // block c524e
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c524e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c524e mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // c5252 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xc5256;
        }
        0xc5256 => {
            // block c5256
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5256 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5256 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // c5259 mov rdx,r13
            rdx = r13;
            // c525c cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // c5260 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c5263 je 0xc527a
            if fl.e() { pc = 0xc527a; continue; }
            pc = 0xc5265;
        }
        0xc5265 => {
            // block c5265
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5265 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // c5269 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c526e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c5270 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // c5274 je 0xc527a
            if fl.e() { pc = 0xc527a; continue; }
            pc = 0xc5276;
        }
        0xc5276 => {
            // block c5276
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5276 jns 0xc528c
            if fl.ns() { pc = 0xc528c; continue; }
            pc = 0xc5278;
        }
        0xc5278 => {
            // block c5278
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5278 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5278 jmp 0xc527f
            pc = 0xc527f; continue;
        }
        0xc527a => {
            // block c527a
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c527a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c527a cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // c527d jae 0xc528c
            if fl.ae() { pc = 0xc528c; continue; }
            pc = 0xc527f;
        }
        0xc527f => {
            // block c527f
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c527f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c527f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c5282 test rax,rax
            fl = logic::<64>(rax & rax);
            // c5285 jne 0xc5207
            if fl.ne() { pc = 0xc5207; continue; }
            pc = 0xc5287;
        }
        0xc5287 => {
            // block c5287
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5287 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5287 jmp 0xc5323
            pc = 0xc5323; continue;
        }
        0xc528c => {
            // block c528c
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c528c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c528c test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c528f jne 0xc5299
            if fl.ne() { pc = 0xc5299; continue; }
            pc = 0xc5291;
        }
        0xc5291 => {
            // block c5291
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5291 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5291 mov r14,rbx
            r14 = rbx;
            // c5294 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // c5297 jmp 0xc52a0
            pc = 0xc52a0; continue;
        }
        0xc5299 => {
            // block c5299
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5299 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5299 mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // c529d mov r14,rbx
            r14 = rbx;
            pc = 0xc52a0;
        }
        0xc52a0 => {
            // block c52a0
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52a0 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // c52a4 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c52a7 mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c52ab cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // c52b0 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c52b3 jne 0xc52ba
            if fl.ne() { pc = 0xc52ba; continue; }
            pc = 0xc52b5;
        }
        0xc52b5 => {
            // block c52b5
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52b5 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // c52b8 jmp 0xc52be
            pc = 0xc52be; continue;
        }
        0xc52ba => {
            // block c52ba
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52ba mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0xc52be;
        }
        0xc52be => {
            // block c52be
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52be cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // c52c1 mov rdx,rax
            rdx = rax;
            // c52c4 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // c52c8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c52cb je 0xc52ec
            if fl.e() { pc = 0xc52ec; continue; }
            pc = 0xc52cd;
        }
        0xc52cd => {
            // block c52cd
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52cd test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c52d0 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c52d4 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // c52d9 mov r12,rax
            r12 = rax;
            // c52dc call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c52e1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c52e3 mov rax,r12
            rax = r12;
            // c52e6 je 0xc52ec
            if fl.e() { pc = 0xc52ec; continue; }
            pc = 0xc52e8;
        }
        0xc52e8 => {
            // block c52e8
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52e8 js 0xc52f1
            if fl.s() { pc = 0xc52f1; continue; }
            pc = 0xc52ea;
        }
        0xc52ea => {
            // block c52ea
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52ea jmp 0xc531a
            pc = 0xc531a; continue;
        }
        0xc52ec => {
            // block c52ec
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52ec cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // c52ef jae 0xc531a
            if fl.ae() { pc = 0xc531a; continue; }
            pc = 0xc52f1;
        }
        0xc52f1 => {
            // block c52f1
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52f1 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // c52f5 test rax,rax
            fl = logic::<64>(rax & rax);
            // c52f8 jne 0xc5207
            if fl.ne() { pc = 0xc5207; continue; }
            pc = 0xc52fe;
        }
        0xc52fe => {
            // block c52fe
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52fe mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c5302 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // c5305 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c5309 mov rax,r14
            rax = r14;
            // c530c jmp 0xc532d
            pc = 0xc532d; continue;
        }
        0xc530e => {
            // block c530e
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c530e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c530e add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c5312 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c5315 mov rax,rdi
            rax = rdi;
            // c5318 jmp 0xc532d
            pc = 0xc532d; continue;
        }
        0xc531a => {
            // block c531a
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c531a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c531a mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c531e mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // c5321 jmp 0xc532d
            pc = 0xc532d; continue;
        }
        0xc5323 => {
            // block c5323
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5323 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5323 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c5327 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // c532a mov rax,rbx
            rax = rbx;
            pc = 0xc532d;
        }
        0xc532d => {
            // block c532d
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c532d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c532d add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c5331 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5332 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c5334 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c5336 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c5338 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c533a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c533b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc548e size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c548e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc548e;
    loop {
        match pc {
        0xc548e => {
            // block c548e
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c548e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c548e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c548f mov rbp,rsp
            rbp = rsp;
            // c5492 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c5494 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c5496 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c5498 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c549a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c549b sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c549f mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c54a3 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c54a6 je 0xc55e2
            if fl.e() { pc = 0xc55e2; continue; }
            pc = 0xc54ac;
        }
        0xc54ac => {
            // block c54ac
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c54ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c54ac mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // c54b0 movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // c54b3 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c54b6 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c54ba shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // c54bd mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // c54c1 mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // c54c5 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // c54c9 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // c54cd mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // c54d1 lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // c54d5 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // c54d9 jmp 0xc54de
            pc = 0xc54de; continue;
        }
        0xc54db => {
            // block c54db
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c54db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c54db mov rbx,rax
            rbx = rax;
            pc = 0xc54de;
        }
        0xc54de => {
            // block c54de
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c54de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c54de test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c54e1 mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c54e5 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // c54ea movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // c54ee mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c54f1 and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c54f5 jne 0xc54ff
            if fl.ne() { pc = 0xc54ff; continue; }
            pc = 0xc54f7;
        }
        0xc54f7 => {
            // block c54f7
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c54f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c54f7 mov r14,rax
            r14 = rax;
            // c54fa shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // c54fd jmp 0xc5503
            pc = 0xc5503; continue;
        }
        0xc54ff => {
            // block c54ff
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c54ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c54ff mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xc5503;
        }
        0xc5503 => {
            // block c5503
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5503 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5503 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // c5507 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c550a mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c550e cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // c5513 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c5516 jne 0xc5522
            if fl.ne() { pc = 0xc5522; continue; }
            pc = 0xc5518;
        }
        0xc5518 => {
            // block c5518
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5518 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5518 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // c551c mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // c5520 jmp 0xc552a
            pc = 0xc552a; continue;
        }
        0xc5522 => {
            // block c5522
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5522 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5522 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // c5526 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xc552a;
        }
        0xc552a => {
            // block c552a
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c552a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c552a cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // c552d mov rdx,r13
            rdx = r13;
            // c5530 cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // c5534 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c5537 je 0xc554e
            if fl.e() { pc = 0xc554e; continue; }
            pc = 0xc5539;
        }
        0xc5539 => {
            // block c5539
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5539 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5539 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // c553d call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5542 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c5544 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // c5548 je 0xc554e
            if fl.e() { pc = 0xc554e; continue; }
            pc = 0xc554a;
        }
        0xc554a => {
            // block c554a
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c554a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c554a jns 0xc5560
            if fl.ns() { pc = 0xc5560; continue; }
            pc = 0xc554c;
        }
        0xc554c => {
            // block c554c
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c554c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c554c jmp 0xc5553
            pc = 0xc5553; continue;
        }
        0xc554e => {
            // block c554e
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c554e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c554e cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // c5551 jae 0xc5560
            if fl.ae() { pc = 0xc5560; continue; }
            pc = 0xc5553;
        }
        0xc5553 => {
            // block c5553
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5553 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5553 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c5556 test rax,rax
            fl = logic::<64>(rax & rax);
            // c5559 jne 0xc54db
            if fl.ne() { pc = 0xc54db; continue; }
            pc = 0xc555b;
        }
        0xc555b => {
            // block c555b
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c555b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c555b jmp 0xc55f7
            pc = 0xc55f7; continue;
        }
        0xc5560 => {
            // block c5560
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5560 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c5563 jne 0xc556d
            if fl.ne() { pc = 0xc556d; continue; }
            pc = 0xc5565;
        }
        0xc5565 => {
            // block c5565
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5565 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5565 mov r14,rbx
            r14 = rbx;
            // c5568 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // c556b jmp 0xc5574
            pc = 0xc5574; continue;
        }
        0xc556d => {
            // block c556d
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c556d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c556d mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // c5571 mov r14,rbx
            r14 = rbx;
            pc = 0xc5574;
        }
        0xc5574 => {
            // block c5574
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5574 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5574 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // c5578 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c557b mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c557f cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // c5584 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c5587 jne 0xc558e
            if fl.ne() { pc = 0xc558e; continue; }
            pc = 0xc5589;
        }
        0xc5589 => {
            // block c5589
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5589 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5589 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // c558c jmp 0xc5592
            pc = 0xc5592; continue;
        }
        0xc558e => {
            // block c558e
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c558e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c558e mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0xc5592;
        }
        0xc5592 => {
            // block c5592
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5592 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5592 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // c5595 mov rdx,rax
            rdx = rax;
            // c5598 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // c559c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c559f je 0xc55c0
            if fl.e() { pc = 0xc55c0; continue; }
            pc = 0xc55a1;
        }
        0xc55a1 => {
            // block c55a1
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55a1 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c55a4 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c55a8 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // c55ad mov r12,rax
            r12 = rax;
            // c55b0 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c55b5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c55b7 mov rax,r12
            rax = r12;
            // c55ba je 0xc55c0
            if fl.e() { pc = 0xc55c0; continue; }
            pc = 0xc55bc;
        }
        0xc55bc => {
            // block c55bc
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55bc js 0xc55c5
            if fl.s() { pc = 0xc55c5; continue; }
            pc = 0xc55be;
        }
        0xc55be => {
            // block c55be
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55be jmp 0xc55ee
            pc = 0xc55ee; continue;
        }
        0xc55c0 => {
            // block c55c0
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55c0 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // c55c3 jae 0xc55ee
            if fl.ae() { pc = 0xc55ee; continue; }
            pc = 0xc55c5;
        }
        0xc55c5 => {
            // block c55c5
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55c5 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // c55c9 test rax,rax
            fl = logic::<64>(rax & rax);
            // c55cc jne 0xc54db
            if fl.ne() { pc = 0xc54db; continue; }
            pc = 0xc55d2;
        }
        0xc55d2 => {
            // block c55d2
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55d2 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c55d6 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // c55d9 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c55dd mov rax,r14
            rax = r14;
            // c55e0 jmp 0xc5601
            pc = 0xc5601; continue;
        }
        0xc55e2 => {
            // block c55e2
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55e2 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c55e6 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c55e9 mov rax,rdi
            rax = rdi;
            // c55ec jmp 0xc5601
            pc = 0xc5601; continue;
        }
        0xc55ee => {
            // block c55ee
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55ee mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c55f2 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // c55f5 jmp 0xc5601
            pc = 0xc5601; continue;
        }
        0xc55f7 => {
            // block c55f7
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c55f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c55f7 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c55fb mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // c55fe mov rax,rbx
            rax = rbx;
            pc = 0xc5601;
        }
        0xc5601 => {
            // block c5601
            #[cfg(feature = "trace")] eprintln!("f_000c548e pc c5601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5601 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c5605 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5606 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c5608 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c560a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c560c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c560e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c560f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7586 size=478
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7586<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd7586;
    loop {
        match pc {
        0xd7586 => {
            // block d7586
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7586 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7586 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7587 mov rbp,rsp
            rbp = rsp;
            // d758a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d758c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d758e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d7590 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d7592 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d7593 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d7597 mov r14,rdi
            r14 = rdi;
            // d759a mov rdi,QWORD PTR [r14+0x6b8]
            rdi = m.r64(r14.wrapping_add(0x6b8)) as u64;
            // d75a1 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d75a4 je 0xd7642
            if fl.e() { pc = 0xd7642; continue; }
            pc = 0xd75aa;
        }
        0xd75aa => {
            // block d75aa
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d75aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d75aa xorps xmm0,xmm0
            x0 = 0;
            // d75ad movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d75b1 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // d75b9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d75bc mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // d75c0 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d75c2 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d75c5 mov rdi,QWORD PTR [r14+0x6b8]
            rdi = m.r64(r14.wrapping_add(0x6b8)) as u64;
            // d75cc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d75cf mov rax,QWORD PTR [rax+0x40]
            rax = m.r64(rax.wrapping_add(0x40)) as u64;
            // d75d3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d75d5 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d75d7 mov rdi,QWORD PTR [r14+0x6b8]
            rdi = m.r64(r14.wrapping_add(0x6b8)) as u64;
            // d75de mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d75e1 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d75e5 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d75ea call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d75ec mov rdi,QWORD PTR [r14+0x6b8]
            rdi = m.r64(r14.wrapping_add(0x6b8)) as u64;
            // d75f3 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // d75f6 mov rbx,QWORD PTR [rcx+0x10]
            rbx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // d75fa lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d75fe mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // d7603 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d7605 call rbx
            { let r = { let t = rbx; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7607 mov rdi,QWORD PTR [r14+0x6b8]
            rdi = m.r64(r14.wrapping_add(0x6b8)) as u64;
            // d760e mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // d7611 mov rcx,QWORD PTR [rcx+0x40]
            rcx = m.r64(rcx.wrapping_add(0x40)) as u64;
            // d7615 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7617 je 0xd770e
            if fl.e() { pc = 0xd770e; continue; }
            pc = 0xd761d;
        }
        0xd761d => {
            // block d761d
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d761d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d761d movzx esi,r15b
            rsi = ((r15 as u8 as u64)) as u32 as u64;
            // d7621 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7623 add r14,0x318
            { let (r, f) = add::<64>(r14, 0x318u64);
              r14 = r;
              fl = f; }
            // d762a mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d762e mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d7632 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // d7635 mov rdi,r14
            rdi = r14;
            // d7638 call 0xebeb2
            { let r = { let t = m.env.stub_va(0x67f); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d763d jmp 0xd7717
            pc = 0xd7717; continue;
        }
        0xd7642 => {
            // block d7642
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7642 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d7645 cmp DWORD PTR [r14+0x330],0x10220
            fl = sub::<32>(m.r32(r14.wrapping_add(0x330)) as u64, 0x10220u64).1;
            // d7650 jne 0xd772f
            if fl.ne() { pc = 0xd772f; continue; }
            pc = 0xd7656;
        }
        0xd7656 => {
            // block d7656
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7656 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7656 mov r12d,DWORD PTR [r14+0x34c]
            r12 = (m.r32(r14.wrapping_add(0x34c)) as u64) as u32 as u64;
            // d765d mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // d7662 mov rdi,r14
            rdi = r14;
            // d7665 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d766a mov edi,0x100
            rdi = (0x100u64) as u32 as u64;
            // d766f call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7674 mov r15,rax
            r15 = rax;
            // d7677 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7679;
        }
        0xd7679 => {
            // block d7679
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7679 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7679 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d767c lea edx,[r12+rbx*1]
            rdx = (r12.wrapping_add(rbx)) as u32 as u64;
            // d7680 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d7685 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d768a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d768d mov rdi,r14
            rdi = r14;
            // d7690 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7693 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d7696 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d769b mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d76a0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d76a5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d76a8 mov rdi,r14
            rdi = r14;
            // d76ab call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d76ae mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d76b1 mov rdi,r14
            rdi = r14;
            // d76b4 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d76ba mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d76bd lea rdx,[r15+rbx*1]
            rdx = r15.wrapping_add(rbx);
            // d76c1 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d76c6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d76cb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d76ce mov rdi,r14
            rdi = r14;
            // d76d1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d76d4 cmp DWORD PTR [r15+rbx*1],0x0
            fl = sub::<32>(m.r32(r15.wrapping_add(rbx)) as u64, 0x0u64).1;
            // d76d9 je 0xd76e8
            if fl.e() { pc = 0xd76e8; continue; }
            pc = 0xd76db;
        }
        0xd76db => {
            // block d76db
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d76db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d76db inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // d76de add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // d76e2 cmp r13d,0x40
            fl = sub::<32>((r13 as u32 as u64), 0x40u64).1;
            // d76e6 jb 0xd7679
            if fl.b() { pc = 0xd7679; continue; }
            pc = 0xd76e8;
        }
        0xd76e8 => {
            // block d76e8
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d76e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d76e8 lea rdi,[r14+0x318]
            rdi = r14.wrapping_add(0x318);
            // d76ef mov rsi,r15
            rsi = r15;
            // d76f2 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d76f7 mov rdi,r15
            rdi = r15;
            // d76fa call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d76ff xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7701 mov rdi,r14
            rdi = r14;
            // d7704 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7709 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // d770c jmp 0xd772f
            pc = 0xd772f; continue;
        }
        0xd770e => {
            // block d770e
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d770e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d770e movzx esi,r15b
            rsi = ((r15 as u8 as u64)) as u32 as u64;
            // d7712 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7714 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7717;
        }
        0xd7717 => {
            // block d7717
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7717 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7717 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d771b test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d771e je 0xd772f
            if fl.e() { pc = 0xd772f; continue; }
            pc = 0xd7720;
        }
        0xd7720 => {
            // block d7720
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7720 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7720 cmp QWORD PTR [rbp-0x38],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, rdi).1;
            // d7724 je 0xd772a
            if fl.e() { pc = 0xd772a; continue; }
            pc = 0xd7726;
        }
        0xd7726 => {
            // block d7726
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7726 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            pc = 0xd772a;
        }
        0xd772a => {
            // block d772a
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d772a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d772a call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd772f;
        }
        0xd772f => {
            // block d772f
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d772f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d772f mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // d7732 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d7736 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7737 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d7739 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d773b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d773d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d773f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7740 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd7741 => {
            // block d7741
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7741 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7741 mov rbx,rax
            rbx = rax;
            // d7744 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d7748 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d774b je 0xd775c
            if fl.e() { pc = 0xd775c; continue; }
            pc = 0xd774d;
        }
        0xd774d => {
            // block d774d
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d774d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d774d cmp QWORD PTR [rbp-0x38],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, rdi).1;
            // d7751 je 0xd7757
            if fl.e() { pc = 0xd7757; continue; }
            pc = 0xd7753;
        }
        0xd7753 => {
            // block d7753
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7753 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7753 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            pc = 0xd7757;
        }
        0xd7757 => {
            // block d7757
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d7757 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7757 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd775c;
        }
        0xd775c => {
            // block d775c
            #[cfg(feature = "trace")] eprintln!("f_000d7586 pc d775c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d775c mov rdi,rbx
            rdi = rbx;
            // d775f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8cba size=466
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000d8cba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd8cba;
    loop {
        match pc {
        0xd8cba => {
            // block d8cba
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8cba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8cba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8cbb mov rbp,rsp
            rbp = rsp;
            // d8cbe push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d8cc0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d8cc2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d8cc4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d8cc6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d8cc7 sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // d8cce mov r13,rdi
            r13 = rdi;
            // d8cd1 mov rax,QWORD PTR [rip+0x67700] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // d8cd8 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d8cdb mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // d8cdf mov ecx,0x2000050
            rcx = (0x2000050u64) as u32 as u64;
            // d8ce4 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8ce7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd8ce9;
        }
        0xd8ce9 => {
            // block d8ce9
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8ce9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8ce9 mov QWORD PTR [rbp-0xb0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (rcx) as u64);
            // d8cf0 mov DWORD PTR [rbp-0xbc],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff44), ((rax as u32 as u64)) as u32);
            // d8cf6 mov QWORD PTR [rbp-0xb8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (r15) as u64);
            // d8cfd mov BYTE PTR [rbp-0xa0],0x3a
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x3au64) as u8);
            // d8d04 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // d8d09 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8d0b lea rdi,[rbp-0x9f]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff61);
            // d8d12 lea rbx,[rip+0x6018a] # 0x138ea3
            rbx = m.env.data_va(317040);
            // d8d19 mov rsi,rbx
            rsi = rbx;
            // d8d1c call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d21 mov r14d,r15d
            r14 = ((r15 as u32 as u64)) as u32 as u64;
            // d8d24 shr r14d,0x8
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x8u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // d8d28 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // d8d2c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8d2e lea rdi,[rbp-0x9d]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff63);
            // d8d35 mov rsi,rbx
            rsi = rbx;
            // d8d38 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d3d movzx edx,r15b
            rdx = ((r15 as u8 as u64)) as u32 as u64;
            // d8d41 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8d43 lea rdi,[rbp-0x9b]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff65);
            // d8d4a mov rsi,rbx
            rsi = rbx;
            // d8d4d call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d52 lea r14d,[r15+r14*1+0x10]
            r14 = (r15.wrapping_add(r14).wrapping_add(0x10)) as u32 as u64;
            // d8d57 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d8d59 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8d5b lea rdi,[rbp-0x99]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff67);
            // d8d62 mov rsi,rbx
            rsi = rbx;
            // d8d65 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d6a xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8d6d lea r12,[rbp-0x97]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff69);
            pc = 0xd8d74;
        }
        0xd8d74 => {
            // block d8d74
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8d74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8d74 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d8d78 mov rcx,QWORD PTR [rbp-0xb0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // d8d7f lea edx,[rcx+r15*1]
            rdx = (rcx.wrapping_add(r15)) as u32 as u64;
            // d8d83 mov esi,0x5b
            rsi = (0x5bu64) as u32 as u64;
            // d8d88 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8d8d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8d90 mov rdi,r13
            rdi = r13;
            // d8d93 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d96 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d8d9a mov esi,0x64
            rsi = (0x64u64) as u32 as u64;
            // d8d9f mov rdi,r13
            rdi = r13;
            // d8da2 call QWORD PTR [rax+0x120]
            { let r = { let t = m.r64(rax.wrapping_add(0x120)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8da8 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d8dac mov esi,0x5c
            rsi = (0x5cu64) as u32 as u64;
            // d8db1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8db6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8db9 mov rdi,r13
            rdi = r13;
            // d8dbc lea rdx,[rbp-0xa4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff5c);
            // d8dc3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8dc6 mov eax,DWORD PTR [rbp-0xa4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c)) as u64) as u32 as u64;
            // d8dcc movzx ebx,ah
            rbx = (((rax >> 8) as u8 as u64)) as u32 as u64;
            // d8dcf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8dd1 mov rdi,r12
            rdi = r12;
            // d8dd4 lea rsi,[rip+0x600c8] # 0x138ea3
            rsi = m.env.data_va(317040);
            // d8ddb mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d8ddd call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8de2 movzx r14d,r14b
            r14 = ((r14 as u8 as u64)) as u32 as u64;
            // d8de6 add r14d,ebx
            { let (r, f) = add::<32>((r14 as u32 as u64), (rbx as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // d8de9 add r12,0x2
            { let (r, f) = add::<64>(r12, 0x2u64);
              r12 = r;
              fl = f; }
            // d8ded add r15d,0x100
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x100u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8df4 cmp r15d,0x1000
            fl = sub::<32>((r15 as u32 as u64), 0x1000u64).1;
            // d8dfb jne 0xd8d74
            if fl.ne() { pc = 0xd8d74; continue; }
            pc = 0xd8e01;
        }
        0xd8e01 => {
            // block d8e01
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8e01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8e01 mov r15,QWORD PTR [rbp-0xb8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // d8e08 movzx r15d,r15w
            r15 = ((r15 as u16 as u64)) as u32 as u64;
            // d8e0c mov rbx,QWORD PTR [rbp-0xb0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // d8e13 add ebx,0x1000
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x1000u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d8e19 add r15d,0x10
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x10u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8e1d neg r14d
            { let (r, f) = neg::<32>((r14 as u32 as u64)); r14 = (r) as u32 as u64; fl = f; }
            // d8e20 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // d8e24 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8e26 lea rdi,[rbp-0x77]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff89);
            // d8e2a lea rsi,[rip+0x60072] # 0x138ea3
            rsi = m.env.data_va(317040);
            // d8e31 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8e36 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d8e3d call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8e42 mov eax,DWORD PTR [rbp-0xbc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff44)) as u64) as u32 as u64;
            // d8e48 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d8e4a cmp eax,0x10
            fl = sub::<32>((rax as u32 as u64), 0x10u64).1;
            // d8e4d mov rcx,rbx
            rcx = rbx;
            // d8e50 jne 0xd8ce9
            if fl.ne() { pc = 0xd8ce9; continue; }
            pc = 0xd8e56;
        }
        0xd8e56 => {
            // block d8e56
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8e56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8e56 lea rdi,[rip+0x6005a] # 0x138eb7
            rdi = m.env.data_va(317060);
            // d8e5d call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8e62 mov rax,QWORD PTR [rip+0x6756f] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // d8e69 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d8e6c cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // d8e70 jne 0xd8e86
            if fl.ne() { pc = 0xd8e86; continue; }
            pc = 0xd8e72;
        }
        0xd8e72 => {
            // block d8e72
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8e72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8e72 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d8e74 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // d8e7b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d8e7c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d8e7e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d8e80 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d8e82 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d8e84 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8e85 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd8e86 => {
            // block d8e86
            #[cfg(feature = "trace")] eprintln!("f_000d8cba pc d8e86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8e86 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdfdb4 size=483
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dfdb4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdfdb4;
    loop {
        match pc {
        0xdfdb4 => {
            // block dfdb4
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dfdb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfdb4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dfdb5 mov rbp,rsp
            rbp = rsp;
            // dfdb8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dfdba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dfdbc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dfdbe push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dfdc0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dfdc1 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // dfdc3 mov DWORD PTR [rdi-0x4],eax
            m.w32(rdi.wrapping_add(0xffff_ffff_ffff_fffc), ((rax as u32 as u64)) as u32);
            // dfdc6 movsxd rbx,r8d
            rbx = sx::<32>((r8 as u32 as u64)) as u64;
            // dfdc9 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // dfdcc movsxd r10,eax
            r10 = sx::<32>((rax as u32 as u64)) as u64;
            // dfdcf lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // dfdd2 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // dfdd4 mov eax,DWORD PTR [rdi+rax*4]
            rax = (m.r32(rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // dfdd7 mov DWORD PTR [rdi+r10*4],eax
            m.w32(rdi.wrapping_add(r10.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // dfddb mov DWORD PTR [rdi+rbx*4],eax
            m.w32(rdi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // dfdde imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfde2 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // dfde4 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dfde5 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // dfde7 mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // dfdea imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // dfdee mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // dfdf0 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dfdf1 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // dfdf3 mov r10d,eax
            r10 = ((rax as u32 as u64)) as u32 as u64;
            // dfdf6 cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // dfdf9 jge 0xdff8c
            if fl.ge() { pc = 0xdff8c; continue; }
            pc = 0xdfdff;
        }
        0xdfdff => {
            // block dfdff
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dfdff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfdff shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // dfe03 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // dfe06 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dfe07 idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // dfe0a mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // dfe0d sub r10d,r11d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // dfe10 movsxd rcx,r11d
            rcx = sx::<32>((r11 as u32 as u64)) as u64;
            // dfe13 lea r13,[rsi+rcx*4]
            r13 = rsi.wrapping_add(rcx.wrapping_mul(4));
            // dfe17 imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            pc = 0xdfe1b;
        }
        0xdfe1b => {
            // block dfe1b
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dfe1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfe1b mov ecx,r11d
            rcx = ((r11 as u32 as u64)) as u32 as u64;
            // dfe1e sar ecx,0x10
            { let (r, f) = sar::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // dfe21 mov r9d,r11d
            r9 = ((r11 as u32 as u64)) as u32 as u64;
            // dfe24 shr r9d,0xb
            { let (r, f) = shr::<32>((r9 as u32 as u64), 0xbu64, fl); r9 = (r) as u32 as u64; fl = f; }
            // dfe28 and r9d,0x1f
            { let r = (r9 as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // dfe2c movsxd rsi,ecx
            rsi = sx::<32>((rcx as u32 as u64)) as u64;
            // dfe2f movzx r8d,BYTE PTR [rdi+rsi*4-0x2]
            r8 = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u32 as u64;
            // dfe35 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // dfe3a sub ecx,r9d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r9 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // dfe3d lea rax,[rip+0x481dc] # 0x128020
            rax = m.env.data_va(247789);
            // dfe44 mov r12d,DWORD PTR [rax+rcx*4]
            r12 = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe48 imul r8d,r12d
            { let (r, f) = imul::<32>((r8 as u32 as u64), (r12 as u32 as u64)); r8 = (r) as u32 as u64; fl = f; }
            // dfe4c movzx ebx,BYTE PTR [rdi+rsi*4+0x2]
            rbx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x2)) as u64) as u32 as u64;
            // dfe51 mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // dfe56 sub ecx,r9d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r9 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // dfe59 mov ecx,DWORD PTR [rax+rcx*4]
            rcx = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe5c imul ebx,ecx
            { let (r, f) = imul::<32>((rbx as u32 as u64), (rcx as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // dfe5f add ebx,r8d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r8 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // dfe62 movzx r14d,BYTE PTR [rdi+rsi*4+0x6]
            r14 = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x6)) as u64) as u32 as u64;
            // dfe68 mov edx,0x60
            rdx = (0x60u64) as u32 as u64;
            // dfe6d sub edx,r9d
            { let (r, f) = sub::<32>((rdx as u32 as u64), (r9 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfe70 mov r8d,DWORD PTR [rax+rdx*4]
            r8 = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe74 imul r14d,r8d
            { let (r, f) = imul::<32>((r14 as u32 as u64), (r8 as u32 as u64)); r14 = (r) as u32 as u64; fl = f; }
            // dfe78 add r14d,ebx
            { let (r, f) = add::<32>((r14 as u32 as u64), (rbx as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // dfe7b movzx ebx,BYTE PTR [rdi+rsi*4+0xa]
            rbx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa)) as u64) as u32 as u64;
            // dfe80 mov edx,0x80
            rdx = (0x80u64) as u32 as u64;
            // dfe85 sub edx,r9d
            { let (r, f) = sub::<32>((rdx as u32 as u64), (r9 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfe88 mov r9d,DWORD PTR [rax+rdx*4]
            r9 = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe8c imul ebx,r9d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r9 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // dfe90 add ebx,r14d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r14 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // dfe93 movzx edx,BYTE PTR [rdi+rsi*4-0x3]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffd)) as u64) as u32 as u64;
            // dfe98 imul edx,r12d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r12 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfe9c movzx eax,BYTE PTR [rdi+rsi*4+0x1]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x1)) as u64) as u32 as u64;
            // dfea1 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfea4 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dfea6 movzx edx,BYTE PTR [rdi+rsi*4+0x5]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x5)) as u64) as u32 as u64;
            // dfeab imul edx,r8d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r8 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfeaf add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfeb1 movzx eax,BYTE PTR [rdi+rsi*4+0x9]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x9)) as u64) as u32 as u64;
            // dfeb6 imul eax,r9d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r9 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfeba add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dfebc mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // dfebf shr r14d,0x8
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x8u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // dfec3 and r14d,0xff00
            { let r = (r14 as u32 as u64) & 0xff00u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // dfeca cmp eax,0xffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffu64).1;
            // dfecf mov eax,0xff00
            rax = (0xff00u64) as u32 as u64;
            // dfed4 cmova r14d,eax
            if fl.a() { r14 = ((rax as u32 as u64)) as u32 as u64; } else { r14 = r14 as u32 as u64; }
            // dfed8 movzx eax,BYTE PTR [rdi+rsi*4-0x4]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // dfedd imul eax,r12d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r12 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfee1 movzx edx,BYTE PTR [rdi+rsi*4]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // dfee5 imul edx,ecx
            { let (r, f) = imul::<32>((rdx as u32 as u64), (rcx as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfee8 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfeea movzx eax,BYTE PTR [rdi+rsi*4+0x4]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x4)) as u64) as u32 as u64;
            // dfeef imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfef3 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dfef5 movzx edx,BYTE PTR [rdi+rsi*4+0x8]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x8)) as u64) as u32 as u64;
            // dfefa imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfefe add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dff00 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // dff02 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // dff05 cmp edx,0xffffff
            fl = sub::<32>((rdx as u32 as u64), 0xffffffu64).1;
            // dff0b movzx r15d,al
            r15 = ((rax as u8 as u64)) as u32 as u64;
            // dff0f mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // dff14 cmova r15d,eax
            if fl.a() { r15 = ((rax as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // dff18 movzx eax,BYTE PTR [rdi+rsi*4-0x1]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ffff)) as u64) as u32 as u64;
            // dff1d imul eax,r12d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r12 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dff21 movzx edx,BYTE PTR [rdi+rsi*4+0x3]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x3)) as u64) as u32 as u64;
            // dff26 imul edx,ecx
            { let (r, f) = imul::<32>((rdx as u32 as u64), (rcx as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dff29 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dff2b movzx eax,BYTE PTR [rdi+rsi*4+0x7]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x7)) as u64) as u32 as u64;
            // dff30 imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dff34 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dff36 movzx ecx,BYTE PTR [rdi+rsi*4+0xb]
            rcx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xb)) as u64) as u32 as u64;
            // dff3b imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // dff3f add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff41 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // dff43 shl eax,0x8
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // dff46 and eax,0xff000000
            { let r = (rax as u32 as u64) & 0xff000000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dff4b cmp ecx,0xffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffu64).1;
            // dff51 mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // dff56 cmova eax,ecx
            if fl.a() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // dff59 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // dff5b and ecx,0xff0000
            { let r = (rcx as u32 as u64) & 0xff0000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff61 cmp ebx,0xffffff
            fl = sub::<32>((rbx as u32 as u64), 0xffffffu64).1;
            // dff67 mov edx,0xff0000
            rdx = (0xff0000u64) as u32 as u64;
            // dff6c cmova ecx,edx
            if fl.a() { rcx = ((rdx as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // dff6f or ecx,r14d
            { let r = (rcx as u32 as u64) | (r14 as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff72 or ecx,r15d
            { let r = (rcx as u32 as u64) | (r15 as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff75 or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff77 mov DWORD PTR [r13+0x0],ecx
            m.w32(r13, ((rcx as u32 as u64)) as u32);
            // dff7b add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // dff7f add r11d,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // dff83 dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // dff86 jne 0xdfe1b
            if fl.ne() { pc = 0xdfe1b; continue; }
            pc = 0xdff8c;
        }
        0xdff8c => {
            // block dff8c
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dff8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dff8c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dff8d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dff8f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dff91 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dff93 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dff95 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dff96 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0eb0 size=404
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0eb0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0eb0;
    loop {
        match pc {
        0xe0eb0 => {
            // block e0eb0
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0eb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0eb0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0eb1 mov rbp,rsp
            rbp = rsp;
            // e0eb4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e0eb6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0eb8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e0eba push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e0ebc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0ebd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0ebe mov DWORD PTR [rbp-0x30],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rdx as u32 as u64)) as u32);
            // e0ec1 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e0ec3 mov r14,rdi
            r14 = rdi;
            // e0ec6 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0ec9 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e0ecc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0ece call QWORD PTR [rax+0x3c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3c0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ed4 mov r12,rax
            r12 = rax;
            // e0ed7 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0eda mov rdi,r14
            rdi = r14;
            // e0edd call QWORD PTR [rax+0x1a90]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a90)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ee3 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e0ee6 jne 0xe0f0f
            if fl.ne() { pc = 0xe0f0f; continue; }
            pc = 0xe0ee8;
        }
        0xe0ee8 => {
            // block e0ee8
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0ee8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0ee8 shr r12,0x20
            { let (r, f) = shr::<64>(r12, 0x20u64, fl); r12 = r; fl = f; }
            // e0eec cmp r12d,0x2cf
            fl = sub::<32>((r12 as u32 as u64), 0x2cfu64).1;
            // e0ef3 jg 0xe0f46
            if fl.g() { pc = 0xe0f46; continue; }
            pc = 0xe0ef5;
        }
        0xe0ef5 => {
            // block e0ef5
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0ef5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0ef5 cmp r12d,0x1e6
            fl = sub::<32>((r12 as u32 as u64), 0x1e6u64).1;
            // e0efc jne 0xe0f6c
            if fl.ne() { pc = 0xe0f6c; continue; }
            pc = 0xe0efe;
        }
        0xe0efe => {
            // block e0efe
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0efe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0efe mov r15d,0x8
            r15 = (0x8u64) as u32 as u64;
            // e0f04 mov r13d,0xf4
            r13 = (0xf4u64) as u32 as u64;
            // e0f0a jmp 0xe0f95
            pc = 0xe0f95; continue;
        }
        0xe0f0f => {
            // block e0f0f
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f0f test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e0f11 jne 0xe1034
            if fl.ne() { pc = 0xe1034; continue; }
            pc = 0xe0f17;
        }
        0xe0f17 => {
            // block e0f17
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f17 cmp r12d,0x2d0
            fl = sub::<32>((r12 as u32 as u64), 0x2d0u64).1;
            // e0f1e je 0xe0f89
            if fl.e() { pc = 0xe0f89; continue; }
            pc = 0xe0f20;
        }
        0xe0f20 => {
            // block e0f20
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f20 cmp r12d,0x500
            fl = sub::<32>((r12 as u32 as u64), 0x500u64).1;
            // e0f27 je 0xe0f3b
            if fl.e() { pc = 0xe0f3b; continue; }
            pc = 0xe0f29;
        }
        0xe0f29 => {
            // block e0f29
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f29 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e0f2c cmp r12d,0x780
            fl = sub::<32>((r12 as u32 as u64), 0x780u64).1;
            // e0f33 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // e0f39 jne 0xe0f95
            if fl.ne() { pc = 0xe0f95; continue; }
            pc = 0xe0f3b;
        }
        0xe0f3b => {
            // block e0f3b
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f3b mov r15d,0x8
            r15 = (0x8u64) as u32 as u64;
            // e0f41 mov r13d,r12d
            r13 = ((r12 as u32 as u64)) as u32 as u64;
            // e0f44 jmp 0xe0f95
            pc = 0xe0f95; continue;
        }
        0xe0f46 => {
            // block e0f46
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f46 cmp r12d,0x2d0
            fl = sub::<32>((r12 as u32 as u64), 0x2d0u64).1;
            // e0f4d je 0xe0f5e
            if fl.e() { pc = 0xe0f5e; continue; }
            pc = 0xe0f4f;
        }
        0xe0f4f => {
            // block e0f4f
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f4f cmp r12d,0x438
            fl = sub::<32>((r12 as u32 as u64), 0x438u64).1;
            // e0f56 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // e0f5c jne 0xe0f95
            if fl.ne() { pc = 0xe0f95; continue; }
            pc = 0xe0f5e;
        }
        0xe0f5e => {
            // block e0f5e
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f5e inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // e0f61 mov r15d,0x7
            r15 = (0x7u64) as u32 as u64;
            // e0f67 mov r13d,r12d
            r13 = ((r12 as u32 as u64)) as u32 as u64;
            // e0f6a jmp 0xe0f95
            pc = 0xe0f95; continue;
        }
        0xe0f6c => {
            // block e0f6c
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f6c cmp r12d,0x240
            fl = sub::<32>((r12 as u32 as u64), 0x240u64).1;
            // e0f73 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // e0f79 jne 0xe0f95
            if fl.ne() { pc = 0xe0f95; continue; }
            pc = 0xe0f7b;
        }
        0xe0f7b => {
            // block e0f7b
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f7b mov r15d,0x8
            r15 = (0x8u64) as u32 as u64;
            // e0f81 mov r13d,0x121
            r13 = (0x121u64) as u32 as u64;
            // e0f87 jmp 0xe0f95
            pc = 0xe0f95; continue;
        }
        0xe0f89 => {
            // block e0f89
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f89 mov r15d,0x8
            r15 = (0x8u64) as u32 as u64;
            // e0f8f mov r13d,0x5a0
            r13 = (0x5a0u64) as u32 as u64;
            pc = 0xe0f95;
        }
        0xe0f95 => {
            // block e0f95
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0f95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0f95 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0f98 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // e0f9c mov rdi,r14
            rdi = r14;
            // e0f9f call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0fa5 mov ecx,0xc0000000
            rcx = (0xc0000000u64) as u32 as u64;
            // e0faa and ecx,DWORD PTR [rbp-0x2c]
            { let r = (rcx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0fad mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // e0fb0 imul ebx,r13d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r13 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // e0fb4 add ebx,0x8000
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x8000u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0fba shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0fbd mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e0fc2 mov edi,DWORD PTR [rbp-0x30]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e0fc5 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // e0fc7 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e0fcc je 0xe1007
            if fl.e() { pc = 0xe1007; continue; }
            pc = 0xe0fce;
        }
        0xe0fce => {
            // block e0fce
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0fce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0fce mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // e0fd1 shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0fd3 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // e0fd6 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0fd8 cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // e0fda cmovbe edx,ebx
            if fl.be() { rdx = ((rbx as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // e0fdd shr r13d,0x2
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x2u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // e0fe1 cmp edx,r13d
            fl = sub::<32>((rdx as u32 as u64), (r13 as u32 as u64)).1;
            // e0fe4 cmovbe r13d,edx
            if fl.be() { r13 = ((rdx as u32 as u64)) as u32 as u64; } else { r13 = r13 as u32 as u64; }
            // e0fe8 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // e0feb mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e0ff0 je 0xe1007
            if fl.e() { pc = 0xe1007; continue; }
            pc = 0xe0ff2;
        }
        0xe0ff2 => {
            // block e0ff2
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e0ff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0ff2 dec r13d
            { let (r, f) = dec::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // e0ff5 imul r13d,edi
            { let (r, f) = imul::<32>((r13 as u32 as u64), (rdi as u32 as u64)); r13 = (r) as u32 as u64; fl = f; }
            // e0ff9 add r13d,0x8001
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x8001u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e1000 shr r13d,0x10
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x10u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // e1004 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            pc = 0xe1007;
        }
        0xe1007 => {
            // block e1007
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e1007 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1007 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e1009 cmovne esi,eax
            if fl.ne() { rsi = ((rax as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // e100c mov eax,0x1fff
            rax = (0x1fffu64) as u32 as u64;
            // e1011 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e1013 div esi
            { let (q, r) = div::<32>(rdx, rax, (rsi as u32 as u64)); rax = q; rdx = r; }
            // e1015 shr esi,1
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e1017 mov rdx,QWORD PTR [r14]
            rdx = m.r64(r14) as u64;
            // e101a shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e101d or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e101f add ebx,r15d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r15 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1022 sub ebx,esi
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rsi as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1024 shl ebx,0x2
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x2u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e1027 or ebx,ecx
            { let r = (rbx as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e1029 mov rdi,r14
            rdi = r14;
            // e102c mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e102e call QWORD PTR [rdx+0x1ad0]
            { let r = { let t = m.r64(rdx.wrapping_add(0x1ad0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe1034;
        }
        0xe1034 => {
            // block e1034
            #[cfg(feature = "trace")] eprintln!("f_000e0eb0 pc e1034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1034 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e1038 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e1039 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e103b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e103d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e103f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e1041 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1042 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1044 size=418
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e1044<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe1044;
    loop {
        match pc {
        0xe1044 => {
            // block e1044
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1044 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1044 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1045 mov rbp,rsp
            rbp = rsp;
            // e1048 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e104a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e104c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e104e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e1050 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1051 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e1052 mov DWORD PTR [rbp-0x30],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rdx as u32 as u64)) as u32);
            // e1055 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e1058 mov r12,rdi
            r12 = rdi;
            // e105b mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e105f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e1061 call QWORD PTR [rax+0x3c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3c0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1067 mov r14,rax
            r14 = rax;
            // e106a test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // e106d je 0xe11d6
            if fl.e() { pc = 0xe11d6; continue; }
            pc = 0xe1073;
        }
        0xe1073 => {
            // block e1073
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1073 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1073 mov rbx,r14
            rbx = r14;
            // e1076 shr rbx,0x20
            { let (r, f) = shr::<64>(rbx, 0x20u64, fl); rbx = r; fl = f; }
            // e107a test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // e107c je 0xe11d6
            if fl.e() { pc = 0xe11d6; continue; }
            pc = 0xe1082;
        }
        0xe1082 => {
            // block e1082
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1082 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1082 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e1086 mov rdi,r12
            rdi = r12;
            // e1089 call QWORD PTR [rax+0x1a90]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a90)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e108f cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e1092 jne 0xe10b4
            if fl.ne() { pc = 0xe10b4; continue; }
            pc = 0xe1094;
        }
        0xe1094 => {
            // block e1094
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1094 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1094 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e1097 cmp ebx,0x2cf
            fl = sub::<32>((rbx as u32 as u64), 0x2cfu64).1;
            // e109d jg 0xe10ea
            if fl.g() { pc = 0xe10ea; continue; }
            pc = 0xe109f;
        }
        0xe109f => {
            // block e109f
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e109f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e109f cmp ebx,0x1e6
            fl = sub::<32>((rbx as u32 as u64), 0x1e6u64).1;
            // e10a5 jne 0xe110b
            if fl.ne() { pc = 0xe110b; continue; }
            pc = 0xe10a7;
        }
        0xe10a7 => {
            // block e10a7
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10a7 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e10ac mov r13d,0xf4
            r13 = (0xf4u64) as u32 as u64;
            // e10b2 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe10b4 => {
            // block e10b4
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10b4 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // e10b7 jne 0xe11d6
            if fl.ne() { pc = 0xe11d6; continue; }
            pc = 0xe10bd;
        }
        0xe10bd => {
            // block e10bd
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10bd cmp r14d,0x2d0
            fl = sub::<32>((r14 as u32 as u64), 0x2d0u64).1;
            // e10c4 je 0xe1125
            if fl.e() { pc = 0xe1125; continue; }
            pc = 0xe10c6;
        }
        0xe10c6 => {
            // block e10c6
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10c6 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // e10cd je 0xe10e0
            if fl.e() { pc = 0xe10e0; continue; }
            pc = 0xe10cf;
        }
        0xe10cf => {
            // block e10cf
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10cf xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e10d2 cmp r14d,0x780
            fl = sub::<32>((r14 as u32 as u64), 0x780u64).1;
            // e10d9 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e10de jne 0xe1130
            if fl.ne() { pc = 0xe1130; continue; }
            pc = 0xe10e0;
        }
        0xe10e0 => {
            // block e10e0
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10e0 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e10e5 mov r13d,r14d
            r13 = ((r14 as u32 as u64)) as u32 as u64;
            // e10e8 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe10ea => {
            // block e10ea
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10ea cmp ebx,0x2d0
            fl = sub::<32>((rbx as u32 as u64), 0x2d0u64).1;
            // e10f0 je 0xe10ff
            if fl.e() { pc = 0xe10ff; continue; }
            pc = 0xe10f2;
        }
        0xe10f2 => {
            // block e10f2
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10f2 cmp ebx,0x438
            fl = sub::<32>((rbx as u32 as u64), 0x438u64).1;
            // e10f8 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e10fd jne 0xe1130
            if fl.ne() { pc = 0xe1130; continue; }
            pc = 0xe10ff;
        }
        0xe10ff => {
            // block e10ff
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10ff inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // e1101 mov eax,0x7
            rax = (0x7u64) as u32 as u64;
            // e1106 mov r13d,ebx
            r13 = ((rbx as u32 as u64)) as u32 as u64;
            // e1109 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe110b => {
            // block e110b
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e110b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e110b cmp ebx,0x240
            fl = sub::<32>((rbx as u32 as u64), 0x240u64).1;
            // e1111 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e1116 jne 0xe1130
            if fl.ne() { pc = 0xe1130; continue; }
            pc = 0xe1118;
        }
        0xe1118 => {
            // block e1118
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1118 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1118 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e111d mov r13d,0x121
            r13 = (0x121u64) as u32 as u64;
            // e1123 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe1125 => {
            // block e1125
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1125 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1125 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e112a mov r13d,0x5a0
            r13 = (0x5a0u64) as u32 as u64;
            pc = 0xe1130;
        }
        0xe1130 => {
            // block e1130
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1130 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1130 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e1132 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e1136 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // e113a mov rdi,r12
            rdi = r12;
            // e113d call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1143 mov ecx,0xc0000000
            rcx = (0xc0000000u64) as u32 as u64;
            // e1148 and ecx,DWORD PTR [rbp-0x2c]
            { let r = (rcx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e114b mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // e114e imul r15d,r13d
            { let (r, f) = imul::<32>((r15 as u32 as u64), (r13 as u32 as u64)); r15 = (r) as u32 as u64; fl = f; }
            // e1152 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // e1155 js 0xe1167
            if fl.s() { pc = 0xe1167; continue; }
            pc = 0xe1157;
        }
        0xe1157 => {
            // block e1157
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1157 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1157 add r15d,0x8000
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x8000u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // e115e sar r15d,0x10
            { let (r, f) = sar::<32>((r15 as u32 as u64), 0x10u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // e1162 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e1165 jmp 0xe1174
            pc = 0xe1174; continue;
        }
        0xe1167 => {
            // block e1167
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1167 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1167 mov esi,0x8000
            rsi = (0x8000u64) as u32 as u64;
            // e116c sub esi,r15d
            { let (r, f) = sub::<32>((rsi as u32 as u64), (r15 as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e116f sar esi,0x10
            { let (r, f) = sar::<32>((rsi as u32 as u64), 0x10u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e1172 neg esi
            { let (r, f) = neg::<32>((rsi as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            pc = 0xe1174;
        }
        0xe1174 => {
            // block e1174
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1174 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1174 mov edi,DWORD PTR [rbp-0x30]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e1177 dec r13d
            { let (r, f) = dec::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // e117a imul r13d,edi
            { let (r, f) = imul::<32>((r13 as u32 as u64), (rdi as u32 as u64)); r13 = (r) as u32 as u64; fl = f; }
            // e117e cmp r13d,0xfffffffe
            fl = sub::<32>((r13 as u32 as u64), 0xfffffffeu64).1;
            // e1182 jg 0xe1193
            if fl.g() { pc = 0xe1193; continue; }
            pc = 0xe1184;
        }
        0xe1184 => {
            // block e1184
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1184 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1184 mov edx,0x7fff
            rdx = (0x7fffu64) as u32 as u64;
            // e1189 sub edx,r13d
            { let (r, f) = sub::<32>((rdx as u32 as u64), (r13 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e118c sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e118f neg edx
            { let (r, f) = neg::<32>((rdx as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // e1191 jmp 0xe11a1
            pc = 0xe11a1; continue;
        }
        0xe1193 => {
            // block e1193
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1193 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1193 add r13d,0x8001
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x8001u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e119a sar r13d,0x10
            { let (r, f) = sar::<32>((r13 as u32 as u64), 0x10u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // e119e mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            pc = 0xe11a1;
        }
        0xe11a1 => {
            // block e11a1
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11a1 mov eax,0x1fff0000
            rax = (0x1fff0000u64) as u32 as u64;
            // e11a6 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // e11a8 je 0xe11c0
            if fl.e() { pc = 0xe11c0; continue; }
            pc = 0xe11aa;
        }
        0xe11aa => {
            // block e11aa
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11aa test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e11ac mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // e11b1 cmovne edi,edx
            if fl.ne() { rdi = ((rdx as u32 as u64)) as u32 as u64; } else { rdi = rdi as u32 as u64; }
            // e11b4 mov eax,0x1fff
            rax = (0x1fffu64) as u32 as u64;
            // e11b9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e11bb div edi
            { let (q, r) = div::<32>(rdx, rax, (rdi as u32 as u64)); rax = q; rdx = r; }
            // e11bd shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0xe11c0;
        }
        0xe11c0 => {
            // block e11c0
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11c0 mov rdx,QWORD PTR [r12]
            rdx = m.r64(r12) as u64;
            // e11c4 add esi,ebx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e11c6 shl esi,0x2
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x2u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e11c9 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e11cb or esi,ecx
            { let r = (rsi as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e11cd mov rdi,r12
            rdi = r12;
            // e11d0 call QWORD PTR [rdx+0x1ad0]
            { let r = { let t = m.r64(rdx.wrapping_add(0x1ad0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe11d6;
        }
        0xe11d6 => {
            // block e11d6
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11d6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e11da pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e11db pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e11dd pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e11df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e11e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e11e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e11e4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe8100 size=404
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000e8100<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe8100;
    loop {
        match pc {
        0xe8100 => {
            // block e8100
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8100 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8100 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e8101 mov rbp,rsp
            rbp = rsp;
            // e8104 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e8106 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e8108 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e810a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e810c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e810d sub rsp,0x498
            { let (r, f) = sub::<64>(rsp, 0x498u64);
              rsp = r;
              fl = f; }
            // e8114 mov r14,rsi
            r14 = rsi;
            // e8117 mov rbx,rdi
            rbx = rdi;
            // e811a mov rax,QWORD PTR [rip+0x582b7] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // e8121 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // e8124 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // e8128 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e812b call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e812e lea r15,[rbx+0x290]
            r15 = rbx.wrapping_add(0x290);
            // e8135 test BYTE PTR [rbx+0x290],0x1
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x290)) as u64 & 0x1u64);
            // e813c jne 0xe8146
            if fl.ne() { pc = 0xe8146; continue; }
            pc = 0xe813e;
        }
        0xe813e => {
            // block e813e
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e813e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e813e mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // e8144 jmp 0xe815b
            pc = 0xe815b; continue;
        }
        0xe8146 => {
            // block e8146
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8146 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8146 mov rax,QWORD PTR [rbx+0x2a0]
            rax = m.r64(rbx.wrapping_add(0x2a0)) as u64;
            // e814d mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // e8150 mov QWORD PTR [rbx+0x298],0x0
            m.w64(rbx.wrapping_add(0x298), (0x0u64) as u64);
            pc = 0xe815b;
        }
        0xe815b => {
            // block e815b
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e815b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e815b test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // e815f jne 0xe8167
            if fl.ne() { pc = 0xe8167; continue; }
            pc = 0xe8161;
        }
        0xe8161 => {
            // block e8161
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8161 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8161 lea rdi,[r14+0x1]
            rdi = r14.wrapping_add(0x1);
            // e8165 jmp 0xe816b
            pc = 0xe816b; continue;
        }
        0xe8167 => {
            // block e8167
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8167 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8167 mov rdi,QWORD PTR [r14+0x10]
            rdi = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xe816b;
        }
        0xe816b => {
            // block e816b
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e816b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e816b lea rsi,[rbp-0x4b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // e8172 call 0xec2a2
            { let r = { let t = m.env.stub_va(0xa6f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8177 mov eax,DWORD PTR [rbp-0x450]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64) as u32 as u64;
            // e817d mov DWORD PTR [rbx+0x260],eax
            m.w32(rbx.wrapping_add(0x260), ((rax as u32 as u64)) as u32);
            // e8183 lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // e818a call 0xec1fa
            { let r = { let t = m.env.stub_va(0x9c7); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e818f mov r12,rax
            r12 = rax;
            // e8192 lea r13,[rbp-0x4b8]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // e8199 mov rdi,r13
            rdi = r13;
            // e819c call 0xec2ea
            { let r = { let t = m.env.stub_va(0xab7); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81a1 mov rdi,r13
            rdi = r13;
            // e81a4 call 0xec1fa
            { let r = { let t = m.env.stub_va(0x9c7); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81a9 mov rdi,rax
            rdi = rax;
            // e81ac call 0xec1a0
            { let r = { let t = m.env.stub_va(0x96d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81b1 mov r13,rax
            r13 = rax;
            // e81b4 mov rdi,r12
            rdi = r12;
            // e81b7 call 0xec1a0
            { let r = { let t = m.env.stub_va(0x96d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81bc mov rcx,rax
            rcx = rax;
            // e81bf lea rsi,[rip+0x55048] # 0x13d20e
            rsi = m.env.data_va(334299);
            // e81c6 lea r12,[rbp-0x420]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fbe0);
            // e81cd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e81cf mov rdi,r12
            rdi = r12;
            // e81d2 mov rdx,r13
            rdx = r13;
            // e81d5 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81da mov rdi,r15
            rdi = r15;
            // e81dd mov rsi,r12
            rsi = r12;
            // e81e0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81e5 lea r15,[rbx+0x8]
            r15 = rbx.wrapping_add(0x8);
            // e81e9 test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // e81ed jne 0xe81f4
            if fl.ne() { pc = 0xe81f4; continue; }
            pc = 0xe81ef;
        }
        0xe81ef => {
            // block e81ef
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e81ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e81ef inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // e81f2 jmp 0xe81f8
            pc = 0xe81f8; continue;
        }
        0xe81f4 => {
            // block e81f4
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e81f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e81f4 mov r14,QWORD PTR [r14+0x10]
            r14 = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xe81f8;
        }
        0xe81f8 => {
            // block e81f8
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e81f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e81f8 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // e81fd mov rdi,r15
            rdi = r15;
            // e8200 mov rsi,r14
            rsi = r14;
            // e8203 call 0xeb918
            { let r = { let t = m.env.stub_va(0xe5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8208 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e820b mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e820f test BYTE PTR [rax+r15*1+0x20],0x5
            fl = logic::<8>(m.r8(rax.wrapping_add(r15).wrapping_add(0x20)) as u64 & 0x5u64);
            // e8215 je 0xe821c
            if fl.e() { pc = 0xe821c; continue; }
            pc = 0xe8217;
        }
        0xe8217 => {
            // block e8217
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8217 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8217 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e821a jmp 0xe8269
            pc = 0xe8269; continue;
        }
        0xe821c => {
            // block e821c
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e821c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e821c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e821f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e8221 mov rdi,rbx
            rdi = rbx;
            // e8224 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8227 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e822a mov rdi,rbx
            rdi = rbx;
            // e822d call QWORD PTR [rax+0xa0]
            { let r = { let t = m.r64(rax.wrapping_add(0xa0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8233 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // e8236 cmp QWORD PTR [rbx+0x90],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x90)) as u64, 0x0u64).1;
            // e823e je 0xe8269
            if fl.e() { pc = 0xe8269; continue; }
            pc = 0xe8240;
        }
        0xe8240 => {
            // block e8240
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8240 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8240 add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // e8244 mov rdi,rbx
            rdi = rbx;
            // e8247 call 0xebf12
            { let r = { let t = m.env.stub_va(0x6df); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e824c test rax,rax
            fl = logic::<64>(rax & rax);
            // e824f jne 0xe8269
            if fl.ne() { pc = 0xe8269; continue; }
            pc = 0xe8251;
        }
        0xe8251 => {
            // block e8251
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8251 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8251 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e8254 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e8258 lea rdi,[r15+rax*1]
            rdi = r15.wrapping_add(rax);
            // e825c mov esi,DWORD PTR [r15+rax*1+0x20]
            rsi = (m.r32(r15.wrapping_add(rax).wrapping_add(0x20)) as u64) as u32 as u64;
            // e8261 or esi,0x4
            { let r = (rsi as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e8264 call 0xec0fe
            { let r = { let t = m.env.stub_va(0x8cb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe8269;
        }
        0xe8269 => {
            // block e8269
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8269 mov rax,QWORD PTR [rip+0x58168] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // e8270 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // e8273 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // e8277 jne 0xe828e
            if fl.ne() { pc = 0xe828e; continue; }
            pc = 0xe8279;
        }
        0xe8279 => {
            // block e8279
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8279 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8279 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // e827c add rsp,0x498
            { let (r, f) = add::<64>(rsp, 0x498u64);
              rsp = r;
              fl = f; }
            // e8283 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e8284 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e8286 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e8288 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e828a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e828c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e828d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe828e => {
            // block e828e
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e828e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e828e call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9208 size=388
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9208<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe9208;
    loop {
        match pc {
        0xe9208 => {
            // block e9208
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9208 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9208 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9209 mov rbp,rsp
            rbp = rsp;
            // e920c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e920e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9210 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e9212 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e9214 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9215 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e9219 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e921c mov r14,rdi
            r14 = rdi;
            // e921f xorps xmm0,xmm0
            x0 = 0;
            // e9222 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // e9226 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // e922e xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e9230 cmp QWORD PTR [r14+0x90],0x0
            fl = sub::<64>(m.r64(r14.wrapping_add(0x90)) as u64, 0x0u64).1;
            // e9238 je 0xe9353
            if fl.e() { pc = 0xe9353; continue; }
            pc = 0xe923e;
        }
        0xe923e => {
            // block e923e
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e923e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e923e lea r13,[r14+0x8]
            r13 = r14.wrapping_add(0x8);
            // e9242 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e9244 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e9246 mov rdi,r13
            rdi = r13;
            // e9249 call 0xec044
            { let r = { let t = m.env.stub_va(0x811); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e924e lea rdi,[r14+0x268]
            rdi = r14.wrapping_add(0x268);
            // e9255 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // e9258 je 0xe9269
            if fl.e() { pc = 0xe9269; continue; }
            pc = 0xe925a;
        }
        0xe925a => {
            // block e925a
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e925a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e925a lea esi,[r15+0x1]
            rsi = (r15.wrapping_add(0x1)) as u32 as u64;
            // e925e mov QWORD PTR [rbp-0x60],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rdi) as u64);
            // e9262 call 0xebf18
            { let r = { let t = m.env.stub_va(0x6e5); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9267 jmp 0xe927d
            pc = 0xe927d; continue;
        }
        0xe9269 => {
            // block e9269
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9269 mov r15d,0x1e8480
            r15 = (0x1e8480u64) as u32 as u64;
            // e926f mov esi,0x1e8480
            rsi = (0x1e8480u64) as u32 as u64;
            // e9274 mov QWORD PTR [rbp-0x60],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rdi) as u64);
            // e9278 call 0xebf18
            { let r = { let t = m.env.stub_va(0x6e5); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe927d;
        }
        0xe927d => {
            // block e927d
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e927d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e927d xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // e927f call 0xec0ec
            { let r = { let t = m.env.stub_va(0x8b9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9284 mov QWORD PTR [rbp-0x68],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r14) as u64);
            // e9288 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // e928b mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e928f mov QWORD PTR [rbp-0x70],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r15) as u64);
            // e9293 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e9296 lea rbx,[rbp-0x50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e929a xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // e929d jmp 0xe92a6
            pc = 0xe92a6; continue;
        }
        0xe929f => {
            // block e929f
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e929f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e929f inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // e92a2 add r14,0x18
            { let (r, f) = add::<64>(r14, 0x18u64);
              r14 = r;
              fl = f; }
            pc = 0xe92a6;
        }
        0xe92a6 => {
            // block e92a6
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e92a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e92a6 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // e92aa mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e92ae add rsi,r13
            { let (r, f) = add::<64>(rsi, r13);
              rsi = r;
              fl = f; }
            // e92b1 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92b5 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92ba lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92be mov rsi,QWORD PTR [rip+0x56ecb] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // e92c5 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92ca mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e92cd mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // e92d1 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // e92d6 mov rdi,rax
            rdi = rax;
            // e92d9 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92db mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e92de lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92e2 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92e7 movsx edx,r12b
            rdx = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // e92eb mov rdi,r13
            rdi = r13;
            // e92ee mov rsi,rbx
            rsi = rbx;
            // e92f1 call 0xebf00
            { let r = { let t = m.env.stub_va(0x6cd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92f6 cmp r15,QWORD PTR [rbp-0x58]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64).1;
            // e92fa jae 0xe9328
            if fl.ae() { pc = 0xe9328; continue; }
            pc = 0xe92fc;
        }
        0xe92fc => {
            // block e92fc
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e92fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e92fc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e92ff mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e9303 mov eax,DWORD PTR [rcx+rax*1+0x20]
            rax = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x20)) as u64) as u32 as u64;
            // e9307 and eax,0x5
            { let r = (rax as u32 as u64) & 0x5u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e930a jne 0xe9328
            if fl.ne() { pc = 0xe9328; continue; }
            pc = 0xe930c;
        }
        0xe930c => {
            // block e930c
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e930c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e930c mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e9310 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // e9313 add rdi,r14
            { let (r, f) = add::<64>(rdi, r14);
              rdi = r;
              fl = f; }
            // e9316 mov rsi,rbx
            rsi = rbx;
            // e9319 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e931e jmp 0xe929f
            pc = 0xe929f; continue;
        }
        0xe9323 => {
            // block e9323
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9323 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9323 mov rbx,rax
            rbx = rax;
            // e9326 jmp 0xe9379
            pc = 0xe9379; continue;
        }
        0xe9328 => {
            // block e9328
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9328 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // e932c cmp eax,0x1e847f
            fl = sub::<32>((rax as u32 as u64), 0x1e847fu64).1;
            // e9331 mov rbx,QWORD PTR [rbp-0x68]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e9335 ja 0xe934d
            if fl.a() { pc = 0xe934d; continue; }
            pc = 0xe9337;
        }
        0xe9337 => {
            // block e9337
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9337 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9337 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e933b add r14,QWORD PTR [rax]
            { let (r, f) = add::<64>(r14, m.r64(rax) as u64);
              r14 = r;
              fl = f; }
            // e933e lea rsi,[rip+0x4fb72] # 0x138eb7
            rsi = m.env.data_va(317060);
            // e9345 mov rdi,r14
            rdi = r14;
            // e9348 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe934d;
        }
        0xe934d => {
            // block e934d
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e934d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e934d mov ebx,DWORD PTR [rbx+0x260]
            rbx = (m.r32(rbx.wrapping_add(0x260)) as u64) as u32 as u64;
            pc = 0xe9353;
        }
        0xe9353 => {
            // block e9353
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9353 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9353 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e9357 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e935c mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e935e add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e9362 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9363 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e9365 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e9367 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9369 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e936b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e936c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe936d => {
            // block e936d
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e936d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e936d mov rbx,rax
            rbx = rax;
            // e9370 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e9374 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe9379;
        }
        0xe9379 => {
            // block e9379
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9379 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e937d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9382 mov rdi,rbx
            rdi = rbx;
            // e9385 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e938a jmp 0xe9323
            pc = 0xe9323; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea9ce size=462
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ea9ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xea9ce;
    loop {
        match pc {
        0xea9ce => {
            // block ea9ce
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc ea9ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea9ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea9cf mov rbp,rsp
            rbp = rsp;
            // ea9d2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ea9d4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea9d6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ea9d8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ea9da push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea9db sub rsp,0x2d8
            { let (r, f) = sub::<64>(rsp, 0x2d8u64);
              rsp = r;
              fl = f; }
            // ea9e2 mov r14,rdi
            r14 = rdi;
            // ea9e5 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ea9e9 call 0x63dd3
            { let r = { let t = m.env.code_va(0x63dd3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea9ee lea r15,[rbp-0x50]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // ea9f2 mov QWORD PTR [rbp-0x48],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u64);
            // ea9fa mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // eaa02 mov QWORD PTR [rbp-0x58],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r15) as u64);
            // eaa06 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // eaa0a lea r13,[rbp-0x38]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // eaa0e cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // eaa11 je 0xeab17
            if fl.e() { pc = 0xeab17; continue; }
            pc = 0xeaa17;
        }
        0xeaa17 => {
            // block eaa17
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa17 lea r12,[rbp-0x58]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0xeaa1b;
        }
        0xeaa1b => {
            // block eaa1b
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa1b mov edi,DWORD PTR [rbx+0x1c]
            rdi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // eaa1e call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaa23 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // eaa26 je 0xeaa34
            if fl.e() { pc = 0xeaa34; continue; }
            pc = 0xeaa28;
        }
        0xeaa28 => {
            // block eaa28
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa28 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // eaa2c mov rdi,r12
            rdi = r12;
            // eaa2f call 0xebbac
            { let r = { let t = m.env.stub_va(0x379); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeaa34;
        }
        0xeaa34 => {
            // block eaa34
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa34 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // eaa38 test rax,rax
            fl = logic::<64>(rax & rax);
            // eaa3b je 0xeaa4a
            if fl.e() { pc = 0xeaa4a; continue; }
            pc = 0xeaa3d;
        }
        0xeaa3d => {
            // block eaa3d
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa3d mov rbx,rax
            rbx = rax;
            // eaa40 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // eaa43 test rax,rax
            fl = logic::<64>(rax & rax);
            // eaa46 jne 0xeaa3d
            if fl.ne() { pc = 0xeaa3d; continue; }
            pc = 0xeaa48;
        }
        0xeaa48 => {
            // block eaa48
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa48 jmp 0xeaa59
            pc = 0xeaa59; continue;
        }
        0xeaa4a => {
            // block eaa4a
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa4a mov rax,rbx
            rax = rbx;
            pc = 0xeaa4d;
        }
        0xeaa4d => {
            // block eaa4d
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa4d mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // eaa51 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // eaa54 mov rax,rbx
            rax = rbx;
            // eaa57 jne 0xeaa4d
            if fl.ne() { pc = 0xeaa4d; continue; }
            pc = 0xeaa59;
        }
        0xeaa59 => {
            // block eaa59
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa59 cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // eaa5c jne 0xeaa1b
            if fl.ne() { pc = 0xeaa1b; continue; }
            pc = 0xeaa5e;
        }
        0xeaa5e => {
            // block eaa5e
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa5e mov rbx,QWORD PTR [rbp-0x58]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // eaa62 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // eaa65 je 0xeab17
            if fl.e() { pc = 0xeab17; continue; }
            pc = 0xeaa6b;
        }
        0xeaa6b => {
            // block eaa6b
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa6b lea r12,[rbp-0x2e0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            pc = 0xeaa72;
        }
        0xeaa72 => {
            // block eaa72
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa72 mov rdi,r12
            rdi = r12;
            // eaa75 call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaa7a mov edi,DWORD PTR [rbx+0x1c]
            rdi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // eaa7d mov rsi,r12
            rsi = r12;
            // eaa80 call 0x1729c
            { let r = { let t = m.env.code_va(0x1729c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaa85 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaa87 jne 0xeaabd
            if fl.ne() { pc = 0xeaabd; continue; }
            pc = 0xeaa89;
        }
        0xeaa89 => {
            // block eaa89
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa89 mov rdi,r12
            rdi = r12;
            // eaa8c call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaa91 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // eaa95 test rax,rax
            fl = logic::<64>(rax & rax);
            // eaa98 je 0xeaaa7
            if fl.e() { pc = 0xeaaa7; continue; }
            pc = 0xeaa9a;
        }
        0xeaa9a => {
            // block eaa9a
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaa9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaa9a mov rbx,rax
            rbx = rax;
            // eaa9d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // eaaa0 test rax,rax
            fl = logic::<64>(rax & rax);
            // eaaa3 jne 0xeaa9a
            if fl.ne() { pc = 0xeaa9a; continue; }
            pc = 0xeaaa5;
        }
        0xeaaa5 => {
            // block eaaa5
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaaa5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaa5 jmp 0xeaab6
            pc = 0xeaab6; continue;
        }
        0xeaaa7 => {
            // block eaaa7
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaaa7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaa7 mov rax,rbx
            rax = rbx;
            pc = 0xeaaaa;
        }
        0xeaaaa => {
            // block eaaaa
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaaaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaaa mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // eaaae cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // eaab1 mov rax,rbx
            rax = rbx;
            // eaab4 jne 0xeaaaa
            if fl.ne() { pc = 0xeaaaa; continue; }
            pc = 0xeaab6;
        }
        0xeaab6 => {
            // block eaab6
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaab6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaab6 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // eaab9 jne 0xeaa72
            if fl.ne() { pc = 0xeaa72; continue; }
            pc = 0xeaabb;
        }
        0xeaabb => {
            // block eaabb
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaabb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaabb jmp 0xeab17
            pc = 0xeab17; continue;
        }
        0xeaabd => {
            // block eaabd
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaabd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaabd movzx eax,WORD PTR [rbp-0x2d8]
            rax = (m.r16(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64) as u32 as u64;
            // eaac4 mov DWORD PTR [r14+0x5c],eax
            m.w32(r14.wrapping_add(0x5c), ((rax as u32 as u64)) as u32);
            // eaac8 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // eaacb mov r15,QWORD PTR [rax+0x60]
            r15 = m.r64(rax.wrapping_add(0x60)) as u64;
            // eaacf mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // eaad2 lea rdi,[rbp-0x2f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // eaad9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eaadb call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaae0 test BYTE PTR [rbp-0x2f8],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd08)) as u64 & 0x1u64);
            // eaae7 jne 0xeaaf2
            if fl.ne() { pc = 0xeaaf2; continue; }
            pc = 0xeaae9;
        }
        0xeaae9 => {
            // block eaae9
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaae9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaae9 lea rsi,[rbp-0x2f7]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd09);
            // eaaf0 jmp 0xeaaf9
            pc = 0xeaaf9; continue;
        }
        0xeaaf2 => {
            // block eaaf2
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaaf2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaf2 mov rsi,QWORD PTR [rbp-0x2e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64;
            pc = 0xeaaf9;
        }
        0xeaaf9 => {
            // block eaaf9
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eaaf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaaf9 mov rdi,r14
            rdi = r14;
            // eaafc call r15
            { let r = { let t = r15; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaaff lea rdi,[rbp-0x2f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // eab06 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eab0b lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // eab12 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeab17;
        }
        0xeab17 => {
            // block eab17
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab17 mov rdi,r14
            rdi = r14;
            // eab1a call 0xe09b6
            { let r = { let t = m.env.code_va(0xe09b6); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eab1f mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // eab21 mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // eab25 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // eab29 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eab2e mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // eab32 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // eab36 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eab3b mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // eab3d add rsp,0x2d8
            { let (r, f) = add::<64>(rsp, 0x2d8u64);
              rsp = r;
              fl = f; }
            // eab44 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eab45 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eab47 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eab49 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eab4b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eab4d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eab4e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeab4f => {
            // block eab4f
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab4f mov rbx,rax
            rbx = rax;
            pc = 0xeab52;
        }
        0xeab52 => {
            // block eab52
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab52 mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // eab56 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // eab5a call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eab5f mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // eab63 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // eab67 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eab6c mov rdi,rbx
            rdi = rbx;
            // eab6f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eab74 jmp 0xeab4f
            pc = 0xeab4f; continue;
        }
        0xeab76 => {
            // block eab76
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab76 mov rbx,rax
            rbx = rax;
            pc = 0xeab79;
        }
        0xeab79 => {
            // block eab79
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab79 lea rdi,[rbp-0x2e0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd20);
            // eab80 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eab85 jmp 0xeab52
            pc = 0xeab52; continue;
        }
        0xeab87 => {
            // block eab87
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab87 jmp 0xeab4f
            pc = 0xeab4f; continue;
        }
        0xeab89 => {
            // block eab89
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab89 jmp 0xeab76
            pc = 0xeab76; continue;
        }
        0xeab8b => {
            // block eab8b
            #[cfg(feature = "trace")] eprintln!("f_000ea9ce pc eab8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab8b mov rbx,rax
            rbx = rax;
            // eab8e lea rdi,[rbp-0x2f8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd08);
            // eab95 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eab9a jmp 0xeab79
            pc = 0xeab79; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
