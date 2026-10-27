//! Mechanically lifted functions (scripts/lift_run.py), module w002. Each one is checked against the original
//! in the world harness (tests/world_w002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x2de78 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002de78<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2de78;
    loop {
        match pc {
        0x2de78 => {
            // block 2de78
            #[cfg(feature = "trace")] eprintln!("f_0002de78 pc 2de78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2de78 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2de79 mov rbp,rsp
            rbp = rsp;
            // 2de7c lea rdi,[rip+0x119265] # 0x1470e8
            rdi = m.env.data_va(374965);
            // 2de83 lea rsi,[rip+0xfffffffffffffebc] # 0x2dd46
            rsi = m.env.code_va(0x2dd46);
            // 2de8a call 0xec266
            { m.outregs(0x2de8f, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa33); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2de8f mov rax,QWORD PTR [rip+0x193b9a] # 0x1c1a30
            rax = m.r64(m.env.data_va(877053)) as u64;
            // 2de96 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2de99 je 0x2de9e
            if crate::cov::br(fl.e()) { pc = 0x2de9e; continue; }
            pc = 0x2de9b;
        }
        0x2de9b => {
            // block 2de9b
            #[cfg(feature = "trace")] eprintln!("f_0002de78 pc 2de9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2de9b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2de9c jmp rax
            { let r = { let t = rax; m.call(t, &[], &[]) }; return Ret::from_call(r); }
        }
        0x2de9e => {
            // block 2de9e
            #[cfg(feature = "trace")] eprintln!("f_0002de78 pc 2de9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2de9e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2dea0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dea1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2df08 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002df08<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2df08;
    loop {
        match pc {
        0x2df08 => {
            // block 2df08
            #[cfg(feature = "trace")] eprintln!("f_0002df08 pc 2df08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2df08 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2df09 mov rbp,rsp
            rbp = rsp;
            // 2df0c lea rdi,[rip+0x1191d5] # 0x1470e8
            rdi = m.env.data_va(374965);
            // 2df13 lea rsi,[rip+0xfffffffffffffe2c] # 0x2dd46
            rsi = m.env.code_va(0x2dd46);
            // 2df1a call 0xec266
            { m.outregs(0x2df1f, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa33); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2df1f mov rax,QWORD PTR [rip+0x193b22] # 0x1c1a48
            rax = m.r64(m.env.data_va(877077)) as u64;
            // 2df26 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2df29 je 0x2df2e
            if crate::cov::br(fl.e()) { pc = 0x2df2e; continue; }
            pc = 0x2df2b;
        }
        0x2df2b => {
            // block 2df2b
            #[cfg(feature = "trace")] eprintln!("f_0002df08 pc 2df2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2df2b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2df2c jmp rax
            { let r = { let t = rax; m.call(t, &[], &[]) }; return Ret::from_call(r); }
        }
        0x2df2e => {
            // block 2df2e
            #[cfg(feature = "trace")] eprintln!("f_0002df08 pc 2df2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2df2e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2df30 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2df31 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dfe3 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dfe3<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2dfe3;
    loop {
        match pc {
        0x2dfe3 => {
            // block 2dfe3
            #[cfg(feature = "trace")] eprintln!("f_0002dfe3 pc 2dfe3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dfe3 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dfe4 mov rbp,rsp
            rbp = rsp;
            // 2dfe7 lea rdi,[rip+0x11910a] # 0x1470f8
            rdi = m.env.data_va(374981);
            // 2dfee lea rsi,[rip+0xffffffffffffff67] # 0x2df5c
            rsi = m.env.code_va(0x2df5c);
            // 2dff5 call 0xec266
            { m.outregs(0x2dffa, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa33); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dffa mov rax,QWORD PTR [rip+0x193a5f] # 0x1c1a60
            rax = m.r64(m.env.data_va(877101)) as u64;
            // 2e001 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2e004 je 0x2e009
            if crate::cov::br(fl.e()) { pc = 0x2e009; continue; }
            pc = 0x2e006;
        }
        0x2e006 => {
            // block 2e006
            #[cfg(feature = "trace")] eprintln!("f_0002dfe3 pc 2e006 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e006 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e007 jmp rax
            { let r = { let t = rax; m.call(t, &[], &[]) }; return Ret::from_call(r); }
        }
        0x2e009 => {
            // block 2e009
            #[cfg(feature = "trace")] eprintln!("f_0002dfe3 pc 2e009 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e009 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e00b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e00c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba962 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba962<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba962;
    loop {
        match pc {
        0xba962 => {
            // block ba962
            #[cfg(feature = "trace")] eprintln!("f_000ba962 pc ba962 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba962 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba963 mov rbp,rsp
            rbp = rsp;
            // ba966 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba967 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ba968 mov rbx,rdi
            rbx = rdi;
            // ba96b mov DWORD PTR [rbp-0xc],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rsi as u32 as u64)) as u32);
            // ba96e lea rdi,[rbx+0x8]
            rdi = rbx.wrapping_add(0x8);
            // ba972 lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // ba976 call 0xebcfc
            { m.outregs(0xba97b, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x4c9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba97b add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // ba97f cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // ba982 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // ba985 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ba989 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba98a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba98b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbba87 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bba87<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbba87;
    loop {
        match pc {
        0xbba87 => {
            // block bba87
            #[cfg(feature = "trace")] eprintln!("f_000bba87 pc bba87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bba87 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bba88 mov rbp,rsp
            rbp = rsp;
            // bba8b mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bba90 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bba93 ja 0xbbaaf
            if crate::cov::br(fl.a()) { pc = 0xbbaaf; continue; }
            pc = 0xbba95;
        }
        0xbba95 => {
            // block bba95
            #[cfg(feature = "trace")] eprintln!("f_000bba87 pc bba95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bba95 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bba97 lea rcx,[rip+0x47022] # 0x102ac0
            rcx = m.env.data_va(94861);
            // bba9e lea rdx,[rip+0x46ffb] # 0x102aa0
            rdx = m.env.data_va(94829);
            // bbaa5 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbaa8 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbaac mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbaaf;
        }
        0xbbaaf => {
            // block bbaaf
            #[cfg(feature = "trace")] eprintln!("f_000bba87 pc bbaaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbaaf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbab0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbbe6 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbbe6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbbe6;
    loop {
        match pc {
        0xbbbe6 => {
            // block bbbe6
            #[cfg(feature = "trace")] eprintln!("f_000bbbe6 pc bbbe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbbe6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbbe7 mov rbp,rsp
            rbp = rsp;
            // bbbea mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // bbbef cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbbf2 ja 0xbbc0e
            if crate::cov::br(fl.a()) { pc = 0xbbc0e; continue; }
            pc = 0xbbbf4;
        }
        0xbbbf4 => {
            // block bbbf4
            #[cfg(feature = "trace")] eprintln!("f_000bbbe6 pc bbbf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbbf4 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbbf6 lea rcx,[rip+0x47063] # 0x102c60
            rcx = m.env.data_va(95277);
            // bbbfd lea rdx,[rip+0x4703c] # 0x102c40
            rdx = m.env.data_va(95245);
            // bbc04 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbc07 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbc0b mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbc0e;
        }
        0xbbc0e => {
            // block bbc0e
            #[cfg(feature = "trace")] eprintln!("f_000bbbe6 pc bbc0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc0e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbc0f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbcef size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbcef<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbcef;
    loop {
        match pc {
        0xbbcef => {
            // block bbcef
            #[cfg(feature = "trace")] eprintln!("f_000bbcef pc bbcef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbcef push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbcf0 mov rbp,rsp
            rbp = rsp;
            // bbcf3 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bbcf8 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbcfb ja 0xbbd17
            if crate::cov::br(fl.a()) { pc = 0xbbd17; continue; }
            pc = 0xbbcfd;
        }
        0xbbcfd => {
            // block bbcfd
            #[cfg(feature = "trace")] eprintln!("f_000bbcef pc bbcfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbcfd mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbcff lea rcx,[rip+0x4704a] # 0x102d50
            rcx = m.env.data_va(95517);
            // bbd06 lea rdx,[rip+0x47023] # 0x102d30
            rdx = m.env.data_va(95485);
            // bbd0d test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbd10 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbd14 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbd17;
        }
        0xbbd17 => {
            // block bbd17
            #[cfg(feature = "trace")] eprintln!("f_000bbcef pc bbd17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbd17 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbd18 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e00d size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e00d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e00d;
    loop {
        match pc {
        0x2e00d => {
            // block 2e00d
            #[cfg(feature = "trace")] eprintln!("f_0002e00d pc 2e00d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e00d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e00e mov rbp,rsp
            rbp = rsp;
            // 2e011 lea rdi,[rip+0x1190e0] # 0x1470f8
            rdi = m.env.data_va(374981);
            // 2e018 lea rsi,[rip+0xffffffffffffff3d] # 0x2df5c
            rsi = m.env.code_va(0x2df5c);
            // 2e01f call 0xec266
            { m.outregs(0x2e024, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa33); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e024 mov rax,QWORD PTR [rip+0x193a3d] # 0x1c1a68
            rax = m.r64(m.env.data_va(877109)) as u64;
            // 2e02b test rax,rax
            fl = logic::<64>(rax & rax);
            // 2e02e je 0x2e033
            if crate::cov::br(fl.e()) { pc = 0x2e033; continue; }
            pc = 0x2e030;
        }
        0x2e030 => {
            // block 2e030
            #[cfg(feature = "trace")] eprintln!("f_0002e00d pc 2e030 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e030 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e031 jmp rax
            { let r = { let t = rax; m.call(t, &[], &[]) }; return Ret::from_call(r); }
        }
        0x2e033 => {
            // block 2e033
            #[cfg(feature = "trace")] eprintln!("f_0002e00d pc 2e033 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e033 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e035 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e036 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x33e42 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00033e42<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x33e42;
    loop {
        match pc {
        0x33e42 => {
            // block 33e42
            #[cfg(feature = "trace")] eprintln!("f_00033e42 pc 33e42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33e42 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 33e43 mov rbp,rsp
            rbp = rsp;
            // 33e46 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 33e48 lea rcx,[rip+0xcc081] # 0xffed0
            rcx = m.env.data_va(83613);
            // 33e4f mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 33e52 lea rdx,[rip+0xcc037] # 0xffe90
            rdx = m.env.data_va(83549);
            // 33e59 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 33e5c cmp esi,0x2
            fl = sub::<32>((rsi as u32 as u64), 0x2u64).1;
            // 33e5f mov DWORD PTR [rdi],ecx
            m.w32(rdi, ((rcx as u32 as u64)) as u32);
            // 33e61 mov DWORD PTR [rdi+0x4],eax
            m.w32(rdi.wrapping_add(0x4), ((rax as u32 as u64)) as u32);
            // 33e64 setne BYTE PTR [rdi+0x8]
            m.w8(rdi.wrapping_add(0x8), ((fl.ne() as u64)) as u8);
            // 33e68 mov DWORD PTR [rdi+0xc],esi
            m.w32(rdi.wrapping_add(0xc), ((rsi as u32 as u64)) as u32);
            // 33e6b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 33e6c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x685e3 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000685e3<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x685e3;
    loop {
        match pc {
        0x685e3 => {
            // block 685e3
            #[cfg(feature = "trace")] eprintln!("f_000685e3 pc 685e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 685e3 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 685e4 mov rbp,rsp
            rbp = rsp;
            // 685e7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 685e9 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 685ea mov ebx,0xa8
            rbx = (0xa8u64) as u32 as u64;
            // 685ef lea r14,[rip+0x15fcca] # 0x1c82c0
            r14 = m.env.data_va(903821);
            pc = 0x685f6;
        }
        0x685f6 => {
            // block 685f6
            #[cfg(feature = "trace")] eprintln!("f_000685e3 pc 685f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 685f6 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 685fa call 0xec032
            { m.outregs(0x685ff, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 685ff add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 68603 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 68607 jne 0x685f6
            if crate::cov::br(fl.ne()) { pc = 0x685f6; continue; }
            pc = 0x68609;
        }
        0x68609 => {
            // block 68609
            #[cfg(feature = "trace")] eprintln!("f_000685e3 pc 68609 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68609 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6860a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6860c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6860d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68639 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068639<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68639;
    loop {
        match pc {
        0x68639 => {
            // block 68639
            #[cfg(feature = "trace")] eprintln!("f_00068639 pc 68639 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68639 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6863a mov rbp,rsp
            rbp = rsp;
            // 6863d push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6863f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68640 mov ebx,0x30
            rbx = (0x30u64) as u32 as u64;
            // 68645 lea r14,[rip+0xdf1a4] # 0x1477f0
            r14 = m.env.data_va(376765);
            pc = 0x6864c;
        }
        0x6864c => {
            // block 6864c
            #[cfg(feature = "trace")] eprintln!("f_00068639 pc 6864c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6864c lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 68650 call 0xec032
            { m.outregs(0x68655, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68655 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 68659 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 6865d jne 0x6864c
            if crate::cov::br(fl.ne()) { pc = 0x6864c; continue; }
            pc = 0x6865f;
        }
        0x6865f => {
            // block 6865f
            #[cfg(feature = "trace")] eprintln!("f_00068639 pc 6865f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6865f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68660 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 68662 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68663 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6868f size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006868f<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6868f;
    loop {
        match pc {
        0x6868f => {
            // block 6868f
            #[cfg(feature = "trace")] eprintln!("f_0006868f pc 6868f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6868f push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68690 mov rbp,rsp
            rbp = rsp;
            // 68693 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68695 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68696 mov ebx,0x180
            rbx = (0x180u64) as u32 as u64;
            // 6869b lea r14,[rip+0xdf26e] # 0x147910
            r14 = m.env.data_va(377053);
            pc = 0x686a2;
        }
        0x686a2 => {
            // block 686a2
            #[cfg(feature = "trace")] eprintln!("f_0006868f pc 686a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 686a2 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 686a6 call 0xec032
            { m.outregs(0x686ab, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 686ab add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 686af cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 686b3 jne 0x686a2
            if crate::cov::br(fl.ne()) { pc = 0x686a2; continue; }
            pc = 0x686b5;
        }
        0x686b5 => {
            // block 686b5
            #[cfg(feature = "trace")] eprintln!("f_0006868f pc 686b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 686b5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 686b6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 686b8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 686b9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x686e5 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000686e5<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x686e5;
    loop {
        match pc {
        0x686e5 => {
            // block 686e5
            #[cfg(feature = "trace")] eprintln!("f_000686e5 pc 686e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 686e5 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 686e6 mov rbp,rsp
            rbp = rsp;
            // 686e9 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 686eb push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 686ec mov ebx,0x90
            rbx = (0x90u64) as u32 as u64;
            // 686f1 lea r14,[rip+0xdf858] # 0x147f50
            r14 = m.env.data_va(378653);
            pc = 0x686f8;
        }
        0x686f8 => {
            // block 686f8
            #[cfg(feature = "trace")] eprintln!("f_000686e5 pc 686f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 686f8 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 686fc call 0xec032
            { m.outregs(0x68701, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68701 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 68705 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 68709 jne 0x686f8
            if crate::cov::br(fl.ne()) { pc = 0x686f8; continue; }
            pc = 0x6870b;
        }
        0x6870b => {
            // block 6870b
            #[cfg(feature = "trace")] eprintln!("f_000686e5 pc 6870b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6870b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6870c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6870e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6870f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6873b size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006873b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6873b;
    loop {
        match pc {
        0x6873b => {
            // block 6873b
            #[cfg(feature = "trace")] eprintln!("f_0006873b pc 6873b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6873b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6873c mov rbp,rsp
            rbp = rsp;
            // 6873f push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68741 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68742 mov ebx,0xf0
            rbx = (0xf0u64) as u32 as u64;
            // 68747 lea r14,[rip+0xdf9c2] # 0x148110
            r14 = m.env.data_va(379101);
            pc = 0x6874e;
        }
        0x6874e => {
            // block 6874e
            #[cfg(feature = "trace")] eprintln!("f_0006873b pc 6874e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6874e lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 68752 call 0xec032
            { m.outregs(0x68757, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68757 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 6875b cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 6875f jne 0x6874e
            if crate::cov::br(fl.ne()) { pc = 0x6874e; continue; }
            pc = 0x68761;
        }
        0x68761 => {
            // block 68761
            #[cfg(feature = "trace")] eprintln!("f_0006873b pc 68761 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68761 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68762 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 68764 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68765 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68791 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068791<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68791;
    loop {
        match pc {
        0x68791 => {
            // block 68791
            #[cfg(feature = "trace")] eprintln!("f_00068791 pc 68791 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68791 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68792 mov rbp,rsp
            rbp = rsp;
            // 68795 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68797 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68798 mov ebx,0x2e8
            rbx = (0x2e8u64) as u32 as u64;
            // 6879d lea r14,[rip+0xdfddc] # 0x148580
            r14 = m.env.data_va(380237);
            pc = 0x687a4;
        }
        0x687a4 => {
            // block 687a4
            #[cfg(feature = "trace")] eprintln!("f_00068791 pc 687a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 687a4 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 687a8 call 0xec032
            { m.outregs(0x687ad, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 687ad add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 687b1 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 687b5 jne 0x687a4
            if crate::cov::br(fl.ne()) { pc = 0x687a4; continue; }
            pc = 0x687b7;
        }
        0x687b7 => {
            // block 687b7
            #[cfg(feature = "trace")] eprintln!("f_00068791 pc 687b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 687b7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 687b8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 687ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 687bb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x687e7 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000687e7<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x687e7;
    loop {
        match pc {
        0x687e7 => {
            // block 687e7
            #[cfg(feature = "trace")] eprintln!("f_000687e7 pc 687e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 687e7 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 687e8 mov rbp,rsp
            rbp = rsp;
            // 687eb push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 687ed push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 687ee mov ebx,0x2e8
            rbx = (0x2e8u64) as u32 as u64;
            // 687f3 lea r14,[rip+0xe01c6] # 0x1489c0
            r14 = m.env.data_va(381325);
            pc = 0x687fa;
        }
        0x687fa => {
            // block 687fa
            #[cfg(feature = "trace")] eprintln!("f_000687e7 pc 687fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 687fa lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 687fe call 0xec032
            { m.outregs(0x68803, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68803 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 68807 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 6880b jne 0x687fa
            if crate::cov::br(fl.ne()) { pc = 0x687fa; continue; }
            pc = 0x6880d;
        }
        0x6880d => {
            // block 6880d
            #[cfg(feature = "trace")] eprintln!("f_000687e7 pc 6880d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6880d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6880e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 68810 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68811 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6883d size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006883d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6883d;
    loop {
        match pc {
        0x6883d => {
            // block 6883d
            #[cfg(feature = "trace")] eprintln!("f_0006883d pc 6883d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6883d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6883e mov rbp,rsp
            rbp = rsp;
            // 68841 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68843 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68844 mov ebx,0x1f8
            rbx = (0x1f8u64) as u32 as u64;
            // 68849 lea r14,[rip+0xe0750] # 0x148fa0
            r14 = m.env.data_va(382829);
            pc = 0x68850;
        }
        0x68850 => {
            // block 68850
            #[cfg(feature = "trace")] eprintln!("f_0006883d pc 68850 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68850 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 68854 call 0xec032
            { m.outregs(0x68859, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68859 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 6885d cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 68861 jne 0x68850
            if crate::cov::br(fl.ne()) { pc = 0x68850; continue; }
            pc = 0x68863;
        }
        0x68863 => {
            // block 68863
            #[cfg(feature = "trace")] eprintln!("f_0006883d pc 68863 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68863 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68864 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 68866 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68867 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68893 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068893<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68893;
    loop {
        match pc {
        0x68893 => {
            // block 68893
            #[cfg(feature = "trace")] eprintln!("f_00068893 pc 68893 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68893 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68894 mov rbp,rsp
            rbp = rsp;
            // 68897 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68899 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6889a mov ebx,0x78
            rbx = (0x78u64) as u32 as u64;
            // 6889f lea r14,[rip+0xe0a7a] # 0x149320
            r14 = m.env.data_va(383725);
            pc = 0x688a6;
        }
        0x688a6 => {
            // block 688a6
            #[cfg(feature = "trace")] eprintln!("f_00068893 pc 688a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 688a6 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 688aa call 0xec032
            { m.outregs(0x688af, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 688af add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 688b3 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 688b7 jne 0x688a6
            if crate::cov::br(fl.ne()) { pc = 0x688a6; continue; }
            pc = 0x688b9;
        }
        0x688b9 => {
            // block 688b9
            #[cfg(feature = "trace")] eprintln!("f_00068893 pc 688b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 688b9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 688ba pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 688bc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 688bd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x33e16 size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00033e16<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x33e16;
    loop {
        match pc {
        0x33e16 => {
            // block 33e16
            #[cfg(feature = "trace")] eprintln!("f_00033e16 pc 33e16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33e16 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 33e17 mov rbp,rsp
            rbp = rsp;
            // 33e1a mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 33e1c lea rcx,[rip+0xcc0ad] # 0xffed0
            rcx = m.env.data_va(83613);
            // 33e23 mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 33e26 lea rdx,[rip+0xcc063] # 0xffe90
            rdx = m.env.data_va(83549);
            // 33e2d mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 33e30 cmp esi,0x2
            fl = sub::<32>((rsi as u32 as u64), 0x2u64).1;
            // 33e33 mov DWORD PTR [rdi],ecx
            m.w32(rdi, ((rcx as u32 as u64)) as u32);
            // 33e35 mov DWORD PTR [rdi+0x4],eax
            m.w32(rdi.wrapping_add(0x4), ((rax as u32 as u64)) as u32);
            // 33e38 setne BYTE PTR [rdi+0x8]
            m.w8(rdi.wrapping_add(0x8), ((fl.ne() as u64)) as u8);
            // 33e3c mov DWORD PTR [rdi+0xc],esi
            m.w32(rdi.wrapping_add(0xc), ((rsi as u32 as u64)) as u32);
            // 33e3f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 33e40 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41fae size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041fae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x41fae;
    loop {
        match pc {
        0x41fae => {
            // block 41fae
            #[cfg(feature = "trace")] eprintln!("f_00041fae pc 41fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41fae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41faf mov rbp,rsp
            rbp = rsp;
            // 41fb2 mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 41fb5 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 41fb8 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 41fbc mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 41fbe lea rdx,[rip+0xbe3fb] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 41fc5 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 41fc8 mov ecx,0x60000000
            rcx = (0x60000000u64) as u32 as u64;
            // 41fcd mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 41fd3 mov edx,r9d
            rdx = ((r9 as u32 as u64)) as u32 as u64;
            // 41fd6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41fd7 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42334 size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042334<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x42334;
    loop {
        match pc {
        0x42334 => {
            // block 42334
            #[cfg(feature = "trace")] eprintln!("f_00042334 pc 42334 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42334 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42335 mov rbp,rsp
            rbp = rsp;
            // 42338 mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 4233b mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 4233e mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 42342 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 42344 lea rdx,[rip+0xbe125] # 0x100470
            rdx = m.env.data_va(85053);
            // 4234b mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4234e mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 42353 mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 42359 mov edx,r9d
            rdx = ((r9 as u32 as u64)) as u32 as u64;
            // 4235c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4235d jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3e4e size=46
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3e4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3e4e;
    loop {
        match pc {
        0xc3e4e => {
            // block c3e4e
            #[cfg(feature = "trace")] eprintln!("f_000c3e4e pc c3e4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3e4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3e4f mov rbp,rsp
            rbp = rsp;
            // c3e52 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3e53 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c3e54 mov rbx,rdi
            rbx = rdi;
            // c3e57 lea rax,[rip+0x8037a] # 0x1441d8
            rax = m.env.data_va(362917);
            // c3e5e mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c3e61 lea rdi,[rbx+0x8]
            rdi = rbx.wrapping_add(0x8);
            // c3e65 mov rsi,QWORD PTR [rbx+0x10]
            rsi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c3e69 call 0xebe22
            { m.outregs(0xc3e6e, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3e6e mov rdi,rbx
            rdi = rbx;
            // c3e71 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c3e75 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3e76 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3e77 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44d00 size=50
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044d00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44d00;
    loop {
        match pc {
        0x44d00 => {
            // block 44d00
            #[cfg(feature = "trace")] eprintln!("f_00044d00 pc 44d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44d00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44d01 mov rbp,rsp
            rbp = rsp;
            // 44d04 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44d08 mov r9d,ecx
            r9 = ((rcx as u32 as u64)) as u32 as u64;
            // 44d0b mov r10,rdx
            r10 = rdx;
            // 44d0e mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 44d10 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 44d13 mov DWORD PTR [rsp],r8d
            m.w32(rsp, ((r8 as u32 as u64)) as u32);
            // 44d17 mov DWORD PTR [rsp+0x8],0x1
            m.w32(rsp.wrapping_add(0x8), (0x1u64) as u32);
            // 44d1f mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 44d24 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 44d26 mov r8,r10
            r8 = r10;
            // 44d29 call QWORD PTR [rax+0x58]
            { m.outregs(0x44d2c, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44d2c add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44d30 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44d31 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd2fe1 size=50
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d2fe1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd2fe1;
    loop {
        match pc {
        0xd2fe1 => {
            // block d2fe1
            #[cfg(feature = "trace")] eprintln!("f_000d2fe1 pc d2fe1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d2fe1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d2fe2 mov rbp,rsp
            rbp = rsp;
            // d2fe5 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d2fe7 cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // d2fea jne 0xd2ff5
            if crate::cov::br(fl.ne()) { pc = 0xd2ff5; continue; }
            pc = 0xd2fec;
        }
        0xd2fec => {
            // block d2fec
            #[cfg(feature = "trace")] eprintln!("f_000d2fe1 pc d2fec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d2fec lea rcx,[rip+0xea59d] # 0x1bd590
            rcx = m.env.data_va(859485);
            // d2ff3 jmp 0xd300a
            pc = 0xd300a; continue;
        }
        0xd2ff5 => {
            // block d2ff5
            #[cfg(feature = "trace")] eprintln!("f_000d2fe1 pc d2ff5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d2ff5 cmp edi,0x1
            fl = sub::<32>((rdi as u32 as u64), 0x1u64).1;
            // d2ff8 jne 0xd3003
            if crate::cov::br(fl.ne()) { pc = 0xd3003; continue; }
            pc = 0xd2ffa;
        }
        0xd2ffa => {
            // block d2ffa
            #[cfg(feature = "trace")] eprintln!("f_000d2fe1 pc d2ffa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d2ffa lea rcx,[rip+0xea4cf] # 0x1bd4d0
            rcx = m.env.data_va(859293);
            // d3001 jmp 0xd300a
            pc = 0xd300a; continue;
        }
        0xd3003 => {
            // block d3003
            #[cfg(feature = "trace")] eprintln!("f_000d2fe1 pc d3003 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3003 lea rcx,[rip+0xea646] # 0x1bd650
            rcx = m.env.data_va(859677);
            pc = 0xd300a;
        }
        0xd300a => {
            // block d300a
            #[cfg(feature = "trace")] eprintln!("f_000d2fe1 pc d300a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d300a lea rax,[rcx+rax*8]
            rax = rcx.wrapping_add(rax.wrapping_mul(8));
            // d300e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d3011 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3012 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19a40 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00019a40<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x19a40;
    loop {
        match pc {
        0x19a40 => {
            // block 19a40
            #[cfg(feature = "trace")] eprintln!("f_00019a40 pc 19a40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a40 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19a41 mov rbp,rsp
            rbp = rsp;
            // 19a44 mov rax,rdi
            rax = rdi;
            // 19a47 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // 19a4a test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 19a4d je 0x19a73
            if crate::cov::br(fl.e()) { pc = 0x19a73; continue; }
            pc = 0x19a4f;
        }
        0x19a4f => {
            // block 19a4f
            #[cfg(feature = "trace")] eprintln!("f_00019a40 pc 19a4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a4f mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 19a53 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // 19a56 je 0x19a6d
            if crate::cov::br(fl.e()) { pc = 0x19a6d; continue; }
            pc = 0x19a58;
        }
        0x19a58 => {
            // block 19a58
            #[cfg(feature = "trace")] eprintln!("f_00019a40 pc 19a58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a58 lea rdx,[rcx-0x4]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19a5c sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // 19a5f not rdx
            { let v = !rdx; rdx = v; }
            // 19a62 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 19a66 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 19a69 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            pc = 0x19a6d;
        }
        0x19a6d => {
            // block 19a6d
            #[cfg(feature = "trace")] eprintln!("f_00019a40 pc 19a6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a6d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19a6e jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x19a73 => {
            // block 19a73
            #[cfg(feature = "trace")] eprintln!("f_00019a40 pc 19a73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a73 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19a74 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25c5e size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_00025c5e_rec)
pub fn f_00025c5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_00025c5e_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_00025c5e_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0x25c5e;
    loop {
        match pc {
        0x25c5e => {
            // block 25c5e
            #[cfg(feature = "trace")] eprintln!("f_00025c5e pc 25c5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25c5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25c5f mov rbp,rsp
            rbp = rsp;
            // 25c62 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 25c64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 25c65 mov rbx,rsi
            rbx = rsi;
            // 25c68 mov r14,rdi
            r14 = rdi;
            // 25c6b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 25c6e je 0x25c93
            if crate::cov::br(fl.e()) { pc = 0x25c93; continue; }
            pc = 0x25c70;
        }
        0x25c70 => {
            // block 25c70
            #[cfg(feature = "trace")] eprintln!("f_00025c5e pc 25c70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25c70 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 25c73 mov rdi,r14
            rdi = r14;
            // 25c76 call 0x25c5e
            { let r = f_00025c5e(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25c7b mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 25c7f mov rdi,r14
            rdi = r14;
            // 25c82 call 0x25c5e
            { let r = f_00025c5e(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25c87 mov rdi,rbx
            rdi = rbx;
            // 25c8a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25c8b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25c8d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25c8e jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x25c93 => {
            // block 25c93
            #[cfg(feature = "trace")] eprintln!("f_00025c5e pc 25c93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25c93 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25c94 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25c96 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25c97 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44a3a size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_00044a3a_rec)
pub fn f_00044a3a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_00044a3a_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_00044a3a_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44a3a;
    loop {
        match pc {
        0x44a3a => {
            // block 44a3a
            #[cfg(feature = "trace")] eprintln!("f_00044a3a pc 44a3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44a3a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44a3b mov rbp,rsp
            rbp = rsp;
            // 44a3e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44a40 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44a41 mov rbx,rsi
            rbx = rsi;
            // 44a44 mov r14,rdi
            r14 = rdi;
            // 44a47 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 44a4a je 0x44a6f
            if crate::cov::br(fl.e()) { pc = 0x44a6f; continue; }
            pc = 0x44a4c;
        }
        0x44a4c => {
            // block 44a4c
            #[cfg(feature = "trace")] eprintln!("f_00044a3a pc 44a4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44a4c mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 44a4f mov rdi,r14
            rdi = r14;
            // 44a52 call 0x44a3a
            { let r = f_00044a3a(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44a57 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 44a5b mov rdi,r14
            rdi = r14;
            // 44a5e call 0x44a3a
            { let r = f_00044a3a(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44a63 mov rdi,rbx
            rdi = rbx;
            // 44a66 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44a67 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44a69 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44a6a jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x44a6f => {
            // block 44a6f
            #[cfg(feature = "trace")] eprintln!("f_00044a3a pc 44a6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44a6f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44a70 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44a72 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44a73 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb59e0 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000b59e0_rec)
pub fn f_000b59e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000b59e0_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000b59e0_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb59e0;
    loop {
        match pc {
        0xb59e0 => {
            // block b59e0
            #[cfg(feature = "trace")] eprintln!("f_000b59e0 pc b59e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b59e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b59e1 mov rbp,rsp
            rbp = rsp;
            // b59e4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b59e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b59e7 mov rbx,rsi
            rbx = rsi;
            // b59ea mov r14,rdi
            r14 = rdi;
            // b59ed test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b59f0 je 0xb5a15
            if crate::cov::br(fl.e()) { pc = 0xb5a15; continue; }
            pc = 0xb59f2;
        }
        0xb59f2 => {
            // block b59f2
            #[cfg(feature = "trace")] eprintln!("f_000b59e0 pc b59f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b59f2 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // b59f5 mov rdi,r14
            rdi = r14;
            // b59f8 call 0xb59e0
            { let r = f_000b59e0(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b59fd mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5a01 mov rdi,r14
            rdi = r14;
            // b5a04 call 0xb59e0
            { let r = f_000b59e0(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5a09 mov rdi,rbx
            rdi = rbx;
            // b5a0c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5a0d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5a0f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5a10 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xb5a15 => {
            // block b5a15
            #[cfg(feature = "trace")] eprintln!("f_000b59e0 pc b5a15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5a15 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5a16 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5a18 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5a19 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5adc size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000b5adc_rec)
pub fn f_000b5adc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000b5adc_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000b5adc_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5adc;
    loop {
        match pc {
        0xb5adc => {
            // block b5adc
            #[cfg(feature = "trace")] eprintln!("f_000b5adc pc b5adc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5adc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5add mov rbp,rsp
            rbp = rsp;
            // b5ae0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5ae2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5ae3 mov rbx,rsi
            rbx = rsi;
            // b5ae6 mov r14,rdi
            r14 = rdi;
            // b5ae9 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5aec je 0xb5b11
            if crate::cov::br(fl.e()) { pc = 0xb5b11; continue; }
            pc = 0xb5aee;
        }
        0xb5aee => {
            // block b5aee
            #[cfg(feature = "trace")] eprintln!("f_000b5adc pc b5aee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5aee mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // b5af1 mov rdi,r14
            rdi = r14;
            // b5af4 call 0xb5adc
            { let r = f_000b5adc(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5af9 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5afd mov rdi,r14
            rdi = r14;
            // b5b00 call 0xb5adc
            { let r = f_000b5adc(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5b05 mov rdi,rbx
            rdi = rbx;
            // b5b08 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5b09 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5b0b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5b0c jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xb5b11 => {
            // block b5b11
            #[cfg(feature = "trace")] eprintln!("f_000b5adc pc b5b11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5b11 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5b12 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5b14 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5b15 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3f52 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000c3f52_rec)
pub fn f_000c3f52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000c3f52_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000c3f52_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3f52;
    loop {
        match pc {
        0xc3f52 => {
            // block c3f52
            #[cfg(feature = "trace")] eprintln!("f_000c3f52 pc c3f52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3f52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3f53 mov rbp,rsp
            rbp = rsp;
            // c3f56 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3f58 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3f59 mov rbx,rsi
            rbx = rsi;
            // c3f5c mov r14,rdi
            r14 = rdi;
            // c3f5f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3f62 je 0xc3f87
            if crate::cov::br(fl.e()) { pc = 0xc3f87; continue; }
            pc = 0xc3f64;
        }
        0xc3f64 => {
            // block c3f64
            #[cfg(feature = "trace")] eprintln!("f_000c3f52 pc c3f64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3f64 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c3f67 mov rdi,r14
            rdi = r14;
            // c3f6a call 0xc3f52
            { let r = f_000c3f52(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3f6f mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c3f73 mov rdi,r14
            rdi = r14;
            // c3f76 call 0xc3f52
            { let r = f_000c3f52(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3f7b mov rdi,rbx
            rdi = rbx;
            // c3f7e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3f7f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3f81 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3f82 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc3f87 => {
            // block c3f87
            #[cfg(feature = "trace")] eprintln!("f_000c3f52 pc c3f87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3f87 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3f88 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3f8a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3f8b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc404e size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000c404e_rec)
pub fn f_000c404e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000c404e_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000c404e_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc404e;
    loop {
        match pc {
        0xc404e => {
            // block c404e
            #[cfg(feature = "trace")] eprintln!("f_000c404e pc c404e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c404e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c404f mov rbp,rsp
            rbp = rsp;
            // c4052 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4054 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4055 mov rbx,rsi
            rbx = rsi;
            // c4058 mov r14,rdi
            r14 = rdi;
            // c405b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c405e je 0xc4083
            if crate::cov::br(fl.e()) { pc = 0xc4083; continue; }
            pc = 0xc4060;
        }
        0xc4060 => {
            // block c4060
            #[cfg(feature = "trace")] eprintln!("f_000c404e pc c4060 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4060 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c4063 mov rdi,r14
            rdi = r14;
            // c4066 call 0xc404e
            { let r = f_000c404e(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c406b mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c406f mov rdi,r14
            rdi = r14;
            // c4072 call 0xc404e
            { let r = f_000c404e(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4077 mov rdi,rbx
            rdi = rbx;
            // c407a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c407b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c407d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c407e jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc4083 => {
            // block c4083
            #[cfg(feature = "trace")] eprintln!("f_000c404e pc c4083 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4083 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4084 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4086 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4087 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4906 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000d4906_rec)
pub fn f_000d4906<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000d4906_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000d4906_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4906;
    loop {
        match pc {
        0xd4906 => {
            // block d4906
            #[cfg(feature = "trace")] eprintln!("f_000d4906 pc d4906 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4906 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4907 mov rbp,rsp
            rbp = rsp;
            // d490a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d490c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d490d mov rbx,rsi
            rbx = rsi;
            // d4910 mov r14,rdi
            r14 = rdi;
            // d4913 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d4916 je 0xd493b
            if crate::cov::br(fl.e()) { pc = 0xd493b; continue; }
            pc = 0xd4918;
        }
        0xd4918 => {
            // block d4918
            #[cfg(feature = "trace")] eprintln!("f_000d4906 pc d4918 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4918 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // d491b mov rdi,r14
            rdi = r14;
            // d491e call 0xd4906
            { let r = f_000d4906(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4923 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // d4927 mov rdi,r14
            rdi = r14;
            // d492a call 0xd4906
            { let r = f_000d4906(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d492f mov rdi,rbx
            rdi = rbx;
            // d4932 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4933 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4935 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4936 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xd493b => {
            // block d493b
            #[cfg(feature = "trace")] eprintln!("f_000d4906 pc d493b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d493b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d493c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d493e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d493f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5dfc size=62
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005dfc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5dfc;
    loop {
        match pc {
        0x5dfc => {
            // block 5dfc
            #[cfg(feature = "trace")] eprintln!("f_00005dfc pc 5dfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dfc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5dfd mov rbp,rsp
            rbp = rsp;
            // 5e00 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5e02 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5e03 mov r14,rsi
            r14 = rsi;
            // 5e06 mov rbx,rdi
            rbx = rdi;
            // 5e09 mov rax,QWORD PTR [rbx+0x210]
            rax = m.r64(rbx.wrapping_add(0x210)) as u64;
            // 5e10 test rax,rax
            fl = logic::<64>(rax & rax);
            // 5e13 jne 0x5e2c
            if crate::cov::br(fl.ne()) { pc = 0x5e2c; continue; }
            pc = 0x5e15;
        }
        0x5e15 => {
            // block 5e15
            #[cfg(feature = "trace")] eprintln!("f_00005dfc pc 5e15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5e15 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5e18 mov rdi,rbx
            rdi = rbx;
            // 5e1b call QWORD PTR [rax+0x80]
            { m.outregs(0x5e21, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5e21 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5e23 je 0x5e33
            if crate::cov::br(fl.e()) { pc = 0x5e33; continue; }
            pc = 0x5e25;
        }
        0x5e25 => {
            // block 5e25
            #[cfg(feature = "trace")] eprintln!("f_00005dfc pc 5e25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5e25 mov rax,QWORD PTR [rbx+0x210]
            rax = m.r64(rbx.wrapping_add(0x210)) as u64;
            pc = 0x5e2c;
        }
        0x5e2c => {
            // block 5e2c
            #[cfg(feature = "trace")] eprintln!("f_00005dfc pc 5e2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5e2c mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 5e2f mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 5e31 jmp 0x5e35
            pc = 0x5e35; continue;
        }
        0x5e33 => {
            // block 5e33
            #[cfg(feature = "trace")] eprintln!("f_00005dfc pc 5e33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5e33 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5e35;
        }
        0x5e35 => {
            // block 5e35
            #[cfg(feature = "trace")] eprintln!("f_00005dfc pc 5e35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5e35 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5e36 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5e38 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5e39 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba98c size=62
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba98c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba98c;
    loop {
        match pc {
        0xba98c => {
            // block ba98c
            #[cfg(feature = "trace")] eprintln!("f_000ba98c pc ba98c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba98c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba98d mov rbp,rsp
            rbp = rsp;
            // ba990 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba992 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba993 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ba997 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // ba99a mov rbx,rdi
            rbx = rdi;
            // ba99d mov DWORD PTR [rbp-0x14],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rsi as u32 as u64)) as u32);
            // ba9a0 lea rdi,[rbx+0x8]
            rdi = rbx.wrapping_add(0x8);
            // ba9a4 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // ba9a8 call 0xebcfc
            { m.outregs(0xba9ad, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x4c9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba9ad add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // ba9b1 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // ba9b4 je 0xba9bf
            if crate::cov::br(fl.e()) { pc = 0xba9bf; continue; }
            pc = 0xba9b6;
        }
        0xba9b6 => {
            // block ba9b6
            #[cfg(feature = "trace")] eprintln!("f_000ba98c pc ba9b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba9b6 cmp DWORD PTR [rax+0x20],r14d
            fl = sub::<32>(m.r32(rax.wrapping_add(0x20)) as u64, (r14 as u32 as u64)).1;
            // ba9ba sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // ba9bd jmp 0xba9c1
            pc = 0xba9c1; continue;
        }
        0xba9bf => {
            // block ba9bf
            #[cfg(feature = "trace")] eprintln!("f_000ba98c pc ba9bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba9bf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xba9c1;
        }
        0xba9c1 => {
            // block ba9c1
            #[cfg(feature = "trace")] eprintln!("f_000ba98c pc ba9c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba9c1 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ba9c5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba9c6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba9c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba9c9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2c26 size=62
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2c26<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2c26;
    loop {
        match pc {
        0xe2c26 => {
            // block e2c26
            #[cfg(feature = "trace")] eprintln!("f_000e2c26 pc e2c26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c26 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2c27 mov rbp,rsp
            rbp = rsp;
            // e2c2a mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e2c2c mov esi,0x43864
            rsi = (0x43864u64) as u32 as u64;
            // e2c31 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // e2c34 ja 0xe2c4d
            if crate::cov::br(fl.a()) { pc = 0xe2c4d; continue; }
            pc = 0xe2c36;
        }
        0xe2c36 => {
            // block e2c36
            #[cfg(feature = "trace")] eprintln!("f_000e2c26 pc e2c36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c36 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e2c39 ja 0xe2c4d
            if crate::cov::br(fl.a()) { pc = 0xe2c4d; continue; }
            pc = 0xe2c3b;
        }
        0xe2c3b => {
            // block e2c3b
            #[cfg(feature = "trace")] eprintln!("f_000e2c26 pc e2c3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c3b cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // e2c3e ja 0xe2c4d
            if crate::cov::br(fl.a()) { pc = 0xe2c4d; continue; }
            pc = 0xe2c40;
        }
        0xe2c40 => {
            // block e2c40
            #[cfg(feature = "trace")] eprintln!("f_000e2c26 pc e2c40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c40 lea eax,[rdx+rax*4]
            rax = (rdx.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e2c43 lea eax,[rcx+rax*4]
            rax = (rcx.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e2c46 lea esi,[rax+rax*2+0x43864]
            rsi = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x43864)) as u32 as u64;
            pc = 0xe2c4d;
        }
        0xe2c4d => {
            // block e2c4d
            #[cfg(feature = "trace")] eprintln!("f_000e2c26 pc e2c4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c4d mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e2c50 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2c53 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e2c57 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e2c59 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2c5e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2c61 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2c62 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdeee size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000deee<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdeee;
    loop {
        match pc {
        0xdeee => {
            // block deee
            #[cfg(feature = "trace")] eprintln!("f_0000deee pc deee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // deee push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // deef mov rbp,rsp
            rbp = rsp;
            // def2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // def4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // def5 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // def8 mov rbx,rdi
            rbx = rdi;
            // defb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // defe call QWORD PTR [rax+0x1728]
            { m.outregs(0xdf04, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1728)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df04 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // df06 je 0xdf27
            if crate::cov::br(fl.e()) { pc = 0xdf27; continue; }
            pc = 0xdf08;
        }
        0xdf08 => {
            // block df08
            #[cfg(feature = "trace")] eprintln!("f_0000deee pc df08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df08 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // df0c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df0f mov esi,0x136
            rsi = (0x136u64) as u32 as u64;
            // df14 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // df19 mov r8d,0x5
            r8 = (0x5u64) as u32 as u64;
            // df1f mov rdi,rbx
            rdi = rbx;
            // df22 call QWORD PTR [rax+0x30]
            { m.outregs(0xdf25, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df25 jmp 0xdf29
            pc = 0xdf29; continue;
        }
        0xdf27 => {
            // block df27
            #[cfg(feature = "trace")] eprintln!("f_0000deee pc df27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df27 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xdf29;
        }
        0xdf29 => {
            // block df29
            #[cfg(feature = "trace")] eprintln!("f_0000deee pc df29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df29 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // df2a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // df2c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // df2d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x104ac size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000104ac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x104ac;
    loop {
        match pc {
        0x104ac => {
            // block 104ac
            #[cfg(feature = "trace")] eprintln!("f_000104ac pc 104ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 104ac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 104ad mov rbp,rsp
            rbp = rsp;
            // 104b0 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 104b4 mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 104b8 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 104bb je 0x104d2
            if crate::cov::br(fl.e()) { pc = 0x104d2; continue; }
            pc = 0x104bd;
        }
        0x104bd => {
            // block 104bd
            #[cfg(feature = "trace")] eprintln!("f_000104ac pc 104bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 104bd lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 104c1 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 104c4 not rdx
            { let v = !rdx; rdx = v; }
            // 104c7 and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 104cb add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 104ce mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0x104d2;
        }
        0x104d2 => {
            // block 104d2
            #[cfg(feature = "trace")] eprintln!("f_000104ac pc 104d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 104d2 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 104d5 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 104d8 je 0x104e0
            if crate::cov::br(fl.e()) { pc = 0x104e0; continue; }
            pc = 0x104da;
        }
        0x104da => {
            // block 104da
            #[cfg(feature = "trace")] eprintln!("f_000104ac pc 104da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 104da pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 104db jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x104e0 => {
            // block 104e0
            #[cfg(feature = "trace")] eprintln!("f_000104ac pc 104e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 104e0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 104e1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19bf2 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00019bf2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x19bf2;
    loop {
        match pc {
        0x19bf2 => {
            // block 19bf2
            #[cfg(feature = "trace")] eprintln!("f_00019bf2 pc 19bf2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19bf2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19bf3 mov rbp,rsp
            rbp = rsp;
            // 19bf6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19bf8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19bf9 mov r14,rsi
            r14 = rsi;
            // 19bfc mov rbx,rdi
            rbx = rdi;
            // 19bff mov rax,r14
            rax = r14;
            // 19c02 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19c06 je 0x19c10
            if crate::cov::br(fl.e()) { pc = 0x19c10; continue; }
            pc = 0x19c08;
        }
        0x19c08 => {
            // block 19c08
            #[cfg(feature = "trace")] eprintln!("f_00019bf2 pc 19c08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19c08 mov rdi,rbx
            rdi = rbx;
            // 19c0b call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x19c10 => {
            // block 19c10
            #[cfg(feature = "trace")] eprintln!("f_00019bf2 pc 19c10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19c10 lea rdi,[r14*4+0x0]
            rdi = r14.wrapping_mul(4);
            // 19c18 call 0xec146
            { m.outregs(0x19c1d, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19c1d mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 19c21 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 19c24 lea rax,[rax+r14*4]
            rax = rax.wrapping_add(r14.wrapping_mul(4));
            // 19c28 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 19c2c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19c2d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19c2f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19c30 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19f62 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00019f62<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x19f62;
    loop {
        match pc {
        0x19f62 => {
            // block 19f62
            #[cfg(feature = "trace")] eprintln!("f_00019f62 pc 19f62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f62 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19f63 mov rbp,rsp
            rbp = rsp;
            // 19f66 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19f68 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19f69 mov r14,rsi
            r14 = rsi;
            // 19f6c mov rbx,rdi
            rbx = rdi;
            // 19f6f mov rax,r14
            rax = r14;
            // 19f72 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19f76 je 0x19f80
            if crate::cov::br(fl.e()) { pc = 0x19f80; continue; }
            pc = 0x19f78;
        }
        0x19f78 => {
            // block 19f78
            #[cfg(feature = "trace")] eprintln!("f_00019f62 pc 19f78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f78 mov rdi,rbx
            rdi = rbx;
            // 19f7b call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x19f80 => {
            // block 19f80
            #[cfg(feature = "trace")] eprintln!("f_00019f62 pc 19f80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f80 lea rdi,[r14*4+0x0]
            rdi = r14.wrapping_mul(4);
            // 19f88 call 0xec146
            { m.outregs(0x19f8d, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19f8d mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 19f91 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 19f94 lea rax,[rax+r14*4]
            rax = rax.wrapping_add(r14.wrapping_mul(4));
            // 19f98 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 19f9c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19f9d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19f9f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19fa0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64ef8 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064ef8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x64ef8;
    loop {
        match pc {
        0x64ef8 => {
            // block 64ef8
            #[cfg(feature = "trace")] eprintln!("f_00064ef8 pc 64ef8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ef8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64ef9 mov rbp,rsp
            rbp = rsp;
            // 64efc mov rax,rdi
            rax = rdi;
            // 64eff mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // 64f02 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 64f05 je 0x64f2b
            if crate::cov::br(fl.e()) { pc = 0x64f2b; continue; }
            pc = 0x64f07;
        }
        0x64f07 => {
            // block 64f07
            #[cfg(feature = "trace")] eprintln!("f_00064ef8 pc 64f07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f07 mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 64f0b cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // 64f0e je 0x64f25
            if crate::cov::br(fl.e()) { pc = 0x64f25; continue; }
            pc = 0x64f10;
        }
        0x64f10 => {
            // block 64f10
            #[cfg(feature = "trace")] eprintln!("f_00064ef8 pc 64f10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f10 lea rdx,[rcx-0x10]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fff0);
            // 64f14 sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // 64f17 not rdx
            { let v = !rdx; rdx = v; }
            // 64f1a and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 64f1e add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 64f21 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            pc = 0x64f25;
        }
        0x64f25 => {
            // block 64f25
            #[cfg(feature = "trace")] eprintln!("f_00064ef8 pc 64f25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f25 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64f26 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x64f2b => {
            // block 64f2b
            #[cfg(feature = "trace")] eprintln!("f_00064ef8 pc 64f2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f2b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64f2c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8c20 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8c20<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc8c20;
    loop {
        match pc {
        0xc8c20 => {
            // block c8c20
            #[cfg(feature = "trace")] eprintln!("f_000c8c20 pc c8c20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8c20 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8c21 mov rbp,rsp
            rbp = rsp;
            // c8c24 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c8c25 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c8c26 lea rbx,[rip+0xff97b] # 0x1c85a8
            rbx = m.env.data_va(904565);
            // c8c2d lea rsi,[rip+0x6f24e] # 0x137e82
            rsi = m.env.data_va(312911);
            // c8c34 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // c8c39 mov rdi,rbx
            rdi = rbx;
            // c8c3c call 0xebfe4
            { m.outregs(0xc8c41, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c8c41 mov rdi,QWORD PTR [rip+0x77530] # 0x140178
            rdi = m.r64(m.env.data_va(346437)) as u64;
            // c8c48 lea rdx,[rip+0xfffffffffff36521] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // c8c4f mov rsi,rbx
            rsi = rbx;
            // c8c52 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c8c56 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c8c57 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8c58 jmp 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10348 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_00010348_rec)
pub fn f_00010348<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_00010348_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_00010348_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0x10348;
    loop {
        match pc {
        0x10348 => {
            // block 10348
            #[cfg(feature = "trace")] eprintln!("f_00010348 pc 10348 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10348 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10349 mov rbp,rsp
            rbp = rsp;
            // 1034c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1034e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1034f mov rbx,rsi
            rbx = rsi;
            // 10352 mov r14,rdi
            r14 = rdi;
            // 10355 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 10358 je 0x1037d
            if crate::cov::br(fl.e()) { pc = 0x1037d; continue; }
            pc = 0x1035a;
        }
        0x1035a => {
            // block 1035a
            #[cfg(feature = "trace")] eprintln!("f_00010348 pc 1035a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1035a mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 1035d mov rdi,r14
            rdi = r14;
            // 10360 call 0x10348
            { let r = f_00010348(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10365 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 10369 mov rdi,r14
            rdi = r14;
            // 1036c call 0x10348
            { let r = f_00010348(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10371 mov rdi,rbx
            rdi = rbx;
            // 10374 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10375 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10377 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10378 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x1037d => {
            // block 1037d
            #[cfg(feature = "trace")] eprintln!("f_00010348 pc 1037d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1037d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1037e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10380 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10381 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14c92 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014c92<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x14c92;
    loop {
        match pc {
        0x14c92 => {
            // block 14c92
            #[cfg(feature = "trace")] eprintln!("f_00014c92 pc 14c92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14c92 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14c93 mov rbp,rsp
            rbp = rsp;
            // 14c96 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14c97 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 14c98 mov rbx,rdi
            rbx = rdi;
            // 14c9b lea rdi,[rbx+0x10]
            rdi = rbx.wrapping_add(0x10);
            // 14c9f or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 14ca2 call 0xeb906
            { m.outregs(0x14ca7, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xd3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14ca7 mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 14caa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 14cae lea rdi,[rbx+rcx*1]
            rdi = rbx.wrapping_add(rcx);
            // 14cb2 test rax,rax
            fl = logic::<64>(rax & rax);
            // 14cb5 je 0x14cbb
            if crate::cov::br(fl.e()) { pc = 0x14cbb; continue; }
            pc = 0x14cb7;
        }
        0x14cb7 => {
            // block 14cb7
            #[cfg(feature = "trace")] eprintln!("f_00014c92 pc 14cb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14cb7 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 14cb9 jmp 0x14cc2
            pc = 0x14cc2; continue;
        }
        0x14cbb => {
            // block 14cbb
            #[cfg(feature = "trace")] eprintln!("f_00014c92 pc 14cbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14cbb mov esi,DWORD PTR [rcx+rbx*1+0x20]
            rsi = (m.r32(rcx.wrapping_add(rbx).wrapping_add(0x20)) as u64) as u32 as u64;
            // 14cbf or esi,0x4
            { let r = (rsi as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x14cc2;
        }
        0x14cc2 => {
            // block 14cc2
            #[cfg(feature = "trace")] eprintln!("f_00014c92 pc 14cc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14cc2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 14cc6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14cc7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14cc8 jmp 0xec0fe
            { let r = { let t = m.env.stub_va(0x8cb); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5a98 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000b5a98_rec)
pub fn f_000b5a98<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000b5a98_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000b5a98_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5a98;
    loop {
        match pc {
        0xb5a98 => {
            // block b5a98
            #[cfg(feature = "trace")] eprintln!("f_000b5a98 pc b5a98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5a98 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5a99 mov rbp,rsp
            rbp = rsp;
            // b5a9c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5a9e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5a9f mov rbx,rsi
            rbx = rsi;
            // b5aa2 mov r14,rdi
            r14 = rdi;
            // b5aa5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5aa8 je 0xb5ad6
            if crate::cov::br(fl.e()) { pc = 0xb5ad6; continue; }
            pc = 0xb5aaa;
        }
        0xb5aaa => {
            // block b5aaa
            #[cfg(feature = "trace")] eprintln!("f_000b5a98 pc b5aaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5aaa mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // b5aad mov rdi,r14
            rdi = r14;
            // b5ab0 call 0xb5a98
            { let r = f_000b5a98(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5ab5 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5ab9 mov rdi,r14
            rdi = r14;
            // b5abc call 0xb5a98
            { let r = f_000b5a98(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5ac1 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // b5ac5 call 0xec032
            { m.outregs(0xb5aca, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5aca mov rdi,rbx
            rdi = rbx;
            // b5acd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5ace pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5ad0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5ad1 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xb5ad6 => {
            // block b5ad6
            #[cfg(feature = "trace")] eprintln!("f_000b5a98 pc b5ad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ad6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5ad7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5ad9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5ada ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba62e size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000ba62e_rec)
pub fn f_000ba62e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000ba62e_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000ba62e_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba62e;
    loop {
        match pc {
        0xba62e => {
            // block ba62e
            #[cfg(feature = "trace")] eprintln!("f_000ba62e pc ba62e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba62e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba62f mov rbp,rsp
            rbp = rsp;
            // ba632 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba634 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba635 mov rbx,rsi
            rbx = rsi;
            // ba638 mov r14,rdi
            r14 = rdi;
            // ba63b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // ba63e je 0xba663
            if crate::cov::br(fl.e()) { pc = 0xba663; continue; }
            pc = 0xba640;
        }
        0xba640 => {
            // block ba640
            #[cfg(feature = "trace")] eprintln!("f_000ba62e pc ba640 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba640 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // ba643 mov rdi,r14
            rdi = r14;
            // ba646 call 0xba62e
            { let r = f_000ba62e(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba64b mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // ba64f mov rdi,r14
            rdi = r14;
            // ba652 call 0xba62e
            { let r = f_000ba62e(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba657 mov rdi,rbx
            rdi = rbx;
            // ba65a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba65b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba65d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba65e jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xba663 => {
            // block ba663
            #[cfg(feature = "trace")] eprintln!("f_000ba62e pc ba663 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba663 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba664 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba666 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba667 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3f8c size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000c3f8c_rec)
pub fn f_000c3f8c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000c3f8c_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000c3f8c_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3f8c;
    loop {
        match pc {
        0xc3f8c => {
            // block c3f8c
            #[cfg(feature = "trace")] eprintln!("f_000c3f8c pc c3f8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3f8c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3f8d mov rbp,rsp
            rbp = rsp;
            // c3f90 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3f92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3f93 mov rbx,rsi
            rbx = rsi;
            // c3f96 mov r14,rdi
            r14 = rdi;
            // c3f99 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3f9c je 0xc3fca
            if crate::cov::br(fl.e()) { pc = 0xc3fca; continue; }
            pc = 0xc3f9e;
        }
        0xc3f9e => {
            // block c3f9e
            #[cfg(feature = "trace")] eprintln!("f_000c3f8c pc c3f9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3f9e mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c3fa1 mov rdi,r14
            rdi = r14;
            // c3fa4 call 0xc3f8c
            { let r = f_000c3f8c(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3fa9 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c3fad mov rdi,r14
            rdi = r14;
            // c3fb0 call 0xc3f8c
            { let r = f_000c3f8c(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3fb5 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // c3fb9 call 0xec032
            { m.outregs(0xc3fbe, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3fbe mov rdi,rbx
            rdi = rbx;
            // c3fc1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3fc2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3fc4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3fc5 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc3fca => {
            // block c3fca
            #[cfg(feature = "trace")] eprintln!("f_000c3f8c pc c3fca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3fca pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3fcb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3fcd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3fce ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
