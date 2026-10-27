//! Mechanically lifted functions (scripts/lift_run.py), module w003. Each one is checked against the original
//! in the world harness (tests/world_w003.rs), except f_00031780 and f_000651c2, which never matched.
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xc40d0 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000c40d0_rec)
pub fn f_000c40d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000c40d0_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000c40d0_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
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
    let mut pc: u32 = 0xc40d0;
    loop {
        match pc {
        0xc40d0 => {
            // block c40d0
            #[cfg(feature = "trace")] eprintln!("f_000c40d0 pc c40d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c40d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c40d1 mov rbp,rsp
            rbp = rsp;
            // c40d4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c40d6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c40d7 mov rbx,rsi
            rbx = rsi;
            // c40da mov r14,rdi
            r14 = rdi;
            // c40dd test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c40e0 je 0xc4105
            if crate::cov::br(fl.e()) { pc = 0xc4105; continue; }
            pc = 0xc40e2;
        }
        0xc40e2 => {
            // block c40e2
            #[cfg(feature = "trace")] eprintln!("f_000c40d0 pc c40e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c40e2 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c40e5 mov rdi,r14
            rdi = r14;
            // c40e8 call 0xc40d0
            { let r = f_000c40d0(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c40ed mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c40f1 mov rdi,r14
            rdi = r14;
            // c40f4 call 0xc40d0
            { let r = f_000c40d0(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c40f9 mov rdi,rbx
            rdi = rbx;
            // c40fc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c40fd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c40ff pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4100 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc4105 => {
            // block c4105
            #[cfg(feature = "trace")] eprintln!("f_000c40d0 pc c4105 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4105 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4106 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4108 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4109 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x16160 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00016160<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x16160;
    loop {
        match pc {
        0x16160 => {
            // block 16160
            #[cfg(feature = "trace")] eprintln!("f_00016160 pc 16160 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16160 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 16161 mov rbp,rsp
            rbp = rsp;
            // 16164 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 16166 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 16167 lea rax,[rip+0x12b2aa] # 0x141418
            rax = m.env.data_va(351205);
            // 1616e mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 16171 mov QWORD PTR [rdi+0x18],0x0
            m.w64(rdi.wrapping_add(0x18), (0x0u64) as u64);
            // 16179 mov QWORD PTR [rdi+0x10],0x0
            m.w64(rdi.wrapping_add(0x10), (0x0u64) as u64);
            // 16181 mov QWORD PTR [rdi+0x8],0x0
            m.w64(rdi.wrapping_add(0x8), (0x0u64) as u64);
            // 16189 lea rbx,[rdi+0x8]
            rbx = rdi.wrapping_add(0x8);
            // 1618d call 0x161b4
            { m.outregs(0x16192, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x161b4); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16192 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 16193 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 16195 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 16196 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x16197 => {
            // block 16197
            #[cfg(feature = "trace")] eprintln!("f_00016160 pc 16197 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16197 mov r14,rax
            r14 = rax;
            // 1619a mov rdi,rbx
            rdi = rbx;
            // 1619d call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 161a2 mov rdi,r14
            rdi = r14;
            // 161a5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42b9e size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042b9e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x42b9e;
    loop {
        match pc {
        0x42b9e => {
            // block 42b9e
            #[cfg(feature = "trace")] eprintln!("f_00042b9e pc 42b9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42b9e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42b9f mov rbp,rsp
            rbp = rsp;
            // 42ba2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42ba4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42ba5 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 42ba8 mov rbx,rdi
            rbx = rdi;
            // 42bab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42bae call QWORD PTR [rax+0x278]
            { m.outregs(0x42bb4, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42bb4 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 42bb6 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42bb9 and edx,0xfff8
            { let r = (rdx as u32 as u64) & 0xfff8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 42bbf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 42bc1 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // 42bc4 ja 0x42be4
            if crate::cov::br(fl.a()) { pc = 0x42be4; continue; }
            pc = 0x42bc6;
        }
        0x42bc6 => {
            // block 42bc6
            #[cfg(feature = "trace")] eprintln!("f_00042b9e pc 42bc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42bc6 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 42bc9 jne 0x42be4
            if crate::cov::br(fl.ne()) { pc = 0x42be4; continue; }
            pc = 0x42bcb;
        }
        0x42bcb => {
            // block 42bcb
            #[cfg(feature = "trace")] eprintln!("f_00042b9e pc 42bcb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42bcb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42bce mov esi,0x904
            rsi = (0x904u64) as u32 as u64;
            // 42bd3 mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // 42bd8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42bdb mov rdi,rbx
            rdi = rbx;
            // 42bde mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42be1 call QWORD PTR [rax+0x30]
            { m.outregs(0x42be4, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x42be4;
        }
        0x42be4 => {
            // block 42be4
            #[cfg(feature = "trace")] eprintln!("f_00042b9e pc 42be4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42be4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42be5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42be7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42be8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1ff8 size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001ff8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x1ff8;
    loop {
        match pc {
        0x1ff8 => {
            // block 1ff8
            #[cfg(feature = "trace")] eprintln!("f_00001ff8 pc 1ff8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1ff8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1ff9 mov rbp,rsp
            rbp = rsp;
            // 1ffc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1ffe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2000 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2002 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2003 mov r14,rcx
            r14 = rcx;
            // 2006 mov rax,rdx
            rax = rdx;
            // 2009 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 200c mov rbx,rdi
            rbx = rdi;
            // 200f mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 2012 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 2015 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2017 mov rsi,rax
            rsi = rax;
            // 201a call QWORD PTR [rcx+0x388]
            { m.outregs(0x2020, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rcx.wrapping_add(0x388)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2020 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2022 je 0x2039
            if crate::cov::br(fl.e()) { pc = 0x2039; continue; }
            pc = 0x2024;
        }
        0x2024 => {
            // block 2024
            #[cfg(feature = "trace")] eprintln!("f_00001ff8 pc 2024 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2024 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2027 mov rdi,rbx
            rdi = rbx;
            // 202a mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 202d mov rdx,r14
            rdx = r14;
            // 2030 call QWORD PTR [rax+0x398]
            { m.outregs(0x2036, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x398)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2036 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x2039;
        }
        0x2039 => {
            // block 2039
            #[cfg(feature = "trace")] eprintln!("f_00001ff8 pc 2039 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2039 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 203c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 203d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 203f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2041 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2043 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2044 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5228c size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005228c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x5228c;
    loop {
        match pc {
        0x5228c => {
            // block 5228c
            #[cfg(feature = "trace")] eprintln!("f_0005228c pc 5228c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5228c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5228d mov rbp,rsp
            rbp = rsp;
            // 52290 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 52294 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // 52297 imul rcx,rax,0x10624dd3
            { let (r, f) = imul::<64>(rax, 0x10624dd3u64); rcx = r; fl = f; }
            // 5229e mov rdx,rcx
            rdx = rcx;
            // 522a1 shr rdx,0x3f
            { let (r, f) = shr::<64>(rdx, 0x3fu64, fl); rdx = r; fl = f; }
            // 522a5 sar rcx,0x26
            { let (r, f) = sar::<64>(rcx, 0x26u64, fl); rcx = r; fl = f; }
            // 522a9 add ecx,edx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rdx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // 522ab movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // 522ae mov QWORD PTR [rbp-0x10],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (rcx) as u64);
            // 522b2 imul rcx,rcx,0xfffffffffffffc18
            { let (r, f) = imul::<64>(rcx, 0xffff_ffff_ffff_fc18u64); rcx = r; fl = f; }
            // 522b9 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 522bc imul rax,rcx,0xf4240
            { let (r, f) = imul::<64>(rcx, 0xf4240u64); rax = r; fl = f; }
            // 522c3 mov QWORD PTR [rbp-0x8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff8), (rax) as u64);
            // 522c7 lea rdi,[rbp-0x10]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // 522cb xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 522cd call 0xec236
            { m.outregs(0x522d2, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa03); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 522d2 cwde
            rax = (rax as u16 as i16 as i32 as u32) as u64;
            // 522d3 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 522d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 522d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5eca size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005eca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x5eca;
    loop {
        match pc {
        0x5eca => {
            // block 5eca
            #[cfg(feature = "trace")] eprintln!("f_00005eca pc 5eca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5eca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5ecb mov rbp,rsp
            rbp = rsp;
            // 5ece push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5ed0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5ed2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5ed3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5ed4 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 5ed7 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5eda mov rbx,rdi
            rbx = rdi;
            // 5edd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5ee0 call QWORD PTR [rax+0x1d28]
            { m.outregs(0x5ee6, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5ee6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5ee8 je 0x5eee
            if crate::cov::br(fl.e()) { pc = 0x5eee; continue; }
            pc = 0x5eea;
        }
        0x5eea => {
            // block 5eea
            #[cfg(feature = "trace")] eprintln!("f_00005eca pc 5eea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5eea xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5eec jmp 0x5f11
            pc = 0x5f11; continue;
        }
        0x5eee => {
            // block 5eee
            #[cfg(feature = "trace")] eprintln!("f_00005eca pc 5eee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5eee mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5ef1 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 5ef4 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 5ef7 movzx esi,cl
            rsi = ((rcx as u8 as u64)) as u32 as u64;
            // 5efa or esi,0x44
            { let r = (rsi as u32 as u64) | 0x44u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5efd mov ecx,0x60000
            rcx = (0x60000u64) as u32 as u64;
            // 5f02 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 5f08 mov rdi,rbx
            rdi = rbx;
            // 5f0b mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 5f0e call QWORD PTR [rax+0x30]
            { m.outregs(0x5f11, 0x1d, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5f11;
        }
        0x5f11 => {
            // block 5f11
            #[cfg(feature = "trace")] eprintln!("f_00005eca pc 5f11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5f11 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5f15 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5f16 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5f18 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5f1a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5f1b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6ce8 size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006ce8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x6ce8;
    loop {
        match pc {
        0x6ce8 => {
            // block 6ce8
            #[cfg(feature = "trace")] eprintln!("f_00006ce8 pc 6ce8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6ce8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6ce9 mov rbp,rsp
            rbp = rsp;
            // 6cec push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6cee push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6cf0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6cf1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6cf2 mov r14,rdx
            r14 = rdx;
            // 6cf5 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 6cf8 mov rbx,rdi
            rbx = rdi;
            // 6cfb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6cfe call QWORD PTR [rax+0x1d30]
            { m.outregs(0x6d04, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6d04 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6d06 je 0x6d0c
            if crate::cov::br(fl.e()) { pc = 0x6d0c; continue; }
            pc = 0x6d08;
        }
        0x6d08 => {
            // block 6d08
            #[cfg(feature = "trace")] eprintln!("f_00006ce8 pc 6d08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6d08 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6d0a jmp 0x6d2e
            pc = 0x6d2e; continue;
        }
        0x6d0c => {
            // block 6d0c
            #[cfg(feature = "trace")] eprintln!("f_00006ce8 pc 6d0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6d0c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6d0f movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 6d13 lea rdx,[rip+0xf86a6] # 0xff3c0
            rdx = m.env.data_va(80781);
            // 6d1a mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6d1d mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // 6d22 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6d25 mov rdi,rbx
            rdi = rbx;
            // 6d28 mov rdx,r14
            rdx = r14;
            // 6d2b call QWORD PTR [rax+0x38]
            { m.outregs(0x6d2e, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6d2e;
        }
        0x6d2e => {
            // block 6d2e
            #[cfg(feature = "trace")] eprintln!("f_00006ce8 pc 6d2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6d2e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6d32 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6d33 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6d35 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6d37 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6d38 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19ba0 size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00019ba0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x19ba0;
    loop {
        match pc {
        0x19ba0 => {
            // block 19ba0
            #[cfg(feature = "trace")] eprintln!("f_00019ba0 pc 19ba0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ba0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19ba1 mov rbp,rsp
            rbp = rsp;
            // 19ba4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19ba5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19ba6 mov rbx,rdi
            rbx = rdi;
            // 19ba9 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 19bac test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 19baf je 0x19beb
            if crate::cov::br(fl.e()) { pc = 0x19beb; continue; }
            pc = 0x19bb1;
        }
        0x19bb1 => {
            // block 19bb1
            #[cfg(feature = "trace")] eprintln!("f_00019ba0 pc 19bb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19bb1 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 19bb5 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 19bb8 je 0x19bcf
            if crate::cov::br(fl.e()) { pc = 0x19bcf; continue; }
            pc = 0x19bba;
        }
        0x19bba => {
            // block 19bba
            #[cfg(feature = "trace")] eprintln!("f_00019ba0 pc 19bba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19bba lea rcx,[rax-0x4]
            rcx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19bbe sub rcx,rdi
            { let (r, f) = sub::<64>(rcx, rdi);
              rcx = r;
              fl = f; }
            // 19bc1 not rcx
            { let v = !rcx; rcx = v; }
            // 19bc4 and rcx,0xfffffffffffffffc
            { let r = rcx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rcx = r;
              fl = f; }
            // 19bc8 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 19bcb mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            pc = 0x19bcf;
        }
        0x19bcf => {
            // block 19bcf
            #[cfg(feature = "trace")] eprintln!("f_00019ba0 pc 19bcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19bcf call 0xec13a
            { m.outregs(0x19bd4, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19bd4 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 19bdc mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 19be4 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            pc = 0x19beb;
        }
        0x19beb => {
            // block 19beb
            #[cfg(feature = "trace")] eprintln!("f_00019ba0 pc 19beb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19beb add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19bef pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19bf0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19bf1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19f10 size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00019f10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x19f10;
    loop {
        match pc {
        0x19f10 => {
            // block 19f10
            #[cfg(feature = "trace")] eprintln!("f_00019f10 pc 19f10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19f11 mov rbp,rsp
            rbp = rsp;
            // 19f14 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19f15 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19f16 mov rbx,rdi
            rbx = rdi;
            // 19f19 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 19f1c test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 19f1f je 0x19f5b
            if crate::cov::br(fl.e()) { pc = 0x19f5b; continue; }
            pc = 0x19f21;
        }
        0x19f21 => {
            // block 19f21
            #[cfg(feature = "trace")] eprintln!("f_00019f10 pc 19f21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f21 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 19f25 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 19f28 je 0x19f3f
            if crate::cov::br(fl.e()) { pc = 0x19f3f; continue; }
            pc = 0x19f2a;
        }
        0x19f2a => {
            // block 19f2a
            #[cfg(feature = "trace")] eprintln!("f_00019f10 pc 19f2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f2a lea rcx,[rax-0x4]
            rcx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19f2e sub rcx,rdi
            { let (r, f) = sub::<64>(rcx, rdi);
              rcx = r;
              fl = f; }
            // 19f31 not rcx
            { let v = !rcx; rcx = v; }
            // 19f34 and rcx,0xfffffffffffffffc
            { let r = rcx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rcx = r;
              fl = f; }
            // 19f38 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 19f3b mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            pc = 0x19f3f;
        }
        0x19f3f => {
            // block 19f3f
            #[cfg(feature = "trace")] eprintln!("f_00019f10 pc 19f3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f3f call 0xec13a
            { m.outregs(0x19f44, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19f44 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 19f4c mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 19f54 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            pc = 0x19f5b;
        }
        0x19f5b => {
            // block 19f5b
            #[cfg(feature = "trace")] eprintln!("f_00019f10 pc 19f5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f5b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19f5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19f60 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19f61 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd663a size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d663a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xd663a;
    loop {
        match pc {
        0xd663a => {
            // block d663a
            #[cfg(feature = "trace")] eprintln!("f_000d663a pc d663a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d663a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d663b mov rbp,rsp
            rbp = rsp;
            // d663e mov QWORD PTR [rip+0xf1f87],0x0 # 0x1c85d0
            m.w64(m.env.data_va(904605), (0x0u64) as u64);
            // d6649 mov QWORD PTR [rip+0xf1f74],0x0 # 0x1c85c8
            m.w64(m.env.data_va(904597), (0x0u64) as u64);
            // d6654 lea rsi,[rip+0xf1f65] # 0x1c85c0
            rsi = m.env.data_va(904589);
            // d665b lea rax,[rip+0xf1f66] # 0x1c85c8
            rax = m.env.data_va(904597);
            // d6662 mov QWORD PTR [rip+0xf1f57],rax # 0x1c85c0
            m.w64(m.env.data_va(904589), (rax) as u64);
            // d6669 lea rdi,[rip+0xfffffffffffff6d0] # 0xd5d40
            rdi = m.env.code_va(0xd5d40);
            // d6670 lea rdx,[rip+0xfffffffffff28af9] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // d6677 call 0xec18e
            { m.outregs(0xd667c, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d667c lea rdi,[rip+0xf1f55] # 0x1c85d8
            rdi = m.env.data_va(904613);
            // d6683 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6684 jmp 0xd5ebe
            { let r = { let t = m.env.code_va(0xd5ebe); m.call(t, &[], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25b6 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000025b6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x25b6;
    loop {
        match pc {
        0x25b6 => {
            // block 25b6
            #[cfg(feature = "trace")] eprintln!("f_000025b6 pc 25b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25b6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25b7 mov rbp,rsp
            rbp = rsp;
            // 25ba push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 25bc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 25be push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 25bf push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 25c0 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 25c3 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 25c6 mov rbx,rdi
            rbx = rdi;
            // 25c9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 25cc call QWORD PTR [rax+0x1d28]
            { m.outregs(0x25d2, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25d2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 25d4 je 0x25da
            if crate::cov::br(fl.e()) { pc = 0x25da; continue; }
            pc = 0x25d6;
        }
        0x25d6 => {
            // block 25d6
            #[cfg(feature = "trace")] eprintln!("f_000025b6 pc 25d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 25d8 jmp 0x25fe
            pc = 0x25fe; continue;
        }
        0x25da => {
            // block 25da
            #[cfg(feature = "trace")] eprintln!("f_000025b6 pc 25da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 25dd mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 25e0 lea rdx,[rip+0xfca99] # 0xff080
            rdx = m.env.data_va(79949);
            // 25e7 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 25ea mov ecx,0x800
            rcx = (0x800u64) as u32 as u64;
            // 25ef mov r8d,0xb
            r8 = (0xbu64) as u32 as u64;
            // 25f5 mov rdi,rbx
            rdi = rbx;
            // 25f8 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 25fb call QWORD PTR [rax+0x30]
            { m.outregs(0x25fe, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x25fe;
        }
        0x25fe => {
            // block 25fe
            #[cfg(feature = "trace")] eprintln!("f_000025b6 pc 25fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25fe add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2602 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2603 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2605 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2607 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2608 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x277e size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000277e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x277e;
    loop {
        match pc {
        0x277e => {
            // block 277e
            #[cfg(feature = "trace")] eprintln!("f_0000277e pc 277e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 277e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 277f mov rbp,rsp
            rbp = rsp;
            // 2782 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2784 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2786 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2787 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2788 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 278b mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 278e mov rbx,rdi
            rbx = rdi;
            // 2791 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2794 call QWORD PTR [rax+0x1d28]
            { m.outregs(0x279a, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 279a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 279c je 0x27a2
            if crate::cov::br(fl.e()) { pc = 0x27a2; continue; }
            pc = 0x279e;
        }
        0x279e => {
            // block 279e
            #[cfg(feature = "trace")] eprintln!("f_0000277e pc 279e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 279e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 27a0 jmp 0x27c6
            pc = 0x27c6; continue;
        }
        0x27a2 => {
            // block 27a2
            #[cfg(feature = "trace")] eprintln!("f_0000277e pc 27a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27a2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 27a5 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 27a8 lea rdx,[rip+0xfc8d1] # 0xff080
            rdx = m.env.data_va(79949);
            // 27af mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 27b2 mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 27b7 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 27bd mov rdi,rbx
            rdi = rbx;
            // 27c0 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 27c3 call QWORD PTR [rax+0x30]
            { m.outregs(0x27c6, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x27c6;
        }
        0x27c6 => {
            // block 27c6
            #[cfg(feature = "trace")] eprintln!("f_0000277e pc 27c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27c6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 27ca pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 27cb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 27cd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 27cf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 27d0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2826 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002826<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x2826;
    loop {
        match pc {
        0x2826 => {
            // block 2826
            #[cfg(feature = "trace")] eprintln!("f_00002826 pc 2826 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2826 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2827 mov rbp,rsp
            rbp = rsp;
            // 282a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 282c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 282e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 282f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2830 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 2833 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2836 mov rbx,rdi
            rbx = rdi;
            // 2839 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 283c call QWORD PTR [rax+0x1d28]
            { m.outregs(0x2842, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2842 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2844 je 0x284a
            if crate::cov::br(fl.e()) { pc = 0x284a; continue; }
            pc = 0x2846;
        }
        0x2846 => {
            // block 2846
            #[cfg(feature = "trace")] eprintln!("f_00002826 pc 2846 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2846 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2848 jmp 0x286e
            pc = 0x286e; continue;
        }
        0x284a => {
            // block 284a
            #[cfg(feature = "trace")] eprintln!("f_00002826 pc 284a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 284a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 284d mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 2850 lea rdx,[rip+0xfc829] # 0xff080
            rdx = m.env.data_va(79949);
            // 2857 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 285a mov ecx,0x400
            rcx = (0x400u64) as u32 as u64;
            // 285f mov r8d,0xa
            r8 = (0xau64) as u32 as u64;
            // 2865 mov rdi,rbx
            rdi = rbx;
            // 2868 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 286b call QWORD PTR [rax+0x30]
            { m.outregs(0x286e, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x286e;
        }
        0x286e => {
            // block 286e
            #[cfg(feature = "trace")] eprintln!("f_00002826 pc 286e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 286e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2872 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2873 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2875 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2877 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2878 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3d3e size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003d3e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x3d3e;
    loop {
        match pc {
        0x3d3e => {
            // block 3d3e
            #[cfg(feature = "trace")] eprintln!("f_00003d3e pc 3d3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d3e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3d3f mov rbp,rsp
            rbp = rsp;
            // 3d42 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3d44 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3d45 mov r14,rsi
            r14 = rsi;
            // 3d48 mov rbx,rdi
            rbx = rdi;
            // 3d4b mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // 3d52 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3d55 call QWORD PTR [rax+0x108]
            { m.outregs(0x3d5b, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d5b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3d5d je 0x3d8a
            if crate::cov::br(fl.e()) { pc = 0x3d8a; continue; }
            pc = 0x3d5f;
        }
        0x3d5f => {
            // block 3d5f
            #[cfg(feature = "trace")] eprintln!("f_00003d3e pc 3d5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d5f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3d62 mov rdi,rbx
            rdi = rbx;
            // 3d65 call QWORD PTR [rax+0x100]
            { m.outregs(0x3d6b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d6b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3d6d je 0x3d8a
            if crate::cov::br(fl.e()) { pc = 0x3d8a; continue; }
            pc = 0x3d6f;
        }
        0x3d6f => {
            // block 3d6f
            #[cfg(feature = "trace")] eprintln!("f_00003d3e pc 3d6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d6f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3d72 mov esi,0x4000b
            rsi = (0x4000bu64) as u32 as u64;
            // 3d77 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3d7c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3d7f mov rdi,rbx
            rdi = rbx;
            // 3d82 mov rdx,r14
            rdx = r14;
            // 3d85 call QWORD PTR [rax+0x38]
            { m.outregs(0x3d88, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d88 jmp 0x3d8c
            pc = 0x3d8c; continue;
        }
        0x3d8a => {
            // block 3d8a
            #[cfg(feature = "trace")] eprintln!("f_00003d3e pc 3d8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d8a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x3d8c;
        }
        0x3d8c => {
            // block 3d8c
            #[cfg(feature = "trace")] eprintln!("f_00003d3e pc 3d8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d8c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3d8d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3d8f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3d90 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4ef4 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004ef4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x4ef4;
    loop {
        match pc {
        0x4ef4 => {
            // block 4ef4
            #[cfg(feature = "trace")] eprintln!("f_00004ef4 pc 4ef4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ef4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4ef5 mov rbp,rsp
            rbp = rsp;
            // 4ef8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4efa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4efc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4efd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4efe mov r14,rdx
            r14 = rdx;
            // 4f01 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 4f04 mov rbx,rdi
            rbx = rdi;
            // 4f07 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4f0a call QWORD PTR [rax+0x1d28]
            { m.outregs(0x4f10, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f10 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4f12 je 0x4f18
            if crate::cov::br(fl.e()) { pc = 0x4f18; continue; }
            pc = 0x4f14;
        }
        0x4f14 => {
            // block 4f14
            #[cfg(feature = "trace")] eprintln!("f_00004ef4 pc 4f14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f14 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4f16 jmp 0x4f3c
            pc = 0x4f3c; continue;
        }
        0x4f18 => {
            // block 4f18
            #[cfg(feature = "trace")] eprintln!("f_00004ef4 pc 4f18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f18 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 4f1b lea rcx,[rip+0xfa3ce] # 0xff2f0
            rcx = m.env.data_va(80573);
            // 4f22 mov esi,DWORD PTR [rcx+rax*4]
            rsi = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 4f25 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4f28 mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // 4f2d mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // 4f33 mov rdi,rbx
            rdi = rbx;
            // 4f36 mov rdx,r14
            rdx = r14;
            // 4f39 call QWORD PTR [rax+0x38]
            { m.outregs(0x4f3c, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4f3c;
        }
        0x4f3c => {
            // block 4f3c
            #[cfg(feature = "trace")] eprintln!("f_00004ef4 pc 4f3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f3c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4f40 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4f41 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4f43 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4f45 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4f46 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5754 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005754<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x5754;
    loop {
        match pc {
        0x5754 => {
            // block 5754
            #[cfg(feature = "trace")] eprintln!("f_00005754 pc 5754 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5754 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5755 mov rbp,rsp
            rbp = rsp;
            // 5758 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 575a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 575c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 575d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 575e mov r14,rdx
            r14 = rdx;
            // 5761 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5764 mov rbx,rdi
            rbx = rdi;
            // 5767 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 576a call QWORD PTR [rax+0x1d28]
            { m.outregs(0x5770, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5770 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5772 je 0x5778
            if crate::cov::br(fl.e()) { pc = 0x5778; continue; }
            pc = 0x5774;
        }
        0x5774 => {
            // block 5774
            #[cfg(feature = "trace")] eprintln!("f_00005754 pc 5774 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5774 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5776 jmp 0x579c
            pc = 0x579c; continue;
        }
        0x5778 => {
            // block 5778
            #[cfg(feature = "trace")] eprintln!("f_00005754 pc 5778 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5778 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 577b mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 577e lea rdx,[rip+0xf98fb] # 0xff080
            rdx = m.env.data_va(79949);
            // 5785 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5788 mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 578d mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 5793 mov rdi,rbx
            rdi = rbx;
            // 5796 mov rdx,r14
            rdx = r14;
            // 5799 call QWORD PTR [rax+0x38]
            { m.outregs(0x579c, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x579c;
        }
        0x579c => {
            // block 579c
            #[cfg(feature = "trace")] eprintln!("f_00005754 pc 579c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 579c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 57a0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 57a1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 57a3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 57a5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 57a6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6b3e size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006b3e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x6b3e;
    loop {
        match pc {
        0x6b3e => {
            // block 6b3e
            #[cfg(feature = "trace")] eprintln!("f_00006b3e pc 6b3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6b3e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6b3f mov rbp,rsp
            rbp = rsp;
            // 6b42 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6b44 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6b46 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6b47 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6b48 mov r14,rdx
            r14 = rdx;
            // 6b4b mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 6b4e mov rbx,rdi
            rbx = rdi;
            // 6b51 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6b54 call QWORD PTR [rax+0x1d28]
            { m.outregs(0x6b5a, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6b5a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6b5c je 0x6b62
            if crate::cov::br(fl.e()) { pc = 0x6b62; continue; }
            pc = 0x6b5e;
        }
        0x6b5e => {
            // block 6b5e
            #[cfg(feature = "trace")] eprintln!("f_00006b3e pc 6b5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6b5e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6b60 jmp 0x6b86
            pc = 0x6b86; continue;
        }
        0x6b62 => {
            // block 6b62
            #[cfg(feature = "trace")] eprintln!("f_00006b3e pc 6b62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6b62 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6b65 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 6b68 lea rdx,[rip+0xf8511] # 0xff080
            rdx = m.env.data_va(79949);
            // 6b6f mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6b72 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 6b77 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 6b7d mov rdi,rbx
            rdi = rbx;
            // 6b80 mov rdx,r14
            rdx = r14;
            // 6b83 call QWORD PTR [rax+0x38]
            { m.outregs(0x6b86, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6b86;
        }
        0x6b86 => {
            // block 6b86
            #[cfg(feature = "trace")] eprintln!("f_00006b3e pc 6b86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6b86 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6b8a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6b8b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6b8d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6b8f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6b90 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd918 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d918<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xd918;
    loop {
        match pc {
        0xd918 => {
            // block d918
            #[cfg(feature = "trace")] eprintln!("f_0000d918 pc d918 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d918 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d919 mov rbp,rsp
            rbp = rsp;
            // d91c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d91e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d920 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d921 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d922 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d925 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d928 mov rbx,rdi
            rbx = rdi;
            // d92b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d92e call QWORD PTR [rax+0x1d28]
            { m.outregs(0xd934, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d934 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d936 je 0xd93c
            if crate::cov::br(fl.e()) { pc = 0xd93c; continue; }
            pc = 0xd938;
        }
        0xd938 => {
            // block d938
            #[cfg(feature = "trace")] eprintln!("f_0000d918 pc d938 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d938 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d93a jmp 0xd961
            pc = 0xd961; continue;
        }
        0xd93c => {
            // block d93c
            #[cfg(feature = "trace")] eprintln!("f_0000d918 pc d93c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d93c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d93f mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // d942 lea rdx,[rip+0xf1a77] # 0xff3c0
            rdx = m.env.data_va(80781);
            // d949 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // d94c movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // d950 mov ecx,0x1000000
            rcx = (0x1000000u64) as u32 as u64;
            // d955 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // d95b mov rdi,rbx
            rdi = rbx;
            // d95e call QWORD PTR [rax+0x30]
            { m.outregs(0xd961, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd961;
        }
        0xd961 => {
            // block d961
            #[cfg(feature = "trace")] eprintln!("f_0000d918 pc d961 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d961 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d965 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d966 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d968 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d96a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d96b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18aa8 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018aa8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x18aa8;
    loop {
        match pc {
        0x18aa8 => {
            // block 18aa8
            #[cfg(feature = "trace")] eprintln!("f_00018aa8 pc 18aa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18aa8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18aa9 mov rbp,rsp
            rbp = rsp;
            // 18aac push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18aae push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18ab0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 18ab2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18ab3 mov r12,rsi
            r12 = rsi;
            // 18ab6 mov r14,rdi
            r14 = rdi;
            // 18ab9 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 18abd cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 18ac2 je 0x18af0
            if crate::cov::br(fl.e()) { pc = 0x18af0; continue; }
            pc = 0x18ac4;
        }
        0x18ac4 => {
            // block 18ac4
            #[cfg(feature = "trace")] eprintln!("f_00018aa8 pc 18ac4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18ac4 lea r15,[rip+0x1117c9] # 0x12a294
            r15 = m.env.data_va(256609);
            pc = 0x18acb;
        }
        0x18acb => {
            // block 18acb
            #[cfg(feature = "trace")] eprintln!("f_00018aa8 pc 18acb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18acb mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 18ad0 mov rdi,r14
            rdi = r14;
            // 18ad3 mov rsi,r15
            rsi = r15;
            // 18ad6 call 0xebb88
            { m.outregs(0x18adb, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18adb mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 18add mov rdi,rax
            rdi = rax;
            // 18ae0 call 0xec098
            { m.outregs(0x18ae5, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ae5 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 18ae9 cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 18aee jne 0x18acb
            if crate::cov::br(fl.ne()) { pc = 0x18acb; continue; }
            pc = 0x18af0;
        }
        0x18af0 => {
            // block 18af0
            #[cfg(feature = "trace")] eprintln!("f_00018aa8 pc 18af0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18af0 mov rax,r14
            rax = r14;
            // 18af3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18af4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18af6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18af8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18afa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18afb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18c4d size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018c4d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x18c4d;
    loop {
        match pc {
        0x18c4d => {
            // block 18c4d
            #[cfg(feature = "trace")] eprintln!("f_00018c4d pc 18c4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c4d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18c4e mov rbp,rsp
            rbp = rsp;
            // 18c51 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18c53 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18c55 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 18c57 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18c58 mov r12,rsi
            r12 = rsi;
            // 18c5b mov r14,rdi
            r14 = rdi;
            // 18c5e mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 18c62 cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 18c67 je 0x18c95
            if crate::cov::br(fl.e()) { pc = 0x18c95; continue; }
            pc = 0x18c69;
        }
        0x18c69 => {
            // block 18c69
            #[cfg(feature = "trace")] eprintln!("f_00018c4d pc 18c69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c69 lea r15,[rip+0x111624] # 0x12a294
            r15 = m.env.data_va(256609);
            pc = 0x18c70;
        }
        0x18c70 => {
            // block 18c70
            #[cfg(feature = "trace")] eprintln!("f_00018c4d pc 18c70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c70 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 18c75 mov rdi,r14
            rdi = r14;
            // 18c78 mov rsi,r15
            rsi = r15;
            // 18c7b call 0xebb88
            { m.outregs(0x18c80, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18c80 mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 18c82 mov rdi,rax
            rdi = rax;
            // 18c85 call 0xec098
            { m.outregs(0x18c8a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18c8a add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 18c8e cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 18c93 jne 0x18c70
            if crate::cov::br(fl.ne()) { pc = 0x18c70; continue; }
            pc = 0x18c95;
        }
        0x18c95 => {
            // block 18c95
            #[cfg(feature = "trace")] eprintln!("f_00018c4d pc 18c95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c95 mov rax,r14
            rax = r14;
            // 18c98 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18c99 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18c9b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18c9d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18c9f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18ca0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3eba size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000c3eba_rec)
pub fn f_000c3eba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000c3eba_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000c3eba_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
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
    let mut pc: u32 = 0xc3eba;
    loop {
        match pc {
        0xc3eba => {
            // block c3eba
            #[cfg(feature = "trace")] eprintln!("f_000c3eba pc c3eba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3eba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3ebb mov rbp,rsp
            rbp = rsp;
            // c3ebe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3ec0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3ec1 mov rbx,rsi
            rbx = rsi;
            // c3ec4 mov r14,rdi
            r14 = rdi;
            // c3ec7 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3eca je 0xc3ef8
            if crate::cov::br(fl.e()) { pc = 0xc3ef8; continue; }
            pc = 0xc3ecc;
        }
        0xc3ecc => {
            // block c3ecc
            #[cfg(feature = "trace")] eprintln!("f_000c3eba pc c3ecc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3ecc mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c3ecf mov rdi,r14
            rdi = r14;
            // c3ed2 call 0xc3eba
            { let r = f_000c3eba(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3ed7 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c3edb mov rdi,r14
            rdi = r14;
            // c3ede call 0xc3eba
            { let r = f_000c3eba(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &[]); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3ee3 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // c3ee7 call 0xec032
            { m.outregs(0xc3eec, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3eec mov rdi,rbx
            rdi = rbx;
            // c3eef pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3ef0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3ef2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3ef3 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc3ef8 => {
            // block c3ef8
            #[cfg(feature = "trace")] eprintln!("f_000c3eba pc c3ef8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3ef8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3ef9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3efb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3efc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc44fe size=87
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c44fe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xc44fe;
    loop {
        match pc {
        0xc44fe => {
            // block c44fe
            #[cfg(feature = "trace")] eprintln!("f_000c44fe pc c44fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c44fe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c44ff mov rbp,rsp
            rbp = rsp;
            // c4502 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4504 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4505 mov r14,rsi
            r14 = rsi;
            // c4508 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // c450c test rax,rax
            fl = logic::<64>(rax & rax);
            // c450f je 0xc451e
            if crate::cov::br(fl.e()) { pc = 0xc451e; continue; }
            pc = 0xc4511;
        }
        0xc4511 => {
            // block c4511
            #[cfg(feature = "trace")] eprintln!("f_000c44fe pc c4511 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4511 mov rbx,rax
            rbx = rax;
            // c4514 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c4517 test rax,rax
            fl = logic::<64>(rax & rax);
            // c451a jne 0xc4511
            if crate::cov::br(fl.ne()) { pc = 0xc4511; continue; }
            pc = 0xc451c;
        }
        0xc451c => {
            // block c451c
            #[cfg(feature = "trace")] eprintln!("f_000c44fe pc c451c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c451c jmp 0xc452d
            pc = 0xc452d; continue;
        }
        0xc451e => {
            // block c451e
            #[cfg(feature = "trace")] eprintln!("f_000c44fe pc c451e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c451e mov rax,r14
            rax = r14;
            pc = 0xc4521;
        }
        0xc4521 => {
            // block c4521
            #[cfg(feature = "trace")] eprintln!("f_000c44fe pc c4521 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4521 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // c4525 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // c4528 mov rax,rbx
            rax = rbx;
            // c452b jne 0xc4521
            if crate::cov::br(fl.ne()) { pc = 0xc4521; continue; }
            pc = 0xc452d;
        }
        0xc452d => {
            // block c452d
            #[cfg(feature = "trace")] eprintln!("f_000c44fe pc c452d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c452d cmp QWORD PTR [rdi],r14
            fl = sub::<64>(m.r64(rdi) as u64, r14).1;
            // c4530 jne 0xc4535
            if crate::cov::br(fl.ne()) { pc = 0xc4535; continue; }
            pc = 0xc4532;
        }
        0xc4532 => {
            // block c4532
            #[cfg(feature = "trace")] eprintln!("f_000c44fe pc c4532 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4532 mov QWORD PTR [rdi],rbx
            m.w64(rdi, (rbx) as u64);
            pc = 0xc4535;
        }
        0xc4535 => {
            // block c4535
            #[cfg(feature = "trace")] eprintln!("f_000c44fe pc c4535 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4535 dec QWORD PTR [rdi+0x10]
            { let (r, f) = dec::<64>(m.r64(rdi.wrapping_add(0x10)) as u64, fl); m.w64(rdi.wrapping_add(0x10), (r) as u64); fl = f; }
            // c4539 mov rdi,QWORD PTR [rdi+0x8]
            rdi = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c453d mov rsi,r14
            rsi = r14;
            // c4540 call 0xebd02
            { m.outregs(0xc4545, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x4cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4545 mov rdi,r14
            rdi = r14;
            // c4548 call 0xec13a
            { m.outregs(0xc454d, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c454d mov rax,rbx
            rax = rbx;
            // c4550 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4551 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4553 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4554 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc431c size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c431c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xc431c;
    loop {
        match pc {
        0xc431c => {
            // block c431c
            #[cfg(feature = "trace")] eprintln!("f_000c431c pc c431c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c431c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c431d mov rbp,rsp
            rbp = rsp;
            // c4320 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c4322 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4324 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4325 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c4326 mov r14,rsi
            r14 = rsi;
            // c4329 mov r15,rdi
            r15 = rdi;
            // c432c mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // c4331 call 0xec146
            { m.outregs(0xc4336, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4336 mov rbx,rax
            rbx = rax;
            // c4339 mov eax,DWORD PTR [r14]
            rax = (m.r32(r14) as u64) as u32 as u64;
            // c433c mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c433f mov eax,DWORD PTR [r14+0x4]
            rax = (m.r32(r14.wrapping_add(0x4)) as u64) as u32 as u64;
            // c4343 mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // c4346 mov rdi,r15
            rdi = r15;
            // c4349 mov rsi,rbx
            rsi = rbx;
            // c434c call 0xebe10
            { m.outregs(0xc4351, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x5dd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4351 mov r14,rax
            r14 = rax;
            // c4354 mov r15b,dl
            r15 = (r15 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // c4357 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // c435b jne 0xc4365
            if crate::cov::br(fl.ne()) { pc = 0xc4365; continue; }
            pc = 0xc435d;
        }
        0xc435d => {
            // block c435d
            #[cfg(feature = "trace")] eprintln!("f_000c431c pc c435d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c435d mov rdi,rbx
            rdi = rbx;
            // c4360 call 0xec13a
            { m.outregs(0xc4365, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc4365;
        }
        0xc4365 => {
            // block c4365
            #[cfg(feature = "trace")] eprintln!("f_000c431c pc c4365 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4365 mov rax,r14
            rax = r14;
            // c4368 mov dl,r15b
            rdx = (rdx & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // c436b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c436f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4370 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4372 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c4374 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4375 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x288de size=93
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000288de<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x288de;
    loop {
        match pc {
        0x288de => {
            // block 288de
            #[cfg(feature = "trace")] eprintln!("f_000288de pc 288de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 288de push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 288df mov rbp,rsp
            rbp = rsp;
            // 288e2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 288e4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 288e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 288e7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 288e8 mov r15,rsi
            r15 = rsi;
            // 288eb mov r14,rdi
            r14 = rdi;
            // 288ee mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 288f1 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 288f5 jmp 0x28928
            pc = 0x28928; continue;
        }
        0x288f7 => {
            // block 288f7
            #[cfg(feature = "trace")] eprintln!("f_000288de pc 288f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 288f7 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 288fb mov rdi,r14
            rdi = r14;
            // 288fe call 0xeb9cc
            { m.outregs(0x28903, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x199); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28903 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 28907 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2890a je 0x28919
            if crate::cov::br(fl.e()) { pc = 0x28919; continue; }
            pc = 0x2890c;
        }
        0x2890c => {
            // block 2890c
            #[cfg(feature = "trace")] eprintln!("f_000288de pc 2890c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2890c mov rbx,rax
            rbx = rax;
            // 2890f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28912 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28915 jne 0x2890c
            if crate::cov::br(fl.ne()) { pc = 0x2890c; continue; }
            pc = 0x28917;
        }
        0x28917 => {
            // block 28917
            #[cfg(feature = "trace")] eprintln!("f_000288de pc 28917 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28917 jmp 0x28928
            pc = 0x28928; continue;
        }
        0x28919 => {
            // block 28919
            #[cfg(feature = "trace")] eprintln!("f_000288de pc 28919 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28919 mov rax,rbx
            rax = rbx;
            pc = 0x2891c;
        }
        0x2891c => {
            // block 2891c
            #[cfg(feature = "trace")] eprintln!("f_000288de pc 2891c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2891c mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 28920 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 28923 mov rax,rbx
            rax = rbx;
            // 28926 jne 0x2891c
            if crate::cov::br(fl.ne()) { pc = 0x2891c; continue; }
            pc = 0x28928;
        }
        0x28928 => {
            // block 28928
            #[cfg(feature = "trace")] eprintln!("f_000288de pc 28928 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28928 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 2892b jne 0x288f7
            if crate::cov::br(fl.ne()) { pc = 0x288f7; continue; }
            pc = 0x2892d;
        }
        0x2892d => {
            // block 2892d
            #[cfg(feature = "trace")] eprintln!("f_000288de pc 2892d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2892d mov rax,r14
            rax = r14;
            // 28930 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 28934 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28935 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28937 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28939 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2893a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4038 size=103
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4038<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xe4038;
    loop {
        match pc {
        0xe4038 => {
            // block e4038
            #[cfg(feature = "trace")] eprintln!("f_000e4038 pc e4038 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4038 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4039 mov rbp,rsp
            rbp = rsp;
            // e403c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e403e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e403f mov rbx,rdi
            rbx = rdi;
            // e4042 mov rax,QWORD PTR [rip+0x5c36f] # 0x1403b8
            rax = m.r64(m.env.data_va(347013)) as u64;
            // e4049 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // e404d mov QWORD PTR [rbx],rcx
            m.w64(rbx, (rcx) as u64);
            // e4050 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // e4054 mov QWORD PTR [rbx+0x78],rax
            m.w64(rbx.wrapping_add(0x78), (rax) as u64);
            // e4058 lea r14,[rbx+0x10]
            r14 = rbx.wrapping_add(0x10);
            // e405c mov rax,QWORD PTR [rip+0x5c345] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // e4063 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // e4067 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e406b lea rdi,[rbx+0x50]
            rdi = rbx.wrapping_add(0x50);
            // e406f call 0xec032
            { m.outregs(0xe4074, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4074 mov rdi,r14
            rdi = r14;
            // e4077 call 0xec0c8
            { m.outregs(0xe407c, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e407c mov rsi,QWORD PTR [rip+0x5c17d] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // e4083 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e4087 lea r14,[rbx+0x78]
            r14 = rbx.wrapping_add(0x78);
            // e408b mov rdi,rbx
            rdi = rbx;
            // e408e call 0xec056
            { m.outregs(0xe4093, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4093 mov rdi,r14
            rdi = r14;
            // e4096 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4097 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4099 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e409a jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x100c0 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000100c0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x100c0;
    loop {
        match pc {
        0x100c0 => {
            // block 100c0
            #[cfg(feature = "trace")] eprintln!("f_000100c0 pc 100c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 100c0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 100c1 mov rbp,rsp
            rbp = rsp;
            // 100c4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 100c6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 100c7 mov rbx,rdi
            rbx = rdi;
            // 100ca mov rax,QWORD PTR [rip+0x1302ef] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 100d1 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 100d5 mov QWORD PTR [rbx],rcx
            m.w64(rbx, (rcx) as u64);
            // 100d8 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 100dc mov QWORD PTR [rbx+0x70],rax
            m.w64(rbx.wrapping_add(0x70), (rax) as u64);
            // 100e0 lea r14,[rbx+0x8]
            r14 = rbx.wrapping_add(0x8);
            // 100e4 mov rax,QWORD PTR [rip+0x1302bd] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 100eb add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 100ef mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 100f3 lea rdi,[rbx+0x48]
            rdi = rbx.wrapping_add(0x48);
            // 100f7 call 0xec032
            { m.outregs(0x100fc, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 100fc mov rdi,r14
            rdi = r14;
            // 100ff call 0xec0c8
            { m.outregs(0x10104, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10104 mov rsi,QWORD PTR [rip+0x1300fd] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 1010b add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 1010f lea r14,[rbx+0x70]
            r14 = rbx.wrapping_add(0x70);
            // 10113 mov rdi,rbx
            rdi = rbx;
            // 10116 call 0xec080
            { m.outregs(0x1011b, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1011b mov rdi,r14
            rdi = r14;
            // 1011e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1011f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10121 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10122 jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15f1c size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00015f1c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x15f1c;
    loop {
        match pc {
        0x15f1c => {
            // block 15f1c
            #[cfg(feature = "trace")] eprintln!("f_00015f1c pc 15f1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f1c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15f1d mov rbp,rsp
            rbp = rsp;
            // 15f20 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15f21 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15f22 mov rbx,rdi
            rbx = rdi;
            // 15f25 mov eax,DWORD PTR [rsi+0x60]
            rax = (m.r32(rsi.wrapping_add(0x60)) as u64) as u32 as u64;
            // 15f28 test al,0x10
            fl = logic::<8>((rax as u8 as u64) & 0x10u64);
            // 15f2a jne 0x15f49
            if crate::cov::br(fl.ne()) { pc = 0x15f49; continue; }
            pc = 0x15f2c;
        }
        0x15f2c => {
            // block 15f2c
            #[cfg(feature = "trace")] eprintln!("f_00015f1c pc 15f2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f2c test al,0x8
            fl = logic::<8>((rax as u8 as u64) & 0x8u64);
            // 15f2e jne 0x15f66
            if crate::cov::br(fl.ne()) { pc = 0x15f66; continue; }
            pc = 0x15f30;
        }
        0x15f30 => {
            // block 15f30
            #[cfg(feature = "trace")] eprintln!("f_00015f1c pc 15f30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f30 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 15f38 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 15f40 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 15f47 jmp 0x15f79
            pc = 0x15f79; continue;
        }
        0x15f49 => {
            // block 15f49
            #[cfg(feature = "trace")] eprintln!("f_00015f1c pc 15f49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f49 mov rax,QWORD PTR [rsi+0x30]
            rax = m.r64(rsi.wrapping_add(0x30)) as u64;
            // 15f4d mov rdx,QWORD PTR [rsi+0x58]
            rdx = m.r64(rsi.wrapping_add(0x58)) as u64;
            // 15f51 cmp rdx,rax
            fl = sub::<64>(rdx, rax).1;
            // 15f54 jae 0x15f5d
            if crate::cov::br(fl.ae()) { pc = 0x15f5d; continue; }
            pc = 0x15f56;
        }
        0x15f56 => {
            // block 15f56
            #[cfg(feature = "trace")] eprintln!("f_00015f1c pc 15f56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f56 mov QWORD PTR [rsi+0x58],rax
            m.w64(rsi.wrapping_add(0x58), (rax) as u64);
            // 15f5a mov rdx,rax
            rdx = rax;
            pc = 0x15f5d;
        }
        0x15f5d => {
            // block 15f5d
            #[cfg(feature = "trace")] eprintln!("f_00015f1c pc 15f5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f5d mov rsi,QWORD PTR [rsi+0x28]
            rsi = m.r64(rsi.wrapping_add(0x28)) as u64;
            // 15f61 mov rdi,rbx
            rdi = rbx;
            // 15f64 jmp 0x15f74
            pc = 0x15f74; continue;
        }
        0x15f66 => {
            // block 15f66
            #[cfg(feature = "trace")] eprintln!("f_00015f1c pc 15f66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f66 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // 15f6a mov rdx,QWORD PTR [rsi+0x20]
            rdx = m.r64(rsi.wrapping_add(0x20)) as u64;
            // 15f6e mov rdi,rbx
            rdi = rbx;
            // 15f71 mov rsi,rax
            rsi = rax;
            pc = 0x15f74;
        }
        0x15f74 => {
            // block 15f74
            #[cfg(feature = "trace")] eprintln!("f_00015f1c pc 15f74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f74 call 0xeb8fa
            { m.outregs(0x15f79, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xc7); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x15f79;
        }
        0x15f79 => {
            // block 15f79
            #[cfg(feature = "trace")] eprintln!("f_00015f1c pc 15f79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f79 mov rax,rbx
            rax = rbx;
            // 15f7c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 15f80 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15f81 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15f82 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25c98 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025c98<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x25c98;
    loop {
        match pc {
        0x25c98 => {
            // block 25c98
            #[cfg(feature = "trace")] eprintln!("f_00025c98 pc 25c98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25c98 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25c99 mov rbp,rsp
            rbp = rsp;
            // 25c9c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 25c9e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 25ca0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 25ca2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 25ca3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 25ca7 mov r15,rsi
            r15 = rsi;
            // 25caa mov r14,rdi
            r14 = rdi;
            // 25cad lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 25cb1 mov rdx,r15
            rdx = r15;
            // 25cb4 call 0xeb9c6
            { m.outregs(0x25cb9, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x193); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25cb9 mov r12,rax
            r12 = rax;
            // 25cbc mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 25cc0 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 25cc3 je 0x25cc9
            if crate::cov::br(fl.e()) { pc = 0x25cc9; continue; }
            pc = 0x25cc5;
        }
        0x25cc5 => {
            // block 25cc5
            #[cfg(feature = "trace")] eprintln!("f_00025c98 pc 25cc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25cc5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 25cc7 jmp 0x25cf0
            pc = 0x25cf0; continue;
        }
        0x25cc9 => {
            // block 25cc9
            #[cfg(feature = "trace")] eprintln!("f_00025c98 pc 25cc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25cc9 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 25cce call 0xec146
            { m.outregs(0x25cd3, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25cd3 mov rbx,rax
            rbx = rax;
            // 25cd6 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 25cd9 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 25cdc mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 25ce0 mov rdi,r14
            rdi = r14;
            // 25ce3 mov rdx,r12
            rdx = r12;
            // 25ce6 mov rcx,rbx
            rcx = rbx;
            // 25ce9 call 0xebae6
            { m.outregs(0x25cee, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x2b3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25cee mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x25cf0;
        }
        0x25cf0 => {
            // block 25cf0
            #[cfg(feature = "trace")] eprintln!("f_00025c98 pc 25cf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25cf0 mov rax,rbx
            rax = rbx;
            // 25cf3 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 25cf7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25cf8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 25cfa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25cfc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 25cfe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25cff ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2da20 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002da20<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x2da20;
    loop {
        match pc {
        0x2da20 => {
            // block 2da20
            #[cfg(feature = "trace")] eprintln!("f_0002da20 pc 2da20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2da20 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2da21 mov rbp,rsp
            rbp = rsp;
            // 2da24 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2da26 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2da28 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2da2a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2da2b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2da2f mov r15,rsi
            r15 = rsi;
            // 2da32 mov r14,rdi
            r14 = rdi;
            // 2da35 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2da39 mov rdx,r15
            rdx = r15;
            // 2da3c call 0xebac8
            { m.outregs(0x2da41, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x295); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2da41 mov r12,rax
            r12 = rax;
            // 2da44 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 2da48 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2da4b je 0x2da51
            if crate::cov::br(fl.e()) { pc = 0x2da51; continue; }
            pc = 0x2da4d;
        }
        0x2da4d => {
            // block 2da4d
            #[cfg(feature = "trace")] eprintln!("f_0002da20 pc 2da4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2da4d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2da4f jmp 0x2da78
            pc = 0x2da78; continue;
        }
        0x2da51 => {
            // block 2da51
            #[cfg(feature = "trace")] eprintln!("f_0002da20 pc 2da51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2da51 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 2da56 call 0xec146
            { m.outregs(0x2da5b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2da5b mov rbx,rax
            rbx = rax;
            // 2da5e mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 2da61 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 2da64 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 2da68 mov rdi,r14
            rdi = r14;
            // 2da6b mov rdx,r12
            rdx = r12;
            // 2da6e mov rcx,rbx
            rcx = rbx;
            // 2da71 call 0xeb99c
            { m.outregs(0x2da76, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x169); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2da76 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x2da78;
        }
        0x2da78 => {
            // block 2da78
            #[cfg(feature = "trace")] eprintln!("f_0002da20 pc 2da78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2da78 mov rax,rbx
            rax = rbx;
            // 2da7b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2da7f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2da80 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2da82 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2da84 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2da86 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2da87 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x65506 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00065506<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x65506;
    loop {
        match pc {
        0x65506 => {
            // block 65506
            #[cfg(feature = "trace")] eprintln!("f_00065506 pc 65506 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65506 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 65507 mov rbp,rsp
            rbp = rsp;
            // 6550a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6550c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6550e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65510 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65511 mov r12,rdx
            r12 = rdx;
            // 65514 mov r14,rsi
            r14 = rsi;
            // 65517 mov rbx,rdi
            rbx = rdi;
            // 6551a mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // 6551f call 0xec146
            { m.outregs(0x65524, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65524 mov r15,rax
            r15 = rax;
            // 65527 lea rdi,[r15+0x20]
            rdi = r15.wrapping_add(0x20);
            // 6552b mov rsi,r12
            rsi = r12;
            // 6552e call 0xec026
            { m.outregs(0x65533, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65533 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 65537 mov DWORD PTR [r15+0x38],0x0
            m.w32(r15.wrapping_add(0x38), (0x0u64) as u32);
            // 6553f mov QWORD PTR [rbx],r15
            m.w64(rbx, (r15) as u64);
            // 65542 mov QWORD PTR [rbx+0x8],r14
            m.w64(rbx.wrapping_add(0x8), (r14) as u64);
            // 65546 mov QWORD PTR [rbx+0x10],0x101
            m.w64(rbx.wrapping_add(0x10), (0x101u64) as u64);
            // 6554e mov rax,rbx
            rax = rbx;
            // 65551 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 65552 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 65554 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 65556 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 65558 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 65559 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6555a => {
            // block 6555a
            #[cfg(feature = "trace")] eprintln!("f_00065506 pc 6555a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6555a mov rbx,rax
            rbx = rax;
            // 6555d mov rdi,r15
            rdi = r15;
            // 65560 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65565 mov rdi,rbx
            rdi = rbx;
            // 65568 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4b62 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4b62<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xc4b62;
    loop {
        match pc {
        0xc4b62 => {
            // block c4b62
            #[cfg(feature = "trace")] eprintln!("f_000c4b62 pc c4b62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b62 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4b63 mov rbp,rsp
            rbp = rsp;
            // c4b66 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c4b68 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4b6a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c4b6c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4b6d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c4b71 mov r15,rsi
            r15 = rsi;
            // c4b74 mov r14,rdi
            r14 = rdi;
            // c4b77 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c4b7b mov rdx,r15
            rdx = r15;
            // c4b7e call 0xebd6e
            { m.outregs(0xc4b83, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x53b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4b83 mov r12,rax
            r12 = rax;
            // c4b86 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // c4b8a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c4b8d je 0xc4b93
            if crate::cov::br(fl.e()) { pc = 0xc4b93; continue; }
            pc = 0xc4b8f;
        }
        0xc4b8f => {
            // block c4b8f
            #[cfg(feature = "trace")] eprintln!("f_000c4b62 pc c4b8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b8f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c4b91 jmp 0xc4bba
            pc = 0xc4bba; continue;
        }
        0xc4b93 => {
            // block c4b93
            #[cfg(feature = "trace")] eprintln!("f_000c4b62 pc c4b93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b93 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // c4b98 call 0xec146
            { m.outregs(0xc4b9d, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4b9d mov rbx,rax
            rbx = rax;
            // c4ba0 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // c4ba3 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c4ba6 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c4baa mov rdi,r14
            rdi = r14;
            // c4bad mov rdx,r12
            rdx = r12;
            // c4bb0 mov rcx,rbx
            rcx = rbx;
            // c4bb3 call 0xebd7a
            { m.outregs(0xc4bb8, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x547); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4bb8 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xc4bba;
        }
        0xc4bba => {
            // block c4bba
            #[cfg(feature = "trace")] eprintln!("f_000c4b62 pc c4bba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4bba mov rax,rbx
            rax = rbx;
            // c4bbd add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c4bc1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4bc2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c4bc4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4bc6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c4bc8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4bc9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc500e size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c500e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xc500e;
    loop {
        match pc {
        0xc500e => {
            // block c500e
            #[cfg(feature = "trace")] eprintln!("f_000c500e pc c500e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c500e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c500f mov rbp,rsp
            rbp = rsp;
            // c5012 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c5014 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c5016 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c5018 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c5019 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c501d mov r15,rsi
            r15 = rsi;
            // c5020 mov r14,rdi
            r14 = rdi;
            // c5023 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c5027 mov rdx,r15
            rdx = r15;
            // c502a call 0xebda4
            { m.outregs(0xc502f, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x571); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c502f mov r12,rax
            r12 = rax;
            // c5032 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // c5036 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c5039 je 0xc503f
            if crate::cov::br(fl.e()) { pc = 0xc503f; continue; }
            pc = 0xc503b;
        }
        0xc503b => {
            // block c503b
            #[cfg(feature = "trace")] eprintln!("f_000c500e pc c503b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c503b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c503d jmp 0xc5066
            pc = 0xc5066; continue;
        }
        0xc503f => {
            // block c503f
            #[cfg(feature = "trace")] eprintln!("f_000c500e pc c503f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c503f mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // c5044 call 0xec146
            { m.outregs(0xc5049, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5049 mov rbx,rax
            rbx = rax;
            // c504c mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // c504f mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c5052 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c5056 mov rdi,r14
            rdi = r14;
            // c5059 mov rdx,r12
            rdx = r12;
            // c505c mov rcx,rbx
            rcx = rbx;
            // c505f call 0xebdb0
            { m.outregs(0xc5064, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x57d); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5064 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xc5066;
        }
        0xc5066 => {
            // block c5066
            #[cfg(feature = "trace")] eprintln!("f_000c500e pc c5066 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5066 mov rax,rbx
            rax = rbx;
            // c5069 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c506d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c506e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c5070 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c5072 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c5074 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c5075 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc5610 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c5610<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xc5610;
    loop {
        match pc {
        0xc5610 => {
            // block c5610
            #[cfg(feature = "trace")] eprintln!("f_000c5610 pc c5610 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5610 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c5611 mov rbp,rsp
            rbp = rsp;
            // c5614 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c5616 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c5618 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c561a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c561b mov r12,rdx
            r12 = rdx;
            // c561e mov r14,rsi
            r14 = rsi;
            // c5621 mov rbx,rdi
            rbx = rdi;
            // c5624 mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c5629 call 0xec146
            { m.outregs(0xc562e, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c562e mov r15,rax
            r15 = rax;
            // c5631 lea rdi,[r15+0x20]
            rdi = r15.wrapping_add(0x20);
            // c5635 mov rsi,r12
            rsi = r12;
            // c5638 call 0xec026
            { m.outregs(0xc563d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c563d add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c5641 mov DWORD PTR [r15+0x38],0x0
            m.w32(r15.wrapping_add(0x38), (0x0u64) as u32);
            // c5649 mov QWORD PTR [rbx],r15
            m.w64(rbx, (r15) as u64);
            // c564c mov QWORD PTR [rbx+0x8],r14
            m.w64(rbx.wrapping_add(0x8), (r14) as u64);
            // c5650 mov QWORD PTR [rbx+0x10],0x101
            m.w64(rbx.wrapping_add(0x10), (0x101u64) as u64);
            // c5658 mov rax,rbx
            rax = rbx;
            // c565b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c565c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c565e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c5660 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c5662 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c5663 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc5664 => {
            // block c5664
            #[cfg(feature = "trace")] eprintln!("f_000c5610 pc c5664 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5664 mov rbx,rax
            rbx = rax;
            // c5667 mov rdi,r15
            rdi = r15;
            // c566a call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c566f mov rdi,rbx
            rdi = rbx;
            // c5672 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0afe size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0afe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xe0afe;
    loop {
        match pc {
        0xe0afe => {
            // block e0afe
            #[cfg(feature = "trace")] eprintln!("f_000e0afe pc e0afe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0afe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0aff mov rbp,rsp
            rbp = rsp;
            // e0b02 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0b03 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0b04 mov rbx,rdi
            rbx = rdi;
            // e0b07 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0b0a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0b0c call QWORD PTR [rax+0x1a48]
            { m.outregs(0xe0b12, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1a48)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0b12 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0b15 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0b17 mov rdi,rbx
            rdi = rbx;
            // e0b1a call QWORD PTR [rax+0x1a50]
            { m.outregs(0xe0b20, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1a50)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0b20 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0b23 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e0b28 mov rdi,rbx
            rdi = rbx;
            // e0b2b call QWORD PTR [rax+0x1a58]
            { m.outregs(0xe0b31, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1a58)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0b31 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0b34 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e0b39 mov rdi,rbx
            rdi = rbx;
            // e0b3c call QWORD PTR [rax+0x1a60]
            { m.outregs(0xe0b42, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1a60)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0b42 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0b45 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0b47 mov rdi,rbx
            rdi = rbx;
            // e0b4a call QWORD PTR [rax+0x1a28]
            { m.outregs(0xe0b50, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1a28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0b50 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0b53 mov rax,QWORD PTR [rax+0x1aa8]
            rax = m.r64(rax.wrapping_add(0x1aa8)) as u64;
            // e0b5a mov esi,0x10000
            rsi = (0x10000u64) as u32 as u64;
            // e0b5f mov rdi,rbx
            rdi = rbx;
            // e0b62 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e0b66 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0b67 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0b68 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64af9 size=111
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064af9<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x64af9;
    loop {
        match pc {
        0x64af9 => {
            // block 64af9
            #[cfg(feature = "trace")] eprintln!("f_00064af9 pc 64af9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64af9 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64afa mov rbp,rsp
            rbp = rsp;
            // 64afd push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 64aff push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 64b01 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 64b02 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 64b03 mov r14,rsi
            r14 = rsi;
            // 64b06 mov r15,rdi
            r15 = rdi;
            // 64b09 lea rax,[r15+0x8]
            rax = r15.wrapping_add(0x8);
            // 64b0d mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // 64b15 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 64b1d mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 64b20 mov rbx,QWORD PTR [r14]
            rbx = m.r64(r14) as u64;
            // 64b23 jmp 0x64b29
            pc = 0x64b29; continue;
        }
        0x64b25 => {
            // block 64b25
            #[cfg(feature = "trace")] eprintln!("f_00064af9 pc 64b25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64b25 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            pc = 0x64b29;
        }
        0x64b29 => {
            // block 64b29
            #[cfg(feature = "trace")] eprintln!("f_00064af9 pc 64b29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64b29 cmp rbx,QWORD PTR [r14+0x8]
            fl = sub::<64>(rbx, m.r64(r14.wrapping_add(0x8)) as u64).1;
            // 64b2d je 0x64b53
            if crate::cov::br(fl.e()) { pc = 0x64b53; continue; }
            pc = 0x64b2f;
        }
        0x64b2f => {
            // block 64b2f
            #[cfg(feature = "trace")] eprintln!("f_00064af9 pc 64b2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64b2f mov rdi,r15
            rdi = r15;
            // 64b32 mov rsi,rbx
            rsi = rbx;
            // 64b35 call 0xebbb8
            { m.outregs(0x64b3a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64b3a jmp 0x64b25
            pc = 0x64b25; continue;
        }
        0x64b3c => {
            // block 64b3c
            #[cfg(feature = "trace")] eprintln!("f_00064af9 pc 64b3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64b3c mov rbx,rax
            rbx = rax;
            // 64b3f mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 64b43 mov rdi,r15
            rdi = r15;
            // 64b46 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64b4b mov rdi,rbx
            rdi = rbx;
            // 64b4e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x64b53;
        }
        0x64b53 => {
            // block 64b53
            #[cfg(feature = "trace")] eprintln!("f_00064af9 pc 64b53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64b53 mov rax,r15
            rax = r15;
            // 64b56 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 64b5a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64b5b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 64b5d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 64b5f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64b60 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14dfc size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014dfc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x14dfc;
    loop {
        match pc {
        0x14dfc => {
            // block 14dfc
            #[cfg(feature = "trace")] eprintln!("f_00014dfc pc 14dfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14dfc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14dfd mov rbp,rsp
            rbp = rsp;
            // 14e00 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 14e02 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14e03 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 14e06 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 14e0a lea rbx,[rdi+rax*1]
            rbx = rdi.wrapping_add(rax);
            // 14e0e mov rcx,QWORD PTR [rip+0x12b58b] # 0x1403a0
            rcx = m.r64(m.env.data_va(346989)) as u64;
            // 14e15 lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // 14e19 mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // 14e1d lea r14,[rdi+rax*1+0x1a8]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x1a8);
            // 14e25 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // 14e29 mov QWORD PTR [rdi+rax*1+0x1a8],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x1a8), (rcx) as u64);
            // 14e31 lea rdi,[rdi+rax*1+0x10]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x10);
            // 14e36 call 0xeb912
            { m.outregs(0x14e3b, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14e3b mov rsi,QWORD PTR [rip+0x12b3ae] # 0x1401f0
            rsi = m.r64(m.env.data_va(346557)) as u64;
            // 14e42 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 14e46 mov rdi,rbx
            rdi = rbx;
            // 14e49 call 0xec056
            { m.outregs(0x14e4e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14e4e mov rdi,r14
            rdi = r14;
            // 14e51 call 0xec116
            { m.outregs(0x14e56, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14e56 mov rdi,rbx
            rdi = rbx;
            // 14e59 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14e5a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 14e5c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14e5d jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a2e0 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a2e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x1a2e0;
    loop {
        match pc {
        0x1a2e0 => {
            // block 1a2e0
            #[cfg(feature = "trace")] eprintln!("f_0001a2e0 pc 1a2e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a2e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a2e1 mov rbp,rsp
            rbp = rsp;
            // 1a2e4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a2e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a2e7 mov r14,rsi
            r14 = rsi;
            // 1a2ea mov rbx,rdi
            rbx = rdi;
            // 1a2ed mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1a2f5 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1a2fd mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 1a304 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a308 sub rsi,QWORD PTR [r14]
            { let (r, f) = sub::<64>(rsi, m.r64(r14) as u64);
              rsi = r;
              fl = f; }
            // 1a30b sar rsi,0x2
            { let (r, f) = sar::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 1a30f je 0x1a337
            if crate::cov::br(fl.e()) { pc = 0x1a337; continue; }
            pc = 0x1a311;
        }
        0x1a311 => {
            // block 1a311
            #[cfg(feature = "trace")] eprintln!("f_0001a2e0 pc 1a311 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a311 mov rdi,rbx
            rdi = rbx;
            // 1a314 call 0xeba68
            { m.outregs(0x1a319, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x235); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a319 mov rsi,QWORD PTR [r14]
            rsi = m.r64(r14) as u64;
            // 1a31c mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a320 sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // 1a323 mov r14,rdx
            r14 = rdx;
            // 1a326 and r14,0xfffffffffffffffc
            { let r = r14 & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              r14 = r;
              fl = f; }
            // 1a32a mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1a32e call 0xec21e
            { m.outregs(0x1a333, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a333 add QWORD PTR [rbx+0x8],r14
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, r14);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            pc = 0x1a337;
        }
        0x1a337 => {
            // block 1a337
            #[cfg(feature = "trace")] eprintln!("f_0001a2e0 pc 1a337 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a337 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a338 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a33a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a33b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1a33c => {
            // block 1a33c
            #[cfg(feature = "trace")] eprintln!("f_0001a2e0 pc 1a33c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a33c mov r14,rax
            r14 = rax;
            // 1a33f mov rdi,rbx
            rdi = rbx;
            // 1a342 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a347 mov rdi,r14
            rdi = r14;
            // 1a34a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a3c0 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a3c0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x1a3c0;
    loop {
        match pc {
        0x1a3c0 => {
            // block 1a3c0
            #[cfg(feature = "trace")] eprintln!("f_0001a3c0 pc 1a3c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a3c0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a3c1 mov rbp,rsp
            rbp = rsp;
            // 1a3c4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a3c6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a3c7 mov r14,rsi
            r14 = rsi;
            // 1a3ca mov rbx,rdi
            rbx = rdi;
            // 1a3cd mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1a3d5 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1a3dd mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 1a3e4 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a3e8 sub rsi,QWORD PTR [r14]
            { let (r, f) = sub::<64>(rsi, m.r64(r14) as u64);
              rsi = r;
              fl = f; }
            // 1a3eb sar rsi,0x2
            { let (r, f) = sar::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 1a3ef je 0x1a417
            if crate::cov::br(fl.e()) { pc = 0x1a417; continue; }
            pc = 0x1a3f1;
        }
        0x1a3f1 => {
            // block 1a3f1
            #[cfg(feature = "trace")] eprintln!("f_0001a3c0 pc 1a3f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a3f1 mov rdi,rbx
            rdi = rbx;
            // 1a3f4 call 0xeb9fc
            { m.outregs(0x1a3f9, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x1c9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a3f9 mov rsi,QWORD PTR [r14]
            rsi = m.r64(r14) as u64;
            // 1a3fc mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a400 sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // 1a403 mov r14,rdx
            r14 = rdx;
            // 1a406 and r14,0xfffffffffffffffc
            { let r = r14 & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              r14 = r;
              fl = f; }
            // 1a40a mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1a40e call 0xec21e
            { m.outregs(0x1a413, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a413 add QWORD PTR [rbx+0x8],r14
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, r14);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            pc = 0x1a417;
        }
        0x1a417 => {
            // block 1a417
            #[cfg(feature = "trace")] eprintln!("f_0001a3c0 pc 1a417 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a417 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a418 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a41a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a41b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1a41c => {
            // block 1a41c
            #[cfg(feature = "trace")] eprintln!("f_0001a3c0 pc 1a41c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a41c mov r14,rax
            r14 = rax;
            // 1a41f mov rdi,rbx
            rdi = rbx;
            // 1a422 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a427 mov rdi,r14
            rdi = r14;
            // 1a42a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4b3a size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4b3a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xd4b3a;
    loop {
        match pc {
        0xd4b3a => {
            // block d4b3a
            #[cfg(feature = "trace")] eprintln!("f_000d4b3a pc d4b3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4b3a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4b3b mov rbp,rsp
            rbp = rsp;
            // d4b3e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4b40 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4b42 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d4b44 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d4b46 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4b47 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d4b48 mov rbx,rcx
            rbx = rcx;
            // d4b4b mov r12,rdi
            r12 = rdi;
            // d4b4e mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // d4b51 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // d4b53 je 0xd4b98
            if crate::cov::br(fl.e()) { pc = 0xd4b98; continue; }
            pc = 0xd4b55;
        }
        0xd4b55 => {
            // block d4b55
            #[cfg(feature = "trace")] eprintln!("f_000d4b3a pc d4b55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4b55 mov QWORD PTR [rbp-0x30],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdx) as u64);
            // d4b59 mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // d4b5c add rbx,0xc
            { let (r, f) = add::<64>(rbx, 0xcu64);
              rbx = r;
              fl = f; }
            // d4b60 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd4b63;
        }
        0xd4b63 => {
            // block d4b63
            #[cfg(feature = "trace")] eprintln!("f_000d4b3a pc d4b63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4b63 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d4b67 mov esi,DWORD PTR [rbx-0xc]
            rsi = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // d4b6a mov ecx,DWORD PTR [rbx-0x4]
            rcx = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // d4b6d mov r8d,DWORD PTR [rbx]
            r8 = (m.r32(rbx) as u64) as u32 as u64;
            // d4b70 lea rdx,[rbx-0x8]
            rdx = rbx.wrapping_add(0xffff_ffff_ffff_fff8);
            // d4b74 mov rdi,r12
            rdi = r12;
            // d4b77 call QWORD PTR [rax+0x38]
            { m.outregs(0xd4b7a, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4b7a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d4b7c je 0xd4b8c
            if crate::cov::br(fl.e()) { pc = 0xd4b8c; continue; }
            pc = 0xd4b7e;
        }
        0xd4b7e => {
            // block d4b7e
            #[cfg(feature = "trace")] eprintln!("f_000d4b3a pc d4b7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4b7e inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // d4b81 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // d4b85 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // d4b88 jb 0xd4b63
            if crate::cov::br(fl.b()) { pc = 0xd4b63; continue; }
            pc = 0xd4b8a;
        }
        0xd4b8a => {
            // block d4b8a
            #[cfg(feature = "trace")] eprintln!("f_000d4b3a pc d4b8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4b8a jmp 0xd4b98
            pc = 0xd4b98; continue;
        }
        0xd4b8c => {
            // block d4b8c
            #[cfg(feature = "trace")] eprintln!("f_000d4b3a pc d4b8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4b8c mov eax,DWORD PTR [rbx-0xc]
            rax = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // d4b8f mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // d4b93 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // d4b95 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd4b98;
        }
        0xd4b98 => {
            // block d4b98
            #[cfg(feature = "trace")] eprintln!("f_000d4b3a pc d4b98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4b98 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // d4b9b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d4b9f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4ba0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d4ba2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d4ba4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4ba6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d4ba8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4ba9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29d6 size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000029d6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x29d6;
    loop {
        match pc {
        0x29d6 => {
            // block 29d6
            #[cfg(feature = "trace")] eprintln!("f_000029d6 pc 29d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29d7 mov rbp,rsp
            rbp = rsp;
            // 29da push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 29dc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 29de push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 29e0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29e1 mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // 29e4 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 29e7 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 29ea mov rbx,rdi
            rbx = rdi;
            // 29ed mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 29f0 call QWORD PTR [rax+0x1d28]
            { m.outregs(0x29f6, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29f6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 29f8 je 0x29ff
            if crate::cov::br(fl.e()) { pc = 0x29ff; continue; }
            pc = 0x29fa;
        }
        0x29fa => {
            // block 29fa
            #[cfg(feature = "trace")] eprintln!("f_000029d6 pc 29fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29fa xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 29fd jmp 0x2a3c
            pc = 0x2a3c; continue;
        }
        0x29ff => {
            // block 29ff
            #[cfg(feature = "trace")] eprintln!("f_000029d6 pc 29ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29ff mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2a02 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 2a05 lea rdx,[rip+0xfc6a4] # 0xff0b0
            rdx = m.env.data_va(79997);
            // 2a0c mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2a0f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2a14 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2a17 mov rdi,rbx
            rdi = rbx;
            // 2a1a mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 2a1d call QWORD PTR [rax+0x30]
            { m.outregs(0x2a20, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2a20 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2a23 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 2a26 je 0x2a3c
            if crate::cov::br(fl.e()) { pc = 0x2a3c; continue; }
            pc = 0x2a28;
        }
        0x2a28 => {
            // block 2a28
            #[cfg(feature = "trace")] eprintln!("f_000029d6 pc 2a28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2a28 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2a2b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2a30 mov rdi,rbx
            rdi = rbx;
            // 2a33 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 2a36 call QWORD PTR [rax+0xbd0]
            { m.outregs(0x2a3c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2a3c;
        }
        0x2a3c => {
            // block 2a3c
            #[cfg(feature = "trace")] eprintln!("f_000029d6 pc 2a3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2a3c mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 2a3f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2a40 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2a42 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2a44 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2a46 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2a47 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1574c size=146
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001574c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x1574c;
    loop {
        match pc {
        0x1574c => {
            // block 1574c
            #[cfg(feature = "trace")] eprintln!("f_0001574c pc 1574c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1574c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1574d mov rbp,rsp
            rbp = rsp;
            // 15750 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15752 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15754 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15756 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15757 mov rbx,rsi
            rbx = rsi;
            // 1575a mov r14,rdi
            r14 = rdi;
            // 1575d mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // 15762 call 0xec146
            { m.outregs(0x15767, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15767 mov r12,rax
            r12 = rax;
            // 1576a mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 1576e mov QWORD PTR [r12+0x30],rax
            m.w64(r12.wrapping_add(0x30), (rax) as u64);
            // 15773 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 15776 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1577a mov QWORD PTR [r12+0x28],rcx
            m.w64(r12.wrapping_add(0x28), (rcx) as u64);
            // 1577f mov QWORD PTR [r12+0x20],rax
            m.w64(r12.wrapping_add(0x20), (rax) as u64);
            // 15784 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1578c mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 15794 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 1579b mov eax,DWORD PTR [rbx+0x18]
            rax = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 1579e mov DWORD PTR [r12+0x38],eax
            m.w32(r12.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 157a3 mov rdi,r14
            rdi = r14;
            // 157a6 mov rsi,r12
            rsi = r12;
            // 157a9 call 0xeb936
            { m.outregs(0x157ae, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x103); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 157ae mov r14,rax
            r14 = rax;
            // 157b1 mov r15b,dl
            r15 = (r15 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 157b4 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // 157b8 jne 0x157ce
            if crate::cov::br(fl.ne()) { pc = 0x157ce; continue; }
            pc = 0x157ba;
        }
        0x157ba => {
            // block 157ba
            #[cfg(feature = "trace")] eprintln!("f_0001574c pc 157ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 157ba mov rdi,r12
            rdi = r12;
            // 157bd add rdi,0x20
            { let (r, f) = add::<64>(rdi, 0x20u64);
              rdi = r;
              fl = f; }
            // 157c1 call 0xec032
            { m.outregs(0x157c6, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 157c6 mov rdi,r12
            rdi = r12;
            // 157c9 call 0xec13a
            { m.outregs(0x157ce, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x157ce;
        }
        0x157ce => {
            // block 157ce
            #[cfg(feature = "trace")] eprintln!("f_0001574c pc 157ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 157ce mov rax,r14
            rax = r14;
            // 157d1 mov dl,r15b
            rdx = (rdx & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 157d4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 157d5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 157d7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 157d9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 157db pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 157dc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe435c size=264
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e435c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xe435c;
    loop {
        match pc {
        0xe435c => {
            // block e435c
            #[cfg(feature = "trace")] eprintln!("f_000e435c pc e435c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e435c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e435d mov rbp,rsp
            rbp = rsp;
            // e4360 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4362 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4364 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e4366 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e4368 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4369 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e436a mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e436d mov r13,rdi
            r13 = rdi;
            // e4370 lea rdi,[r13+0x34]
            rdi = r13.wrapping_add(0x34);
            // e4374 call 0xec2c0
            { m.outregs(0xe4379, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4379 mov r15,rax
            r15 = rax;
            // e437c mov esi,0xffffffff
            rsi = (0xffffffffu64) as u32 as u64;
            // e4381 mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // e4386 mov rdi,r13
            rdi = r13;
            // e4389 call 0xe4464
            { m.outregs(0xe438e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe4464); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e438e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e4390 je 0xe4432
            if crate::cov::br(fl.e()) { pc = 0xe4432; continue; }
            pc = 0xe4396;
        }
        0xe4396 => {
            // block e4396
            #[cfg(feature = "trace")] eprintln!("f_000e435c pc e4396 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4396 mov eax,DWORD PTR [r13+0x2048]
            rax = (m.r32(r13.wrapping_add(0x2048)) as u64) as u32 as u64;
            // e439d inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e439f mov DWORD PTR [r13+0x2048],eax
            m.w32(r13.wrapping_add(0x2048), ((rax as u32 as u64)) as u32);
            // e43a6 cmp eax,0x1f4
            fl = sub::<32>((rax as u32 as u64), 0x1f4u64).1;
            // e43ab mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e43b0 cmovbe esi,eax
            if fl.be() { rsi = ((rax as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // e43b3 mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // e43b8 mov rdi,r13
            rdi = r13;
            // e43bb call 0xe4464
            { m.outregs(0xe43c0, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe4464); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e43c0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e43c2 je 0xe443e
            if crate::cov::br(fl.e()) { pc = 0xe443e; continue; }
            pc = 0xe43c4;
        }
        0xe43c4 => {
            // block e43c4
            #[cfg(feature = "trace")] eprintln!("f_000e435c pc e43c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e43c4 lea r12d,[r15+0x4]
            r12 = (r15.wrapping_add(0x4)) as u32 as u64;
            // e43c8 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // e43cb sar eax,0x1f
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e43ce shr eax,0x1e
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1eu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e43d1 add eax,r15d
            { let (r, f) = add::<32>((rax as u32 as u64), (r15 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e43d4 and eax,0xfffffffc
            { let r = (rax as u32 as u64) & 0xfffffffcu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e43d7 sub r15d,eax
            { let (r, f) = sub::<32>((r15 as u32 as u64), (rax as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // e43da sub r12d,r15d
            { let (r, f) = sub::<32>((r12 as u32 as u64), (r15 as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // e43dd mov r15d,r12d
            r15 = ((r12 as u32 as u64)) as u32 as u64;
            // e43e0 sar r15d,0x1f
            { let (r, f) = sar::<32>((r15 as u32 as u64), 0x1fu64, fl); r15 = (r) as u32 as u64; fl = f; }
            // e43e4 shr r15d,0x1e
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x1eu64, fl); r15 = (r) as u32 as u64; fl = f; }
            // e43e8 add r15d,r12d
            { let (r, f) = add::<32>((r15 as u32 as u64), (r12 as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // e43eb sar r15d,0x2
            { let (r, f) = sar::<32>((r15 as u32 as u64), 0x2u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // e43ef mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // e43f4 mov rdi,r13
            rdi = r13;
            // e43f7 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e43fa call 0xe4464
            { m.outregs(0xe43ff, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe4464); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e43ff test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e4401 je 0xe444a
            if crate::cov::br(fl.e()) { pc = 0xe444a; continue; }
            pc = 0xe4403;
        }
        0xe4403 => {
            // block e4403
            #[cfg(feature = "trace")] eprintln!("f_000e435c pc e4403 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4403 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e4405 cmp r12d,0x3
            fl = sub::<32>((r12 as u32 as u64), 0x3u64).1;
            // e4409 jle 0xe4425
            if crate::cov::br(fl.le()) { pc = 0xe4425; continue; }
            pc = 0xe440b;
        }
        0xe440b => {
            // block e440b
            #[cfg(feature = "trace")] eprintln!("f_000e435c pc e440b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e440b mov esi,DWORD PTR [r13+rbx*4+0x34]
            rsi = (m.r32(r13.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x34)) as u64) as u32 as u64;
            // e4410 mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // e4415 mov rdi,r13
            rdi = r13;
            // e4418 call 0xe4464
            { m.outregs(0xe441d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe4464); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e441d inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e4420 cmp ebx,r15d
            fl = sub::<32>((rbx as u32 as u64), (r15 as u32 as u64)).1;
            // e4423 jl 0xe440b
            if crate::cov::br(fl.l()) { pc = 0xe440b; continue; }
            pc = 0xe4425;
        }
        0xe4425 => {
            // block e4425
            #[cfg(feature = "trace")] eprintln!("f_000e435c pc e4425 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4425 mov rdi,r13
            rdi = r13;
            // e4428 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // e442b call 0xe44a8
            { m.outregs(0xe4430, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe44a8); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4430 jmp 0xe4454
            pc = 0xe4454; continue;
        }
        0xe4432 => {
            // block e4432
            #[cfg(feature = "trace")] eprintln!("f_000e435c pc e4432 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4432 mov DWORD PTR [r13+0x8],0xe
            m.w32(r13.wrapping_add(0x8), (0xeu64) as u32);
            // e443a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e443c jmp 0xe4454
            pc = 0xe4454; continue;
        }
        0xe443e => {
            // block e443e
            #[cfg(feature = "trace")] eprintln!("f_000e435c pc e443e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e443e mov DWORD PTR [r13+0x8],0xf
            m.w32(r13.wrapping_add(0x8), (0xfu64) as u32);
            // e4446 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4448 jmp 0xe4454
            pc = 0xe4454; continue;
        }
        0xe444a => {
            // block e444a
            #[cfg(feature = "trace")] eprintln!("f_000e435c pc e444a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e444a mov DWORD PTR [r13+0x8],0x10
            m.w32(r13.wrapping_add(0x8), (0x10u64) as u32);
            // e4452 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4454;
        }
        0xe4454 => {
            // block e4454
            #[cfg(feature = "trace")] eprintln!("f_000e435c pc e4454 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4454 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e4458 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4459 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e445b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e445d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e445f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4461 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4462 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: unproven, tier P2v @0x31780 size=268
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00031780<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x31780;
    loop {
        match pc {
        0x31780 => {
            // block 31780
            #[cfg(feature = "trace")] eprintln!("f_00031780 pc 31780 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31780 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 31781 mov rbp,rsp
            rbp = rsp;
            // 31784 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 31786 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31788 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 3178a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 3178c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3178d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 31791 mov r14,rsi
            r14 = rsi;
            // 31794 mov r12,rdi
            r12 = rdi;
            // 31797 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 3179b mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 317a0 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 317a3 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 317a6 jns 0x317b4
            if crate::cov::br(fl.ns()) { pc = 0x317b4; continue; }
            pc = 0x317a8;
        }
        0x317a8 => {
            // block 317a8
            #[cfg(feature = "trace")] eprintln!("f_00031780 pc 317a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 317a8 mov rdi,r12
            rdi = r12;
            // 317ab call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x317b4 => {
            // block 317b4
            #[cfg(feature = "trace")] eprintln!("f_00031780 pc 317b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 317b4 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 317b9 sub r15,rcx
            { let (r, f) = sub::<64>(r15, rcx);
              r15 = r;
              fl = f; }
            // 317bc movabs rax,0x3fffffffffffffff
            rax = 0x3fff_ffff_ffff_ffffu64;
            // 317c6 cmp r15,rax
            fl = sub::<64>(r15, rax).1;
            // 317c9 jae 0x317f7
            if crate::cov::br(fl.ae()) { pc = 0x317f7; continue; }
            pc = 0x317cb;
        }
        0x317cb => {
            // block 317cb
            #[cfg(feature = "trace")] eprintln!("f_00031780 pc 317cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 317cb mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 317cf add r15,r15
            { let (r, f) = add::<64>(r15, r15);
              r15 = r;
              fl = f; }
            // 317d2 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 317d5 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 317d9 mov r13,QWORD PTR [r12+0x8]
            r13 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 317de mov rbx,r13
            rbx = r13;
            // 317e1 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 317e4 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 317e8 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 317eb test r15,r15
            fl = logic::<64>(r15 & r15);
            // 317ee mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 317f3 jne 0x31814
            if crate::cov::br(fl.ne()) { pc = 0x31814; continue; }
            pc = 0x317f5;
        }
        0x317f5 => {
            // block 317f5
            #[cfg(feature = "trace")] eprintln!("f_00031780 pc 317f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 317f5 jmp 0x3181f
            pc = 0x3181f; continue;
        }
        0x317f7 => {
            // block 317f7
            #[cfg(feature = "trace")] eprintln!("f_00031780 pc 317f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 317f7 mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 317fb mov r13,QWORD PTR [r12+0x8]
            r13 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 31800 mov rbx,r13
            rbx = r13;
            // 31803 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 31806 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 3180a movabs r15,0x7fffffffffffffff
            r15 = 0x7fff_ffff_ffff_ffffu64;
            pc = 0x31814;
        }
        0x31814 => {
            // block 31814
            #[cfg(feature = "trace")] eprintln!("f_00031780 pc 31814 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31814 mov rdi,r15
            rdi = r15;
            // 31817 call 0xec146
            { m.outregs(0x3181c, 0xd, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3181c mov r14,r15
            r14 = r15;
            pc = 0x3181f;
        }
        0x3181f => {
            // block 3181f
            #[cfg(feature = "trace")] eprintln!("f_00031780 pc 3181f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3181f add r14,rax
            { let (r, f) = add::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // 31822 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 31826 mov cl,BYTE PTR [rcx]
            rcx = (rcx & !0xff) | ((m.r8(rcx) as u64) as u8 as u64);
            // 31828 mov BYTE PTR [rax+rbx*1],cl
            m.w8(rax.wrapping_add(rbx), ((rcx as u8 as u64)) as u8);
            // 3182b lea rcx,[rax+rbx*1+0x1]
            rcx = rax.wrapping_add(rbx).wrapping_add(0x1);
            // 31830 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // 31834 mov r15,QWORD PTR [rbp-0x38]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 31838 sub r13,r15
            { let (r, f) = sub::<64>(r13, r15);
              r13 = r;
              fl = f; }
            // 3183b sub rbx,r13
            { let (r, f) = sub::<64>(rbx, r13);
              rbx = r;
              fl = f; }
            // 3183e add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 31841 mov rdi,rbx
            rdi = rbx;
            // 31844 mov rsi,r15
            rsi = r15;
            // 31847 mov rdx,r13
            rdx = r13;
            // 3184a call 0xec21e
            { m.outregs(0x3184f, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3184f mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 31853 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 31857 mov QWORD PTR [r12+0x8],rax
            m.w64(r12.wrapping_add(0x8), (rax) as u64);
            // 3185c mov QWORD PTR [r12+0x10],r14
            m.w64(r12.wrapping_add(0x10), (r14) as u64);
            // 31861 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 31864 je 0x3187c
            if crate::cov::br(fl.e()) { pc = 0x3187c; continue; }
            pc = 0x31866;
        }
        0x31866 => {
            // block 31866
            #[cfg(feature = "trace")] eprintln!("f_00031780 pc 31866 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31866 mov rdi,r15
            rdi = r15;
            // 31869 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 3186d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3186e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 31870 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 31872 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 31874 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 31876 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 31877 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x3187c => {
            // block 3187c
            #[cfg(feature = "trace")] eprintln!("f_00031780 pc 3187c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3187c add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 31880 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 31881 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 31883 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 31885 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 31887 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 31889 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3188a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: unproven, tier P2v @0x651c2 size=274
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000651c2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x651c2;
    loop {
        match pc {
        0x651c2 => {
            // block 651c2
            #[cfg(feature = "trace")] eprintln!("f_000651c2 pc 651c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 651c2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 651c3 mov rbp,rsp
            rbp = rsp;
            // 651c6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 651c8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 651ca push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 651cc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 651ce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 651cf sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 651d3 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 651d7 mov r13,rdi
            r13 = rdi;
            // 651da mov rdi,QWORD PTR [r13+0x0]
            rdi = m.r64(r13) as u64;
            // 651de mov r15,QWORD PTR [r13+0x8]
            r15 = m.r64(r13.wrapping_add(0x8)) as u64;
            // 651e2 sub r15,rdi
            { let (r, f) = sub::<64>(r15, rdi);
              r15 = r;
              fl = f; }
            // 651e5 cmp r15,0xfffffffffffffffd
            fl = sub::<64>(r15, 0xffff_ffff_ffff_fffdu64).1;
            // 651e9 jg 0x651f7
            if crate::cov::br(fl.g()) { pc = 0x651f7; continue; }
            pc = 0x651eb;
        }
        0x651eb => {
            // block 651eb
            #[cfg(feature = "trace")] eprintln!("f_000651c2 pc 651eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 651eb mov rdi,r13
            rdi = r13;
            // 651ee call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x651f7 => {
            // block 651f7
            #[cfg(feature = "trace")] eprintln!("f_000651c2 pc 651f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 651f7 lea rax,[r13+0x10]
            rax = r13.wrapping_add(0x10);
            // 651fb mov rcx,QWORD PTR [r13+0x10]
            rcx = m.r64(r13.wrapping_add(0x10)) as u64;
            // 651ff mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 65203 sub rcx,rdi
            { let (r, f) = sub::<64>(rcx, rdi);
              rcx = r;
              fl = f; }
            // 65206 mov rdx,rcx
            rdx = rcx;
            // 65209 sar rdx,1
            { let (r, f) = sar::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 6520c movabs rsi,0x3ffffffffffffffe
            rsi = 0x3fff_ffff_ffff_fffeu64;
            // 65216 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 65219 ja 0x6522a
            if crate::cov::br(fl.a()) { pc = 0x6522a; continue; }
            pc = 0x6521b;
        }
        0x6521b => {
            // block 6521b
            #[cfg(feature = "trace")] eprintln!("f_000651c2 pc 6521b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6521b sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 6521e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 65221 cmp rcx,r15
            fl = sub::<64>(rcx, r15).1;
            // 65224 cmovae r15,rcx
            if fl.ae() { r15 = rcx; }
            // 65228 jmp 0x65234
            pc = 0x65234; continue;
        }
        0x6522a => {
            // block 6522a
            #[cfg(feature = "trace")] eprintln!("f_000651c2 pc 6522a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6522a movabs r15,0x7fffffffffffffff
            r15 = 0x7fff_ffff_ffff_ffffu64;
            pc = 0x65234;
        }
        0x65234 => {
            // block 65234
            #[cfg(feature = "trace")] eprintln!("f_000651c2 pc 65234 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65234 mov r14,QWORD PTR [r13+0x8]
            r14 = m.r64(r13.wrapping_add(0x8)) as u64;
            // 65238 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 6523c sub r14,rdi
            { let (r, f) = sub::<64>(r14, rdi);
              r14 = r;
              fl = f; }
            // 6523f mov QWORD PTR [rbp-0x58],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rdi) as u64);
            // 65243 mov r12,r14
            r12 = r14;
            // 65246 sar r12,1
            { let (r, f) = sar::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // 65249 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 65251 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 65255 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 65257 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 6525a je 0x65268
            if crate::cov::br(fl.e()) { pc = 0x65268; continue; }
            pc = 0x6525c;
        }
        0x6525c => {
            // block 6525c
            #[cfg(feature = "trace")] eprintln!("f_000651c2 pc 6525c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6525c lea rdi,[r15+r15*1]
            rdi = r15.wrapping_add(r15);
            // 65260 call 0xec146
            { m.outregs(0x65265, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65265 mov rbx,rax
            rbx = rax;
            pc = 0x65268;
        }
        0x65268 => {
            // block 65268
            #[cfg(feature = "trace")] eprintln!("f_000651c2 pc 65268 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65268 lea rax,[rbx+r15*2]
            rax = rbx.wrapping_add(r15.wrapping_mul(2));
            // 6526c mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 65270 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 65274 mov ax,WORD PTR [rax]
            rax = (rax & !0xffff) | ((m.r16(rax) as u64) as u16 as u64);
            // 65277 mov WORD PTR [rbx+r12*2],ax
            m.w16(rbx.wrapping_add(r12.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // 6527c lea r12,[rbx+r12*2+0x2]
            r12 = rbx.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0x2);
            // 65281 mov rdi,rbx
            rdi = rbx;
            // 65284 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 65288 mov rsi,r15
            rsi = r15;
            // 6528b mov rdx,r14
            rdx = r14;
            // 6528e call 0xec21e
            { m.outregs(0x65293, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65293 mov QWORD PTR [r13+0x0],rbx
            m.w64(r13, (rbx) as u64);
            // 65297 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 6529b mov QWORD PTR [r13+0x8],r12
            m.w64(r13.wrapping_add(0x8), (r12) as u64);
            // 6529f mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 652a3 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 652a7 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 652ab mov QWORD PTR [r13+0x10],rax
            m.w64(r13.wrapping_add(0x10), (rax) as u64);
            // 652af mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 652b3 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 652b7 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 652bb lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 652bf call 0xebb82
            { m.outregs(0x652c4, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x34f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 652c4 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 652c8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 652c9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 652cb pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 652cd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 652cf pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 652d1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 652d2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea2a2 size=346
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000ea2a2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xea2a2;
    loop {
        match pc {
        0xea2a2 => {
            // block ea2a2
            #[cfg(feature = "trace")] eprintln!("f_000ea2a2 pc ea2a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea2a2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea2a3 mov rbp,rsp
            rbp = rsp;
            // ea2a6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ea2a8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea2aa push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ea2ac push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ea2ae push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea2af sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // ea2b3 mov r15,rsi
            r15 = rsi;
            // ea2b6 mov r14,rdi
            r14 = rdi;
            // ea2b9 mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // ea2bd mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // ea2c1 mov rcx,rax
            rcx = rax;
            // ea2c4 sub rcx,rbx
            { let (r, f) = sub::<64>(rcx, rbx);
              rcx = r;
              fl = f; }
            // ea2c7 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // ea2cb movabs r13,0xaaaaaaaaaaaaaaab
            r13 = 0xaaaa_aaaa_aaaa_aaabu64;
            // ea2d5 imul rcx,r13
            { let (r, f) = imul::<64>(rcx, r13); rcx = r; fl = f; }
            // ea2d9 cmp rcx,r15
            fl = sub::<64>(rcx, r15).1;
            // ea2dc jae 0xea3c4
            if crate::cov::br(fl.ae()) { pc = 0xea3c4; continue; }
            pc = 0xea2e2;
        }
        0xea2e2 => {
            // block ea2e2
            #[cfg(feature = "trace")] eprintln!("f_000ea2a2 pc ea2e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea2e2 movabs r12,0xaaaaaaaaaaaaaaa
            r12 = 0xaaa_aaaa_aaaa_aaaau64;
            // ea2ec mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // ea2ef sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // ea2f2 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // ea2f6 imul rbx,r13
            { let (r, f) = imul::<64>(rbx, r13); rbx = r; fl = f; }
            // ea2fa add rbx,r15
            { let (r, f) = add::<64>(rbx, r15);
              rbx = r;
              fl = f; }
            // ea2fd cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // ea300 jbe 0xea311
            if crate::cov::br(fl.be()) { pc = 0xea311; continue; }
            pc = 0xea302;
        }
        0xea302 => {
            // block ea302
            #[cfg(feature = "trace")] eprintln!("f_000ea2a2 pc ea302 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea302 mov rdi,r14
            rdi = r14;
            // ea305 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0xea311 => {
            // block ea311
            #[cfg(feature = "trace")] eprintln!("f_000ea2a2 pc ea311 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea311 lea rdx,[r14+0x10]
            rdx = r14.wrapping_add(0x10);
            // ea315 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // ea318 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // ea31c imul rax,r13
            { let (r, f) = imul::<64>(rax, r13); rax = r; fl = f; }
            // ea320 movabs rsi,0x555555555555554
            rsi = 0x555_5555_5555_5554u64;
            // ea32a cmp rax,rsi
            fl = sub::<64>(rax, rsi).1;
            // ea32d ja 0xea33c
            if crate::cov::br(fl.a()) { pc = 0xea33c; continue; }
            pc = 0xea32f;
        }
        0xea32f => {
            // block ea32f
            #[cfg(feature = "trace")] eprintln!("f_000ea2a2 pc ea32f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea32f add rax,rax
            { let (r, f) = add::<64>(rax, rax);
              rax = r;
              fl = f; }
            // ea332 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // ea335 cmovb rax,rbx
            if fl.b() { rax = rbx; }
            // ea339 mov r12,rax
            r12 = rax;
            pc = 0xea33c;
        }
        0xea33c => {
            // block ea33c
            #[cfg(feature = "trace")] eprintln!("f_000ea2a2 pc ea33c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea33c mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // ea340 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // ea343 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // ea347 imul rbx,r13
            { let (r, f) = imul::<64>(rbx, r13); rbx = r; fl = f; }
            // ea34b mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // ea353 mov QWORD PTR [rbp-0x30],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdx) as u64);
            // ea357 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // ea359 test r12,r12
            fl = logic::<64>(r12 & r12);
            // ea35c je 0xea36f
            if crate::cov::br(fl.e()) { pc = 0xea36f; continue; }
            pc = 0xea35e;
        }
        0xea35e => {
            // block ea35e
            #[cfg(feature = "trace")] eprintln!("f_000ea2a2 pc ea35e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea35e lea rax,[r12*8+0x0]
            rax = r12.wrapping_mul(8);
            // ea366 lea rdi,[rax+rax*2]
            rdi = rax.wrapping_add(rax.wrapping_mul(2));
            // ea36a call 0xec146
            { m.outregs(0xea36f, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xea36f;
        }
        0xea36f => {
            // block ea36f
            #[cfg(feature = "trace")] eprintln!("f_000ea2a2 pc ea36f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea36f mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // ea373 lea rcx,[rbx+rbx*2]
            rcx = rbx.wrapping_add(rbx.wrapping_mul(2));
            // ea377 lea rdi,[rax+rcx*8]
            rdi = rax.wrapping_add(rcx.wrapping_mul(8));
            // ea37b mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // ea37f lea rcx,[r12+r12*2]
            rcx = r12.wrapping_add(r12.wrapping_mul(2));
            // ea383 lea rcx,[rax+rcx*8]
            rcx = rax.wrapping_add(rcx.wrapping_mul(8));
            // ea387 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // ea38b lea rcx,[r15*8+0x0]
            rcx = r15.wrapping_mul(8);
            // ea393 lea rsi,[rcx+rcx*2]
            rsi = rcx.wrapping_add(rcx.wrapping_mul(2));
            // ea397 add rbx,r15
            { let (r, f) = add::<64>(rbx, r15);
              rbx = r;
              fl = f; }
            // ea39a lea rcx,[rbx+rbx*2]
            rcx = rbx.wrapping_add(rbx.wrapping_mul(2));
            // ea39e lea rbx,[rax+rcx*8]
            rbx = rax.wrapping_add(rcx.wrapping_mul(8));
            // ea3a2 call 0xec188
            { m.outregs(0xea3a7, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea3a7 mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // ea3ab lea rbx,[rbp-0x50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // ea3af mov rdi,r14
            rdi = r14;
            // ea3b2 mov rsi,rbx
            rsi = rbx;
            // ea3b5 call 0xebe94
            { m.outregs(0xea3ba, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x661); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea3ba mov rdi,rbx
            rdi = rbx;
            // ea3bd call 0xebef4
            { m.outregs(0xea3c2, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x6c1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea3c2 jmp 0xea3ec
            pc = 0xea3ec; continue;
        }
        0xea3c4 => {
            // block ea3c4
            #[cfg(feature = "trace")] eprintln!("f_000ea2a2 pc ea3c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea3c4 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // ea3cc mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // ea3d4 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // ea3db mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // ea3df add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // ea3e3 mov QWORD PTR [r14+0x8],rbx
            m.w64(r14.wrapping_add(0x8), (rbx) as u64);
            // ea3e7 dec r15
            { let (r, f) = dec::<64>(r15, fl); r15 = r; fl = f; }
            // ea3ea jne 0xea3c4
            if crate::cov::br(fl.ne()) { pc = 0xea3c4; continue; }
            pc = 0xea3ec;
        }
        0xea3ec => {
            // block ea3ec
            #[cfg(feature = "trace")] eprintln!("f_000ea2a2 pc ea3ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea3ec add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // ea3f0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ea3f1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ea3f3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ea3f5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ea3f7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ea3f9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea3fa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
