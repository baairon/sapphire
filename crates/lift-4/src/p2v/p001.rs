//! Mechanically lifted functions (scripts/lift_run.py), module p001. Each one is checked against the original
//! in the world harness (tests/world_p001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xdc0aa size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dc0aa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdc0aa;
    loop {
        match pc {
        0xdc0aa => {
            // block dc0aa
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0aa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc0ab mov rbp,rsp
            rbp = rsp;
            // dc0ae mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc0b1 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // dc0b5 mov rdx,rcx
            rdx = rcx;
            // dc0b8 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // dc0bb cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // dc0be jae 0xdc0c9
            if fl.ae() { pc = 0xdc0c9; continue; }
            pc = 0xdc0c0;
        }
        0xdc0c0 => {
            // block dc0c0
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0c0 sub rsi,rdx
            { let (r, f) = sub::<64>(rsi, rdx);
              rsi = r;
              fl = f; }
            // dc0c3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc0c4 jmp 0xebed6
            { let r = { let t = m.env.stub_va(0x6a3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xdc0c9 => {
            // block dc0c9
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0c9 jbe 0xdc0d7
            if fl.be() { pc = 0xdc0d7; continue; }
            pc = 0xdc0cb;
        }
        0xdc0cb => {
            // block dc0cb
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0cb add rax,rsi
            { let (r, f) = add::<64>(rax, rsi);
              rax = r;
              fl = f; }
            // dc0ce cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // dc0d1 je 0xdc0d7
            if fl.e() { pc = 0xdc0d7; continue; }
            pc = 0xdc0d3;
        }
        0xdc0d3 => {
            // block dc0d3
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0d3 mov QWORD PTR [rdi+0x8],rax
            m.w64(rdi.wrapping_add(0x8), (rax) as u64);
            pc = 0xdc0d7;
        }
        0xdc0d7 => {
            // block dc0d7
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc0d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

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
            { let r = { let t = m.env.stub_va(0x5bf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc0d8 mov ecx,0x6c
            rcx = (0x6cu64) as u32 as u64;
            // bc0dd lea rdx,[rip+0x10c54c] # 0x1c8630
            rdx = m.env.data_va(904701);
            // bc0e4 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // bc0e7 je 0xbc0ec
            if fl.e() { pc = 0xbc0ec; continue; }
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
            { let r = { let t = m.env.stub_va(0x60d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc118 mov ecx,0x6c
            rcx = (0x6cu64) as u32 as u64;
            // bc11d lea rdx,[rip+0x10c554] # 0x1c8678
            rdx = m.env.data_va(904773);
            // bc124 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // bc127 je 0xbc12c
            if fl.e() { pc = 0xbc12c; continue; }
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
            if fl.e() { pc = 0xba4d1; continue; }
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
            if fl.e() { pc = 0xba4cb; continue; }
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
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
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

/// parity: P2v @0x4b10 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004b10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4b10;
    loop {
        match pc {
        0x4b10 => {
            // block 4b10
            #[cfg(feature = "trace")] eprintln!("f_00004b10 pc 4b10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4b11 mov rbp,rsp
            rbp = rsp;
            // 4b14 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4b18 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // 4b1f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 4b22 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // 4b26 call QWORD PTR [rax+0x1e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1e0)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b2c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4b2e je 0x4b4b
            if fl.e() { pc = 0x4b4b; continue; }
            pc = 0x4b30;
        }
        0x4b30 => {
            // block 4b30
            #[cfg(feature = "trace")] eprintln!("f_00004b10 pc 4b30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b30 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4b32 mov ecx,DWORD PTR [rbp-0x4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 4b35 lea edx,[rcx-0xeb0c]
            rdx = (rcx.wrapping_add(0xffff_ffff_ffff_14f4)) as u32 as u64;
            // 4b3b cmp edx,0x3
            fl = sub::<32>((rdx as u32 as u64), 0x3u64).1;
            // 4b3e jb 0x4b4d
            if fl.b() { pc = 0x4b4d; continue; }
            pc = 0x4b40;
        }
        0x4b40 => {
            // block 4b40
            #[cfg(feature = "trace")] eprintln!("f_00004b10 pc 4b40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b40 add ecx,0xffff24f9
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xffff24f9u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4b46 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 4b49 jb 0x4b4d
            if fl.b() { pc = 0x4b4d; continue; }
            pc = 0x4b4b;
        }
        0x4b4b => {
            // block 4b4b
            #[cfg(feature = "trace")] eprintln!("f_00004b10 pc 4b4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b4b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4b4d;
        }
        0x4b4d => {
            // block 4b4d
            #[cfg(feature = "trace")] eprintln!("f_00004b10 pc 4b4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b4d add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4b51 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4b52 ret
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
            if fl.e() { pc = 0xc430d; continue; }
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
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
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
            if fl.e() { pc = 0xe0a77; continue; }
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
            if fl.e() { pc = 0xe0a6b; continue; }
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
            { let r = { let t = m.r64(rax) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a5f mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e0a63 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // e0a66 jne 0xe0a51
            if fl.ne() { pc = 0xe0a51; continue; }
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
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
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
            if fl.e() { pc = 0x28e9f; continue; }
            pc = 0x28e70;
        }
        0x28e70 => {
            // block 28e70
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e70 cmp QWORD PTR [rbx],0x0
            fl = sub::<64>(m.r64(rbx) as u64, 0x0u64).1;
            // 28e74 je 0x28e93
            if fl.e() { pc = 0x28e93; continue; }
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
            if fl.e() { pc = 0x28e93; continue; }
            pc = 0x28e7e;
        }
        0x28e7e => {
            // block 28e7e
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e7e mov rdi,r14
            rdi = r14;
            // 28e81 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e86 mov rdi,r14
            rdi = r14;
            // 28e89 mov rsi,rbx
            rsi = rbx;
            // 28e8c call 0x28dca
            { let r = { let t = m.env.code_va(0x28dca); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x28ea8); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0xe0dbd; continue; }
            pc = 0xe0da7;
        }
        0xe0da7 => {
            // block e0da7
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0da7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0da7 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // e0daa jne 0xe0db0
            if fl.ne() { pc = 0xe0db0; continue; }
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
            if fl.e() { pc = 0x56f5; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x5d8)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56dd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 56df je 0x56f5
            if fl.e() { pc = 0x56f5; continue; }
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
            if fl.e() { pc = 0x56f7; continue; }
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

/// parity: P2v @0x4383a size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004383a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4383a;
    loop {
        match pc {
        0x4383a => {
            // block 4383a
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 4383a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4383a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4383b mov rbp,rsp
            rbp = rsp;
            // 4383e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43840 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43841 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43845 mov r14,rdx
            r14 = rdx;
            // 43848 mov rbx,rsi
            rbx = rsi;
            // 4384b mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 43852 mov DWORD PTR [rbp-0x18],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0x8u64) as u32);
            // 43859 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 4385c lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 43860 lea rdx,[rbp-0x18]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 43864 call QWORD PTR [rax+0x890]
            { let r = { let t = m.r64(rax.wrapping_add(0x890)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4386a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4386c je 0x43883
            if fl.e() { pc = 0x43883; continue; }
            pc = 0x4386e;
        }
        0x4386e => {
            // block 4386e
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 4386e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4386e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 43871 je 0x43878
            if fl.e() { pc = 0x43878; continue; }
            pc = 0x43873;
        }
        0x43873 => {
            // block 43873
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 43873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43873 mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 43876 mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            pc = 0x43878;
        }
        0x43878 => {
            // block 43878
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 43878 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43878 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 4387b je 0x43883
            if fl.e() { pc = 0x43883; continue; }
            pc = 0x4387d;
        }
        0x4387d => {
            // block 4387d
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 4387d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4387d mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 43880 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            pc = 0x43883;
        }
        0x43883 => {
            // block 43883
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 43883 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43883 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43887 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43888 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4388a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4388b ret
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
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3117f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31181 je 0x311b4
            if fl.e() { pc = 0x311b4; continue; }
            pc = 0x31183;
        }
        0x31183 => {
            // block 31183
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 31183 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31183 mov rdi,r14
            rdi = r14;
            // 31186 call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3118b and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3118e cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 31193 jne 0x311b4
            if fl.ne() { pc = 0x311b4; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x2db52 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002db52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2db52;
    loop {
        match pc {
        0x2db52 => {
            // block 2db52
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2db53 mov rbp,rsp
            rbp = rsp;
            // 2db56 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2db5a test rax,rax
            fl = logic::<64>(rax & rax);
            // 2db5d je 0x2db90
            if fl.e() { pc = 0x2db90; continue; }
            pc = 0x2db5f;
        }
        0x2db5f => {
            // block 2db5f
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db5f mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 2db61 jmp 0x2db66
            pc = 0x2db66; continue;
        }
        0x2db63 => {
            // block 2db63
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db63 mov rax,rdx
            rax = rdx;
            pc = 0x2db66;
        }
        0x2db66 => {
            // block 2db66
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db66 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 2db69 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 2db6b jge 0x2db77
            if fl.ge() { pc = 0x2db77; continue; }
            pc = 0x2db6d;
        }
        0x2db6d => {
            // block 2db6d
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db6d mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 2db70 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2db73 jne 0x2db63
            if fl.ne() { pc = 0x2db63; continue; }
            pc = 0x2db75;
        }
        0x2db75 => {
            // block 2db75
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db75 jmp 0x2db9c
            pc = 0x2db9c; continue;
        }
        0x2db77 => {
            // block 2db77
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db77 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 2db79 jge 0x2dba4
            if fl.ge() { pc = 0x2dba4; continue; }
            pc = 0x2db7b;
        }
        0x2db7b => {
            // block 2db7b
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db7b mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2db7f test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2db82 jne 0x2db63
            if fl.ne() { pc = 0x2db63; continue; }
            pc = 0x2db84;
        }
        0x2db84 => {
            // block 2db84
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db84 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2db87 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 2db8b mov rsi,rax
            rsi = rax;
            // 2db8e jmp 0x2dba7
            pc = 0x2dba7; continue;
        }
        0x2db90 => {
            // block 2db90
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db90 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 2db94 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 2db97 mov rsi,rdi
            rsi = rdi;
            // 2db9a jmp 0x2dba7
            pc = 0x2dba7; continue;
        }
        0x2db9c => {
            // block 2db9c
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db9c mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2db9f mov rsi,rax
            rsi = rax;
            // 2dba2 jmp 0x2dba7
            pc = 0x2dba7; continue;
        }
        0x2dba4 => {
            // block 2dba4
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2dba4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dba4 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x2dba7;
        }
        0x2dba7 => {
            // block 2dba7
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2dba7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dba7 mov rax,rsi
            rax = rsi;
            // 2dbaa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dbab ret
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
            if fl.e() { pc = 0x44b9c; continue; }
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
            if fl.ge() { pc = 0x44b83; continue; }
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
            if fl.ne() { pc = 0x44b6f; continue; }
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
            if fl.ge() { pc = 0x44bb0; continue; }
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
            if fl.ne() { pc = 0x44b6f; continue; }
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
            if fl.e() { pc = 0xc4402; continue; }
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
            if fl.ae() { pc = 0xc43e9; continue; }
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
            if fl.ne() { pc = 0xc43d5; continue; }
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
            if fl.ae() { pc = 0xc4416; continue; }
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
            if fl.ne() { pc = 0xc43d5; continue; }
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
            if fl.e() { pc = 0xc4dea; continue; }
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
            if fl.ae() { pc = 0xc4dd1; continue; }
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
            if fl.ne() { pc = 0xc4dbd; continue; }
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
            if fl.ae() { pc = 0xc4dfe; continue; }
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
            if fl.ne() { pc = 0xc4dbd; continue; }
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
            if fl.e() { pc = 0xc5156; continue; }
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
            if fl.ae() { pc = 0xc513d; continue; }
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
            if fl.ne() { pc = 0xc5129; continue; }
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
            if fl.ae() { pc = 0xc516a; continue; }
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
            if fl.ne() { pc = 0xc5129; continue; }
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

/// parity: P2v @0xc56c0 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c56c0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc56c0;
    loop {
        match pc {
        0xc56c0 => {
            // block c56c0
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56c0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c56c1 mov rbp,rsp
            rbp = rsp;
            // c56c4 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c56c8 test rax,rax
            fl = logic::<64>(rax & rax);
            // c56cb je 0xc56fe
            if fl.e() { pc = 0xc56fe; continue; }
            pc = 0xc56cd;
        }
        0xc56cd => {
            // block c56cd
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56cd mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c56cf jmp 0xc56d4
            pc = 0xc56d4; continue;
        }
        0xc56d1 => {
            // block c56d1
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56d1 mov rax,rdx
            rax = rdx;
            pc = 0xc56d4;
        }
        0xc56d4 => {
            // block c56d4
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56d4 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c56d7 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c56d9 jae 0xc56e5
            if fl.ae() { pc = 0xc56e5; continue; }
            pc = 0xc56db;
        }
        0xc56db => {
            // block c56db
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56db mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c56de test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c56e1 jne 0xc56d1
            if fl.ne() { pc = 0xc56d1; continue; }
            pc = 0xc56e3;
        }
        0xc56e3 => {
            // block c56e3
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56e3 jmp 0xc570a
            pc = 0xc570a; continue;
        }
        0xc56e5 => {
            // block c56e5
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56e5 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c56e7 jae 0xc5712
            if fl.ae() { pc = 0xc5712; continue; }
            pc = 0xc56e9;
        }
        0xc56e9 => {
            // block c56e9
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56e9 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // c56ed test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c56f0 jne 0xc56d1
            if fl.ne() { pc = 0xc56d1; continue; }
            pc = 0xc56f2;
        }
        0xc56f2 => {
            // block c56f2
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56f2 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c56f5 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c56f9 mov rsi,rax
            rsi = rax;
            // c56fc jmp 0xc5715
            pc = 0xc5715; continue;
        }
        0xc56fe => {
            // block c56fe
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56fe add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c5702 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c5705 mov rsi,rdi
            rsi = rdi;
            // c5708 jmp 0xc5715
            pc = 0xc5715; continue;
        }
        0xc570a => {
            // block c570a
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c570a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c570a mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c570d mov rsi,rax
            rsi = rax;
            // c5710 jmp 0xc5715
            pc = 0xc5715; continue;
        }
        0xc5712 => {
            // block c5712
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c5712 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5712 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc5715;
        }
        0xc5715 => {
            // block c5715
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c5715 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5715 mov rax,rsi
            rax = rsi;
            // c5718 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c5719 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd49a8 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d49a8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd49a8;
    loop {
        match pc {
        0xd49a8 => {
            // block d49a8
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49a8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d49a9 mov rbp,rsp
            rbp = rsp;
            // d49ac mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // d49b0 test rax,rax
            fl = logic::<64>(rax & rax);
            // d49b3 je 0xd49e6
            if fl.e() { pc = 0xd49e6; continue; }
            pc = 0xd49b5;
        }
        0xd49b5 => {
            // block d49b5
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49b5 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // d49b7 jmp 0xd49bc
            pc = 0xd49bc; continue;
        }
        0xd49b9 => {
            // block d49b9
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49b9 mov rax,rdx
            rax = rdx;
            pc = 0xd49bc;
        }
        0xd49bc => {
            // block d49bc
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49bc mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // d49bf cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // d49c1 jge 0xd49cd
            if fl.ge() { pc = 0xd49cd; continue; }
            pc = 0xd49c3;
        }
        0xd49c3 => {
            // block d49c3
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49c3 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // d49c6 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // d49c9 jne 0xd49b9
            if fl.ne() { pc = 0xd49b9; continue; }
            pc = 0xd49cb;
        }
        0xd49cb => {
            // block d49cb
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49cb jmp 0xd49f2
            pc = 0xd49f2; continue;
        }
        0xd49cd => {
            // block d49cd
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49cd cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // d49cf jge 0xd49fa
            if fl.ge() { pc = 0xd49fa; continue; }
            pc = 0xd49d1;
        }
        0xd49d1 => {
            // block d49d1
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49d1 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // d49d5 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // d49d8 jne 0xd49b9
            if fl.ne() { pc = 0xd49b9; continue; }
            pc = 0xd49da;
        }
        0xd49da => {
            // block d49da
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49da mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // d49dd add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // d49e1 mov rsi,rax
            rsi = rax;
            // d49e4 jmp 0xd49fd
            pc = 0xd49fd; continue;
        }
        0xd49e6 => {
            // block d49e6
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49e6 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // d49ea mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // d49ed mov rsi,rdi
            rsi = rdi;
            // d49f0 jmp 0xd49fd
            pc = 0xd49fd; continue;
        }
        0xd49f2 => {
            // block d49f2
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49f2 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // d49f5 mov rsi,rax
            rsi = rax;
            // d49f8 jmp 0xd49fd
            pc = 0xd49fd; continue;
        }
        0xd49fa => {
            // block d49fa
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49fa mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xd49fd;
        }
        0xd49fd => {
            // block d49fd
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49fd mov rax,rsi
            rax = rsi;
            // d4a00 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4a01 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3193 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3193<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd3193;
    loop {
        match pc {
        0xd3193 => {
            // block d3193
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d3193 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3193 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3194 mov rbp,rsp
            rbp = rsp;
            // d3197 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3199 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d319b push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d319d push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d319f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d31a0 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d31a1 mov r14,rdx
            r14 = rdx;
            // d31a4 mov r15,rsi
            r15 = rsi;
            // d31a7 mov r12d,edi
            r12 = ((rdi as u32 as u64)) as u32 as u64;
            // d31aa mov ebx,DWORD PTR [r14]
            rbx = (m.r32(r14) as u64) as u32 as u64;
            // d31ad xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d31b0 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // d31b2 jle 0xd31da
            if fl.le() { pc = 0xd31da; continue; }
            pc = 0xd31b4;
        }
        0xd31b4 => {
            // block d31b4
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d31b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31b4 movsxd rsi,r13d
            rsi = sx::<32>((r13 as u32 as u64)) as u64;
            // d31b7 add rsi,r15
            { let (r, f) = add::<64>(rsi, r15);
              rsi = r;
              fl = f; }
            // d31ba movsxd rdx,ebx
            rdx = sx::<32>((rbx as u32 as u64)) as u64;
            // d31bd xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d31bf mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // d31c2 call 0xec28a
            { let r = { let t = m.env.stub_va(0xa57); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d31c7 cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // d31ca je 0xd31da
            if fl.e() { pc = 0xd31da; continue; }
            pc = 0xd31cc;
        }
        0xd31cc => {
            // block d31cc
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d31cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31cc add r13d,eax
            { let (r, f) = add::<32>((r13 as u32 as u64), (rax as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // d31cf sub ebx,eax
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // d31d1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d31d3 cmp r13d,DWORD PTR [r14]
            fl = sub::<32>((r13 as u32 as u64), m.r32(r14) as u64).1;
            // d31d6 jl 0xd31b4
            if fl.l() { pc = 0xd31b4; continue; }
            pc = 0xd31d8;
        }
        0xd31d8 => {
            // block d31d8
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d31d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31d8 jmp 0xd31df
            pc = 0xd31df; continue;
        }
        0xd31da => {
            // block d31da
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d31da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31da mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            pc = 0xd31df;
        }
        0xd31df => {
            // block d31df
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d31df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31df mov DWORD PTR [r14],r13d
            m.w32(r14, ((r13 as u32 as u64)) as u32);
            // d31e2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d31e6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d31e7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d31e9 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d31eb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d31ed pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d31ef pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d31f0 ret
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d2c test BYTE PTR [rbp-0x17],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe9)) as u64 & 0x1u64);
            // 3d30 jne 0x3d0c
            if fl.ne() { pc = 0x3d0c; continue; }
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

/// parity: P2v @0x25d00 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025d00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x25d00;
    loop {
        match pc {
        0x25d00 => {
            // block 25d00
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25d01 mov rbp,rsp
            rbp = rsp;
            // 25d04 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 25d08 test rax,rax
            fl = logic::<64>(rax & rax);
            // 25d0b je 0x25d3e
            if fl.e() { pc = 0x25d3e; continue; }
            pc = 0x25d0d;
        }
        0x25d0d => {
            // block 25d0d
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d0d mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 25d0f jmp 0x25d14
            pc = 0x25d14; continue;
        }
        0x25d11 => {
            // block 25d11
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d11 mov rax,rdx
            rax = rdx;
            pc = 0x25d14;
        }
        0x25d14 => {
            // block 25d14
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d14 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 25d17 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 25d19 jge 0x25d25
            if fl.ge() { pc = 0x25d25; continue; }
            pc = 0x25d1b;
        }
        0x25d1b => {
            // block 25d1b
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d1b mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 25d1e test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 25d21 jne 0x25d11
            if fl.ne() { pc = 0x25d11; continue; }
            pc = 0x25d23;
        }
        0x25d23 => {
            // block 25d23
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d23 jmp 0x25d4a
            pc = 0x25d4a; continue;
        }
        0x25d25 => {
            // block 25d25
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d25 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 25d27 jge 0x25d52
            if fl.ge() { pc = 0x25d52; continue; }
            pc = 0x25d29;
        }
        0x25d29 => {
            // block 25d29
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d29 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 25d2d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 25d30 jne 0x25d11
            if fl.ne() { pc = 0x25d11; continue; }
            pc = 0x25d32;
        }
        0x25d32 => {
            // block 25d32
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d32 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 25d35 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 25d39 mov rsi,rax
            rsi = rax;
            // 25d3c jmp 0x25d55
            pc = 0x25d55; continue;
        }
        0x25d3e => {
            // block 25d3e
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d3e add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 25d42 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 25d45 mov rsi,rdi
            rsi = rdi;
            // 25d48 jmp 0x25d55
            pc = 0x25d55; continue;
        }
        0x25d4a => {
            // block 25d4a
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d4a mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 25d4d mov rsi,rax
            rsi = rax;
            // 25d50 jmp 0x25d55
            pc = 0x25d55; continue;
        }
        0x25d52 => {
            // block 25d52
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d52 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x25d55;
        }
        0x25d55 => {
            // block 25d55
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d55 mov rax,rsi
            rax = rsi;
            // 25d58 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25d59 ret
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
            if fl.e() { pc = 0x2dcda; continue; }
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
            if fl.ae() { pc = 0x2dcc1; continue; }
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
            if fl.ne() { pc = 0x2dcad; continue; }
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
            if fl.ae() { pc = 0x2dcee; continue; }
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
            if fl.ne() { pc = 0x2dcad; continue; }
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
            if fl.e() { pc = 0xb665e; continue; }
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
            if fl.b() { pc = 0xb6631; continue; }
            pc = 0xb6628;
        }
        0xb6628 => {
            // block b6628
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6628 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6628 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // b662a jb 0xb663c
            if fl.b() { pc = 0xb663c; continue; }
            pc = 0xb662c;
        }
        0xb662c => {
            // block b662c
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b662c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b662c cmp DWORD PTR [rax+0x20],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x20)) as u64, (rcx as u32 as u64)).1;
            // b662f jae 0xb663c
            if fl.ae() { pc = 0xb663c; continue; }
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
            if fl.ne() { pc = 0xb6621; continue; }
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
            if fl.ne() { pc = 0xb661b; continue; }
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
            if fl.e() { pc = 0xb665e; continue; }
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
            if fl.b() { pc = 0xb665e; continue; }
            pc = 0xb6655;
        }
        0xb6655 => {
            // block b6655
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6655 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6655 cmp esi,edx
            fl = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64)).1;
            // b6657 jb 0xb6661
            if fl.b() { pc = 0xb6661; continue; }
            pc = 0xb6659;
        }
        0xb6659 => {
            // block b6659
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6659 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6659 cmp ecx,DWORD PTR [rax+0x20]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x20)) as u64).1;
            // b665c jae 0xb6661
            if fl.ae() { pc = 0xb6661; continue; }
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
            { let r = { let t = m.env.code_va(0xe4a46); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e46c1 mov ecx,DWORD PTR [rbp-0x1c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e46c4 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e46c6 test cl,0x2
            fl = logic::<8>((rcx as u8 as u64) & 0x2u64);
            // e46c9 je 0xe46e1
            if fl.e() { pc = 0xe46e1; continue; }
            pc = 0xe46cb;
        }
        0xe46cb => {
            // block e46cb
            #[cfg(feature = "trace")] eprintln!("f_000e4684 pc e46cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e46cb mov rdi,rbx
            rdi = rbx;
            // e46ce call 0xe4a46
            { let r = { let t = m.env.code_va(0xe4a46); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e46d3 sub rax,QWORD PTR [rbx+0x2040]
            { let (r, f) = sub::<64>(rax, m.r64(rbx.wrapping_add(0x2040)) as u64);
              rax = r;
              fl = f; }
            // e46da cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // e46dd jbe 0xe46a7
            if fl.be() { pc = 0xe46a7; continue; }
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

/// parity: P2v @0xe46ec size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e46ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe46ec;
    loop {
        match pc {
        0xe46ec => {
            // block e46ec
            #[cfg(feature = "trace")] eprintln!("f_000e46ec pc e46ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e46ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e46ed mov rbp,rsp
            rbp = rsp;
            // e46f0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e46f2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e46f4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e46f5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e46f6 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e46f9 mov rbx,rdi
            rbx = rdi;
            // e46fc call 0xe4a46
            { let r = { let t = m.env.code_va(0xe4a46); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4701 mov QWORD PTR [rbx+0x2040],rax
            m.w64(rbx.wrapping_add(0x2040), (rax) as u64);
            // e4708 mov r15d,r14d
            r15 = ((r14 as u32 as u64)) as u32 as u64;
            // e470b lea r14,[rbp-0x1c]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            pc = 0xe470f;
        }
        0xe470f => {
            // block e470f
            #[cfg(feature = "trace")] eprintln!("f_000e46ec pc e470f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e470f mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e4712 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e4715 mov esi,DWORD PTR [rbx+0x30]
            rsi = (m.r32(rbx.wrapping_add(0x30)) as u64) as u32 as u64;
            // e4718 add esi,0x4
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x4u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e471b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e4720 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4723 mov rdx,r14
            rdx = r14;
            // e4726 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4729 mov ecx,DWORD PTR [rbp-0x1c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e472c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e472e test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // e4731 je 0xe4749
            if fl.e() { pc = 0xe4749; continue; }
            pc = 0xe4733;
        }
        0xe4733 => {
            // block e4733
            #[cfg(feature = "trace")] eprintln!("f_000e46ec pc e4733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4733 mov rdi,rbx
            rdi = rbx;
            // e4736 call 0xe4a46
            { let r = { let t = m.env.code_va(0xe4a46); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e473b sub rax,QWORD PTR [rbx+0x2040]
            { let (r, f) = sub::<64>(rax, m.r64(rbx.wrapping_add(0x2040)) as u64);
              rax = r;
              fl = f; }
            // e4742 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // e4745 jbe 0xe470f
            if fl.be() { pc = 0xe470f; continue; }
            pc = 0xe4747;
        }
        0xe4747 => {
            // block e4747
            #[cfg(feature = "trace")] eprintln!("f_000e46ec pc e4747 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4747 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4749;
        }
        0xe4749 => {
            // block e4749
            #[cfg(feature = "trace")] eprintln!("f_000e46ec pc e4749 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4749 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e474d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e474e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4750 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4752 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4753 ret
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
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xc579e; continue; }
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
            if fl.ge() { pc = 0xc57ad; continue; }
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
            if fl.ne() { pc = 0xc579b; continue; }
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
            if fl.ne() { pc = 0xc579b; continue; }
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
            { let r = { let t = m.env.stub_va(0x58f); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xc58fe; continue; }
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
            if fl.ge() { pc = 0xc590d; continue; }
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
            if fl.ne() { pc = 0xc58fb; continue; }
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
            if fl.ne() { pc = 0xc58fb; continue; }
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
            { let r = { let t = m.env.stub_va(0x5a7); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4975 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e4978 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // e497b test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e497d je 0xe49b6
            if fl.e() { pc = 0xe49b6; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1880)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4990 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e4992 jne 0xe49b6
            if fl.ne() { pc = 0xe49b6; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e49a7 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // e49a9 jne 0xe4984
            if fl.ne() { pc = 0xe4984; continue; }
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

/// parity: P2v @0x42d7c size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042d7c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42d7c;
    loop {
        match pc {
        0x42d7c => {
            // block 42d7c
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42d7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42d7c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42d7d mov rbp,rsp
            rbp = rsp;
            // 42d80 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42d82 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42d84 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 42d86 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42d87 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 42d8a mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // 42d8d mov rbx,rdi
            rbx = rdi;
            // 42d90 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42d93 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42d99 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42d9c cmp r14d,0x1ffff
            fl = sub::<32>((r14 as u32 as u64), 0x1ffffu64).1;
            // 42da3 ja 0x42deb
            if fl.a() { pc = 0x42deb; continue; }
            pc = 0x42da5;
        }
        0x42da5 => {
            // block 42da5
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42da5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42da5 cmp r12d,0x1
            fl = sub::<32>((r12 as u32 as u64), 0x1u64).1;
            // 42da9 jg 0x42deb
            if fl.g() { pc = 0x42deb; continue; }
            pc = 0x42dab;
        }
        0x42dab => {
            // block 42dab
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42dab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42dab xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 42dad jne 0x42deb
            if fl.ne() { pc = 0x42deb; continue; }
            pc = 0x42daf;
        }
        0x42daf => {
            // block 42daf
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42daf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42daf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42db2 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42db5 mov esi,0x902
            rsi = (0x902u64) as u32 as u64;
            // 42dba mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42dbf xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42dc2 mov rdi,rbx
            rdi = rbx;
            // 42dc5 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42dc8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42dcb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42dcd je 0x42deb
            if fl.e() { pc = 0x42deb; continue; }
            pc = 0x42dcf;
        }
        0x42dcf => {
            // block 42dcf
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42dcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42dcf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42dd2 mov esi,0x906
            rsi = (0x906u64) as u32 as u64;
            // 42dd7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42ddc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42ddf mov rdi,rbx
            rdi = rbx;
            // 42de2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42de5 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42de8 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x42deb;
        }
        0x42deb => {
            // block 42deb
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42deb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42deb mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 42dee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42def pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 42df1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42df3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42df5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42df6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
