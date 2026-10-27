//! Mechanically lifted functions (scripts/lift_run.py), module q001. Each one is checked against the original
//! in the world harness (tests/world_q001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xbc0bc size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc0bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbc0bc;
    loop {
        match pc {
        0xbc0bc => {
            // block bc0bc
            #[cfg(feature = "trace")] eprintln!("f_000bc0bc pc bc0bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc0bd mov rbp,rsp
            rbp = rsp;
            // bc0c0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc0c1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bc0c2 mov rbx,rsi
            rbx = rsi;
            // bc0c5 mov DWORD PTR [rbp-0xc],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rdi as u32 as u64)) as u32);
            // bc0c8 lea rdi,[rip+0x10c559] # 0x1c8628
            rdi = m.env.data_va(904693);
            // bc0cf lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // bc0d3 call 0xebdf2
            { m.outregs(0xbc0d8, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x5bf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc0d8 mov ecx,0x6c
            rcx = (0x6cu64) as u32 as u64;
            // bc0dd lea rdx,[rip+0x10c54c] # 0x1c8630
            rdx = m.env.data_va(904701);
            // bc0e4 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // bc0e7 je 0xbc0ec
            if crate::cov::br(fl.e()) { pc = 0xbc0ec; continue; }
            pc = 0xbc0e9;
        }
        0xbc0e9 => {
            // block bc0e9
            #[cfg(feature = "trace")] eprintln!("f_000bc0bc pc bc0e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0e9 mov ecx,DWORD PTR [rax+0x20]
            rcx = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            pc = 0xbc0ec;
        }
        0xbc0ec => {
            // block bc0ec
            #[cfg(feature = "trace")] eprintln!("f_000bc0bc pc bc0ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0ec mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            // bc0ee cmp ecx,0x6c
            fl = sub::<32>((rcx as u32 as u64), 0x6cu64).1;
            // bc0f1 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc0f4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bc0f8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc0f9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc0fa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba49e size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba49e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba49e;
    loop {
        match pc {
        0xba49e => {
            // block ba49e
            #[cfg(feature = "trace")] eprintln!("f_000ba49e pc ba49e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba49e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba49f mov rbp,rsp
            rbp = rsp;
            // ba4a2 mov rax,rdi
            rax = rdi;
            // ba4a5 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // ba4a8 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // ba4ab je 0xba4d1
            if crate::cov::br(fl.e()) { pc = 0xba4d1; continue; }
            pc = 0xba4ad;
        }
        0xba4ad => {
            // block ba4ad
            #[cfg(feature = "trace")] eprintln!("f_000ba49e pc ba4ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba4ad mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // ba4b1 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // ba4b4 je 0xba4cb
            if crate::cov::br(fl.e()) { pc = 0xba4cb; continue; }
            pc = 0xba4b6;
        }
        0xba4b6 => {
            // block ba4b6
            #[cfg(feature = "trace")] eprintln!("f_000ba49e pc ba4b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba4b6 lea rdx,[rcx-0x8]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fff8);
            // ba4ba sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // ba4bd not rdx
            { let v = !rdx; rdx = v; }
            // ba4c0 and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // ba4c4 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // ba4c7 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            pc = 0xba4cb;
        }
        0xba4cb => {
            // block ba4cb
            #[cfg(feature = "trace")] eprintln!("f_000ba49e pc ba4cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba4cb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba4cc jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xba4d1 => {
            // block ba4d1
            #[cfg(feature = "trace")] eprintln!("f_000ba49e pc ba4d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba4d1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba4d2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0a36 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0a36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0a36;
    loop {
        match pc {
        0xe0a36 => {
            // block e0a36
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0a37 mov rbp,rsp
            rbp = rsp;
            // e0a3a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0a3c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0a3d mov rbx,rdi
            rbx = rdi;
            // e0a40 mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            // e0a43 test r14,r14
            fl = logic::<64>(r14 & r14);
            // e0a46 je 0xe0a77
            if crate::cov::br(fl.e()) { pc = 0xe0a77; continue; }
            pc = 0xe0a48;
        }
        0xe0a48 => {
            // block e0a48
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a48 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e0a4c cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // e0a4f je 0xe0a6b
            if crate::cov::br(fl.e()) { pc = 0xe0a6b; continue; }
            pc = 0xe0a51;
        }
        0xe0a51 => {
            // block e0a51
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a51 lea rdi,[rax-0x38]
            rdi = rax.wrapping_add(0xffff_ffff_ffff_ffc8);
            // e0a55 mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // e0a59 mov rax,QWORD PTR [rax-0x38]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e0a5d call QWORD PTR [rax]
            { m.outregs(0xe0a5f, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a5f mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e0a63 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // e0a66 jne 0xe0a51
            if crate::cov::br(fl.ne()) { pc = 0xe0a51; continue; }
            pc = 0xe0a68;
        }
        0xe0a68 => {
            // block e0a68
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a68 mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            pc = 0xe0a6b;
        }
        0xe0a6b => {
            // block e0a6b
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a6b mov rdi,r14
            rdi = r14;
            // e0a6e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0a6f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0a71 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0a72 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xe0a77 => {
            // block e0a77
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a77 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0a78 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0a7a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0a7b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28e5e size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028e5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28e5e;
    loop {
        match pc {
        0x28e5e => {
            // block 28e5e
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28e5f mov rbp,rsp
            rbp = rsp;
            // 28e62 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28e64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28e65 mov rbx,rsi
            rbx = rsi;
            // 28e68 mov r14,rdi
            r14 = rdi;
            // 28e6b cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 28e6e je 0x28e9f
            if crate::cov::br(fl.e()) { pc = 0x28e9f; continue; }
            pc = 0x28e70;
        }
        0x28e70 => {
            // block 28e70
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e70 cmp QWORD PTR [rbx],0x0
            fl = sub::<64>(m.r64(rbx) as u64, 0x0u64).1;
            // 28e74 je 0x28e93
            if crate::cov::br(fl.e()) { pc = 0x28e93; continue; }
            pc = 0x28e76;
        }
        0x28e76 => {
            // block 28e76
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e76 mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28e79 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 28e7c je 0x28e93
            if crate::cov::br(fl.e()) { pc = 0x28e93; continue; }
            pc = 0x28e7e;
        }
        0x28e7e => {
            // block 28e7e
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e7e mov rdi,r14
            rdi = r14;
            // 28e81 call 0x28cf2
            { m.outregs(0x28e86, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e86 mov rdi,r14
            rdi = r14;
            // 28e89 mov rsi,rbx
            rsi = rbx;
            // 28e8c call 0x28dca
            { m.outregs(0x28e91, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x28dca); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e91 jmp 0x28e9f
            pc = 0x28e9f; continue;
        }
        0x28e93 => {
            // block 28e93
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e93 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 28e95 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28e97 mov rdi,r14
            rdi = r14;
            // 28e9a call 0x28ea8
            { m.outregs(0x28e9f, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x28e9f;
        }
        0x28e9f => {
            // block 28e9f
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e9f mov rax,r14
            rax = r14;
            // 28ea2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28ea3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28ea5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28ea6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0d78 size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0d78<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0d78;
    loop {
        match pc {
        0xe0d78 => {
            // block e0d78
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d78 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0d79 mov rbp,rsp
            rbp = rsp;
            // e0d7c sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0d80 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // e0d87 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0d8a lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0d8e call QWORD PTR [rax+0x1ae8]
            { m.outregs(0xe0d94, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0d94 mov ecx,DWORD PTR [rbp-0x4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0d97 shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e0d9a and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0d9d mov DWORD PTR [rbp-0x4],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), ((rcx as u32 as u64)) as u32);
            // e0da0 mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // e0da5 je 0xe0dbd
            if crate::cov::br(fl.e()) { pc = 0xe0dbd; continue; }
            pc = 0xe0da7;
        }
        0xe0da7 => {
            // block e0da7
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0da7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0da7 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // e0daa jne 0xe0db0
            if crate::cov::br(fl.ne()) { pc = 0xe0db0; continue; }
            pc = 0xe0dac;
        }
        0xe0dac => {
            // block e0dac
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0dac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0dac xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0dae jmp 0xe0dbd
            pc = 0xe0dbd; continue;
        }
        0xe0db0 => {
            // block e0db0
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0db0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0db0 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // e0db3 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // e0db6 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // e0db9 lea eax,[rax+rax*1+0x1]
            rax = (rax.wrapping_add(rax).wrapping_add(0x1)) as u32 as u64;
            pc = 0xe0dbd;
        }
        0xe0dbd => {
            // block e0dbd
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0dbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0dbd add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0dc1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0dc2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x31164 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00031164<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x31164;
    loop {
        match pc {
        0x31164 => {
            // block 31164
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 31164 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31164 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 31165 mov rbp,rsp
            rbp = rsp;
            // 31168 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3116a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3116b mov rbx,rdi
            rbx = rdi;
            // 3116e lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // 31172 mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 31176 mov rdi,r14
            rdi = r14;
            // 31179 call QWORD PTR [rax+0x108]
            { m.outregs(0x3117f, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3117f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31181 je 0x311b4
            if crate::cov::br(fl.e()) { pc = 0x311b4; continue; }
            pc = 0x31183;
        }
        0x31183 => {
            // block 31183
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 31183 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31183 mov rdi,r14
            rdi = r14;
            // 31186 call 0xb6914
            { m.outregs(0x3118b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xb6914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3118b and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3118e cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 31193 jne 0x311b4
            if crate::cov::br(fl.ne()) { pc = 0x311b4; continue; }
            pc = 0x31195;
        }
        0x31195 => {
            // block 31195
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 31195 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31195 mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 31199 mov esi,DWORD PTR [rbx+0x2a4]
            rsi = (m.r32(rbx.wrapping_add(0x2a4)) as u64) as u32 as u64;
            // 3119f mov edx,0xe6
            rdx = (0xe6u64) as u32 as u64;
            // 311a4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 311a9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 311ac mov rdi,r14
            rdi = r14;
            // 311af call QWORD PTR [rax+0x30]
            { m.outregs(0x311b2, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 311b2 jmp 0x311b6
            pc = 0x311b6; continue;
        }
        0x311b4 => {
            // block 311b4
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 311b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 311b4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x311b6;
        }
        0x311b6 => {
            // block 311b6
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 311b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 311b6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 311b7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 311b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 311ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44b5e size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044b5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44b5e;
    loop {
        match pc {
        0x44b5e => {
            // block 44b5e
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44b5f mov rbp,rsp
            rbp = rsp;
            // 44b62 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 44b66 test rax,rax
            fl = logic::<64>(rax & rax);
            // 44b69 je 0x44b9c
            if crate::cov::br(fl.e()) { pc = 0x44b9c; continue; }
            pc = 0x44b6b;
        }
        0x44b6b => {
            // block 44b6b
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b6b mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 44b6d jmp 0x44b72
            pc = 0x44b72; continue;
        }
        0x44b6f => {
            // block 44b6f
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b6f mov rax,rdx
            rax = rdx;
            pc = 0x44b72;
        }
        0x44b72 => {
            // block 44b72
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b72 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 44b75 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 44b77 jge 0x44b83
            if crate::cov::br(fl.ge()) { pc = 0x44b83; continue; }
            pc = 0x44b79;
        }
        0x44b79 => {
            // block 44b79
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b79 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 44b7c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 44b7f jne 0x44b6f
            if crate::cov::br(fl.ne()) { pc = 0x44b6f; continue; }
            pc = 0x44b81;
        }
        0x44b81 => {
            // block 44b81
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b81 jmp 0x44ba8
            pc = 0x44ba8; continue;
        }
        0x44b83 => {
            // block 44b83
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b83 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 44b85 jge 0x44bb0
            if crate::cov::br(fl.ge()) { pc = 0x44bb0; continue; }
            pc = 0x44b87;
        }
        0x44b87 => {
            // block 44b87
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b87 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 44b8b test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 44b8e jne 0x44b6f
            if crate::cov::br(fl.ne()) { pc = 0x44b6f; continue; }
            pc = 0x44b90;
        }
        0x44b90 => {
            // block 44b90
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b90 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 44b93 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 44b97 mov rsi,rax
            rsi = rax;
            // 44b9a jmp 0x44bb3
            pc = 0x44bb3; continue;
        }
        0x44b9c => {
            // block 44b9c
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44b9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b9c add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 44ba0 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 44ba3 mov rsi,rdi
            rsi = rdi;
            // 44ba6 jmp 0x44bb3
            pc = 0x44bb3; continue;
        }
        0x44ba8 => {
            // block 44ba8
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44ba8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44ba8 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 44bab mov rsi,rax
            rsi = rax;
            // 44bae jmp 0x44bb3
            pc = 0x44bb3; continue;
        }
        0x44bb0 => {
            // block 44bb0
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44bb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44bb0 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x44bb3;
        }
        0x44bb3 => {
            // block 44bb3
            #[cfg(feature = "trace")] eprintln!("f_00044b5e pc 44bb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44bb3 mov rax,rsi
            rax = rsi;
            // 44bb6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44bb7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc43c4 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c43c4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc43c4;
    loop {
        match pc {
        0xc43c4 => {
            // block c43c4
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43c4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c43c5 mov rbp,rsp
            rbp = rsp;
            // c43c8 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c43cc test rax,rax
            fl = logic::<64>(rax & rax);
            // c43cf je 0xc4402
            if crate::cov::br(fl.e()) { pc = 0xc4402; continue; }
            pc = 0xc43d1;
        }
        0xc43d1 => {
            // block c43d1
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43d1 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c43d3 jmp 0xc43d8
            pc = 0xc43d8; continue;
        }
        0xc43d5 => {
            // block c43d5
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43d5 mov rax,rdx
            rax = rdx;
            pc = 0xc43d8;
        }
        0xc43d8 => {
            // block c43d8
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43d8 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c43db cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c43dd jae 0xc43e9
            if crate::cov::br(fl.ae()) { pc = 0xc43e9; continue; }
            pc = 0xc43df;
        }
        0xc43df => {
            // block c43df
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43df mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c43e2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c43e5 jne 0xc43d5
            if crate::cov::br(fl.ne()) { pc = 0xc43d5; continue; }
            pc = 0xc43e7;
        }
        0xc43e7 => {
            // block c43e7
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43e7 jmp 0xc440e
            pc = 0xc440e; continue;
        }
        0xc43e9 => {
            // block c43e9
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43e9 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c43eb jae 0xc4416
            if crate::cov::br(fl.ae()) { pc = 0xc4416; continue; }
            pc = 0xc43ed;
        }
        0xc43ed => {
            // block c43ed
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43ed mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // c43f1 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c43f4 jne 0xc43d5
            if crate::cov::br(fl.ne()) { pc = 0xc43d5; continue; }
            pc = 0xc43f6;
        }
        0xc43f6 => {
            // block c43f6
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43f6 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c43f9 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c43fd mov rsi,rax
            rsi = rax;
            // c4400 jmp 0xc4419
            pc = 0xc4419; continue;
        }
        0xc4402 => {
            // block c4402
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c4402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4402 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c4406 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c4409 mov rsi,rdi
            rsi = rdi;
            // c440c jmp 0xc4419
            pc = 0xc4419; continue;
        }
        0xc440e => {
            // block c440e
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c440e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c440e mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c4411 mov rsi,rax
            rsi = rax;
            // c4414 jmp 0xc4419
            pc = 0xc4419; continue;
        }
        0xc4416 => {
            // block c4416
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c4416 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4416 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc4419;
        }
        0xc4419 => {
            // block c4419
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c4419 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4419 mov rax,rsi
            rax = rsi;
            // c441c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c441d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4dac size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4dac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4dac;
    loop {
        match pc {
        0xc4dac => {
            // block c4dac
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4dad mov rbp,rsp
            rbp = rsp;
            // c4db0 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c4db4 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4db7 je 0xc4dea
            if crate::cov::br(fl.e()) { pc = 0xc4dea; continue; }
            pc = 0xc4db9;
        }
        0xc4db9 => {
            // block c4db9
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4db9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4db9 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c4dbb jmp 0xc4dc0
            pc = 0xc4dc0; continue;
        }
        0xc4dbd => {
            // block c4dbd
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dbd mov rax,rdx
            rax = rdx;
            pc = 0xc4dc0;
        }
        0xc4dc0 => {
            // block c4dc0
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dc0 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c4dc3 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c4dc5 jae 0xc4dd1
            if crate::cov::br(fl.ae()) { pc = 0xc4dd1; continue; }
            pc = 0xc4dc7;
        }
        0xc4dc7 => {
            // block c4dc7
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dc7 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c4dca test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4dcd jne 0xc4dbd
            if crate::cov::br(fl.ne()) { pc = 0xc4dbd; continue; }
            pc = 0xc4dcf;
        }
        0xc4dcf => {
            // block c4dcf
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dcf jmp 0xc4df6
            pc = 0xc4df6; continue;
        }
        0xc4dd1 => {
            // block c4dd1
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dd1 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c4dd3 jae 0xc4dfe
            if crate::cov::br(fl.ae()) { pc = 0xc4dfe; continue; }
            pc = 0xc4dd5;
        }
        0xc4dd5 => {
            // block c4dd5
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dd5 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // c4dd9 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4ddc jne 0xc4dbd
            if crate::cov::br(fl.ne()) { pc = 0xc4dbd; continue; }
            pc = 0xc4dde;
        }
        0xc4dde => {
            // block c4dde
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dde mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c4de1 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c4de5 mov rsi,rax
            rsi = rax;
            // c4de8 jmp 0xc4e01
            pc = 0xc4e01; continue;
        }
        0xc4dea => {
            // block c4dea
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dea add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c4dee mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c4df1 mov rsi,rdi
            rsi = rdi;
            // c4df4 jmp 0xc4e01
            pc = 0xc4e01; continue;
        }
        0xc4df6 => {
            // block c4df6
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4df6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4df6 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c4df9 mov rsi,rax
            rsi = rax;
            // c4dfc jmp 0xc4e01
            pc = 0xc4e01; continue;
        }
        0xc4dfe => {
            // block c4dfe
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dfe mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc4e01;
        }
        0xc4e01 => {
            // block c4e01
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4e01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4e01 mov rax,rsi
            rax = rsi;
            // c4e04 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4e05 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3cdc size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003cdc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3cdc;
    loop {
        match pc {
        0x3cdc => {
            // block 3cdc
            #[cfg(feature = "trace")] eprintln!("f_00003cdc pc 3cdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3cdc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3cdd mov rbp,rsp
            rbp = rsp;
            // 3ce0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3ce2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3ce3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 3ce7 mov rbx,rdi
            rbx = rdi;
            // 3cea mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3ced lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 3cf1 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // 3cf6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3cfb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3cfe call QWORD PTR [rax+0x38]
            { m.outregs(0x3d01, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d01 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 3d08 lea r14,[rbp-0x18]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            pc = 0x3d0c;
        }
        0x3d0c => {
            // block 3d0c
            #[cfg(feature = "trace")] eprintln!("f_00003cdc pc 3d0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d0c mov DWORD PTR [rbp-0x18],0x100
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0x100u64) as u32);
            // 3d13 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3d16 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3d1b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3d20 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3d23 mov rdi,rbx
            rdi = rbx;
            // 3d26 mov rdx,r14
            rdx = r14;
            // 3d29 call QWORD PTR [rax+0x38]
            { m.outregs(0x3d2c, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d2c test BYTE PTR [rbp-0x17],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe9)) as u64 & 0x1u64);
            // 3d30 jne 0x3d0c
            if crate::cov::br(fl.ne()) { pc = 0x3d0c; continue; }
            pc = 0x3d32;
        }
        0x3d32 => {
            // block 3d32
            #[cfg(feature = "trace")] eprintln!("f_00003cdc pc 3d32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d32 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 3d34 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 3d38 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3d39 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3d3b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3d3c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dc9c size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dc9c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2dc9c;
    loop {
        match pc {
        0x2dc9c => {
            // block 2dc9c
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dc9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dc9c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dc9d mov rbp,rsp
            rbp = rsp;
            // 2dca0 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2dca4 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2dca7 je 0x2dcda
            if crate::cov::br(fl.e()) { pc = 0x2dcda; continue; }
            pc = 0x2dca9;
        }
        0x2dca9 => {
            // block 2dca9
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dca9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dca9 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 2dcab jmp 0x2dcb0
            pc = 0x2dcb0; continue;
        }
        0x2dcad => {
            // block 2dcad
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcad mov rax,rdx
            rax = rdx;
            pc = 0x2dcb0;
        }
        0x2dcb0 => {
            // block 2dcb0
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcb0 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 2dcb3 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 2dcb5 jae 0x2dcc1
            if crate::cov::br(fl.ae()) { pc = 0x2dcc1; continue; }
            pc = 0x2dcb7;
        }
        0x2dcb7 => {
            // block 2dcb7
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcb7 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 2dcba test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2dcbd jne 0x2dcad
            if crate::cov::br(fl.ne()) { pc = 0x2dcad; continue; }
            pc = 0x2dcbf;
        }
        0x2dcbf => {
            // block 2dcbf
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcbf jmp 0x2dce6
            pc = 0x2dce6; continue;
        }
        0x2dcc1 => {
            // block 2dcc1
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcc1 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 2dcc3 jae 0x2dcee
            if crate::cov::br(fl.ae()) { pc = 0x2dcee; continue; }
            pc = 0x2dcc5;
        }
        0x2dcc5 => {
            // block 2dcc5
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcc5 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2dcc9 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2dccc jne 0x2dcad
            if crate::cov::br(fl.ne()) { pc = 0x2dcad; continue; }
            pc = 0x2dcce;
        }
        0x2dcce => {
            // block 2dcce
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcce mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2dcd1 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 2dcd5 mov rsi,rax
            rsi = rax;
            // 2dcd8 jmp 0x2dcf1
            pc = 0x2dcf1; continue;
        }
        0x2dcda => {
            // block 2dcda
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcda add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 2dcde mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 2dce1 mov rsi,rdi
            rsi = rdi;
            // 2dce4 jmp 0x2dcf1
            pc = 0x2dcf1; continue;
        }
        0x2dce6 => {
            // block 2dce6
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dce6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dce6 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2dce9 mov rsi,rax
            rsi = rax;
            // 2dcec jmp 0x2dcf1
            pc = 0x2dcf1; continue;
        }
        0x2dcee => {
            // block 2dcee
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcee mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x2dcf1;
        }
        0x2dcf1 => {
            // block 2dcf1
            #[cfg(feature = "trace")] eprintln!("f_0002dc9c pc 2dcf1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dcf1 mov rax,rsi
            rax = rsi;
            // 2dcf4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dcf5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6602 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6602<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb6602;
    loop {
        match pc {
        0xb6602 => {
            // block b6602
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6602 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6602 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6603 mov rbp,rsp
            rbp = rsp;
            // b6606 mov r8,QWORD PTR [rdi+0x8]
            r8 = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b660a add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b660e test r8,r8
            fl = logic::<64>(r8 & r8);
            // b6611 je 0xb665e
            if crate::cov::br(fl.e()) { pc = 0xb665e; continue; }
            pc = 0xb6613;
        }
        0xb6613 => {
            // block b6613
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6613 mov edx,DWORD PTR [rsi]
            rdx = (m.r32(rsi) as u64) as u32 as u64;
            // b6615 mov ecx,DWORD PTR [rsi+0x4]
            rcx = (m.r32(rsi.wrapping_add(0x4)) as u64) as u32 as u64;
            // b6618 mov rax,rdi
            rax = rdi;
            pc = 0xb661b;
        }
        0xb661b => {
            // block b661b
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b661b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b661b mov r9,rax
            r9 = rax;
            // b661e mov rax,r8
            rax = r8;
            pc = 0xb6621;
        }
        0xb6621 => {
            // block b6621
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6621 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6621 mov esi,DWORD PTR [rax+0x1c]
            rsi = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b6624 cmp esi,edx
            fl = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64)).1;
            // b6626 jb 0xb6631
            if crate::cov::br(fl.b()) { pc = 0xb6631; continue; }
            pc = 0xb6628;
        }
        0xb6628 => {
            // block b6628
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6628 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6628 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // b662a jb 0xb663c
            if crate::cov::br(fl.b()) { pc = 0xb663c; continue; }
            pc = 0xb662c;
        }
        0xb662c => {
            // block b662c
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b662c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b662c cmp DWORD PTR [rax+0x20],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x20)) as u64, (rcx as u32 as u64)).1;
            // b662f jae 0xb663c
            if crate::cov::br(fl.ae()) { pc = 0xb663c; continue; }
            pc = 0xb6631;
        }
        0xb6631 => {
            // block b6631
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6631 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6631 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // b6635 test rax,rax
            fl = logic::<64>(rax & rax);
            // b6638 jne 0xb6621
            if crate::cov::br(fl.ne()) { pc = 0xb6621; continue; }
            pc = 0xb663a;
        }
        0xb663a => {
            // block b663a
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b663a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b663a jmp 0xb6646
            pc = 0xb6646; continue;
        }
        0xb663c => {
            // block b663c
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b663c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b663c mov r8,QWORD PTR [rax]
            r8 = m.r64(rax) as u64;
            // b663f test r8,r8
            fl = logic::<64>(r8 & r8);
            // b6642 jne 0xb661b
            if crate::cov::br(fl.ne()) { pc = 0xb661b; continue; }
            pc = 0xb6644;
        }
        0xb6644 => {
            // block b6644
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6644 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6644 jmp 0xb6649
            pc = 0xb6649; continue;
        }
        0xb6646 => {
            // block b6646
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6646 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6646 mov rax,r9
            rax = r9;
            pc = 0xb6649;
        }
        0xb6649 => {
            // block b6649
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6649 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6649 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // b664c je 0xb665e
            if crate::cov::br(fl.e()) { pc = 0xb665e; continue; }
            pc = 0xb664e;
        }
        0xb664e => {
            // block b664e
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b664e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b664e mov esi,DWORD PTR [rax+0x1c]
            rsi = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b6651 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // b6653 jb 0xb665e
            if crate::cov::br(fl.b()) { pc = 0xb665e; continue; }
            pc = 0xb6655;
        }
        0xb6655 => {
            // block b6655
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6655 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6655 cmp esi,edx
            fl = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64)).1;
            // b6657 jb 0xb6661
            if crate::cov::br(fl.b()) { pc = 0xb6661; continue; }
            pc = 0xb6659;
        }
        0xb6659 => {
            // block b6659
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6659 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6659 cmp ecx,DWORD PTR [rax+0x20]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x20)) as u64).1;
            // b665c jae 0xb6661
            if crate::cov::br(fl.ae()) { pc = 0xb6661; continue; }
            pc = 0xb665e;
        }
        0xb665e => {
            // block b665e
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b665e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b665e mov rax,rdi
            rax = rdi;
            pc = 0xb6661;
        }
        0xb6661 => {
            // block b6661
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6661 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6661 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6662 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc5762 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c5762<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc5762;
    loop {
        match pc {
        0xc5762 => {
            // block c5762
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c5762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5762 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c5763 mov rbp,rsp
            rbp = rsp;
            // c5766 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c5768 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c576a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c576b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c576c mov rbx,rsi
            rbx = rsi;
            // c576f mov r14,rdi
            r14 = rdi;
            // c5772 mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // c5777 call 0xec146
            { m.outregs(0xc577c, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c577c mov r15,rax
            r15 = rax;
            // c577f mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // c5781 mov DWORD PTR [r15+0x1c],eax
            m.w32(r15.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c5785 mov ecx,DWORD PTR [rbx+0x4]
            rcx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // c5788 mov DWORD PTR [r15+0x20],ecx
            m.w32(r15.wrapping_add(0x20), ((rcx as u32 as u64)) as u32);
            // c578c mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // c5790 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // c5793 jne 0xc579e
            if crate::cov::br(fl.ne()) { pc = 0xc579e; continue; }
            pc = 0xc5795;
        }
        0xc5795 => {
            // block c5795
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c5795 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5795 lea rsi,[r14+0x8]
            rsi = r14.wrapping_add(0x8);
            // c5799 jmp 0xc57bc
            pc = 0xc57bc; continue;
        }
        0xc579b => {
            // block c579b
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c579b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c579b mov rsi,rcx
            rsi = rcx;
            pc = 0xc579e;
        }
        0xc579e => {
            // block c579e
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c579e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c579e cmp eax,DWORD PTR [rsi+0x1c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rsi.wrapping_add(0x1c)) as u64).1;
            // c57a1 jge 0xc57ad
            if crate::cov::br(fl.ge()) { pc = 0xc57ad; continue; }
            pc = 0xc57a3;
        }
        0xc57a3 => {
            // block c57a3
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57a3 mov rcx,QWORD PTR [rsi]
            rcx = m.r64(rsi) as u64;
            // c57a6 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c57a9 jne 0xc579b
            if crate::cov::br(fl.ne()) { pc = 0xc579b; continue; }
            pc = 0xc57ab;
        }
        0xc57ab => {
            // block c57ab
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57ab jmp 0xc57bc
            pc = 0xc57bc; continue;
        }
        0xc57ad => {
            // block c57ad
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57ad mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c57b1 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c57b4 jne 0xc579b
            if crate::cov::br(fl.ne()) { pc = 0xc579b; continue; }
            pc = 0xc57b6;
        }
        0xc57b6 => {
            // block c57b6
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57b6 lea rdx,[rsi+0x8]
            rdx = rsi.wrapping_add(0x8);
            // c57ba jmp 0xc57bf
            pc = 0xc57bf; continue;
        }
        0xc57bc => {
            // block c57bc
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57bc mov rdx,rsi
            rdx = rsi;
            pc = 0xc57bf;
        }
        0xc57bf => {
            // block c57bf
            #[cfg(feature = "trace")] eprintln!("f_000c5762 pc c57bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c57bf mov rdi,r14
            rdi = r14;
            // c57c2 mov rcx,r15
            rcx = r15;
            // c57c5 call 0xebdc2
            { m.outregs(0xc57ca, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x58f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c57ca mov rax,r15
            rax = r15;
            // c57cd add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c57d1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c57d2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c57d4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c57d6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c57d7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc58c2 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c58c2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc58c2;
    loop {
        match pc {
        0xc58c2 => {
            // block c58c2
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c58c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c58c2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c58c3 mov rbp,rsp
            rbp = rsp;
            // c58c6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c58c8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c58ca push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c58cb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c58cc mov rbx,rsi
            rbx = rsi;
            // c58cf mov r14,rdi
            r14 = rdi;
            // c58d2 mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // c58d7 call 0xec146
            { m.outregs(0xc58dc, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c58dc mov r15,rax
            r15 = rax;
            // c58df mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // c58e1 mov DWORD PTR [r15+0x1c],eax
            m.w32(r15.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c58e5 mov ecx,DWORD PTR [rbx+0x4]
            rcx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // c58e8 mov DWORD PTR [r15+0x20],ecx
            m.w32(r15.wrapping_add(0x20), ((rcx as u32 as u64)) as u32);
            // c58ec mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // c58f0 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // c58f3 jne 0xc58fe
            if crate::cov::br(fl.ne()) { pc = 0xc58fe; continue; }
            pc = 0xc58f5;
        }
        0xc58f5 => {
            // block c58f5
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c58f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c58f5 lea rsi,[r14+0x8]
            rsi = r14.wrapping_add(0x8);
            // c58f9 jmp 0xc591c
            pc = 0xc591c; continue;
        }
        0xc58fb => {
            // block c58fb
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c58fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c58fb mov rsi,rcx
            rsi = rcx;
            pc = 0xc58fe;
        }
        0xc58fe => {
            // block c58fe
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c58fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c58fe cmp eax,DWORD PTR [rsi+0x1c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rsi.wrapping_add(0x1c)) as u64).1;
            // c5901 jge 0xc590d
            if crate::cov::br(fl.ge()) { pc = 0xc590d; continue; }
            pc = 0xc5903;
        }
        0xc5903 => {
            // block c5903
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c5903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5903 mov rcx,QWORD PTR [rsi]
            rcx = m.r64(rsi) as u64;
            // c5906 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c5909 jne 0xc58fb
            if crate::cov::br(fl.ne()) { pc = 0xc58fb; continue; }
            pc = 0xc590b;
        }
        0xc590b => {
            // block c590b
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c590b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c590b jmp 0xc591c
            pc = 0xc591c; continue;
        }
        0xc590d => {
            // block c590d
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c590d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c590d mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c5911 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c5914 jne 0xc58fb
            if crate::cov::br(fl.ne()) { pc = 0xc58fb; continue; }
            pc = 0xc5916;
        }
        0xc5916 => {
            // block c5916
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c5916 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5916 lea rdx,[rsi+0x8]
            rdx = rsi.wrapping_add(0x8);
            // c591a jmp 0xc591f
            pc = 0xc591f; continue;
        }
        0xc591c => {
            // block c591c
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c591c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c591c mov rdx,rsi
            rdx = rsi;
            pc = 0xc591f;
        }
        0xc591f => {
            // block c591f
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c591f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c591f mov rdi,r14
            rdi = r14;
            // c5922 mov rcx,r15
            rcx = r15;
            // c5925 call 0xebdda
            { m.outregs(0xc592a, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x5a7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c592a mov rax,r15
            rax = r15;
            // c592d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c5931 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5932 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c5934 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c5936 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c5937 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe494e size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e494e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe494e;
    loop {
        match pc {
        0xe494e => {
            // block e494e
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e494e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e494e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e494f mov rbp,rsp
            rbp = rsp;
            // e4952 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4954 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4956 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4957 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e4958 mov r15,rdi
            r15 = rdi;
            // e495b mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e495e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e4961 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e4965 mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // e496a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e496f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4972 call QWORD PTR [rax+0x38]
            { m.outregs(0xe4975, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4975 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e4978 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // e497b test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e497d je 0xe49b6
            if crate::cov::br(fl.e()) { pc = 0xe49b6; continue; }
            pc = 0xe497f;
        }
        0xe497f => {
            // block e497f
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e497f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e497f mov ebx,0xffffffec
            rbx = (0xffffffecu64) as u32 as u64;
            pc = 0xe4984;
        }
        0xe4984 => {
            // block e4984
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e4984 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4984 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e4987 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e498a call QWORD PTR [rax+0x1880]
            { m.outregs(0xe4990, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1880)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4990 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e4992 jne 0xe49b6
            if crate::cov::br(fl.ne()) { pc = 0xe49b6; continue; }
            pc = 0xe4994;
        }
        0xe4994 => {
            // block e4994
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e4994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4994 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e4997 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e499a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e499c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // e49a1 call QWORD PTR [rax+0xbd0]
            { m.outregs(0xe49a7, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e49a7 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // e49a9 jne 0xe4984
            if crate::cov::br(fl.ne()) { pc = 0xe4984; continue; }
            pc = 0xe49ab;
        }
        0xe49ab => {
            // block e49ab
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e49ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e49ab mov DWORD PTR [r15+0x8],0x16
            m.w32(r15.wrapping_add(0x8), (0x16u64) as u32);
            // e49b3 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe49b6;
        }
        0xe49b6 => {
            // block e49b6
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e49b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e49b6 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // e49b9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e49bd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e49be pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e49c0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e49c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e49c3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bb44 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bb44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2bb44;
    loop {
        match pc {
        0x2bb44 => {
            // block 2bb44
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bb45 mov rbp,rsp
            rbp = rsp;
            // 2bb48 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bb4a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bb4c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2bb4e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bb4f mov r12,rsi
            r12 = rsi;
            // 2bb52 mov r14,rdi
            r14 = rdi;
            // 2bb55 lea rbx,[r12+0x8]
            rbx = r12.wrapping_add(0x8);
            // 2bb5a mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2bb5f mov rdi,r12
            rdi = r12;
            // 2bb62 call 0xebcd2
            { m.outregs(0x2bb67, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bb67 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 2bb70 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 2bb74 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 2bb7d mov r15,QWORD PTR [r14+0x50]
            r15 = m.r64(r14.wrapping_add(0x50)) as u64;
            // 2bb81 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2bb84 je 0x2bb9b
            if crate::cov::br(fl.e()) { pc = 0x2bb9b; continue; }
            pc = 0x2bb86;
        }
        0x2bb86 => {
            // block 2bb86
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb86 cmp DWORD PTR [r14+0x58],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x58)) as u64, 0x0u64).1;
            // 2bb8b je 0x2bb9b
            if crate::cov::br(fl.e()) { pc = 0x2bb9b; continue; }
            pc = 0x2bb8d;
        }
        0x2bb8d => {
            // block 2bb8d
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb8d mov eax,DWORD PTR [r14+0x4c]
            rax = (m.r32(r14.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 2bb91 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2bb93 je 0x2bb9b
            if crate::cov::br(fl.e()) { pc = 0x2bb9b; continue; }
            pc = 0x2bb95;
        }
        0x2bb95 => {
            // block 2bb95
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb95 cmp eax,DWORD PTR [r14+0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x20)) as u64).1;
            // 2bb99 jbe 0x2bba6
            if crate::cov::br(fl.be()) { pc = 0x2bba6; continue; }
            pc = 0x2bb9b;
        }
        0x2bb9b => {
            // block 2bb9b
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb9b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bb9d;
        }
        0x2bb9d => {
            // block 2bb9d
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb9d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bb9e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2bba0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bba2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2bba4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bba5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2bba6 => {
            // block 2bba6
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bba6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bba6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bba8;
        }
        0x2bba8 => {
            // block 2bba8
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bba8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bba8 mov esi,DWORD PTR [r15+rbx*4]
            rsi = (m.r32(r15.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2bbac mov rdi,r12
            rdi = r12;
            // 2bbaf call 0x60955
            { m.outregs(0x2bbb4, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x60955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bbb4 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2bbb7 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2bbb9 cmp ebx,DWORD PTR [r14+0x58]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x58)) as u64).1;
            // 2bbbd jb 0x2bba8
            if crate::cov::br(fl.b()) { pc = 0x2bba8; continue; }
            pc = 0x2bbbf;
        }
        0x2bbbf => {
            // block 2bbbf
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bbbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bbbf jmp 0x2bb9d
            pc = 0x2bb9d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e76e size=128
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e76e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e76e;
    loop {
        match pc {
        0x2e76e => {
            // block 2e76e
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e76e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e76e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e76f mov rbp,rsp
            rbp = rsp;
            // 2e772 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e774 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e776 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e777 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2e778 mov r14,rsi
            r14 = rsi;
            // 2e77b mov r15,rdi
            r15 = rdi;
            // 2e77e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e781 call QWORD PTR [rax+0x70]
            { m.outregs(0x2e784, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x70)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e784 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e786 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2e789 je 0x2e7e3
            if crate::cov::br(fl.e()) { pc = 0x2e7e3; continue; }
            pc = 0x2e78b;
        }
        0x2e78b => {
            // block 2e78b
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e78b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e78b mov rbx,QWORD PTR [r15+0x10]
            rbx = m.r64(r15.wrapping_add(0x10)) as u64;
            // 2e78f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e791 cmp rbx,QWORD PTR [r15+0x18]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x18)) as u64).1;
            // 2e795 je 0x2e7e3
            if crate::cov::br(fl.e()) { pc = 0x2e7e3; continue; }
            pc = 0x2e797;
        }
        0x2e797 => {
            // block 2e797
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e797 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e797 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 2e79a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2e79d call QWORD PTR [rax+0x18]
            { m.outregs(0x2e7a0, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e7a0 mov rdi,r14
            rdi = r14;
            // 2e7a3 mov rsi,rax
            rsi = rax;
            // 2e7a6 call 0xec2ae
            { m.outregs(0x2e7ab, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e7ab test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e7ad je 0x2e7e0
            if crate::cov::br(fl.e()) { pc = 0x2e7e0; continue; }
            pc = 0x2e7af;
        }
        0x2e7af => {
            // block 2e7af
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7af add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2e7b3 cmp rbx,QWORD PTR [r15+0x18]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x18)) as u64).1;
            // 2e7b7 jne 0x2e797
            if crate::cov::br(fl.ne()) { pc = 0x2e797; continue; }
            pc = 0x2e7b9;
        }
        0x2e7b9 => {
            // block 2e7b9
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7b9 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 2e7bd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e7bf cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 2e7c2 je 0x2e7e3
            if crate::cov::br(fl.e()) { pc = 0x2e7e3; continue; }
            pc = 0x2e7c4;
        }
        0x2e7c4 => {
            // block 2e7c4
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7c4 lea rsi,[rip+0xfc3b5] # 0x12ab80
            rsi = m.env.data_va(258893);
            // 2e7cb mov rdi,r14
            rdi = r14;
            // 2e7ce call 0xec2ae
            { m.outregs(0x2e7d3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e7d3 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 2e7d5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e7d7 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 2e7d9 jne 0x2e7e3
            if crate::cov::br(fl.ne()) { pc = 0x2e7e3; continue; }
            pc = 0x2e7db;
        }
        0x2e7db => {
            // block 2e7db
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7db mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e7de jmp 0x2e7e3
            pc = 0x2e7e3; continue;
        }
        0x2e7e0 => {
            // block 2e7e0
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7e0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            pc = 0x2e7e3;
        }
        0x2e7e3 => {
            // block 2e7e3
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7e3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2e7e7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e7e8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e7ea pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e7ec pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e7ed ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f1f8 size=138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f1f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2f1f8;
    loop {
        match pc {
        0x2f1f8 => {
            // block 2f1f8
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f1f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f1f9 mov rbp,rsp
            rbp = rsp;
            // 2f1fc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f1fe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f1ff sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f203 mov rbx,rdi
            rbx = rdi;
            // 2f206 lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // 2f20a mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f20e mov rdi,r14
            rdi = r14;
            // 2f211 call QWORD PTR [rax+0x108]
            { m.outregs(0x2f217, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f217 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2f219 je 0x2f279
            if crate::cov::br(fl.e()) { pc = 0x2f279; continue; }
            pc = 0x2f21b;
        }
        0x2f21b => {
            // block 2f21b
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f21b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f21b mov rdi,r14
            rdi = r14;
            // 2f21e call 0xb6914
            { m.outregs(0x2f223, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xb6914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f223 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2f226 cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 2f22b jne 0x2f279
            if crate::cov::br(fl.ne()) { pc = 0x2f279; continue; }
            pc = 0x2f22d;
        }
        0x2f22d => {
            // block 2f22d
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f22d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f22d mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f231 mov esi,DWORD PTR [rbx+0x2b4]
            rsi = (m.r32(rbx.wrapping_add(0x2b4)) as u64) as u32 as u64;
            // 2f237 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2f239 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2f23e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f241 mov rdi,r14
            rdi = r14;
            // 2f244 call QWORD PTR [rax+0x30]
            { m.outregs(0x2f247, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f247 mov rdi,rbx
            rdi = rbx;
            // 2f24a call 0x31164
            { m.outregs(0x2f24f, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x31164); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f24f mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // 2f253 lea rsi,[rbp-0x11]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffef);
            // 2f257 mov rdi,rbx
            rdi = rbx;
            // 2f25a call 0x311bc
            { m.outregs(0x2f25f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x311bc); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f25f movzx esi,BYTE PTR [rbp-0x11]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u32 as u64;
            // 2f263 or esi,0x80
            { let r = (rsi as u32 as u64) | 0x80u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2f269 mov rdi,rbx
            rdi = rbx;
            // 2f26c add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f270 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f271 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f273 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f274 jmp 0x31368
            { let r = { let t = m.env.code_va(0x31368); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
        0x2f279 => {
            // block 2f279
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f279 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f279 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f27d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f27e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f280 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f281 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15f84 size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x78d
pub fn f_00015f84<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15f84;
    loop {
        match pc {
        0x15f84 => {
            // block 15f84
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15f84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f84 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15f85 mov rbp,rsp
            rbp = rsp;
            // 15f88 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15f8a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15f8c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15f8e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15f90 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15f91 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15f92 mov r14,rdx
            r14 = rdx;
            // 15f95 mov rbx,rsi
            rbx = rsi;
            // 15f98 mov r15,rdi
            r15 = rdi;
            // 15f9b mov r13,r14
            r13 = r14;
            // 15f9e sub r13,rbx
            { let (r, f) = sub::<64>(r13, rbx);
              r13 = r;
              fl = f; }
            // 15fa1 cmp r13,0xffffffffffffffef
            fl = sub::<64>(r13, 0xffff_ffff_ffff_ffefu64).1;
            // 15fa5 jbe 0x15fb1
            if crate::cov::br(fl.be()) { pc = 0x15fb1; continue; }
            pc = 0x15fa7;
        }
        0x15fa7 => {
            // block 15fa7
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fa7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fa7 mov rdi,r15
            rdi = r15;
            // 15faa call 0xebfc0
            { let _ = { let t = m.env.stub_va(0x78d); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x15fb1 => {
            // block 15fb1
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fb1 cmp r13,0x16
            fl = sub::<64>(r13, 0x16u64).1;
            // 15fb5 ja 0x15fc7
            if crate::cov::br(fl.a()) { pc = 0x15fc7; continue; }
            pc = 0x15fb7;
        }
        0x15fb7 => {
            // block 15fb7
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fb7 lea rax,[r13+r13*1+0x0]
            rax = r13.wrapping_add(r13);
            // 15fbc mov BYTE PTR [r15],al
            m.w8(r15, ((rax as u8 as u64)) as u8);
            // 15fbf inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 15fc2 mov rax,r15
            rax = r15;
            // 15fc5 jmp 0x15fe6
            pc = 0x15fe6; continue;
        }
        0x15fc7 => {
            // block 15fc7
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fc7 lea r12,[r13+0x10]
            r12 = r13.wrapping_add(0x10);
            // 15fcb and r12,0xfffffffffffffff0
            { let r = r12 & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              r12 = r;
              fl = f; }
            // 15fcf mov rdi,r12
            rdi = r12;
            // 15fd2 call 0xec146
            { m.outregs(0x15fd7, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15fd7 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 15fdb or r12,0x1
            { let r = r12 | 0x1u64; let f = logic::<64>(r);
              r12 = r;
              fl = f; }
            // 15fdf mov QWORD PTR [r15],r12
            m.w64(r15, (r12) as u64);
            // 15fe2 mov QWORD PTR [r15+0x8],r13
            m.w64(r15.wrapping_add(0x8), (r13) as u64);
            pc = 0x15fe6;
        }
        0x15fe6 => {
            // block 15fe6
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fe6 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 15fe9 je 0x16000
            if crate::cov::br(fl.e()) { pc = 0x16000; continue; }
            pc = 0x15feb;
        }
        0x15feb => {
            // block 15feb
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15feb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15feb mov rcx,rax
            rcx = rax;
            pc = 0x15fee;
        }
        0x15fee => {
            // block 15fee
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fee mov dl,BYTE PTR [rbx]
            rdx = (rdx & !0xff) | ((m.r8(rbx) as u64) as u8 as u64);
            // 15ff0 mov BYTE PTR [rcx],dl
            m.w8(rcx, ((rdx as u8 as u64)) as u8);
            // 15ff2 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 15ff5 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 15ff8 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 15ffb jne 0x15fee
            if crate::cov::br(fl.ne()) { pc = 0x15fee; continue; }
            pc = 0x15ffd;
        }
        0x15ffd => {
            // block 15ffd
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15ffd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ffd add rax,r13
            { let (r, f) = add::<64>(rax, r13);
              rax = r;
              fl = f; }
            pc = 0x16000;
        }
        0x16000 => {
            // block 16000
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 16000 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16000 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 16003 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 16007 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 16008 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1600a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1600c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1600e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 16010 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 16011 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1729c size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001729c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1729c;
    loop {
        match pc {
        0x1729c => {
            // block 1729c
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 1729c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1729c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1729d mov rbp,rsp
            rbp = rsp;
            // 172a0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 172a2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 172a3 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 172a7 mov r14,rsi
            r14 = rsi;
            // 172aa mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // 172ac lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 172b0 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 172b5 call 0x16080
            { m.outregs(0x172ba, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x16080); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 172ba mov rax,QWORD PTR [rbp-0x28]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 172be mov rcx,QWORD PTR [rbp-0x20]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 172c2 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 172c5 je 0x172d6
            if crate::cov::br(fl.e()) { pc = 0x172d6; continue; }
            pc = 0x172c7;
        }
        0x172c7 => {
            // block 172c7
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172c7 cmp DWORD PTR [rax],ebx
            fl = sub::<32>(m.r32(rax) as u64, (rbx as u32 as u64)).1;
            // 172c9 je 0x172da
            if crate::cov::br(fl.e()) { pc = 0x172da; continue; }
            pc = 0x172cb;
        }
        0x172cb => {
            // block 172cb
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172cb add rax,0xf8
            { let (r, f) = add::<64>(rax, 0xf8u64);
              rax = r;
              fl = f; }
            // 172d1 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 172d4 jne 0x172c7
            if crate::cov::br(fl.ne()) { pc = 0x172c7; continue; }
            pc = 0x172d6;
        }
        0x172d6 => {
            // block 172d6
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172d6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 172d8 jmp 0x172f3
            pc = 0x172f3; continue;
        }
        0x172da => {
            // block 172da
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172da mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 172dd mov rbx,QWORD PTR [rcx+0x10]
            rbx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 172e1 movzx esi,WORD PTR [rax+0x4]
            rsi = (m.r16(rax.wrapping_add(0x4)) as u64) as u32 as u64;
            // 172e5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 172e7 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 172e9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 172ec mov rdi,r14
            rdi = r14;
            // 172ef call rbx
            { m.outregs(0x172f1, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rbx; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 172f1 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x172f3;
        }
        0x172f3 => {
            // block 172f3
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172f3 lea rax,[rip+0x12a11e] # 0x141418
            rax = m.env.data_va(351205);
            // 172fa mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 172fe lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 17302 call 0xeb94e
            { m.outregs(0x17307, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x11b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17307 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 17309 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 1730d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1730e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 17310 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17311 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x17312 => {
            // block 17312
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 17312 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17312 mov rbx,rax
            rbx = rax;
            // 17315 lea rax,[rip+0x12a0fc] # 0x141418
            rax = m.env.data_va(351205);
            // 1731c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 17320 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 17324 call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17329 mov rdi,rbx
            rdi = rbx;
            // 1732c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc623e size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c623e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc623e;
    loop {
        match pc {
        0xc623e => {
            // block c623e
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c623e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c623e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c623f mov rbp,rsp
            rbp = rsp;
            // c6242 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c6244 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c6246 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c6248 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c624a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c624b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c624c mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // c624f mov rbx,rdi
            rbx = rdi;
            // c6252 lea rax,[rip+0x28c1] # 0xc8b1a
            rax = m.env.code_va(0xc8b1a);
            // c6259 lea r12,[rip+0x293d] # 0xc8b9d
            r12 = m.env.code_va(0xc8b9d);
            // c6260 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // c6263 cmovne r12,rax
            if fl.ne() { r12 = rax; }
            // c6267 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c626a jle 0xc62c5
            if crate::cov::br(fl.le()) { pc = 0xc62c5; continue; }
            pc = 0xc626c;
        }
        0xc626c => {
            // block c626c
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c626c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c626c add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c626e and ecx,0xfffffffc
            { let r = (rcx as u32 as u64) & 0xfffffffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c6271 movsxd rax,ecx
            rax = sx::<32>((rcx as u32 as u64)) as u64;
            // c6274 lea r13,[rsi+rax*2]
            r13 = rsi.wrapping_add(rax.wrapping_mul(2));
            // c6278 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc627b;
        }
        0xc627b => {
            // block c627b
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c627b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c627b mov rdi,rbx
            rdi = rbx;
            // c627e lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // c6282 call r12
            { m.outregs(0xc6285, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = r12; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c6285 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // c6289 jne 0xc62ac
            if crate::cov::br(fl.ne()) { pc = 0xc62ac; continue; }
            pc = 0xc628b;
        }
        0xc628b => {
            // block c628b
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c628b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c628b mov ax,WORD PTR [rbp-0x30]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u16 as u64);
            // c628f mov WORD PTR [r13+0x0],ax
            m.w16(r13, ((rax as u16 as u64)) as u16);
            // c6294 mov ax,WORD PTR [rbp-0x2e]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2)) as u64) as u16 as u64);
            // c6298 mov WORD PTR [r13+0x2],ax
            m.w16(r13.wrapping_add(0x2), ((rax as u16 as u64)) as u16);
            // c629d mov ax,WORD PTR [rbp-0x2c]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u16 as u64);
            // c62a1 mov WORD PTR [r13+0x4],ax
            m.w16(r13.wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // c62a6 add r13,0x6
            { let (r, f) = add::<64>(r13, 0x6u64);
              r13 = r;
              fl = f; }
            // c62aa jmp 0xc62b9
            pc = 0xc62b9; continue;
        }
        0xc62ac => {
            // block c62ac
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c62ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62ac mov ax,WORD PTR [rbp-0x2e]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2)) as u64) as u16 as u64);
            // c62b0 mov WORD PTR [r13+0x0],ax
            m.w16(r13, ((rax as u16 as u64)) as u16);
            // c62b5 add r13,0x2
            { let (r, f) = add::<64>(r13, 0x2u64);
              r13 = r;
              fl = f; }
            pc = 0xc62b9;
        }
        0xc62b9 => {
            // block c62b9
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c62b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62b9 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // c62bd inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // c62c0 cmp r14d,r15d
            fl = sub::<32>((r14 as u32 as u64), (r15 as u32 as u64)).1;
            // c62c3 jne 0xc627b
            if crate::cov::br(fl.ne()) { pc = 0xc627b; continue; }
            pc = 0xc62c5;
        }
        0xc62c5 => {
            // block c62c5
            #[cfg(feature = "trace")] eprintln!("f_000c623e pc c62c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62c5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c62c9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c62ca pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c62cc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c62ce pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c62d0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c62d2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c62d3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11282 size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011282<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x11282;
    loop {
        match pc {
        0x11282 => {
            // block 11282
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 11282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11282 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11283 mov rbp,rsp
            rbp = rsp;
            // 11286 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11288 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1128a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1128b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1128c mov r15,rdi
            r15 = rdi;
            // 1128f lea rax,[rip+0x13000a] # 0x1412a0
            rax = m.env.data_va(350829);
            // 11296 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 11299 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1129d mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 112a1 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 112a4 je 0x112c1
            if crate::cov::br(fl.e()) { pc = 0x112c1; continue; }
            pc = 0x112a6;
        }
        0x112a6 => {
            // block 112a6
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112a6 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 112aa test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 112ad je 0x112b8
            if crate::cov::br(fl.e()) { pc = 0x112b8; continue; }
            pc = 0x112af;
        }
        0x112af => {
            // block 112af
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112af mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 112b2 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 112b6 call rax
            { m.outregs(0x112b8, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x112b8;
        }
        0x112b8 => {
            // block 112b8
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112b8 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 112bc cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 112bf jne 0x112a6
            if crate::cov::br(fl.ne()) { pc = 0x112a6; continue; }
            pc = 0x112c1;
        }
        0x112c1 => {
            // block 112c1
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112c1 mov rdi,QWORD PTR [r15+0x40]
            rdi = m.r64(r15.wrapping_add(0x40)) as u64;
            // 112c5 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 112c8 je 0x112e8
            if crate::cov::br(fl.e()) { pc = 0x112e8; continue; }
            pc = 0x112ca;
        }
        0x112ca => {
            // block 112ca
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112ca mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 112cd mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // 112d1 call rax
            { m.outregs(0x112d3, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112d3 mov rdi,QWORD PTR [r15+0x40]
            rdi = m.r64(r15.wrapping_add(0x40)) as u64;
            // 112d7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 112da mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 112de call rax
            { m.outregs(0x112e0, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112e0 mov QWORD PTR [r15+0x40],0x0
            m.w64(r15.wrapping_add(0x40), (0x0u64) as u64);
            pc = 0x112e8;
        }
        0x112e8 => {
            // block 112e8
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112e8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 112eb mov rax,QWORD PTR [rax+0x88]
            rax = m.r64(rax.wrapping_add(0x88)) as u64;
            // 112f2 mov rdi,r15
            rdi = r15;
            // 112f5 call rax
            { m.outregs(0x112f7, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112f7 mov rdi,r15
            rdi = r15;
            // 112fa add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 112fe pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 112ff pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11301 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11303 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11304 jmp 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x11309 => {
            // block 11309
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 11309 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11309 jmp 0x1130b
            pc = 0x1130b; continue;
        }
        0x1130b => {
            // block 1130b
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 1130b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1130b mov r14,rax
            r14 = rax;
            // 1130e mov rdi,r15
            rdi = r15;
            // 11311 call 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11316 mov rdi,r14
            rdi = r14;
            // 11319 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5c8e size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5c8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd5c8e;
    loop {
        match pc {
        0xd5c8e => {
            // block d5c8e
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5c8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5c8f mov rbp,rsp
            rbp = rsp;
            // d5c92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5c93 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d5c94 mov rbx,rdi
            rbx = rdi;
            // d5c97 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // d5c9e mov DWORD PTR [rbp-0x10],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u32);
            // d5ca5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5ca8 lea rdx,[rbp-0x10]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // d5cac mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // d5cb1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5cb6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5cb9 call QWORD PTR [rax+0x38]
            { m.outregs(0xd5cbc, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5cbc mov ecx,DWORD PTR [rbp-0x10]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64) as u32 as u64;
            // d5cbf cmp ecx,0x106466ff
            fl = sub::<32>((rcx as u32 as u64), 0x106466ffu64).1;
            // d5cc5 jg 0xd5cfe
            if crate::cov::br(fl.g()) { pc = 0xd5cfe; continue; }
            pc = 0xd5cc7;
        }
        0xd5cc7 => {
            // block d5cc7
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cc7 cmp ecx,0x10518400
            fl = sub::<32>((rcx as u32 as u64), 0x10518400u64).1;
            // d5ccd je 0xd5cd7
            if crate::cov::br(fl.e()) { pc = 0xd5cd7; continue; }
            pc = 0xd5ccf;
        }
        0xd5ccf => {
            // block d5ccf
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5ccf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ccf cmp ecx,0x10518450
            fl = sub::<32>((rcx as u32 as u64), 0x10518450u64).1;
            // d5cd5 jne 0xd5d21
            if crate::cov::br(fl.ne()) { pc = 0xd5d21; continue; }
            pc = 0xd5cd7;
        }
        0xd5cd7 => {
            // block d5cd7
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cd7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5cda lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d5cde mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // d5ce3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5ce8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5ceb mov rdi,rbx
            rdi = rbx;
            // d5cee call QWORD PTR [rax+0x38]
            { m.outregs(0xd5cf1, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5cf1 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // d5cf4 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d5cf6 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d5cf9 seta al
            rax = (rax & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // d5cfc jmp 0xd5d23
            pc = 0xd5d23; continue;
        }
        0xd5cfe => {
            // block d5cfe
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cfe mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d5d00 lea edx,[rcx-0x10646700]
            rdx = (rcx.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d5d06 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // d5d09 jb 0xd5d23
            if crate::cov::br(fl.b()) { pc = 0xd5d23; continue; }
            pc = 0xd5d0b;
        }
        0xd5d0b => {
            // block d5d0b
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d0b lea edx,[rcx-0x10646705]
            rdx = (rcx.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d5d11 cmp edx,0x2
            fl = sub::<32>((rdx as u32 as u64), 0x2u64).1;
            // d5d14 jb 0xd5d23
            if crate::cov::br(fl.b()) { pc = 0xd5d23; continue; }
            pc = 0xd5d16;
        }
        0xd5d16 => {
            // block d5d16
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d16 add ecx,0xef8ef7b0
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xef8ef7b0u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d5d1c cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // d5d1f jb 0xd5d23
            if crate::cov::br(fl.b()) { pc = 0xd5d23; continue; }
            pc = 0xd5d21;
        }
        0xd5d21 => {
            // block d5d21
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d21 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd5d23;
        }
        0xd5d23 => {
            // block d5d23
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d23 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d5d27 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5d28 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5d29 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2b8dc size=166
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002b8dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2b8dc;
    loop {
        match pc {
        0x2b8dc => {
            // block 2b8dc
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b8dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b8dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b8dd mov rbp,rsp
            rbp = rsp;
            // 2b8e0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2b8e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2b8e3 mov rbx,rsi
            rbx = rsi;
            // 2b8e6 mov r14,rdi
            r14 = rdi;
            // 2b8e9 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 2b8ed mov DWORD PTR [r14+0x20],eax
            m.w32(r14.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // 2b8f1 mov DWORD PTR [r14+0x4c],0x0
            m.w32(r14.wrapping_add(0x4c), (0x0u64) as u32);
            // 2b8f9 lea rdi,[r14+0x24]
            rdi = r14.wrapping_add(0x24);
            // 2b8fd shl rax,0x2
            { let (r, f) = shl::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 2b901 movabs rsi,0x3fffffffc
            rsi = 0x0_0003_ffff_fffcu64;
            // 2b90b and rsi,rax
            { let r = rsi & rax; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 2b90e call 0x28cf2
            { m.outregs(0x2b913, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b913 lea rdi,[r14+0x50]
            rdi = r14.wrapping_add(0x50);
            // 2b917 mov esi,DWORD PTR [r14+0x20]
            rsi = (m.r32(r14.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2b91b shl rsi,0x2
            { let (r, f) = shl::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 2b91f call 0x28cf2
            { m.outregs(0x2b924, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b924 lea rdi,[r14+0x78]
            rdi = r14.wrapping_add(0x78);
            // 2b928 mov esi,DWORD PTR [r14+0x20]
            rsi = (m.r32(r14.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2b92c shl rsi,0x2
            { let (r, f) = shl::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 2b930 call 0x28cf2
            { m.outregs(0x2b935, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b935 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2b938 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2b93c cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 2b93f je 0x2b97b
            if crate::cov::br(fl.e()) { pc = 0x2b97b; continue; }
            pc = 0x2b941;
        }
        0x2b941 => {
            // block 2b941
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b941 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b941 mov rcx,QWORD PTR [r14+0x24]
            rcx = m.r64(r14.wrapping_add(0x24)) as u64;
            // 2b945 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x2b947;
        }
        0x2b947 => {
            // block 2b947
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b947 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b947 mov esi,DWORD PTR [rax+0x1c]
            rsi = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 2b94a mov edi,edx
            rdi = ((rdx as u32 as u64)) as u32 as u64;
            // 2b94c mov DWORD PTR [rcx+rdi*4],esi
            m.w32(rcx.wrapping_add(rdi.wrapping_mul(4)), ((rsi as u32 as u64)) as u32);
            // 2b94f mov rsi,QWORD PTR [rax+0x8]
            rsi = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2b953 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2b956 je 0x2b965
            if crate::cov::br(fl.e()) { pc = 0x2b965; continue; }
            pc = 0x2b958;
        }
        0x2b958 => {
            // block 2b958
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b958 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b958 mov rax,rsi
            rax = rsi;
            // 2b95b mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 2b95e test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2b961 jne 0x2b958
            if crate::cov::br(fl.ne()) { pc = 0x2b958; continue; }
            pc = 0x2b963;
        }
        0x2b963 => {
            // block 2b963
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b963 jmp 0x2b974
            pc = 0x2b974; continue;
        }
        0x2b965 => {
            // block 2b965
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b965 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b965 mov rsi,rax
            rsi = rax;
            pc = 0x2b968;
        }
        0x2b968 => {
            // block 2b968
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b968 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // 2b96c cmp QWORD PTR [rax],rsi
            fl = sub::<64>(m.r64(rax) as u64, rsi).1;
            // 2b96f mov rsi,rax
            rsi = rax;
            // 2b972 jne 0x2b968
            if crate::cov::br(fl.ne()) { pc = 0x2b968; continue; }
            pc = 0x2b974;
        }
        0x2b974 => {
            // block 2b974
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b974 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b974 inc edx
            { let (r, f) = inc::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // 2b976 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 2b979 jne 0x2b947
            if crate::cov::br(fl.ne()) { pc = 0x2b947; continue; }
            pc = 0x2b97b;
        }
        0x2b97b => {
            // block 2b97b
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b97b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b97b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2b97d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2b97e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2b980 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b981 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x12041 size=167
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012041<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x12041;
    loop {
        match pc {
        0x12041 => {
            // block 12041
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12041 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12041 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12042 mov rbp,rsp
            rbp = rsp;
            // 12045 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12047 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12049 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1204b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1204d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1204e push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1204f mov r15,r8
            r15 = r8;
            // 12052 mov r14,rcx
            r14 = rcx;
            // 12055 mov rbx,rdx
            rbx = rdx;
            // 12058 mov r12,rsi
            r12 = rsi;
            // 1205b mov r13,rdi
            r13 = rdi;
            // 1205e jmp 0x12064
            pc = 0x12064; continue;
        }
        0x12060 => {
            // block 12060
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12060 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12060 add r15,0x38
            { let (r, f) = add::<64>(r15, 0x38u64);
              r15 = r;
              fl = f; }
            pc = 0x12064;
        }
        0x12064 => {
            // block 12064
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12064 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12064 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 12067 je 0x12099
            if crate::cov::br(fl.e()) { pc = 0x12099; continue; }
            pc = 0x12069;
        }
        0x12069 => {
            // block 12069
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12069 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12069 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1206c je 0x120b8
            if crate::cov::br(fl.e()) { pc = 0x120b8; continue; }
            pc = 0x1206e;
        }
        0x1206e => {
            // block 1206e
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 1206e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1206e mov eax,DWORD PTR [rbx+0x18]
            rax = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 12071 cmp eax,DWORD PTR [r13+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(r13.wrapping_add(0x18)) as u64).1;
            // 12075 jge 0x12088
            if crate::cov::br(fl.ge()) { pc = 0x12088; continue; }
            pc = 0x12077;
        }
        0x12077 => {
            // block 12077
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12077 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12077 mov rdi,r15
            rdi = r15;
            // 1207a mov rsi,rbx
            rsi = rbx;
            // 1207d call 0xe064a
            { m.outregs(0x12082, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12082 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12086 jmp 0x12060
            pc = 0x12060; continue;
        }
        0x12088 => {
            // block 12088
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12088 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12088 mov rdi,r15
            rdi = r15;
            // 1208b mov rsi,r13
            rsi = r13;
            // 1208e call 0xe064a
            { m.outregs(0x12093, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12093 add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 12097 jmp 0x12060
            pc = 0x12060; continue;
        }
        0x12099 => {
            // block 12099
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12099 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12099 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1209c je 0x120d9
            if crate::cov::br(fl.e()) { pc = 0x120d9; continue; }
            pc = 0x1209e;
        }
        0x1209e => {
            // block 1209e
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 1209e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1209e mov rdi,r15
            rdi = r15;
            // 120a1 mov rsi,rbx
            rsi = rbx;
            // 120a4 call 0xe064a
            { m.outregs(0x120a9, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 120a9 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 120ad add r15,0x38
            { let (r, f) = add::<64>(r15, 0x38u64);
              r15 = r;
              fl = f; }
            // 120b1 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 120b4 jne 0x1209e
            if crate::cov::br(fl.ne()) { pc = 0x1209e; continue; }
            pc = 0x120b6;
        }
        0x120b6 => {
            // block 120b6
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120b6 jmp 0x120d9
            pc = 0x120d9; continue;
        }
        0x120b8 => {
            // block 120b8
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120b8 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 120bb je 0x120d9
            if crate::cov::br(fl.e()) { pc = 0x120d9; continue; }
            pc = 0x120bd;
        }
        0x120bd => {
            // block 120bd
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120bd sub r12,r13
            { let (r, f) = sub::<64>(r12, r13);
              r12 = r;
              fl = f; }
            // 120c0 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x120c2;
        }
        0x120c2 => {
            // block 120c2
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120c2 lea rsi,[r13+rbx*1+0x0]
            rsi = r13.wrapping_add(rbx);
            // 120c7 lea rdi,[r15+rbx*1]
            rdi = r15.wrapping_add(rbx);
            // 120cb call 0xe064a
            { m.outregs(0x120d0, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 120d0 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 120d4 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 120d7 jne 0x120c2
            if crate::cov::br(fl.ne()) { pc = 0x120c2; continue; }
            pc = 0x120d9;
        }
        0x120d9 => {
            // block 120d9
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120d9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 120dd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 120de pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 120e0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 120e2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 120e4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 120e6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 120e7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd61a size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d61a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd61a;
    loop {
        match pc {
        0xd61a => {
            // block d61a
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d61a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d61a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d61b mov rbp,rsp
            rbp = rsp;
            // d61e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d620 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d622 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d624 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d625 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d629 mov r14,rdx
            r14 = rdx;
            // d62c mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d62e mov r12,rdi
            r12 = rdi;
            // d631 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d636 call 0xd0fb9
            { m.outregs(0xd63b, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd0fb9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d63b movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d63e cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d640 jbe 0xd6b5
            if crate::cov::br(fl.be()) { pc = 0xd6b5; continue; }
            pc = 0xd642;
        }
        0xd642 => {
            // block d642
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d642 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // d649 mov r15d,0x10
            r15 = (0x10u64) as u32 as u64;
            // d64f test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d652 je 0xd662
            if crate::cov::br(fl.e()) { pc = 0xd662; continue; }
            pc = 0xd654;
        }
        0xd654 => {
            // block d654
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d654 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d654 movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d657 mov r15d,0x14
            r15 = (0x14u64) as u32 as u64;
            // d65d cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d660 jne 0xd6b1
            if crate::cov::br(fl.ne()) { pc = 0xd6b1; continue; }
            pc = 0xd662;
        }
        0xd662 => {
            // block d662
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d662 mov BYTE PTR [rbp-0x25],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), (0x0u64) as u8);
            // d666 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d66a lea rsi,[rbp-0x25]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdb);
            // d66e mov rdi,r12
            rdi = r12;
            // d671 call QWORD PTR [rax+0x1820]
            { m.outregs(0xd677, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1820)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d677 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d679 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d67b cmp BYTE PTR [rbp-0x25],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb)) as u64, 0x0u64).1;
            // d67f je 0xd6b7
            if crate::cov::br(fl.e()) { pc = 0xd6b7; continue; }
            pc = 0xd681;
        }
        0xd681 => {
            // block d681
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d681 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d681 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d684 jne 0xd6b7
            if crate::cov::br(fl.ne()) { pc = 0xd6b7; continue; }
            pc = 0xd686;
        }
        0xd686 => {
            // block d686
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d686 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d686 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d68a lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // d68e mov esi,0xe9
            rsi = (0xe9u64) as u32 as u64;
            // d693 mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // d698 mov rdi,r12
            rdi = r12;
            // d69b mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // d69e call QWORD PTR [rax+0x38]
            { m.outregs(0xd6a1, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6a3 je 0xd6b5
            if crate::cov::br(fl.e()) { pc = 0xd6b5; continue; }
            pc = 0xd6a5;
        }
        0xd6a5 => {
            // block d6a5
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a5 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // d6a8 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d6aa mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // d6ad mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d6af jmp 0xd6b7
            pc = 0xd6b7; continue;
        }
        0xd6b1 => {
            // block d6b1
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b1 mov BYTE PTR [rbp-0x25],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), (0x0u64) as u8);
            pc = 0xd6b5;
        }
        0xd6b5 => {
            // block d6b5
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd6b7;
        }
        0xd6b7 => {
            // block d6b7
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d6bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6bc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6be pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6c0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d6c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6c3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3914 size=172
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003914<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3914;
    loop {
        match pc {
        0x3914 => {
            // block 3914
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 3914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3914 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3915 mov rbp,rsp
            rbp = rsp;
            // 3918 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 391a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 391c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 391e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 3920 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3921 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3922 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 3925 mov r12,rdi
            r12 = rdi;
            // 3928 shl r14d,0xe
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0xeu64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 392c and r14d,0x3fffc000
            { let r = (r14 as u32 as u64) & 0x3fffc000u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 3933 je 0x39ab
            if crate::cov::br(fl.e()) { pc = 0x39ab; continue; }
            pc = 0x3935;
        }
        0x3935 => {
            // block 3935
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 3935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3935 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 3938 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 393a xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x393d;
        }
        0x393d => {
            // block 393d
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 393d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 393d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3941 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 3946 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 394b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 394e mov rdi,r12
            rdi = r12;
            // 3951 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 3953 call QWORD PTR [rax+0x30]
            { m.outregs(0x3956, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3956 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 395a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 395f mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 3964 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3969 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 396c mov rdi,r12
            rdi = r12;
            // 396f call QWORD PTR [rax+0x30]
            { m.outregs(0x3972, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3972 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3976 mov rdi,r12
            rdi = r12;
            // 3979 call QWORD PTR [rax+0x9e0]
            { m.outregs(0x397f, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 397f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3983 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // 3988 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 398d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3990 mov rdi,r12
            rdi = r12;
            // 3993 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 3997 call QWORD PTR [rax+0x38]
            { m.outregs(0x399a, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 399a cmp DWORD PTR [rbp-0x2c],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64, 0xffffffffu64).1;
            // 399e jne 0x39ae
            if crate::cov::br(fl.ne()) { pc = 0x39ae; continue; }
            pc = 0x39a0;
        }
        0x39a0 => {
            // block 39a0
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39a0 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 39a3 add ebx,0x4
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x4u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 39a6 cmp r15d,r14d
            fl = sub::<32>((r15 as u32 as u64), (r14 as u32 as u64)).1;
            // 39a9 jb 0x393d
            if crate::cov::br(fl.b()) { pc = 0x393d; continue; }
            pc = 0x39ab;
        }
        0x39ab => {
            // block 39ab
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39ab mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x39ae;
        }
        0x39ae => {
            // block 39ae
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39ae mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 39b1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 39b5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 39b6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 39b8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 39ba pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 39bc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 39be pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 39bf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd386 size=172
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d386<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd386;
    loop {
        match pc {
        0xd386 => {
            // block d386
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d386 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d386 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d387 mov rbp,rsp
            rbp = rsp;
            // d38a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d38c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d38e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d390 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d391 mov r14,rdx
            r14 = rdx;
            // d394 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d396 mov r12,rdi
            r12 = rdi;
            // d399 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d39e call 0xd0ea0
            { m.outregs(0xd3a3, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd0ea0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3a3 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d3a6 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d3a8 jbe 0xd3cb
            if crate::cov::br(fl.be()) { pc = 0xd3cb; continue; }
            pc = 0xd3aa;
        }
        0xd3aa => {
            // block d3aa
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3aa test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d3ad je 0xd3cf
            if crate::cov::br(fl.e()) { pc = 0xd3cf; continue; }
            pc = 0xd3af;
        }
        0xd3af => {
            // block d3af
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3af movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d3b2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3b4 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d3b7 mov esi,0xfe
            rsi = (0xfeu64) as u32 as u64;
            // d3bc cmovne esi,ecx
            if fl.ne() { rsi = ((rcx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // d3bf mov r15d,0xff
            r15 = (0xffu64) as u32 as u64;
            // d3c5 cmovne r15d,ecx
            if fl.ne() { r15 = ((rcx as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // d3c9 jmp 0xd3da
            pc = 0xd3da; continue;
        }
        0xd3cb => {
            // block d3cb
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cb xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3cd jmp 0xd426
            pc = 0xd426; continue;
        }
        0xd3cf => {
            // block d3cf
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cf mov r15d,0x71
            r15 = (0x71u64) as u32 as u64;
            // d3d5 mov esi,0x70
            rsi = (0x70u64) as u32 as u64;
            pc = 0xd3da;
        }
        0xd3da => {
            // block d3da
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3da mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // d3e1 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3e3 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // d3e5 je 0xd426
            if crate::cov::br(fl.e()) { pc = 0xd426; continue; }
            pc = 0xd3e7;
        }
        0xd3e7 => {
            // block d3e7
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e7 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // d3ea je 0xd426
            if crate::cov::br(fl.e()) { pc = 0xd426; continue; }
            pc = 0xd3ec;
        }
        0xd3ec => {
            // block d3ec
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3ec lea rdx,[r14+0x4]
            rdx = r14.wrapping_add(0x4);
            // d3f0 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d3f4 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3f6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d3fb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d3fe mov rdi,r12
            rdi = r12;
            // d401 call QWORD PTR [rax+0x38]
            { m.outregs(0xd404, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d404 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d406 je 0xd426
            if crate::cov::br(fl.e()) { pc = 0xd426; continue; }
            pc = 0xd408;
        }
        0xd408 => {
            // block d408
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d408 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d408 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // d40c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d410 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d415 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d418 mov rdi,r12
            rdi = r12;
            // d41b mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d41e mov rdx,r14
            rdx = r14;
            // d421 call QWORD PTR [rax+0x38]
            { m.outregs(0xd424, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d424 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xd426;
        }
        0xd426 => {
            // block d426
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d426 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d426 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d428 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d429 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d42b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d42d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d42f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d430 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44320 size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044320<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44320;
    loop {
        match pc {
        0x44320 => {
            // block 44320
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 44320 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44320 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44321 mov rbp,rsp
            rbp = rsp;
            // 44324 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44326 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44328 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4432a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4432c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4432d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4432e mov r14,rcx
            r14 = rcx;
            // 44331 mov r12,rdx
            r12 = rdx;
            // 44334 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 44337 mov rbx,rdi
            rbx = rdi;
            // 4433a mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 4433d call 0xcf5fd
            { m.outregs(0x44342, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcf5fd); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44342 mov DWORD PTR [r12],0x8
            m.w32(r12, (0x8u64) as u32);
            // 4434a mov DWORD PTR [r14],0x20
            m.w32(r14, (0x20u64) as u32);
            // 44351 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 44354 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 44357 jb 0x44368
            if crate::cov::br(fl.b()) { pc = 0x44368; continue; }
            pc = 0x44359;
        }
        0x44359 => {
            // block 44359
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 44359 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44359 lea ecx,[r15*4+0x4]
            rcx = (r15.wrapping_mul(4).wrapping_add(0x4)) as u32 as u64;
            // 44361 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // 44364 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 44366 jbe 0x4437d
            if crate::cov::br(fl.be()) { pc = 0x4437d; continue; }
            pc = 0x44368;
        }
        0x44368 => {
            // block 44368
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 44368 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44368 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x4436b;
        }
        0x4436b => {
            // block 4436b
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 4436b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4436b mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 4436e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 44372 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44373 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 44375 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 44377 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44379 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4437b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4437c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4437d => {
            // block 4437d
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 4437d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4437d mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 44384 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 44387 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 4438b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 4438e mov esi,0xbe
            rsi = (0xbeu64) as u32 as u64;
            // 44393 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44398 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4439b mov rdi,rbx
            rdi = rbx;
            // 4439e call QWORD PTR [rax+0x38]
            { m.outregs(0x443a1, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 443a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 443a3 je 0x4436b
            if crate::cov::br(fl.e()) { pc = 0x4436b; continue; }
            pc = 0x443a5;
        }
        0x443a5 => {
            // block 443a5
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 443a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 443a5 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 443a8 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 443ab lea rdx,[rip+0xbc2de] # 0x100690
            rdx = m.env.data_va(85597);
            // 443b2 mov cl,BYTE PTR [rdx+rcx*4]
            rcx = (rcx & !0xff) | ((m.r8(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u8 as u64);
            // 443b5 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 443b7 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 443b9 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 443bc mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // 443bf shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 443c2 mov DWORD PTR [r12],ecx
            m.w32(r12, ((rcx as u32 as u64)) as u32);
            // 443c6 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 443c9 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 443cc mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 443cf jmp 0x4436b
            pc = 0x4436b; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11e28 size=180
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011e28<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x11e28;
    loop {
        match pc {
        0x11e28 => {
            // block 11e28
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e28 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11e29 mov rbp,rsp
            rbp = rsp;
            // 11e2c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11e2e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11e30 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 11e32 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 11e34 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11e35 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 11e39 mov QWORD PTR [rbp-0x68],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rsi) as u64);
            // 11e3d mov r13,rdi
            r13 = rdi;
            // 11e40 cmp r13,rsi
            fl = sub::<64>(r13, rsi).1;
            // 11e43 je 0x11eb7
            if crate::cov::br(fl.e()) { pc = 0x11eb7; continue; }
            pc = 0x11e45;
        }
        0x11e45 => {
            // block 11e45
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e45 lea rax,[r13+0x38]
            rax = r13.wrapping_add(0x38);
            // 11e49 cmp rax,QWORD PTR [rbp-0x68]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 11e4d je 0x11eb7
            if crate::cov::br(fl.e()) { pc = 0x11eb7; continue; }
            pc = 0x11e4f;
        }
        0x11e4f => {
            // block 11e4f
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e4f lea rbx,[rbp-0x60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11e53 mov r12,r13
            r12 = r13;
            pc = 0x11e56;
        }
        0x11e56 => {
            // block 11e56
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e56 mov r15,rax
            r15 = rax;
            // 11e59 mov rdi,rbx
            rdi = rbx;
            // 11e5c mov rsi,r15
            rsi = r15;
            // 11e5f call 0xe06fe
            { m.outregs(0x11e64, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11e64 cmp r15,r13
            fl = sub::<64>(r15, r13).1;
            // 11e67 mov rbx,r15
            rbx = r15;
            // 11e6a mov r14,r15
            r14 = r15;
            // 11e6d je 0x11e90
            if crate::cov::br(fl.e()) { pc = 0x11e90; continue; }
            pc = 0x11e6f;
        }
        0x11e6f => {
            // block 11e6f
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e6f mov eax,DWORD PTR [rbp-0x48]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 11e72 cmp eax,DWORD PTR [r14-0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // 11e76 jge 0x11e90
            if crate::cov::br(fl.ge()) { pc = 0x11e90; continue; }
            pc = 0x11e78;
        }
        0x11e78 => {
            // block 11e78
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e78 add r14,0xffffffffffffffc8
            { let (r, f) = add::<64>(r14, 0xffff_ffff_ffff_ffc8u64);
              r14 = r;
              fl = f; }
            // 11e7c mov rdi,rbx
            rdi = rbx;
            // 11e7f mov rsi,r14
            rsi = r14;
            // 11e82 call 0xe064a
            { m.outregs(0x11e87, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11e87 add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            // 11e8b cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 11e8e jne 0x11e6f
            if crate::cov::br(fl.ne()) { pc = 0x11e6f; continue; }
            pc = 0x11e90;
        }
        0x11e90 => {
            // block 11e90
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e90 mov rdi,rbx
            rdi = rbx;
            // 11e93 lea rbx,[rbp-0x60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11e97 mov rsi,rbx
            rsi = rbx;
            // 11e9a call 0xe064a
            { m.outregs(0x11e9f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11e9f mov rdi,rbx
            rdi = rbx;
            // 11ea2 call 0xe05fa
            { m.outregs(0x11ea7, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11ea7 add r12,0x70
            { let (r, f) = add::<64>(r12, 0x70u64);
              r12 = r;
              fl = f; }
            // 11eab cmp r12,QWORD PTR [rbp-0x68]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 11eaf mov rax,r12
            rax = r12;
            // 11eb2 mov r12,r15
            r12 = r15;
            // 11eb5 jne 0x11e56
            if crate::cov::br(fl.ne()) { pc = 0x11e56; continue; }
            pc = 0x11eb7;
        }
        0x11eb7 => {
            // block 11eb7
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11eb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11eb7 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 11ebb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11ebc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11ebe pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 11ec0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11ec2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11ec4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11ec5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x11ec6 => {
            // block 11ec6
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11ec6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11ec6 mov rbx,rax
            rbx = rax;
            // 11ec9 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11ecd call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11ed2 mov rdi,rbx
            rdi = rbx;
            // 11ed5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11eda jmp 0x11ec6
            pc = 0x11ec6; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
