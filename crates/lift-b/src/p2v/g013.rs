//! Mechanically lifted functions (scripts/lift_run.py), module g013. Each one is checked against the original
//! in the world harness (tests/world_g013.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x15430 size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00015430<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x15430;
    loop {
        match pc {
        0x15430 => {
            // block 15430
            // 15430 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15431 mov rbp,rsp
            rbp = rsp;
            // 15434 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15435 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15436 mov r8d,edx
            r8 = ((rdx as u32 as u64)) as u32 as u64;
            // 15439 mov rbx,rdi
            rbx = rdi;
            // 1543c mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 1543f mov rdx,QWORD PTR [rbp+0x90]
            rdx = m.r64(rbp.wrapping_add(0x90)) as u64;
            // 15446 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 15448 call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1544b mov rax,rbx
            rax = rbx;
            // 1544e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 15452 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15453 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15454 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4388c size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004388c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4388c;
    loop {
        match pc {
        0x4388c => {
            // block 4388c
            // 4388c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4388d mov rbp,rsp
            rbp = rsp;
            // 43890 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43893 jle 0x43899
            if fl.le() { pc = 0x43899; continue; }
            pc = 0x43895;
        }
        0x43895 => {
            // block 43895
            // 43895 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43897 jmp 0x438b9
            pc = 0x438b9; continue;
        }
        0x43899 => {
            // block 43899
            // 43899 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 4389c mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 4389e lea rsi,[rip+0xbcb1b] # 0x1003c0
            rsi = m.env.data_va(84877);
            // 438a5 mov esi,DWORD PTR [rsi+rcx*4]
            rsi = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 438a8 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // 438ab mov ecx,0x200
            rcx = (0x200u64) as u32 as u64;
            // 438b0 mov r8d,0x9
            r8 = (0x9u64) as u32 as u64;
            // 438b6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x438b9;
        }
        0x438b9 => {
            // block 438b9
            // 438b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 438ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4397c size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004397c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4397c;
    loop {
        match pc {
        0x4397c => {
            // block 4397c
            // 4397c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4397d mov rbp,rsp
            rbp = rsp;
            // 43980 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43983 jle 0x43989
            if fl.le() { pc = 0x43989; continue; }
            pc = 0x43985;
        }
        0x43985 => {
            // block 43985
            // 43985 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43987 jmp 0x439a9
            pc = 0x439a9; continue;
        }
        0x43989 => {
            // block 43989
            // 43989 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 4398c mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 4398e lea rsi,[rip+0xbca2b] # 0x1003c0
            rsi = m.env.data_va(84877);
            // 43995 mov esi,DWORD PTR [rsi+rcx*4]
            rsi = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43998 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // 4399b mov ecx,0x800
            rcx = (0x800u64) as u32 as u64;
            // 439a0 mov r8d,0xb
            r8 = (0xbu64) as u32 as u64;
            // 439a6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x439a9;
        }
        0x439a9 => {
            // block 439a9
            // 439a9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 439aa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x443d2 size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000443d2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x443d2;
    loop {
        match pc {
        0x443d2 => {
            // block 443d2
            // 443d2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 443d3 mov rbp,rsp
            rbp = rsp;
            // 443d6 lea edx,[rcx+rdx*4]
            rdx = (rcx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // 443d9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 443dc mov r9,QWORD PTR [rax+0x30]
            r9 = m.r64(rax.wrapping_add(0x30)) as u64;
            // 443e0 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 443e2 lea rcx,[rip+0xbc2a7] # 0x100690
            rcx = m.env.data_va(85597);
            // 443e9 mov r8d,DWORD PTR [rcx+rax*4]
            r8 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 443ed mov eax,0xf
            rax = (0xfu64) as u32 as u64;
            // 443f2 mov cl,r8b
            rcx = (rcx & !0xff) | (((r8 as u8 as u64)) as u8 as u64);
            // 443f5 shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 443f7 mov esi,0xbe
            rsi = (0xbeu64) as u32 as u64;
            // 443fc mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 443fe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 443ff jmp r9
            { let r = { let t = r9; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

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
            // dc0c0 sub rsi,rdx
            { let (r, f) = sub::<64>(rsi, rdx);
              rsi = r;
              fl = f; }
            // dc0c3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc0c4 jmp 0xebed6
            { let r = { let t = m.env.stub_va(0x6a3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xdc0c9 => {
            // block dc0c9
            // dc0c9 jbe 0xdc0d7
            if fl.be() { pc = 0xdc0d7; continue; }
            pc = 0xdc0cb;
        }
        0xdc0cb => {
            // block dc0cb
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
            // dc0d3 mov QWORD PTR [rdi+0x8],rax
            m.w64(rdi.wrapping_add(0x8), (rax) as u64);
            pc = 0xdc0d7;
        }
        0xdc0d7 => {
            // block dc0d7
            // dc0d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc0d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11a52 size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011a52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x11a52;
    loop {
        match pc {
        0x11a52 => {
            // block 11a52
            // 11a52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11a53 mov rbp,rsp
            rbp = rsp;
            // 11a56 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11a57 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 11a58 mov rbx,rdi
            rbx = rdi;
            // 11a5b mov rax,QWORD PTR [rip+0x12e7b6] # 0x140218
            rax = m.r64(m.env.data_va(346597)) as u64;
            // 11a62 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 11a66 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 11a69 mov rdi,QWORD PTR [rbx+0x18]
            rdi = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 11a6d test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 11a70 je 0x11a77
            if fl.e() { pc = 0x11a77; continue; }
            pc = 0x11a72;
        }
        0x11a72 => {
            // block 11a72
            // 11a72 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x11a77;
        }
        0x11a77 => {
            // block 11a77
            // 11a77 mov rdi,rbx
            rdi = rbx;
            // 11a7a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 11a7e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11a7f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11a80 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43ade size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043ade<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43ade;
    loop {
        match pc {
        0x43ade => {
            // block 43ade
            // 43ade push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43adf mov rbp,rsp
            rbp = rsp;
            // 43ae2 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43ae5 jle 0x43aeb
            if fl.le() { pc = 0x43aeb; continue; }
            pc = 0x43ae7;
        }
        0x43ae7 => {
            // block 43ae7
            // 43ae7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43ae9 jmp 0x43b10
            pc = 0x43b10; continue;
        }
        0x43aeb => {
            // block 43aeb
            // 43aeb mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 43aed lea rcx,[rip+0xbcacc] # 0x1005c0
            rcx = m.env.data_va(85389);
            // 43af4 mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 43af7 lea rsi,[rip+0xbcaf2] # 0x1005f0
            rsi = m.env.data_va(85437);
            // 43afe mov r8d,DWORD PTR [rsi+rax*4]
            r8 = (m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 43b02 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43b05 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // 43b08 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 43b0d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43b10;
        }
        0x43b10 => {
            // block 43b10
            // 43b10 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43b11 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3e7c size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3e7c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3e7c;
    loop {
        match pc {
        0xc3e7c => {
            // block c3e7c
            // c3e7c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3e7d mov rbp,rsp
            rbp = rsp;
            // c3e80 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3e82 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3e83 mov rbx,rdi
            rbx = rdi;
            // c3e86 lea rdi,[rbx+0x8]
            rdi = rbx.wrapping_add(0x8);
            // c3e8a lea r14,[rbx+0x10]
            r14 = rbx.wrapping_add(0x10);
            // c3e8e mov rsi,QWORD PTR [rbx+0x10]
            rsi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c3e92 call 0xebe22
            { let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3e97 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // c3e9f mov QWORD PTR [rbx+0x8],r14
            m.w64(rbx.wrapping_add(0x8), (r14) as u64);
            // c3ea3 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // c3eab pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3eac pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3eae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3eaf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3174a size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0003174a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3174a;
    loop {
        match pc {
        0x3174a => {
            // block 3174a
            // 3174a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3174b mov rbp,rsp
            rbp = rsp;
            // 3174e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31750 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31751 mov r14,rsi
            r14 = rsi;
            // 31754 mov rbx,rdi
            rbx = rdi;
            // 31757 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 3175a jns 0x31764
            if fl.ns() { pc = 0x31764; continue; }
            pc = 0x3175c;
        }
        0x3175c => {
            // block 3175c
            // 3175c mov rdi,rbx
            rdi = rbx;
            // 3175f call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x31764 => {
            // block 31764
            // 31764 mov rdi,r14
            rdi = r14;
            // 31767 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3176c mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 31770 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 31773 add rax,r14
            { let (r, f) = add::<64>(rax, r14);
              rax = r;
              fl = f; }
            // 31776 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 3177a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3177b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3177d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3177e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44cca size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044cca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44cca;
    loop {
        match pc {
        0x44cca => {
            // block 44cca
            // 44cca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44ccb mov rbp,rsp
            rbp = rsp;
            // 44cce sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44cd2 mov r9d,ecx
            r9 = ((rcx as u32 as u64)) as u32 as u64;
            // 44cd5 mov r10,rdx
            r10 = rdx;
            // 44cd8 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 44cda mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 44cdd mov DWORD PTR [rsp],r8d
            m.w32(rsp, ((r8 as u32 as u64)) as u32);
            // 44ce1 mov DWORD PTR [rsp+0x8],0x1
            m.w32(rsp.wrapping_add(0x8), (0x1u64) as u32);
            // 44ce9 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 44cee mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 44cf3 mov r8,r10
            r8 = r10;
            // 44cf6 call QWORD PTR [rax+0x58]
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44cf9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44cfd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44cfe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x54cc size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000054cc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x54cc;
    loop {
        match pc {
        0x54cc => {
            // block 54cc
            // 54cc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 54cd mov rbp,rsp
            rbp = rsp;
            // 54d0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 54d2 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // 54d5 jne 0x54e0
            if fl.ne() { pc = 0x54e0; continue; }
            pc = 0x54d7;
        }
        0x54d7 => {
            // block 54d7
            // 54d7 mov r10b,dl
            r10 = (r10 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 54da xor r10b,0x1
            { let r = (r10 as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              r10 = (r10 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 54de je 0x5501
            if fl.e() { pc = 0x5501; continue; }
            pc = 0x54e0;
        }
        0x54e0 => {
            // block 54e0
            // 54e0 mov r10,QWORD PTR [rdi]
            r10 = m.r64(rdi) as u64;
            // 54e3 mov r10,QWORD PTR [r10+0x5c8]
            r10 = m.r64(r10.wrapping_add(0x5c8)) as u64;
            // 54ea test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // 54ed je 0x54f4
            if fl.e() { pc = 0x54f4; continue; }
            pc = 0x54ef;
        }
        0x54ef => {
            // block 54ef
            // 54ef movzx eax,dl
            rax = ((rdx as u8 as u64)) as u32 as u64;
            // 54f2 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0x54f4;
        }
        0x54f4 => {
            // block 54f4
            // 54f4 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 54f6 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 54f8 mov ecx,r8d
            rcx = ((r8 as u32 as u64)) as u32 as u64;
            // 54fb mov r8d,r9d
            r8 = ((r9 as u32 as u64)) as u32 as u64;
            // 54fe call r10
            { let r = { let t = r10; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5501;
        }
        0x5501 => {
            // block 5501
            // 5501 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5502 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x270b9 size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000270b9<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x270b9;
    loop {
        match pc {
        0x270b9 => {
            // block 270b9
            // 270b9 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 270ba mov rbp,rsp
            rbp = rsp;
            // 270bd push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 270be push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 270bf mov rbx,rdi
            rbx = rdi;
            // 270c2 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 270c6 cmp rax,QWORD PTR [rbx+0x10]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // 270ca je 0x270df
            if fl.e() { pc = 0x270df; continue; }
            pc = 0x270cc;
        }
        0x270cc => {
            // block 270cc
            // 270cc mov ecx,DWORD PTR [rsi+0x8]
            rcx = (m.r32(rsi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 270cf mov DWORD PTR [rax+0x8],ecx
            m.w32(rax.wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 270d2 mov rcx,QWORD PTR [rsi]
            rcx = m.r64(rsi) as u64;
            // 270d5 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 270d8 add QWORD PTR [rbx+0x8],0xc
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, 0xcu64);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 270dd jmp 0x270e7
            pc = 0x270e7; continue;
        }
        0x270df => {
            // block 270df
            // 270df mov rdi,rbx
            rdi = rbx;
            // 270e2 call 0xebaf2
            { let r = { let t = m.env.stub_va(0x2bf); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x270e7;
        }
        0x270e7 => {
            // block 270e7
            // 270e7 mov rax,rbx
            rax = rbx;
            // 270ea add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 270ee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 270ef pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 270f0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdf4c size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000df4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdf4c;
    loop {
        match pc {
        0xdf4c => {
            // block df4c
            // df4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df4d mov rbp,rsp
            rbp = rsp;
            // df50 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // df52 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df53 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // df56 mov rbx,rdi
            rbx = rdi;
            // df59 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df5c call QWORD PTR [rax+0x1728]
            { let r = { let t = m.r64(rax.wrapping_add(0x1728)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df62 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // df64 je 0xdf81
            if fl.e() { pc = 0xdf81; continue; }
            pc = 0xdf66;
        }
        0xdf66 => {
            // block df66
            // df66 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df69 mov esi,0x137
            rsi = (0x137u64) as u32 as u64;
            // df6e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // df73 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df76 mov rdi,rbx
            rdi = rbx;
            // df79 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df7c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df7f jmp 0xdf83
            pc = 0xdf83; continue;
        }
        0xdf81 => {
            // block df81
            // df81 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xdf83;
        }
        0xdf83 => {
            // block df83
            // df83 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // df84 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // df86 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // df87 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42bea size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042bea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42bea;
    loop {
        match pc {
        0x42bea => {
            // block 42bea
            // 42bea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42beb mov rbp,rsp
            rbp = rsp;
            // 42bee push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42bf0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42bf1 mov r14,rsi
            r14 = rsi;
            // 42bf4 mov rbx,rdi
            rbx = rdi;
            // 42bf7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42bfa call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42c00 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42c02 je 0x42c1f
            if fl.e() { pc = 0x42c1f; continue; }
            pc = 0x42c04;
        }
        0x42c04 => {
            // block 42c04
            // 42c04 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42c07 mov esi,0x901
            rsi = (0x901u64) as u32 as u64;
            // 42c0c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42c11 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42c14 mov rdi,rbx
            rdi = rbx;
            // 42c17 mov rdx,r14
            rdx = r14;
            // 42c1a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42c1d jmp 0x42c21
            pc = 0x42c21; continue;
        }
        0x42c1f => {
            // block 42c1f
            // 42c1f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42c21;
        }
        0x42c21 => {
            // block 42c21
            // 42c21 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42c22 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42c24 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42c25 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4270 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4270<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4270;
    loop {
        match pc {
        0xe4270 => {
            // block e4270
            // e4270 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4271 mov rbp,rsp
            rbp = rsp;
            // e4274 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4276 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4277 mov rbx,rdi
            rbx = rdi;
            // e427a mov r14,QWORD PTR [rbx+0x8]
            r14 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e427e jmp 0xe428d
            pc = 0xe428d; continue;
        }
        0xe4280 => {
            // block e4280
            // e4280 add rdi,0xffffffffffffffe8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffe8u64);
              rdi = r;
              fl = f; }
            // e4284 mov QWORD PTR [rbx+0x10],rdi
            m.w64(rbx.wrapping_add(0x10), (rdi) as u64);
            // e4288 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe428d;
        }
        0xe428d => {
            // block e428d
            // e428d mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e4291 cmp rdi,r14
            fl = sub::<64>(rdi, r14).1;
            // e4294 jne 0xe4280
            if fl.ne() { pc = 0xe4280; continue; }
            pc = 0xe4296;
        }
        0xe4296 => {
            // block e4296
            // e4296 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e4299 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e429c je 0xe42a7
            if fl.e() { pc = 0xe42a7; continue; }
            pc = 0xe429e;
        }
        0xe429e => {
            // block e429e
            // e429e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e429f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e42a1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e42a2 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xe42a7 => {
            // block e42a7
            // e42a7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e42a8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e42aa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e42ab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11bb4 size=61
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011bb4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x11bb4;
    loop {
        match pc {
        0x11bb4 => {
            // block 11bb4
            // 11bb4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11bb5 mov rbp,rsp
            rbp = rsp;
            // 11bb8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11bba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11bbb mov rbx,rdi
            rbx = rdi;
            // 11bbe mov r14,QWORD PTR [rbx+0x8]
            r14 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 11bc2 jmp 0x11bd2
            pc = 0x11bd2; continue;
        }
        0x11bc4 => {
            // block 11bc4
            // 11bc4 lea rdi,[rax-0x38]
            rdi = rax.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 11bc8 mov QWORD PTR [rbx+0x10],rdi
            m.w64(rbx.wrapping_add(0x10), (rdi) as u64);
            // 11bcc mov rax,QWORD PTR [rax-0x38]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 11bd0 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x11bd2;
        }
        0x11bd2 => {
            // block 11bd2
            // 11bd2 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 11bd6 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 11bd9 jne 0x11bc4
            if fl.ne() { pc = 0x11bc4; continue; }
            pc = 0x11bdb;
        }
        0x11bdb => {
            // block 11bdb
            // 11bdb mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 11bde test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 11be1 je 0x11bec
            if fl.e() { pc = 0x11bec; continue; }
            pc = 0x11be3;
        }
        0x11be3 => {
            // block 11be3
            // 11be3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11be4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11be6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11be7 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x11bec => {
            // block 11bec
            // 11bec pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11bed pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11bef pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11bf0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5e8c size=62
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005e8c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5e8c;
    loop {
        match pc {
        0x5e8c => {
            // block 5e8c
            // 5e8c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5e8d mov rbp,rsp
            rbp = rsp;
            // 5e90 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5e92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5e93 mov r14,rsi
            r14 = rsi;
            // 5e96 mov rbx,rdi
            rbx = rdi;
            // 5e99 mov rax,QWORD PTR [rbx+0x240]
            rax = m.r64(rbx.wrapping_add(0x240)) as u64;
            // 5ea0 test rax,rax
            fl = logic::<64>(rax & rax);
            // 5ea3 jne 0x5ebc
            if fl.ne() { pc = 0x5ebc; continue; }
            pc = 0x5ea5;
        }
        0x5ea5 => {
            // block 5ea5
            // 5ea5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5ea8 mov rdi,rbx
            rdi = rbx;
            // 5eab call QWORD PTR [rax+0xa0]
            { let r = { let t = m.r64(rax.wrapping_add(0xa0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5eb1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5eb3 je 0x5ec3
            if fl.e() { pc = 0x5ec3; continue; }
            pc = 0x5eb5;
        }
        0x5eb5 => {
            // block 5eb5
            // 5eb5 mov rax,QWORD PTR [rbx+0x240]
            rax = m.r64(rbx.wrapping_add(0x240)) as u64;
            pc = 0x5ebc;
        }
        0x5ebc => {
            // block 5ebc
            // 5ebc mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 5ebf mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 5ec1 jmp 0x5ec5
            pc = 0x5ec5; continue;
        }
        0x5ec3 => {
            // block 5ec3
            // 5ec3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5ec5;
        }
        0x5ec5 => {
            // block 5ec5
            // 5ec5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5ec6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5ec8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5ec9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xded7a size=62
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ded7a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xded7a;
    loop {
        match pc {
        0xded7a => {
            // block ded7a
            // ded7a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ded7b mov rbp,rsp
            rbp = rsp;
            // ded7e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ded80 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ded81 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // ded84 mov rbx,rdi
            rbx = rdi;
            // ded87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ded8a mov rsi,rdx
            rsi = rdx;
            // ded8d call QWORD PTR [rax+0x1930]
            { let r = { let t = m.r64(rax.wrapping_add(0x1930)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ded93 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ded96 mov rdi,rbx
            rdi = rbx;
            // ded99 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // ded9c call QWORD PTR [rax+0x1970]
            { let r = { let t = m.r64(rax.wrapping_add(0x1970)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // deda2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // deda5 mov rax,QWORD PTR [rax+0x1930]
            rax = m.r64(rax.wrapping_add(0x1930)) as u64;
            // dedac xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dedae mov rdi,rbx
            rdi = rbx;
            // dedb1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dedb2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dedb4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dedb5 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xde72 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000de72<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xde72;
    loop {
        match pc {
        0xde72 => {
            // block de72
            // de72 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // de73 mov rbp,rsp
            rbp = rsp;
            // de76 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // de78 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // de79 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // de7c mov rbx,rdi
            rbx = rdi;
            // de7f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de82 call QWORD PTR [rax+0x1728]
            { let r = { let t = m.r64(rax.wrapping_add(0x1728)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de88 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // de8a je 0xdeab
            if fl.e() { pc = 0xdeab; continue; }
            pc = 0xde8c;
        }
        0xde8c => {
            // block de8c
            // de8c movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // de90 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de93 mov esi,0x136
            rsi = (0x136u64) as u32 as u64;
            // de98 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // de9d mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // dea3 mov rdi,rbx
            rdi = rbx;
            // dea6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dea9 jmp 0xdead
            pc = 0xdead; continue;
        }
        0xdeab => {
            // block deab
            // deab xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xdead;
        }
        0xdead => {
            // block dead
            // dead pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // deae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // deb0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // deb1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x102a0 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000102a0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x102a0;
    loop {
        match pc {
        0x102a0 => {
            // block 102a0
            // 102a0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 102a1 mov rbp,rsp
            rbp = rsp;
            // 102a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 102a5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 102a6 mov rbx,rdi
            rbx = rdi;
            // 102a9 mov rax,QWORD PTR [rip+0x1300f8] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 102b0 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 102b4 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 102b7 lea rdi,[rbx+0x40]
            rdi = rbx.wrapping_add(0x40);
            // 102bb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 102c0 mov rdi,rbx
            rdi = rbx;
            // 102c3 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 102c8 mov rdi,rbx
            rdi = rbx;
            // 102cb add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 102cf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 102d0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 102d1 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11a12 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011a12<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x11a12;
    loop {
        match pc {
        0x11a12 => {
            // block 11a12
            // 11a12 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11a13 mov rbp,rsp
            rbp = rsp;
            // 11a16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11a17 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 11a18 mov rbx,rdi
            rbx = rdi;
            // 11a1b mov rax,QWORD PTR [rip+0x12e7f6] # 0x140218
            rax = m.r64(m.env.data_va(346597)) as u64;
            // 11a22 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 11a26 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 11a29 mov rdi,QWORD PTR [rbx+0x18]
            rdi = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 11a2d add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // 11a31 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 11a34 je 0x11a3b
            if fl.e() { pc = 0x11a3b; continue; }
            pc = 0x11a36;
        }
        0x11a36 => {
            // block 11a36
            // 11a36 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x11a3b;
        }
        0x11a3b => {
            // block 11a3b
            // 11a3b mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 11a43 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 11a4a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 11a4e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11a4f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11a50 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19daa size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00019daa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x19daa;
    loop {
        match pc {
        0x19daa => {
            // block 19daa
            // 19daa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19dab mov rbp,rsp
            rbp = rsp;
            // 19dae push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19db0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19db1 mov r14,rsi
            r14 = rsi;
            // 19db4 mov rbx,rdi
            rbx = rdi;
            // 19db7 mov rax,r14
            rax = r14;
            // 19dba shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19dbe je 0x19dc8
            if fl.e() { pc = 0x19dc8; continue; }
            pc = 0x19dc0;
        }
        0x19dc0 => {
            // block 19dc0
            // 19dc0 mov rdi,rbx
            rdi = rbx;
            // 19dc3 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x19dc8 => {
            // block 19dc8
            // 19dc8 lea rdi,[r14*4+0x0]
            rdi = r14.wrapping_mul(4);
            // 19dd0 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19dd5 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 19dd9 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 19ddc lea rax,[rax+r14*4]
            rax = rax.wrapping_add(r14.wrapping_mul(4));
            // 19de0 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 19de4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19de5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19de7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19de8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x297b8 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000297b8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x297b8;
    loop {
        match pc {
        0x297b8 => {
            // block 297b8
            // 297b8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 297b9 mov rbp,rsp
            rbp = rsp;
            // 297bc mov rcx,QWORD PTR [rdi+0x50]
            rcx = m.r64(rdi.wrapping_add(0x50)) as u64;
            // 297c0 mov DWORD PTR [rsi],0xffffffff
            m.w32(rsi, (0xffffffffu64) as u32);
            // 297c6 mov DWORD PTR [rsi+0x4],0xffffffff
            m.w32(rsi.wrapping_add(0x4), (0xffffffffu64) as u32);
            // 297cd mov DWORD PTR [rsi+0x8],0xffffffff
            m.w32(rsi.wrapping_add(0x8), (0xffffffffu64) as u32);
            // 297d4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 297d6 cmp edx,0x1a
            fl = sub::<32>((rdx as u32 as u64), 0x1au64).1;
            // 297d9 ja 0x297f6
            if fl.a() { pc = 0x297f6; continue; }
            pc = 0x297db;
        }
        0x297db => {
            // block 297db
            // 297db test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 297de je 0x297f6
            if fl.e() { pc = 0x297f6; continue; }
            pc = 0x297e0;
        }
        0x297e0 => {
            // block 297e0
            // 297e0 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 297e2 lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // 297e6 mov edx,DWORD PTR [rcx+rax*4+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8)) as u64) as u32 as u64;
            // 297ea mov DWORD PTR [rsi+0x8],edx
            m.w32(rsi.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 297ed mov rax,QWORD PTR [rcx+rax*4]
            rax = m.r64(rcx.wrapping_add(rax.wrapping_mul(4))) as u64;
            // 297f1 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 297f4 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x297f6;
        }
        0x297f6 => {
            // block 297f6
            // 297f6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 297f7 ret
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
            { let r = { let t = m.env.stub_va(0x5bf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // bc0e9 mov ecx,DWORD PTR [rax+0x20]
            rcx = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            pc = 0xbc0ec;
        }
        0xbc0ec => {
            // block bc0ec
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
            { let r = { let t = m.env.stub_va(0x60d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // bc129 mov ecx,DWORD PTR [rax+0x20]
            rcx = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            pc = 0xbc12c;
        }
        0xbc12c => {
            // block bc12c
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

/// parity: P2v @0xe5e42 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e5e42<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe5e42;
    loop {
        match pc {
        0xe5e42 => {
            // block e5e42
            // e5e42 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5e43 mov rbp,rsp
            rbp = rsp;
            // e5e46 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e5e48 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5e4a cmp esi,0xd
            fl = sub::<32>((rsi as u32 as u64), 0xdu64).1;
            // e5e4d ja 0xe5e5c
            if fl.a() { pc = 0xe5e5c; continue; }
            pc = 0xe5e4f;
        }
        0xe5e4f => {
            // block e5e4f
            // e5e4f movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // e5e52 lea rcx,[rip+0x43ea7] # 0x129d00
            rcx = m.env.data_va(255181);
            // e5e59 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe5e5c;
        }
        0xe5e5c => {
            // block e5e5c
            // e5e5c mov ecx,0xf0fffff
            rcx = (0xf0fffffu64) as u32 as u64;
            // e5e61 and ecx,DWORD PTR [rdi+0x84]
            { let r = (rcx as u32 as u64) & m.r32(rdi.wrapping_add(0x84)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e5e67 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // e5e69 and edx,0xd
            { let r = (rdx as u32 as u64) & 0xdu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5e6c shl edx,0x14
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x14u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e5e6f or edx,ecx
            { let r = (rdx as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5e71 and eax,0x70
            { let r = (rax as u32 as u64) & 0x70u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5e74 shl eax,0x18
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x18u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5e77 or eax,edx
            { let r = (rax as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5e79 mov DWORD PTR [rdi+0x84],eax
            m.w32(rdi.wrapping_add(0x84), ((rax as u32 as u64)) as u32);
            // e5e7f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5e80 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdd6c size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000dd6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdd6c;
    loop {
        match pc {
        0xdd6c => {
            // block dd6c
            // dd6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dd6d mov rbp,rsp
            rbp = rsp;
            // dd70 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dd72 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dd73 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // dd76 mov rbx,rdi
            rbx = rdi;
            // dd79 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dd7c call QWORD PTR [rax+0x1728]
            { let r = { let t = m.r64(rax.wrapping_add(0x1728)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd82 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dd84 je 0xdda8
            if fl.e() { pc = 0xdda8; continue; }
            pc = 0xdd86;
        }
        0xdd86 => {
            // block dd86
            // dd86 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // dd8a sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // dd8d movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // dd90 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dd93 mov esi,0x136
            rsi = (0x136u64) as u32 as u64;
            // dd98 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dd9d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dda0 mov rdi,rbx
            rdi = rbx;
            // dda3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dda6 jmp 0xddaa
            pc = 0xddaa; continue;
        }
        0xdda8 => {
            // block dda8
            // dda8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xddaa;
        }
        0xddaa => {
            // block ddaa
            // ddaa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ddab pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ddad pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ddae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28ce size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000028ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28ce;
    loop {
        match pc {
        0x28ce => {
            // block 28ce
            // 28ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28cf mov rbp,rsp
            rbp = rsp;
            // 28d2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28d4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28d5 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 28d8 mov rbx,rdi
            rbx = rdi;
            // 28db mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28de call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e4 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 28e6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 28e8 cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // 28ec ja 0x290f
            if fl.a() { pc = 0x290f; continue; }
            pc = 0x28ee;
        }
        0x28ee => {
            // block 28ee
            // 28ee xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 28f1 je 0x290f
            if fl.e() { pc = 0x290f; continue; }
            pc = 0x28f3;
        }
        0x28f3 => {
            // block 28f3
            // 28f3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28f6 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 28fb mov ecx,0x300000
            rcx = (0x300000u64) as u32 as u64;
            // 2900 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 2906 mov rdi,rbx
            rdi = rbx;
            // 2909 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 290c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x290f;
        }
        0x290f => {
            // block 290f
            // 290f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2910 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2912 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2913 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xddf0 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ddf0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xddf0;
    loop {
        match pc {
        0xddf0 => {
            // block ddf0
            // ddf0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ddf1 mov rbp,rsp
            rbp = rsp;
            // ddf4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ddf6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ddf7 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // ddfa mov rbx,rdi
            rbx = rdi;
            // ddfd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de00 call QWORD PTR [rax+0x1728]
            { let r = { let t = m.r64(rax.wrapping_add(0x1728)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de06 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // de08 je 0xde2f
            if fl.e() { pc = 0xde2f; continue; }
            pc = 0xde0a;
        }
        0xde0a => {
            // block de0a
            // de0a cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // de0e sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // de11 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // de14 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // de17 mov esi,0x136
            rsi = (0x136u64) as u32 as u64;
            // de1c mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // de21 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // de27 mov rdi,rbx
            rdi = rbx;
            // de2a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de2d jmp 0xde31
            pc = 0xde31; continue;
        }
        0xde2f => {
            // block de2f
            // de2f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xde31;
        }
        0xde31 => {
            // block de31
            // de31 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // de32 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // de34 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // de35 ret
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
            // e0a51 lea rdi,[rax-0x38]
            rdi = rax.wrapping_add(0xffff_ffff_ffff_ffc8);
            // e0a55 mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // e0a59 mov rax,QWORD PTR [rax-0x38]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e0a5d call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // e0a68 mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            pc = 0xe0a6b;
        }
        0xe0a6b => {
            // block e0a6b
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

/// parity: P2v @0xdc1e size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000dc1e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdc1e;
    loop {
        match pc {
        0xdc1e => {
            // block dc1e
            // dc1e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc1f mov rbp,rsp
            rbp = rsp;
            // dc22 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dc24 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dc25 mov rbx,rdi
            rbx = rdi;
            // dc28 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dc2b mov esi,0x139
            rsi = (0x139u64) as u32 as u64;
            // dc30 mov edx,0x1234567
            rdx = (0x1234567u64) as u32 as u64;
            // dc35 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dc3a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dc3d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc40 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // dc43 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dc46 mov esi,0x138
            rsi = (0x138u64) as u32 as u64;
            // dc4b mov edx,0xa5a55a5a
            rdx = (0xa5a55a5au64) as u32 as u64;
            // dc50 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // dc55 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dc58 mov rdi,rbx
            rdi = rbx;
            // dc5b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc5e and al,r14b
            { let r = (rax as u8 as u64) & (r14 as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // dc61 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc62 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc64 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc65 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d83c size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d83c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2d83c;
    loop {
        match pc {
        0x2d83c => {
            // block 2d83c
            // 2d83c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d83d mov rbp,rsp
            rbp = rsp;
            // 2d840 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2d844 mov rcx,QWORD PTR [rdi+0x10]
            rcx = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 2d848 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 2d84b je 0x2d874
            if fl.e() { pc = 0x2d874; continue; }
            pc = 0x2d84d;
        }
        0x2d84d => {
            // block 2d84d
            // 2d84d lea rax,[rcx-0xc]
            rax = rcx.wrapping_add(0xffff_ffff_ffff_fff4);
            // 2d851 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 2d854 movabs rdx,0xaaaaaaaaaaaaaaab
            rdx = 0xaaaa_aaaa_aaaa_aaabu64;
            // 2d85e mul rdx
            { let (lo, hi, f) = mul1::<64>(rax, rdx); rax = lo; rdx = hi; fl = f; }
            // 2d861 shr rdx,0x3
            { let (r, f) = shr::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // 2d865 not rdx
            rdx = !rdx;
            // 2d868 lea rax,[rdx+rdx*2]
            rax = rdx.wrapping_add(rdx.wrapping_mul(2));
            // 2d86c lea rax,[rcx+rax*4]
            rax = rcx.wrapping_add(rax.wrapping_mul(4));
            // 2d870 mov QWORD PTR [rdi+0x10],rax
            m.w64(rdi.wrapping_add(0x10), (rax) as u64);
            pc = 0x2d874;
        }
        0x2d874 => {
            // block 2d874
            // 2d874 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 2d877 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2d87a je 0x2d882
            if fl.e() { pc = 0x2d882; continue; }
            pc = 0x2d87c;
        }
        0x2d87c => {
            // block 2d87c
            // 2d87c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d87d jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2d882 => {
            // block 2d882
            // 2d882 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d883 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43144 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043144<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43144;
    loop {
        match pc {
        0x43144 => {
            // block 43144
            // 43144 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43145 mov rbp,rsp
            rbp = rsp;
            // 43148 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4314a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4314b mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 4314e mov rbx,rdi
            rbx = rdi;
            // 43151 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 43154 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4315a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4315c je 0x43185
            if fl.e() { pc = 0x43185; continue; }
            pc = 0x4315e;
        }
        0x4315e => {
            // block 4315e
            // 4315e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 43161 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 43164 sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 43167 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 4316a lea edx,[rcx+rcx*2]
            rdx = (rcx.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // 4316d mov esi,0x905
            rsi = (0x905u64) as u32 as u64;
            // 43172 mov ecx,0xc0000
            rcx = (0xc0000u64) as u32 as u64;
            // 43177 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 4317d mov rdi,rbx
            rdi = rbx;
            // 43180 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43183 jmp 0x43187
            pc = 0x43187; continue;
        }
        0x43185 => {
            // block 43185
            // 43185 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x43187;
        }
        0x43187 => {
            // block 43187
            // 43187 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43188 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4318a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4318b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5cf4c size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005cf4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5cf4c;
    loop {
        match pc {
        0x5cf4c => {
            // block 5cf4c
            // 5cf4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5cf4d mov rbp,rsp
            rbp = rsp;
            // 5cf50 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5cf51 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5cf52 mov rbx,rsi
            rbx = rsi;
            // 5cf55 shr rbx,0x20
            { let (r, f) = shr::<64>(rbx, 0x20u64, fl); rbx = r; fl = f; }
            // 5cf59 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5cf5e lea rsi,[rip+0xcec87] # 0x12bbec
            rsi = m.env.data_va(263097);
            // 5cf65 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 5cf6a mov rdi,rax
            rdi = rax;
            // 5cf6d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5cf72 mov rdi,rax
            rdi = rax;
            // 5cf75 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 5cf77 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5cf7c lea rsi,[rip+0xcec6c] # 0x12bbef
            rsi = m.env.data_va(263100);
            // 5cf83 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 5cf88 mov rdi,rax
            rdi = rax;
            // 5cf8b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5cf8f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5cf90 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5cf91 jmp 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5bc0 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5bc0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd5bc0;
    loop {
        match pc {
        0xd5bc0 => {
            // block d5bc0
            // d5bc0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5bc1 mov rbp,rsp
            rbp = rsp;
            // d5bc4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d5bc6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5bc7 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d5bca mov rbx,rdi
            rbx = rdi;
            // d5bcd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5bd0 call QWORD PTR [rax+0x100]
            { let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5bd6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5bd8 je 0xd5bff
            if fl.e() { pc = 0xd5bff; continue; }
            pc = 0xd5bda;
        }
        0xd5bda => {
            // block d5bda
            // d5bda mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5bdd mov rdi,rbx
            rdi = rbx;
            // d5be0 call QWORD PTR [rax+0xf8]
            { let r = { let t = m.r64(rax.wrapping_add(0xf8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5be6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5be8 je 0xd5c03
            if fl.e() { pc = 0xd5c03; continue; }
            pc = 0xd5bea;
        }
        0xd5bea => {
            // block d5bea
            // d5bea test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // d5bed je 0xd5bff
            if fl.e() { pc = 0xd5bff; continue; }
            pc = 0xd5bef;
        }
        0xd5bef => {
            // block d5bef
            // d5bef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5bf2 mov rdi,rbx
            rdi = rbx;
            // d5bf5 call QWORD PTR [rax+0xf0]
            { let r = { let t = m.r64(rax.wrapping_add(0xf0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5bfb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5bfd je 0xd5c03
            if fl.e() { pc = 0xd5c03; continue; }
            pc = 0xd5bff;
        }
        0xd5bff => {
            // block d5bff
            // d5bff mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d5c01 jmp 0xd5c05
            pc = 0xd5c05; continue;
        }
        0xd5c03 => {
            // block d5c03
            // d5c03 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd5c05;
        }
        0xd5c05 => {
            // block d5c05
            // d5c05 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5c06 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d5c08 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5c09 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
