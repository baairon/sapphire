//! Mechanically lifted functions (scripts/lift_run.py), module p001. Each one is checked against the original
//! in the world harness (tests/world_p001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xbc0fc size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc0fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbc0fc;
    loop {
        match pc {
        0xbc0fc => {
            // block bc0fc
            #[cfg(feature = "trace")] eprintln!("f_000bc0fc pc bc0fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc0fd mov rbp,rsp
            rbp = rsp;
            // bc100 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc101 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bc102 mov rbx,rsi
            rbx = rsi;
            // bc105 mov DWORD PTR [rbp-0xc],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rdi as u32 as u64)) as u32);
            // bc108 lea rdi,[rip+0x10c561] # 0x1c8670
            rdi = m.env.data_va(904765);
            // bc10f lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // bc113 call 0xebe40
            { let r = { let t = m.env.stub_va(0x60d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bc118 mov ecx,0x6c
            rcx = (0x6cu64) as u32 as u64;
            // bc11d lea rdx,[rip+0x10c554] # 0x1c8678
            rdx = m.env.data_va(904773);
            // bc124 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // bc127 je 0xbc12c
            if crate::cov::brk(0xbc127, fl.e()) { pc = 0xbc12c; continue; }
            pc = 0xbc129;
        }
        0xbc129 => {
            // block bc129
            #[cfg(feature = "trace")] eprintln!("f_000bc0fc pc bc129 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc129 mov ecx,DWORD PTR [rax+0x20]
            rcx = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            pc = 0xbc12c;
        }
        0xbc12c => {
            // block bc12c
            #[cfg(feature = "trace")] eprintln!("f_000bc0fc pc bc12c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc12c mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            // bc12e cmp ecx,0x6c
            fl = sub::<32>((rcx as u32 as u64), 0x6cu64).1;
            // bc131 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc134 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bc138 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc139 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc13a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc42d8 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000c42d8_rec)
pub fn f_000c42d8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000c42d8_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000c42d8_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
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
    let mut pc: u32 = 0xc42d8;
    loop {
        match pc {
        0xc42d8 => {
            // block c42d8
            #[cfg(feature = "trace")] eprintln!("f_000c42d8 pc c42d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            if crate::cov::brk(0xc42e8, fl.e()) { pc = 0xc430d; continue; }
            pc = 0xc42ea;
        }
        0xc42ea => {
            // block c42ea
            #[cfg(feature = "trace")] eprintln!("f_000c42d8 pc c42ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c42ea mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // c42ed mov rdi,r14
            rdi = r14;
            // c42f0 call 0xc42d8
            { rsp = rsp.wrapping_sub(8); m.w64(rsp, 0x100c42f5); let r = f_000c42d8_rec(m, [rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15], [x0, 0, 0, 0, 0, 0, 0, 0]); rsp = rsp.wrapping_add(8); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c42f5 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c42f9 mov rdi,r14
            rdi = r14;
            // c42fc call 0xc42d8
            { rsp = rsp.wrapping_sub(8); m.w64(rsp, 0x100c4301); let r = f_000c42d8_rec(m, [rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15], [x0, 0, 0, 0, 0, 0, 0, 0]); rsp = rsp.wrapping_add(8); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            #[cfg(feature = "trace")] eprintln!("f_000c42d8 pc c430d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xe2fe4 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2fe4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2fe4;
    loop {
        match pc {
        0xe2fe4 => {
            // block e2fe4
            #[cfg(feature = "trace")] eprintln!("f_000e2fe4 pc e2fe4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2fe4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2fe5 mov rbp,rsp
            rbp = rsp;
            // e2fe8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2fe9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e2fea mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e2fec mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e2fef mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2ff2 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // e2ff6 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e2ffb mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e3000 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e3003 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e3006 mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // e3009 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e300b cmp ebx,0x1
            fl = sub::<32>((rbx as u32 as u64), 0x1u64).1;
            // e300e jne 0xe301a
            if crate::cov::brk(0xe300e, fl.ne()) { pc = 0xe301a; continue; }
            pc = 0xe3010;
        }
        0xe3010 => {
            // block e3010
            #[cfg(feature = "trace")] eprintln!("f_000e2fe4 pc e3010 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3010 test ecx,0x40000000
            fl = logic::<32>((rcx as u32 as u64) & 0x40000000u64);
            // e3016 je 0xe301e
            if crate::cov::brk(0xe3016, fl.e()) { pc = 0xe301e; continue; }
            pc = 0xe3018;
        }
        0xe3018 => {
            // block e3018
            #[cfg(feature = "trace")] eprintln!("f_000e2fe4 pc e3018 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3018 jmp 0xe3020
            pc = 0xe3020; continue;
        }
        0xe301a => {
            // block e301a
            #[cfg(feature = "trace")] eprintln!("f_000e2fe4 pc e301a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e301a test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // e301c js 0xe3020
            if crate::cov::brk(0xe301c, fl.s()) { pc = 0xe3020; continue; }
            pc = 0xe301e;
        }
        0xe301e => {
            // block e301e
            #[cfg(feature = "trace")] eprintln!("f_000e2fe4 pc e301e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e301e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe3020;
        }
        0xe3020 => {
            // block e3020
            #[cfg(feature = "trace")] eprintln!("f_000e2fe4 pc e3020 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3020 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e3024 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e3025 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e3026 ret
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            if crate::cov::brk(0xe0da5, fl.e()) { pc = 0xe0dbd; continue; }
            pc = 0xe0da7;
        }
        0xe0da7 => {
            // block e0da7
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0da7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0da7 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // e0daa jne 0xe0db0
            if crate::cov::brk(0xe0daa, fl.ne()) { pc = 0xe0db0; continue; }
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

/// parity: P2v @0x56ae size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000056ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x56ae;
    loop {
        match pc {
        0x56ae => {
            // block 56ae
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 56af mov rbp,rsp
            rbp = rsp;
            // 56b2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 56b4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 56b5 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 56b9 mov r14,rdx
            r14 = rdx;
            // 56bc mov rbx,rsi
            rbx = rsi;
            // 56bf mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // 56c3 mov BYTE PTR [rbp-0x12],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffee), (0x0u64) as u8);
            // 56c7 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 56ca je 0x56f5
            if crate::cov::brk(0x56ca, fl.e()) { pc = 0x56f5; continue; }
            pc = 0x56cc;
        }
        0x56cc => {
            // block 56cc
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56cc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 56cf lea rsi,[rbp-0x11]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffef);
            // 56d3 lea rdx,[rbp-0x12]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffee);
            // 56d7 call QWORD PTR [rax+0x5d8]
            { let r = { let t = m.r64(rax.wrapping_add(0x5d8)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 56dd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 56df je 0x56f5
            if crate::cov::brk(0x56df, fl.e()) { pc = 0x56f5; continue; }
            pc = 0x56e1;
        }
        0x56e1 => {
            // block 56e1
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56e1 mov al,BYTE PTR [rbp-0x11]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u8 as u64);
            // 56e4 mov BYTE PTR [rbx],al
            m.w8(rbx, ((rax as u8 as u64)) as u8);
            // 56e6 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 56e8 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 56eb je 0x56f7
            if crate::cov::brk(0x56eb, fl.e()) { pc = 0x56f7; continue; }
            pc = 0x56ed;
        }
        0x56ed => {
            // block 56ed
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56ed mov cl,BYTE PTR [rbp-0x12]
            rcx = (rcx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffee)) as u64) as u8 as u64);
            // 56f0 mov BYTE PTR [r14],cl
            m.w8(r14, ((rcx as u8 as u64)) as u8);
            // 56f3 jmp 0x56f7
            pc = 0x56f7; continue;
        }
        0x56f5 => {
            // block 56f5
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x56f7;
        }
        0x56f7 => {
            // block 56f7
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 56fb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 56fc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 56fe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 56ff ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8ab2 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008ab2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8ab2;
    loop {
        match pc {
        0x8ab2 => {
            // block 8ab2
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8ab2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8ab2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8ab3 mov rbp,rsp
            rbp = rsp;
            // 8ab6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8ab7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 8ab8 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 8abb lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 8abf xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8ac1 mov esi,0x51
            rsi = (0x51u64) as u32 as u64;
            // 8ac6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8acb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8ace call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8ad1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8ad3 je 0x8b03
            if crate::cov::brk(0x8ad3, fl.e()) { pc = 0x8b03; continue; }
            pc = 0x8ad5;
        }
        0x8ad5 => {
            // block 8ad5
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8ad5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8ad5 mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 8ad8 test ecx,0x10000
            fl = logic::<32>((rcx as u32 as u64) & 0x10000u64);
            // 8ade je 0x8b03
            if crate::cov::brk(0x8ade, fl.e()) { pc = 0x8b03; continue; }
            pc = 0x8ae0;
        }
        0x8ae0 => {
            // block 8ae0
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8ae0 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 8ae2 and eax,0x8000
            { let r = (rax as u32 as u64) & 0x8000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8ae7 test ecx,0x40000
            fl = logic::<32>((rcx as u32 as u64) & 0x40000u64);
            // 8aed jne 0x8af9
            if crate::cov::brk(0x8aed, fl.ne()) { pc = 0x8af9; continue; }
            pc = 0x8aef;
        }
        0x8aef => {
            // block 8aef
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8aef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8aef shr eax,0xa
            { let (r, f) = shr::<32>((rax as u32 as u64), 0xau64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8af2 xor eax,0x20
            { let r = (rax as u32 as u64) ^ 0x20u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8af5 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 8af7 jmp 0x8b03
            pc = 0x8b03; continue;
        }
        0x8af9 => {
            // block 8af9
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8af9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8af9 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 8afb mov ebx,0x21
            rbx = (0x21u64) as u32 as u64;
            // 8b00 cmove ebx,eax
            if fl.e() { rbx = ((rax as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            pc = 0x8b03;
        }
        0x8b03 => {
            // block 8b03
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8b03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8b03 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 8b05 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 8b09 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8b0a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8b0b ret
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
            if crate::cov::brk(0x44b69, fl.e()) { pc = 0x44b9c; continue; }
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
            if crate::cov::brk(0x44b77, fl.ge()) { pc = 0x44b83; continue; }
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
            if crate::cov::brk(0x44b7f, fl.ne()) { pc = 0x44b6f; continue; }
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
            if crate::cov::brk(0x44b85, fl.ge()) { pc = 0x44bb0; continue; }
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
            if crate::cov::brk(0x44b8e, fl.ne()) { pc = 0x44b6f; continue; }
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

/// parity: P2v @0xc5118 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c5118<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc5118;
    loop {
        match pc {
        0xc5118 => {
            // block c5118
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c5118 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5118 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c5119 mov rbp,rsp
            rbp = rsp;
            // c511c mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c5120 test rax,rax
            fl = logic::<64>(rax & rax);
            // c5123 je 0xc5156
            if crate::cov::brk(0xc5123, fl.e()) { pc = 0xc5156; continue; }
            pc = 0xc5125;
        }
        0xc5125 => {
            // block c5125
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c5125 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5125 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c5127 jmp 0xc512c
            pc = 0xc512c; continue;
        }
        0xc5129 => {
            // block c5129
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c5129 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5129 mov rax,rdx
            rax = rdx;
            pc = 0xc512c;
        }
        0xc512c => {
            // block c512c
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c512c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c512c mov edx,DWORD PTR [rax+0x20]
            rdx = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            // c512f cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c5131 jae 0xc513d
            if crate::cov::brk(0xc5131, fl.ae()) { pc = 0xc513d; continue; }
            pc = 0xc5133;
        }
        0xc5133 => {
            // block c5133
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c5133 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5133 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c5136 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c5139 jne 0xc5129
            if crate::cov::brk(0xc5139, fl.ne()) { pc = 0xc5129; continue; }
            pc = 0xc513b;
        }
        0xc513b => {
            // block c513b
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c513b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c513b jmp 0xc5162
            pc = 0xc5162; continue;
        }
        0xc513d => {
            // block c513d
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c513d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c513d cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c513f jae 0xc516a
            if crate::cov::brk(0xc513f, fl.ae()) { pc = 0xc516a; continue; }
            pc = 0xc5141;
        }
        0xc5141 => {
            // block c5141
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c5141 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5141 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // c5145 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c5148 jne 0xc5129
            if crate::cov::brk(0xc5148, fl.ne()) { pc = 0xc5129; continue; }
            pc = 0xc514a;
        }
        0xc514a => {
            // block c514a
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c514a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c514a mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c514d add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c5151 mov rsi,rax
            rsi = rax;
            // c5154 jmp 0xc516d
            pc = 0xc516d; continue;
        }
        0xc5156 => {
            // block c5156
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c5156 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5156 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c515a mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c515d mov rsi,rdi
            rsi = rdi;
            // c5160 jmp 0xc516d
            pc = 0xc516d; continue;
        }
        0xc5162 => {
            // block c5162
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c5162 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5162 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c5165 mov rsi,rax
            rsi = rax;
            // c5168 jmp 0xc516d
            pc = 0xc516d; continue;
        }
        0xc516a => {
            // block c516a
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c516a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c516a mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc516d;
        }
        0xc516d => {
            // block c516d
            #[cfg(feature = "trace")] eprintln!("f_000c5118 pc c516d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c516d mov rax,rsi
            rax = rsi;
            // c5170 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c5171 ret
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
            if crate::cov::brk(0x2dca7, fl.e()) { pc = 0x2dcda; continue; }
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
            if crate::cov::brk(0x2dcb5, fl.ae()) { pc = 0x2dcc1; continue; }
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
            if crate::cov::brk(0x2dcbd, fl.ne()) { pc = 0x2dcad; continue; }
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
            if crate::cov::brk(0x2dcc3, fl.ae()) { pc = 0x2dcee; continue; }
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
            if crate::cov::brk(0x2dccc, fl.ne()) { pc = 0x2dcad; continue; }
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

/// parity: P2v @0xc4e4e size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4e4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4e4e;
    loop {
        match pc {
        0xc4e4e => {
            // block c4e4e
            #[cfg(feature = "trace")] eprintln!("f_000c4e4e pc c4e4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4e4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4e4f mov rbp,rsp
            rbp = rsp;
            // c4e52 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c4e54 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4e56 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c4e58 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4e59 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c4e5d mov r15,rdx
            r15 = rdx;
            // c4e60 mov r14,rdi
            r14 = rdi;
            // c4e63 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c4e67 mov rcx,r15
            rcx = r15;
            // c4e6a call 0xebd86
            { let r = { let t = m.env.stub_va(0x553); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c4e6f mov r12,rax
            r12 = rax;
            // c4e72 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // c4e76 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c4e79 jne 0xc4ea0
            if crate::cov::brk(0xc4e79, fl.ne()) { pc = 0xc4ea0; continue; }
            pc = 0xc4e7b;
        }
        0xc4e7b => {
            // block c4e7b
            #[cfg(feature = "trace")] eprintln!("f_000c4e4e pc c4e7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4e7b mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // c4e80 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c4e85 mov rbx,rax
            rbx = rax;
            // c4e88 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // c4e8b mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c4e8e mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c4e92 mov rdi,r14
            rdi = r14;
            // c4e95 mov rdx,r12
            rdx = r12;
            // c4e98 mov rcx,rbx
            rcx = rbx;
            // c4e9b call 0xebd9e
            { let r = { let t = m.env.stub_va(0x56b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xc4ea0;
        }
        0xc4ea0 => {
            // block c4ea0
            #[cfg(feature = "trace")] eprintln!("f_000c4e4e pc c4ea0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ea0 mov rax,rbx
            rax = rbx;
            // c4ea3 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c4ea7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4ea8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c4eaa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4eac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c4eae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4eaf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4684 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4684<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4684;
    loop {
        match pc {
        0xe4684 => {
            // block e4684
            #[cfg(feature = "trace")] eprintln!("f_000e4684 pc e4684 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4684 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4685 mov rbp,rsp
            rbp = rsp;
            // e4688 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e468a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e468c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e468d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e468e mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e4691 mov rbx,rdi
            rbx = rdi;
            // e4694 call 0xe4a46
            { let r = { let t = m.env.code_va(0xe4a46); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e4699 mov QWORD PTR [rbx+0x2040],rax
            m.w64(rbx.wrapping_add(0x2040), (rax) as u64);
            // e46a0 mov r15d,r14d
            r15 = ((r14 as u32 as u64)) as u32 as u64;
            // e46a3 lea r14,[rbp-0x1c]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            pc = 0xe46a7;
        }
        0xe46a7 => {
            // block e46a7
            #[cfg(feature = "trace")] eprintln!("f_000e4684 pc e46a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e46a7 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e46aa mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e46ad mov esi,DWORD PTR [rbx+0x30]
            rsi = (m.r32(rbx.wrapping_add(0x30)) as u64) as u32 as u64;
            // e46b0 add esi,0x4
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x4u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e46b3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e46b8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e46bb mov rdx,r14
            rdx = r14;
            // e46be call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e46c1 mov ecx,DWORD PTR [rbp-0x1c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e46c4 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e46c6 test cl,0x2
            fl = logic::<8>((rcx as u8 as u64) & 0x2u64);
            // e46c9 je 0xe46e1
            if crate::cov::brk(0xe46c9, fl.e()) { pc = 0xe46e1; continue; }
            pc = 0xe46cb;
        }
        0xe46cb => {
            // block e46cb
            #[cfg(feature = "trace")] eprintln!("f_000e4684 pc e46cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e46cb mov rdi,rbx
            rdi = rbx;
            // e46ce call 0xe4a46
            { let r = { let t = m.env.code_va(0xe4a46); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e46d3 sub rax,QWORD PTR [rbx+0x2040]
            { let (r, f) = sub::<64>(rax, m.r64(rbx.wrapping_add(0x2040)) as u64);
              rax = r;
              fl = f; }
            // e46da cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // e46dd jbe 0xe46a7
            if crate::cov::brk(0xe46dd, fl.be()) { pc = 0xe46a7; continue; }
            pc = 0xe46df;
        }
        0xe46df => {
            // block e46df
            #[cfg(feature = "trace")] eprintln!("f_000e4684 pc e46df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e46df xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe46e1;
        }
        0xe46e1 => {
            // block e46e1
            #[cfg(feature = "trace")] eprintln!("f_000e4684 pc e46e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e46e1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e46e5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e46e6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e46e8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e46ea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e46eb ret
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
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            if crate::cov::brk(0xc5793, fl.ne()) { pc = 0xc579e; continue; }
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
            if crate::cov::brk(0xc57a1, fl.ge()) { pc = 0xc57ad; continue; }
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
            if crate::cov::brk(0xc57a9, fl.ne()) { pc = 0xc579b; continue; }
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
            if crate::cov::brk(0xc57b4, fl.ne()) { pc = 0xc579b; continue; }
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
            { let r = { let t = m.env.stub_va(0x58f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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

/// parity: P2v @0xd648e size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d648e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd648e;
    loop {
        match pc {
        0xd648e => {
            // block d648e
            #[cfg(feature = "trace")] eprintln!("f_000d648e pc d648e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d648e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d648f mov rbp,rsp
            rbp = rsp;
            // d6492 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d6494 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d6496 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d6498 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6499 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d649d mov r12,rsi
            r12 = rsi;
            // d64a0 mov r14,rdi
            r14 = rdi;
            // d64a3 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // d64a7 mov rdx,r12
            rdx = r12;
            // d64aa call 0xebeb8
            { let r = { let t = m.env.stub_va(0x685); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d64af mov r15,rax
            r15 = rax;
            // d64b2 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // d64b5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d64b8 jne 0xd64f8
            if crate::cov::brk(0xd64b8, fl.ne()) { pc = 0xd64f8; continue; }
            pc = 0xd64ba;
        }
        0xd64ba => {
            // block d64ba
            #[cfg(feature = "trace")] eprintln!("f_000d648e pc d64ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d64ba mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // d64bf call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d64c4 mov rbx,rax
            rbx = rax;
            // d64c7 mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // d64cb mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // d64ce mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // d64d6 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // d64de mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // d64e6 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // d64ea mov rdi,r14
            rdi = r14;
            // d64ed mov rdx,r15
            rdx = r15;
            // d64f0 mov rcx,rbx
            rcx = rbx;
            // d64f3 call 0xebec4
            { let r = { let t = m.env.stub_va(0x691); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd64f8;
        }
        0xd64f8 => {
            // block d64f8
            #[cfg(feature = "trace")] eprintln!("f_000d648e pc d64f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d64f8 add rbx,0x28
            { let (r, f) = add::<64>(rbx, 0x28u64);
              rbx = r;
              fl = f; }
            // d64fc mov rax,rbx
            rax = rbx;
            // d64ff add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d6503 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6504 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6506 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6508 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d650a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d650b ret
            return Ret { rax, rdx, xmm0: x0 };
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
            { let r = { let t = m.r64(rax.wrapping_add(0x70)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2e784 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e786 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2e789 je 0x2e7e3
            if crate::cov::brk(0x2e789, fl.e()) { pc = 0x2e7e3; continue; }
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
            if crate::cov::brk(0x2e795, fl.e()) { pc = 0x2e7e3; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2e7a0 mov rdi,r14
            rdi = r14;
            // 2e7a3 mov rsi,rax
            rsi = rax;
            // 2e7a6 call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2e7ab test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e7ad je 0x2e7e0
            if crate::cov::brk(0x2e7ad, fl.e()) { pc = 0x2e7e0; continue; }
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
            if crate::cov::brk(0x2e7b7, fl.ne()) { pc = 0x2e797; continue; }
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
            if crate::cov::brk(0x2e7c2, fl.e()) { pc = 0x2e7e3; continue; }
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
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2e7d3 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 2e7d5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e7d7 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 2e7d9 jne 0x2e7e3
            if crate::cov::brk(0x2e7d9, fl.ne()) { pc = 0x2e7e3; continue; }
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

/// parity: P2v @0x5a600 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a600<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5a600;
    loop {
        match pc {
        0x5a600 => {
            // block 5a600
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a600 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a600 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a601 mov rbp,rsp
            rbp = rsp;
            // 5a604 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5a606 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5a608 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5a609 movzx r9d,cl
            r9 = ((rcx as u8 as u64)) as u32 as u64;
            // 5a60d mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 5a613 xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // 5a616 lea r11,[rip+0xa6733] # 0x100d50
            r11 = m.env.data_va(87325);
            // 5a61d lea rbx,[rip+0xa691c] # 0x100f40
            rbx = m.env.data_va(87821);
            // 5a624 lea r14,[rip+0xa6b05] # 0x101130
            r14 = m.env.data_va(88317);
            // 5a62b lea r15,[rip+0xa6d5e] # 0x101390
            r15 = m.env.data_va(88925);
            pc = 0x5a632;
        }
        0x5a632 => {
            // block 5a632
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a632 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a632 cmp DWORD PTR [r10+r11*1],edi
            fl = sub::<32>(m.r32(r10.wrapping_add(r11)) as u64, (rdi as u32 as u64)).1;
            // 5a636 jne 0x5a669
            if crate::cov::brk(0x5a636, fl.ne()) { pc = 0x5a669; continue; }
            pc = 0x5a638;
        }
        0x5a638 => {
            // block 5a638
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a638 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a638 cmp esi,DWORD PTR [r10+rbx*1+0x4]
            fl = sub::<32>((rsi as u32 as u64), m.r32(r10.wrapping_add(rbx).wrapping_add(0x4)) as u64).1;
            // 5a63d jne 0x5a669
            if crate::cov::brk(0x5a63d, fl.ne()) { pc = 0x5a669; continue; }
            pc = 0x5a63f;
        }
        0x5a63f => {
            // block 5a63f
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a63f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a63f cmp edx,DWORD PTR [r10+r14*1]
            fl = sub::<32>((rdx as u32 as u64), m.r32(r10.wrapping_add(r14)) as u64).1;
            // 5a643 jne 0x5a669
            if crate::cov::brk(0x5a643, fl.ne()) { pc = 0x5a669; continue; }
            pc = 0x5a645;
        }
        0x5a645 => {
            // block 5a645
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a645 mov ecx,DWORD PTR [r10+r15*1]
            rcx = (m.r32(r10.wrapping_add(r15)) as u64) as u32 as u64;
            // 5a649 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5a64b cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 5a64e ja 0x5a65c
            if crate::cov::brk(0x5a64e, fl.a()) { pc = 0x5a65c; continue; }
            pc = 0x5a650;
        }
        0x5a650 => {
            // block 5a650
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a650 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a650 mov eax,0x7a9
            rax = (0x7a9u64) as u32 as u64;
            // 5a655 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5a657 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a65a jmp 0x5a65e
            pc = 0x5a65e; continue;
        }
        0x5a65c => {
            // block 5a65c
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a65c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a65c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5a65e;
        }
        0x5a65e => {
            // block 5a65e
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a65e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a65e movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 5a661 xor eax,0x1
            { let r = (rax as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a664 cmp r9d,eax
            fl = sub::<32>((r9 as u32 as u64), (rax as u32 as u64)).1;
            // 5a667 je 0x5a67a
            if crate::cov::brk(0x5a667, fl.e()) { pc = 0x5a67a; continue; }
            pc = 0x5a669;
        }
        0x5a669 => {
            // block 5a669
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a669 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a669 inc r8
            { let (r, f) = inc::<64>(r8, fl); r8 = r; fl = f; }
            // 5a66c add r10,0x4
            { let (r, f) = add::<64>(r10, 0x4u64);
              r10 = r;
              fl = f; }
            // 5a670 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a672 cmp r8,0x7a
            fl = sub::<64>(r8, 0x7au64).1;
            // 5a676 jl 0x5a632
            if crate::cov::brk(0x5a676, fl.l()) { pc = 0x5a632; continue; }
            pc = 0x5a678;
        }
        0x5a678 => {
            // block 5a678
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a678 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a678 jmp 0x5a67d
            pc = 0x5a67d; continue;
        }
        0x5a67a => {
            // block 5a67a
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a67a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a67a mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            pc = 0x5a67d;
        }
        0x5a67d => {
            // block 5a67d
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a67d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a67d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5a67e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5a680 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5a682 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a683 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15f84 size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
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
            if crate::cov::brk(0x15fa5, fl.be()) { pc = 0x15fb1; continue; }
            pc = 0x15fa7;
        }
        0x15fa7 => {
            // block 15fa7
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fa7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fa7 mov rdi,r15
            rdi = r15;
            // 15faa call 0xebfc0
            { let r = { let t = m.env.stub_va(0x78d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 15faf jmp 0x15fc7
            pc = 0x15fc7; continue;
        }
        0x15fb1 => {
            // block 15fb1
            #[cfg(feature = "trace")] eprintln!("f_00015f84 pc 15fb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15fb1 cmp r13,0x16
            fl = sub::<64>(r13, 0x16u64).1;
            // 15fb5 ja 0x15fc7
            if crate::cov::brk(0x15fb5, fl.a()) { pc = 0x15fc7; continue; }
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
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            if crate::cov::brk(0x15fe9, fl.e()) { pc = 0x16000; continue; }
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
            if crate::cov::brk(0x15ffb, fl.ne()) { pc = 0x15fee; continue; }
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

/// parity: P2v @0x595ae size=149
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000595ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x595ae;
    loop {
        match pc {
        0x595ae => {
            // block 595ae
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 595ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 595ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 595af mov rbp,rsp
            rbp = rsp;
            // 595b2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 595b4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 595b6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 595b8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 595ba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 595bb sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 595bf mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // 595c2 mov DWORD PTR [rbp-0x6c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rdx as u32 as u64)) as u32);
            // 595c5 mov rbx,rdi
            rbx = rdi;
            // 595c8 movzx eax,BYTE PTR [rbp+0x10]
            rax = (m.r8(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 595cc movzx ecx,r8b
            rcx = ((r8 as u8 as u64)) as u32 as u64;
            // 595d0 movzx r8d,r9b
            r8 = ((r9 as u8 as u64)) as u32 as u64;
            // 595d4 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 595d8 mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // 595db call 0x32c5c
            { let r = { let t = m.env.code_va(0x32c5c); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 595e0 cmp DWORD PTR [rbp-0x68],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, 0x0u64).1;
            // 595e4 je 0x59634
            if crate::cov::brk(0x595e4, fl.e()) { pc = 0x59634; continue; }
            pc = 0x595e6;
        }
        0x595e6 => {
            // block 595e6
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 595e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 595e6 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 595e9 and eax,0xffffff
            { let r = (rax as u32 as u64) & 0xffffffu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 595ee mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 595f0 shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 595f3 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 595f6 mov r14w,0x1
            r14 = (r14 & !0xffff) | ((0x1u64) as u16 as u64);
            // 595fb movzx r12d,cl
            r12 = ((rcx as u8 as u64)) as u32 as u64;
            // 595ff movzx r15d,r15b
            r15 = ((r15 as u8 as u64)) as u32 as u64;
            // 59603 movzx r13d,al
            r13 = ((rax as u8 as u64)) as u32 as u64;
            pc = 0x59607;
        }
        0x59607 => {
            // block 59607
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 59607 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59607 mov r8d,DWORD PTR [rbp-0x64]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 5960b mov rdi,rbx
            rdi = rbx;
            // 5960e mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 59611 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 59614 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // 59617 mov r9d,DWORD PTR [rbp-0x6c]
            r9 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // 5961b call 0x59541
            { let r = { let t = m.env.code_va(0x59541); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 59620 mov eax,DWORD PTR [rbp-0x60]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 59623 lea rbx,[rbx+rax*4]
            rbx = rbx.wrapping_add(rax.wrapping_mul(4));
            // 59627 movzx eax,r14w
            rax = ((r14 as u16 as u64)) as u32 as u64;
            // 5962b lea r14d,[r14+0x1]
            r14 = (r14.wrapping_add(0x1)) as u32 as u64;
            // 5962f cmp eax,DWORD PTR [rbp-0x68]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 59632 jb 0x59607
            if crate::cov::brk(0x59632, fl.b()) { pc = 0x59607; continue; }
            pc = 0x59634;
        }
        0x59634 => {
            // block 59634
            #[cfg(feature = "trace")] eprintln!("f_000595ae pc 59634 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59634 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 59638 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 59639 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5963b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5963d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5963f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 59641 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 59642 ret
            return Ret { rax, rdx, xmm0: x0 };
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
            if crate::cov::brk(0xc626a, fl.le()) { pc = 0xc62c5; continue; }
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
            { let r = { let t = r12; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // c6285 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // c6289 jne 0xc62ac
            if crate::cov::brk(0xc6289, fl.ne()) { pc = 0xc62ac; continue; }
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
            if crate::cov::brk(0xc62c3, fl.ne()) { pc = 0xc627b; continue; }
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

/// parity: P2v @0x89c8 size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000089c8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x89c8;
    loop {
        match pc {
        0x89c8 => {
            // block 89c8
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 89c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 89c8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 89c9 mov rbp,rsp
            rbp = rsp;
            // 89cc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 89ce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 89cf sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 89d3 mov rbx,rdi
            rbx = rdi;
            // 89d6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 89d9 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 89dd call QWORD PTR [rax+0x12b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x12b0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 89e3 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 89e6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 89e8 je 0x8a58
            if crate::cov::brk(0x89e8, fl.e()) { pc = 0x8a58; continue; }
            pc = 0x89ea;
        }
        0x89ea => {
            // block 89ea
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 89ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 89ea test BYTE PTR [rbp-0x14],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64 & 0x1u64);
            // 89ee je 0x8a58
            if crate::cov::brk(0x89ee, fl.e()) { pc = 0x8a58; continue; }
            pc = 0x89f0;
        }
        0x89f0 => {
            // block 89f0
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 89f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 89f0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 89f3 mov rdi,rbx
            rdi = rbx;
            // 89f6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 89fc mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 89fe call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8a03 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 8a06 jne 0x8a27
            if crate::cov::brk(0x8a06, fl.ne()) { pc = 0x8a27; continue; }
            pc = 0x8a08;
        }
        0x8a08 => {
            // block 8a08
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a08 mov edi,DWORD PTR [rbp-0x14]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 8a0b mov esi,edi
            rsi = ((rdi as u32 as u64)) as u32 as u64;
            // 8a0d shr esi,0x18
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x18u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 8a10 and esi,0x7
            { let r = (rsi as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8a13 mov r14d,0x19
            r14 = (0x19u64) as u32 as u64;
            // 8a19 cmp esi,0x5
            fl = sub::<32>((rsi as u32 as u64), 0x5u64).1;
            // 8a1c je 0x8a58
            if crate::cov::brk(0x8a1c, fl.e()) { pc = 0x8a58; continue; }
            pc = 0x8a1e;
        }
        0x8a1e => {
            // block 8a1e
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a1e shr edi,0x1c
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x1cu64, fl); rdi = (r) as u32 as u64; fl = f; }
            // 8a21 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 8a23 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 8a25 jmp 0x8a4d
            pc = 0x8a4d; continue;
        }
        0x8a27 => {
            // block 8a27
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a27 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 8a2a jb 0x8a58
            if crate::cov::brk(0x8a2a, fl.b()) { pc = 0x8a58; continue; }
            pc = 0x8a2c;
        }
        0x8a2c => {
            // block 8a2c
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a2c mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 8a2f mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 8a31 shr edi,0x1c
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x1cu64, fl); rdi = (r) as u32 as u64; fl = f; }
            // 8a34 shr eax,0x4
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8a37 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 8a39 and esi,0xf
            { let r = (rsi as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8a3c and eax,0xe
            { let r = (rax as u32 as u64) & 0xeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8a3f cmp eax,0x6
            fl = sub::<32>((rax as u32 as u64), 0x6u64).1;
            // 8a42 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 8a45 movzx ecx,al
            rcx = ((rax as u8 as u64)) as u32 as u64;
            // 8a48 shl ecx,0x3
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x3u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 8a4b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x8a4d;
        }
        0x8a4d => {
            // block 8a4d
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a4d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8a50 call 0x6d8
            { let r = { let t = m.env.code_va(0x6d8); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8a55 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x8a58;
        }
        0x8a58 => {
            // block 8a58
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a58 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 8a5b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8a5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8a60 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8a62 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8a63 ret
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
            if crate::cov::brk(0x112a4, fl.e()) { pc = 0x112c1; continue; }
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
            if crate::cov::brk(0x112ad, fl.e()) { pc = 0x112b8; continue; }
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
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            if crate::cov::brk(0x112bf, fl.ne()) { pc = 0x112a6; continue; }
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
            if crate::cov::brk(0x112c8, fl.e()) { pc = 0x112e8; continue; }
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
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 112d3 mov rdi,QWORD PTR [r15+0x40]
            rdi = m.r64(r15.wrapping_add(0x40)) as u64;
            // 112d7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 112da mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 112de call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11316 mov rdi,r14
            rdi = r14;
            // 11319 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29d86 size=164
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029d86<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x29d86;
    loop {
        match pc {
        0x29d86 => {
            // block 29d86
            #[cfg(feature = "trace")] eprintln!("f_00029d86 pc 29d86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d86 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29d87 mov rbp,rsp
            rbp = rsp;
            // 29d8a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29d8b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 29d8c mov rbx,rdi
            rbx = rdi;
            // 29d8f movabs rax,0x0
            rax = 0x0u64;
            // 29d99 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 29d9c mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 29da4 movabs rax,0x800000050
            rax = 0x0_0008_0000_0050u64;
            // 29dae mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 29db2 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 29db6 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 29dbe mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 29dc6 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 29dce mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 29dd6 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 29dde mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 29de6 mov esi,0xe0
            rsi = (0xe0u64) as u32 as u64;
            // 29deb call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29df0 mov rax,QWORD PTR [rbx+0x20]
            rax = m.r64(rbx.wrapping_add(0x20)) as u64;
            // 29df4 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // 29df6 test rax,rax
            fl = logic::<64>(rax & rax);
            // 29df9 je 0x29e0a
            if crate::cov::brk(0x29df9, fl.e()) { pc = 0x29e0a; continue; }
            pc = 0x29dfb;
        }
        0x29dfb => {
            // block 29dfb
            #[cfg(feature = "trace")] eprintln!("f_00029d86 pc 29dfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29dfb mov esi,DWORD PTR [rbx+0x28]
            rsi = (m.r32(rbx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 29dfe mov rdi,rax
            rdi = rax;
            // 29e01 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29e06 mov rdi,QWORD PTR [rbx+0x20]
            rdi = m.r64(rbx.wrapping_add(0x20)) as u64;
            pc = 0x29e0a;
        }
        0x29e0a => {
            // block 29e0a
            #[cfg(feature = "trace")] eprintln!("f_00029d86 pc 29e0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29e0a movabs rax,0x6e74763200000000
            rax = 0x6e74_7632_0000_0000u64;
            // 29e14 mov QWORD PTR [rbx+0x48],rax
            m.w64(rbx.wrapping_add(0x48), (rax) as u64);
            // 29e18 mov esi,0xe0
            rsi = (0xe0u64) as u32 as u64;
            // 29e1d call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 29e22 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 29e26 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29e27 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29e28 ret
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
            { let r = { let t = m.env.code_va(0xd0fb9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d63b movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d63e cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d640 jbe 0xd6b5
            if crate::cov::brk(0xd640, fl.be()) { pc = 0xd6b5; continue; }
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
            if crate::cov::brk(0xd652, fl.e()) { pc = 0xd662; continue; }
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
            if crate::cov::brk(0xd660, fl.ne()) { pc = 0xd6b1; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1820)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d677 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d679 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d67b cmp BYTE PTR [rbp-0x25],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb)) as u64, 0x0u64).1;
            // d67f je 0xd6b7
            if crate::cov::brk(0xd67f, fl.e()) { pc = 0xd6b7; continue; }
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
            if crate::cov::brk(0xd684, fl.ne()) { pc = 0xd6b7; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6a3 je 0xd6b5
            if crate::cov::brk(0xd6a3, fl.e()) { pc = 0xd6b5; continue; }
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

/// parity: P2v @0xbc13c size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc13c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbc13c;
    loop {
        match pc {
        0xbc13c => {
            // block bc13c
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc13c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc13c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc13d mov rbp,rsp
            rbp = rsp;
            // bc140 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc142 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc144 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc146 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc147 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc14b mov r15,rsi
            r15 = rsi;
            // bc14e mov r14d,edi
            r14 = ((rdi as u32 as u64)) as u32 as u64;
            // bc151 mov DWORD PTR [rbp-0x24],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((r14 as u32 as u64)) as u32);
            // bc155 lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // bc159 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // bc15d mov rdi,r15
            rdi = r15;
            // bc160 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bc165 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // bc16d mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // bc170 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // bc178 lea rdi,[rip+0x10c521] # 0x1c86a0
            rdi = m.env.data_va(904813);
            // bc17f lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // bc183 call 0xebdc8
            { let r = { let t = m.env.stub_va(0x595); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bc188 mov rbx,rax
            rbx = rax;
            // bc18b lea r12,[rip+0x10c516] # 0x1c86a8
            r12 = m.env.data_va(904821);
            // bc192 jmp 0xbc1cb
            pc = 0xbc1cb; continue;
        }
        0xbc194 => {
            // block bc194
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc194 cmp DWORD PTR [rbx+0x1c],r14d
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x1c)) as u64, (r14 as u32 as u64)).1;
            // bc198 jne 0xbc1d0
            if crate::cov::brk(0xbc198, fl.ne()) { pc = 0xbc1d0; continue; }
            pc = 0xbc19a;
        }
        0xbc19a => {
            // block bc19a
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc19a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc19a lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // bc19e mov rdi,r15
            rdi = r15;
            // bc1a1 call 0xebd98
            { let r = { let t = m.env.stub_va(0x565); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bc1a6 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc1aa test rax,rax
            fl = logic::<64>(rax & rax);
            // bc1ad je 0xbc1bc
            if crate::cov::brk(0xbc1ad, fl.e()) { pc = 0xbc1bc; continue; }
            pc = 0xbc1af;
        }
        0xbc1af => {
            // block bc1af
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1af mov rbx,rax
            rbx = rax;
            // bc1b2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bc1b5 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc1b8 jne 0xbc1af
            if crate::cov::brk(0xbc1b8, fl.ne()) { pc = 0xbc1af; continue; }
            pc = 0xbc1ba;
        }
        0xbc1ba => {
            // block bc1ba
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1ba jmp 0xbc1cb
            pc = 0xbc1cb; continue;
        }
        0xbc1bc => {
            // block bc1bc
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1bc mov rax,rbx
            rax = rbx;
            pc = 0xbc1bf;
        }
        0xbc1bf => {
            // block bc1bf
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1bf mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc1c3 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // bc1c6 mov rax,rbx
            rax = rbx;
            // bc1c9 jne 0xbc1bf
            if crate::cov::brk(0xbc1c9, fl.ne()) { pc = 0xbc1bf; continue; }
            pc = 0xbc1cb;
        }
        0xbc1cb => {
            // block bc1cb
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1cb cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // bc1ce jne 0xbc194
            if crate::cov::brk(0xbc1ce, fl.ne()) { pc = 0xbc194; continue; }
            pc = 0xbc1d0;
        }
        0xbc1d0 => {
            // block bc1d0
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1d0 cmp QWORD PTR [r15+0x10],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x0u64).1;
            // bc1d5 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc1d8 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc1dc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc1dd pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc1df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc1e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc1e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc1e4 ret
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
            { let r = { let t = m.env.code_va(0xcf5fd); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 44342 mov DWORD PTR [r12],0x8
            m.w32(r12, (0x8u64) as u32);
            // 4434a mov DWORD PTR [r14],0x20
            m.w32(r14, (0x20u64) as u32);
            // 44351 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 44354 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 44357 jb 0x44368
            if crate::cov::brk(0x44357, fl.b()) { pc = 0x44368; continue; }
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
            if crate::cov::brk(0x44366, fl.be()) { pc = 0x4437d; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 443a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 443a3 je 0x4436b
            if crate::cov::brk(0x443a3, fl.e()) { pc = 0x4436b; continue; }
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
