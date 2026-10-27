//! Mechanically lifted functions (scripts/lift_run.py), module v007. Each one is checked against the original
//! in the world harness (tests/world_v007.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x19c32 size=294
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00019c32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x19c32;
    loop {
        match pc {
        0x19c32 => {
            // block 19c32
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19c32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19c32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19c33 mov rbp,rsp
            rbp = rsp;
            // 19c36 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19c38 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19c3a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 19c3c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 19c3e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19c3f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19c40 mov r12,rdx
            r12 = rdx;
            // 19c43 mov r15,rdi
            r15 = rdi;
            // 19c46 mov r14,r12
            r14 = r12;
            // 19c49 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 19c4c mov rbx,r14
            rbx = r14;
            // 19c4f sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19c53 mov r13,QWORD PTR [r15]
            r13 = m.r64(r15) as u64;
            // 19c56 mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19c5a sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19c5d sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19c61 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19c64 jbe 0x19cab
            if fl.be() { pc = 0x19cab; continue; }
            pc = 0x19c66;
        }
        0x19c66 => {
            // block 19c66
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19c66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19c66 mov r12,rsi
            r12 = rsi;
            // 19c69 mov rdi,r15
            rdi = r15;
            // 19c6c call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19c71 mov rax,rbx
            rax = rbx;
            // 19c74 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19c78 je 0x19c82
            if fl.e() { pc = 0x19c82; continue; }
            pc = 0x19c7a;
        }
        0x19c7a => {
            // block 19c7a
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19c7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19c7a mov rdi,r15
            rdi = r15;
            // 19c7d call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x19c82 => {
            // block 19c82
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19c82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19c82 mov rsi,QWORD PTR [r15+0x10]
            rsi = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19c86 sub rsi,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rsi, m.r64(r15) as u64);
              rsi = r;
              fl = f; }
            // 19c89 mov rax,rsi
            rax = rsi;
            // 19c8c sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19c90 movabs rcx,0x1ffffffffffffffe
            rcx = 0x1fff_ffff_ffff_fffeu64;
            // 19c9a cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 19c9d ja 0x19cc4
            if fl.a() { pc = 0x19cc4; continue; }
            pc = 0x19c9f;
        }
        0x19c9f => {
            // block 19c9f
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19c9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19c9f sar rsi,1
            { let (r, f) = sar::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 19ca2 cmp rsi,rbx
            fl = sub::<64>(rsi, rbx).1;
            // 19ca5 cmovb rsi,rbx
            if fl.b() { rsi = rbx; }
            // 19ca9 jmp 0x19cce
            pc = 0x19cce; continue;
        }
        0x19cab => {
            // block 19cab
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19cab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19cab mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19caf sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19cb2 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19cb6 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19cb9 jbe 0x19ceb
            if fl.be() { pc = 0x19ceb; continue; }
            pc = 0x19cbb;
        }
        0x19cbb => {
            // block 19cbb
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19cbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19cbb lea rbx,[rsi+rax*4]
            rbx = rsi.wrapping_add(rax.wrapping_mul(4));
            // 19cbf mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 19cc2 jmp 0x19cf1
            pc = 0x19cf1; continue;
        }
        0x19cc4 => {
            // block 19cc4
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19cc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19cc4 movabs rsi,0x3fffffffffffffff
            rsi = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x19cce;
        }
        0x19cce => {
            // block 19cce
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19cce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19cce mov rdi,r15
            rdi = r15;
            // 19cd1 call 0xeba68
            { let r = { let t = m.env.stub_va(0x235); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19cd6 mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19cda mov rsi,r12
            rsi = r12;
            // 19cdd mov rdx,r14
            rdx = r14;
            // 19ce0 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19ce5 shl rbx,0x2
            { let (r, f) = shl::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19ce9 jmp 0x19d22
            pc = 0x19d22; continue;
        }
        0x19ceb => {
            // block 19ceb
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19ceb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ceb xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 19cee mov rbx,r12
            rbx = r12;
            pc = 0x19cf1;
        }
        0x19cf1 => {
            // block 19cf1
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19cf1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19cf1 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 19cf5 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 19cf8 mov rdi,r13
            rdi = r13;
            // 19cfb mov rdx,rbx
            rdx = rbx;
            // 19cfe call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19d03 mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19d07 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 19d0a je 0x19d28
            if fl.e() { pc = 0x19d28; continue; }
            pc = 0x19d0c;
        }
        0x19d0c => {
            // block 19d0c
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19d0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19d0c mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 19d10 sub r12,rsi
            { let (r, f) = sub::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 19d13 mov rbx,r12
            rbx = r12;
            // 19d16 and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19d1a mov rdx,r12
            rdx = r12;
            // 19d1d call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x19d22;
        }
        0x19d22 => {
            // block 19d22
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19d22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19d22 add QWORD PTR [r15+0x8],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rbx);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 19d26 jmp 0x19d49
            pc = 0x19d49; continue;
        }
        0x19d28 => {
            // block 19d28
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19d28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19d28 and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19d2c add r13,rbx
            { let (r, f) = add::<64>(r13, rbx);
              r13 = r;
              fl = f; }
            // 19d2f cmp rdi,r13
            fl = sub::<64>(rdi, r13).1;
            // 19d32 je 0x19d49
            if fl.e() { pc = 0x19d49; continue; }
            pc = 0x19d34;
        }
        0x19d34 => {
            // block 19d34
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19d34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19d34 lea rax,[rdi-0x4]
            rax = rdi.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19d38 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19d3b not rax
            { let v = !rax; rax = v; }
            // 19d3e and rax,0xfffffffffffffffc
            { let r = rax & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 19d42 add rax,rdi
            { let (r, f) = add::<64>(rax, rdi);
              rax = r;
              fl = f; }
            // 19d45 mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            pc = 0x19d49;
        }
        0x19d49 => {
            // block 19d49
            #[cfg(feature = "trace")] eprintln!("f_00019c32 pc 19d49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19d49 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19d4d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19d4e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 19d50 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 19d52 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19d54 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 19d56 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19d57 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19fa2 size=294
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00019fa2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x19fa2;
    loop {
        match pc {
        0x19fa2 => {
            // block 19fa2
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 19fa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19fa2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19fa3 mov rbp,rsp
            rbp = rsp;
            // 19fa6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19fa8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19faa push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 19fac push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 19fae push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19faf push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19fb0 mov r12,rdx
            r12 = rdx;
            // 19fb3 mov r15,rdi
            r15 = rdi;
            // 19fb6 mov r14,r12
            r14 = r12;
            // 19fb9 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 19fbc mov rbx,r14
            rbx = r14;
            // 19fbf sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19fc3 mov r13,QWORD PTR [r15]
            r13 = m.r64(r15) as u64;
            // 19fc6 mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19fca sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19fcd sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19fd1 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19fd4 jbe 0x1a01b
            if fl.be() { pc = 0x1a01b; continue; }
            pc = 0x19fd6;
        }
        0x19fd6 => {
            // block 19fd6
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 19fd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19fd6 mov r12,rsi
            r12 = rsi;
            // 19fd9 mov rdi,r15
            rdi = r15;
            // 19fdc call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19fe1 mov rax,rbx
            rax = rbx;
            // 19fe4 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19fe8 je 0x19ff2
            if fl.e() { pc = 0x19ff2; continue; }
            pc = 0x19fea;
        }
        0x19fea => {
            // block 19fea
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 19fea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19fea mov rdi,r15
            rdi = r15;
            // 19fed call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x19ff2 => {
            // block 19ff2
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 19ff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ff2 mov rsi,QWORD PTR [r15+0x10]
            rsi = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19ff6 sub rsi,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rsi, m.r64(r15) as u64);
              rsi = r;
              fl = f; }
            // 19ff9 mov rax,rsi
            rax = rsi;
            // 19ffc sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 1a000 movabs rcx,0x1ffffffffffffffe
            rcx = 0x1fff_ffff_ffff_fffeu64;
            // 1a00a cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 1a00d ja 0x1a034
            if fl.a() { pc = 0x1a034; continue; }
            pc = 0x1a00f;
        }
        0x1a00f => {
            // block 1a00f
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a00f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a00f sar rsi,1
            { let (r, f) = sar::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 1a012 cmp rsi,rbx
            fl = sub::<64>(rsi, rbx).1;
            // 1a015 cmovb rsi,rbx
            if fl.b() { rsi = rbx; }
            // 1a019 jmp 0x1a03e
            pc = 0x1a03e; continue;
        }
        0x1a01b => {
            // block 1a01b
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a01b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a01b mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1a01f sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 1a022 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 1a026 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 1a029 jbe 0x1a05b
            if fl.be() { pc = 0x1a05b; continue; }
            pc = 0x1a02b;
        }
        0x1a02b => {
            // block 1a02b
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a02b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a02b lea rbx,[rsi+rax*4]
            rbx = rsi.wrapping_add(rax.wrapping_mul(4));
            // 1a02f mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 1a032 jmp 0x1a061
            pc = 0x1a061; continue;
        }
        0x1a034 => {
            // block 1a034
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a034 movabs rsi,0x3fffffffffffffff
            rsi = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1a03e;
        }
        0x1a03e => {
            // block 1a03e
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a03e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a03e mov rdi,r15
            rdi = r15;
            // 1a041 call 0xeb9fc
            { let r = { let t = m.env.stub_va(0x1c9); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a046 mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1a04a mov rsi,r12
            rsi = r12;
            // 1a04d mov rdx,r14
            rdx = r14;
            // 1a050 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a055 shl rbx,0x2
            { let (r, f) = shl::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1a059 jmp 0x1a092
            pc = 0x1a092; continue;
        }
        0x1a05b => {
            // block 1a05b
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a05b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a05b xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 1a05e mov rbx,r12
            rbx = r12;
            pc = 0x1a061;
        }
        0x1a061 => {
            // block 1a061
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a061 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a061 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 1a065 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1a068 mov rdi,r13
            rdi = r13;
            // 1a06b mov rdx,rbx
            rdx = rbx;
            // 1a06e call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a073 mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1a077 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 1a07a je 0x1a098
            if fl.e() { pc = 0x1a098; continue; }
            pc = 0x1a07c;
        }
        0x1a07c => {
            // block 1a07c
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a07c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a07c mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 1a080 sub r12,rsi
            { let (r, f) = sub::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 1a083 mov rbx,r12
            rbx = r12;
            // 1a086 and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 1a08a mov rdx,r12
            rdx = r12;
            // 1a08d call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1a092;
        }
        0x1a092 => {
            // block 1a092
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a092 add QWORD PTR [r15+0x8],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rbx);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 1a096 jmp 0x1a0b9
            pc = 0x1a0b9; continue;
        }
        0x1a098 => {
            // block 1a098
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a098 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a098 and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 1a09c add r13,rbx
            { let (r, f) = add::<64>(r13, rbx);
              r13 = r;
              fl = f; }
            // 1a09f cmp rdi,r13
            fl = sub::<64>(rdi, r13).1;
            // 1a0a2 je 0x1a0b9
            if fl.e() { pc = 0x1a0b9; continue; }
            pc = 0x1a0a4;
        }
        0x1a0a4 => {
            // block 1a0a4
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a0a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a0a4 lea rax,[rdi-0x4]
            rax = rdi.wrapping_add(0xffff_ffff_ffff_fffc);
            // 1a0a8 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 1a0ab not rax
            { let v = !rax; rax = v; }
            // 1a0ae and rax,0xfffffffffffffffc
            { let r = rax & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 1a0b2 add rax,rdi
            { let (r, f) = add::<64>(rax, rdi);
              rax = r;
              fl = f; }
            // 1a0b5 mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            pc = 0x1a0b9;
        }
        0x1a0b9 => {
            // block 1a0b9
            #[cfg(feature = "trace")] eprintln!("f_00019fa2 pc 1a0b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a0b9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1a0bd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a0be pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1a0c0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1a0c2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a0c4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1a0c6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a0c7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x320e2 size=302
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000320e2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x320e2;
    loop {
        match pc {
        0x320e2 => {
            // block 320e2
            #[cfg(feature = "trace")] eprintln!("f_000320e2 pc 320e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 320e2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 320e3 mov rbp,rsp
            rbp = rsp;
            // 320e6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 320e8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 320ea push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 320ec push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 320ee push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 320ef sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 320f3 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 320f7 mov r13,rdi
            r13 = rdi;
            // 320fa mov rsi,QWORD PTR [r13+0x0]
            rsi = m.r64(r13) as u64;
            // 320fe mov rbx,QWORD PTR [r13+0x8]
            rbx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 32102 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 32105 sar rbx,0x4
            { let (r, f) = sar::<64>(rbx, 0x4u64, fl); rbx = r; fl = f; }
            // 32109 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 3210c mov rax,rbx
            rax = rbx;
            // 3210f shr rax,0x3c
            { let (r, f) = shr::<64>(rax, 0x3cu64, fl); rax = r; fl = f; }
            // 32113 je 0x32121
            if fl.e() { pc = 0x32121; continue; }
            pc = 0x32115;
        }
        0x32115 => {
            // block 32115
            #[cfg(feature = "trace")] eprintln!("f_000320e2 pc 32115 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32115 mov rdi,r13
            rdi = r13;
            // 32118 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x32121 => {
            // block 32121
            #[cfg(feature = "trace")] eprintln!("f_000320e2 pc 32121 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32121 lea rax,[r13+0x10]
            rax = r13.wrapping_add(0x10);
            // 32125 mov r15,QWORD PTR [r13+0x10]
            r15 = m.r64(r13.wrapping_add(0x10)) as u64;
            // 32129 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 3212d sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 32130 mov rcx,r15
            rcx = r15;
            // 32133 sar rcx,0x4
            { let (r, f) = sar::<64>(rcx, 0x4u64, fl); rcx = r; fl = f; }
            // 32137 movabs rdx,0x7fffffffffffffe
            rdx = 0x7ff_ffff_ffff_fffeu64;
            // 32141 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 32144 ja 0x32153
            if fl.a() { pc = 0x32153; continue; }
            pc = 0x32146;
        }
        0x32146 => {
            // block 32146
            #[cfg(feature = "trace")] eprintln!("f_000320e2 pc 32146 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32146 sar r15,0x3
            { let (r, f) = sar::<64>(r15, 0x3u64, fl); r15 = r; fl = f; }
            // 3214a cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 3214d cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 32151 jmp 0x3215d
            pc = 0x3215d; continue;
        }
        0x32153 => {
            // block 32153
            #[cfg(feature = "trace")] eprintln!("f_000320e2 pc 32153 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32153 movabs r15,0xfffffffffffffff
            r15 = 0xfff_ffff_ffff_ffffu64;
            pc = 0x3215d;
        }
        0x3215d => {
            // block 3215d
            #[cfg(feature = "trace")] eprintln!("f_000320e2 pc 3215d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3215d mov r14,QWORD PTR [r13+0x8]
            r14 = m.r64(r13.wrapping_add(0x8)) as u64;
            // 32161 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 32165 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 32168 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 3216c mov r12,r14
            r12 = r14;
            // 3216f sar r12,0x4
            { let (r, f) = sar::<64>(r12, 0x4u64, fl); r12 = r; fl = f; }
            // 32173 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 3217b mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 3217f xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 32181 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 32184 je 0x32195
            if fl.e() { pc = 0x32195; continue; }
            pc = 0x32186;
        }
        0x32186 => {
            // block 32186
            #[cfg(feature = "trace")] eprintln!("f_000320e2 pc 32186 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32186 mov rdi,r15
            rdi = r15;
            // 32189 shl rdi,0x4
            { let (r, f) = shl::<64>(rdi, 0x4u64, fl); rdi = r; fl = f; }
            // 3218d call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32192 mov rbx,rax
            rbx = rax;
            pc = 0x32195;
        }
        0x32195 => {
            // block 32195
            #[cfg(feature = "trace")] eprintln!("f_000320e2 pc 32195 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32195 shl r12,0x4
            { let (r, f) = shl::<64>(r12, 0x4u64, fl); r12 = r; fl = f; }
            // 32199 shl r15,0x4
            { let (r, f) = shl::<64>(r15, 0x4u64, fl); r15 = r; fl = f; }
            // 3219d add r15,rbx
            { let (r, f) = add::<64>(r15, rbx);
              r15 = r;
              fl = f; }
            // 321a0 mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 321a4 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 321a7 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 321ab mov QWORD PTR [rbx+r12*1+0x8],rcx
            m.w64(rbx.wrapping_add(r12).wrapping_add(0x8), (rcx) as u64);
            // 321b0 mov QWORD PTR [rbx+r12*1],rax
            m.w64(rbx.wrapping_add(r12), (rax) as u64);
            // 321b4 lea rax,[rbx+r12*1+0x10]
            rax = rbx.wrapping_add(r12).wrapping_add(0x10);
            // 321b9 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 321bd mov rdi,rbx
            rdi = rbx;
            // 321c0 mov r12,QWORD PTR [rbp-0x58]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 321c4 mov rsi,r12
            rsi = r12;
            // 321c7 mov rdx,r14
            rdx = r14;
            // 321ca call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 321cf mov QWORD PTR [r13+0x0],rbx
            m.w64(r13, (rbx) as u64);
            // 321d3 mov QWORD PTR [rbp-0x48],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r12) as u64);
            // 321d7 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 321db mov QWORD PTR [r13+0x8],rax
            m.w64(r13.wrapping_add(0x8), (rax) as u64);
            // 321df mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 321e3 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 321e7 mov QWORD PTR [r13+0x10],r15
            m.w64(r13.wrapping_add(0x10), (r15) as u64);
            // 321eb mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 321ef mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 321f3 mov QWORD PTR [rbp-0x50],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r12) as u64);
            // 321f7 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 321fb call 0xebd08
            { let r = { let t = m.env.stub_va(0x4d5); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32200 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 32204 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32205 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 32207 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 32209 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3220b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3220d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3220e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4114 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000c4114<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4114;
    loop {
        match pc {
        0xc4114 => {
            // block c4114
            #[cfg(feature = "trace")] eprintln!("f_000c4114 pc c4114 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4114 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4115 mov rbp,rsp
            rbp = rsp;
            // c4118 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c411a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c411c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c411e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c4120 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4121 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c4125 mov r14,rsi
            r14 = rsi;
            // c4128 mov r13,rdi
            r13 = rdi;
            // c412b movabs r15,0xaaaaaaaaaaaaaaa
            r15 = 0xaaa_aaaa_aaaa_aaaau64;
            // c4135 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // c4139 mov rbx,QWORD PTR [r13+0x8]
            rbx = m.r64(r13.wrapping_add(0x8)) as u64;
            // c413d sub rbx,rax
            { let (r, f) = sub::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // c4140 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // c4144 movabs r12,0xaaaaaaaaaaaaaaab
            r12 = 0xaaaa_aaaa_aaaa_aaabu64;
            // c414e imul rbx,r12
            { let (r, f) = imul::<64>(rbx, r12); rbx = r; fl = f; }
            // c4152 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // c4155 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // c4158 jbe 0xc4166
            if fl.be() { pc = 0xc4166; continue; }
            pc = 0xc415a;
        }
        0xc415a => {
            // block c415a
            #[cfg(feature = "trace")] eprintln!("f_000c4114 pc c415a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c415a mov rdi,r13
            rdi = r13;
            // c415d call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0xc4166 => {
            // block c4166
            #[cfg(feature = "trace")] eprintln!("f_000c4114 pc c4166 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4166 lea rcx,[r13+0x10]
            rcx = r13.wrapping_add(0x10);
            // c416a mov rdx,QWORD PTR [r13+0x10]
            rdx = m.r64(r13.wrapping_add(0x10)) as u64;
            // c416e sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // c4171 sar rdx,0x3
            { let (r, f) = sar::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // c4175 imul rdx,r12
            { let (r, f) = imul::<64>(rdx, r12); rdx = r; fl = f; }
            // c4179 movabs rsi,0x555555555555554
            rsi = 0x555_5555_5555_5554u64;
            // c4183 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // c4186 ja 0xc4195
            if fl.a() { pc = 0xc4195; continue; }
            pc = 0xc4188;
        }
        0xc4188 => {
            // block c4188
            #[cfg(feature = "trace")] eprintln!("f_000c4114 pc c4188 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4188 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // c418b cmp rdx,rbx
            fl = sub::<64>(rdx, rbx).1;
            // c418e cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // c4192 mov r15,rdx
            r15 = rdx;
            pc = 0xc4195;
        }
        0xc4195 => {
            // block c4195
            #[cfg(feature = "trace")] eprintln!("f_000c4114 pc c4195 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4195 mov rbx,QWORD PTR [r13+0x8]
            rbx = m.r64(r13.wrapping_add(0x8)) as u64;
            // c4199 sub rbx,rax
            { let (r, f) = sub::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // c419c sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // c41a0 imul rbx,r12
            { let (r, f) = imul::<64>(rbx, r12); rbx = r; fl = f; }
            // c41a4 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // c41ac mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // c41b0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c41b2 test r15,r15
            fl = logic::<64>(r15 & r15);
            // c41b5 je 0xc41c8
            if fl.e() { pc = 0xc41c8; continue; }
            pc = 0xc41b7;
        }
        0xc41b7 => {
            // block c41b7
            #[cfg(feature = "trace")] eprintln!("f_000c4114 pc c41b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c41b7 lea rax,[r15*8+0x0]
            rax = r15.wrapping_mul(8);
            // c41bf lea rdi,[rax+rax*2]
            rdi = rax.wrapping_add(rax.wrapping_mul(2));
            // c41c3 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc41c8;
        }
        0xc41c8 => {
            // block c41c8
            #[cfg(feature = "trace")] eprintln!("f_000c4114 pc c41c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c41c8 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // c41cc lea rcx,[rbx+rbx*2]
            rcx = rbx.wrapping_add(rbx.wrapping_mul(2));
            // c41d0 lea rdx,[rax+rcx*8]
            rdx = rax.wrapping_add(rcx.wrapping_mul(8));
            // c41d4 mov QWORD PTR [rbp-0x48],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdx) as u64);
            // c41d8 lea rdx,[r15+r15*2]
            rdx = r15.wrapping_add(r15.wrapping_mul(2));
            // c41dc lea rdx,[rax+rdx*8]
            rdx = rax.wrapping_add(rdx.wrapping_mul(8));
            // c41e0 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // c41e4 mov rdx,QWORD PTR [r14+0x10]
            rdx = m.r64(r14.wrapping_add(0x10)) as u64;
            // c41e8 mov QWORD PTR [rax+rcx*8+0x10],rdx
            m.w64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10), (rdx) as u64);
            // c41ed mov rdx,QWORD PTR [r14]
            rdx = m.r64(r14) as u64;
            // c41f0 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // c41f4 mov QWORD PTR [rax+rcx*8+0x8],rsi
            m.w64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8), (rsi) as u64);
            // c41f9 mov QWORD PTR [rax+rcx*8],rdx
            m.w64(rax.wrapping_add(rcx.wrapping_mul(8)), (rdx) as u64);
            // c41fd mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // c4205 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // c420d mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // c4214 lea rax,[rax+rcx*8+0x18]
            rax = rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18);
            // c4219 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // c421d lea r14,[rbp-0x50]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // c4221 mov rdi,r13
            rdi = r13;
            // c4224 mov rsi,r14
            rsi = r14;
            // c4227 call 0xebe94
            { let r = { let t = m.env.stub_va(0x661); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c422c mov rdi,r14
            rdi = r14;
            // c422f call 0xebef4
            { let r = { let t = m.env.stub_va(0x6c1); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4234 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c4238 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4239 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c423b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c423d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c423f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c4241 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4242 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x104ec size=366
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000104ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x104ec;
    loop {
        match pc {
        0x104ec => {
            // block 104ec
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 104ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 104ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 104ed mov rbp,rsp
            rbp = rsp;
            // 104f0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 104f2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 104f4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 104f6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 104f8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 104f9 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 104fd mov r14,rsi
            r14 = rsi;
            // 10500 mov r12,rdi
            r12 = rdi;
            // 10503 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 10508 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1050d mov rax,r15
            rax = r15;
            // 10510 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // 10513 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10517 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 1051a jae 0x10549
            if fl.ae() { pc = 0x10549; continue; }
            pc = 0x1051c;
        }
        0x1051c => {
            // block 1051c
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 1051c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1051c mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 10520 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 10523 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // 10527 add rbx,r14
            { let (r, f) = add::<64>(rbx, r14);
              rbx = r;
              fl = f; }
            // 1052a mov rax,rbx
            rax = rbx;
            // 1052d shr rax,0x3d
            { let (r, f) = shr::<64>(rax, 0x3du64, fl); rax = r; fl = f; }
            // 10531 je 0x10567
            if fl.e() { pc = 0x10567; continue; }
            pc = 0x10533;
        }
        0x10533 => {
            // block 10533
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 10533 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10533 mov rdi,r12
            rdi = r12;
            // 10536 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x10549 => {
            // block 10549
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 10549 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10549 lea rsi,[r14*8+0x0]
            rsi = r14.wrapping_mul(8);
            // 10551 mov rdi,rbx
            rdi = rbx;
            // 10554 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10559 lea rax,[rbx+r14*8]
            rax = rbx.wrapping_add(r14.wrapping_mul(8));
            // 1055d mov QWORD PTR [r12+0x8],rax
            m.w64(r12.wrapping_add(0x8), (rax) as u64);
            // 10562 jmp 0x1064a
            pc = 0x1064a; continue;
        }
        0x10567 => {
            // block 10567
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 10567 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10567 mov rax,r15
            rax = r15;
            pc = 0x1056a;
        }
        0x1056a => {
            // block 1056a
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 1056a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1056a mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 1056e lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 10573 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 10576 mov rcx,r15
            rcx = r15;
            // 10579 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 1057d movabs rdx,0xffffffffffffffe
            rdx = 0xfff_ffff_ffff_fffeu64;
            // 10587 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1058a ja 0x10599
            if fl.a() { pc = 0x10599; continue; }
            pc = 0x1058c;
        }
        0x1058c => {
            // block 1058c
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 1058c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1058c sar r15,0x2
            { let (r, f) = sar::<64>(r15, 0x2u64, fl); r15 = r; fl = f; }
            // 10590 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 10593 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 10597 jmp 0x105a3
            pc = 0x105a3; continue;
        }
        0x10599 => {
            // block 10599
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 10599 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10599 movabs r15,0x1fffffffffffffff
            r15 = 0x1fff_ffff_ffff_ffffu64;
            pc = 0x105a3;
        }
        0x105a3 => {
            // block 105a3
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 105a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 105a3 mov QWORD PTR [rbp-0x58],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r14) as u64);
            // 105a7 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 105ac mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 105b0 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 105b3 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 105b7 mov r13,r14
            r13 = r14;
            // 105ba sar r13,0x3
            { let (r, f) = sar::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // 105be mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 105c6 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 105ca xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 105cc test r15,r15
            fl = logic::<64>(r15 & r15);
            // 105cf je 0x105e1
            if fl.e() { pc = 0x105e1; continue; }
            pc = 0x105d1;
        }
        0x105d1 => {
            // block 105d1
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 105d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 105d1 lea rdi,[r15*8+0x0]
            rdi = r15.wrapping_mul(8);
            // 105d9 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 105de mov rbx,rax
            rbx = rax;
            pc = 0x105e1;
        }
        0x105e1 => {
            // block 105e1
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 105e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 105e1 lea rdi,[rbx+r13*8]
            rdi = rbx.wrapping_add(r13.wrapping_mul(8));
            // 105e5 lea rax,[rbx+r15*8]
            rax = rbx.wrapping_add(r15.wrapping_mul(8));
            // 105e9 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 105ed mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 105f1 lea rsi,[r15*8+0x0]
            rsi = r15.wrapping_mul(8);
            // 105f9 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 105fe add r13,r15
            { let (r, f) = add::<64>(r13, r15);
              r13 = r;
              fl = f; }
            // 10601 lea r13,[rbx+r13*8]
            r13 = rbx.wrapping_add(r13.wrapping_mul(8));
            // 10605 mov rdi,rbx
            rdi = rbx;
            // 10608 mov r15,QWORD PTR [rbp-0x60]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1060c mov rsi,r15
            rsi = r15;
            // 1060f mov rdx,r14
            rdx = r14;
            // 10612 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10617 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1061b mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1061f mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 10624 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 10628 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1062c mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 10630 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 10635 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 10639 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1063d mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 10641 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10645 call 0xeb864
            { let r = { let t = m.env.stub_va(0x31); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1064a;
        }
        0x1064a => {
            // block 1064a
            #[cfg(feature = "trace")] eprintln!("f_000104ec pc 1064a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1064a add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1064e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1064f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 10651 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 10653 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10655 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10657 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10658 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18f84 size=422
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018f84<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x18f84;
    loop {
        match pc {
        0x18f84 => {
            // block 18f84
            #[cfg(feature = "trace")] eprintln!("f_00018f84 pc 18f84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f84 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18f85 mov rbp,rsp
            rbp = rsp;
            // 18f88 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18f89 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 18f8a mov rbx,rsi
            rbx = rsi;
            // 18f8d mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18f8f call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18f94 mov WORD PTR [rbx+0x30],ax
            m.w16(rbx.wrapping_add(0x30), ((rax as u16 as u64)) as u16);
            // 18f98 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18f9a call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18f9f mov WORD PTR [rbx+0x32],ax
            m.w16(rbx.wrapping_add(0x32), ((rax as u16 as u64)) as u16);
            // 18fa3 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18fa5 call 0xcf7a1
            { let r = { let t = m.env.code_va(0xcf7a1); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18faa mov WORD PTR [rbx+0x36],ax
            m.w16(rbx.wrapping_add(0x36), ((rax as u16 as u64)) as u16);
            // 18fae mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18fb0 call 0xcf778
            { let r = { let t = m.env.code_va(0xcf778); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18fb5 mov WORD PTR [rbx+0x34],ax
            m.w16(rbx.wrapping_add(0x34), ((rax as u16 as u64)) as u16);
            // 18fb9 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18fbb call 0xd0319
            { let r = { let t = m.env.code_va(0xd0319); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18fc0 mov WORD PTR [rbx+0x3a],ax
            m.w16(rbx.wrapping_add(0x3a), ((rax as u16 as u64)) as u16);
            // 18fc4 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18fc6 call 0xd02ac
            { let r = { let t = m.env.code_va(0xd02ac); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18fcb mov WORD PTR [rbx+0x38],ax
            m.w16(rbx.wrapping_add(0x38), ((rax as u16 as u64)) as u16);
            // 18fcf mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18fd1 call 0xd041c
            { let r = { let t = m.env.code_va(0xd041c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18fd6 mov WORD PTR [rbx+0x3c],ax
            m.w16(rbx.wrapping_add(0x3c), ((rax as u16 as u64)) as u16);
            // 18fda mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18fdc call 0xd0758
            { let r = { let t = m.env.code_va(0xd0758); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18fe1 mov WORD PTR [rbx+0x3e],ax
            m.w16(rbx.wrapping_add(0x3e), ((rax as u16 as u64)) as u16);
            // 18fe5 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18fe7 call 0xd0a75
            { let r = { let t = m.env.code_va(0xd0a75); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18fec mov WORD PTR [rbx+0x40],ax
            m.w16(rbx.wrapping_add(0x40), ((rax as u16 as u64)) as u16);
            // 18ff0 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18ff2 call 0xcfbe8
            { let r = { let t = m.env.code_va(0xcfbe8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ff7 mov WORD PTR [rbx+0x42],ax
            m.w16(rbx.wrapping_add(0x42), ((rax as u16 as u64)) as u16);
            // 18ffb mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 18ffd call 0xce5e7
            { let r = { let t = m.env.code_va(0xce5e7); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19002 mov WORD PTR [rbx+0x44],ax
            m.w16(rbx.wrapping_add(0x44), ((rax as u16 as u64)) as u16);
            // 19006 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19008 call 0xcc093
            { let r = { let t = m.env.code_va(0xcc093); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1900d mov BYTE PTR [rbx+0x48],al
            m.w8(rbx.wrapping_add(0x48), ((rax as u8 as u64)) as u8);
            // 19010 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19012 call 0xccb96
            { let r = { let t = m.env.code_va(0xccb96); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19017 mov BYTE PTR [rbx+0x49],al
            m.w8(rbx.wrapping_add(0x49), ((rax as u8 as u64)) as u8);
            // 1901a mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 1901c call 0xcc2f2
            { let r = { let t = m.env.code_va(0xcc2f2); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19021 mov BYTE PTR [rbx+0x4a],al
            m.w8(rbx.wrapping_add(0x4a), ((rax as u8 as u64)) as u8);
            // 19024 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19026 call 0xccb9e
            { let r = { let t = m.env.code_va(0xccb9e); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1902b mov BYTE PTR [rbx+0x4b],al
            m.w8(rbx.wrapping_add(0x4b), ((rax as u8 as u64)) as u8);
            // 1902e mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19030 call 0xcbda9
            { let r = { let t = m.env.code_va(0xcbda9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19035 mov BYTE PTR [rbx+0x4e],al
            m.w8(rbx.wrapping_add(0x4e), ((rax as u8 as u64)) as u8);
            // 19038 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 1903a call 0xcca07
            { let r = { let t = m.env.code_va(0xcca07); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1903f mov BYTE PTR [rbx+0x4f],al
            m.w8(rbx.wrapping_add(0x4f), ((rax as u8 as u64)) as u8);
            // 19042 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19044 call 0xccc9a
            { let r = { let t = m.env.code_va(0xccc9a); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19049 mov BYTE PTR [rbx+0x50],al
            m.w8(rbx.wrapping_add(0x50), ((rax as u8 as u64)) as u8);
            // 1904c mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 1904e call 0xcbc03
            { let r = { let t = m.env.code_va(0xcbc03); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19053 mov BYTE PTR [rbx+0x51],al
            m.w8(rbx.wrapping_add(0x51), ((rax as u8 as u64)) as u8);
            // 19056 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19058 call 0xcd2df
            { let r = { let t = m.env.code_va(0xcd2df); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1905d mov BYTE PTR [rbx+0x4c],al
            m.w8(rbx.wrapping_add(0x4c), ((rax as u8 as u64)) as u8);
            // 19060 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19062 call 0xcbfd6
            { let r = { let t = m.env.code_va(0xcbfd6); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19067 mov BYTE PTR [rbx+0x4d],al
            m.w8(rbx.wrapping_add(0x4d), ((rax as u8 as u64)) as u8);
            // 1906a mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 1906c call 0xcee78
            { let r = { let t = m.env.code_va(0xcee78); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19071 mov WORD PTR [rbx+0xea],ax
            m.w16(rbx.wrapping_add(0xea), ((rax as u16 as u64)) as u16);
            // 19078 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 1907a call 0xcf200
            { let r = { let t = m.env.code_va(0xcf200); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1907f mov DWORD PTR [rbx+0xf0],eax
            m.w32(rbx.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 19085 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19087 call 0xcb56a
            { let r = { let t = m.env.code_va(0xcb56a); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1908c mov BYTE PTR [rbx+0x53],al
            m.w8(rbx.wrapping_add(0x53), ((rax as u8 as u64)) as u8);
            // 1908f mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19091 call 0xcb6fa
            { let r = { let t = m.env.code_va(0xcb6fa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19096 mov BYTE PTR [rbx+0x54],al
            m.w8(rbx.wrapping_add(0x54), ((rax as u8 as u64)) as u8);
            // 19099 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 1909b call 0xcb584
            { let r = { let t = m.env.code_va(0xcb584); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 190a0 mov BYTE PTR [rbx+0x55],al
            m.w8(rbx.wrapping_add(0x55), ((rax as u8 as u64)) as u8);
            // 190a3 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 190a5 call 0xcc4b2
            { let r = { let t = m.env.code_va(0xcc4b2); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 190aa mov BYTE PTR [rbx+0x46],al
            m.w8(rbx.wrapping_add(0x46), ((rax as u8 as u64)) as u8);
            // 190ad mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 190af call 0xccc92
            { let r = { let t = m.env.code_va(0xccc92); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 190b4 mov BYTE PTR [rbx+0x47],al
            m.w8(rbx.wrapping_add(0x47), ((rax as u8 as u64)) as u8);
            // 190b7 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 190b9 call 0xccb8e
            { let r = { let t = m.env.code_va(0xccb8e); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 190be mov BYTE PTR [rbx+0x56],al
            m.w8(rbx.wrapping_add(0x56), ((rax as u8 as u64)) as u8);
            // 190c1 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 190c3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 190c5 call 0xd12f4
            { let r = { let t = m.env.code_va(0xd12f4); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 190ca mov BYTE PTR [rbx+0x57],al
            m.w8(rbx.wrapping_add(0x57), ((rax as u8 as u64)) as u8);
            // 190cd mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 190cf call 0xd0ea0
            { let r = { let t = m.env.code_va(0xd0ea0); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 190d4 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 190d7 setne BYTE PTR [rbx+0x58]
            m.w8(rbx.wrapping_add(0x58), ((fl.ne() as u64)) as u8);
            // 190db mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 190dd call 0xd0fb9
            { let r = { let t = m.env.code_va(0xd0fb9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 190e2 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 190e5 setne BYTE PTR [rbx+0x59]
            m.w8(rbx.wrapping_add(0x59), ((fl.ne() as u64)) as u8);
            // 190e9 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 190eb call 0xcc62b
            { let r = { let t = m.env.code_va(0xcc62b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 190f0 mov BYTE PTR [rbx+0x5a],al
            m.w8(rbx.wrapping_add(0x5a), ((rax as u8 as u64)) as u8);
            // 190f3 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 190f5 call 0xcd1b9
            { let r = { let t = m.env.code_va(0xcd1b9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 190fa mov BYTE PTR [rbx+0x5b],al
            m.w8(rbx.wrapping_add(0x5b), ((rax as u8 as u64)) as u8);
            // 190fd mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 190ff call 0xcd0e6
            { let r = { let t = m.env.code_va(0xcd0e6); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19104 mov BYTE PTR [rbx+0x5c],al
            m.w8(rbx.wrapping_add(0x5c), ((rax as u8 as u64)) as u8);
            // 19107 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19109 call 0xcc7a3
            { let r = { let t = m.env.code_va(0xcc7a3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1910e mov BYTE PTR [rbx+0x5d],al
            m.w8(rbx.wrapping_add(0x5d), ((rax as u8 as u64)) as u8);
            // 19111 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19113 call 0xd0960
            { let r = { let t = m.env.code_va(0xd0960); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19118 mov WORD PTR [rbx+0xec],ax
            m.w16(rbx.wrapping_add(0xec), ((rax as u16 as u64)) as u16);
            // 1911f mov BYTE PTR [rbx+0x52],0x0
            m.w8(rbx.wrapping_add(0x52), (0x0u64) as u8);
            // 19123 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19127 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19128 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19129 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc5980 size=532
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c5980<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc5980;
    loop {
        match pc {
        0xc5980 => {
            // block c5980
            #[cfg(feature = "trace")] eprintln!("f_000c5980 pc c5980 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5980 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c5981 mov rbp,rsp
            rbp = rsp;
            // c5984 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c5986 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c5987 mov QWORD PTR [rip+0x102c76],0x0 # 0x1c8608
            m.w64(m.env.data_va(904661), (0x0u64) as u64);
            // c5992 mov QWORD PTR [rip+0x102c63],0x0 # 0x1c8600
            m.w64(m.env.data_va(904653), (0x0u64) as u64);
            // c599d lea rsi,[rip+0x102c54] # 0x1c85f8
            rsi = m.env.data_va(904645);
            // c59a4 lea rax,[rip+0x102c55] # 0x1c8600
            rax = m.env.data_va(904653);
            // c59ab mov QWORD PTR [rip+0x102c46],rax # 0x1c85f8
            m.w64(m.env.data_va(904645), (rax) as u64);
            // c59b2 lea rdi,[rip+0xffffffffffffe017] # 0xc39d0
            rdi = m.env.code_va(0xc39d0);
            // c59b9 lea r14,[rip+0xfffffffffff397b0] # 0xfffffffffffff170
            r14 = m.env.data_va(-968387);
            // c59c0 mov rdx,r14
            rdx = r14;
            // c59c3 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c59c8 mov QWORD PTR [rip+0x102c4d],0x0 # 0x1c8620
            m.w64(m.env.data_va(904685), (0x0u64) as u64);
            // c59d3 mov QWORD PTR [rip+0x102c3a],0x0 # 0x1c8618
            m.w64(m.env.data_va(904677), (0x0u64) as u64);
            // c59de lea rsi,[rip+0x102c2b] # 0x1c8610
            rsi = m.env.data_va(904669);
            // c59e5 lea rax,[rip+0x102c2c] # 0x1c8618
            rax = m.env.data_va(904677);
            // c59ec mov QWORD PTR [rip+0x102c1d],rax # 0x1c8610
            m.w64(m.env.data_va(904669), (rax) as u64);
            // c59f3 lea rdi,[rip+0xffffffffffffdfe4] # 0xc39de
            rdi = m.env.code_va(0xc39de);
            // c59fa mov rdx,r14
            rdx = r14;
            // c59fd call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5a02 mov QWORD PTR [rip+0x102c2b],0x0 # 0x1c8638
            m.w64(m.env.data_va(904709), (0x0u64) as u64);
            // c5a0d mov QWORD PTR [rip+0x102c18],0x0 # 0x1c8630
            m.w64(m.env.data_va(904701), (0x0u64) as u64);
            // c5a18 lea rsi,[rip+0x102c09] # 0x1c8628
            rsi = m.env.data_va(904693);
            // c5a1f lea rax,[rip+0x102c0a] # 0x1c8630
            rax = m.env.data_va(904701);
            // c5a26 mov QWORD PTR [rip+0x102bfb],rax # 0x1c8628
            m.w64(m.env.data_va(904693), (rax) as u64);
            // c5a2d lea rdi,[rip+0xffffffffffffdfb8] # 0xc39ec
            rdi = m.env.code_va(0xc39ec);
            // c5a34 mov rdx,r14
            rdx = r14;
            // c5a37 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5a3c mov QWORD PTR [rip+0x102c09],0x0 # 0x1c8650
            m.w64(m.env.data_va(904733), (0x0u64) as u64);
            // c5a47 mov QWORD PTR [rip+0x102bf6],0x0 # 0x1c8648
            m.w64(m.env.data_va(904725), (0x0u64) as u64);
            // c5a52 lea rsi,[rip+0x102be7] # 0x1c8640
            rsi = m.env.data_va(904717);
            // c5a59 lea rax,[rip+0x102be8] # 0x1c8648
            rax = m.env.data_va(904725);
            // c5a60 mov QWORD PTR [rip+0x102bd9],rax # 0x1c8640
            m.w64(m.env.data_va(904717), (rax) as u64);
            // c5a67 lea rdi,[rip+0xffffffffffffdf8c] # 0xc39fa
            rdi = m.env.code_va(0xc39fa);
            // c5a6e mov rdx,r14
            rdx = r14;
            // c5a71 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5a76 mov QWORD PTR [rip+0x102be7],0x0 # 0x1c8668
            m.w64(m.env.data_va(904757), (0x0u64) as u64);
            // c5a81 mov QWORD PTR [rip+0x102bd4],0x0 # 0x1c8660
            m.w64(m.env.data_va(904749), (0x0u64) as u64);
            // c5a8c lea rsi,[rip+0x102bc5] # 0x1c8658
            rsi = m.env.data_va(904741);
            // c5a93 lea rax,[rip+0x102bc6] # 0x1c8660
            rax = m.env.data_va(904749);
            // c5a9a mov QWORD PTR [rip+0x102bb7],rax # 0x1c8658
            m.w64(m.env.data_va(904741), (rax) as u64);
            // c5aa1 lea rdi,[rip+0xffffffffffffdf60] # 0xc3a08
            rdi = m.env.code_va(0xc3a08);
            // c5aa8 mov rdx,r14
            rdx = r14;
            // c5aab call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5ab0 mov QWORD PTR [rip+0x102bc5],0x0 # 0x1c8680
            m.w64(m.env.data_va(904781), (0x0u64) as u64);
            // c5abb mov QWORD PTR [rip+0x102bb2],0x0 # 0x1c8678
            m.w64(m.env.data_va(904773), (0x0u64) as u64);
            // c5ac6 lea rsi,[rip+0x102ba3] # 0x1c8670
            rsi = m.env.data_va(904765);
            // c5acd lea rax,[rip+0x102ba4] # 0x1c8678
            rax = m.env.data_va(904773);
            // c5ad4 mov QWORD PTR [rip+0x102b95],rax # 0x1c8670
            m.w64(m.env.data_va(904765), (rax) as u64);
            // c5adb lea rdi,[rip+0xffffffffffffdf34] # 0xc3a16
            rdi = m.env.code_va(0xc3a16);
            // c5ae2 mov rdx,r14
            rdx = r14;
            // c5ae5 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5aea mov QWORD PTR [rip+0x102ba3],0x0 # 0x1c8698
            m.w64(m.env.data_va(904805), (0x0u64) as u64);
            // c5af5 mov QWORD PTR [rip+0x102b90],0x0 # 0x1c8690
            m.w64(m.env.data_va(904797), (0x0u64) as u64);
            // c5b00 lea rsi,[rip+0x102b81] # 0x1c8688
            rsi = m.env.data_va(904789);
            // c5b07 lea rax,[rip+0x102b82] # 0x1c8690
            rax = m.env.data_va(904797);
            // c5b0e mov QWORD PTR [rip+0x102b73],rax # 0x1c8688
            m.w64(m.env.data_va(904789), (rax) as u64);
            // c5b15 lea rdi,[rip+0xffffffffffffdf08] # 0xc3a24
            rdi = m.env.code_va(0xc3a24);
            // c5b1c mov rdx,r14
            rdx = r14;
            // c5b1f call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5b24 mov QWORD PTR [rip+0x102b81],0x0 # 0x1c86b0
            m.w64(m.env.data_va(904829), (0x0u64) as u64);
            // c5b2f mov QWORD PTR [rip+0x102b6e],0x0 # 0x1c86a8
            m.w64(m.env.data_va(904821), (0x0u64) as u64);
            // c5b3a lea rsi,[rip+0x102b5f] # 0x1c86a0
            rsi = m.env.data_va(904813);
            // c5b41 lea rax,[rip+0x102b60] # 0x1c86a8
            rax = m.env.data_va(904821);
            // c5b48 mov QWORD PTR [rip+0x102b51],rax # 0x1c86a0
            m.w64(m.env.data_va(904813), (rax) as u64);
            // c5b4f lea rdi,[rip+0xffffffffffffdedc] # 0xc3a32
            rdi = m.env.code_va(0xc3a32);
            // c5b56 mov rdx,r14
            rdx = r14;
            // c5b59 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5b5e lea rbx,[rip+0x102a2b] # 0x1c8590
            rbx = m.env.data_va(904541);
            // c5b65 lea rsi,[rip+0x72316] # 0x137e82
            rsi = m.env.data_va(312911);
            // c5b6c mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // c5b71 mov rdi,rbx
            rdi = rbx;
            // c5b74 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5b79 mov rdi,QWORD PTR [rip+0x7a5f8] # 0x140178
            rdi = m.r64(m.env.data_va(346437)) as u64;
            // c5b80 mov rsi,rbx
            rsi = rbx;
            // c5b83 mov rdx,r14
            rdx = r14;
            // c5b86 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5b8b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5b8c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c5b8e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c5b8f jmp 0xbca4e
            { let r = { let t = m.env.code_va(0xbca4e); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a9ca size=660
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001a9ca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1a9ca;
    loop {
        match pc {
        0x1a9ca => {
            // block 1a9ca
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1a9ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a9ca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a9cb mov rbp,rsp
            rbp = rsp;
            // 1a9ce push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1a9d0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a9d2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1a9d4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1a9d6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a9d7 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 1a9db mov r12,rsi
            r12 = rsi;
            // 1a9de mov r14,rdi
            r14 = rdi;
            // 1a9e1 movabs r15,0x108421084210842
            r15 = 0x108_4210_8421_0842u64;
            // 1a9eb mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1a9ee mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a9f2 sub rbx,rax
            { let (r, f) = sub::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 1a9f5 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // 1a9f9 movabs rdi,0xef7bdef7bdef7bdf
            rdi = 0xef7b_def7_bdef_7bdfu64;
            // 1aa03 imul rbx,rdi
            { let (r, f) = imul::<64>(rbx, rdi); rbx = r; fl = f; }
            // 1aa07 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1aa0a cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 1aa0d jbe 0x1aa20
            if fl.be() { pc = 0x1aa20; continue; }
            pc = 0x1aa0f;
        }
        0x1aa0f => {
            // block 1aa0f
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1aa0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1aa0f mov r13,rdi
            r13 = rdi;
            // 1aa12 mov rdi,r14
            rdi = r14;
            // 1aa15 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x1aa20 => {
            // block 1aa20
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1aa20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1aa20 lea rcx,[r14+0x10]
            rcx = r14.wrapping_add(0x10);
            // 1aa24 mov rdx,QWORD PTR [r14+0x10]
            rdx = m.r64(r14.wrapping_add(0x10)) as u64;
            // 1aa28 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 1aa2b sar rdx,0x3
            { let (r, f) = sar::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // 1aa2f imul rdx,rdi
            { let (r, f) = imul::<64>(rdx, rdi); rdx = r; fl = f; }
            // 1aa33 movabs rsi,0x84210842108420
            rsi = 0x84_2108_4210_8420u64;
            // 1aa3d cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 1aa40 ja 0x1aa4f
            if fl.a() { pc = 0x1aa4f; continue; }
            pc = 0x1aa42;
        }
        0x1aa42 => {
            // block 1aa42
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1aa42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1aa42 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 1aa45 cmp rdx,rbx
            fl = sub::<64>(rdx, rbx).1;
            // 1aa48 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // 1aa4c mov r15,rdx
            r15 = rdx;
            pc = 0x1aa4f;
        }
        0x1aa4f => {
            // block 1aa4f
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1aa4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1aa4f mov r13,QWORD PTR [r14+0x8]
            r13 = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1aa53 sub r13,rax
            { let (r, f) = sub::<64>(r13, rax);
              r13 = r;
              fl = f; }
            // 1aa56 sar r13,0x3
            { let (r, f) = sar::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // 1aa5a imul r13,rdi
            { let (r, f) = imul::<64>(r13, rdi); r13 = r; fl = f; }
            // 1aa5e mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1aa66 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // 1aa6a xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1aa6c test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1aa6f je 0x1aa80
            if fl.e() { pc = 0x1aa80; continue; }
            pc = 0x1aa71;
        }
        0x1aa71 => {
            // block 1aa71
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1aa71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1aa71 imul rdi,r15,0xf8
            { let (r, f) = imul::<64>(r15, 0xf8u64); rdi = r; fl = f; }
            // 1aa78 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1aa7d mov rbx,rax
            rbx = rax;
            pc = 0x1aa80;
        }
        0x1aa80 => {
            // block 1aa80
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1aa80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1aa80 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // 1aa84 imul r13,r13,0xf8
            { let (r, f) = imul::<64>(r13, 0xf8u64); r13 = r; fl = f; }
            // 1aa8b lea rax,[rbx+r13*1]
            rax = rbx.wrapping_add(r13);
            // 1aa8f mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1aa93 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 1aa97 imul rax,r15,0xf8
            { let (r, f) = imul::<64>(r15, 0xf8u64); rax = r; fl = f; }
            // 1aa9e add rax,rbx
            { let (r, f) = add::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // 1aaa1 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1aaa5 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1aaaa mov QWORD PTR [rbx+r13*1+0x10],rax
            m.w64(rbx.wrapping_add(r13).wrapping_add(0x10), (rax) as u64);
            // 1aaaf mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 1aab3 mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1aab8 mov QWORD PTR [rbx+r13*1+0x8],rcx
            m.w64(rbx.wrapping_add(r13).wrapping_add(0x8), (rcx) as u64);
            // 1aabd mov QWORD PTR [rbx+r13*1],rax
            m.w64(rbx.wrapping_add(r13), (rax) as u64);
            // 1aac1 lea r15,[rbx+r13*1+0x18]
            r15 = rbx.wrapping_add(r13).wrapping_add(0x18);
            // 1aac6 lea rsi,[r12+0x18]
            rsi = r12.wrapping_add(0x18);
            // 1aacb mov rdi,r15
            rdi = r15;
            // 1aace call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1aad3 mov ax,WORD PTR [r12+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 1aad9 mov WORD PTR [rbx+r13*1+0x5c],ax
            m.w16(rbx.wrapping_add(r13).wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 1aadf mov eax,DWORD PTR [r12+0x58]
            rax = (m.r32(r12.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1aae4 mov DWORD PTR [rbx+r13*1+0x58],eax
            m.w32(rbx.wrapping_add(r13).wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 1aae9 mov rax,QWORD PTR [r12+0x50]
            rax = m.r64(r12.wrapping_add(0x50)) as u64;
            // 1aaee mov QWORD PTR [rbx+r13*1+0x50],rax
            m.w64(rbx.wrapping_add(r13).wrapping_add(0x50), (rax) as u64);
            // 1aaf3 mov rax,QWORD PTR [r12+0x48]
            rax = m.r64(r12.wrapping_add(0x48)) as u64;
            // 1aaf8 mov QWORD PTR [rbx+r13*1+0x48],rax
            m.w64(rbx.wrapping_add(r13).wrapping_add(0x48), (rax) as u64);
            // 1aafd mov rax,QWORD PTR [r12+0x40]
            rax = m.r64(r12.wrapping_add(0x40)) as u64;
            // 1ab02 mov QWORD PTR [rbx+r13*1+0x40],rax
            m.w64(rbx.wrapping_add(r13).wrapping_add(0x40), (rax) as u64);
            // 1ab07 mov rax,QWORD PTR [r12+0x30]
            rax = m.r64(r12.wrapping_add(0x30)) as u64;
            // 1ab0c mov rcx,QWORD PTR [r12+0x38]
            rcx = m.r64(r12.wrapping_add(0x38)) as u64;
            // 1ab11 mov QWORD PTR [rbx+r13*1+0x38],rcx
            m.w64(rbx.wrapping_add(r13).wrapping_add(0x38), (rcx) as u64);
            // 1ab16 mov QWORD PTR [rbx+r13*1+0x30],rax
            m.w64(rbx.wrapping_add(r13).wrapping_add(0x30), (rax) as u64);
            // 1ab1b lea rdi,[rbx+r13*1+0x60]
            rdi = rbx.wrapping_add(r13).wrapping_add(0x60);
            // 1ab20 mov QWORD PTR [rbp-0x60],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rdi) as u64);
            // 1ab24 lea rsi,[r12+0x60]
            rsi = r12.wrapping_add(0x60);
            // 1ab29 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1ab2e lea rdi,[rbx+r13*1+0x78]
            rdi = rbx.wrapping_add(r13).wrapping_add(0x78);
            // 1ab33 mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            // 1ab37 lea rsi,[r12+0x78]
            rsi = r12.wrapping_add(0x78);
            // 1ab3c call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1ab41 mov QWORD PTR [rbp-0x58],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r15) as u64);
            // 1ab45 lea rdi,[rbx+r13*1+0x90]
            rdi = rbx.wrapping_add(r13).wrapping_add(0x90);
            // 1ab4d mov QWORD PTR [rbp-0x70],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rdi) as u64);
            // 1ab51 lea rsi,[r12+0x90]
            rsi = r12.wrapping_add(0x90);
            // 1ab59 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1ab5e lea r15,[rbx+r13*1+0xa8]
            r15 = rbx.wrapping_add(r13).wrapping_add(0xa8);
            // 1ab66 lea rsi,[r12+0xa8]
            rsi = r12.wrapping_add(0xa8);
            // 1ab6e mov rdi,r15
            rdi = r15;
            // 1ab71 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1ab76 lea rdi,[rbx+r13*1+0xc0]
            rdi = rbx.wrapping_add(r13).wrapping_add(0xc0);
            // 1ab7e lea rsi,[r12+0xc0]
            rsi = r12.wrapping_add(0xc0);
            // 1ab86 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1ab8b mov eax,DWORD PTR [r12+0xf0]
            rax = (m.r32(r12.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1ab93 mov DWORD PTR [rbx+r13*1+0xf0],eax
            m.w32(rbx.wrapping_add(r13).wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 1ab9b mov rax,QWORD PTR [r12+0xe8]
            rax = m.r64(r12.wrapping_add(0xe8)) as u64;
            // 1aba3 mov QWORD PTR [rbx+r13*1+0xe8],rax
            m.w64(rbx.wrapping_add(r13).wrapping_add(0xe8), (rax) as u64);
            // 1abab mov rax,QWORD PTR [r12+0xd8]
            rax = m.r64(r12.wrapping_add(0xd8)) as u64;
            // 1abb3 mov rcx,QWORD PTR [r12+0xe0]
            rcx = m.r64(r12.wrapping_add(0xe0)) as u64;
            // 1abbb mov QWORD PTR [rbx+r13*1+0xe0],rcx
            m.w64(rbx.wrapping_add(r13).wrapping_add(0xe0), (rcx) as u64);
            // 1abc3 mov QWORD PTR [rbx+r13*1+0xd8],rax
            m.w64(rbx.wrapping_add(r13).wrapping_add(0xd8), (rax) as u64);
            // 1abcb lea rax,[rbx+r13*1+0xf8]
            rax = rbx.wrapping_add(r13).wrapping_add(0xf8);
            // 1abd3 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1abd7 lea rbx,[rbp-0x50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1abdb mov rdi,r14
            rdi = r14;
            // 1abde mov rsi,rbx
            rsi = rbx;
            // 1abe1 call 0xeb9de
            { let r = { let t = m.env.stub_va(0x1ab); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1abe6 mov rdi,rbx
            rdi = rbx;
            // 1abe9 call 0xeb96c
            { let r = { let t = m.env.stub_va(0x139); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1abee add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 1abf2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1abf3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1abf5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1abf7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1abf9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1abfb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1abfc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1abfd => {
            // block 1abfd
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1abfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1abfd mov rbx,rax
            rbx = rax;
            // 1ac00 jmp 0x1ac4d
            pc = 0x1ac4d; continue;
        }
        0x1ac02 => {
            // block 1ac02
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1ac02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ac02 mov QWORD PTR [rbp-0x58],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r15) as u64);
            // 1ac06 mov rbx,rax
            rbx = rax;
            // 1ac09 jmp 0x1ac44
            pc = 0x1ac44; continue;
        }
        0x1ac0b => {
            // block 1ac0b
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1ac0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ac0b mov QWORD PTR [rbp-0x58],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r15) as u64);
            // 1ac0f mov rbx,rax
            rbx = rax;
            // 1ac12 jmp 0x1ac3b
            pc = 0x1ac3b; continue;
        }
        0x1ac14 => {
            // block 1ac14
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1ac14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ac14 mov rbx,rax
            rbx = rax;
            // 1ac17 jmp 0x1ac32
            pc = 0x1ac32; continue;
        }
        0x1ac19 => {
            // block 1ac19
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1ac19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ac19 mov rbx,rax
            rbx = rax;
            // 1ac1c jmp 0x1ac29
            pc = 0x1ac29; continue;
        }
        0x1ac1e => {
            // block 1ac1e
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1ac1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ac1e mov rbx,rax
            rbx = rax;
            // 1ac21 mov rdi,r15
            rdi = r15;
            // 1ac24 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1ac29;
        }
        0x1ac29 => {
            // block 1ac29
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1ac29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ac29 mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1ac2d call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1ac32;
        }
        0x1ac32 => {
            // block 1ac32
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1ac32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ac32 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1ac36 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1ac3b;
        }
        0x1ac3b => {
            // block 1ac3b
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1ac3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ac3b mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1ac3f call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1ac44;
        }
        0x1ac44 => {
            // block 1ac44
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1ac44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ac44 mov rdi,QWORD PTR [rbp-0x58]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1ac48 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1ac4d;
        }
        0x1ac4d => {
            // block 1ac4d
            #[cfg(feature = "trace")] eprintln!("f_0001a9ca pc 1ac4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ac4d lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1ac51 call 0xeb96c
            { let r = { let t = m.env.stub_va(0x139); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1ac56 mov rdi,rbx
            rdi = rbx;
            // 1ac59 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe65f6 size=1154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e65f6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe65f6;
    loop {
        match pc {
        0xe65f6 => {
            // block e65f6
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e65f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e65f6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e65f7 mov rbp,rsp
            rbp = rsp;
            // e65fa push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e65fc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e65fe push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e6600 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e6602 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e6603 sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // e6607 mov rbx,rdi
            rbx = rdi;
            // e660a mov r15,rdx
            r15 = rdx;
            // e660d shr r15,0x20
            { let (r, f) = shr::<64>(r15, 0x20u64, fl); r15 = r; fl = f; }
            // e6611 cmp BYTE PTR [rbx+0x8c],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x8c)) as u64, 0x0u64).1;
            // e6618 je 0xe6640
            if fl.e() { pc = 0xe6640; continue; }
            pc = 0xe661a;
        }
        0xe661a => {
            // block e661a
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e661a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e661a cmp QWORD PTR [rbx+0x90],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x90)) as u64, 0x0u64).1;
            // e6622 je 0xe6640
            if fl.e() { pc = 0xe6640; continue; }
            pc = 0xe6624;
        }
        0xe6624 => {
            // block e6624
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6624 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6624 cmp DWORD PTR [rbx+0x98],esi
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x98)) as u64, (rsi as u32 as u64)).1;
            // e662a jne 0xe6640
            if fl.ne() { pc = 0xe6640; continue; }
            pc = 0xe662c;
        }
        0xe662c => {
            // block e662c
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e662c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e662c cmp r15d,DWORD PTR [rbx+0x9c]
            fl = sub::<32>((r15 as u32 as u64), m.r32(rbx.wrapping_add(0x9c)) as u64).1;
            // e6633 jne 0xe6640
            if fl.ne() { pc = 0xe6640; continue; }
            pc = 0xe6635;
        }
        0xe6635 => {
            // block e6635
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6635 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6635 mov dil,0x1
            rdi = (rdi & !0xff) | ((0x1u64) as u8 as u64);
            // e6638 cmp edx,DWORD PTR [rbx+0xa0]
            fl = sub::<32>((rdx as u32 as u64), m.r32(rbx.wrapping_add(0xa0)) as u64).1;
            // e663e je 0xe6651
            if fl.e() { pc = 0xe6651; continue; }
            pc = 0xe6640;
        }
        0xe6640 => {
            // block e6640
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6640 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6640 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // e6643 ja 0xe664f
            if fl.a() { pc = 0xe664f; continue; }
            pc = 0xe6645;
        }
        0xe6645 => {
            // block e6645
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6645 mov eax,0xc7
            rax = (0xc7u64) as u32 as u64;
            // e664a bt eax,esi
            fl.cf = bit::<32>((rax as u32 as u64), (rsi as u32 as u64));
            // e664d jb 0xe6663
            if fl.b() { pc = 0xe6663; continue; }
            pc = 0xe664f;
        }
        0xe664f => {
            // block e664f
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e664f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e664f xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xe6651;
        }
        0xe6651 => {
            // block e6651
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6651 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6651 mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // e6654 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // e6658 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e6659 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e665b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e665d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e665f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e6661 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e6662 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe6663 => {
            // block e6663
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6663 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6663 mov QWORD PTR [rbp-0x68],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdx) as u64);
            // e6667 mov QWORD PTR [rbp-0x78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rcx) as u64);
            // e666b movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // e666e lea rcx,[rip+0x436eb] # 0x129d60
            rcx = m.env.data_va(255277);
            // e6675 mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // e6678 cmp r15d,0x384
            fl = sub::<32>((r15 as u32 as u64), 0x384u64).1;
            // e667f jbe 0xe66a5
            if fl.be() { pc = 0xe66a5; continue; }
            pc = 0xe6681;
        }
        0xe6681 => {
            // block e6681
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6681 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6681 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e6685 cmp eax,0x5a0
            fl = sub::<32>((rax as u32 as u64), 0x5a0u64).1;
            // e668a seta al
            rax = (rax & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // e668d movzx r14d,al
            r14 = ((rax as u8 as u64)) as u32 as u64;
            // e6691 or r14d,0x2
            { let r = (r14 as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e6695 mov DWORD PTR [rbp-0x60],0x6c
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x6cu64) as u32);
            // e669c mov DWORD PTR [rbp-0x5c],0x6
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), (0x6u64) as u32);
            // e66a3 jmp 0xe66ca
            pc = 0xe66ca; continue;
        }
        0xe66a5 => {
            // block e66a5
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e66a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e66a5 cmp r15d,0x28a
            fl = sub::<32>((r15 as u32 as u64), 0x28au64).1;
            // e66ac seta al
            rax = (rax & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // e66af movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // e66b2 lea r14d,[rax+0x1]
            r14 = (rax.wrapping_add(0x1)) as u32 as u64;
            // e66b6 lea edx,[rax+rax*1+0x2]
            rdx = (rax.wrapping_add(rax).wrapping_add(0x2)) as u32 as u64;
            // e66ba mov DWORD PTR [rbp-0x5c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rdx as u32 as u64)) as u32);
            // e66bd lea eax,[rax*4+0x4]
            rax = (rax.wrapping_mul(4).wrapping_add(0x4)) as u32 as u64;
            // e66c4 lea eax,[rax+rax*8]
            rax = (rax.wrapping_add(rax.wrapping_mul(8))) as u32 as u64;
            // e66c7 mov DWORD PTR [rbp-0x60],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), ((rax as u32 as u64)) as u32);
            pc = 0xe66ca;
        }
        0xe66ca => {
            // block e66ca
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e66ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e66ca mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e66cc mov QWORD PTR [rbp-0x80],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rcx) as u64);
            // e66d0 imul eax,r14d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r14 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e66d4 shl eax,0x3
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x3u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e66d7 lea eax,[rax+rax*2]
            rax = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // e66da mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // e66dd shl ecx,0x6
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x6u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e66e0 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // e66e2 mov DWORD PTR [rbp-0x4c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((rsi as u32 as u64)) as u32);
            // e66e5 cmovne ecx,eax
            if fl.ne() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // e66e8 mov rdi,QWORD PTR [rbx+0x90]
            rdi = m.r64(rbx.wrapping_add(0x90)) as u64;
            // e66ef test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e66f2 je 0xe670a
            if fl.e() { pc = 0xe670a; continue; }
            pc = 0xe66f4;
        }
        0xe66f4 => {
            // block e66f4
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e66f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e66f4 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // e66f7 call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e66fc mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // e66ff mov QWORD PTR [rbx+0x90],0x0
            m.w64(rbx.wrapping_add(0x90), (0x0u64) as u64);
            pc = 0xe670a;
        }
        0xe670a => {
            // block e670a
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e670a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e670a mov DWORD PTR [rbp-0x6c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rcx as u32 as u64)) as u32);
            // e670d mov eax,DWORD PTR [rbp-0x60]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // e6710 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e6713 lea eax,[rax+rax*4]
            rax = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e6716 lea eax,[rax+rax*2]
            rax = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // e6719 movsxd rdi,eax
            rdi = sx::<32>((rax as u32 as u64)) as u64;
            // e671c call 0xec20c
            { let r = { let t = m.env.stub_va(0x9d9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e6721 mov QWORD PTR [rbx+0x90],rax
            m.w64(rbx.wrapping_add(0x90), (rax) as u64);
            // e6728 mov dil,0x1
            rdi = (rdi & !0xff) | ((0x1u64) as u8 as u64);
            // e672b test rax,rax
            fl = logic::<64>(rax & rax);
            // e672e je 0xe6651
            if fl.e() { pc = 0xe6651; continue; }
            pc = 0xe6734;
        }
        0xe6734 => {
            // block e6734
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6734 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6734 mov QWORD PTR [rbp-0x90],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (r15) as u64);
            // e673b mov QWORD PTR [rbp-0x88],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rbx) as u64);
            // e6742 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // e6746 mov eax,DWORD PTR [rbp-0x4c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u32 as u64;
            // e6749 and eax,0xfffffffb
            { let r = (rax as u32 as u64) & 0xfffffffbu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e674c mov DWORD PTR [rbp-0x50],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), ((rax as u32 as u64)) as u32);
            // e674f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6751 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            pc = 0xe6755;
        }
        0xe6755 => {
            // block e6755
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6755 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6755 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6757 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0xe675b;
        }
        0xe675b => {
            // block e675b
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e675b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e675b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe675d;
        }
        0xe675d => {
            // block e675d
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e675d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e675d mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e6761 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e6763 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe6766;
        }
        0xe6766 => {
            // block e6766
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6766 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6766 imul rax,QWORD PTR [rbp-0x48],0x1b0
            { let (r, f) = imul::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, 0x1b0u64); rax = r; fl = f; }
            // e676e lea rcx,[rip+0x41bdb] # 0x128350
            rcx = m.env.data_va(248605);
            // e6775 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // e6778 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // e677c lea rcx,[rcx+rcx*2]
            rcx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // e6780 lea rax,[rax+rcx*8]
            rax = rax.wrapping_add(rcx.wrapping_mul(8));
            // e6784 movsx ecx,BYTE PTR [r15+rax*1]
            rcx = (sx::<8>(m.r8(r15.wrapping_add(rax)) as u64) as u64) as u32 as u64;
            // e6789 mov eax,DWORD PTR [rbp-0x4c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u32 as u64;
            // e678c cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e678f jne 0xe67d7
            if fl.ne() { pc = 0xe67d7; continue; }
            pc = 0xe6791;
        }
        0xe6791 => {
            // block e6791
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6791 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6791 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // e6795 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6797 movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // e679a cmp edx,0x3
            fl = sub::<32>((rdx as u32 as u64), 0x3u64).1;
            // e679d mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // e67a2 ja 0xe67ae
            if fl.a() { pc = 0xe67ae; continue; }
            pc = 0xe67a4;
        }
        0xe67a4 => {
            // block e67a4
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e67a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e67a4 shl ecx,0x3
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x3u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e67a7 mov edx,0xc0864c10
            rdx = (0xc0864c10u64) as u32 as u64;
            // e67ac shr edx,cl
            { let (r, f) = shr::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            pc = 0xe67ae;
        }
        0xe67ae => {
            // block e67ae
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e67ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e67ae mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e67b2 lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // e67b6 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // e67ba mov BYTE PTR [rcx],0x80
            m.w8(rcx, (0x80u64) as u8);
            // e67bd mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e67c1 lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // e67c5 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // e67c9 mov BYTE PTR [rcx],dl
            m.w8(rcx, ((rdx as u8 as u64)) as u8);
            // e67cb inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e67cd cmp eax,r14d
            fl = sub::<32>((rax as u32 as u64), (r14 as u32 as u64)).1;
            // e67d0 jl 0xe67ae
            if fl.l() { pc = 0xe67ae; continue; }
            pc = 0xe67d2;
        }
        0xe67d2 => {
            // block e67d2
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e67d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e67d2 jmp 0xe6966
            pc = 0xe6966; continue;
        }
        0xe67d7 => {
            // block e67d7
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e67d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e67d7 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e67d9 je 0xe6843
            if fl.e() { pc = 0xe6843; continue; }
            pc = 0xe67db;
        }
        0xe67db => {
            // block e67db
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e67db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e67db cmp DWORD PTR [rbp-0x50],0x2
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64, 0x2u64).1;
            // e67df jne 0xe6888
            if fl.ne() { pc = 0xe6888; continue; }
            pc = 0xe67e5;
        }
        0xe67e5 => {
            // block e67e5
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e67e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e67e5 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // e67e9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e67eb movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // e67ee cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e67f1 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e67f6 ja 0xe67fe
            if fl.a() { pc = 0xe67fe; continue; }
            pc = 0xe67f8;
        }
        0xe67f8 => {
            // block e67f8
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e67f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e67f8 mov al,cl
            rax = (rax & !0xff) | (((rcx as u8 as u64)) as u8 as u64);
            // e67fa mov cl,0x55
            rcx = (rcx & !0xff) | ((0x55u64) as u8 as u64);
            // e67fc mul cl
            { let (lo, hi, f) = mul1::<8>(rax, (rcx as u8 as u64)); rax = (rax & !0xffffu64) | (hi << 8) | lo; fl = f; }
            pc = 0xe67fe;
        }
        0xe67fe => {
            // block e67fe
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e67fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e67fe mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e6802 lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // e6806 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // e680a mov BYTE PTR [rcx],0x0
            m.w8(rcx, (0x0u64) as u8);
            // e680d mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e6811 lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // e6815 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // e6819 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // e681b mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e681f lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // e6823 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // e6827 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // e6829 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e682d lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // e6831 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // e6835 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // e6837 inc edx
            { let (r, f) = inc::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e6839 cmp edx,r14d
            fl = sub::<32>((rdx as u32 as u64), (r14 as u32 as u64)).1;
            // e683c jl 0xe67fe
            if fl.l() { pc = 0xe67fe; continue; }
            pc = 0xe683e;
        }
        0xe683e => {
            // block e683e
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e683e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e683e jmp 0xe6966
            pc = 0xe6966; continue;
        }
        0xe6843 => {
            // block e6843
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6843 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6843 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e6845 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // e6848 cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e684b mov r13d,0x0
            r13 = (0x0u64) as u32 as u64;
            // e6851 ja 0xe685e
            if fl.a() { pc = 0xe685e; continue; }
            pc = 0xe6853;
        }
        0xe6853 => {
            // block e6853
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6853 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6853 imul r13d,ecx,0x124
            { let (r, f) = imul::<32>((rcx as u32 as u64), 0x124u64); r13 = (r) as u32 as u64; fl = f; }
            // e685a add r13d,0x40
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x40u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe685e;
        }
        0xe685e => {
            // block e685e
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e685e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e685e mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // e6862 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0xe6865;
        }
        0xe6865 => {
            // block e6865
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6865 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6865 mov ecx,0x200
            rcx = (0x200u64) as u32 as u64;
            // e686a lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e686e mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // e6871 mov r8d,r13d
            r8 = ((r13 as u32 as u64)) as u32 as u64;
            // e6874 call 0xe6a78
            { let r = { let t = m.env.code_va(0xe6a78); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e6879 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // e687b inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // e687e cmp ebx,r14d
            fl = sub::<32>((rbx as u32 as u64), (r14 as u32 as u64)).1;
            // e6881 jl 0xe6865
            if fl.l() { pc = 0xe6865; continue; }
            pc = 0xe6883;
        }
        0xe6883 => {
            // block e6883
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6883 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6883 jmp 0xe6966
            pc = 0xe6966; continue;
        }
        0xe6888 => {
            // block e6888
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6888 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6888 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e688b jne 0xe6908
            if fl.ne() { pc = 0xe6908; continue; }
            pc = 0xe688d;
        }
        0xe688d => {
            // block e688d
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e688d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e688d mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // e6891 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6893 movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // e6896 cmp edx,0x3
            fl = sub::<32>((rdx as u32 as u64), 0x3u64).1;
            // e6899 mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // e689e ja 0xe68a9
            if fl.a() { pc = 0xe68a9; continue; }
            pc = 0xe68a0;
        }
        0xe68a0 => {
            // block e68a0
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e68a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e68a0 imul edx,ecx,0x124
            { let (r, f) = imul::<32>((rcx as u32 as u64), 0x124u64); rdx = (r) as u32 as u64; fl = f; }
            // e68a6 add edx,0x40
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x40u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe68a9;
        }
        0xe68a9 => {
            // block e68a9
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e68a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e68a9 mov r8d,edx
            r8 = ((rdx as u32 as u64)) as u32 as u64;
            // e68ac shr r8d,0x2
            { let (r, f) = shr::<32>((r8 as u32 as u64), 0x2u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // e68b0 mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // e68b2 shr esi,0x4
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x4u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e68b5 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // e68b7 shl ebx,0x4
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x4u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e68ba mov edi,edx
            rdi = ((rdx as u32 as u64)) as u32 as u64;
            // e68bc shr edi,0x6
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x6u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // e68bf or edi,ebx
            { let r = (rdi as u32 as u64) | (rbx as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // e68c1 shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            pc = 0xe68c4;
        }
        0xe68c4 => {
            // block e68c4
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e68c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e68c4 mov rbx,QWORD PTR [rbp-0x30]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e68c8 lea rcx,[rbx+0x1]
            rcx = rbx.wrapping_add(0x1);
            // e68cc mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // e68d0 mov BYTE PTR [rbx],r8b
            m.w8(rbx, ((r8 as u8 as u64)) as u8);
            // e68d3 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e68d7 lea rbx,[rcx+0x1]
            rbx = rcx.wrapping_add(0x1);
            // e68db mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // e68df mov BYTE PTR [rcx],sil
            m.w8(rcx, ((rsi as u8 as u64)) as u8);
            // e68e2 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e68e6 lea rbx,[rcx+0x1]
            rbx = rcx.wrapping_add(0x1);
            // e68ea mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // e68ee mov BYTE PTR [rcx],dil
            m.w8(rcx, ((rdi as u8 as u64)) as u8);
            // e68f1 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e68f5 lea rbx,[rcx+0x1]
            rbx = rcx.wrapping_add(0x1);
            // e68f9 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // e68fd mov BYTE PTR [rcx],dl
            m.w8(rcx, ((rdx as u8 as u64)) as u8);
            // e68ff inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e6901 cmp eax,r14d
            fl = sub::<32>((rax as u32 as u64), (r14 as u32 as u64)).1;
            // e6904 jl 0xe68c4
            if fl.l() { pc = 0xe68c4; continue; }
            pc = 0xe6906;
        }
        0xe6906 => {
            // block e6906
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6906 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6906 jmp 0xe6966
            pc = 0xe6966; continue;
        }
        0xe6908 => {
            // block e6908
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6908 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6908 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // e690c cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // e690f jne 0xe6966
            if fl.ne() { pc = 0xe6966; continue; }
            pc = 0xe6911;
        }
        0xe6911 => {
            // block e6911
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6911 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6911 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e6913 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // e6916 cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e6919 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e691e ja 0xe6926
            if fl.a() { pc = 0xe6926; continue; }
            pc = 0xe6920;
        }
        0xe6920 => {
            // block e6920
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6920 mov al,cl
            rax = (rax & !0xff) | (((rcx as u8 as u64)) as u8 as u64);
            // e6922 mov cl,0x55
            rcx = (rcx & !0xff) | ((0x55u64) as u8 as u64);
            // e6924 mul cl
            { let (lo, hi, f) = mul1::<8>(rax, (rcx as u8 as u64)); rax = (rax & !0xffffu64) | (hi << 8) | lo; fl = f; }
            pc = 0xe6926;
        }
        0xe6926 => {
            // block e6926
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6926 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6926 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e692a lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // e692e mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // e6932 mov BYTE PTR [rcx],0x0
            m.w8(rcx, (0x0u64) as u8);
            // e6935 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e6939 lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // e693d mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // e6941 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // e6943 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e6947 lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // e694b mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // e694f mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // e6951 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e6955 lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // e6959 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // e695d mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // e695f inc edx
            { let (r, f) = inc::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e6961 cmp edx,r14d
            fl = sub::<32>((rdx as u32 as u64), (r14 as u32 as u64)).1;
            // e6964 jl 0xe6926
            if fl.l() { pc = 0xe6926; continue; }
            pc = 0xe6966;
        }
        0xe6966 => {
            // block e6966
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6966 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // e6969 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e696d add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e6970 cmp r15,0x18
            fl = sub::<64>(r15, 0x18u64).1;
            // e6974 jne 0xe6766
            if fl.ne() { pc = 0xe6766; continue; }
            pc = 0xe697a;
        }
        0xe697a => {
            // block e697a
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e697a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e697a mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e697e inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e6980 cmp eax,DWORD PTR [rbp-0x5c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64).1;
            // e6983 jl 0xe675d
            if fl.l() { pc = 0xe675d; continue; }
            pc = 0xe6989;
        }
        0xe6989 => {
            // block e6989
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6989 mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // e698d inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e6990 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // e6994 cmp rax,0x12
            fl = sub::<64>(rax, 0x12u64).1;
            // e6998 jne 0xe675b
            if fl.ne() { pc = 0xe675b; continue; }
            pc = 0xe699e;
        }
        0xe699e => {
            // block e699e
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e699e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e699e mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e69a2 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e69a5 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // e69a9 cmp rax,0xf
            fl = sub::<64>(rax, 0xfu64).1;
            // e69ad jne 0xe6755
            if fl.ne() { pc = 0xe6755; continue; }
            pc = 0xe69b3;
        }
        0xe69b3 => {
            // block e69b3
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e69b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e69b3 mov rdx,QWORD PTR [rbp-0x88]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // e69ba mov BYTE PTR [rdx+0x8c],0x1
            m.w8(rdx.wrapping_add(0x8c), (0x1u64) as u8);
            // e69c1 mov eax,DWORD PTR [rbp-0x4c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u32 as u64;
            // e69c4 mov DWORD PTR [rdx+0x98],eax
            m.w32(rdx.wrapping_add(0x98), ((rax as u32 as u64)) as u32);
            // e69ca mov rdi,QWORD PTR [rbp-0x90]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // e69d1 mov DWORD PTR [rdx+0x9c],edi
            m.w32(rdx.wrapping_add(0x9c), ((rdi as u32 as u64)) as u32);
            // e69d7 mov rcx,QWORD PTR [rbp-0x68]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e69db mov DWORD PTR [rdx+0xa0],ecx
            m.w32(rdx.wrapping_add(0xa0), ((rcx as u32 as u64)) as u32);
            // e69e1 mov esi,DWORD PTR [rbp-0x6c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // e69e4 mov DWORD PTR [rdx+0xa4],esi
            m.w32(rdx.wrapping_add(0xa4), ((rsi as u32 as u64)) as u32);
            // e69ea mov ebx,DWORD PTR [rbp-0x60]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // e69ed mov DWORD PTR [rdx+0xa8],ebx
            m.w32(rdx.wrapping_add(0xa8), ((rbx as u32 as u64)) as u32);
            // e69f3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e69f5 mov r8d,eax
            r8 = ((rax as u32 as u64)) as u32 as u64;
            // e69f8 je 0xe6a03
            if fl.e() { pc = 0xe6a03; continue; }
            pc = 0xe69fa;
        }
        0xe69fa => {
            // block e69fa
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e69fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e69fa mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // e69fe imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e6a01 jmp 0xe6a13
            pc = 0xe6a13; continue;
        }
        0xe6a03 => {
            // block e6a03
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6a03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6a03 shl ecx,0x4
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e6a06 mov eax,0xaaaaaaab
            rax = (0xaaaaaaabu64) as u32 as u64;
            // e6a0b imul rax,rcx
            { let (r, f) = imul::<64>(rax, rcx); rax = r; fl = f; }
            // e6a0f shr rax,0x22
            { let (r, f) = shr::<64>(rax, 0x22u64, fl); rax = r; fl = f; }
            pc = 0xe6a13;
        }
        0xe6a13 => {
            // block e6a13
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6a13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6a13 mov rbx,QWORD PTR [rbp-0x78]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // e6a17 imul ecx,esi,0xfffffff5
            { let (r, f) = imul::<32>((rsi as u32 as u64), 0xfffffff5u64); rcx = (r) as u32 as u64; fl = f; }
            // e6a1a add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6a1c mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e6a1e shr eax,0x1f
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e6a21 add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e6a23 sar eax,1
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e6a25 mov DWORD PTR [rdx+0xac],eax
            m.w32(rdx.wrapping_add(0xac), ((rax as u32 as u64)) as u32);
            // e6a2b test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // e6a2d je 0xe6a3f
            if fl.e() { pc = 0xe6a3f; continue; }
            pc = 0xe6a2f;
        }
        0xe6a2f => {
            // block e6a2f
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6a2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6a2f imul ebx,edi
            { let (r, f) = imul::<32>((rbx as u32 as u64), (rdi as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // e6a32 imul rcx,rbx,0x51eb851f
            { let (r, f) = imul::<64>(rbx, 0x51eb851fu64); rcx = r; fl = f; }
            // e6a39 shr rcx,0x25
            { let (r, f) = shr::<64>(rcx, 0x25u64, fl); rcx = r; fl = f; }
            // e6a3d jmp 0xe6a57
            pc = 0xe6a57; continue;
        }
        0xe6a3f => {
            // block e6a3f
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6a3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6a3f and rdi,0x1fffffff
            { let r = rdi & 0x1fffffffu64; let f = logic::<64>(r);
              rdi = r;
              fl = f; }
            // e6a46 shl rdi,0x3
            { let (r, f) = shl::<64>(rdi, 0x3u64, fl); rdi = r; fl = f; }
            // e6a4a mov ecx,0xcccccccd
            rcx = (0xcccccccdu64) as u32 as u64;
            // e6a4f imul rcx,rdi
            { let (r, f) = imul::<64>(rcx, rdi); rcx = r; fl = f; }
            // e6a53 shr rcx,0x23
            { let (r, f) = shr::<64>(rcx, 0x23u64, fl); rcx = r; fl = f; }
            pc = 0xe6a57;
        }
        0xe6a57 => {
            // block e6a57
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6a57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6a57 mov dil,0x1
            rdi = (rdi & !0xff) | ((0x1u64) as u8 as u64);
            // e6a5a mov DWORD PTR [rdx+0xb0],ecx
            m.w32(rdx.wrapping_add(0xb0), ((rcx as u32 as u64)) as u32);
            // e6a60 test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // e6a63 jne 0xe6651
            if fl.ne() { pc = 0xe6651; continue; }
            pc = 0xe6a69;
        }
        0xe6a69 => {
            // block e6a69
            #[cfg(feature = "trace")] eprintln!("f_000e65f6 pc e6a69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6a69 and eax,0xfffffff0
            { let r = (rax as u32 as u64) & 0xfffffff0u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6a6c mov DWORD PTR [rdx+0xac],eax
            m.w32(rdx.wrapping_add(0xac), ((rax as u32 as u64)) as u32);
            // e6a72 jmp 0xe6651
            pc = 0xe6651; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
