//! Mechanically lifted functions (scripts/lift_run.py), module v005. Each one is checked against the original
//! in the world harness (tests/world_v005.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x1a6e4 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a6e4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1a6e4;
    loop {
        match pc {
        0x1a6e4 => {
            // block 1a6e4
            #[cfg(feature = "trace")] eprintln!("f_0001a6e4 pc 1a6e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a6e4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a6e5 mov rbp,rsp
            rbp = rsp;
            // 1a6e8 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 1a6ec mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 1a6f0 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 1a6f3 je 0x1a70a
            if fl.e() { pc = 0x1a70a; continue; }
            pc = 0x1a6f5;
        }
        0x1a6f5 => {
            // block 1a6f5
            #[cfg(feature = "trace")] eprintln!("f_0001a6e4 pc 1a6f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a6f5 lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 1a6f9 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 1a6fc not rdx
            { let v = !rdx; rdx = v; }
            // 1a6ff and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 1a703 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 1a706 mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0x1a70a;
        }
        0x1a70a => {
            // block 1a70a
            #[cfg(feature = "trace")] eprintln!("f_0001a6e4 pc 1a70a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a70a mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 1a70d test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1a710 je 0x1a718
            if fl.e() { pc = 0x1a718; continue; }
            pc = 0x1a712;
        }
        0x1a712 => {
            // block 1a712
            #[cfg(feature = "trace")] eprintln!("f_0001a6e4 pc 1a712 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a712 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a713 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x1a718 => {
            // block 1a718
            #[cfg(feature = "trace")] eprintln!("f_0001a6e4 pc 1a718 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a718 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a719 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a994 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a994<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1a994;
    loop {
        match pc {
        0x1a994 => {
            // block 1a994
            #[cfg(feature = "trace")] eprintln!("f_0001a994 pc 1a994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a994 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a995 mov rbp,rsp
            rbp = rsp;
            // 1a998 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 1a99c mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 1a9a0 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 1a9a3 je 0x1a9ba
            if fl.e() { pc = 0x1a9ba; continue; }
            pc = 0x1a9a5;
        }
        0x1a9a5 => {
            // block 1a9a5
            #[cfg(feature = "trace")] eprintln!("f_0001a994 pc 1a9a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a9a5 lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 1a9a9 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 1a9ac not rdx
            { let v = !rdx; rdx = v; }
            // 1a9af and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 1a9b3 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 1a9b6 mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0x1a9ba;
        }
        0x1a9ba => {
            // block 1a9ba
            #[cfg(feature = "trace")] eprintln!("f_0001a994 pc 1a9ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a9ba mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 1a9bd test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1a9c0 je 0x1a9c8
            if fl.e() { pc = 0x1a9c8; continue; }
            pc = 0x1a9c2;
        }
        0x1a9c2 => {
            // block 1a9c2
            #[cfg(feature = "trace")] eprintln!("f_0001a994 pc 1a9c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a9c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a9c3 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x1a9c8 => {
            // block 1a9c8
            #[cfg(feature = "trace")] eprintln!("f_0001a994 pc 1a9c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a9c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a9c9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e428 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e428<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e428;
    loop {
        match pc {
        0x2e428 => {
            // block 2e428
            #[cfg(feature = "trace")] eprintln!("f_0002e428 pc 2e428 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e428 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e429 mov rbp,rsp
            rbp = rsp;
            // 2e42c sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2e430 mov QWORD PTR [rbp-0x8],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff8), (rsi) as u64);
            // 2e434 mov rax,QWORD PTR [rdi+0x18]
            rax = m.r64(rdi.wrapping_add(0x18)) as u64;
            // 2e438 cmp rax,QWORD PTR [rdi+0x20]
            fl = sub::<64>(rax, m.r64(rdi.wrapping_add(0x20)) as u64).1;
            // 2e43c je 0x2e448
            if fl.e() { pc = 0x2e448; continue; }
            pc = 0x2e43e;
        }
        0x2e43e => {
            // block 2e43e
            #[cfg(feature = "trace")] eprintln!("f_0002e428 pc 2e43e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e43e mov QWORD PTR [rax],rsi
            m.w64(rax, (rsi) as u64);
            // 2e441 add QWORD PTR [rdi+0x18],0x8
            { let (r, f) = add::<64>(m.r64(rdi.wrapping_add(0x18)) as u64, 0x8u64);
              m.w64(rdi.wrapping_add(0x18), (r) as u64);
              fl = f; }
            // 2e446 jmp 0x2e455
            pc = 0x2e455; continue;
        }
        0x2e448 => {
            // block 2e448
            #[cfg(feature = "trace")] eprintln!("f_0002e428 pc 2e448 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e448 add rdi,0x10
            { let (r, f) = add::<64>(rdi, 0x10u64);
              rdi = r;
              fl = f; }
            // 2e44c lea rsi,[rbp-0x8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff8);
            // 2e450 call 0xebb04
            { let r = { let t = m.env.stub_va(0x2d1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e455;
        }
        0x2e455 => {
            // block 2e455
            #[cfg(feature = "trace")] eprintln!("f_0002e428 pc 2e455 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e455 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2e457 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2e45b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e45c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ee6a size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002ee6a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2ee6a;
    loop {
        match pc {
        0x2ee6a => {
            // block 2ee6a
            #[cfg(feature = "trace")] eprintln!("f_0002ee6a pc 2ee6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ee6a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ee6b mov rbp,rsp
            rbp = rsp;
            // 2ee6e mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2ee72 mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 2ee76 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 2ee79 je 0x2ee90
            if fl.e() { pc = 0x2ee90; continue; }
            pc = 0x2ee7b;
        }
        0x2ee7b => {
            // block 2ee7b
            #[cfg(feature = "trace")] eprintln!("f_0002ee6a pc 2ee7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ee7b lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 2ee7f sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 2ee82 not rdx
            { let v = !rdx; rdx = v; }
            // 2ee85 and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 2ee89 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 2ee8c mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0x2ee90;
        }
        0x2ee90 => {
            // block 2ee90
            #[cfg(feature = "trace")] eprintln!("f_0002ee6a pc 2ee90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ee90 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 2ee93 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2ee96 je 0x2ee9e
            if fl.e() { pc = 0x2ee9e; continue; }
            pc = 0x2ee98;
        }
        0x2ee98 => {
            // block 2ee98
            #[cfg(feature = "trace")] eprintln!("f_0002ee6a pc 2ee98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ee98 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ee99 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x2ee9e => {
            // block 2ee9e
            #[cfg(feature = "trace")] eprintln!("f_0002ee6a pc 2ee9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ee9e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ee9f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44d32 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044d32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44d32;
    loop {
        match pc {
        0x44d32 => {
            // block 44d32
            #[cfg(feature = "trace")] eprintln!("f_00044d32 pc 44d32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44d32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44d33 mov rbp,rsp
            rbp = rsp;
            // 44d36 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44d3a mov r8,rdx
            r8 = rdx;
            // 44d3d mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // 44d40 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 44d43 mov DWORD PTR [rsp],ecx
            m.w32(rsp, ((rcx as u32 as u64)) as u32);
            // 44d46 mov DWORD PTR [rsp+0x8],0x1
            m.w32(rsp.wrapping_add(0x8), (0x1u64) as u32);
            // 44d4e mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 44d53 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 44d58 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // 44d5b mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // 44d5e call QWORD PTR [rax+0x58]
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44d61 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44d65 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44d66 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x652d4 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000652d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x652d4;
    loop {
        match pc {
        0x652d4 => {
            // block 652d4
            #[cfg(feature = "trace")] eprintln!("f_000652d4 pc 652d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 652d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 652d5 mov rbp,rsp
            rbp = rsp;
            // 652d8 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 652dc mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 652e0 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 652e3 je 0x652fa
            if fl.e() { pc = 0x652fa; continue; }
            pc = 0x652e5;
        }
        0x652e5 => {
            // block 652e5
            #[cfg(feature = "trace")] eprintln!("f_000652d4 pc 652e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 652e5 lea rdx,[rax-0x2]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffe);
            // 652e9 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 652ec not rdx
            { let v = !rdx; rdx = v; }
            // 652ef and rdx,0xfffffffffffffffe
            { let r = rdx & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 652f3 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 652f6 mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0x652fa;
        }
        0x652fa => {
            // block 652fa
            #[cfg(feature = "trace")] eprintln!("f_000652d4 pc 652fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 652fa mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 652fd test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 65300 je 0x65308
            if fl.e() { pc = 0x65308; continue; }
            pc = 0x65302;
        }
        0x65302 => {
            // block 65302
            #[cfg(feature = "trace")] eprintln!("f_000652d4 pc 65302 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65302 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 65303 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x65308 => {
            // block 65308
            #[cfg(feature = "trace")] eprintln!("f_000652d4 pc 65308 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65308 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 65309 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e0b8 size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e0b8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e0b8;
    loop {
        match pc {
        0x2e0b8 => {
            // block 2e0b8
            #[cfg(feature = "trace")] eprintln!("f_0002e0b8 pc 2e0b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e0b8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e0b9 mov rbp,rsp
            rbp = rsp;
            // 2e0bc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e0be push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e0c0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e0c2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e0c3 mov r14,rcx
            r14 = rcx;
            // 2e0c6 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 2e0c9 mov r12,rsi
            r12 = rsi;
            // 2e0cc mov rbx,rdi
            rbx = rdi;
            // 2e0cf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2e0d2 call QWORD PTR [rax+0xa8]
            { let r = { let t = m.r64(rax.wrapping_add(0xa8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e0d8 mov QWORD PTR [rbx+0x8],r12
            m.w64(rbx.wrapping_add(0x8), (r12) as u64);
            // 2e0dc mov BYTE PTR [rbx+0x60],r15b
            m.w8(rbx.wrapping_add(0x60), ((r15 as u8 as u64)) as u8);
            // 2e0e0 mov QWORD PTR [rbx+0x68],r14
            m.w64(rbx.wrapping_add(0x68), (r14) as u64);
            // 2e0e4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e0e6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e0e7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e0e9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e0eb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e0ed pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e0ee ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x46b68 size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00046b68<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x46b68;
    loop {
        match pc {
        0x46b68 => {
            // block 46b68
            #[cfg(feature = "trace")] eprintln!("f_00046b68 pc 46b68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46b68 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 46b69 mov rbp,rsp
            rbp = rsp;
            // 46b6c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 46b6d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 46b6e mov rbx,rdi
            rbx = rdi;
            // 46b71 cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // 46b75 je 0x46b97
            if fl.e() { pc = 0x46b97; continue; }
            pc = 0x46b77;
        }
        0x46b77 => {
            // block 46b77
            #[cfg(feature = "trace")] eprintln!("f_00046b68 pc 46b77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46b77 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 46b7a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 46b7c mov rdi,rbx
            rdi = rbx;
            // 46b7f call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46b85 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 46b87 je 0x46b97
            if fl.e() { pc = 0x46b97; continue; }
            pc = 0x46b89;
        }
        0x46b89 => {
            // block 46b89
            #[cfg(feature = "trace")] eprintln!("f_00046b68 pc 46b89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46b89 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 46b8c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 46b8e mov rdi,rbx
            rdi = rbx;
            // 46b91 call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x46b97;
        }
        0x46b97 => {
            // block 46b97
            #[cfg(feature = "trace")] eprintln!("f_00046b68 pc 46b97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46b97 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 46b99 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 46b9d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 46b9e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 46b9f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2decc size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002decc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2decc;
    loop {
        match pc {
        0x2decc => {
            // block 2decc
            #[cfg(feature = "trace")] eprintln!("f_0002decc pc 2decc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2decc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2decd mov rbp,rsp
            rbp = rsp;
            // 2ded0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2ded1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2ded2 mov rbx,rdi
            rbx = rdi;
            // 2ded5 lea rdi,[rip+0x11920c] # 0x1470e8
            rdi = m.env.data_va(374965);
            // 2dedc lea rsi,[rip+0xfffffffffffffe63] # 0x2dd46
            rsi = m.env.code_va(0x2dd46);
            // 2dee3 call 0xec266
            { let r = { let t = m.env.stub_va(0xa33); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dee8 mov rax,QWORD PTR [rip+0x193b51] # 0x1c1a40
            rax = m.r64(m.env.data_va(877069)) as u64;
            // 2deef test rax,rax
            fl = logic::<64>(rax & rax);
            // 2def2 je 0x2deff
            if fl.e() { pc = 0x2deff; continue; }
            pc = 0x2def4;
        }
        0x2def4 => {
            // block 2def4
            #[cfg(feature = "trace")] eprintln!("f_0002decc pc 2def4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2def4 mov rdi,rbx
            rdi = rbx;
            // 2def7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2defb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2defc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2defd jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x2deff => {
            // block 2deff
            #[cfg(feature = "trace")] eprintln!("f_0002decc pc 2deff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2deff xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2df01 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2df05 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2df06 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2df07 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42eb8 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042eb8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x42eb8;
    loop {
        match pc {
        0x42eb8 => {
            // block 42eb8
            #[cfg(feature = "trace")] eprintln!("f_00042eb8 pc 42eb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42eb8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42eb9 mov rbp,rsp
            rbp = rsp;
            // 42ebc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42ebe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42ebf mov r14,rdx
            r14 = rdx;
            // 42ec2 mov rbx,rdi
            rbx = rdi;
            // 42ec5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42ec8 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42ece test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42ed0 je 0x42eed
            if fl.e() { pc = 0x42eed; continue; }
            pc = 0x42ed2;
        }
        0x42ed2 => {
            // block 42ed2
            #[cfg(feature = "trace")] eprintln!("f_00042eb8 pc 42ed2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42ed2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42ed5 mov esi,0x903
            rsi = (0x903u64) as u32 as u64;
            // 42eda mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42edf xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42ee2 mov rdi,rbx
            rdi = rbx;
            // 42ee5 mov rdx,r14
            rdx = r14;
            // 42ee8 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42eeb jmp 0x42eef
            pc = 0x42eef; continue;
        }
        0x42eed => {
            // block 42eed
            #[cfg(feature = "trace")] eprintln!("f_00042eb8 pc 42eed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42eed xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42eef;
        }
        0x42eef => {
            // block 42eef
            #[cfg(feature = "trace")] eprintln!("f_00042eb8 pc 42eef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42eef pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42ef0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42ef2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42ef3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68550 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068550<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68550;
    loop {
        match pc {
        0x68550 => {
            // block 68550
            #[cfg(feature = "trace")] eprintln!("f_00068550 pc 68550 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68550 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68551 mov rbp,rsp
            rbp = rsp;
            // 68554 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 68558 mov DWORD PTR [rbp-0x8],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff8), ((rdi as u32 as u64)) as u32);
            // 6855b mov DWORD PTR [rbp-0x4],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), ((rsi as u32 as u64)) as u32);
            // 6855e lea rdi,[rip+0x160013] # 0x1c8578
            rdi = m.env.data_va(904517);
            // 68565 lea rsi,[rbp-0x8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff8);
            // 68569 call 0xebbdc
            { let r = { let t = m.env.stub_va(0x3a9); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6856e mov rcx,rax
            rcx = rax;
            // 68571 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 68576 lea rdx,[rip+0x160003] # 0x1c8580
            rdx = m.env.data_va(904525);
            // 6857d cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 68580 je 0x68585
            if fl.e() { pc = 0x68585; continue; }
            pc = 0x68582;
        }
        0x68582 => {
            // block 68582
            #[cfg(feature = "trace")] eprintln!("f_00068550 pc 68582 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68582 mov eax,DWORD PTR [rcx+0x24]
            rax = (m.r32(rcx.wrapping_add(0x24)) as u64) as u32 as u64;
            pc = 0x68585;
        }
        0x68585 => {
            // block 68585
            #[cfg(feature = "trace")] eprintln!("f_00068550 pc 68585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68585 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 68589 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6858a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd70d4 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d70d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd70d4;
    loop {
        match pc {
        0xd70d4 => {
            // block d70d4
            #[cfg(feature = "trace")] eprintln!("f_000d70d4 pc d70d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d70d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d70d5 mov rbp,rsp
            rbp = rsp;
            // d70d8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d70d9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d70da mov rbx,rdi
            rbx = rdi;
            // d70dd mov DWORD PTR [rbx+0x370],0x0
            m.w32(rbx.wrapping_add(0x370), (0x0u64) as u32);
            // d70e7 lea rax,[rip+0x61c56] # 0x138d44
            rax = m.env.data_va(316689);
            // d70ee mov rdi,rsi
            rdi = rsi;
            // d70f1 mov rsi,rax
            rsi = rax;
            // d70f4 call 0xec2d2
            { let r = { let t = m.env.stub_va(0xa9f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d70f9 test rax,rax
            fl = logic::<64>(rax & rax);
            // d70fc setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // d70ff movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // d7102 mov DWORD PTR [rbx+0x370],eax
            m.w32(rbx.wrapping_add(0x370), ((rax as u32 as u64)) as u32);
            // d7108 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d710c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d710d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d710e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x437a2 size=62
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000437a2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x437a2;
    loop {
        match pc {
        0x437a2 => {
            // block 437a2
            #[cfg(feature = "trace")] eprintln!("f_000437a2 pc 437a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 437a2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 437a3 mov rbp,rsp
            rbp = rsp;
            // 437a6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 437a8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 437a9 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 437ac mov rbx,rdi
            rbx = rdi;
            // 437af mov edi,edx
            rdi = ((rdx as u32 as u64)) as u32 as u64;
            // 437b1 call 0x5c575
            { let r = { let t = m.env.code_va(0x5c575); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 437b6 shl eax,0x4
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 437b9 or eax,r14d
            { let r = (rax as u32 as u64) | (r14 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 437bc mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 437bf mov r9,QWORD PTR [rcx+0x30]
            r9 = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 437c3 mov esi,0xbe
            rsi = (0xbeu64) as u32 as u64;
            // 437c8 mov ecx,0x3f0000
            rcx = (0x3f0000u64) as u32 as u64;
            // 437cd mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 437d3 mov rdi,rbx
            rdi = rbx;
            // 437d6 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 437d8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 437d9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 437db pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 437dc jmp r9
            { let r = { let t = r9; m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1038c size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001038c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1038c;
    loop {
        match pc {
        0x1038c => {
            // block 1038c
            #[cfg(feature = "trace")] eprintln!("f_0001038c pc 1038c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1038c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1038d mov rbp,rsp
            rbp = rsp;
            // 10390 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10392 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10394 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10395 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 10396 mov r14,rdi
            r14 = rdi;
            // 10399 call 0xeb834
            { let r = { let t = m.env.stub_va(0x1); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1039e mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 103a2 mov r15,QWORD PTR [r14+0x10]
            r15 = m.r64(r14.wrapping_add(0x10)) as u64;
            // 103a6 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 103a9 je 0x103bc
            if fl.e() { pc = 0x103bc; continue; }
            pc = 0x103ab;
        }
        0x103ab => {
            // block 103ab
            #[cfg(feature = "trace")] eprintln!("f_0001038c pc 103ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 103ab mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 103ae call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 103b3 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 103b7 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 103ba jne 0x103ab
            if fl.ne() { pc = 0x103ab; continue; }
            pc = 0x103bc;
        }
        0x103bc => {
            // block 103bc
            #[cfg(feature = "trace")] eprintln!("f_0001038c pc 103bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 103bc mov rdi,r14
            rdi = r14;
            // 103bf add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 103c3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 103c4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 103c6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 103c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 103c9 jmp 0xeb84c
            { let r = { let t = m.env.stub_va(0x19); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42c26 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042c26<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x42c26;
    loop {
        match pc {
        0x42c26 => {
            // block 42c26
            #[cfg(feature = "trace")] eprintln!("f_00042c26 pc 42c26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42c26 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42c27 mov rbp,rsp
            rbp = rsp;
            // 42c2a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42c2c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42c2d mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 42c30 mov rbx,rdi
            rbx = rdi;
            // 42c33 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42c36 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42c3c mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 42c3e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 42c40 cmp r14d,0x1ffff
            fl = sub::<32>((r14 as u32 as u64), 0x1ffffu64).1;
            // 42c47 ja 0x42c67
            if fl.a() { pc = 0x42c67; continue; }
            pc = 0x42c49;
        }
        0x42c49 => {
            // block 42c49
            #[cfg(feature = "trace")] eprintln!("f_00042c26 pc 42c49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42c49 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 42c4c jne 0x42c67
            if fl.ne() { pc = 0x42c67; continue; }
            pc = 0x42c4e;
        }
        0x42c4e => {
            // block 42c4e
            #[cfg(feature = "trace")] eprintln!("f_00042c26 pc 42c4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42c4e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42c51 mov esi,0x901
            rsi = (0x901u64) as u32 as u64;
            // 42c56 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42c5b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42c5e mov rdi,rbx
            rdi = rbx;
            // 42c61 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42c64 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x42c67;
        }
        0x42c67 => {
            // block 42c67
            #[cfg(feature = "trace")] eprintln!("f_00042c26 pc 42c67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42c67 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42c68 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42c6a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42c6b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe09e8 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e09e8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe09e8;
    loop {
        match pc {
        0xe09e8 => {
            // block e09e8
            #[cfg(feature = "trace")] eprintln!("f_000e09e8 pc e09e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e09e8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e09e9 mov rbp,rsp
            rbp = rsp;
            // e09ec push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e09ee push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e09ef mov rbx,rsi
            rbx = rsi;
            // e09f2 mov r14,rdi
            r14 = rdi;
            // e09f5 mov rdi,QWORD PTR [r14+0x20]
            rdi = m.r64(r14.wrapping_add(0x20)) as u64;
            // e09f9 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e09fc je 0xe0a03
            if fl.e() { pc = 0xe0a03; continue; }
            pc = 0xe09fe;
        }
        0xe09fe => {
            // block e09fe
            #[cfg(feature = "trace")] eprintln!("f_000e09e8 pc e09fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e09fe call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe0a03;
        }
        0xe0a03 => {
            // block e0a03
            #[cfg(feature = "trace")] eprintln!("f_000e09e8 pc e0a03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a03 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // e0a06 je 0xe0a18
            if fl.e() { pc = 0xe0a18; continue; }
            pc = 0xe0a08;
        }
        0xe0a08 => {
            // block e0a08
            #[cfg(feature = "trace")] eprintln!("f_000e09e8 pc e0a08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a08 cmp BYTE PTR [rbx],0x0
            fl = sub::<8>(m.r8(rbx) as u64, 0x0u64).1;
            // e0a0b lea rdi,[rip+0x582b6] # 0x138cc8
            rdi = m.env.data_va(316565);
            // e0a12 cmovne rdi,rbx
            if fl.ne() { rdi = rbx; }
            // e0a16 jmp 0xe0a1f
            pc = 0xe0a1f; continue;
        }
        0xe0a18 => {
            // block e0a18
            #[cfg(feature = "trace")] eprintln!("f_000e09e8 pc e0a18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a18 lea rdi,[rip+0x582a9] # 0x138cc8
            rdi = m.env.data_va(316565);
            pc = 0xe0a1f;
        }
        0xe0a1f => {
            // block e0a1f
            #[cfg(feature = "trace")] eprintln!("f_000e09e8 pc e0a1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a1f call 0xec2ba
            { let r = { let t = m.env.stub_va(0xa87); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a24 mov QWORD PTR [r14+0x20],rax
            m.w64(r14.wrapping_add(0x20), (rax) as u64);
            // e0a28 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0a29 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0a2b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0a2c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1701a size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001701a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1701a;
    loop {
        match pc {
        0x1701a => {
            // block 1701a
            #[cfg(feature = "trace")] eprintln!("f_0001701a pc 1701a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1701a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1701b mov rbp,rsp
            rbp = rsp;
            // 1701e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 17020 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 17021 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 17023 mov r14,rdi
            r14 = rdi;
            // 17026 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 17028 je 0x17033
            if fl.e() { pc = 0x17033; continue; }
            pc = 0x1702a;
        }
        0x1702a => {
            // block 1702a
            #[cfg(feature = "trace")] eprintln!("f_0001701a pc 1702a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1702a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1702d mov rdi,r14
            rdi = r14;
            // 17030 call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17033;
        }
        0x17033 => {
            // block 17033
            #[cfg(feature = "trace")] eprintln!("f_0001701a pc 17033 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17033 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 17036 mov rdi,r14
            rdi = r14;
            // 17039 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1703c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 1703f mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 17043 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 17046 je 0x1705a
            if fl.e() { pc = 0x1705a; continue; }
            pc = 0x17048;
        }
        0x17048 => {
            // block 17048
            #[cfg(feature = "trace")] eprintln!("f_0001701a pc 17048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17048 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 1704a cmp DWORD PTR [rcx],ebx
            fl = sub::<32>(m.r32(rcx) as u64, (rbx as u32 as u64)).1;
            // 1704c je 0x1705c
            if fl.e() { pc = 0x1705c; continue; }
            pc = 0x1704e;
        }
        0x1704e => {
            // block 1704e
            #[cfg(feature = "trace")] eprintln!("f_0001701a pc 1704e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1704e add rcx,0xf8
            { let (r, f) = add::<64>(rcx, 0xf8u64);
              rcx = r;
              fl = f; }
            // 17055 cmp rdx,rcx
            fl = sub::<64>(rdx, rcx).1;
            // 17058 jne 0x17048
            if fl.ne() { pc = 0x17048; continue; }
            pc = 0x1705a;
        }
        0x1705a => {
            // block 1705a
            #[cfg(feature = "trace")] eprintln!("f_0001701a pc 1705a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1705a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x1705c;
        }
        0x1705c => {
            // block 1705c
            #[cfg(feature = "trace")] eprintln!("f_0001701a pc 1705c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1705c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1705d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1705f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17060 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dbac size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dbac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2dbac;
    loop {
        match pc {
        0x2dbac => {
            // block 2dbac
            #[cfg(feature = "trace")] eprintln!("f_0002dbac pc 2dbac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dbac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dbad mov rbp,rsp
            rbp = rsp;
            // 2dbb0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2dbb1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2dbb2 mov rbx,rdi
            rbx = rdi;
            // 2dbb5 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 2dbbd mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 2dbc4 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // 2dbc8 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // 2dbcb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2dbce cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // 2dbd2 je 0x2dbdd
            if fl.e() { pc = 0x2dbdd; continue; }
            pc = 0x2dbd4;
        }
        0x2dbd4 => {
            // block 2dbd4
            #[cfg(feature = "trace")] eprintln!("f_0002dbac pc 2dbd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dbd4 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 2dbd7 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 2dbda mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0x2dbdd;
        }
        0x2dbdd => {
            // block 2dbdd
            #[cfg(feature = "trace")] eprintln!("f_0002dbac pc 2dbdd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dbdd mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2dbe1 mov rsi,rcx
            rsi = rcx;
            // 2dbe4 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dbe9 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // 2dbed add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2dbf1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2dbf2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dbf3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4309e size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004309e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4309e;
    loop {
        match pc {
        0x4309e => {
            // block 4309e
            #[cfg(feature = "trace")] eprintln!("f_0004309e pc 4309e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4309e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4309f mov rbp,rsp
            rbp = rsp;
            // 430a2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 430a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 430a5 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 430a8 mov rbx,rdi
            rbx = rdi;
            // 430ab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 430ae call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 430b4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 430b6 je 0x430df
            if fl.e() { pc = 0x430df; continue; }
            pc = 0x430b8;
        }
        0x430b8 => {
            // block 430b8
            #[cfg(feature = "trace")] eprintln!("f_0004309e pc 430b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 430b8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 430bb test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 430be sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 430c1 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 430c4 lea edx,[rcx+rcx*2]
            rdx = (rcx.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // 430c7 mov esi,0x905
            rsi = (0x905u64) as u32 as u64;
            // 430cc mov ecx,0x30000
            rcx = (0x30000u64) as u32 as u64;
            // 430d1 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 430d7 mov rdi,rbx
            rdi = rbx;
            // 430da call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 430dd jmp 0x430e1
            pc = 0x430e1; continue;
        }
        0x430df => {
            // block 430df
            #[cfg(feature = "trace")] eprintln!("f_0004309e pc 430df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 430df xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x430e1;
        }
        0x430e1 => {
            // block 430e1
            #[cfg(feature = "trace")] eprintln!("f_0004309e pc 430e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 430e1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 430e2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 430e4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 430e5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x431ea size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000431ea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x431ea;
    loop {
        match pc {
        0x431ea => {
            // block 431ea
            #[cfg(feature = "trace")] eprintln!("f_000431ea pc 431ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 431ea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 431eb mov rbp,rsp
            rbp = rsp;
            // 431ee push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 431f0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 431f1 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 431f4 mov rbx,rdi
            rbx = rdi;
            // 431f7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 431fa call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43200 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43202 je 0x4322b
            if fl.e() { pc = 0x4322b; continue; }
            pc = 0x43204;
        }
        0x43204 => {
            // block 43204
            #[cfg(feature = "trace")] eprintln!("f_000431ea pc 43204 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43204 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 43207 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 4320a sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 4320d movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 43210 lea edx,[rcx+rcx*2]
            rdx = (rcx.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // 43213 mov esi,0x905
            rsi = (0x905u64) as u32 as u64;
            // 43218 mov ecx,0x300000
            rcx = (0x300000u64) as u32 as u64;
            // 4321d mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 43223 mov rdi,rbx
            rdi = rbx;
            // 43226 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43229 jmp 0x4322d
            pc = 0x4322d; continue;
        }
        0x4322b => {
            // block 4322b
            #[cfg(feature = "trace")] eprintln!("f_000431ea pc 4322b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4322b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4322d;
        }
        0x4322d => {
            // block 4322d
            #[cfg(feature = "trace")] eprintln!("f_000431ea pc 4322d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4322d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4322e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43230 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43231 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44bb8 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044bb8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44bb8;
    loop {
        match pc {
        0x44bb8 => {
            // block 44bb8
            #[cfg(feature = "trace")] eprintln!("f_00044bb8 pc 44bb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44bb8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44bb9 mov rbp,rsp
            rbp = rsp;
            // 44bbc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44bbd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 44bbe mov rbx,rdi
            rbx = rdi;
            // 44bc1 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 44bc9 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 44bd0 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // 44bd4 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // 44bd7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 44bda cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // 44bde je 0x44be9
            if fl.e() { pc = 0x44be9; continue; }
            pc = 0x44be0;
        }
        0x44be0 => {
            // block 44be0
            #[cfg(feature = "trace")] eprintln!("f_00044bb8 pc 44be0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44be0 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 44be3 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 44be6 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0x44be9;
        }
        0x44be9 => {
            // block 44be9
            #[cfg(feature = "trace")] eprintln!("f_00044bb8 pc 44be9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44be9 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 44bed mov rsi,rcx
            rsi = rcx;
            // 44bf0 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44bf5 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // 44bf9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 44bfd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44bfe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44bff ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x65630 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00065630<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x65630;
    loop {
        match pc {
        0x65630 => {
            // block 65630
            #[cfg(feature = "trace")] eprintln!("f_00065630 pc 65630 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65630 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 65631 mov rbp,rsp
            rbp = rsp;
            // 65634 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65635 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 65636 mov rbx,rdi
            rbx = rdi;
            // 65639 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 65641 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 65648 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // 6564c mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // 6564f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 65652 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // 65656 je 0x65661
            if fl.e() { pc = 0x65661; continue; }
            pc = 0x65658;
        }
        0x65658 => {
            // block 65658
            #[cfg(feature = "trace")] eprintln!("f_00065630 pc 65658 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65658 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 6565b mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 6565e mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0x65661;
        }
        0x65661 => {
            // block 65661
            #[cfg(feature = "trace")] eprintln!("f_00065630 pc 65661 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65661 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 65665 mov rsi,rcx
            rsi = rcx;
            // 65668 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6566d inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // 65671 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 65675 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 65676 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 65677 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb548c size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b548c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb548c;
    loop {
        match pc {
        0xb548c => {
            // block b548c
            #[cfg(feature = "trace")] eprintln!("f_000b548c pc b548c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b548c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b548d mov rbp,rsp
            rbp = rsp;
            // b5490 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5491 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b5492 mov rbx,rdi
            rbx = rdi;
            // b5495 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // b549d mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // b54a4 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // b54a8 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // b54ab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b54ae cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // b54b2 je 0xb54bd
            if fl.e() { pc = 0xb54bd; continue; }
            pc = 0xb54b4;
        }
        0xb54b4 => {
            // block b54b4
            #[cfg(feature = "trace")] eprintln!("f_000b548c pc b54b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b54b4 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b54b7 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b54ba mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xb54bd;
        }
        0xb54bd => {
            // block b54bd
            #[cfg(feature = "trace")] eprintln!("f_000b548c pc b54bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b54bd mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b54c1 mov rsi,rcx
            rsi = rcx;
            // b54c4 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b54c9 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // b54cd add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b54d1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b54d2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b54d3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb58ae size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b58ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb58ae;
    loop {
        match pc {
        0xb58ae => {
            // block b58ae
            #[cfg(feature = "trace")] eprintln!("f_000b58ae pc b58ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b58ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b58af mov rbp,rsp
            rbp = rsp;
            // b58b2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b58b3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b58b4 mov rbx,rdi
            rbx = rdi;
            // b58b7 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // b58bf mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // b58c6 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // b58ca mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // b58cd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b58d0 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // b58d4 je 0xb58df
            if fl.e() { pc = 0xb58df; continue; }
            pc = 0xb58d6;
        }
        0xb58d6 => {
            // block b58d6
            #[cfg(feature = "trace")] eprintln!("f_000b58ae pc b58d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b58d6 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b58d9 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b58dc mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xb58df;
        }
        0xb58df => {
            // block b58df
            #[cfg(feature = "trace")] eprintln!("f_000b58ae pc b58df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b58df mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b58e3 mov rsi,rcx
            rsi = rcx;
            // b58e6 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b58eb inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // b58ef add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b58f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b58f4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b58f5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5998 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5998<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5998;
    loop {
        match pc {
        0xb5998 => {
            // block b5998
            #[cfg(feature = "trace")] eprintln!("f_000b5998 pc b5998 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5998 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5999 mov rbp,rsp
            rbp = rsp;
            // b599c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b599d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b599e mov rbx,rdi
            rbx = rdi;
            // b59a1 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // b59a9 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // b59b0 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // b59b4 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // b59b7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b59ba cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // b59be je 0xb59c9
            if fl.e() { pc = 0xb59c9; continue; }
            pc = 0xb59c0;
        }
        0xb59c0 => {
            // block b59c0
            #[cfg(feature = "trace")] eprintln!("f_000b5998 pc b59c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b59c0 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b59c3 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b59c6 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xb59c9;
        }
        0xb59c9 => {
            // block b59c9
            #[cfg(feature = "trace")] eprintln!("f_000b5998 pc b59c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b59c9 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b59cd mov rsi,rcx
            rsi = rcx;
            // b59d0 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b59d5 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // b59d9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b59dd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b59de pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b59df ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6580 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6580<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb6580;
    loop {
        match pc {
        0xb6580 => {
            // block b6580
            #[cfg(feature = "trace")] eprintln!("f_000b6580 pc b6580 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6580 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6581 mov rbp,rsp
            rbp = rsp;
            // b6584 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b6585 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b6586 mov rbx,rdi
            rbx = rdi;
            // b6589 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // b6591 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // b6598 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // b659c mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // b659f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b65a2 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // b65a6 je 0xb65b1
            if fl.e() { pc = 0xb65b1; continue; }
            pc = 0xb65a8;
        }
        0xb65a8 => {
            // block b65a8
            #[cfg(feature = "trace")] eprintln!("f_000b6580 pc b65a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b65a8 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b65ab mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b65ae mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xb65b1;
        }
        0xb65b1 => {
            // block b65b1
            #[cfg(feature = "trace")] eprintln!("f_000b6580 pc b65b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b65b1 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b65b5 mov rsi,rcx
            rsi = rcx;
            // b65b8 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b65bd inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // b65c1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b65c5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b65c6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b65c7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba73a size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba73a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba73a;
    loop {
        match pc {
        0xba73a => {
            // block ba73a
            #[cfg(feature = "trace")] eprintln!("f_000ba73a pc ba73a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba73a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba73b mov rbp,rsp
            rbp = rsp;
            // ba73e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba73f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ba740 mov rbx,rdi
            rbx = rdi;
            // ba743 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // ba74b mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // ba752 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // ba756 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // ba759 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ba75c cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // ba760 je 0xba76b
            if fl.e() { pc = 0xba76b; continue; }
            pc = 0xba762;
        }
        0xba762 => {
            // block ba762
            #[cfg(feature = "trace")] eprintln!("f_000ba73a pc ba762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba762 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // ba765 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // ba768 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xba76b;
        }
        0xba76b => {
            // block ba76b
            #[cfg(feature = "trace")] eprintln!("f_000ba73a pc ba76b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba76b mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // ba76f mov rsi,rcx
            rsi = rcx;
            // ba772 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba777 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // ba77b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ba77f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba780 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba781 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4c24 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4c24<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4c24;
    loop {
        match pc {
        0xc4c24 => {
            // block c4c24
            #[cfg(feature = "trace")] eprintln!("f_000c4c24 pc c4c24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4c24 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4c25 mov rbp,rsp
            rbp = rsp;
            // c4c28 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4c29 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c4c2a mov rbx,rdi
            rbx = rdi;
            // c4c2d mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c4c35 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c4c3c mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c4c40 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c4c43 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c4c46 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c4c4a je 0xc4c55
            if fl.e() { pc = 0xc4c55; continue; }
            pc = 0xc4c4c;
        }
        0xc4c4c => {
            // block c4c4c
            #[cfg(feature = "trace")] eprintln!("f_000c4c24 pc c4c4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4c4c mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c4c4f mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c4c52 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc4c55;
        }
        0xc4c55 => {
            // block c4c55
            #[cfg(feature = "trace")] eprintln!("f_000c4c24 pc c4c55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4c55 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4c59 mov rsi,rcx
            rsi = rcx;
            // c4c5c call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4c61 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c4c65 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c4c69 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4c6a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4c6b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4e06 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4e06<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4e06;
    loop {
        match pc {
        0xc4e06 => {
            // block c4e06
            #[cfg(feature = "trace")] eprintln!("f_000c4e06 pc c4e06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4e06 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4e07 mov rbp,rsp
            rbp = rsp;
            // c4e0a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4e0b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c4e0c mov rbx,rdi
            rbx = rdi;
            // c4e0f mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c4e17 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c4e1e mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c4e22 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c4e25 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c4e28 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c4e2c je 0xc4e37
            if fl.e() { pc = 0xc4e37; continue; }
            pc = 0xc4e2e;
        }
        0xc4e2e => {
            // block c4e2e
            #[cfg(feature = "trace")] eprintln!("f_000c4e06 pc c4e2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4e2e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c4e31 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c4e34 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc4e37;
        }
        0xc4e37 => {
            // block c4e37
            #[cfg(feature = "trace")] eprintln!("f_000c4e06 pc c4e37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4e37 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4e3b mov rsi,rcx
            rsi = rcx;
            // c4e3e call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4e43 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c4e47 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c4e4b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4e4c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4e4d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc50d0 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c50d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc50d0;
    loop {
        match pc {
        0xc50d0 => {
            // block c50d0
            #[cfg(feature = "trace")] eprintln!("f_000c50d0 pc c50d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c50d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c50d1 mov rbp,rsp
            rbp = rsp;
            // c50d4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c50d5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c50d6 mov rbx,rdi
            rbx = rdi;
            // c50d9 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c50e1 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c50e8 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c50ec mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c50ef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c50f2 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c50f6 je 0xc5101
            if fl.e() { pc = 0xc5101; continue; }
            pc = 0xc50f8;
        }
        0xc50f8 => {
            // block c50f8
            #[cfg(feature = "trace")] eprintln!("f_000c50d0 pc c50f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c50f8 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c50fb mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c50fe mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc5101;
        }
        0xc5101 => {
            // block c5101
            #[cfg(feature = "trace")] eprintln!("f_000c50d0 pc c5101 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5101 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c5105 mov rsi,rcx
            rsi = rcx;
            // c5108 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c510d inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c5111 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c5115 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5116 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c5117 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc53a4 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c53a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc53a4;
    loop {
        match pc {
        0xc53a4 => {
            // block c53a4
            #[cfg(feature = "trace")] eprintln!("f_000c53a4 pc c53a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c53a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c53a5 mov rbp,rsp
            rbp = rsp;
            // c53a8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c53a9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c53aa mov rbx,rdi
            rbx = rdi;
            // c53ad mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c53b5 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c53bc mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c53c0 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c53c3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c53c6 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c53ca je 0xc53d5
            if fl.e() { pc = 0xc53d5; continue; }
            pc = 0xc53cc;
        }
        0xc53cc => {
            // block c53cc
            #[cfg(feature = "trace")] eprintln!("f_000c53a4 pc c53cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c53cc mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c53cf mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c53d2 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc53d5;
        }
        0xc53d5 => {
            // block c53d5
            #[cfg(feature = "trace")] eprintln!("f_000c53a4 pc c53d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c53d5 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c53d9 mov rsi,rcx
            rsi = rcx;
            // c53dc call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c53e1 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c53e5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c53e9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c53ea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c53eb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc5678 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c5678<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc5678;
    loop {
        match pc {
        0xc5678 => {
            // block c5678
            #[cfg(feature = "trace")] eprintln!("f_000c5678 pc c5678 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5678 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c5679 mov rbp,rsp
            rbp = rsp;
            // c567c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c567d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c567e mov rbx,rdi
            rbx = rdi;
            // c5681 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c5689 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c5690 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c5694 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c5697 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c569a cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c569e je 0xc56a9
            if fl.e() { pc = 0xc56a9; continue; }
            pc = 0xc56a0;
        }
        0xc56a0 => {
            // block c56a0
            #[cfg(feature = "trace")] eprintln!("f_000c5678 pc c56a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56a0 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c56a3 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c56a6 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc56a9;
        }
        0xc56a9 => {
            // block c56a9
            #[cfg(feature = "trace")] eprintln!("f_000c5678 pc c56a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56a9 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c56ad mov rsi,rcx
            rsi = rcx;
            // c56b0 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c56b5 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c56b9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c56bd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c56be pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c56bf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc57d8 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c57d8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc57d8;
    loop {
        match pc {
        0xc57d8 => {
            // block c57d8
            #[cfg(feature = "trace")] eprintln!("f_000c57d8 pc c57d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57d8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c57d9 mov rbp,rsp
            rbp = rsp;
            // c57dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c57dd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c57de mov rbx,rdi
            rbx = rdi;
            // c57e1 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c57e9 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c57f0 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c57f4 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c57f7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c57fa cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c57fe je 0xc5809
            if fl.e() { pc = 0xc5809; continue; }
            pc = 0xc5800;
        }
        0xc5800 => {
            // block c5800
            #[cfg(feature = "trace")] eprintln!("f_000c57d8 pc c5800 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5800 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c5803 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c5806 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc5809;
        }
        0xc5809 => {
            // block c5809
            #[cfg(feature = "trace")] eprintln!("f_000c57d8 pc c5809 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5809 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c580d mov rsi,rcx
            rsi = rcx;
            // c5810 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5815 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c5819 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c581d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c581e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c581f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc5938 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c5938<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc5938;
    loop {
        match pc {
        0xc5938 => {
            // block c5938
            #[cfg(feature = "trace")] eprintln!("f_000c5938 pc c5938 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5938 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c5939 mov rbp,rsp
            rbp = rsp;
            // c593c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c593d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c593e mov rbx,rdi
            rbx = rdi;
            // c5941 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c5949 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c5950 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c5954 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c5957 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c595a cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c595e je 0xc5969
            if fl.e() { pc = 0xc5969; continue; }
            pc = 0xc5960;
        }
        0xc5960 => {
            // block c5960
            #[cfg(feature = "trace")] eprintln!("f_000c5938 pc c5960 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5960 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c5963 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c5966 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc5969;
        }
        0xc5969 => {
            // block c5969
            #[cfg(feature = "trace")] eprintln!("f_000c5938 pc c5969 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5969 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c596d mov rsi,rcx
            rsi = rcx;
            // c5970 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5975 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c5979 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c597d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c597e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c597f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4fc6 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004fc6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4fc6;
    loop {
        match pc {
        0x4fc6 => {
            // block 4fc6
            #[cfg(feature = "trace")] eprintln!("f_00004fc6 pc 4fc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fc6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4fc7 mov rbp,rsp
            rbp = rsp;
            // 4fca push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4fcc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4fcd mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 4fcf mov r14,rdi
            r14 = rdi;
            // 4fd2 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4fd5 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fdb mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 4fdd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4fdf cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 4fe2 ja 0x500b
            if fl.a() { pc = 0x500b; continue; }
            pc = 0x4fe4;
        }
        0x4fe4 => {
            // block 4fe4
            #[cfg(feature = "trace")] eprintln!("f_00004fc6 pc 4fe4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fe4 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 4fe7 je 0x500b
            if fl.e() { pc = 0x500b; continue; }
            pc = 0x4fe9;
        }
        0x4fe9 => {
            // block 4fe9
            #[cfg(feature = "trace")] eprintln!("f_00004fc6 pc 4fe9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fe9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4fec mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 4fee lea rdx,[rip+0xfa2fb] # 0xff2f0
            rdx = m.env.data_va(80573);
            // 4ff5 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4ff8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4ffa mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 4fff mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            // 5005 mov rdi,r14
            rdi = r14;
            // 5008 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x500b;
        }
        0x500b => {
            // block 500b
            #[cfg(feature = "trace")] eprintln!("f_00004fc6 pc 500b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 500b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 500c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 500e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 500f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d946 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d946<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2d946;
    loop {
        match pc {
        0x2d946 => {
            // block 2d946
            #[cfg(feature = "trace")] eprintln!("f_0002d946 pc 2d946 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d946 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d947 mov rbp,rsp
            rbp = rsp;
            // 2d94a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d94b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2d94c mov rbx,rdi
            rbx = rdi;
            // 2d94f mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 2d957 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 2d95e mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // 2d962 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // 2d965 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2d968 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // 2d96c je 0x2d977
            if fl.e() { pc = 0x2d977; continue; }
            pc = 0x2d96e;
        }
        0x2d96e => {
            // block 2d96e
            #[cfg(feature = "trace")] eprintln!("f_0002d946 pc 2d96e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d96e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 2d971 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 2d974 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0x2d977;
        }
        0x2d977 => {
            // block 2d977
            #[cfg(feature = "trace")] eprintln!("f_0002d946 pc 2d977 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d977 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2d97b mov rsi,rcx
            rsi = rcx;
            // 2d97e call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d983 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // 2d987 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2d98b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d98c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d98d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x62f8c size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00062f8c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x62f8c;
    loop {
        match pc {
        0x62f8c => {
            // block 62f8c
            #[cfg(feature = "trace")] eprintln!("f_00062f8c pc 62f8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f8c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 62f8d mov rbp,rsp
            rbp = rsp;
            // 62f90 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 62f92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 62f93 mov r14,rdi
            r14 = rdi;
            // 62f96 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 62f98 cmp esi,0x28
            fl = sub::<32>((rsi as u32 as u64), 0x28u64).1;
            // 62f9b ja 0x62faa
            if fl.a() { pc = 0x62faa; continue; }
            pc = 0x62f9d;
        }
        0x62f9d => {
            // block 62f9d
            #[cfg(feature = "trace")] eprintln!("f_00062f8c pc 62f9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f9d mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 62f9f lea rcx,[rip+0xdf32a] # 0x1422d0
            rcx = m.env.data_va(354973);
            // 62fa6 mov rax,QWORD PTR [rcx+rax*8]
            rax = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            pc = 0x62faa;
        }
        0x62faa => {
            // block 62faa
            #[cfg(feature = "trace")] eprintln!("f_00062f8c pc 62faa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62faa test rax,rax
            fl = logic::<64>(rax & rax);
            // 62fad lea rbx,[rip+0xd5d14] # 0x138cc8
            rbx = m.env.data_va(316565);
            // 62fb4 cmovne rbx,rax
            if fl.ne() { rbx = rax; }
            // 62fb8 mov rdi,rbx
            rdi = rbx;
            // 62fbb call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 62fc0 mov rdi,r14
            rdi = r14;
            // 62fc3 mov rsi,rbx
            rsi = rbx;
            // 62fc6 mov rdx,rax
            rdx = rax;
            // 62fc9 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 62fce mov rax,r14
            rax = r14;
            // 62fd1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 62fd2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 62fd4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 62fd5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4a02 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4a02<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd4a02;
    loop {
        match pc {
        0xd4a02 => {
            // block d4a02
            #[cfg(feature = "trace")] eprintln!("f_000d4a02 pc d4a02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4a02 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4a03 mov rbp,rsp
            rbp = rsp;
            // d4a06 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4a07 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d4a08 mov rbx,rdi
            rbx = rdi;
            // d4a0b mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // d4a13 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // d4a1a mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // d4a1e mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // d4a21 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d4a24 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // d4a28 je 0xd4a33
            if fl.e() { pc = 0xd4a33; continue; }
            pc = 0xd4a2a;
        }
        0xd4a2a => {
            // block d4a2a
            #[cfg(feature = "trace")] eprintln!("f_000d4a02 pc d4a2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4a2a mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d4a2d mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // d4a30 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xd4a33;
        }
        0xd4a33 => {
            // block d4a33
            #[cfg(feature = "trace")] eprintln!("f_000d4a02 pc d4a33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4a33 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // d4a37 mov rsi,rcx
            rsi = rcx;
            // d4a3a call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4a3f inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // d4a43 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d4a47 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4a48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4a49 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe42f6 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e42f6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe42f6;
    loop {
        match pc {
        0xe42f6 => {
            // block e42f6
            #[cfg(feature = "trace")] eprintln!("f_000e42f6 pc e42f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e42f6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e42f7 mov rbp,rsp
            rbp = rsp;
            // e42fa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e42fb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e42fc mov rbx,rdi
            rbx = rdi;
            // e42ff mov QWORD PTR [rbx],rsi
            m.w64(rbx, (rsi) as u64);
            // e4302 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // e430a mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // e4312 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // e431a mov DWORD PTR [rbx+0x30],0x41c00
            m.w32(rbx.wrapping_add(0x30), (0x41c00u64) as u32);
            // e4321 lea rdi,[rbx+0x34]
            rdi = rbx.wrapping_add(0x34);
            // e4325 mov esi,0x2008
            rsi = (0x2008u64) as u32 as u64;
            // e432a call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e432f mov DWORD PTR [rbx+0x2048],0x1
            m.w32(rbx.wrapping_add(0x2048), (0x1u64) as u32);
            // e4339 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e433d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e433e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e433f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3013 size=75
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3013<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd3013;
    loop {
        match pc {
        0xd3013 => {
            // block d3013
            #[cfg(feature = "trace")] eprintln!("f_000d3013 pc d3013 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3013 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3014 mov rbp,rsp
            rbp = rsp;
            // d3017 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d3018 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d3019 mov rbx,rdi
            rbx = rdi;
            // d301c mov ecx,DWORD PTR [rbx]
            rcx = (m.r32(rbx) as u64) as u32 as u64;
            // d301e mov eax,DWORD PTR [rbx+0x4]
            rax = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // d3021 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // d3024 jne 0xd302f
            if fl.ne() { pc = 0xd302f; continue; }
            pc = 0xd3026;
        }
        0xd3026 => {
            // block d3026
            #[cfg(feature = "trace")] eprintln!("f_000d3013 pc d3026 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3026 lea rcx,[rip+0xea563] # 0x1bd590
            rcx = m.env.data_va(859485);
            // d302d jmp 0xd3044
            pc = 0xd3044; continue;
        }
        0xd302f => {
            // block d302f
            #[cfg(feature = "trace")] eprintln!("f_000d3013 pc d302f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d302f cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // d3032 jne 0xd303d
            if fl.ne() { pc = 0xd303d; continue; }
            pc = 0xd3034;
        }
        0xd3034 => {
            // block d3034
            #[cfg(feature = "trace")] eprintln!("f_000d3013 pc d3034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3034 lea rcx,[rip+0xea495] # 0x1bd4d0
            rcx = m.env.data_va(859293);
            // d303b jmp 0xd3044
            pc = 0xd3044; continue;
        }
        0xd303d => {
            // block d303d
            #[cfg(feature = "trace")] eprintln!("f_000d3013 pc d303d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d303d lea rcx,[rip+0xea60c] # 0x1bd650
            rcx = m.env.data_va(859677);
            pc = 0xd3044;
        }
        0xd3044 => {
            // block d3044
            #[cfg(feature = "trace")] eprintln!("f_000d3013 pc d3044 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3044 lea rax,[rcx+rax*8]
            rax = rcx.wrapping_add(rax.wrapping_mul(8));
            // d3048 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // d304b call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3050 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d3052 lea rax,[rbx+rax*1+0x40]
            rax = rbx.wrapping_add(rax).wrapping_add(0x40);
            // d3057 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d305b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d305c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d305d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
