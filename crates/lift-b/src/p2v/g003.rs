//! Mechanically lifted functions (scripts/lift_run.py), module g003. Each one is checked against the original
//! in the world harness (tests/world_g003.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xc4cfc size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4cfc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4cfc;
    loop {
        match pc {
        0xc4cfc => {
            // block c4cfc
            // c4cfc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4cfd mov rbp,rsp
            rbp = rsp;
            // c4d00 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c4d04 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c4d08 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4d0b je 0xc4d3f
            if fl.e() { pc = 0xc4d3f; continue; }
            pc = 0xc4d0d;
        }
        0xc4d0d => {
            // block c4d0d
            // c4d0d mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // c4d0f mov rsi,rdi
            rsi = rdi;
            pc = 0xc4d12;
        }
        0xc4d12 => {
            // block c4d12
            // c4d12 mov rax,rdx
            rax = rdx;
            pc = 0xc4d15;
        }
        0xc4d15 => {
            // block c4d15
            // c4d15 cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // c4d18 jge 0xc4d25
            if fl.ge() { pc = 0xc4d25; continue; }
            pc = 0xc4d1a;
        }
        0xc4d1a => {
            // block c4d1a
            // c4d1a mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // c4d1e test rax,rax
            fl = logic::<64>(rax & rax);
            // c4d21 jne 0xc4d15
            if fl.ne() { pc = 0xc4d15; continue; }
            pc = 0xc4d23;
        }
        0xc4d23 => {
            // block c4d23
            // c4d23 jmp 0xc4d32
            pc = 0xc4d32; continue;
        }
        0xc4d25 => {
            // block c4d25
            // c4d25 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c4d28 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4d2b mov rsi,rax
            rsi = rax;
            // c4d2e jne 0xc4d12
            if fl.ne() { pc = 0xc4d12; continue; }
            pc = 0xc4d30;
        }
        0xc4d30 => {
            // block c4d30
            // c4d30 jmp 0xc4d35
            pc = 0xc4d35; continue;
        }
        0xc4d32 => {
            // block c4d32
            // c4d32 mov rax,rsi
            rax = rsi;
            pc = 0xc4d35;
        }
        0xc4d35 => {
            // block c4d35
            // c4d35 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // c4d38 je 0xc4d3f
            if fl.e() { pc = 0xc4d3f; continue; }
            pc = 0xc4d3a;
        }
        0xc4d3a => {
            // block c4d3a
            // c4d3a cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // c4d3d jge 0xc4d42
            if fl.ge() { pc = 0xc4d42; continue; }
            pc = 0xc4d3f;
        }
        0xc4d3f => {
            // block c4d3f
            // c4d3f mov rax,rdi
            rax = rdi;
            pc = 0xc4d42;
        }
        0xc4d42 => {
            // block c4d42
            // c4d42 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4d43 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4fc6 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4fc6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4fc6;
    loop {
        match pc {
        0xc4fc6 => {
            // block c4fc6
            // c4fc6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4fc7 mov rbp,rsp
            rbp = rsp;
            // c4fca mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c4fce add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c4fd2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4fd5 je 0xc5009
            if fl.e() { pc = 0xc5009; continue; }
            pc = 0xc4fd7;
        }
        0xc4fd7 => {
            // block c4fd7
            // c4fd7 mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // c4fd9 mov rsi,rdi
            rsi = rdi;
            pc = 0xc4fdc;
        }
        0xc4fdc => {
            // block c4fdc
            // c4fdc mov rax,rdx
            rax = rdx;
            pc = 0xc4fdf;
        }
        0xc4fdf => {
            // block c4fdf
            // c4fdf cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // c4fe2 jge 0xc4fef
            if fl.ge() { pc = 0xc4fef; continue; }
            pc = 0xc4fe4;
        }
        0xc4fe4 => {
            // block c4fe4
            // c4fe4 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // c4fe8 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4feb jne 0xc4fdf
            if fl.ne() { pc = 0xc4fdf; continue; }
            pc = 0xc4fed;
        }
        0xc4fed => {
            // block c4fed
            // c4fed jmp 0xc4ffc
            pc = 0xc4ffc; continue;
        }
        0xc4fef => {
            // block c4fef
            // c4fef mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c4ff2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4ff5 mov rsi,rax
            rsi = rax;
            // c4ff8 jne 0xc4fdc
            if fl.ne() { pc = 0xc4fdc; continue; }
            pc = 0xc4ffa;
        }
        0xc4ffa => {
            // block c4ffa
            // c4ffa jmp 0xc4fff
            pc = 0xc4fff; continue;
        }
        0xc4ffc => {
            // block c4ffc
            // c4ffc mov rax,rsi
            rax = rsi;
            pc = 0xc4fff;
        }
        0xc4fff => {
            // block c4fff
            // c4fff cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // c5002 je 0xc5009
            if fl.e() { pc = 0xc5009; continue; }
            pc = 0xc5004;
        }
        0xc5004 => {
            // block c5004
            // c5004 cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // c5007 jge 0xc500c
            if fl.ge() { pc = 0xc500c; continue; }
            pc = 0xc5009;
        }
        0xc5009 => {
            // block c5009
            // c5009 mov rax,rdi
            rax = rdi;
            pc = 0xc500c;
        }
        0xc500c => {
            // block c500c
            // c500c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c500d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6536 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6536<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb6536;
    loop {
        match pc {
        0xb6536 => {
            // block b6536
            // b6536 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6537 mov rbp,rsp
            rbp = rsp;
            // b653a mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b653e add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b6542 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b6545 je 0xb6579
            if fl.e() { pc = 0xb6579; continue; }
            pc = 0xb6547;
        }
        0xb6547 => {
            // block b6547
            // b6547 mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // b6549 mov rsi,rdi
            rsi = rdi;
            pc = 0xb654c;
        }
        0xb654c => {
            // block b654c
            // b654c mov rax,rdx
            rax = rdx;
            pc = 0xb654f;
        }
        0xb654f => {
            // block b654f
            // b654f cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // b6552 jae 0xb655f
            if fl.ae() { pc = 0xb655f; continue; }
            pc = 0xb6554;
        }
        0xb6554 => {
            // block b6554
            // b6554 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // b6558 test rax,rax
            fl = logic::<64>(rax & rax);
            // b655b jne 0xb654f
            if fl.ne() { pc = 0xb654f; continue; }
            pc = 0xb655d;
        }
        0xb655d => {
            // block b655d
            // b655d jmp 0xb656c
            pc = 0xb656c; continue;
        }
        0xb655f => {
            // block b655f
            // b655f mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // b6562 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b6565 mov rsi,rax
            rsi = rax;
            // b6568 jne 0xb654c
            if fl.ne() { pc = 0xb654c; continue; }
            pc = 0xb656a;
        }
        0xb656a => {
            // block b656a
            // b656a jmp 0xb656f
            pc = 0xb656f; continue;
        }
        0xb656c => {
            // block b656c
            // b656c mov rax,rsi
            rax = rsi;
            pc = 0xb656f;
        }
        0xb656f => {
            // block b656f
            // b656f cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // b6572 je 0xb6579
            if fl.e() { pc = 0xb6579; continue; }
            pc = 0xb6574;
        }
        0xb6574 => {
            // block b6574
            // b6574 cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // b6577 jae 0xb657c
            if fl.ae() { pc = 0xb657c; continue; }
            pc = 0xb6579;
        }
        0xb6579 => {
            // block b6579
            // b6579 mov rax,rdi
            rax = rdi;
            pc = 0xb657c;
        }
        0xb657c => {
            // block b657c
            // b657c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b657d ret
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
            // 2db5f mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 2db61 jmp 0x2db66
            pc = 0x2db66; continue;
        }
        0x2db63 => {
            // block 2db63
            // 2db63 mov rax,rdx
            rax = rdx;
            pc = 0x2db66;
        }
        0x2db66 => {
            // block 2db66
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
            // 2db75 jmp 0x2db9c
            pc = 0x2db9c; continue;
        }
        0x2db77 => {
            // block 2db77
            // 2db77 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 2db79 jge 0x2dba4
            if fl.ge() { pc = 0x2dba4; continue; }
            pc = 0x2db7b;
        }
        0x2db7b => {
            // block 2db7b
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
            // 2db9c mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2db9f mov rsi,rax
            rsi = rax;
            // 2dba2 jmp 0x2dba7
            pc = 0x2dba7; continue;
        }
        0x2dba4 => {
            // block 2dba4
            // 2dba4 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x2dba7;
        }
        0x2dba7 => {
            // block 2dba7
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
            // 44b6b mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 44b6d jmp 0x44b72
            pc = 0x44b72; continue;
        }
        0x44b6f => {
            // block 44b6f
            // 44b6f mov rax,rdx
            rax = rdx;
            pc = 0x44b72;
        }
        0x44b72 => {
            // block 44b72
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
            // 44b81 jmp 0x44ba8
            pc = 0x44ba8; continue;
        }
        0x44b83 => {
            // block 44b83
            // 44b83 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 44b85 jge 0x44bb0
            if fl.ge() { pc = 0x44bb0; continue; }
            pc = 0x44b87;
        }
        0x44b87 => {
            // block 44b87
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
            // 44ba8 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 44bab mov rsi,rax
            rsi = rax;
            // 44bae jmp 0x44bb3
            pc = 0x44bb3; continue;
        }
        0x44bb0 => {
            // block 44bb0
            // 44bb0 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x44bb3;
        }
        0x44bb3 => {
            // block 44bb3
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

/// parity: P2v @0xb522e size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b522e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb522e;
    loop {
        match pc {
        0xb522e => {
            // block b522e
            // b522e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b522f mov rbp,rsp
            rbp = rsp;
            // b5232 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b5236 test rax,rax
            fl = logic::<64>(rax & rax);
            // b5239 je 0xb526c
            if fl.e() { pc = 0xb526c; continue; }
            pc = 0xb523b;
        }
        0xb523b => {
            // block b523b
            // b523b mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // b523d jmp 0xb5242
            pc = 0xb5242; continue;
        }
        0xb523f => {
            // block b523f
            // b523f mov rax,rdx
            rax = rdx;
            pc = 0xb5242;
        }
        0xb5242 => {
            // block b5242
            // b5242 mov edx,DWORD PTR [rax+0x20]
            rdx = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            // b5245 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // b5247 jae 0xb5253
            if fl.ae() { pc = 0xb5253; continue; }
            pc = 0xb5249;
        }
        0xb5249 => {
            // block b5249
            // b5249 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // b524c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b524f jne 0xb523f
            if fl.ne() { pc = 0xb523f; continue; }
            pc = 0xb5251;
        }
        0xb5251 => {
            // block b5251
            // b5251 jmp 0xb5278
            pc = 0xb5278; continue;
        }
        0xb5253 => {
            // block b5253
            // b5253 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // b5255 jae 0xb5280
            if fl.ae() { pc = 0xb5280; continue; }
            pc = 0xb5257;
        }
        0xb5257 => {
            // block b5257
            // b5257 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // b525b test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b525e jne 0xb523f
            if fl.ne() { pc = 0xb523f; continue; }
            pc = 0xb5260;
        }
        0xb5260 => {
            // block b5260
            // b5260 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // b5263 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // b5267 mov rsi,rax
            rsi = rax;
            // b526a jmp 0xb5283
            pc = 0xb5283; continue;
        }
        0xb526c => {
            // block b526c
            // b526c add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b5270 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // b5273 mov rsi,rdi
            rsi = rdi;
            // b5276 jmp 0xb5283
            pc = 0xb5283; continue;
        }
        0xb5278 => {
            // block b5278
            // b5278 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // b527b mov rsi,rax
            rsi = rax;
            // b527e jmp 0xb5283
            pc = 0xb5283; continue;
        }
        0xb5280 => {
            // block b5280
            // b5280 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xb5283;
        }
        0xb5283 => {
            // block b5283
            // b5283 mov rax,rsi
            rax = rsi;
            // b5286 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5287 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb593e size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b593e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb593e;
    loop {
        match pc {
        0xb593e => {
            // block b593e
            // b593e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b593f mov rbp,rsp
            rbp = rsp;
            // b5942 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b5946 test rax,rax
            fl = logic::<64>(rax & rax);
            // b5949 je 0xb597c
            if fl.e() { pc = 0xb597c; continue; }
            pc = 0xb594b;
        }
        0xb594b => {
            // block b594b
            // b594b mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // b594d jmp 0xb5952
            pc = 0xb5952; continue;
        }
        0xb594f => {
            // block b594f
            // b594f mov rax,rdx
            rax = rdx;
            pc = 0xb5952;
        }
        0xb5952 => {
            // block b5952
            // b5952 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b5955 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // b5957 jae 0xb5963
            if fl.ae() { pc = 0xb5963; continue; }
            pc = 0xb5959;
        }
        0xb5959 => {
            // block b5959
            // b5959 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // b595c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b595f jne 0xb594f
            if fl.ne() { pc = 0xb594f; continue; }
            pc = 0xb5961;
        }
        0xb5961 => {
            // block b5961
            // b5961 jmp 0xb5988
            pc = 0xb5988; continue;
        }
        0xb5963 => {
            // block b5963
            // b5963 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // b5965 jae 0xb5990
            if fl.ae() { pc = 0xb5990; continue; }
            pc = 0xb5967;
        }
        0xb5967 => {
            // block b5967
            // b5967 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // b596b test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b596e jne 0xb594f
            if fl.ne() { pc = 0xb594f; continue; }
            pc = 0xb5970;
        }
        0xb5970 => {
            // block b5970
            // b5970 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // b5973 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // b5977 mov rsi,rax
            rsi = rax;
            // b597a jmp 0xb5993
            pc = 0xb5993; continue;
        }
        0xb597c => {
            // block b597c
            // b597c add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b5980 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // b5983 mov rsi,rdi
            rsi = rdi;
            // b5986 jmp 0xb5993
            pc = 0xb5993; continue;
        }
        0xb5988 => {
            // block b5988
            // b5988 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // b598b mov rsi,rax
            rsi = rax;
            // b598e jmp 0xb5993
            pc = 0xb5993; continue;
        }
        0xb5990 => {
            // block b5990
            // b5990 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xb5993;
        }
        0xb5993 => {
            // block b5993
            // b5993 mov rax,rsi
            rax = rsi;
            // b5996 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5997 ret
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
            // c43d1 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c43d3 jmp 0xc43d8
            pc = 0xc43d8; continue;
        }
        0xc43d5 => {
            // block c43d5
            // c43d5 mov rax,rdx
            rax = rdx;
            pc = 0xc43d8;
        }
        0xc43d8 => {
            // block c43d8
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
            // c43e7 jmp 0xc440e
            pc = 0xc440e; continue;
        }
        0xc43e9 => {
            // block c43e9
            // c43e9 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c43eb jae 0xc4416
            if fl.ae() { pc = 0xc4416; continue; }
            pc = 0xc43ed;
        }
        0xc43ed => {
            // block c43ed
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
            // c440e mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c4411 mov rsi,rax
            rsi = rax;
            // c4414 jmp 0xc4419
            pc = 0xc4419; continue;
        }
        0xc4416 => {
            // block c4416
            // c4416 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc4419;
        }
        0xc4419 => {
            // block c4419
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
            // c4db9 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c4dbb jmp 0xc4dc0
            pc = 0xc4dc0; continue;
        }
        0xc4dbd => {
            // block c4dbd
            // c4dbd mov rax,rdx
            rax = rdx;
            pc = 0xc4dc0;
        }
        0xc4dc0 => {
            // block c4dc0
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
            // c4dcf jmp 0xc4df6
            pc = 0xc4df6; continue;
        }
        0xc4dd1 => {
            // block c4dd1
            // c4dd1 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c4dd3 jae 0xc4dfe
            if fl.ae() { pc = 0xc4dfe; continue; }
            pc = 0xc4dd5;
        }
        0xc4dd5 => {
            // block c4dd5
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
            // c4df6 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c4df9 mov rsi,rax
            rsi = rax;
            // c4dfc jmp 0xc4e01
            pc = 0xc4e01; continue;
        }
        0xc4dfe => {
            // block c4dfe
            // c4dfe mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc4e01;
        }
        0xc4e01 => {
            // block c4e01
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
            // c5125 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c5127 jmp 0xc512c
            pc = 0xc512c; continue;
        }
        0xc5129 => {
            // block c5129
            // c5129 mov rax,rdx
            rax = rdx;
            pc = 0xc512c;
        }
        0xc512c => {
            // block c512c
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
            // c513b jmp 0xc5162
            pc = 0xc5162; continue;
        }
        0xc513d => {
            // block c513d
            // c513d cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c513f jae 0xc516a
            if fl.ae() { pc = 0xc516a; continue; }
            pc = 0xc5141;
        }
        0xc5141 => {
            // block c5141
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
            // c5162 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c5165 mov rsi,rax
            rsi = rax;
            // c5168 jmp 0xc516d
            pc = 0xc516d; continue;
        }
        0xc516a => {
            // block c516a
            // c516a mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc516d;
        }
        0xc516d => {
            // block c516d
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
            // c56cd mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c56cf jmp 0xc56d4
            pc = 0xc56d4; continue;
        }
        0xc56d1 => {
            // block c56d1
            // c56d1 mov rax,rdx
            rax = rdx;
            pc = 0xc56d4;
        }
        0xc56d4 => {
            // block c56d4
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
            // c56e3 jmp 0xc570a
            pc = 0xc570a; continue;
        }
        0xc56e5 => {
            // block c56e5
            // c56e5 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c56e7 jae 0xc5712
            if fl.ae() { pc = 0xc5712; continue; }
            pc = 0xc56e9;
        }
        0xc56e9 => {
            // block c56e9
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
            // c570a mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c570d mov rsi,rax
            rsi = rax;
            // c5710 jmp 0xc5715
            pc = 0xc5715; continue;
        }
        0xc5712 => {
            // block c5712
            // c5712 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc5715;
        }
        0xc5715 => {
            // block c5715
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
            // d49b5 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // d49b7 jmp 0xd49bc
            pc = 0xd49bc; continue;
        }
        0xd49b9 => {
            // block d49b9
            // d49b9 mov rax,rdx
            rax = rdx;
            pc = 0xd49bc;
        }
        0xd49bc => {
            // block d49bc
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
            // d49cb jmp 0xd49f2
            pc = 0xd49f2; continue;
        }
        0xd49cd => {
            // block d49cd
            // d49cd cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // d49cf jge 0xd49fa
            if fl.ge() { pc = 0xd49fa; continue; }
            pc = 0xd49d1;
        }
        0xd49d1 => {
            // block d49d1
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
            // d49f2 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // d49f5 mov rsi,rax
            rsi = rax;
            // d49f8 jmp 0xd49fd
            pc = 0xd49fd; continue;
        }
        0xd49fa => {
            // block d49fa
            // d49fa mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xd49fd;
        }
        0xd49fd => {
            // block d49fd
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
            // 25d0d mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 25d0f jmp 0x25d14
            pc = 0x25d14; continue;
        }
        0x25d11 => {
            // block 25d11
            // 25d11 mov rax,rdx
            rax = rdx;
            pc = 0x25d14;
        }
        0x25d14 => {
            // block 25d14
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
            // 25d23 jmp 0x25d4a
            pc = 0x25d4a; continue;
        }
        0x25d25 => {
            // block 25d25
            // 25d25 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 25d27 jge 0x25d52
            if fl.ge() { pc = 0x25d52; continue; }
            pc = 0x25d29;
        }
        0x25d29 => {
            // block 25d29
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
            // 25d4a mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 25d4d mov rsi,rax
            rsi = rax;
            // 25d50 jmp 0x25d55
            pc = 0x25d55; continue;
        }
        0x25d52 => {
            // block 25d52
            // 25d52 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x25d55;
        }
        0x25d55 => {
            // block 25d55
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
            // 2dca9 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 2dcab jmp 0x2dcb0
            pc = 0x2dcb0; continue;
        }
        0x2dcad => {
            // block 2dcad
            // 2dcad mov rax,rdx
            rax = rdx;
            pc = 0x2dcb0;
        }
        0x2dcb0 => {
            // block 2dcb0
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
            // 2dcbf jmp 0x2dce6
            pc = 0x2dce6; continue;
        }
        0x2dcc1 => {
            // block 2dcc1
            // 2dcc1 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 2dcc3 jae 0x2dcee
            if fl.ae() { pc = 0x2dcee; continue; }
            pc = 0x2dcc5;
        }
        0x2dcc5 => {
            // block 2dcc5
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
            // 2dce6 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2dce9 mov rsi,rax
            rsi = rax;
            // 2dcec jmp 0x2dcf1
            pc = 0x2dcf1; continue;
        }
        0x2dcee => {
            // block 2dcee
            // 2dcee mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x2dcf1;
        }
        0x2dcf1 => {
            // block 2dcf1
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
            // b661b mov r9,rax
            r9 = rax;
            // b661e mov rax,r8
            rax = r8;
            pc = 0xb6621;
        }
        0xb6621 => {
            // block b6621
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
            // b6628 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // b662a jb 0xb663c
            if fl.b() { pc = 0xb663c; continue; }
            pc = 0xb662c;
        }
        0xb662c => {
            // block b662c
            // b662c cmp DWORD PTR [rax+0x20],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x20)) as u64, (rcx as u32 as u64)).1;
            // b662f jae 0xb663c
            if fl.ae() { pc = 0xb663c; continue; }
            pc = 0xb6631;
        }
        0xb6631 => {
            // block b6631
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
            // b663a jmp 0xb6646
            pc = 0xb6646; continue;
        }
        0xb663c => {
            // block b663c
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
            // b6644 jmp 0xb6649
            pc = 0xb6649; continue;
        }
        0xb6646 => {
            // block b6646
            // b6646 mov rax,r9
            rax = r9;
            pc = 0xb6649;
        }
        0xb6649 => {
            // block b6649
            // b6649 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // b664c je 0xb665e
            if fl.e() { pc = 0xb665e; continue; }
            pc = 0xb664e;
        }
        0xb664e => {
            // block b664e
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
            // b6655 cmp esi,edx
            fl = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64)).1;
            // b6657 jb 0xb6661
            if fl.b() { pc = 0xb6661; continue; }
            pc = 0xb6659;
        }
        0xb6659 => {
            // block b6659
            // b6659 cmp ecx,DWORD PTR [rax+0x20]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x20)) as u64).1;
            // b665c jae 0xb6661
            if fl.ae() { pc = 0xb6661; continue; }
            pc = 0xb665e;
        }
        0xb665e => {
            // block b665e
            // b665e mov rax,rdi
            rax = rdi;
            pc = 0xb6661;
        }
        0xb6661 => {
            // block b6661
            // b6661 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6662 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d9d8 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d9d8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2d9d8;
    loop {
        match pc {
        0x2d9d8 => {
            // block 2d9d8
            // 2d9d8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d9d9 mov rbp,rsp
            rbp = rsp;
            // 2d9dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d9dd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2d9de mov rbx,rdi
            rbx = rdi;
            // 2d9e1 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 2d9e9 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 2d9f0 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // 2d9f4 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // 2d9f7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2d9fa cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // 2d9fe je 0x2da09
            if fl.e() { pc = 0x2da09; continue; }
            pc = 0x2da00;
        }
        0x2da00 => {
            // block 2da00
            // 2da00 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 2da03 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 2da06 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0x2da09;
        }
        0x2da09 => {
            // block 2da09
            // 2da09 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2da0d mov rsi,rcx
            rsi = rcx;
            // 2da10 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2da15 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // 2da19 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2da1d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2da1e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2da1f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5288 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5288<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5288;
    loop {
        match pc {
        0xb5288 => {
            // block b5288
            // b5288 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5289 mov rbp,rsp
            rbp = rsp;
            // b528c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b528d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b528e mov rbx,rdi
            rbx = rdi;
            // b5291 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // b5299 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // b52a0 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // b52a4 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // b52a7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b52aa cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // b52ae je 0xb52b9
            if fl.e() { pc = 0xb52b9; continue; }
            pc = 0xb52b0;
        }
        0xb52b0 => {
            // block b52b0
            // b52b0 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b52b3 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b52b6 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xb52b9;
        }
        0xb52b9 => {
            // block b52b9
            // b52b9 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b52bd mov rsi,rcx
            rsi = rcx;
            // b52c0 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b52c5 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // b52c9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b52cd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b52ce pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b52cf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5646 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5646<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5646;
    loop {
        match pc {
        0xb5646 => {
            // block b5646
            // b5646 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5647 mov rbp,rsp
            rbp = rsp;
            // b564a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b564b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b564c mov rbx,rdi
            rbx = rdi;
            // b564f mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // b5657 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // b565e mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // b5662 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // b5665 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b5668 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // b566c je 0xb5677
            if fl.e() { pc = 0xb5677; continue; }
            pc = 0xb566e;
        }
        0xb566e => {
            // block b566e
            // b566e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b5671 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b5674 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xb5677;
        }
        0xb5677 => {
            // block b5677
            // b5677 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b567b mov rsi,rcx
            rsi = rcx;
            // b567e call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5683 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // b5687 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b568b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b568c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b568d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb613a size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b613a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb613a;
    loop {
        match pc {
        0xb613a => {
            // block b613a
            // b613a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b613b mov rbp,rsp
            rbp = rsp;
            // b613e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b613f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b6140 mov rbx,rdi
            rbx = rdi;
            // b6143 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // b614b mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // b6152 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // b6156 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // b6159 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b615c cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // b6160 je 0xb616b
            if fl.e() { pc = 0xb616b; continue; }
            pc = 0xb6162;
        }
        0xb6162 => {
            // block b6162
            // b6162 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b6165 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b6168 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xb616b;
        }
        0xb616b => {
            // block b616b
            // b616b mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b616f mov rsi,rcx
            rsi = rcx;
            // b6172 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6177 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // b617b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b617f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b6180 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6181 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc5172 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c5172<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc5172;
    loop {
        match pc {
        0xc5172 => {
            // block c5172
            // c5172 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c5173 mov rbp,rsp
            rbp = rsp;
            // c5176 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c5177 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c5178 mov rbx,rdi
            rbx = rdi;
            // c517b mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c5183 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c518a mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c518e mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c5191 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c5194 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c5198 je 0xc51a3
            if fl.e() { pc = 0xc51a3; continue; }
            pc = 0xc519a;
        }
        0xc519a => {
            // block c519a
            // c519a mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c519d mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c51a0 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc51a3;
        }
        0xc51a3 => {
            // block c51a3
            // c51a3 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c51a7 mov rsi,rcx
            rsi = rcx;
            // c51aa call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c51af inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c51b3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c51b7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c51b8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c51b9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc5446 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c5446<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc5446;
    loop {
        match pc {
        0xc5446 => {
            // block c5446
            // c5446 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c5447 mov rbp,rsp
            rbp = rsp;
            // c544a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c544b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c544c mov rbx,rdi
            rbx = rdi;
            // c544f mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c5457 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c545e mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c5462 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c5465 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c5468 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c546c je 0xc5477
            if fl.e() { pc = 0xc5477; continue; }
            pc = 0xc546e;
        }
        0xc546e => {
            // block c546e
            // c546e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c5471 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c5474 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc5477;
        }
        0xc5477 => {
            // block c5477
            // c5477 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c547b mov rsi,rcx
            rsi = rcx;
            // c547e call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5483 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c5487 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c548b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c548c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c548d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc571a size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c571a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc571a;
    loop {
        match pc {
        0xc571a => {
            // block c571a
            // c571a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c571b mov rbp,rsp
            rbp = rsp;
            // c571e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c571f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c5720 mov rbx,rdi
            rbx = rdi;
            // c5723 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c572b mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c5732 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c5736 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c5739 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c573c cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c5740 je 0xc574b
            if fl.e() { pc = 0xc574b; continue; }
            pc = 0xc5742;
        }
        0xc5742 => {
            // block c5742
            // c5742 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c5745 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c5748 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc574b;
        }
        0xc574b => {
            // block c574b
            // c574b mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c574f mov rsi,rcx
            rsi = rcx;
            // c5752 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5757 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c575b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c575f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5760 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c5761 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc587a size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c587a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc587a;
    loop {
        match pc {
        0xc587a => {
            // block c587a
            // c587a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c587b mov rbp,rsp
            rbp = rsp;
            // c587e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c587f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c5880 mov rbx,rdi
            rbx = rdi;
            // c5883 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c588b mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c5892 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c5896 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c5899 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c589c cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c58a0 je 0xc58ab
            if fl.e() { pc = 0xc58ab; continue; }
            pc = 0xc58a2;
        }
        0xc58a2 => {
            // block c58a2
            // c58a2 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c58a5 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c58a8 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc58ab;
        }
        0xc58ab => {
            // block c58ab
            // c58ab mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c58af mov rsi,rcx
            rsi = rcx;
            // c58b2 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c58b7 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c58bb add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c58bf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c58c0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c58c1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x159ae size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000159ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x159ae;
    loop {
        match pc {
        0x159ae => {
            // block 159ae
            // 159ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 159af mov rbp,rsp
            rbp = rsp;
            // 159b2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 159b3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 159b4 mov rbx,rdi
            rbx = rdi;
            // 159b7 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 159bf mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 159c6 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // 159ca mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // 159cd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 159d0 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // 159d4 je 0x159df
            if fl.e() { pc = 0x159df; continue; }
            pc = 0x159d6;
        }
        0x159d6 => {
            // block 159d6
            // 159d6 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 159d9 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 159dc mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0x159df;
        }
        0x159df => {
            // block 159df
            // 159df mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 159e3 mov rsi,rcx
            rsi = rcx;
            // 159e6 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 159eb inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // 159ef add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 159f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 159f4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 159f5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25de6 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025de6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x25de6;
    loop {
        match pc {
        0x25de6 => {
            // block 25de6
            // 25de6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25de7 mov rbp,rsp
            rbp = rsp;
            // 25dea push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 25deb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 25dec mov rbx,rdi
            rbx = rdi;
            // 25def mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 25df7 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 25dfe mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // 25e02 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // 25e05 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 25e08 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // 25e0c je 0x25e17
            if fl.e() { pc = 0x25e17; continue; }
            pc = 0x25e0e;
        }
        0x25e0e => {
            // block 25e0e
            // 25e0e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 25e11 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 25e14 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0x25e17;
        }
        0x25e17 => {
            // block 25e17
            // 25e17 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 25e1b mov rsi,rcx
            rsi = rcx;
            // 25e1e call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25e23 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // 25e27 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 25e2b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25e2c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25e2d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x199d4 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000199d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x199d4;
    loop {
        match pc {
        0x199d4 => {
            // block 199d4
            // 199d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 199d5 mov rbp,rsp
            rbp = rsp;
            // 199d8 mov rax,rdi
            rax = rdi;
            // 199db mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // 199de test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 199e1 je 0x19a07
            if fl.e() { pc = 0x19a07; continue; }
            pc = 0x199e3;
        }
        0x199e3 => {
            // block 199e3
            // 199e3 mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 199e7 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // 199ea je 0x19a01
            if fl.e() { pc = 0x19a01; continue; }
            pc = 0x199ec;
        }
        0x199ec => {
            // block 199ec
            // 199ec lea rdx,[rcx-0x4]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffc);
            // 199f0 sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // 199f3 not rdx
            rdx = !rdx;
            // 199f6 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 199fa add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 199fd mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            pc = 0x19a01;
        }
        0x19a01 => {
            // block 19a01
            // 19a01 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19a02 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x19a07 => {
            // block 19a07
            // 19a07 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19a08 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ec00 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002ec00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2ec00;
    loop {
        match pc {
        0x2ec00 => {
            // block 2ec00
            // 2ec00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ec01 mov rbp,rsp
            rbp = rsp;
            // 2ec04 mov rax,rdi
            rax = rdi;
            // 2ec07 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // 2ec0a test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2ec0d je 0x2ec33
            if fl.e() { pc = 0x2ec33; continue; }
            pc = 0x2ec0f;
        }
        0x2ec0f => {
            // block 2ec0f
            // 2ec0f mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2ec13 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // 2ec16 je 0x2ec2d
            if fl.e() { pc = 0x2ec2d; continue; }
            pc = 0x2ec18;
        }
        0x2ec18 => {
            // block 2ec18
            // 2ec18 lea rdx,[rcx-0x8]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fff8);
            // 2ec1c sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // 2ec1f not rdx
            rdx = !rdx;
            // 2ec22 and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 2ec26 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 2ec29 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            pc = 0x2ec2d;
        }
        0x2ec2d => {
            // block 2ec2d
            // 2ec2d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ec2e jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2ec33 => {
            // block 2ec33
            // 2ec33 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ec34 ret
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
            // ba4b6 lea rdx,[rcx-0x8]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fff8);
            // ba4ba sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // ba4bd not rdx
            rdx = !rdx;
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
            // ba4cb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba4cc jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xba4d1 => {
            // block ba4d1
            // ba4d1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba4d2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19d58 size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00019d58<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x19d58;
    loop {
        match pc {
        0x19d58 => {
            // block 19d58
            // 19d58 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19d59 mov rbp,rsp
            rbp = rsp;
            // 19d5c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19d5d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19d5e mov rbx,rdi
            rbx = rdi;
            // 19d61 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 19d64 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 19d67 je 0x19da3
            if fl.e() { pc = 0x19da3; continue; }
            pc = 0x19d69;
        }
        0x19d69 => {
            // block 19d69
            // 19d69 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 19d6d cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 19d70 je 0x19d87
            if fl.e() { pc = 0x19d87; continue; }
            pc = 0x19d72;
        }
        0x19d72 => {
            // block 19d72
            // 19d72 lea rcx,[rax-0x4]
            rcx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19d76 sub rcx,rdi
            { let (r, f) = sub::<64>(rcx, rdi);
              rcx = r;
              fl = f; }
            // 19d79 not rcx
            rcx = !rcx;
            // 19d7c and rcx,0xfffffffffffffffc
            { let r = rcx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rcx = r;
              fl = f; }
            // 19d80 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 19d83 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            pc = 0x19d87;
        }
        0x19d87 => {
            // block 19d87
            // 19d87 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19d8c mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 19d94 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 19d9c mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            pc = 0x19da3;
        }
        0x19da3 => {
            // block 19da3
            // 19da3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19da7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19da8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19da9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a0c8 size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a0c8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a0c8;
    loop {
        match pc {
        0x1a0c8 => {
            // block 1a0c8
            // 1a0c8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a0c9 mov rbp,rsp
            rbp = rsp;
            // 1a0cc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a0cd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1a0ce mov rbx,rdi
            rbx = rdi;
            // 1a0d1 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 1a0d4 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1a0d7 je 0x1a113
            if fl.e() { pc = 0x1a113; continue; }
            pc = 0x1a0d9;
        }
        0x1a0d9 => {
            // block 1a0d9
            // 1a0d9 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1a0dd cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 1a0e0 je 0x1a0f7
            if fl.e() { pc = 0x1a0f7; continue; }
            pc = 0x1a0e2;
        }
        0x1a0e2 => {
            // block 1a0e2
            // 1a0e2 lea rcx,[rax-0x4]
            rcx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 1a0e6 sub rcx,rdi
            { let (r, f) = sub::<64>(rcx, rdi);
              rcx = r;
              fl = f; }
            // 1a0e9 not rcx
            rcx = !rcx;
            // 1a0ec and rcx,0xfffffffffffffffc
            { let r = rcx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rcx = r;
              fl = f; }
            // 1a0f0 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 1a0f3 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            pc = 0x1a0f7;
        }
        0x1a0f7 => {
            // block 1a0f7
            // 1a0f7 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a0fc mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1a104 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1a10c mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            pc = 0x1a113;
        }
        0x1a113 => {
            // block 1a113
            // 1a113 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1a117 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a118 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a119 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15e10 size=268
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00015e10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x15e10;
    loop {
        match pc {
        0x15e10 => {
            // block 15e10
            // 15e10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15e11 mov rbp,rsp
            rbp = rsp;
            // 15e14 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15e16 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15e18 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15e1a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15e1c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15e1d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 15e21 mov r14,rsi
            r14 = rsi;
            // 15e24 mov r12,rdi
            r12 = rdi;
            // 15e27 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 15e2b mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 15e30 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 15e33 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 15e36 jns 0x15e44
            if fl.ns() { pc = 0x15e44; continue; }
            pc = 0x15e38;
        }
        0x15e38 => {
            // block 15e38
            // 15e38 mov rdi,r12
            rdi = r12;
            // 15e3b call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x15e44 => {
            // block 15e44
            // 15e44 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 15e49 sub r15,rcx
            { let (r, f) = sub::<64>(r15, rcx);
              r15 = r;
              fl = f; }
            // 15e4c movabs rax,0x3fffffffffffffff
            rax = 0x3fff_ffff_ffff_ffffu64;
            // 15e56 cmp r15,rax
            fl = sub::<64>(r15, rax).1;
            // 15e59 jae 0x15e87
            if fl.ae() { pc = 0x15e87; continue; }
            pc = 0x15e5b;
        }
        0x15e5b => {
            // block 15e5b
            // 15e5b mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 15e5f add r15,r15
            { let (r, f) = add::<64>(r15, r15);
              r15 = r;
              fl = f; }
            // 15e62 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 15e65 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 15e69 mov r13,QWORD PTR [r12+0x8]
            r13 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 15e6e mov rbx,r13
            rbx = r13;
            // 15e71 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 15e74 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 15e78 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 15e7b test r15,r15
            fl = logic::<64>(r15 & r15);
            // 15e7e mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 15e83 jne 0x15ea4
            if fl.ne() { pc = 0x15ea4; continue; }
            pc = 0x15e85;
        }
        0x15e85 => {
            // block 15e85
            // 15e85 jmp 0x15eaf
            pc = 0x15eaf; continue;
        }
        0x15e87 => {
            // block 15e87
            // 15e87 mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 15e8b mov r13,QWORD PTR [r12+0x8]
            r13 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 15e90 mov rbx,r13
            rbx = r13;
            // 15e93 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 15e96 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 15e9a movabs r15,0x7fffffffffffffff
            r15 = 0x7fff_ffff_ffff_ffffu64;
            pc = 0x15ea4;
        }
        0x15ea4 => {
            // block 15ea4
            // 15ea4 mov rdi,r15
            rdi = r15;
            // 15ea7 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15eac mov r14,r15
            r14 = r15;
            pc = 0x15eaf;
        }
        0x15eaf => {
            // block 15eaf
            // 15eaf add r14,rax
            { let (r, f) = add::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // 15eb2 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 15eb6 mov cl,BYTE PTR [rcx]
            rcx = (rcx & !0xff) | ((m.r8(rcx) as u64) as u8 as u64);
            // 15eb8 mov BYTE PTR [rax+rbx*1],cl
            m.w8(rax.wrapping_add(rbx), ((rcx as u8 as u64)) as u8);
            // 15ebb lea rcx,[rax+rbx*1+0x1]
            rcx = rax.wrapping_add(rbx).wrapping_add(0x1);
            // 15ec0 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // 15ec4 mov r15,QWORD PTR [rbp-0x38]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 15ec8 sub r13,r15
            { let (r, f) = sub::<64>(r13, r15);
              r13 = r;
              fl = f; }
            // 15ecb sub rbx,r13
            { let (r, f) = sub::<64>(rbx, r13);
              rbx = r;
              fl = f; }
            // 15ece add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 15ed1 mov rdi,rbx
            rdi = rbx;
            // 15ed4 mov rsi,r15
            rsi = r15;
            // 15ed7 mov rdx,r13
            rdx = r13;
            // 15eda call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15edf mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 15ee3 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 15ee7 mov QWORD PTR [r12+0x8],rax
            m.w64(r12.wrapping_add(0x8), (rax) as u64);
            // 15eec mov QWORD PTR [r12+0x10],r14
            m.w64(r12.wrapping_add(0x10), (r14) as u64);
            // 15ef1 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 15ef4 je 0x15f0c
            if fl.e() { pc = 0x15f0c; continue; }
            pc = 0x15ef6;
        }
        0x15ef6 => {
            // block 15ef6
            // 15ef6 mov rdi,r15
            rdi = r15;
            // 15ef9 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 15efd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15efe pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15f00 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15f02 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15f04 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15f06 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15f07 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x15f0c => {
            // block 15f0c
            // 15f0c add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 15f10 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15f11 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15f13 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15f15 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15f17 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15f19 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15f1a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb608 size=280
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000eb608<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xeb608;
    loop {
        match pc {
        0xeb608 => {
            // block eb608
            // eb608 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb609 mov rbp,rsp
            rbp = rsp;
            // eb60c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb60e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb610 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // eb612 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eb614 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb615 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // eb619 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // eb61d mov r15,rdi
            r15 = rdi;
            // eb620 movabs r12,0x492492492492492
            r12 = 0x492_4924_9249_2492u64;
            // eb62a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // eb62d mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // eb631 sub rbx,rax
            { let (r, f) = sub::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // eb634 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // eb638 movabs r13,0x6db6db6db6db6db7
            r13 = 0x6db6_db6d_b6db_6db7u64;
            // eb642 imul rbx,r13
            { let (r, f) = imul::<64>(rbx, r13); rbx = r; fl = f; }
            // eb646 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // eb649 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // eb64c jbe 0xeb659
            if fl.be() { pc = 0xeb659; continue; }
            pc = 0xeb64e;
        }
        0xeb64e => {
            // block eb64e
            // eb64e mov rdi,r15
            rdi = r15;
            // eb651 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0xeb659 => {
            // block eb659
            // eb659 lea rcx,[r15+0x10]
            rcx = r15.wrapping_add(0x10);
            // eb65d mov rdx,QWORD PTR [r15+0x10]
            rdx = m.r64(r15.wrapping_add(0x10)) as u64;
            // eb661 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // eb664 sar rdx,0x3
            { let (r, f) = sar::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // eb668 imul rdx,r13
            { let (r, f) = imul::<64>(rdx, r13); rdx = r; fl = f; }
            // eb66c movabs rsi,0x249249249249248
            rsi = 0x249_2492_4924_9248u64;
            // eb676 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // eb679 ja 0xeb688
            if fl.a() { pc = 0xeb688; continue; }
            pc = 0xeb67b;
        }
        0xeb67b => {
            // block eb67b
            // eb67b add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // eb67e cmp rdx,rbx
            fl = sub::<64>(rdx, rbx).1;
            // eb681 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // eb685 mov r12,rdx
            r12 = rdx;
            pc = 0xeb688;
        }
        0xeb688 => {
            // block eb688
            // eb688 mov r14,QWORD PTR [r15+0x8]
            r14 = m.r64(r15.wrapping_add(0x8)) as u64;
            // eb68c sub r14,rax
            { let (r, f) = sub::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // eb68f sar r14,0x3
            { let (r, f) = sar::<64>(r14, 0x3u64, fl); r14 = r; fl = f; }
            // eb693 imul r14,r13
            { let (r, f) = imul::<64>(r14, r13); r14 = r; fl = f; }
            // eb697 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // eb69f mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // eb6a3 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // eb6a5 test r12,r12
            fl = logic::<64>(r12 & r12);
            // eb6a8 je 0xeb6b6
            if fl.e() { pc = 0xeb6b6; continue; }
            pc = 0xeb6aa;
        }
        0xeb6aa => {
            // block eb6aa
            // eb6aa imul rdi,r12,0x38
            { let (r, f) = imul::<64>(r12, 0x38u64); rdi = r; fl = f; }
            // eb6ae call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb6b3 mov rbx,rax
            rbx = rax;
            pc = 0xeb6b6;
        }
        0xeb6b6 => {
            // block eb6b6
            // eb6b6 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // eb6ba imul r14,r14,0x38
            { let (r, f) = imul::<64>(r14, 0x38u64); r14 = r; fl = f; }
            // eb6be lea rdi,[rbx+r14*1]
            rdi = rbx.wrapping_add(r14);
            // eb6c2 mov QWORD PTR [rbp-0x40],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdi) as u64);
            // eb6c6 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // eb6ca imul rax,r12,0x38
            { let (r, f) = imul::<64>(r12, 0x38u64); rax = r; fl = f; }
            // eb6ce add rax,rbx
            { let (r, f) = add::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // eb6d1 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // eb6d5 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // eb6d9 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb6de lea rax,[r14+rbx*1+0x38]
            rax = r14.wrapping_add(rbx).wrapping_add(0x38);
            // eb6e3 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // eb6e7 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // eb6eb mov rdi,r15
            rdi = r15;
            // eb6ee call 0xebf30
            { let r = { let t = m.env.stub_va(0x6fd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb6f3 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // eb6f7 call 0xeb8ac
            { let r = { let t = m.env.stub_va(0x79); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb6fc add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // eb700 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb701 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eb703 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eb705 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb707 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb709 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb70a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeb70b => {
            // block eb70b
            // eb70b mov rbx,rax
            rbx = rax;
            // eb70e lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // eb712 call 0xeb8ac
            { let r = { let t = m.env.stub_va(0x79); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb717 mov rdi,rbx
            rdi = rbx;
            // eb71a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdc0da size=282
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000dc0da<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdc0da;
    loop {
        match pc {
        0xdc0da => {
            // block dc0da
            // dc0da push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc0db mov rbp,rsp
            rbp = rsp;
            // dc0de push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dc0e0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dc0e2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dc0e4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dc0e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dc0e7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // dc0e8 mov r13,rsi
            r13 = rsi;
            // dc0eb mov r14,rdi
            r14 = rdi;
            // dc0ee mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // dc0f2 mov r15,QWORD PTR [r14+0x10]
            r15 = m.r64(r14.wrapping_add(0x10)) as u64;
            // dc0f6 mov rax,r15
            rax = r15;
            // dc0f9 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // dc0fc cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // dc0ff jae 0xdc14d
            if fl.ae() { pc = 0xdc14d; continue; }
            pc = 0xdc101;
        }
        0xdc101 => {
            // block dc101
            // dc101 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dc104 sub rbx,rax
            { let (r, f) = sub::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // dc107 add rbx,r13
            { let (r, f) = add::<64>(rbx, r13);
              rbx = r;
              fl = f; }
            // dc10a jns 0xdc11b
            if fl.ns() { pc = 0xdc11b; continue; }
            pc = 0xdc10c;
        }
        0xdc10c => {
            // block dc10c
            // dc10c mov rdi,r14
            rdi = r14;
            // dc10f call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0xdc11b => {
            // block dc11b
            // dc11b sub r15,rax
            { let (r, f) = sub::<64>(r15, rax);
              r15 = r;
              fl = f; }
            // dc11e movabs rcx,0x3fffffffffffffff
            rcx = 0x3fff_ffff_ffff_ffffu64;
            // dc128 cmp r15,rcx
            fl = sub::<64>(r15, rcx).1;
            // dc12b jae 0xdc165
            if fl.ae() { pc = 0xdc165; continue; }
            pc = 0xdc12d;
        }
        0xdc12d => {
            // block dc12d
            // dc12d add r15,r15
            { let (r, f) = add::<64>(r15, r15);
              r15 = r;
              fl = f; }
            // dc130 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // dc133 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // dc137 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // dc13b sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // dc13e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dc140 test r15,r15
            fl = logic::<64>(r15 & r15);
            // dc143 mov r12d,0x0
            r12 = (0x0u64) as u32 as u64;
            // dc149 jne 0xdc176
            if fl.ne() { pc = 0xdc176; continue; }
            pc = 0xdc14b;
        }
        0xdc14b => {
            // block dc14b
            // dc14b jmp 0xdc18a
            pc = 0xdc18a; continue;
        }
        0xdc14d => {
            // block dc14d
            // dc14d mov BYTE PTR [rbx],0x0
            m.w8(rbx, (0x0u64) as u8);
            // dc150 mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // dc154 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // dc157 mov QWORD PTR [r14+0x8],rbx
            m.w64(r14.wrapping_add(0x8), (rbx) as u64);
            // dc15b dec r13
            { let (r, f) = dec::<64>(r13, fl); r13 = r; fl = f; }
            // dc15e jne 0xdc14d
            if fl.ne() { pc = 0xdc14d; continue; }
            pc = 0xdc160;
        }
        0xdc160 => {
            // block dc160
            // dc160 jmp 0xdc1e5
            pc = 0xdc1e5; continue;
        }
        0xdc165 => {
            // block dc165
            // dc165 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // dc169 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // dc16c movabs r15,0x7fffffffffffffff
            r15 = 0x7fff_ffff_ffff_ffffu64;
            pc = 0xdc176;
        }
        0xdc176 => {
            // block dc176
            // dc176 mov rdi,r15
            rdi = r15;
            // dc179 mov rbx,rcx
            rbx = rcx;
            // dc17c call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc181 mov rcx,rbx
            rcx = rbx;
            // dc184 mov r12,rax
            r12 = rax;
            // dc187 mov rax,r15
            rax = r15;
            pc = 0xdc18a;
        }
        0xdc18a => {
            // block dc18a
            // dc18a lea rbx,[r12+rcx*1]
            rbx = r12.wrapping_add(rcx);
            // dc18e add rax,r12
            { let (r, f) = add::<64>(rax, r12);
              rax = r;
              fl = f; }
            // dc191 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            pc = 0xdc195;
        }
        0xdc195 => {
            // block dc195
            // dc195 mov BYTE PTR [rbx],0x0
            m.w8(rbx, (0x0u64) as u8);
            // dc198 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // dc19b dec r13
            { let (r, f) = dec::<64>(r13, fl); r13 = r; fl = f; }
            // dc19e jne 0xdc195
            if fl.ne() { pc = 0xdc195; continue; }
            pc = 0xdc1a0;
        }
        0xdc1a0 => {
            // block dc1a0
            // dc1a0 mov r15,QWORD PTR [r14]
            r15 = m.r64(r14) as u64;
            // dc1a3 mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // dc1a7 sub rdx,r15
            { let (r, f) = sub::<64>(rdx, r15);
              rdx = r;
              fl = f; }
            // dc1aa sub rcx,rdx
            { let (r, f) = sub::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // dc1ad add r12,rcx
            { let (r, f) = add::<64>(r12, rcx);
              r12 = r;
              fl = f; }
            // dc1b0 mov rdi,r12
            rdi = r12;
            // dc1b3 mov rsi,r15
            rsi = r15;
            // dc1b6 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc1bb mov QWORD PTR [r14],r12
            m.w64(r14, (r12) as u64);
            // dc1be mov QWORD PTR [r14+0x8],rbx
            m.w64(r14.wrapping_add(0x8), (rbx) as u64);
            // dc1c2 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // dc1c6 mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // dc1ca test r15,r15
            fl = logic::<64>(r15 & r15);
            // dc1cd je 0xdc1e5
            if fl.e() { pc = 0xdc1e5; continue; }
            pc = 0xdc1cf;
        }
        0xdc1cf => {
            // block dc1cf
            // dc1cf mov rdi,r15
            rdi = r15;
            // dc1d2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // dc1d6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc1d7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dc1d9 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dc1db pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc1dd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dc1df pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc1e0 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xdc1e5 => {
            // block dc1e5
            // dc1e5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // dc1e9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc1ea pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dc1ec pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dc1ee pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc1f0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dc1f2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc1f3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a46a size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001a46a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a46a;
    loop {
        match pc {
        0x1a46a => {
            // block 1a46a
            // 1a46a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a46b mov rbp,rsp
            rbp = rsp;
            // 1a46e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1a470 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a472 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1a474 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1a476 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a477 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a47b mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1a47f mov r12,rdi
            r12 = rdi;
            // 1a482 mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1a486 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a48b sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1a48e sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1a492 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1a495 mov rax,rbx
            rax = rbx;
            // 1a498 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1a49c je 0x1a4aa
            if fl.e() { pc = 0x1a4aa; continue; }
            pc = 0x1a49e;
        }
        0x1a49e => {
            // block 1a49e
            // 1a49e mov rdi,r12
            rdi = r12;
            // 1a4a1 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x1a4aa => {
            // block 1a4aa
            // 1a4aa lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1a4af mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1a4b4 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1a4b8 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1a4bb mov rcx,r15
            rcx = r15;
            // 1a4be sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1a4c2 movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1a4cc cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1a4cf ja 0x1a4dd
            if fl.a() { pc = 0x1a4dd; continue; }
            pc = 0x1a4d1;
        }
        0x1a4d1 => {
            // block 1a4d1
            // 1a4d1 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1a4d4 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1a4d7 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1a4db jmp 0x1a4e7
            pc = 0x1a4e7; continue;
        }
        0x1a4dd => {
            // block 1a4dd
            // 1a4dd movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1a4e7;
        }
        0x1a4e7 => {
            // block 1a4e7
            // 1a4e7 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a4ec mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1a4f0 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1a4f3 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1a4f7 mov r13,r14
            r13 = r14;
            // 1a4fa sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1a4fe mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1a506 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1a50a xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1a50c test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1a50f je 0x1a521
            if fl.e() { pc = 0x1a521; continue; }
            pc = 0x1a511;
        }
        0x1a511 => {
            // block 1a511
            // 1a511 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1a519 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a51e mov rbx,rax
            rbx = rax;
            pc = 0x1a521;
        }
        0x1a521 => {
            // block 1a521
            // 1a521 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1a525 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1a529 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1a52d mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1a52f mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1a533 lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1a538 mov rdi,rbx
            rdi = rbx;
            // 1a53b mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1a53f mov rsi,r15
            rsi = r15;
            // 1a542 mov rdx,r14
            rdx = r14;
            // 1a545 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a54a mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1a54e mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1a552 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1a557 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1a55b mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1a55f mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1a563 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1a568 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1a56c mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1a570 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1a574 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1a578 call 0xeb978
            { let r = { let t = m.env.stub_va(0x145); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a57d add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a581 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a582 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1a584 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1a586 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a588 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1a58a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a58b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a71a size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001a71a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a71a;
    loop {
        match pc {
        0x1a71a => {
            // block 1a71a
            // 1a71a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a71b mov rbp,rsp
            rbp = rsp;
            // 1a71e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1a720 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a722 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1a724 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1a726 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a727 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a72b mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1a72f mov r12,rdi
            r12 = rdi;
            // 1a732 mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1a736 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a73b sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1a73e sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1a742 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1a745 mov rax,rbx
            rax = rbx;
            // 1a748 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1a74c je 0x1a75a
            if fl.e() { pc = 0x1a75a; continue; }
            pc = 0x1a74e;
        }
        0x1a74e => {
            // block 1a74e
            // 1a74e mov rdi,r12
            rdi = r12;
            // 1a751 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x1a75a => {
            // block 1a75a
            // 1a75a lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1a75f mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1a764 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1a768 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1a76b mov rcx,r15
            rcx = r15;
            // 1a76e sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1a772 movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1a77c cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1a77f ja 0x1a78d
            if fl.a() { pc = 0x1a78d; continue; }
            pc = 0x1a781;
        }
        0x1a781 => {
            // block 1a781
            // 1a781 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1a784 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1a787 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1a78b jmp 0x1a797
            pc = 0x1a797; continue;
        }
        0x1a78d => {
            // block 1a78d
            // 1a78d movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1a797;
        }
        0x1a797 => {
            // block 1a797
            // 1a797 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a79c mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1a7a0 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1a7a3 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1a7a7 mov r13,r14
            r13 = r14;
            // 1a7aa sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1a7ae mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1a7b6 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1a7ba xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1a7bc test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1a7bf je 0x1a7d1
            if fl.e() { pc = 0x1a7d1; continue; }
            pc = 0x1a7c1;
        }
        0x1a7c1 => {
            // block 1a7c1
            // 1a7c1 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1a7c9 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a7ce mov rbx,rax
            rbx = rax;
            pc = 0x1a7d1;
        }
        0x1a7d1 => {
            // block 1a7d1
            // 1a7d1 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1a7d5 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1a7d9 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1a7dd mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1a7df mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1a7e3 lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1a7e8 mov rdi,rbx
            rdi = rbx;
            // 1a7eb mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1a7ef mov rsi,r15
            rsi = r15;
            // 1a7f2 mov rdx,r14
            rdx = r14;
            // 1a7f5 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a7fa mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1a7fe mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1a802 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1a807 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1a80b mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1a80f mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1a813 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1a818 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1a81c mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1a820 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1a824 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1a828 call 0xeb97e
            { let r = { let t = m.env.stub_va(0x14b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a82d add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a831 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a832 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1a834 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1a836 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a838 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1a83a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a83b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
