//! Mechanically lifted functions (scripts/lift_run.py), module g002. Each one is checked against the original
//! in the world harness (tests/world_g002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x1a83c size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a83c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1a83c;
    loop {
        match pc {
        0x1a83c => {
            // block 1a83c
            // 1a83c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a83d mov rbp,rsp
            rbp = rsp;
            // 1a840 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 1a844 mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 1a848 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 1a84b je 0x1a862
            if fl.e() { pc = 0x1a862; continue; }
            pc = 0x1a84d;
        }
        0x1a84d => {
            // block 1a84d
            // 1a84d lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 1a851 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 1a854 not rdx
            rdx = !rdx;
            // 1a857 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 1a85b add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 1a85e mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0x1a862;
        }
        0x1a862 => {
            // block 1a862
            // 1a862 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 1a865 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1a868 je 0x1a870
            if fl.e() { pc = 0x1a870; continue; }
            pc = 0x1a86a;
        }
        0x1a86a => {
            // block 1a86a
            // 1a86a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a86b jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x1a870 => {
            // block 1a870
            // 1a870 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a871 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc47a4 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c47a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc47a4;
    loop {
        match pc {
        0xc47a4 => {
            // block c47a4
            // c47a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c47a5 mov rbp,rsp
            rbp = rsp;
            // c47a8 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c47ac mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // c47b0 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // c47b3 je 0xc47ca
            if fl.e() { pc = 0xc47ca; continue; }
            pc = 0xc47b5;
        }
        0xc47b5 => {
            // block c47b5
            // c47b5 lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // c47b9 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // c47bc not rdx
            rdx = !rdx;
            // c47bf and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // c47c3 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // c47c6 mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0xc47ca;
        }
        0xc47ca => {
            // block c47ca
            // c47ca mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // c47cd test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // c47d0 je 0xc47d8
            if fl.e() { pc = 0xc47d8; continue; }
            pc = 0xc47d2;
        }
        0xc47d2 => {
            // block c47d2
            // c47d2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c47d3 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc47d8 => {
            // block c47d8
            // c47d8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c47d9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdfc5a size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dfc5a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdfc5a;
    loop {
        match pc {
        0xdfc5a => {
            // block dfc5a
            // dfc5a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dfc5b mov rbp,rsp
            rbp = rsp;
            // dfc5e mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // dfc62 mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // dfc66 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // dfc69 je 0xdfc80
            if fl.e() { pc = 0xdfc80; continue; }
            pc = 0xdfc6b;
        }
        0xdfc6b => {
            // block dfc6b
            // dfc6b lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // dfc6f sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // dfc72 not rdx
            rdx = !rdx;
            // dfc75 and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // dfc79 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // dfc7c mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0xdfc80;
        }
        0xdfc80 => {
            // block dfc80
            // dfc80 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // dfc83 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // dfc86 je 0xdfc8e
            if fl.e() { pc = 0xdfc8e; continue; }
            pc = 0xdfc88;
        }
        0xdfc88 => {
            // block dfc88
            // dfc88 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dfc89 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xdfc8e => {
            // block dfc8e
            // dfc8e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dfc8f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41d40 size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041d40<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x41d40;
    loop {
        match pc {
        0x41d40 => {
            // block 41d40
            // 41d40 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41d41 mov rbp,rsp
            rbp = rsp;
            // 41d44 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 41d48 mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 41d4c cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 41d4f je 0x41d66
            if fl.e() { pc = 0x41d66; continue; }
            pc = 0x41d51;
        }
        0x41d51 => {
            // block 41d51
            // 41d51 lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 41d55 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 41d58 not rdx
            rdx = !rdx;
            // 41d5b and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 41d5f add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 41d62 mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0x41d66;
        }
        0x41d66 => {
            // block 41d66
            // 41d66 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 41d69 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 41d6c je 0x41d74
            if fl.e() { pc = 0x41d74; continue; }
            pc = 0x41d6e;
        }
        0x41d6e => {
            // block 41d6e
            // 41d6e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41d6f jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x41d74 => {
            // block 41d74
            // 41d74 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41d75 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a430 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a430<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1a430;
    loop {
        match pc {
        0x1a430 => {
            // block 1a430
            // 1a430 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a431 mov rbp,rsp
            rbp = rsp;
            // 1a434 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a436 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a437 mov rbx,rsi
            rbx = rsi;
            // 1a43a mov r14,rdi
            r14 = rdi;
            // 1a43d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 1a440 je 0x1a465
            if fl.e() { pc = 0x1a465; continue; }
            pc = 0x1a442;
        }
        0x1a442 => {
            // block 1a442
            // 1a442 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 1a445 mov rdi,r14
            rdi = r14;
            // 1a448 call 0x1a430
            { let r = f_0001a430(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a44d mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1a451 mov rdi,r14
            rdi = r14;
            // 1a454 call 0x1a430
            { let r = f_0001a430(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a459 mov rdi,rbx
            rdi = rbx;
            // 1a45c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a45d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a45f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a460 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x1a465 => {
            // block 1a465
            // 1a465 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a466 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a468 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a469 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25d62 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025d62<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x25d62;
    loop {
        match pc {
        0x25d62 => {
            // block 25d62
            // 25d62 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25d63 mov rbp,rsp
            rbp = rsp;
            // 25d66 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 25d68 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 25d69 mov rbx,rsi
            rbx = rsi;
            // 25d6c mov r14,rdi
            r14 = rdi;
            // 25d6f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 25d72 je 0x25d97
            if fl.e() { pc = 0x25d97; continue; }
            pc = 0x25d74;
        }
        0x25d74 => {
            // block 25d74
            // 25d74 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 25d77 mov rdi,r14
            rdi = r14;
            // 25d7a call 0x25d62
            { let r = f_00025d62(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25d7f mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 25d83 mov rdi,r14
            rdi = r14;
            // 25d86 call 0x25d62
            { let r = f_00025d62(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25d8b mov rdi,rbx
            rdi = rbx;
            // 25d8e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25d8f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25d91 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25d92 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x25d97 => {
            // block 25d97
            // 25d97 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25d98 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25d9a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25d9b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x57d10 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00057d10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x57d10;
    loop {
        match pc {
        0x57d10 => {
            // block 57d10
            // 57d10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 57d11 mov rbp,rsp
            rbp = rsp;
            // 57d14 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 57d16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 57d17 mov rbx,rsi
            rbx = rsi;
            // 57d1a mov r14,rdi
            r14 = rdi;
            // 57d1d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 57d20 je 0x57d45
            if fl.e() { pc = 0x57d45; continue; }
            pc = 0x57d22;
        }
        0x57d22 => {
            // block 57d22
            // 57d22 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 57d25 mov rdi,r14
            rdi = r14;
            // 57d28 call 0x57d10
            { let r = f_00057d10(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57d2d mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 57d31 mov rdi,r14
            rdi = r14;
            // 57d34 call 0x57d10
            { let r = f_00057d10(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57d39 mov rdi,rbx
            rdi = rbx;
            // 57d3c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 57d3d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 57d3f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 57d40 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x57d45 => {
            // block 57d45
            // 57d45 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 57d46 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 57d48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 57d49 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5a1a size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5a1a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5a1a;
    loop {
        match pc {
        0xb5a1a => {
            // block b5a1a
            // b5a1a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5a1b mov rbp,rsp
            rbp = rsp;
            // b5a1e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5a20 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5a21 mov rbx,rsi
            rbx = rsi;
            // b5a24 mov r14,rdi
            r14 = rdi;
            // b5a27 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5a2a je 0xb5a4f
            if fl.e() { pc = 0xb5a4f; continue; }
            pc = 0xb5a2c;
        }
        0xb5a2c => {
            // block b5a2c
            // b5a2c mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // b5a2f mov rdi,r14
            rdi = r14;
            // b5a32 call 0xb5a1a
            { let r = f_000b5a1a(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5a37 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5a3b mov rdi,r14
            rdi = r14;
            // b5a3e call 0xb5a1a
            { let r = f_000b5a1a(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5a43 mov rdi,rbx
            rdi = rbx;
            // b5a46 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5a47 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5a49 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5a4a jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xb5a4f => {
            // block b5a4f
            // b5a4f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5a50 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5a52 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5a53 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb65c8 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b65c8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb65c8;
    loop {
        match pc {
        0xb65c8 => {
            // block b65c8
            // b65c8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b65c9 mov rbp,rsp
            rbp = rsp;
            // b65cc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b65ce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b65cf mov rbx,rsi
            rbx = rsi;
            // b65d2 mov r14,rdi
            r14 = rdi;
            // b65d5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b65d8 je 0xb65fd
            if fl.e() { pc = 0xb65fd; continue; }
            pc = 0xb65da;
        }
        0xb65da => {
            // block b65da
            // b65da mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // b65dd mov rdi,r14
            rdi = r14;
            // b65e0 call 0xb65c8
            { let r = f_000b65c8(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b65e5 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b65e9 mov rdi,r14
            rdi = r14;
            // b65ec call 0xb65c8
            { let r = f_000b65c8(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b65f1 mov rdi,rbx
            rdi = rbx;
            // b65f4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b65f5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b65f7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b65f8 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xb65fd => {
            // block b65fd
            // b65fd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b65fe pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b6600 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6601 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4014 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4014<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4014;
    loop {
        match pc {
        0xc4014 => {
            // block c4014
            // c4014 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4015 mov rbp,rsp
            rbp = rsp;
            // c4018 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c401a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c401b mov rbx,rsi
            rbx = rsi;
            // c401e mov r14,rdi
            r14 = rdi;
            // c4021 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c4024 je 0xc4049
            if fl.e() { pc = 0xc4049; continue; }
            pc = 0xc4026;
        }
        0xc4026 => {
            // block c4026
            // c4026 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c4029 mov rdi,r14
            rdi = r14;
            // c402c call 0xc4014
            { let r = f_000c4014(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4031 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4035 mov rdi,r14
            rdi = r14;
            // c4038 call 0xc4014
            { let r = f_000c4014(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c403d mov rdi,rbx
            rdi = rbx;
            // c4040 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4041 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4043 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4044 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc4049 => {
            // block c4049
            // c4049 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c404a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c404c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c404d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4f8c size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4f8c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4f8c;
    loop {
        match pc {
        0xc4f8c => {
            // block c4f8c
            // c4f8c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4f8d mov rbp,rsp
            rbp = rsp;
            // c4f90 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4f92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4f93 mov rbx,rsi
            rbx = rsi;
            // c4f96 mov r14,rdi
            r14 = rdi;
            // c4f99 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c4f9c je 0xc4fc1
            if fl.e() { pc = 0xc4fc1; continue; }
            pc = 0xc4f9e;
        }
        0xc4f9e => {
            // block c4f9e
            // c4f9e mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c4fa1 mov rdi,r14
            rdi = r14;
            // c4fa4 call 0xc4f8c
            { let r = f_000c4f8c(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4fa9 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4fad mov rdi,r14
            rdi = r14;
            // c4fb0 call 0xc4f8c
            { let r = f_000c4f8c(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4fb5 mov rdi,rbx
            rdi = rbx;
            // c4fb8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4fb9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4fbb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4fbc jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc4fc1 => {
            // block c4fc1
            // c4fc1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4fc2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4fc4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4fc5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x110fc size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000110fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x110fc;
    loop {
        match pc {
        0x110fc => {
            // block 110fc
            // 110fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 110fd mov rbp,rsp
            rbp = rsp;
            // 11100 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11102 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11103 mov rbx,rsi
            rbx = rsi;
            // 11106 mov r14,rdi
            r14 = rdi;
            // 11109 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 1110c je 0x11131
            if fl.e() { pc = 0x11131; continue; }
            pc = 0x1110e;
        }
        0x1110e => {
            // block 1110e
            // 1110e mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 11111 mov rdi,r14
            rdi = r14;
            // 11114 call 0x110fc
            { let r = f_000110fc(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11119 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1111d mov rdi,r14
            rdi = r14;
            // 11120 call 0x110fc
            { let r = f_000110fc(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11125 mov rdi,rbx
            rdi = rbx;
            // 11128 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11129 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1112b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1112c jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x11131 => {
            // block 11131
            // 11131 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11132 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11134 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11135 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15460 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00015460<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15460;
    loop {
        match pc {
        0x15460 => {
            // block 15460
            // 15460 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15461 mov rbp,rsp
            rbp = rsp;
            // 15464 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15466 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15467 mov rbx,rsi
            rbx = rsi;
            // 1546a mov r14,rdi
            r14 = rdi;
            // 1546d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 15470 je 0x1549e
            if fl.e() { pc = 0x1549e; continue; }
            pc = 0x15472;
        }
        0x15472 => {
            // block 15472
            // 15472 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 15475 mov rdi,r14
            rdi = r14;
            // 15478 call 0x15460
            { let r = f_00015460(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1547d mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 15481 mov rdi,r14
            rdi = r14;
            // 15484 call 0x15460
            { let r = f_00015460(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15489 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 1548d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15492 mov rdi,rbx
            rdi = rbx;
            // 15495 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15496 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15498 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15499 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x1549e => {
            // block 1549e
            // 1549e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1549f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 154a1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 154a2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5a54 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5a54<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5a54;
    loop {
        match pc {
        0xb5a54 => {
            // block b5a54
            // b5a54 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5a55 mov rbp,rsp
            rbp = rsp;
            // b5a58 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5a5a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5a5b mov rbx,rsi
            rbx = rsi;
            // b5a5e mov r14,rdi
            r14 = rdi;
            // b5a61 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5a64 je 0xb5a92
            if fl.e() { pc = 0xb5a92; continue; }
            pc = 0xb5a66;
        }
        0xb5a66 => {
            // block b5a66
            // b5a66 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // b5a69 mov rdi,r14
            rdi = r14;
            // b5a6c call 0xb5a54
            { let r = f_000b5a54(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5a71 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5a75 mov rdi,r14
            rdi = r14;
            // b5a78 call 0xb5a54
            { let r = f_000b5a54(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5a7d lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // b5a81 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5a86 mov rdi,rbx
            rdi = rbx;
            // b5a89 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5a8a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5a8c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5a8d jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xb5a92 => {
            // block b5a92
            // b5a92 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5a93 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5a95 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5a96 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5b16 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5b16<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5b16;
    loop {
        match pc {
        0xb5b16 => {
            // block b5b16
            // b5b16 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5b17 mov rbp,rsp
            rbp = rsp;
            // b5b1a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5b1c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5b1d mov rbx,rsi
            rbx = rsi;
            // b5b20 mov r14,rdi
            r14 = rdi;
            // b5b23 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5b26 je 0xb5b54
            if fl.e() { pc = 0xb5b54; continue; }
            pc = 0xb5b28;
        }
        0xb5b28 => {
            // block b5b28
            // b5b28 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // b5b2b mov rdi,r14
            rdi = r14;
            // b5b2e call 0xb5b16
            { let r = f_000b5b16(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5b33 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5b37 mov rdi,r14
            rdi = r14;
            // b5b3a call 0xb5b16
            { let r = f_000b5b16(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5b3f lea rdi,[rbx+0x28]
            rdi = rbx.wrapping_add(0x28);
            // b5b43 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5b48 mov rdi,rbx
            rdi = rbx;
            // b5b4b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5b4c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5b4e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5b4f jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xb5b54 => {
            // block b5b54
            // b5b54 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5b55 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5b57 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5b58 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3f0e size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3f0e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3f0e;
    loop {
        match pc {
        0xc3f0e => {
            // block c3f0e
            // c3f0e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3f0f mov rbp,rsp
            rbp = rsp;
            // c3f12 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3f14 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3f15 mov rbx,rsi
            rbx = rsi;
            // c3f18 mov r14,rdi
            r14 = rdi;
            // c3f1b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3f1e je 0xc3f4c
            if fl.e() { pc = 0xc3f4c; continue; }
            pc = 0xc3f20;
        }
        0xc3f20 => {
            // block c3f20
            // c3f20 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c3f23 mov rdi,r14
            rdi = r14;
            // c3f26 call 0xc3f0e
            { let r = f_000c3f0e(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3f2b mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c3f2f mov rdi,r14
            rdi = r14;
            // c3f32 call 0xc3f0e
            { let r = f_000c3f0e(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3f37 lea rdi,[rbx+0x28]
            rdi = rbx.wrapping_add(0x28);
            // c3f3b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3f40 mov rdi,rbx
            rdi = rbx;
            // c3f43 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3f44 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3f46 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3f47 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc3f4c => {
            // block c3f4c
            // c3f4c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3f4d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3f4f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3f50 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3fd0 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3fd0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3fd0;
    loop {
        match pc {
        0xc3fd0 => {
            // block c3fd0
            // c3fd0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3fd1 mov rbp,rsp
            rbp = rsp;
            // c3fd4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3fd6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3fd7 mov rbx,rsi
            rbx = rsi;
            // c3fda mov r14,rdi
            r14 = rdi;
            // c3fdd test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3fe0 je 0xc400e
            if fl.e() { pc = 0xc400e; continue; }
            pc = 0xc3fe2;
        }
        0xc3fe2 => {
            // block c3fe2
            // c3fe2 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c3fe5 mov rdi,r14
            rdi = r14;
            // c3fe8 call 0xc3fd0
            { let r = f_000c3fd0(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3fed mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c3ff1 mov rdi,r14
            rdi = r14;
            // c3ff4 call 0xc3fd0
            { let r = f_000c3fd0(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3ff9 lea rdi,[rbx+0x28]
            rdi = rbx.wrapping_add(0x28);
            // c3ffd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4002 mov rdi,rbx
            rdi = rbx;
            // c4005 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4006 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4008 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4009 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc400e => {
            // block c400e
            // c400e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c400f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4011 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4012 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc42d8 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c42d8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc42d8;
    loop {
        match pc {
        0xc42d8 => {
            // block c42d8
            // c42d8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c42d9 mov rbp,rsp
            rbp = rsp;
            // c42dc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c42de push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c42df mov rbx,rsi
            rbx = rsi;
            // c42e2 mov r14,rdi
            r14 = rdi;
            // c42e5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c42e8 je 0xc430d
            if fl.e() { pc = 0xc430d; continue; }
            pc = 0xc42ea;
        }
        0xc42ea => {
            // block c42ea
            // c42ea mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c42ed mov rdi,r14
            rdi = r14;
            // c42f0 call 0xc42d8
            { let r = f_000c42d8(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c42f5 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c42f9 mov rdi,r14
            rdi = r14;
            // c42fc call 0xc42d8
            { let r = f_000c42d8(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4301 mov rdi,rbx
            rdi = rbx;
            // c4304 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4305 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4307 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4308 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc430d => {
            // block c430d
            // c430d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c430e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4310 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4311 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4088 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4088<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4088;
    loop {
        match pc {
        0xc4088 => {
            // block c4088
            // c4088 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4089 mov rbp,rsp
            rbp = rsp;
            // c408c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c408e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c408f mov rbx,rsi
            rbx = rsi;
            // c4092 mov r14,rdi
            r14 = rdi;
            // c4095 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c4098 je 0xc40bd
            if fl.e() { pc = 0xc40bd; continue; }
            pc = 0xc409a;
        }
        0xc409a => {
            // block c409a
            // c409a mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c409d mov rdi,r14
            rdi = r14;
            // c40a0 call 0xc4088
            { let r = f_000c4088(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c40a5 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c40a9 mov rdi,r14
            rdi = r14;
            // c40ac call 0xc4088
            { let r = f_000c4088(m.env, [rdi, rsi, rdx, rcx, r8, r9], [0; 8], &[]).as_call(); rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c40b1 mov rdi,rbx
            rdi = rbx;
            // c40b4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c40b5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c40b7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c40b8 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc40bd => {
            // block c40bd
            // c40bd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c40be pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c40c0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c40c1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25b4c size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025b4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x25b4c;
    loop {
        match pc {
        0x25b4c => {
            // block 25b4c
            // 25b4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25b4d mov rbp,rsp
            rbp = rsp;
            // 25b50 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 25b52 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 25b54 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 25b56 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 25b57 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 25b5b mov r15,rsi
            r15 = rsi;
            // 25b5e mov r14,rdi
            r14 = rdi;
            // 25b61 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 25b65 mov rdx,r15
            rdx = r15;
            // 25b68 call 0xeb9ae
            { let r = { let t = m.env.stub_va(0x17b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25b6d mov r12,rax
            r12 = rax;
            // 25b70 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 25b74 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 25b77 je 0x25b7d
            if fl.e() { pc = 0x25b7d; continue; }
            pc = 0x25b79;
        }
        0x25b79 => {
            // block 25b79
            // 25b79 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 25b7b jmp 0x25ba4
            pc = 0x25ba4; continue;
        }
        0x25b7d => {
            // block 25b7d
            // 25b7d mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 25b82 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25b87 mov rbx,rax
            rbx = rax;
            // 25b8a mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 25b8d mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 25b90 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 25b94 mov rdi,r14
            rdi = r14;
            // 25b97 mov rdx,r12
            rdx = r12;
            // 25b9a mov rcx,rbx
            rcx = rbx;
            // 25b9d call 0xeb9ba
            { let r = { let t = m.env.stub_va(0x187); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25ba2 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x25ba4;
        }
        0x25ba4 => {
            // block 25ba4
            // 25ba4 mov rax,rbx
            rax = rbx;
            // 25ba7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 25bab pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25bac pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 25bae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25bb0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 25bb2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25bb3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d884 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d884<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2d884;
    loop {
        match pc {
        0x2d884 => {
            // block 2d884
            // 2d884 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d885 mov rbp,rsp
            rbp = rsp;
            // 2d888 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2d88a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d88c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2d88e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d88f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2d893 mov r15,rsi
            r15 = rsi;
            // 2d896 mov r14,rdi
            r14 = rdi;
            // 2d899 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2d89d mov rdx,r15
            rdx = r15;
            // 2d8a0 call 0xebab6
            { let r = { let t = m.env.stub_va(0x283); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d8a5 mov r12,rax
            r12 = rax;
            // 2d8a8 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 2d8ac test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2d8af je 0x2d8b5
            if fl.e() { pc = 0x2d8b5; continue; }
            pc = 0x2d8b1;
        }
        0x2d8b1 => {
            // block 2d8b1
            // 2d8b1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2d8b3 jmp 0x2d8dc
            pc = 0x2d8dc; continue;
        }
        0x2d8b5 => {
            // block 2d8b5
            // 2d8b5 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 2d8ba call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d8bf mov rbx,rax
            rbx = rax;
            // 2d8c2 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 2d8c5 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 2d8c8 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 2d8cc mov rdi,r14
            rdi = r14;
            // 2d8cf mov rdx,r12
            rdx = r12;
            // 2d8d2 mov rcx,rbx
            rcx = rbx;
            // 2d8d5 call 0xebac2
            { let r = { let t = m.env.stub_va(0x28f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d8da mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x2d8dc;
        }
        0x2d8dc => {
            // block 2d8dc
            // 2d8dc mov rax,rbx
            rax = rbx;
            // 2d8df add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2d8e3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d8e4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2d8e6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d8e8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d8ea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d8eb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2daea size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002daea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2daea;
    loop {
        match pc {
        0x2daea => {
            // block 2daea
            // 2daea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2daeb mov rbp,rsp
            rbp = rsp;
            // 2daee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2daf0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2daf2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2daf4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2daf5 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2daf9 mov r15,rsi
            r15 = rsi;
            // 2dafc mov r14,rdi
            r14 = rdi;
            // 2daff lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2db03 mov rdx,r15
            rdx = r15;
            // 2db06 call 0xebad4
            { let r = { let t = m.env.stub_va(0x2a1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2db0b mov r12,rax
            r12 = rax;
            // 2db0e mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 2db12 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2db15 je 0x2db1b
            if fl.e() { pc = 0x2db1b; continue; }
            pc = 0x2db17;
        }
        0x2db17 => {
            // block 2db17
            // 2db17 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2db19 jmp 0x2db42
            pc = 0x2db42; continue;
        }
        0x2db1b => {
            // block 2db1b
            // 2db1b mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 2db20 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2db25 mov rbx,rax
            rbx = rax;
            // 2db28 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 2db2b mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 2db2e mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 2db32 mov rdi,r14
            rdi = r14;
            // 2db35 mov rdx,r12
            rdx = r12;
            // 2db38 mov rcx,rbx
            rcx = rbx;
            // 2db3b call 0xebae0
            { let r = { let t = m.env.stub_va(0x2ad); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2db40 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x2db42;
        }
        0x2db42 => {
            // block 2db42
            // 2db42 mov rax,rbx
            rax = rbx;
            // 2db45 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2db49 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2db4a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2db4c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2db4e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2db50 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2db51 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44c00 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044c00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44c00;
    loop {
        match pc {
        0x44c00 => {
            // block 44c00
            // 44c00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44c01 mov rbp,rsp
            rbp = rsp;
            // 44c04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44c06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44c08 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 44c0a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44c0b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44c0f mov r15,rsi
            r15 = rsi;
            // 44c12 mov r14,rdi
            r14 = rdi;
            // 44c15 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 44c19 mov rdx,r15
            rdx = r15;
            // 44c1c call 0xebb46
            { let r = { let t = m.env.stub_va(0x313); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44c21 mov r12,rax
            r12 = rax;
            // 44c24 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 44c28 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 44c2b je 0x44c31
            if fl.e() { pc = 0x44c31; continue; }
            pc = 0x44c2d;
        }
        0x44c2d => {
            // block 44c2d
            // 44c2d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 44c2f jmp 0x44c58
            pc = 0x44c58; continue;
        }
        0x44c31 => {
            // block 44c31
            // 44c31 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 44c36 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44c3b mov rbx,rax
            rbx = rax;
            // 44c3e mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 44c41 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 44c44 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 44c48 mov rdi,r14
            rdi = r14;
            // 44c4b mov rdx,r12
            rdx = r12;
            // 44c4e mov rcx,rbx
            rcx = rbx;
            // 44c51 call 0xebb5e
            { let r = { let t = m.env.stub_va(0x32b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44c56 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x44c58;
        }
        0x44c58 => {
            // block 44c58
            // 44c58 mov rax,rbx
            rax = rbx;
            // 44c5b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44c5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44c60 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 44c62 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44c64 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 44c66 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44c67 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6556e size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006556e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6556e;
    loop {
        match pc {
        0x6556e => {
            // block 6556e
            // 6556e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6556f mov rbp,rsp
            rbp = rsp;
            // 65572 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 65574 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 65576 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65578 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65579 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 6557d mov r15,rsi
            r15 = rsi;
            // 65580 mov r14,rdi
            r14 = rdi;
            // 65583 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 65587 mov rdx,r15
            rdx = r15;
            // 6558a call 0xebba6
            { let r = { let t = m.env.stub_va(0x373); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6558f mov r12,rax
            r12 = rax;
            // 65592 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 65596 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 65599 je 0x6559f
            if fl.e() { pc = 0x6559f; continue; }
            pc = 0x6559b;
        }
        0x6559b => {
            // block 6559b
            // 6559b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6559d jmp 0x655c6
            pc = 0x655c6; continue;
        }
        0x6559f => {
            // block 6559f
            // 6559f mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 655a4 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 655a9 mov rbx,rax
            rbx = rax;
            // 655ac mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 655af mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 655b2 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 655b6 mov rdi,r14
            rdi = r14;
            // 655b9 mov rdx,r12
            rdx = r12;
            // 655bc mov rcx,rbx
            rcx = rbx;
            // 655bf call 0xebbb2
            { let r = { let t = m.env.stub_va(0x37f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 655c4 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x655c6;
        }
        0x655c6 => {
            // block 655c6
            // 655c6 mov rax,rbx
            rax = rbx;
            // 655c9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 655cd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 655ce pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 655d0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 655d2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 655d4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 655d5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4d44 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4d44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4d44;
    loop {
        match pc {
        0xc4d44 => {
            // block c4d44
            // c4d44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4d45 mov rbp,rsp
            rbp = rsp;
            // c4d48 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c4d4a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4d4c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c4d4e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4d4f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c4d53 mov r15,rsi
            r15 = rsi;
            // c4d56 mov r14,rdi
            r14 = rdi;
            // c4d59 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c4d5d mov rdx,r15
            rdx = r15;
            // c4d60 call 0xebd8c
            { let r = { let t = m.env.stub_va(0x559); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4d65 mov r12,rax
            r12 = rax;
            // c4d68 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // c4d6c test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c4d6f je 0xc4d75
            if fl.e() { pc = 0xc4d75; continue; }
            pc = 0xc4d71;
        }
        0xc4d71 => {
            // block c4d71
            // c4d71 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c4d73 jmp 0xc4d9c
            pc = 0xc4d9c; continue;
        }
        0xc4d75 => {
            // block c4d75
            // c4d75 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // c4d7a call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4d7f mov rbx,rax
            rbx = rax;
            // c4d82 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // c4d85 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c4d88 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c4d8c mov rdi,r14
            rdi = r14;
            // c4d8f mov rdx,r12
            rdx = r12;
            // c4d92 mov rcx,rbx
            rcx = rbx;
            // c4d95 call 0xebd9e
            { let r = { let t = m.env.stub_va(0x56b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4d9a mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xc4d9c;
        }
        0xc4d9c => {
            // block c4d9c
            // c4d9c mov rax,rbx
            rax = rbx;
            // c4d9f add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c4da3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4da4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c4da6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4da8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c4daa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4dab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4940 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4940<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd4940;
    loop {
        match pc {
        0xd4940 => {
            // block d4940
            // d4940 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4941 mov rbp,rsp
            rbp = rsp;
            // d4944 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4946 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4948 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d494a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d494b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d494f mov r15,rsi
            r15 = rsi;
            // d4952 mov r14,rdi
            r14 = rdi;
            // d4955 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // d4959 mov rdx,r15
            rdx = r15;
            // d495c call 0xebe9a
            { let r = { let t = m.env.stub_va(0x667); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4961 mov r12,rax
            r12 = rax;
            // d4964 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // d4968 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d496b je 0xd4971
            if fl.e() { pc = 0xd4971; continue; }
            pc = 0xd496d;
        }
        0xd496d => {
            // block d496d
            // d496d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d496f jmp 0xd4998
            pc = 0xd4998; continue;
        }
        0xd4971 => {
            // block d4971
            // d4971 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // d4976 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d497b mov rbx,rax
            rbx = rax;
            // d497e mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // d4981 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // d4984 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // d4988 mov rdi,r14
            rdi = r14;
            // d498b mov rdx,r12
            rdx = r12;
            // d498e mov rcx,rbx
            rcx = rbx;
            // d4991 call 0xebea6
            { let r = { let t = m.env.stub_va(0x673); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4996 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd4998;
        }
        0xd4998 => {
            // block d4998
            // d4998 mov rax,rbx
            rax = rbx;
            // d499b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d499f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d49a0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d49a2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d49a4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d49a6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d49a7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dcfe size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dcfe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2dcfe;
    loop {
        match pc {
        0x2dcfe => {
            // block 2dcfe
            // 2dcfe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dcff mov rbp,rsp
            rbp = rsp;
            // 2dd02 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2dd06 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 2dd0a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2dd0d je 0x2dd41
            if fl.e() { pc = 0x2dd41; continue; }
            pc = 0x2dd0f;
        }
        0x2dd0f => {
            // block 2dd0f
            // 2dd0f mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // 2dd11 mov rsi,rdi
            rsi = rdi;
            pc = 0x2dd14;
        }
        0x2dd14 => {
            // block 2dd14
            // 2dd14 mov rax,rdx
            rax = rdx;
            pc = 0x2dd17;
        }
        0x2dd17 => {
            // block 2dd17
            // 2dd17 cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // 2dd1a jae 0x2dd27
            if fl.ae() { pc = 0x2dd27; continue; }
            pc = 0x2dd1c;
        }
        0x2dd1c => {
            // block 2dd1c
            // 2dd1c mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2dd20 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2dd23 jne 0x2dd17
            if fl.ne() { pc = 0x2dd17; continue; }
            pc = 0x2dd25;
        }
        0x2dd25 => {
            // block 2dd25
            // 2dd25 jmp 0x2dd34
            pc = 0x2dd34; continue;
        }
        0x2dd27 => {
            // block 2dd27
            // 2dd27 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 2dd2a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2dd2d mov rsi,rax
            rsi = rax;
            // 2dd30 jne 0x2dd14
            if fl.ne() { pc = 0x2dd14; continue; }
            pc = 0x2dd32;
        }
        0x2dd32 => {
            // block 2dd32
            // 2dd32 jmp 0x2dd37
            pc = 0x2dd37; continue;
        }
        0x2dd34 => {
            // block 2dd34
            // 2dd34 mov rax,rsi
            rax = rsi;
            pc = 0x2dd37;
        }
        0x2dd37 => {
            // block 2dd37
            // 2dd37 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 2dd3a je 0x2dd41
            if fl.e() { pc = 0x2dd41; continue; }
            pc = 0x2dd3c;
        }
        0x2dd3c => {
            // block 2dd3c
            // 2dd3c cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // 2dd3f jae 0x2dd44
            if fl.ae() { pc = 0x2dd44; continue; }
            pc = 0x2dd41;
        }
        0x2dd41 => {
            // block 2dd41
            // 2dd41 mov rax,rdi
            rax = rdi;
            pc = 0x2dd44;
        }
        0x2dd44 => {
            // block 2dd44
            // 2dd44 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dd45 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44a74 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044a74<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44a74;
    loop {
        match pc {
        0x44a74 => {
            // block 44a74
            // 44a74 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44a75 mov rbp,rsp
            rbp = rsp;
            // 44a78 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 44a7c add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 44a80 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 44a83 je 0x44ab7
            if fl.e() { pc = 0x44ab7; continue; }
            pc = 0x44a85;
        }
        0x44a85 => {
            // block 44a85
            // 44a85 mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // 44a87 mov rsi,rdi
            rsi = rdi;
            pc = 0x44a8a;
        }
        0x44a8a => {
            // block 44a8a
            // 44a8a mov rax,rdx
            rax = rdx;
            pc = 0x44a8d;
        }
        0x44a8d => {
            // block 44a8d
            // 44a8d cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // 44a90 jge 0x44a9d
            if fl.ge() { pc = 0x44a9d; continue; }
            pc = 0x44a92;
        }
        0x44a92 => {
            // block 44a92
            // 44a92 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 44a96 test rax,rax
            fl = logic::<64>(rax & rax);
            // 44a99 jne 0x44a8d
            if fl.ne() { pc = 0x44a8d; continue; }
            pc = 0x44a9b;
        }
        0x44a9b => {
            // block 44a9b
            // 44a9b jmp 0x44aaa
            pc = 0x44aaa; continue;
        }
        0x44a9d => {
            // block 44a9d
            // 44a9d mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 44aa0 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 44aa3 mov rsi,rax
            rsi = rax;
            // 44aa6 jne 0x44a8a
            if fl.ne() { pc = 0x44a8a; continue; }
            pc = 0x44aa8;
        }
        0x44aa8 => {
            // block 44aa8
            // 44aa8 jmp 0x44aad
            pc = 0x44aad; continue;
        }
        0x44aaa => {
            // block 44aaa
            // 44aaa mov rax,rsi
            rax = rsi;
            pc = 0x44aad;
        }
        0x44aad => {
            // block 44aad
            // 44aad cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 44ab0 je 0x44ab7
            if fl.e() { pc = 0x44ab7; continue; }
            pc = 0x44ab2;
        }
        0x44ab2 => {
            // block 44ab2
            // 44ab2 cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // 44ab5 jge 0x44aba
            if fl.ge() { pc = 0x44aba; continue; }
            pc = 0x44ab7;
        }
        0x44ab7 => {
            // block 44ab7
            // 44ab7 mov rax,rdi
            rax = rdi;
            pc = 0x44aba;
        }
        0x44aba => {
            // block 44aba
            // 44aba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44abb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x582ea size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000582ea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x582ea;
    loop {
        match pc {
        0x582ea => {
            // block 582ea
            // 582ea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 582eb mov rbp,rsp
            rbp = rsp;
            // 582ee mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 582f2 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 582f6 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 582f9 je 0x5832d
            if fl.e() { pc = 0x5832d; continue; }
            pc = 0x582fb;
        }
        0x582fb => {
            // block 582fb
            // 582fb mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // 582fd mov rsi,rdi
            rsi = rdi;
            pc = 0x58300;
        }
        0x58300 => {
            // block 58300
            // 58300 mov rax,rdx
            rax = rdx;
            pc = 0x58303;
        }
        0x58303 => {
            // block 58303
            // 58303 cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // 58306 jge 0x58313
            if fl.ge() { pc = 0x58313; continue; }
            pc = 0x58308;
        }
        0x58308 => {
            // block 58308
            // 58308 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 5830c test rax,rax
            fl = logic::<64>(rax & rax);
            // 5830f jne 0x58303
            if fl.ne() { pc = 0x58303; continue; }
            pc = 0x58311;
        }
        0x58311 => {
            // block 58311
            // 58311 jmp 0x58320
            pc = 0x58320; continue;
        }
        0x58313 => {
            // block 58313
            // 58313 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 58316 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 58319 mov rsi,rax
            rsi = rax;
            // 5831c jne 0x58300
            if fl.ne() { pc = 0x58300; continue; }
            pc = 0x5831e;
        }
        0x5831e => {
            // block 5831e
            // 5831e jmp 0x58323
            pc = 0x58323; continue;
        }
        0x58320 => {
            // block 58320
            // 58320 mov rax,rsi
            rax = rsi;
            pc = 0x58323;
        }
        0x58323 => {
            // block 58323
            // 58323 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 58326 je 0x5832d
            if fl.e() { pc = 0x5832d; continue; }
            pc = 0x58328;
        }
        0x58328 => {
            // block 58328
            // 58328 cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // 5832b jge 0x58330
            if fl.ge() { pc = 0x58330; continue; }
            pc = 0x5832d;
        }
        0x5832d => {
            // block 5832d
            // 5832d mov rax,rdi
            rax = rdi;
            pc = 0x58330;
        }
        0x58330 => {
            // block 58330
            // 58330 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58331 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb58f6 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b58f6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb58f6;
    loop {
        match pc {
        0xb58f6 => {
            // block b58f6
            // b58f6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b58f7 mov rbp,rsp
            rbp = rsp;
            // b58fa mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b58fe add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b5902 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5905 je 0xb5939
            if fl.e() { pc = 0xb5939; continue; }
            pc = 0xb5907;
        }
        0xb5907 => {
            // block b5907
            // b5907 mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // b5909 mov rsi,rdi
            rsi = rdi;
            pc = 0xb590c;
        }
        0xb590c => {
            // block b590c
            // b590c mov rax,rdx
            rax = rdx;
            pc = 0xb590f;
        }
        0xb590f => {
            // block b590f
            // b590f cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // b5912 jae 0xb591f
            if fl.ae() { pc = 0xb591f; continue; }
            pc = 0xb5914;
        }
        0xb5914 => {
            // block b5914
            // b5914 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // b5918 test rax,rax
            fl = logic::<64>(rax & rax);
            // b591b jne 0xb590f
            if fl.ne() { pc = 0xb590f; continue; }
            pc = 0xb591d;
        }
        0xb591d => {
            // block b591d
            // b591d jmp 0xb592c
            pc = 0xb592c; continue;
        }
        0xb591f => {
            // block b591f
            // b591f mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // b5922 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5925 mov rsi,rax
            rsi = rax;
            // b5928 jne 0xb590c
            if fl.ne() { pc = 0xb590c; continue; }
            pc = 0xb592a;
        }
        0xb592a => {
            // block b592a
            // b592a jmp 0xb592f
            pc = 0xb592f; continue;
        }
        0xb592c => {
            // block b592c
            // b592c mov rax,rsi
            rax = rsi;
            pc = 0xb592f;
        }
        0xb592f => {
            // block b592f
            // b592f cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // b5932 je 0xb5939
            if fl.e() { pc = 0xb5939; continue; }
            pc = 0xb5934;
        }
        0xb5934 => {
            // block b5934
            // b5934 cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // b5937 jae 0xb593c
            if fl.ae() { pc = 0xb593c; continue; }
            pc = 0xb5939;
        }
        0xb5939 => {
            // block b5939
            // b5939 mov rax,rdi
            rax = rdi;
            pc = 0xb593c;
        }
        0xb593c => {
            // block b593c
            // b593c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b593d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5b5a size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5b5a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5b5a;
    loop {
        match pc {
        0xb5b5a => {
            // block b5b5a
            // b5b5a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5b5b mov rbp,rsp
            rbp = rsp;
            // b5b5e mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b5b62 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b5b66 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5b69 je 0xb5b9d
            if fl.e() { pc = 0xb5b9d; continue; }
            pc = 0xb5b6b;
        }
        0xb5b6b => {
            // block b5b6b
            // b5b6b mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // b5b6d mov rsi,rdi
            rsi = rdi;
            pc = 0xb5b70;
        }
        0xb5b70 => {
            // block b5b70
            // b5b70 mov rax,rdx
            rax = rdx;
            pc = 0xb5b73;
        }
        0xb5b73 => {
            // block b5b73
            // b5b73 cmp DWORD PTR [rax+0x20],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x20)) as u64, (rcx as u32 as u64)).1;
            // b5b76 jae 0xb5b83
            if fl.ae() { pc = 0xb5b83; continue; }
            pc = 0xb5b78;
        }
        0xb5b78 => {
            // block b5b78
            // b5b78 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // b5b7c test rax,rax
            fl = logic::<64>(rax & rax);
            // b5b7f jne 0xb5b73
            if fl.ne() { pc = 0xb5b73; continue; }
            pc = 0xb5b81;
        }
        0xb5b81 => {
            // block b5b81
            // b5b81 jmp 0xb5b90
            pc = 0xb5b90; continue;
        }
        0xb5b83 => {
            // block b5b83
            // b5b83 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // b5b86 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5b89 mov rsi,rax
            rsi = rax;
            // b5b8c jne 0xb5b70
            if fl.ne() { pc = 0xb5b70; continue; }
            pc = 0xb5b8e;
        }
        0xb5b8e => {
            // block b5b8e
            // b5b8e jmp 0xb5b93
            pc = 0xb5b93; continue;
        }
        0xb5b90 => {
            // block b5b90
            // b5b90 mov rax,rsi
            rax = rsi;
            pc = 0xb5b93;
        }
        0xb5b93 => {
            // block b5b93
            // b5b93 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // b5b96 je 0xb5b9d
            if fl.e() { pc = 0xb5b9d; continue; }
            pc = 0xb5b98;
        }
        0xb5b98 => {
            // block b5b98
            // b5b98 cmp ecx,DWORD PTR [rax+0x20]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x20)) as u64).1;
            // b5b9b jae 0xb5ba0
            if fl.ae() { pc = 0xb5ba0; continue; }
            pc = 0xb5b9d;
        }
        0xb5b9d => {
            // block b5b9d
            // b5b9d mov rax,rdi
            rax = rdi;
            pc = 0xb5ba0;
        }
        0xb5ba0 => {
            // block b5ba0
            // b5ba0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5ba1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6664 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6664<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb6664;
    loop {
        match pc {
        0xb6664 => {
            // block b6664
            // b6664 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6665 mov rbp,rsp
            rbp = rsp;
            // b6668 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b666c add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b6670 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b6673 je 0xb66a7
            if fl.e() { pc = 0xb66a7; continue; }
            pc = 0xb6675;
        }
        0xb6675 => {
            // block b6675
            // b6675 mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // b6677 mov rsi,rdi
            rsi = rdi;
            pc = 0xb667a;
        }
        0xb667a => {
            // block b667a
            // b667a mov rax,rdx
            rax = rdx;
            pc = 0xb667d;
        }
        0xb667d => {
            // block b667d
            // b667d cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // b6680 jae 0xb668d
            if fl.ae() { pc = 0xb668d; continue; }
            pc = 0xb6682;
        }
        0xb6682 => {
            // block b6682
            // b6682 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // b6686 test rax,rax
            fl = logic::<64>(rax & rax);
            // b6689 jne 0xb667d
            if fl.ne() { pc = 0xb667d; continue; }
            pc = 0xb668b;
        }
        0xb668b => {
            // block b668b
            // b668b jmp 0xb669a
            pc = 0xb669a; continue;
        }
        0xb668d => {
            // block b668d
            // b668d mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // b6690 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b6693 mov rsi,rax
            rsi = rax;
            // b6696 jne 0xb667a
            if fl.ne() { pc = 0xb667a; continue; }
            pc = 0xb6698;
        }
        0xb6698 => {
            // block b6698
            // b6698 jmp 0xb669d
            pc = 0xb669d; continue;
        }
        0xb669a => {
            // block b669a
            // b669a mov rax,rsi
            rax = rsi;
            pc = 0xb669d;
        }
        0xb669d => {
            // block b669d
            // b669d cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // b66a0 je 0xb66a7
            if fl.e() { pc = 0xb66a7; continue; }
            pc = 0xb66a2;
        }
        0xb66a2 => {
            // block b66a2
            // b66a2 cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // b66a5 jae 0xb66aa
            if fl.ae() { pc = 0xb66aa; continue; }
            pc = 0xb66a7;
        }
        0xb66a7 => {
            // block b66a7
            // b66a7 mov rax,rdi
            rax = rdi;
            pc = 0xb66aa;
        }
        0xb66aa => {
            // block b66aa
            // b66aa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b66ab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc44b6 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c44b6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc44b6;
    loop {
        match pc {
        0xc44b6 => {
            // block c44b6
            // c44b6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c44b7 mov rbp,rsp
            rbp = rsp;
            // c44ba mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c44be add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c44c2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c44c5 je 0xc44f9
            if fl.e() { pc = 0xc44f9; continue; }
            pc = 0xc44c7;
        }
        0xc44c7 => {
            // block c44c7
            // c44c7 mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // c44c9 mov rsi,rdi
            rsi = rdi;
            pc = 0xc44cc;
        }
        0xc44cc => {
            // block c44cc
            // c44cc mov rax,rdx
            rax = rdx;
            pc = 0xc44cf;
        }
        0xc44cf => {
            // block c44cf
            // c44cf cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // c44d2 jae 0xc44df
            if fl.ae() { pc = 0xc44df; continue; }
            pc = 0xc44d4;
        }
        0xc44d4 => {
            // block c44d4
            // c44d4 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // c44d8 test rax,rax
            fl = logic::<64>(rax & rax);
            // c44db jne 0xc44cf
            if fl.ne() { pc = 0xc44cf; continue; }
            pc = 0xc44dd;
        }
        0xc44dd => {
            // block c44dd
            // c44dd jmp 0xc44ec
            pc = 0xc44ec; continue;
        }
        0xc44df => {
            // block c44df
            // c44df mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c44e2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c44e5 mov rsi,rax
            rsi = rax;
            // c44e8 jne 0xc44cc
            if fl.ne() { pc = 0xc44cc; continue; }
            pc = 0xc44ea;
        }
        0xc44ea => {
            // block c44ea
            // c44ea jmp 0xc44ef
            pc = 0xc44ef; continue;
        }
        0xc44ec => {
            // block c44ec
            // c44ec mov rax,rsi
            rax = rsi;
            pc = 0xc44ef;
        }
        0xc44ef => {
            // block c44ef
            // c44ef cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // c44f2 je 0xc44f9
            if fl.e() { pc = 0xc44f9; continue; }
            pc = 0xc44f4;
        }
        0xc44f4 => {
            // block c44f4
            // c44f4 cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // c44f7 jae 0xc44fc
            if fl.ae() { pc = 0xc44fc; continue; }
            pc = 0xc44f9;
        }
        0xc44f9 => {
            // block c44f9
            // c44f9 mov rax,rdi
            rax = rdi;
            pc = 0xc44fc;
        }
        0xc44fc => {
            // block c44fc
            // c44fc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c44fd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4822 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4822<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4822;
    loop {
        match pc {
        0xc4822 => {
            // block c4822
            // c4822 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4823 mov rbp,rsp
            rbp = rsp;
            // c4826 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c482a add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c482e test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4831 je 0xc4865
            if fl.e() { pc = 0xc4865; continue; }
            pc = 0xc4833;
        }
        0xc4833 => {
            // block c4833
            // c4833 mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // c4835 mov rsi,rdi
            rsi = rdi;
            pc = 0xc4838;
        }
        0xc4838 => {
            // block c4838
            // c4838 mov rax,rdx
            rax = rdx;
            pc = 0xc483b;
        }
        0xc483b => {
            // block c483b
            // c483b cmp DWORD PTR [rax+0x20],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x20)) as u64, (rcx as u32 as u64)).1;
            // c483e jge 0xc484b
            if fl.ge() { pc = 0xc484b; continue; }
            pc = 0xc4840;
        }
        0xc4840 => {
            // block c4840
            // c4840 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // c4844 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4847 jne 0xc483b
            if fl.ne() { pc = 0xc483b; continue; }
            pc = 0xc4849;
        }
        0xc4849 => {
            // block c4849
            // c4849 jmp 0xc4858
            pc = 0xc4858; continue;
        }
        0xc484b => {
            // block c484b
            // c484b mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c484e test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4851 mov rsi,rax
            rsi = rax;
            // c4854 jne 0xc4838
            if fl.ne() { pc = 0xc4838; continue; }
            pc = 0xc4856;
        }
        0xc4856 => {
            // block c4856
            // c4856 jmp 0xc485b
            pc = 0xc485b; continue;
        }
        0xc4858 => {
            // block c4858
            // c4858 mov rax,rsi
            rax = rsi;
            pc = 0xc485b;
        }
        0xc485b => {
            // block c485b
            // c485b cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // c485e je 0xc4865
            if fl.e() { pc = 0xc4865; continue; }
            pc = 0xc4860;
        }
        0xc4860 => {
            // block c4860
            // c4860 cmp ecx,DWORD PTR [rax+0x20]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x20)) as u64).1;
            // c4863 jge 0xc4868
            if fl.ge() { pc = 0xc4868; continue; }
            pc = 0xc4865;
        }
        0xc4865 => {
            // block c4865
            // c4865 mov rax,rdi
            rax = rdi;
            pc = 0xc4868;
        }
        0xc4868 => {
            // block c4868
            // c4868 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4869 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4c6c size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4c6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4c6c;
    loop {
        match pc {
        0xc4c6c => {
            // block c4c6c
            // c4c6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4c6d mov rbp,rsp
            rbp = rsp;
            // c4c70 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c4c74 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c4c78 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4c7b je 0xc4caf
            if fl.e() { pc = 0xc4caf; continue; }
            pc = 0xc4c7d;
        }
        0xc4c7d => {
            // block c4c7d
            // c4c7d mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // c4c7f mov rsi,rdi
            rsi = rdi;
            pc = 0xc4c82;
        }
        0xc4c82 => {
            // block c4c82
            // c4c82 mov rax,rdx
            rax = rdx;
            pc = 0xc4c85;
        }
        0xc4c85 => {
            // block c4c85
            // c4c85 cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // c4c88 jae 0xc4c95
            if fl.ae() { pc = 0xc4c95; continue; }
            pc = 0xc4c8a;
        }
        0xc4c8a => {
            // block c4c8a
            // c4c8a mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // c4c8e test rax,rax
            fl = logic::<64>(rax & rax);
            // c4c91 jne 0xc4c85
            if fl.ne() { pc = 0xc4c85; continue; }
            pc = 0xc4c93;
        }
        0xc4c93 => {
            // block c4c93
            // c4c93 jmp 0xc4ca2
            pc = 0xc4ca2; continue;
        }
        0xc4c95 => {
            // block c4c95
            // c4c95 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c4c98 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4c9b mov rsi,rax
            rsi = rax;
            // c4c9e jne 0xc4c82
            if fl.ne() { pc = 0xc4c82; continue; }
            pc = 0xc4ca0;
        }
        0xc4ca0 => {
            // block c4ca0
            // c4ca0 jmp 0xc4ca5
            pc = 0xc4ca5; continue;
        }
        0xc4ca2 => {
            // block c4ca2
            // c4ca2 mov rax,rsi
            rax = rsi;
            pc = 0xc4ca5;
        }
        0xc4ca5 => {
            // block c4ca5
            // c4ca5 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // c4ca8 je 0xc4caf
            if fl.e() { pc = 0xc4caf; continue; }
            pc = 0xc4caa;
        }
        0xc4caa => {
            // block c4caa
            // c4caa cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // c4cad jae 0xc4cb2
            if fl.ae() { pc = 0xc4cb2; continue; }
            pc = 0xc4caf;
        }
        0xc4caf => {
            // block c4caf
            // c4caf mov rax,rdi
            rax = rdi;
            pc = 0xc4cb2;
        }
        0xc4cb2 => {
            // block c4cb2
            // c4cb2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4cb3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
