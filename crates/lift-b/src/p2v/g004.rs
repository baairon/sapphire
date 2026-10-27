//! Mechanically lifted functions (scripts/lift_run.py), module g004. Each one is checked against the original
//! in the world harness (tests/world_g004.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x1afb0 size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001afb0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1afb0;
    loop {
        match pc {
        0x1afb0 => {
            // block 1afb0
            // 1afb0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1afb1 mov rbp,rsp
            rbp = rsp;
            // 1afb4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1afb6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1afb8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1afba push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1afbc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1afbd sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1afc1 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1afc5 mov r12,rdi
            r12 = rdi;
            // 1afc8 mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1afcc mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1afd1 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1afd4 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1afd8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1afdb mov rax,rbx
            rax = rbx;
            // 1afde shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1afe2 je 0x1aff0
            if fl.e() { pc = 0x1aff0; continue; }
            pc = 0x1afe4;
        }
        0x1afe4 => {
            // block 1afe4
            // 1afe4 mov rdi,r12
            rdi = r12;
            // 1afe7 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x1aff0 => {
            // block 1aff0
            // 1aff0 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1aff5 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1affa mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1affe sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1b001 mov rcx,r15
            rcx = r15;
            // 1b004 sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1b008 movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1b012 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1b015 ja 0x1b023
            if fl.a() { pc = 0x1b023; continue; }
            pc = 0x1b017;
        }
        0x1b017 => {
            // block 1b017
            // 1b017 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1b01a cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1b01d cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1b021 jmp 0x1b02d
            pc = 0x1b02d; continue;
        }
        0x1b023 => {
            // block 1b023
            // 1b023 movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1b02d;
        }
        0x1b02d => {
            // block 1b02d
            // 1b02d mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1b032 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1b036 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1b039 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1b03d mov r13,r14
            r13 = r14;
            // 1b040 sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1b044 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1b04c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1b050 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1b052 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1b055 je 0x1b067
            if fl.e() { pc = 0x1b067; continue; }
            pc = 0x1b057;
        }
        0x1b057 => {
            // block 1b057
            // 1b057 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1b05f call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b064 mov rbx,rax
            rbx = rax;
            pc = 0x1b067;
        }
        0x1b067 => {
            // block 1b067
            // 1b067 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1b06b mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1b06f mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1b073 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1b075 mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1b079 lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1b07e mov rdi,rbx
            rdi = rbx;
            // 1b081 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1b085 mov rsi,r15
            rsi = r15;
            // 1b088 mov rdx,r14
            rdx = r14;
            // 1b08b call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b090 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1b094 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1b098 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1b09d mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1b0a1 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1b0a5 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1b0a9 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1b0ae mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1b0b2 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1b0b6 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1b0ba lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1b0be call 0xeb978
            { let r = { let t = m.env.stub_va(0x145); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b0c3 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1b0c7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1b0c8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1b0ca pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1b0cc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1b0ce pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1b0d0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1b0d1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1b1f4 size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001b1f4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1b1f4;
    loop {
        match pc {
        0x1b1f4 => {
            // block 1b1f4
            // 1b1f4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1b1f5 mov rbp,rsp
            rbp = rsp;
            // 1b1f8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1b1fa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1b1fc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1b1fe push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1b200 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1b201 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1b205 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1b209 mov r12,rdi
            r12 = rdi;
            // 1b20c mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1b210 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1b215 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1b218 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1b21c inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1b21f mov rax,rbx
            rax = rbx;
            // 1b222 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1b226 je 0x1b234
            if fl.e() { pc = 0x1b234; continue; }
            pc = 0x1b228;
        }
        0x1b228 => {
            // block 1b228
            // 1b228 mov rdi,r12
            rdi = r12;
            // 1b22b call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x1b234 => {
            // block 1b234
            // 1b234 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1b239 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1b23e mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1b242 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1b245 mov rcx,r15
            rcx = r15;
            // 1b248 sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1b24c movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1b256 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1b259 ja 0x1b267
            if fl.a() { pc = 0x1b267; continue; }
            pc = 0x1b25b;
        }
        0x1b25b => {
            // block 1b25b
            // 1b25b sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1b25e cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1b261 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1b265 jmp 0x1b271
            pc = 0x1b271; continue;
        }
        0x1b267 => {
            // block 1b267
            // 1b267 movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1b271;
        }
        0x1b271 => {
            // block 1b271
            // 1b271 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1b276 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1b27a sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1b27d mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1b281 mov r13,r14
            r13 = r14;
            // 1b284 sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1b288 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1b290 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1b294 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1b296 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1b299 je 0x1b2ab
            if fl.e() { pc = 0x1b2ab; continue; }
            pc = 0x1b29b;
        }
        0x1b29b => {
            // block 1b29b
            // 1b29b lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1b2a3 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b2a8 mov rbx,rax
            rbx = rax;
            pc = 0x1b2ab;
        }
        0x1b2ab => {
            // block 1b2ab
            // 1b2ab lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1b2af mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1b2b3 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1b2b7 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1b2b9 mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1b2bd lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1b2c2 mov rdi,rbx
            rdi = rbx;
            // 1b2c5 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1b2c9 mov rsi,r15
            rsi = r15;
            // 1b2cc mov rdx,r14
            rdx = r14;
            // 1b2cf call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b2d4 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1b2d8 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1b2dc mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1b2e1 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1b2e5 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1b2e9 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1b2ed mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1b2f2 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1b2f6 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1b2fa mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1b2fe lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1b302 call 0xeb972
            { let r = { let t = m.env.stub_va(0x13f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b307 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1b30b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1b30c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1b30e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1b310 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1b312 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1b314 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1b315 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41c1e size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00041c1e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x41c1e;
    loop {
        match pc {
        0x41c1e => {
            // block 41c1e
            // 41c1e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41c1f mov rbp,rsp
            rbp = rsp;
            // 41c22 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 41c24 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 41c26 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 41c28 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 41c2a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 41c2b sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 41c2f mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 41c33 mov r12,rdi
            r12 = rdi;
            // 41c36 mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 41c3a mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 41c3f sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 41c42 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 41c46 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 41c49 mov rax,rbx
            rax = rbx;
            // 41c4c shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 41c50 je 0x41c5e
            if fl.e() { pc = 0x41c5e; continue; }
            pc = 0x41c52;
        }
        0x41c52 => {
            // block 41c52
            // 41c52 mov rdi,r12
            rdi = r12;
            // 41c55 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x41c5e => {
            // block 41c5e
            // 41c5e lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 41c63 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 41c68 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 41c6c sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 41c6f mov rcx,r15
            rcx = r15;
            // 41c72 sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 41c76 movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 41c80 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 41c83 ja 0x41c91
            if fl.a() { pc = 0x41c91; continue; }
            pc = 0x41c85;
        }
        0x41c85 => {
            // block 41c85
            // 41c85 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 41c88 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 41c8b cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 41c8f jmp 0x41c9b
            pc = 0x41c9b; continue;
        }
        0x41c91 => {
            // block 41c91
            // 41c91 movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x41c9b;
        }
        0x41c9b => {
            // block 41c9b
            // 41c9b mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 41ca0 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 41ca4 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 41ca7 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 41cab mov r13,r14
            r13 = r14;
            // 41cae sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 41cb2 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 41cba mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 41cbe xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 41cc0 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 41cc3 je 0x41cd5
            if fl.e() { pc = 0x41cd5; continue; }
            pc = 0x41cc5;
        }
        0x41cc5 => {
            // block 41cc5
            // 41cc5 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 41ccd call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41cd2 mov rbx,rax
            rbx = rax;
            pc = 0x41cd5;
        }
        0x41cd5 => {
            // block 41cd5
            // 41cd5 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 41cd9 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 41cdd mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 41ce1 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 41ce3 mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 41ce7 lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 41cec mov rdi,rbx
            rdi = rbx;
            // 41cef mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 41cf3 mov rsi,r15
            rsi = r15;
            // 41cf6 mov rdx,r14
            rdx = r14;
            // 41cf9 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41cfe mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 41d02 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 41d06 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 41d0b mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 41d0f mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 41d13 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 41d17 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 41d1c mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 41d20 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 41d24 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 41d28 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 41d2c call 0xebb34
            { let r = { let t = m.env.stub_va(0x301); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41d31 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 41d35 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 41d36 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 41d38 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 41d3a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 41d3c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 41d3e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41d3f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ed46 size=292
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0002ed46<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2ed46;
    loop {
        match pc {
        0x2ed46 => {
            // block 2ed46
            // 2ed46 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ed47 mov rbp,rsp
            rbp = rsp;
            // 2ed4a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2ed4c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2ed4e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2ed50 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2ed52 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2ed53 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 2ed57 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 2ed5b mov r12,rdi
            r12 = rdi;
            // 2ed5e mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 2ed62 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2ed67 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 2ed6a sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // 2ed6e inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2ed71 mov rax,rbx
            rax = rbx;
            // 2ed74 shr rax,0x3d
            { let (r, f) = shr::<64>(rax, 0x3du64, fl); rax = r; fl = f; }
            // 2ed78 je 0x2ed86
            if fl.e() { pc = 0x2ed86; continue; }
            pc = 0x2ed7a;
        }
        0x2ed7a => {
            // block 2ed7a
            // 2ed7a mov rdi,r12
            rdi = r12;
            // 2ed7d call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x2ed86 => {
            // block 2ed86
            // 2ed86 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 2ed8b mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2ed90 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 2ed94 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 2ed97 mov rcx,r15
            rcx = r15;
            // 2ed9a sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 2ed9e movabs rdx,0xffffffffffffffe
            rdx = 0xfff_ffff_ffff_fffeu64;
            // 2eda8 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 2edab ja 0x2edba
            if fl.a() { pc = 0x2edba; continue; }
            pc = 0x2edad;
        }
        0x2edad => {
            // block 2edad
            // 2edad sar r15,0x2
            { let (r, f) = sar::<64>(r15, 0x2u64, fl); r15 = r; fl = f; }
            // 2edb1 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 2edb4 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 2edb8 jmp 0x2edc4
            pc = 0x2edc4; continue;
        }
        0x2edba => {
            // block 2edba
            // 2edba movabs r15,0x1fffffffffffffff
            r15 = 0x1fff_ffff_ffff_ffffu64;
            pc = 0x2edc4;
        }
        0x2edc4 => {
            // block 2edc4
            // 2edc4 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2edc9 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 2edcd sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 2edd0 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 2edd4 mov r13,r14
            r13 = r14;
            // 2edd7 sar r13,0x3
            { let (r, f) = sar::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // 2eddb mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 2ede3 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 2ede7 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 2ede9 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2edec je 0x2edfe
            if fl.e() { pc = 0x2edfe; continue; }
            pc = 0x2edee;
        }
        0x2edee => {
            // block 2edee
            // 2edee lea rdi,[r15*8+0x0]
            rdi = r15.wrapping_mul(8);
            // 2edf6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2edfb mov rbx,rax
            rbx = rax;
            pc = 0x2edfe;
        }
        0x2edfe => {
            // block 2edfe
            // 2edfe lea rax,[rbx+r15*8]
            rax = rbx.wrapping_add(r15.wrapping_mul(8));
            // 2ee02 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 2ee06 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 2ee0a mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 2ee0d mov QWORD PTR [rbx+r13*8],rax
            m.w64(rbx.wrapping_add(r13.wrapping_mul(8)), (rax) as u64);
            // 2ee11 lea r13,[rbx+r13*8+0x8]
            r13 = rbx.wrapping_add(r13.wrapping_mul(8)).wrapping_add(0x8);
            // 2ee16 mov rdi,rbx
            rdi = rbx;
            // 2ee19 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 2ee1d mov rsi,r15
            rsi = r15;
            // 2ee20 mov rdx,r14
            rdx = r14;
            // 2ee23 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ee28 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 2ee2c mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 2ee30 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 2ee35 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 2ee39 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 2ee3d mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 2ee41 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 2ee46 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 2ee4a mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 2ee4e mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 2ee52 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2ee56 call 0xebafe
            { let r = { let t = m.env.stub_va(0x2cb); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ee5b add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 2ee5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2ee60 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2ee62 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2ee64 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2ee66 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2ee68 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ee69 ret
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
            // dfcc4 mov rdi,r12
            rdi = r12;
            // dfcc7 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0xdfcd0 => {
            // block dfcd0
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
            // dfd04 movabs r15,0x1fffffffffffffff
            r15 = 0x1fff_ffff_ffff_ffffu64;
            pc = 0xdfd0e;
        }
        0xdfd0e => {
            // block dfd0e
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
            // dfd38 lea rdi,[r15*8+0x0]
            rdi = r15.wrapping_mul(8);
            // dfd40 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfd45 mov rbx,rax
            rbx = rax;
            pc = 0xdfd48;
        }
        0xdfd48 => {
            // block dfd48
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
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x6a9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // 19aae mov r12,rsi
            r12 = rsi;
            // 19ab1 mov rdi,r15
            rdi = r15;
            // 19ab4 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // 19ac2 mov rdi,r15
            rdi = r15;
            // 19ac5 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x19aca => {
            // block 19aca
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
            // 19b03 lea rbx,[rsi+rax*4]
            rbx = rsi.wrapping_add(rax.wrapping_mul(4));
            // 19b07 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 19b0a jmp 0x19b39
            pc = 0x19b39; continue;
        }
        0x19b0c => {
            // block 19b0c
            // 19b0c movabs rsi,0x3fffffffffffffff
            rsi = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x19b16;
        }
        0x19b16 => {
            // block 19b16
            // 19b16 mov rdi,r15
            rdi = r15;
            // 19b19 call 0xeba20
            { let r = { let t = m.env.stub_va(0x1ed); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19b1e mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19b22 mov rsi,r12
            rsi = r12;
            // 19b25 mov rdx,r14
            rdx = r14;
            // 19b28 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19b2d shl rbx,0x2
            { let (r, f) = shl::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19b31 jmp 0x19b6a
            pc = 0x19b6a; continue;
        }
        0x19b33 => {
            // block 19b33
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
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x19b6a;
        }
        0x19b6a => {
            // block 19b6a
            // 19b6a add QWORD PTR [r15+0x8],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rbx);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 19b6e jmp 0x19b91
            pc = 0x19b91; continue;
        }
        0x19b70 => {
            // block 19b70
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
            // 19b7c lea rax,[rdi-0x4]
            rax = rdi.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19b80 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19b83 not rax
            rax = !rax;
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
            // 19e1e mov r12,rsi
            r12 = rsi;
            // 19e21 mov rdi,r15
            rdi = r15;
            // 19e24 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // 19e32 mov rdi,r15
            rdi = r15;
            // 19e35 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x19e3a => {
            // block 19e3a
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
            // 19e73 lea rbx,[rsi+rax*4]
            rbx = rsi.wrapping_add(rax.wrapping_mul(4));
            // 19e77 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 19e7a jmp 0x19ea9
            pc = 0x19ea9; continue;
        }
        0x19e7c => {
            // block 19e7c
            // 19e7c movabs rsi,0x3fffffffffffffff
            rsi = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x19e86;
        }
        0x19e86 => {
            // block 19e86
            // 19e86 mov rdi,r15
            rdi = r15;
            // 19e89 call 0xeba44
            { let r = { let t = m.env.stub_va(0x211); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19e8e mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19e92 mov rsi,r12
            rsi = r12;
            // 19e95 mov rdx,r14
            rdx = r14;
            // 19e98 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19e9d shl rbx,0x2
            { let (r, f) = shl::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19ea1 jmp 0x19eda
            pc = 0x19eda; continue;
        }
        0x19ea3 => {
            // block 19ea3
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
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x19eda;
        }
        0x19eda => {
            // block 19eda
            // 19eda add QWORD PTR [r15+0x8],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rbx);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 19ede jmp 0x19f01
            pc = 0x19f01; continue;
        }
        0x19ee0 => {
            // block 19ee0
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
            // 19eec lea rax,[rdi-0x4]
            rax = rdi.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19ef0 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19ef3 not rax
            rax = !rax;
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

/// parity: P2v @0x65678 size=302
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00065678<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x65678;
    loop {
        match pc {
        0x65678 => {
            // block 65678
            // 65678 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 65679 mov rbp,rsp
            rbp = rsp;
            // 6567c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6567e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 65680 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 65682 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65684 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65685 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 65689 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 6568d mov r13,rdi
            r13 = rdi;
            // 65690 mov rsi,QWORD PTR [r13+0x0]
            rsi = m.r64(r13) as u64;
            // 65694 mov rbx,QWORD PTR [r13+0x8]
            rbx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 65698 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 6569b sar rbx,0x4
            { let (r, f) = sar::<64>(rbx, 0x4u64, fl); rbx = r; fl = f; }
            // 6569f inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 656a2 mov rax,rbx
            rax = rbx;
            // 656a5 shr rax,0x3c
            { let (r, f) = shr::<64>(rax, 0x3cu64, fl); rax = r; fl = f; }
            // 656a9 je 0x656b7
            if fl.e() { pc = 0x656b7; continue; }
            pc = 0x656ab;
        }
        0x656ab => {
            // block 656ab
            // 656ab mov rdi,r13
            rdi = r13;
            // 656ae call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x656b7 => {
            // block 656b7
            // 656b7 lea rax,[r13+0x10]
            rax = r13.wrapping_add(0x10);
            // 656bb mov r15,QWORD PTR [r13+0x10]
            r15 = m.r64(r13.wrapping_add(0x10)) as u64;
            // 656bf mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 656c3 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 656c6 mov rcx,r15
            rcx = r15;
            // 656c9 sar rcx,0x4
            { let (r, f) = sar::<64>(rcx, 0x4u64, fl); rcx = r; fl = f; }
            // 656cd movabs rdx,0x7fffffffffffffe
            rdx = 0x7ff_ffff_ffff_fffeu64;
            // 656d7 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 656da ja 0x656e9
            if fl.a() { pc = 0x656e9; continue; }
            pc = 0x656dc;
        }
        0x656dc => {
            // block 656dc
            // 656dc sar r15,0x3
            { let (r, f) = sar::<64>(r15, 0x3u64, fl); r15 = r; fl = f; }
            // 656e0 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 656e3 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 656e7 jmp 0x656f3
            pc = 0x656f3; continue;
        }
        0x656e9 => {
            // block 656e9
            // 656e9 movabs r15,0xfffffffffffffff
            r15 = 0xfff_ffff_ffff_ffffu64;
            pc = 0x656f3;
        }
        0x656f3 => {
            // block 656f3
            // 656f3 mov r14,QWORD PTR [r13+0x8]
            r14 = m.r64(r13.wrapping_add(0x8)) as u64;
            // 656f7 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 656fb sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 656fe mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 65702 mov r12,r14
            r12 = r14;
            // 65705 sar r12,0x4
            { let (r, f) = sar::<64>(r12, 0x4u64, fl); r12 = r; fl = f; }
            // 65709 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 65711 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 65715 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 65717 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 6571a je 0x6572b
            if fl.e() { pc = 0x6572b; continue; }
            pc = 0x6571c;
        }
        0x6571c => {
            // block 6571c
            // 6571c mov rdi,r15
            rdi = r15;
            // 6571f shl rdi,0x4
            { let (r, f) = shl::<64>(rdi, 0x4u64, fl); rdi = r; fl = f; }
            // 65723 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65728 mov rbx,rax
            rbx = rax;
            pc = 0x6572b;
        }
        0x6572b => {
            // block 6572b
            // 6572b shl r12,0x4
            { let (r, f) = shl::<64>(r12, 0x4u64, fl); r12 = r; fl = f; }
            // 6572f shl r15,0x4
            { let (r, f) = shl::<64>(r15, 0x4u64, fl); r15 = r; fl = f; }
            // 65733 add r15,rbx
            { let (r, f) = add::<64>(r15, rbx);
              r15 = r;
              fl = f; }
            // 65736 mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 6573a mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 6573d mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 65741 mov QWORD PTR [rbx+r12*1+0x8],rcx
            m.w64(rbx.wrapping_add(r12).wrapping_add(0x8), (rcx) as u64);
            // 65746 mov QWORD PTR [rbx+r12*1],rax
            m.w64(rbx.wrapping_add(r12), (rax) as u64);
            // 6574a lea rax,[rbx+r12*1+0x10]
            rax = rbx.wrapping_add(r12).wrapping_add(0x10);
            // 6574f mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 65753 mov rdi,rbx
            rdi = rbx;
            // 65756 mov r12,QWORD PTR [rbp-0x58]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 6575a mov rsi,r12
            rsi = r12;
            // 6575d mov rdx,r14
            rdx = r14;
            // 65760 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65765 mov QWORD PTR [r13+0x0],rbx
            m.w64(r13, (rbx) as u64);
            // 65769 mov QWORD PTR [rbp-0x48],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r12) as u64);
            // 6576d mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 65771 mov QWORD PTR [r13+0x8],rax
            m.w64(r13.wrapping_add(0x8), (rax) as u64);
            // 65775 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 65779 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 6577d mov QWORD PTR [r13+0x10],r15
            m.w64(r13.wrapping_add(0x10), (r15) as u64);
            // 65781 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 65785 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 65789 mov QWORD PTR [rbp-0x50],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r12) as u64);
            // 6578d lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 65791 call 0xebd08
            { let r = { let t = m.env.stub_va(0x4d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65796 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 6579a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6579b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6579d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 6579f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 657a1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 657a3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 657a4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d6d0 size=364
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0002d6d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2d6d0;
    loop {
        match pc {
        0x2d6d0 => {
            // block 2d6d0
            // 2d6d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d6d1 mov rbp,rsp
            rbp = rsp;
            // 2d6d4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2d6d6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d6d8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2d6da push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2d6dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d6dd sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 2d6e1 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 2d6e5 mov r12,rdi
            r12 = rdi;
            // 2d6e8 movabs r15,0x1555555555555555
            r15 = 0x1555_5555_5555_5555u64;
            // 2d6f2 mov r14,QWORD PTR [r12]
            r14 = m.r64(r12) as u64;
            // 2d6f6 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2d6fb sub rbx,r14
            { let (r, f) = sub::<64>(rbx, r14);
              rbx = r;
              fl = f; }
            // 2d6fe sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 2d702 movabs rsi,0xaaaaaaaaaaaaaaab
            rsi = 0xaaaa_aaaa_aaaa_aaabu64;
            // 2d70c imul rbx,rsi
            { let (r, f) = imul::<64>(rbx, rsi); rbx = r; fl = f; }
            // 2d710 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2d713 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 2d716 jbe 0x2d72a
            if fl.be() { pc = 0x2d72a; continue; }
            pc = 0x2d718;
        }
        0x2d718 => {
            // block 2d718
            // 2d718 mov rdi,r12
            rdi = r12;
            // 2d71b mov r14,rsi
            r14 = rsi;
            // 2d71e call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x2d72a => {
            // block 2d72a
            // 2d72a lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 2d72f mov rcx,QWORD PTR [r12+0x10]
            rcx = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2d734 mov QWORD PTR [rbp-0x60],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rcx) as u64);
            // 2d738 sub rcx,r14
            { let (r, f) = sub::<64>(rcx, r14);
              rcx = r;
              fl = f; }
            // 2d73b sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 2d73f imul rcx,rsi
            { let (r, f) = imul::<64>(rcx, rsi); rcx = r; fl = f; }
            // 2d743 movabs rdx,0xaaaaaaaaaaaaaa9
            rdx = 0xaaa_aaaa_aaaa_aaa9u64;
            // 2d74d cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 2d750 ja 0x2d75f
            if fl.a() { pc = 0x2d75f; continue; }
            pc = 0x2d752;
        }
        0x2d752 => {
            // block 2d752
            // 2d752 add rcx,rcx
            { let (r, f) = add::<64>(rcx, rcx);
              rcx = r;
              fl = f; }
            // 2d755 cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // 2d758 cmovb rcx,rbx
            if fl.b() { rcx = rbx; }
            // 2d75c mov r15,rcx
            r15 = rcx;
            pc = 0x2d75f;
        }
        0x2d75f => {
            // block 2d75f
            // 2d75f mov r13,QWORD PTR [r12+0x8]
            r13 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2d764 mov QWORD PTR [rbp-0x68],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r13) as u64);
            // 2d768 sub r13,r14
            { let (r, f) = sub::<64>(r13, r14);
              r13 = r;
              fl = f; }
            // 2d76b mov rbx,r13
            rbx = r13;
            // 2d76e sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 2d772 imul rbx,rsi
            { let (r, f) = imul::<64>(rbx, rsi); rbx = r; fl = f; }
            // 2d776 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 2d77e mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 2d782 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 2d784 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2d787 je 0x2d79d
            if fl.e() { pc = 0x2d79d; continue; }
            pc = 0x2d789;
        }
        0x2d789 => {
            // block 2d789
            // 2d789 lea rax,[r15*4+0x0]
            rax = r15.wrapping_mul(4);
            // 2d791 lea rdi,[rax+rax*2]
            rdi = rax.wrapping_add(rax.wrapping_mul(2));
            // 2d795 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d79a mov rcx,rax
            rcx = rax;
            pc = 0x2d79d;
        }
        0x2d79d => {
            // block 2d79d
            // 2d79d lea rax,[rbx+rbx*2]
            rax = rbx.wrapping_add(rbx.wrapping_mul(2));
            // 2d7a1 lea rdx,[r15+r15*2]
            rdx = r15.wrapping_add(r15.wrapping_mul(2));
            // 2d7a5 lea rdx,[rcx+rdx*4]
            rdx = rcx.wrapping_add(rdx.wrapping_mul(4));
            // 2d7a9 mov QWORD PTR [rbp-0x70],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rdx) as u64);
            // 2d7ad mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 2d7b1 mov edx,DWORD PTR [rsi+0x8]
            rdx = (m.r32(rsi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 2d7b4 mov DWORD PTR [rcx+rax*4+0x8],edx
            m.w32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2d7b8 mov rdx,QWORD PTR [rsi]
            rdx = m.r64(rsi) as u64;
            // 2d7bb mov QWORD PTR [rcx+rax*4],rdx
            m.w64(rcx.wrapping_add(rax.wrapping_mul(4)), (rdx) as u64);
            // 2d7bf lea r15,[rcx+rax*4+0xc]
            r15 = rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xc);
            // 2d7c4 movabs rdx,0xd555555555555555
            rdx = 0xd555_5555_5555_5555u64;
            // 2d7ce mov rax,r13
            rax = r13;
            // 2d7d1 imul rdx
            { let (lo, hi, f) = imul1::<64>(rax, rdx); rax = lo; rdx = hi; fl = f; }
            // 2d7d4 mov rax,rdx
            rax = rdx;
            // 2d7d7 shr rax,0x3f
            { let (r, f) = shr::<64>(rax, 0x3fu64, fl); rax = r; fl = f; }
            // 2d7db sar rdx,1
            { let (r, f) = sar::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2d7de add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 2d7e1 add rdx,rbx
            { let (r, f) = add::<64>(rdx, rbx);
              rdx = r;
              fl = f; }
            // 2d7e4 lea rax,[rdx+rdx*2]
            rax = rdx.wrapping_add(rdx.wrapping_mul(2));
            // 2d7e8 lea rbx,[rcx+rax*4]
            rbx = rcx.wrapping_add(rax.wrapping_mul(4));
            // 2d7ec mov rdi,rbx
            rdi = rbx;
            // 2d7ef mov rsi,r14
            rsi = r14;
            // 2d7f2 mov rdx,r13
            rdx = r13;
            // 2d7f5 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d7fa mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 2d7fe mov QWORD PTR [rbp-0x48],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r14) as u64);
            // 2d802 mov QWORD PTR [r12+0x8],r15
            m.w64(r12.wrapping_add(0x8), (r15) as u64);
            // 2d807 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 2d80b mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 2d80f mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 2d813 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 2d818 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 2d81c mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 2d820 mov QWORD PTR [rbp-0x50],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r14) as u64);
            // 2d824 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2d828 call 0xeba9e
            { let r = { let t = m.env.stub_va(0x26b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d82d add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 2d831 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d832 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2d834 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2d836 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d838 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d83a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d83b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe06ae size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e06ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe06ae;
    loop {
        match pc {
        0xe06ae => {
            // block e06ae
            // e06ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e06af mov rbp,rsp
            rbp = rsp;
            // e06b2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e06b4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e06b5 mov r14,rsi
            r14 = rsi;
            // e06b8 mov rbx,rdi
            rbx = rdi;
            // e06bb lea rax,[rip+0x65b06] # 0x1461c8
            rax = m.env.data_va(371093);
            // e06c2 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e06c5 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // e06c9 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // e06cd mov rdi,QWORD PTR [r14+0x10]
            rdi = m.r64(r14.wrapping_add(0x10)) as u64;
            // e06d1 call 0xec2ba
            { let r = { let t = m.env.stub_va(0xa87); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e06d6 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e06da mov eax,DWORD PTR [r14+0x18]
            rax = (m.r32(r14.wrapping_add(0x18)) as u64) as u32 as u64;
            // e06de mov DWORD PTR [rbx+0x18],eax
            m.w32(rbx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // e06e1 mov eax,DWORD PTR [r14+0x1c]
            rax = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // e06e5 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // e06e8 movups xmm0,XMMWORD PTR [r14+0x20]
            x0 = m.r128(r14.wrapping_add(0x20));
            // e06ed movups XMMWORD PTR [rbx+0x20],xmm0
            m.w128(rbx.wrapping_add(0x20), x0);
            // e06f1 mov rax,QWORD PTR [r14+0x30]
            rax = m.r64(r14.wrapping_add(0x30)) as u64;
            // e06f5 mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // e06f9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e06fa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e06fc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e06fd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe06fe size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e06fe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe06fe;
    loop {
        match pc {
        0xe06fe => {
            // block e06fe
            // e06fe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e06ff mov rbp,rsp
            rbp = rsp;
            // e0702 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0704 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0705 mov r14,rsi
            r14 = rsi;
            // e0708 mov rbx,rdi
            rbx = rdi;
            // e070b lea rax,[rip+0x65ab6] # 0x1461c8
            rax = m.env.data_va(371093);
            // e0712 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e0715 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // e0719 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // e071d mov rdi,QWORD PTR [r14+0x10]
            rdi = m.r64(r14.wrapping_add(0x10)) as u64;
            // e0721 call 0xec2ba
            { let r = { let t = m.env.stub_va(0xa87); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0726 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e072a mov eax,DWORD PTR [r14+0x18]
            rax = (m.r32(r14.wrapping_add(0x18)) as u64) as u32 as u64;
            // e072e mov DWORD PTR [rbx+0x18],eax
            m.w32(rbx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // e0731 mov eax,DWORD PTR [r14+0x1c]
            rax = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // e0735 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // e0738 movups xmm0,XMMWORD PTR [r14+0x20]
            x0 = m.r128(r14.wrapping_add(0x20));
            // e073d movups XMMWORD PTR [rbx+0x20],xmm0
            m.w128(rbx.wrapping_add(0x20), x0);
            // e0741 mov rax,QWORD PTR [r14+0x30]
            rax = m.r64(r14.wrapping_add(0x30)) as u64;
            // e0745 mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // e0749 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e074a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e074c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e074d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd162b size=949
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d162b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd162b;
    loop {
        match pc {
        0xd162b => {
            // block d162b
            // d162b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d162c mov rbp,rsp
            rbp = rsp;
            // d162f mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // d1634 cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // d163a jg 0xd1664
            if fl.g() { pc = 0xd1664; continue; }
            pc = 0xd163c;
        }
        0xd163c => {
            // block d163c
            // d163c cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // d1642 jg 0xd16ad
            if fl.g() { pc = 0xd16ad; continue; }
            pc = 0xd1644;
        }
        0xd1644 => {
            // block d1644
            // d1644 lea eax,[rdi-0x10266400]
            rax = (rdi.wrapping_add(0xffff_ffff_efd9_9c00)) as u32 as u64;
            // d164a cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d164d jb 0xd1778
            if fl.b() { pc = 0xd1778; continue; }
            pc = 0xd1653;
        }
        0xd1653 => {
            // block d1653
            // d1653 cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // d1659 je 0xd1778
            if fl.e() { pc = 0xd1778; continue; }
            pc = 0xd165f;
        }
        0xd165f => {
            // block d165f
            // d165f jmp 0xd17d5
            pc = 0xd17d5; continue;
        }
        0xd1664 => {
            // block d1664
            // d1664 cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // d166a jg 0xd16d2
            if fl.g() { pc = 0xd16d2; continue; }
            pc = 0xd166c;
        }
        0xd166c => {
            // block d166c
            // d166c cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // d1672 jg 0xd1709
            if fl.g() { pc = 0xd1709; continue; }
            pc = 0xd1678;
        }
        0xd1678 => {
            // block d1678
            // d1678 cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // d167e jg 0xd1760
            if fl.g() { pc = 0xd1760; continue; }
            pc = 0xd1684;
        }
        0xd1684 => {
            // block d1684
            // d1684 cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // d168a je 0xd17b1
            if fl.e() { pc = 0xd17b1; continue; }
            pc = 0xd1690;
        }
        0xd1690 => {
            // block d1690
            // d1690 cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // d1696 je 0xd1778
            if fl.e() { pc = 0xd1778; continue; }
            pc = 0xd169c;
        }
        0xd169c => {
            // block d169c
            // d169c cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // d16a2 je 0xd1778
            if fl.e() { pc = 0xd1778; continue; }
            pc = 0xd16a8;
        }
        0xd16a8 => {
            // block d16a8
            // d16a8 jmp 0xd17d5
            pc = 0xd17d5; continue;
        }
        0xd16ad => {
            // block d16ad
            // d16ad cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // d16b3 jg 0xd172e
            if fl.g() { pc = 0xd172e; continue; }
            pc = 0xd16b5;
        }
        0xd16b5 => {
            // block d16b5
            // d16b5 cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // d16bb je 0xd1778
            if fl.e() { pc = 0xd1778; continue; }
            pc = 0xd16c1;
        }
        0xd16c1 => {
            // block d16c1
            // d16c1 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // d16c7 je 0xd1778
            if fl.e() { pc = 0xd1778; continue; }
            pc = 0xd16cd;
        }
        0xd16cd => {
            // block d16cd
            // d16cd jmp 0xd17d5
            pc = 0xd17d5; continue;
        }
        0xd16d2 => {
            // block d16d2
            // d16d2 cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // d16d8 jg 0xd1743
            if fl.g() { pc = 0xd1743; continue; }
            pc = 0xd16da;
        }
        0xd16da => {
            // block d16da
            // d16da lea eax,[rdi-0x10646700]
            rax = (rdi.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d16e0 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // d16e3 jb 0xd17b1
            if fl.b() { pc = 0xd17b1; continue; }
            pc = 0xd16e9;
        }
        0xd16e9 => {
            // block d16e9
            // d16e9 lea eax,[rdi-0x10646705]
            rax = (rdi.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d16ef cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d16f2 jb 0xd17b1
            if fl.b() { pc = 0xd17b1; continue; }
            pc = 0xd16f8;
        }
        0xd16f8 => {
            // block d16f8
            // d16f8 cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // d16fe je 0xd17b1
            if fl.e() { pc = 0xd17b1; continue; }
            pc = 0xd1704;
        }
        0xd1704 => {
            // block d1704
            // d1704 jmp 0xd17d5
            pc = 0xd17d5; continue;
        }
        0xd1709 => {
            // block d1709
            // d1709 cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // d170f jg 0xd178f
            if fl.g() { pc = 0xd178f; continue; }
            pc = 0xd1711;
        }
        0xd1711 => {
            // block d1711
            // d1711 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // d1717 je 0xd17b1
            if fl.e() { pc = 0xd17b1; continue; }
            pc = 0xd171d;
        }
        0xd171d => {
            // block d171d
            // d171d cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // d1723 je 0xd17b1
            if fl.e() { pc = 0xd17b1; continue; }
            pc = 0xd1729;
        }
        0xd1729 => {
            // block d1729
            // d1729 jmp 0xd17d5
            pc = 0xd17d5; continue;
        }
        0xd172e => {
            // block d172e
            // d172e cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // d1734 je 0xd17b1
            if fl.e() { pc = 0xd17b1; continue; }
            pc = 0xd1736;
        }
        0xd1736 => {
            // block d1736
            // d1736 cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // d173c je 0xd1778
            if fl.e() { pc = 0xd1778; continue; }
            pc = 0xd173e;
        }
        0xd173e => {
            // block d173e
            // d173e jmp 0xd17d5
            pc = 0xd17d5; continue;
        }
        0xd1743 => {
            // block d1743
            // d1743 lea eax,[rdi-0x10710850]
            rax = (rdi.wrapping_add(0xffff_ffff_ef8e_f7b0)) as u32 as u64;
            // d1749 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d174c jb 0xd17b1
            if fl.b() { pc = 0xd17b1; continue; }
            pc = 0xd174e;
        }
        0xd174e => {
            // block d174e
            // d174e cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // d1754 je 0xd17b1
            if fl.e() { pc = 0xd17b1; continue; }
            pc = 0xd1756;
        }
        0xd1756 => {
            // block d1756
            // d1756 cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // d175c jne 0xd17d5
            if fl.ne() { pc = 0xd17d5; continue; }
            pc = 0xd175e;
        }
        0xd175e => {
            // block d175e
            // d175e jmp 0xd17b1
            pc = 0xd17b1; continue;
        }
        0xd1760 => {
            // block d1760
            // d1760 cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // d1766 jg 0xd17a1
            if fl.g() { pc = 0xd17a1; continue; }
            pc = 0xd1768;
        }
        0xd1768 => {
            // block d1768
            // d1768 cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // d176e je 0xd17b1
            if fl.e() { pc = 0xd17b1; continue; }
            pc = 0xd1770;
        }
        0xd1770 => {
            // block d1770
            // d1770 cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // d1776 jne 0xd17d5
            if fl.ne() { pc = 0xd17d5; continue; }
            pc = 0xd1778;
        }
        0xd1778 => {
            // block d1778
            // d1778 add esi,0xfffffff8
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffff8u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d177b cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // d177e jae 0xd17d5
            if fl.ae() { pc = 0xd17d5; continue; }
            pc = 0xd1780;
        }
        0xd1780 => {
            // block d1780
            // d1780 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // d1783 lea rcx,[rip+0x31726] # 0x102eb0
            rcx = m.env.data_va(95869);
            pc = 0xd178a;
        }
        0xd178a => {
            // block d178a
            // d178a mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // d178d jmp 0xd17d5
            pc = 0xd17d5; continue;
        }
        0xd178f => {
            // block d178f
            // d178f cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // d1795 je 0xd17b1
            if fl.e() { pc = 0xd17b1; continue; }
            pc = 0xd1797;
        }
        0xd1797 => {
            // block d1797
            // d1797 cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // d179d je 0xd17b1
            if fl.e() { pc = 0xd17b1; continue; }
            pc = 0xd179f;
        }
        0xd179f => {
            // block d179f
            // d179f jmp 0xd17d5
            pc = 0xd17d5; continue;
        }
        0xd17a1 => {
            // block d17a1
            // d17a1 cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // d17a7 je 0xd17b1
            if fl.e() { pc = 0xd17b1; continue; }
            pc = 0xd17a9;
        }
        0xd17a9 => {
            // block d17a9
            // d17a9 cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // d17af jne 0xd17d5
            if fl.ne() { pc = 0xd17d5; continue; }
            pc = 0xd17b1;
        }
        0xd17b1 => {
            // block d17b1
            // d17b1 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d17b3 cmp esi,0x11
            fl = sub::<32>((rsi as u32 as u64), 0x11u64).1;
            // d17b6 ja 0xd17d5
            if fl.a() { pc = 0xd17d5; continue; }
            pc = 0xd17b8;
        }
        0xd17b8 => {
            // block d17b8
            // d17b8 lea rsi,[rip+0x1d9] # 0xd1998
            rsi = m.env.code_va(0xd1998);
            // d17bf movsxd rax,DWORD PTR [rsi+rax*4]
            rax = sx::<32>(m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // d17c3 add rax,rsi
            { let (r, f) = add::<64>(rax, rsi);
              rax = r;
              fl = f; }
            // d17c6 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd17c8 => {
            // block d17c8
            // d17c8 cmp edx,0x16
            fl = sub::<32>((rdx as u32 as u64), 0x16u64).1;
            // d17cb sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d17ce movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // d17d1 lea ecx,[rax+rax*1+0x2]
            rcx = (rax.wrapping_add(rax).wrapping_add(0x2)) as u32 as u64;
            pc = 0xd17d5;
        }
        0xd17d5 => {
            // block d17d5
            // d17d5 mov eax,0x800000
            rax = (0x800000u64) as u32 as u64;
            // d17da cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // d17e0 jg 0xd180a
            if fl.g() { pc = 0xd180a; continue; }
            pc = 0xd17e2;
        }
        0xd17e2 => {
            // block d17e2
            // d17e2 cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // d17e8 jg 0xd1853
            if fl.g() { pc = 0xd1853; continue; }
            pc = 0xd17ea;
        }
        0xd17ea => {
            // block d17ea
            // d17ea lea edx,[rdi-0x10266400]
            rdx = (rdi.wrapping_add(0xffff_ffff_efd9_9c00)) as u32 as u64;
            // d17f0 cmp edx,0x2
            fl = sub::<32>((rdx as u32 as u64), 0x2u64).1;
            // d17f3 jb 0xd1936
            if fl.b() { pc = 0xd1936; continue; }
            pc = 0xd17f9;
        }
        0xd17f9 => {
            // block d17f9
            // d17f9 cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // d17ff jne 0xd1934
            if fl.ne() { pc = 0xd1934; continue; }
            pc = 0xd1805;
        }
        0xd1805 => {
            // block d1805
            // d1805 jmp 0xd1936
            pc = 0xd1936; continue;
        }
        0xd180a => {
            // block d180a
            // d180a cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // d1810 jg 0xd1878
            if fl.g() { pc = 0xd1878; continue; }
            pc = 0xd1812;
        }
        0xd1812 => {
            // block d1812
            // d1812 cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // d1818 jg 0xd18af
            if fl.g() { pc = 0xd18af; continue; }
            pc = 0xd181e;
        }
        0xd181e => {
            // block d181e
            // d181e cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // d1824 jg 0xd18f8
            if fl.g() { pc = 0xd18f8; continue; }
            pc = 0xd182a;
        }
        0xd182a => {
            // block d182a
            // d182a cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // d1830 je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd1836;
        }
        0xd1836 => {
            // block d1836
            // d1836 cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // d183c je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd1842;
        }
        0xd1842 => {
            // block d1842
            // d1842 cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // d1848 jne 0xd1934
            if fl.ne() { pc = 0xd1934; continue; }
            pc = 0xd184e;
        }
        0xd184e => {
            // block d184e
            // d184e jmp 0xd1936
            pc = 0xd1936; continue;
        }
        0xd1853 => {
            // block d1853
            // d1853 cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // d1859 jg 0xd18c9
            if fl.g() { pc = 0xd18c9; continue; }
            pc = 0xd185b;
        }
        0xd185b => {
            // block d185b
            // d185b cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // d1861 je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd1867;
        }
        0xd1867 => {
            // block d1867
            // d1867 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // d186d jne 0xd1934
            if fl.ne() { pc = 0xd1934; continue; }
            pc = 0xd1873;
        }
        0xd1873 => {
            // block d1873
            // d1873 jmp 0xd1936
            pc = 0xd1936; continue;
        }
        0xd1878 => {
            // block d1878
            // d1878 cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // d187e jg 0xd18db
            if fl.g() { pc = 0xd18db; continue; }
            pc = 0xd1880;
        }
        0xd1880 => {
            // block d1880
            // d1880 lea edx,[rdi-0x10646700]
            rdx = (rdi.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d1886 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // d1889 jb 0xd1936
            if fl.b() { pc = 0xd1936; continue; }
            pc = 0xd188f;
        }
        0xd188f => {
            // block d188f
            // d188f lea edx,[rdi-0x10646705]
            rdx = (rdi.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d1895 cmp edx,0x2
            fl = sub::<32>((rdx as u32 as u64), 0x2u64).1;
            // d1898 jb 0xd1936
            if fl.b() { pc = 0xd1936; continue; }
            pc = 0xd189e;
        }
        0xd189e => {
            // block d189e
            // d189e cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // d18a4 jne 0xd1934
            if fl.ne() { pc = 0xd1934; continue; }
            pc = 0xd18aa;
        }
        0xd18aa => {
            // block d18aa
            // d18aa jmp 0xd1936
            pc = 0xd1936; continue;
        }
        0xd18af => {
            // block d18af
            // d18af cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // d18b5 jg 0xd1912
            if fl.g() { pc = 0xd1912; continue; }
            pc = 0xd18b7;
        }
        0xd18b7 => {
            // block d18b7
            // d18b7 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // d18bd je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd18bf;
        }
        0xd18bf => {
            // block d18bf
            // d18bf cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // d18c5 jne 0xd1934
            if fl.ne() { pc = 0xd1934; continue; }
            pc = 0xd18c7;
        }
        0xd18c7 => {
            // block d18c7
            // d18c7 jmp 0xd1936
            pc = 0xd1936; continue;
        }
        0xd18c9 => {
            // block d18c9
            // d18c9 cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // d18cf je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd18d1;
        }
        0xd18d1 => {
            // block d18d1
            // d18d1 cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // d18d7 jne 0xd1934
            if fl.ne() { pc = 0xd1934; continue; }
            pc = 0xd18d9;
        }
        0xd18d9 => {
            // block d18d9
            // d18d9 jmp 0xd1936
            pc = 0xd1936; continue;
        }
        0xd18db => {
            // block d18db
            // d18db lea edx,[rdi-0x10710850]
            rdx = (rdi.wrapping_add(0xffff_ffff_ef8e_f7b0)) as u32 as u64;
            // d18e1 cmp edx,0x2
            fl = sub::<32>((rdx as u32 as u64), 0x2u64).1;
            // d18e4 jb 0xd1936
            if fl.b() { pc = 0xd1936; continue; }
            pc = 0xd18e6;
        }
        0xd18e6 => {
            // block d18e6
            // d18e6 cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // d18ec je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd18ee;
        }
        0xd18ee => {
            // block d18ee
            // d18ee cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // d18f4 je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd18f6;
        }
        0xd18f6 => {
            // block d18f6
            // d18f6 jmp 0xd1934
            pc = 0xd1934; continue;
        }
        0xd18f8 => {
            // block d18f8
            // d18f8 cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // d18fe jg 0xd1924
            if fl.g() { pc = 0xd1924; continue; }
            pc = 0xd1900;
        }
        0xd1900 => {
            // block d1900
            // d1900 cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // d1906 je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd1908;
        }
        0xd1908 => {
            // block d1908
            // d1908 cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // d190e jne 0xd1934
            if fl.ne() { pc = 0xd1934; continue; }
            pc = 0xd1910;
        }
        0xd1910 => {
            // block d1910
            // d1910 jmp 0xd1936
            pc = 0xd1936; continue;
        }
        0xd1912 => {
            // block d1912
            // d1912 cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // d1918 je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd191a;
        }
        0xd191a => {
            // block d191a
            // d191a cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // d1920 jne 0xd1934
            if fl.ne() { pc = 0xd1934; continue; }
            pc = 0xd1922;
        }
        0xd1922 => {
            // block d1922
            // d1922 jmp 0xd1936
            pc = 0xd1936; continue;
        }
        0xd1924 => {
            // block d1924
            // d1924 cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // d192a je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd192c;
        }
        0xd192c => {
            // block d192c
            // d192c cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // d1932 je 0xd1936
            if fl.e() { pc = 0xd1936; continue; }
            pc = 0xd1934;
        }
        0xd1934 => {
            // block d1934
            // d1934 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd1936;
        }
        0xd1936 => {
            // block d1936
            // d1936 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // d1939 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d193a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd193b => {
            // block d193b
            // d193b add edx,0xfffffff0
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff0u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d193e cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d1941 jae 0xd17d5
            if fl.ae() { pc = 0xd17d5; continue; }
            pc = 0xd1947;
        }
        0xd1947 => {
            // block d1947
            // d1947 movsxd rax,edx
            rax = sx::<32>((rdx as u32 as u64)) as u64;
            // d194a lea rcx,[rip+0x3157f] # 0x102ed0
            rcx = m.env.data_va(95901);
            // d1951 jmp 0xd178a
            pc = 0xd178a; continue;
        }
        0xd1956 => {
            // block d1956
            // d1956 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // d195b cmp edx,0x10
            fl = sub::<32>((rdx as u32 as u64), 0x10u64).1;
            // d195e je 0xd17d5
            if fl.e() { pc = 0xd17d5; continue; }
            pc = 0xd1964;
        }
        0xd1964 => {
            // block d1964
            // d1964 cmp edx,0x15
            fl = sub::<32>((rdx as u32 as u64), 0x15u64).1;
            // d1967 je 0xd17d5
            if fl.e() { pc = 0xd17d5; continue; }
            pc = 0xd196d;
        }
        0xd196d => {
            // block d196d
            // d196d mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // d1972 jmp 0xd17d5
            pc = 0xd17d5; continue;
        }
        0xd1977 => {
            // block d1977
            // d1977 add edx,0xfffffffe
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffffeu64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d197a mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // d197f cmp edx,0x14
            fl = sub::<32>((rdx as u32 as u64), 0x14u64).1;
            // d1982 jae 0xd17d5
            if fl.ae() { pc = 0xd17d5; continue; }
            pc = 0xd1988;
        }
        0xd1988 => {
            // block d1988
            // d1988 movsxd rax,edx
            rax = sx::<32>((rdx as u32 as u64)) as u64;
            // d198b lea rcx,[rip+0x314ce] # 0x102e60
            rcx = m.env.data_va(95789);
            // d1992 jmp 0xd178a
            pc = 0xd178a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe064a size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e064a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe064a;
    loop {
        match pc {
        0xe064a => {
            // block e064a
            // e064a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e064b mov rbp,rsp
            rbp = rsp;
            // e064e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e064f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0650 mov rbx,rdi
            rbx = rdi;
            // e0653 mov rax,QWORD PTR [rsi+0x30]
            rax = m.r64(rsi.wrapping_add(0x30)) as u64;
            // e0657 mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // e065b mov rax,QWORD PTR [rsi+0x28]
            rax = m.r64(rsi.wrapping_add(0x28)) as u64;
            // e065f mov QWORD PTR [rbx+0x28],rax
            m.w64(rbx.wrapping_add(0x28), (rax) as u64);
            // e0663 mov rax,QWORD PTR [rsi+0x20]
            rax = m.r64(rsi.wrapping_add(0x20)) as u64;
            // e0667 mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // e066b mov rax,QWORD PTR [rsi+0x18]
            rax = m.r64(rsi.wrapping_add(0x18)) as u64;
            // e066f mov QWORD PTR [rbx+0x18],rax
            m.w64(rbx.wrapping_add(0x18), (rax) as u64);
            // e0673 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // e0677 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e067b mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // e067e mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // e0682 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // e0686 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e0689 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // e068d test rax,rax
            fl = logic::<64>(rax & rax);
            // e0690 lea rdi,[rip+0x4b551] # 0x12bbe8
            rdi = m.env.data_va(263093);
            // e0697 cmovne rdi,rax
            if fl.ne() { rdi = rax; }
            // e069b call 0xec2ba
            { let r = { let t = m.env.stub_va(0xa87); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e06a0 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e06a4 mov rax,rbx
            rax = rbx;
            // e06a7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e06ab pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e06ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e06ad ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd1b6c size=926
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d1b6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd1b6c;
    loop {
        match pc {
        0xd1b6c => {
            // block d1b6c
            // d1b6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d1b6d mov rbp,rsp
            rbp = rsp;
            // d1b70 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // d1b76 cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // d1b7c jg 0xd1ba6
            if fl.g() { pc = 0xd1ba6; continue; }
            pc = 0xd1b7e;
        }
        0xd1b7e => {
            // block d1b7e
            // d1b7e cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // d1b84 jg 0xd1bef
            if fl.g() { pc = 0xd1bef; continue; }
            pc = 0xd1b86;
        }
        0xd1b86 => {
            // block d1b86
            // d1b86 lea eax,[rdi-0x10266400]
            rax = (rdi.wrapping_add(0xffff_ffff_efd9_9c00)) as u32 as u64;
            // d1b8c cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1b8f jb 0xd1cc6
            if fl.b() { pc = 0xd1cc6; continue; }
            pc = 0xd1b95;
        }
        0xd1b95 => {
            // block d1b95
            // d1b95 cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // d1b9b je 0xd1cc6
            if fl.e() { pc = 0xd1cc6; continue; }
            pc = 0xd1ba1;
        }
        0xd1ba1 => {
            // block d1ba1
            // d1ba1 jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1ba6 => {
            // block d1ba6
            // d1ba6 cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // d1bac jg 0xd1c14
            if fl.g() { pc = 0xd1c14; continue; }
            pc = 0xd1bae;
        }
        0xd1bae => {
            // block d1bae
            // d1bae cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // d1bb4 jg 0xd1c4b
            if fl.g() { pc = 0xd1c4b; continue; }
            pc = 0xd1bba;
        }
        0xd1bba => {
            // block d1bba
            // d1bba cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // d1bc0 jg 0xd1cae
            if fl.g() { pc = 0xd1cae; continue; }
            pc = 0xd1bc6;
        }
        0xd1bc6 => {
            // block d1bc6
            // d1bc6 cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // d1bcc je 0xd1d00
            if fl.e() { pc = 0xd1d00; continue; }
            pc = 0xd1bd2;
        }
        0xd1bd2 => {
            // block d1bd2
            // d1bd2 cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // d1bd8 je 0xd1cc6
            if fl.e() { pc = 0xd1cc6; continue; }
            pc = 0xd1bde;
        }
        0xd1bde => {
            // block d1bde
            // d1bde cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // d1be4 je 0xd1cc6
            if fl.e() { pc = 0xd1cc6; continue; }
            pc = 0xd1bea;
        }
        0xd1bea => {
            // block d1bea
            // d1bea jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1bef => {
            // block d1bef
            // d1bef cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // d1bf5 jg 0xd1c74
            if fl.g() { pc = 0xd1c74; continue; }
            pc = 0xd1bf7;
        }
        0xd1bf7 => {
            // block d1bf7
            // d1bf7 cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // d1bfd je 0xd1cc6
            if fl.e() { pc = 0xd1cc6; continue; }
            pc = 0xd1c03;
        }
        0xd1c03 => {
            // block d1c03
            // d1c03 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // d1c09 je 0xd1cc6
            if fl.e() { pc = 0xd1cc6; continue; }
            pc = 0xd1c0f;
        }
        0xd1c0f => {
            // block d1c0f
            // d1c0f jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1c14 => {
            // block d1c14
            // d1c14 cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // d1c1a jg 0xd1c8d
            if fl.g() { pc = 0xd1c8d; continue; }
            pc = 0xd1c1c;
        }
        0xd1c1c => {
            // block d1c1c
            // d1c1c lea eax,[rdi-0x10646700]
            rax = (rdi.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d1c22 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // d1c25 jb 0xd1d00
            if fl.b() { pc = 0xd1d00; continue; }
            pc = 0xd1c2b;
        }
        0xd1c2b => {
            // block d1c2b
            // d1c2b lea eax,[rdi-0x10646705]
            rax = (rdi.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d1c31 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1c34 jb 0xd1d00
            if fl.b() { pc = 0xd1d00; continue; }
            pc = 0xd1c3a;
        }
        0xd1c3a => {
            // block d1c3a
            // d1c3a cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // d1c40 je 0xd1d00
            if fl.e() { pc = 0xd1d00; continue; }
            pc = 0xd1c46;
        }
        0xd1c46 => {
            // block d1c46
            // d1c46 jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1c4b => {
            // block d1c4b
            // d1c4b cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // d1c51 jg 0xd1cde
            if fl.g() { pc = 0xd1cde; continue; }
            pc = 0xd1c57;
        }
        0xd1c57 => {
            // block d1c57
            // d1c57 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // d1c5d je 0xd1d00
            if fl.e() { pc = 0xd1d00; continue; }
            pc = 0xd1c63;
        }
        0xd1c63 => {
            // block d1c63
            // d1c63 cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // d1c69 je 0xd1d00
            if fl.e() { pc = 0xd1d00; continue; }
            pc = 0xd1c6f;
        }
        0xd1c6f => {
            // block d1c6f
            // d1c6f jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1c74 => {
            // block d1c74
            // d1c74 cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // d1c7a je 0xd1d00
            if fl.e() { pc = 0xd1d00; continue; }
            pc = 0xd1c80;
        }
        0xd1c80 => {
            // block d1c80
            // d1c80 cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // d1c86 je 0xd1cc6
            if fl.e() { pc = 0xd1cc6; continue; }
            pc = 0xd1c88;
        }
        0xd1c88 => {
            // block d1c88
            // d1c88 jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1c8d => {
            // block d1c8d
            // d1c8d lea eax,[rdi-0x10710850]
            rax = (rdi.wrapping_add(0xffff_ffff_ef8e_f7b0)) as u32 as u64;
            // d1c93 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1c96 jb 0xd1d00
            if fl.b() { pc = 0xd1d00; continue; }
            pc = 0xd1c98;
        }
        0xd1c98 => {
            // block d1c98
            // d1c98 cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // d1c9e je 0xd1d00
            if fl.e() { pc = 0xd1d00; continue; }
            pc = 0xd1ca0;
        }
        0xd1ca0 => {
            // block d1ca0
            // d1ca0 cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // d1ca6 jne 0xd1d37
            if fl.ne() { pc = 0xd1d37; continue; }
            pc = 0xd1cac;
        }
        0xd1cac => {
            // block d1cac
            // d1cac jmp 0xd1d00
            pc = 0xd1d00; continue;
        }
        0xd1cae => {
            // block d1cae
            // d1cae cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // d1cb4 jg 0xd1cf0
            if fl.g() { pc = 0xd1cf0; continue; }
            pc = 0xd1cb6;
        }
        0xd1cb6 => {
            // block d1cb6
            // d1cb6 cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // d1cbc je 0xd1d00
            if fl.e() { pc = 0xd1d00; continue; }
            pc = 0xd1cbe;
        }
        0xd1cbe => {
            // block d1cbe
            // d1cbe cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // d1cc4 jne 0xd1d37
            if fl.ne() { pc = 0xd1d37; continue; }
            pc = 0xd1cc6;
        }
        0xd1cc6 => {
            // block d1cc6
            // d1cc6 add esi,0xfffffff8
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffff8u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d1cc9 cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // d1ccc jae 0xd1d37
            if fl.ae() { pc = 0xd1d37; continue; }
            pc = 0xd1cce;
        }
        0xd1cce => {
            // block d1cce
            // d1cce movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // d1cd1 lea rcx,[rip+0x311d8] # 0x102eb0
            rcx = m.env.data_va(95869);
            // d1cd8 mov r8d,DWORD PTR [rcx+rax*4]
            r8 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // d1cdc jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1cde => {
            // block d1cde
            // d1cde cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // d1ce4 je 0xd1d00
            if fl.e() { pc = 0xd1d00; continue; }
            pc = 0xd1ce6;
        }
        0xd1ce6 => {
            // block d1ce6
            // d1ce6 cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // d1cec je 0xd1d00
            if fl.e() { pc = 0xd1d00; continue; }
            pc = 0xd1cee;
        }
        0xd1cee => {
            // block d1cee
            // d1cee jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1cf0 => {
            // block d1cf0
            // d1cf0 cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // d1cf6 je 0xd1d00
            if fl.e() { pc = 0xd1d00; continue; }
            pc = 0xd1cf8;
        }
        0xd1cf8 => {
            // block d1cf8
            // d1cf8 cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // d1cfe jne 0xd1d37
            if fl.ne() { pc = 0xd1d37; continue; }
            pc = 0xd1d00;
        }
        0xd1d00 => {
            // block d1d00
            // d1d00 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // d1d05 cmp esi,0x11
            fl = sub::<32>((rsi as u32 as u64), 0x11u64).1;
            // d1d08 ja 0xd1d25
            if fl.a() { pc = 0xd1d25; continue; }
            pc = 0xd1d0a;
        }
        0xd1d0a => {
            // block d1d0a
            // d1d0a mov ecx,0x23d30
            rcx = (0x23d30u64) as u32 as u64;
            // d1d0f bt ecx,esi
            fl.cf = bit::<32>((rcx as u32 as u64), (rsi as u32 as u64));
            // d1d12 jae 0xd1eac
            if fl.ae() { pc = 0xd1eac; continue; }
            pc = 0xd1d18;
        }
        0xd1d18 => {
            // block d1d18
            // d1d18 cmp edx,0x16
            fl = sub::<32>((rdx as u32 as u64), 0x16u64).1;
            // d1d1b sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d1d1e movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // d1d21 lea eax,[rax+rax*1+0x2]
            rax = (rax.wrapping_add(rax).wrapping_add(0x2)) as u32 as u64;
            pc = 0xd1d25;
        }
        0xd1d25 => {
            // block d1d25
            // d1d25 and esi,0xfffffffe
            { let r = (rsi as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d1d28 lea r8d,[rax*4+0x0]
            r8 = (rax.wrapping_mul(4)) as u32 as u64;
            // d1d30 cmp esi,0x10
            fl = sub::<32>((rsi as u32 as u64), 0x10u64).1;
            // d1d33 cmovne r8d,eax
            if fl.ne() { r8 = ((rax as u32 as u64)) as u32 as u64; } else { r8 = r8 as u32 as u64; }
            pc = 0xd1d37;
        }
        0xd1d37 => {
            // block d1d37
            // d1d37 cmp edx,0x10
            fl = sub::<32>((rdx as u32 as u64), 0x10u64).1;
            // d1d3a sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d1d3d shl r8d,cl
            { let (r, f) = shl::<32>((r8 as u32 as u64), (rcx as u8 as u64), fl); r8 = (r) as u32 as u64; fl = f; }
            // d1d40 mov eax,0x10
            rax = (0x10u64) as u32 as u64;
            // d1d45 cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // d1d4b jg 0xd1d75
            if fl.g() { pc = 0xd1d75; continue; }
            pc = 0xd1d4d;
        }
        0xd1d4d => {
            // block d1d4d
            // d1d4d cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // d1d53 jg 0xd1dc3
            if fl.g() { pc = 0xd1dc3; continue; }
            pc = 0xd1d55;
        }
        0xd1d55 => {
            // block d1d55
            // d1d55 lea eax,[rdi-0x10266400]
            rax = (rdi.wrapping_add(0xffff_ffff_efd9_9c00)) as u32 as u64;
            // d1d5b cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1d5e jb 0xd1e58
            if fl.b() { pc = 0xd1e58; continue; }
            pc = 0xd1d64;
        }
        0xd1d64 => {
            // block d1d64
            // d1d64 cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // d1d6a je 0xd1e58
            if fl.e() { pc = 0xd1e58; continue; }
            pc = 0xd1d70;
        }
        0xd1d70 => {
            // block d1d70
            // d1d70 jmp 0xd1f01
            pc = 0xd1f01; continue;
        }
        0xd1d75 => {
            // block d1d75
            // d1d75 cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // d1d7b jg 0xd1de8
            if fl.g() { pc = 0xd1de8; continue; }
            pc = 0xd1d7d;
        }
        0xd1d7d => {
            // block d1d7d
            // d1d7d cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // d1d83 jg 0xd1e1f
            if fl.g() { pc = 0xd1e1f; continue; }
            pc = 0xd1d89;
        }
        0xd1d89 => {
            // block d1d89
            // d1d89 cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // d1d8f jg 0xd1e83
            if fl.g() { pc = 0xd1e83; continue; }
            pc = 0xd1d95;
        }
        0xd1d95 => {
            // block d1d95
            // d1d95 cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // d1d9b je 0xd1e93
            if fl.e() { pc = 0xd1e93; continue; }
            pc = 0xd1da1;
        }
        0xd1da1 => {
            // block d1da1
            // d1da1 cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // d1da7 je 0xd1f03
            if fl.e() { pc = 0xd1f03; continue; }
            pc = 0xd1dad;
        }
        0xd1dad => {
            // block d1dad
            // d1dad cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // d1db3 jne 0xd1f01
            if fl.ne() { pc = 0xd1f01; continue; }
            pc = 0xd1db9;
        }
        0xd1db9 => {
            // block d1db9
            // d1db9 mov eax,0x18
            rax = (0x18u64) as u32 as u64;
            // d1dbe jmp 0xd1f03
            pc = 0xd1f03; continue;
        }
        0xd1dc3 => {
            // block d1dc3
            // d1dc3 cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // d1dc9 jg 0xd1e44
            if fl.g() { pc = 0xd1e44; continue; }
            pc = 0xd1dcb;
        }
        0xd1dcb => {
            // block d1dcb
            // d1dcb cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // d1dd1 je 0xd1f03
            if fl.e() { pc = 0xd1f03; continue; }
            pc = 0xd1dd7;
        }
        0xd1dd7 => {
            // block d1dd7
            // d1dd7 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // d1ddd jne 0xd1f01
            if fl.ne() { pc = 0xd1f01; continue; }
            pc = 0xd1de3;
        }
        0xd1de3 => {
            // block d1de3
            // d1de3 jmp 0xd1e93
            pc = 0xd1e93; continue;
        }
        0xd1de8 => {
            // block d1de8
            // d1de8 cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // d1dee jg 0xd1e62
            if fl.g() { pc = 0xd1e62; continue; }
            pc = 0xd1df0;
        }
        0xd1df0 => {
            // block d1df0
            // d1df0 lea eax,[rdi-0x10646700]
            rax = (rdi.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d1df6 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // d1df9 jb 0xd1ef2
            if fl.b() { pc = 0xd1ef2; continue; }
            pc = 0xd1dff;
        }
        0xd1dff => {
            // block d1dff
            // d1dff lea eax,[rdi-0x10646705]
            rax = (rdi.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d1e05 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1e08 jb 0xd1ef2
            if fl.b() { pc = 0xd1ef2; continue; }
            pc = 0xd1e0e;
        }
        0xd1e0e => {
            // block d1e0e
            // d1e0e cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // d1e14 je 0xd1ef2
            if fl.e() { pc = 0xd1ef2; continue; }
            pc = 0xd1e1a;
        }
        0xd1e1a => {
            // block d1e1a
            // d1e1a jmp 0xd1f01
            pc = 0xd1f01; continue;
        }
        0xd1e1f => {
            // block d1e1f
            // d1e1f cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // d1e25 jg 0xd1e9a
            if fl.g() { pc = 0xd1e9a; continue; }
            pc = 0xd1e27;
        }
        0xd1e27 => {
            // block d1e27
            // d1e27 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // d1e2d je 0xd1ef2
            if fl.e() { pc = 0xd1ef2; continue; }
            pc = 0xd1e33;
        }
        0xd1e33 => {
            // block d1e33
            // d1e33 cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // d1e39 jne 0xd1f01
            if fl.ne() { pc = 0xd1f01; continue; }
            pc = 0xd1e3f;
        }
        0xd1e3f => {
            // block d1e3f
            // d1e3f jmp 0xd1ef2
            pc = 0xd1ef2; continue;
        }
        0xd1e44 => {
            // block d1e44
            // d1e44 cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // d1e4a je 0xd1e79
            if fl.e() { pc = 0xd1e79; continue; }
            pc = 0xd1e4c;
        }
        0xd1e4c => {
            // block d1e4c
            // d1e4c cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // d1e52 jne 0xd1f01
            if fl.ne() { pc = 0xd1f01; continue; }
            pc = 0xd1e58;
        }
        0xd1e58 => {
            // block d1e58
            // d1e58 mov eax,0x20
            rax = (0x20u64) as u32 as u64;
            // d1e5d jmp 0xd1f03
            pc = 0xd1f03; continue;
        }
        0xd1e62 => {
            // block d1e62
            // d1e62 lea eax,[rdi-0x10710850]
            rax = (rdi.wrapping_add(0xffff_ffff_ef8e_f7b0)) as u32 as u64;
            // d1e68 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1e6b jb 0xd1ef2
            if fl.b() { pc = 0xd1ef2; continue; }
            pc = 0xd1e71;
        }
        0xd1e71 => {
            // block d1e71
            // d1e71 cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // d1e77 jne 0xd1eea
            if fl.ne() { pc = 0xd1eea; continue; }
            pc = 0xd1e79;
        }
        0xd1e79 => {
            // block d1e79
            // d1e79 mov eax,0x38
            rax = (0x38u64) as u32 as u64;
            // d1e7e jmp 0xd1f03
            pc = 0xd1f03; continue;
        }
        0xd1e83 => {
            // block d1e83
            // d1e83 cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // d1e89 jg 0xd1ed8
            if fl.g() { pc = 0xd1ed8; continue; }
            pc = 0xd1e8b;
        }
        0xd1e8b => {
            // block d1e8b
            // d1e8b cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // d1e91 jne 0xd1ef9
            if fl.ne() { pc = 0xd1ef9; continue; }
            pc = 0xd1e93;
        }
        0xd1e93 => {
            // block d1e93
            // d1e93 mov eax,0x40
            rax = (0x40u64) as u32 as u64;
            // d1e98 jmp 0xd1f03
            pc = 0xd1f03; continue;
        }
        0xd1e9a => {
            // block d1e9a
            // d1e9a cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // d1ea0 je 0xd1ef2
            if fl.e() { pc = 0xd1ef2; continue; }
            pc = 0xd1ea2;
        }
        0xd1ea2 => {
            // block d1ea2
            // d1ea2 cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // d1ea8 jne 0xd1f01
            if fl.ne() { pc = 0xd1f01; continue; }
            pc = 0xd1eaa;
        }
        0xd1eaa => {
            // block d1eaa
            // d1eaa jmp 0xd1ef2
            pc = 0xd1ef2; continue;
        }
        0xd1eac => {
            // block d1eac
            // d1eac mov ecx,0x10001
            rcx = (0x10001u64) as u32 as u64;
            // d1eb1 bt ecx,esi
            fl.cf = bit::<32>((rcx as u32 as u64), (rsi as u32 as u64));
            // d1eb4 jae 0xd1d25
            if fl.ae() { pc = 0xd1d25; continue; }
            pc = 0xd1eba;
        }
        0xd1eba => {
            // block d1eba
            // d1eba lea ecx,[rdx-0x10]
            rcx = (rdx.wrapping_add(0xffff_ffff_ffff_fff0)) as u32 as u64;
            // d1ebd cmp ecx,0x7
            fl = sub::<32>((rcx as u32 as u64), 0x7u64).1;
            // d1ec0 jae 0xd1d25
            if fl.ae() { pc = 0xd1d25; continue; }
            pc = 0xd1ec6;
        }
        0xd1ec6 => {
            // block d1ec6
            // d1ec6 movsxd rax,ecx
            rax = sx::<32>((rcx as u32 as u64)) as u64;
            // d1ec9 lea rcx,[rip+0x31000] # 0x102ed0
            rcx = m.env.data_va(95901);
            // d1ed0 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // d1ed3 jmp 0xd1d25
            pc = 0xd1d25; continue;
        }
        0xd1ed8 => {
            // block d1ed8
            // d1ed8 cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // d1ede je 0xd1ef2
            if fl.e() { pc = 0xd1ef2; continue; }
            pc = 0xd1ee0;
        }
        0xd1ee0 => {
            // block d1ee0
            // d1ee0 cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // d1ee6 jne 0xd1f01
            if fl.ne() { pc = 0xd1f01; continue; }
            pc = 0xd1ee8;
        }
        0xd1ee8 => {
            // block d1ee8
            // d1ee8 jmp 0xd1ef2
            pc = 0xd1ef2; continue;
        }
        0xd1eea => {
            // block d1eea
            // d1eea cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // d1ef0 jne 0xd1f01
            if fl.ne() { pc = 0xd1f01; continue; }
            pc = 0xd1ef2;
        }
        0xd1ef2 => {
            // block d1ef2
            // d1ef2 mov eax,0x6f
            rax = (0x6fu64) as u32 as u64;
            // d1ef7 jmp 0xd1f03
            pc = 0xd1f03; continue;
        }
        0xd1ef9 => {
            // block d1ef9
            // d1ef9 cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // d1eff je 0xd1f03
            if fl.e() { pc = 0xd1f03; continue; }
            pc = 0xd1f01;
        }
        0xd1f01 => {
            // block d1f01
            // d1f01 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd1f03;
        }
        0xd1f03 => {
            // block d1f03
            // d1f03 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d1f05 div r8d
            { let (q, r) = div::<32>(rdx, rax, (r8 as u32 as u64)); rax = q; rdx = r; }
            // d1f08 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d1f09 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe05da size=32
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e05da<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe05da;
    loop {
        match pc {
        0xe05da => {
            // block e05da
            // e05da push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e05db mov rbp,rsp
            rbp = rsp;
            // e05de lea rax,[rip+0x65be3] # 0x1461c8
            rax = m.env.data_va(371093);
            // e05e5 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // e05e8 mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            // e05ec test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e05ef je 0xe05f7
            if fl.e() { pc = 0xe05f7; continue; }
            pc = 0xe05f1;
        }
        0xe05f1 => {
            // block e05f1
            // e05f1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e05f2 jmp 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xe05f7 => {
            // block e05f7
            // e05f7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e05f8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe05fa size=32
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e05fa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe05fa;
    loop {
        match pc {
        0xe05fa => {
            // block e05fa
            // e05fa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e05fb mov rbp,rsp
            rbp = rsp;
            // e05fe lea rax,[rip+0x65bc3] # 0x1461c8
            rax = m.env.data_va(371093);
            // e0605 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // e0608 mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            // e060c test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e060f je 0xe0617
            if fl.e() { pc = 0xe0617; continue; }
            pc = 0xe0611;
        }
        0xe0611 => {
            // block e0611
            // e0611 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0612 jmp 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xe0617 => {
            // block e0617
            // e0617 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0618 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe07aa size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e07aa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe07aa;
    loop {
        match pc {
        0xe07aa => {
            // block e07aa
            // e07aa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e07ab mov rbp,rsp
            rbp = rsp;
            // e07ae push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e07af push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e07b0 mov rbx,rdi
            rbx = rdi;
            // e07b3 lea rax,[rip+0x65a2e] # 0x1461e8
            rax = m.env.data_va(371125);
            // e07ba mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e07bd mov rdi,QWORD PTR [rbx+0x20]
            rdi = m.r64(rbx.wrapping_add(0x20)) as u64;
            // e07c1 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e07c4 je 0xe07d3
            if fl.e() { pc = 0xe07d3; continue; }
            pc = 0xe07c6;
        }
        0xe07c6 => {
            // block e07c6
            // e07c6 call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e07cb mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0xe07d3;
        }
        0xe07d3 => {
            // block e07d3
            // e07d3 mov rdi,QWORD PTR [rbx+0x28]
            rdi = m.r64(rbx.wrapping_add(0x28)) as u64;
            // e07d7 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e07da je 0xe07e1
            if fl.e() { pc = 0xe07e1; continue; }
            pc = 0xe07dc;
        }
        0xe07dc => {
            // block e07dc
            // e07dc call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe07e1;
        }
        0xe07e1 => {
            // block e07e1
            // e07e1 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // e07e9 mov DWORD PTR [rbx+0x30],0x0
            m.w32(rbx.wrapping_add(0x30), (0x0u64) as u32);
            // e07f0 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // e07f4 mov rdi,rbx
            rdi = rbx;
            // e07f7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e07fb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e07fc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e07fd jmp 0xebeee
            { let r = { let t = m.env.stub_va(0x6bb); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c746 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c746<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5c746;
    loop {
        match pc {
        0x5c746 => {
            // block 5c746
            // 5c746 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c747 mov rbp,rsp
            rbp = rsp;
            // 5c74a mov eax,0x29
            rax = (0x29u64) as u32 as u64;
            // 5c74f cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // 5c752 ja 0x5c760
            if fl.a() { pc = 0x5c760; continue; }
            pc = 0x5c754;
        }
        0x5c754 => {
            // block 5c754
            // 5c754 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 5c756 lea rcx,[rip+0xa4123] # 0x100880
            rcx = m.env.data_va(86093);
            // 5c75d mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5c760;
        }
        0x5c760 => {
            // block 5c760
            // 5c760 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c761 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c7a8 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c7a8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5c7a8;
    loop {
        match pc {
        0x5c7a8 => {
            // block 5c7a8
            // 5c7a8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c7a9 mov rbp,rsp
            rbp = rsp;
            // 5c7ac mov eax,0xa
            rax = (0xau64) as u32 as u64;
            // 5c7b1 cmp edi,0x1a
            fl = sub::<32>((rdi as u32 as u64), 0x1au64).1;
            // 5c7b4 ja 0x5c7c2
            if fl.a() { pc = 0x5c7c2; continue; }
            pc = 0x5c7b6;
        }
        0x5c7b6 => {
            // block 5c7b6
            // 5c7b6 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 5c7b8 lea rcx,[rip+0xa4181] # 0x100940
            rcx = m.env.data_va(86285);
            // 5c7bf mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5c7c2;
        }
        0x5c7c2 => {
            // block 5c7c2
            // 5c7c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c7c3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c8a5 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c8a5<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5c8a5;
    loop {
        match pc {
        0x5c8a5 => {
            // block 5c8a5
            // 5c8a5 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c8a6 mov rbp,rsp
            rbp = rsp;
            // 5c8a9 mov eax,0xa
            rax = (0xau64) as u32 as u64;
            // 5c8ae cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // 5c8b1 ja 0x5c8bf
            if fl.a() { pc = 0x5c8bf; continue; }
            pc = 0x5c8b3;
        }
        0x5c8b3 => {
            // block 5c8b3
            // 5c8b3 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 5c8b5 lea rcx,[rip+0xa41c4] # 0x100a80
            rcx = m.env.data_va(86605);
            // 5c8bc mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5c8bf;
        }
        0x5c8bf => {
            // block 5c8bf
            // 5c8bf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c8c0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c988 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c988<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5c988;
    loop {
        match pc {
        0x5c988 => {
            // block 5c988
            // 5c988 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c989 mov rbp,rsp
            rbp = rsp;
            // 5c98c mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 5c991 cmp edi,0x9
            fl = sub::<32>((rdi as u32 as u64), 0x9u64).1;
            // 5c994 ja 0x5c9a2
            if fl.a() { pc = 0x5c9a2; continue; }
            pc = 0x5c996;
        }
        0x5c996 => {
            // block 5c996
            // 5c996 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 5c998 lea rcx,[rip+0xa4131] # 0x100ad0
            rcx = m.env.data_va(86685);
            // 5c99f mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5c9a2;
        }
        0x5c9a2 => {
            // block 5c9a2
            // 5c9a2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c9a3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbab1 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbab1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbab1;
    loop {
        match pc {
        0xbbab1 => {
            // block bbab1
            // bbab1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbab2 mov rbp,rsp
            rbp = rsp;
            // bbab5 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bbaba cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbabd ja 0xbbacb
            if fl.a() { pc = 0xbbacb; continue; }
            pc = 0xbbabf;
        }
        0xbbabf => {
            // block bbabf
            // bbabf mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbac1 lea rcx,[rip+0x47018] # 0x102ae0
            rcx = m.env.data_va(94893);
            // bbac8 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbacb;
        }
        0xbbacb => {
            // block bbacb
            // bbacb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbacc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c5e1 size=33
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c5e1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5c5e1;
    loop {
        match pc {
        0x5c5e1 => {
            // block 5c5e1
            // 5c5e1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c5e2 mov rbp,rsp
            rbp = rsp;
            // 5c5e5 dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 5c5e7 cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // 5c5ea ja 0x5c5fb
            if fl.a() { pc = 0x5c5fb; continue; }
            pc = 0x5c5ec;
        }
        0x5c5ec => {
            // block 5c5ec
            // 5c5ec movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // 5c5ef lea rcx,[rip+0xa548a] # 0x101a80
            rcx = m.env.data_va(90701);
            // 5c5f6 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5c5f9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c5fa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5c5fb => {
            // block 5c5fb
            // 5c5fb mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // 5c600 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c601 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0758 size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0758<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0758;
    loop {
        match pc {
        0xe0758 => {
            // block e0758
            // e0758 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0759 mov rbp,rsp
            rbp = rsp;
            // e075c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e075d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e075e mov rbx,rdi
            rbx = rdi;
            // e0761 lea rax,[rip+0x65a80] # 0x1461e8
            rax = m.env.data_va(371125);
            // e0768 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e076b mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // e0773 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // e077b mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // e0783 lea rdi,[rip+0x5853e] # 0x138cc8
            rdi = m.env.data_va(316565);
            // e078a call 0xec2ba
            { let r = { let t = m.env.stub_va(0xa87); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e078f mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // e0793 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // e079b mov DWORD PTR [rbx+0x30],0x0
            m.w32(rbx.wrapping_add(0x30), (0x0u64) as u32);
            // e07a2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e07a6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e07a7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e07a8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5094 size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5094<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5094;
    loop {
        match pc {
        0xb5094 => {
            // block b5094
            // b5094 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5095 mov rbp,rsp
            rbp = rsp;
            // b5098 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b509a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b509c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b509e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b509f mov r15,rdi
            r15 = rdi;
            // b50a2 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b50a7 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // b50ab test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // b50ad jne 0xb50bd
            if fl.ne() { pc = 0xb50bd; continue; }
            pc = 0xb50af;
        }
        0xb50af => {
            // block b50af
            // b50af lea r12,[r15+0x1]
            r12 = r15.wrapping_add(0x1);
            // b50b3 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // b50b6 lea r14,[r15+rax*1+0x1]
            r14 = r15.wrapping_add(rax).wrapping_add(0x1);
            // b50bb jmp 0xb50c8
            pc = 0xb50c8; continue;
        }
        0xb50bd => {
            // block b50bd
            // b50bd mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // b50c1 mov r14,QWORD PTR [r15+0x8]
            r14 = m.r64(r15.wrapping_add(0x8)) as u64;
            // b50c5 add r14,r12
            { let (r, f) = add::<64>(r14, r12);
              r14 = r;
              fl = f; }
            pc = 0xb50c8;
        }
        0xb50c8 => {
            // block b50c8
            // b50c8 mov rbx,r12
            rbx = r12;
            // b50cb cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // b50ce je 0xb50e7
            if fl.e() { pc = 0xb50e7; continue; }
            pc = 0xb50d0;
        }
        0xb50d0 => {
            // block b50d0
            // b50d0 movsx edi,BYTE PTR [r12]
            rdi = (sx::<8>(m.r8(r12) as u64) as u64) as u32 as u64;
            // b50d5 call 0xec19a
            { let r = { let t = m.env.stub_va(0x967); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b50da mov BYTE PTR [rbx],al
            m.w8(rbx, ((rax as u8 as u64)) as u8);
            // b50dc inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // b50df inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // b50e2 cmp r14,r12
            fl = sub::<64>(r14, r12).1;
            // b50e5 jne 0xb50d0
            if fl.ne() { pc = 0xb50d0; continue; }
            pc = 0xb50e7;
        }
        0xb50e7 => {
            // block b50e7
            // b50e7 mov rax,r15
            rax = r15;
            // b50ea pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b50eb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b50ed pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b50ef pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b50f1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b50f2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb50f3 => {
            // block b50f3
            // b50f3 mov r14,rax
            r14 = rax;
            // b50f6 mov rdi,r15
            rdi = r15;
            // b50f9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b50fe mov rdi,r14
            rdi = r14;
            // b5101 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd81de size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d81de<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd81de;
    loop {
        match pc {
        0xd81de => {
            // block d81de
            // d81de push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d81df mov rbp,rsp
            rbp = rsp;
            // d81e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d81e3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d81e4 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // d81e6 mov rbx,rdi
            rbx = rdi;
            // d81e9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d81ec mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d81f1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d81f6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d81f9 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d81fc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d81ff mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8204 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d8209 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d820e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8211 mov rdi,rbx
            rdi = rbx;
            // d8214 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8217 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d821a mov rdi,rbx
            rdi = rbx;
            // d821d call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8223 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8226 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d822b mov edx,0xd8
            rdx = (0xd8u64) as u32 as u64;
            // d8230 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8235 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8238 mov rdi,rbx
            rdi = rbx;
            // d823b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d823e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8241 mov rdi,rbx
            rdi = rbx;
            // d8244 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d8248 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d8249 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d824a jmp QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4a4c size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4a4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd4a4c;
    loop {
        match pc {
        0xd4a4c => {
            // block d4a4c
            // d4a4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4a4d mov rbp,rsp
            rbp = rsp;
            // d4a50 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4a51 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d4a52 mov rbx,rdi
            rbx = rdi;
            // d4a55 lea rax,[rip+0x6f7fc] # 0x144258
            rax = m.env.data_va(363045);
            // d4a5c mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // d4a5f mov WORD PTR [rbx+0x8],0x0
            m.w16(rbx.wrapping_add(0x8), (0x0u64) as u16);
            // d4a65 mov BYTE PTR [rbx+0xa],0x0
            m.w8(rbx.wrapping_add(0xa), (0x0u64) as u8);
            // d4a69 mov DWORD PTR [rbx+0xc],0x100
            m.w32(rbx.wrapping_add(0xc), (0x100u64) as u32);
            // d4a70 mov DWORD PTR [rbx+0x10],0xffffffff
            m.w32(rbx.wrapping_add(0x10), (0xffffffffu64) as u32);
            // d4a77 mov BYTE PTR [rbx+0x14],0x0
            m.w8(rbx.wrapping_add(0x14), (0x0u64) as u8);
            // d4a7b lea rdi,[rbx+0x18]
            rdi = rbx.wrapping_add(0x18);
            // d4a7f mov esi,0x1f4
            rsi = (0x1f4u64) as u32 as u64;
            // d4a84 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4a89 mov DWORD PTR [rbx+0x230],0x0
            m.w32(rbx.wrapping_add(0x230), (0x0u64) as u32);
            // d4a93 mov QWORD PTR [rbx+0x228],0x0
            m.w64(rbx.wrapping_add(0x228), (0x0u64) as u64);
            // d4a9e mov QWORD PTR [rbx+0x220],0x0
            m.w64(rbx.wrapping_add(0x220), (0x0u64) as u64);
            // d4aa9 mov QWORD PTR [rbx+0x218],0x0
            m.w64(rbx.wrapping_add(0x218), (0x0u64) as u64);
            // d4ab4 mov QWORD PTR [rbx+0x210],0x0
            m.w64(rbx.wrapping_add(0x210), (0x0u64) as u64);
            // d4abf mov QWORD PTR [rbx+0x248],0x0
            m.w64(rbx.wrapping_add(0x248), (0x0u64) as u64);
            // d4aca mov QWORD PTR [rbx+0x240],0x0
            m.w64(rbx.wrapping_add(0x240), (0x0u64) as u64);
            // d4ad5 mov QWORD PTR [rbx+0x238],0x0
            m.w64(rbx.wrapping_add(0x238), (0x0u64) as u64);
            // d4ae0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d4ae4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4ae5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4ae6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdc1f4 size=476
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dc1f4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdc1f4;
    loop {
        match pc {
        0xdc1f4 => {
            // block dc1f4
            // dc1f4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc1f5 mov rbp,rsp
            rbp = rsp;
            // dc1f8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dc1fa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dc1fc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dc1fe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dc1ff sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // dc203 mov r15,rdi
            r15 = rdi;
            // dc206 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dc209 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // dc20b xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dc20d call QWORD PTR [rax+0x18e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x18e8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc213 lea r14,[r15+0x258]
            r14 = r15.wrapping_add(0x258);
            // dc21a lea r12,[rip+0xe157f] # 0x1bd7a0
            r12 = m.env.data_va(860013);
            pc = 0xdc221;
        }
        0xdc221 => {
            // block dc221
            // dc221 mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc228 cmp rax,QWORD PTR [r15+0x268]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc22f je 0xdc242
            if fl.e() { pc = 0xdc242; continue; }
            pc = 0xdc231;
        }
        0xdc231 => {
            // block dc231
            // dc231 mov rcx,QWORD PTR [rbx+r12*1]
            rcx = m.r64(rbx.wrapping_add(r12)) as u64;
            // dc235 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dc238 add QWORD PTR [r15+0x260],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x260)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x260), (r) as u64);
              fl = f; }
            // dc240 jmp 0xdc24e
            pc = 0xdc24e; continue;
        }
        0xdc242 => {
            // block dc242
            // dc242 lea rsi,[r12+rbx*1]
            rsi = r12.wrapping_add(rbx);
            // dc246 mov rdi,r14
            rdi = r14;
            // dc249 call 0xebee2
            { let r = { let t = m.env.stub_va(0x6af); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdc24e;
        }
        0xdc24e => {
            // block dc24e
            // dc24e add rbx,0x608
            { let (r, f) = add::<64>(rbx, 0x608u64);
              rbx = r;
              fl = f; }
            // dc255 cmp rbx,0x3040
            fl = sub::<64>(rbx, 0x3040u64).1;
            // dc25c jne 0xdc221
            if fl.ne() { pc = 0xdc221; continue; }
            pc = 0xdc25e;
        }
        0xdc25e => {
            // block dc25e
            // dc25e lea rax,[rip+0x50ecf] # 0x12d134
            rax = m.env.data_va(268545);
            // dc265 mov QWORD PTR [rbp-0x28],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rax) as u64);
            // dc269 mov rcx,QWORD PTR [r15+0x260]
            rcx = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc270 cmp rcx,QWORD PTR [r15+0x268]
            fl = sub::<64>(rcx, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc277 jae 0xdc290
            if fl.ae() { pc = 0xdc290; continue; }
            pc = 0xdc279;
        }
        0xdc279 => {
            // block dc279
            // dc279 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // dc27c mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc283 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // dc287 mov QWORD PTR [r15+0x260],rax
            m.w64(r15.wrapping_add(0x260), (rax) as u64);
            // dc28e jmp 0xdc2a3
            pc = 0xdc2a3; continue;
        }
        0xdc290 => {
            // block dc290
            // dc290 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // dc294 mov rdi,r14
            rdi = r14;
            // dc297 call 0xebee8
            { let r = { let t = m.env.stub_va(0x6b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc29c mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            pc = 0xdc2a3;
        }
        0xdc2a3 => {
            // block dc2a3
            // dc2a3 lea rcx,[rip+0x5cf78] # 0x139222
            rcx = m.env.data_va(317935);
            // dc2aa mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // dc2ae cmp rax,QWORD PTR [r15+0x268]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc2b5 jae 0xdc2ce
            if fl.ae() { pc = 0xdc2ce; continue; }
            pc = 0xdc2b7;
        }
        0xdc2b7 => {
            // block dc2b7
            // dc2b7 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dc2ba mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc2c1 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // dc2c5 mov QWORD PTR [r15+0x260],rax
            m.w64(r15.wrapping_add(0x260), (rax) as u64);
            // dc2cc jmp 0xdc2e1
            pc = 0xdc2e1; continue;
        }
        0xdc2ce => {
            // block dc2ce
            // dc2ce lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // dc2d2 mov rdi,r14
            rdi = r14;
            // dc2d5 call 0xebee8
            { let r = { let t = m.env.stub_va(0x6b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc2da mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            pc = 0xdc2e1;
        }
        0xdc2e1 => {
            // block dc2e1
            // dc2e1 lea rcx,[rip+0x5cf41] # 0x139229
            rcx = m.env.data_va(317942);
            // dc2e8 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // dc2ec cmp rax,QWORD PTR [r15+0x268]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc2f3 jae 0xdc30c
            if fl.ae() { pc = 0xdc30c; continue; }
            pc = 0xdc2f5;
        }
        0xdc2f5 => {
            // block dc2f5
            // dc2f5 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dc2f8 mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc2ff add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // dc303 mov QWORD PTR [r15+0x260],rax
            m.w64(r15.wrapping_add(0x260), (rax) as u64);
            // dc30a jmp 0xdc31f
            pc = 0xdc31f; continue;
        }
        0xdc30c => {
            // block dc30c
            // dc30c lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // dc310 mov rdi,r14
            rdi = r14;
            // dc313 call 0xebee8
            { let r = { let t = m.env.stub_va(0x6b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc318 mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            pc = 0xdc31f;
        }
        0xdc31f => {
            // block dc31f
            // dc31f lea rcx,[rip+0x5cf0e] # 0x139234
            rcx = m.env.data_va(317953);
            // dc326 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // dc32a cmp rax,QWORD PTR [r15+0x268]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc331 jae 0xdc34a
            if fl.ae() { pc = 0xdc34a; continue; }
            pc = 0xdc333;
        }
        0xdc333 => {
            // block dc333
            // dc333 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dc336 mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            // dc33d add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // dc341 mov QWORD PTR [r15+0x260],rax
            m.w64(r15.wrapping_add(0x260), (rax) as u64);
            // dc348 jmp 0xdc35d
            pc = 0xdc35d; continue;
        }
        0xdc34a => {
            // block dc34a
            // dc34a lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // dc34e mov rdi,r14
            rdi = r14;
            // dc351 call 0xebee8
            { let r = { let t = m.env.stub_va(0x6b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc356 mov rax,QWORD PTR [r15+0x260]
            rax = m.r64(r15.wrapping_add(0x260)) as u64;
            pc = 0xdc35d;
        }
        0xdc35d => {
            // block dc35d
            // dc35d lea rcx,[rip+0x5cedf] # 0x139243
            rcx = m.env.data_va(317968);
            // dc364 mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // dc368 cmp rax,QWORD PTR [r15+0x268]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x268)) as u64).1;
            // dc36f jae 0xdc37e
            if fl.ae() { pc = 0xdc37e; continue; }
            pc = 0xdc371;
        }
        0xdc371 => {
            // block dc371
            // dc371 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // dc374 add QWORD PTR [r15+0x260],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x260)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x260), (r) as u64);
              fl = f; }
            // dc37c jmp 0xdc38a
            pc = 0xdc38a; continue;
        }
        0xdc37e => {
            // block dc37e
            // dc37e lea rsi,[rbp-0x48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // dc382 mov rdi,r14
            rdi = r14;
            // dc385 call 0xebee8
            { let r = { let t = m.env.stub_va(0x6b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdc38a;
        }
        0xdc38a => {
            // block dc38a
            // dc38a mov WORD PTR [r15+0x270],0x7
            m.w16(r15.wrapping_add(0x270), (0x7u64) as u16);
            // dc394 mov DWORD PTR [r15+0x274],0x0
            m.w32(r15.wrapping_add(0x274), (0x0u64) as u32);
            // dc39f mov BYTE PTR [r15+0x278],0x0
            m.w8(r15.wrapping_add(0x278), (0x0u64) as u8);
            // dc3a7 mov BYTE PTR [r15+0x279],0x1
            m.w8(r15.wrapping_add(0x279), (0x1u64) as u8);
            // dc3af mov BYTE PTR [r15+0x27b],0x1
            m.w8(r15.wrapping_add(0x27b), (0x1u64) as u8);
            // dc3b7 mov QWORD PTR [r15+0x280],0x0
            m.w64(r15.wrapping_add(0x280), (0x0u64) as u64);
            // dc3c2 add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // dc3c6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc3c7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dc3c9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc3cb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dc3cd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc3ce ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6722 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6722<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb6722;
    loop {
        match pc {
        0xb6722 => {
            // block b6722
            // b6722 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6723 mov rbp,rsp
            rbp = rsp;
            // b6726 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6727 jmp 0xb66ac
            { let r = { let t = m.env.code_va(0xb66ac); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb68ee size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b68ee<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb68ee;
    loop {
        match pc {
        0xb68ee => {
            // block b68ee
            // b68ee push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b68ef mov rbp,rsp
            rbp = rsp;
            // b68f2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b68f3 jmp 0xb6894
            { let r = { let t = m.env.code_va(0xb6894); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd680a size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d680a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd680a;
    loop {
        match pc {
        0xd680a => {
            // block d680a
            // d680a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d680b mov rbp,rsp
            rbp = rsp;
            // d680e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d680f jmp 0xd668c
            { let r = { let t = m.env.code_va(0xd668c); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6d4e size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6d4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd6d4e;
    loop {
        match pc {
        0xd6d4e => {
            // block d6d4e
            // d6d4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6d4f mov rbp,rsp
            rbp = rsp;
            // d6d52 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6d53 jmp 0xd6c84
            { let r = { let t = m.env.code_va(0xd6c84); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0802 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0802<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0802;
    loop {
        match pc {
        0xe0802 => {
            // block e0802
            // e0802 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0803 mov rbp,rsp
            rbp = rsp;
            // e0806 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0807 jmp 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe13bc size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e13bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe13bc;
    loop {
        match pc {
        0xe13bc => {
            // block e13bc
            // e13bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e13bd mov rbp,rsp
            rbp = rsp;
            // e13c0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e13c1 jmp 0xe42ac
            { let r = { let t = m.env.code_va(0xe42ac); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe13c6 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e13c6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe13c6;
    loop {
        match pc {
        0xe13c6 => {
            // block e13c6
            // e13c6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e13c7 mov rbp,rsp
            rbp = rsp;
            // e13ca pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e13cb jmp 0xe42ac
            { let r = { let t = m.env.code_va(0xe42ac); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
