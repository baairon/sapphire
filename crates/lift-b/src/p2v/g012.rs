//! Mechanically lifted functions (scripts/lift_run.py), module g012. Each one is checked against the original
//! in the world harness (tests/world_g012.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x1613e size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001613e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1613e;
    loop {
        match pc {
        0x1613e => {
            // block 1613e
            // 1613e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1613f mov rbp,rsp
            rbp = rsp;
            // 16142 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 16143 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 16144 mov rbx,rdi
            rbx = rdi;
            // 16147 cmp rbx,rsi
            fl = sub::<64>(rbx, rsi).1;
            // 1614a je 0x16155
            if fl.e() { pc = 0x16155; continue; }
            pc = 0x1614c;
        }
        0x1614c => {
            // block 1614c
            // 1614c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1614f mov rdi,rbx
            rdi = rbx;
            // 16152 call QWORD PTR [rax+0x68]
            { let r = { let t = m.r64(rax.wrapping_add(0x68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x16155;
        }
        0x16155 => {
            // block 16155
            // 16155 mov rax,rbx
            rax = rbx;
            // 16158 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1615c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1615d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1615e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28f6c size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028f6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28f6c;
    loop {
        match pc {
        0x28f6c => {
            // block 28f6c
            // 28f6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28f6d mov rbp,rsp
            rbp = rsp;
            // 28f70 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 28f73 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28f76 je 0x28f8b
            if fl.e() { pc = 0x28f8b; continue; }
            pc = 0x28f78;
        }
        0x28f78 => {
            // block 28f78
            // 28f78 mov ecx,DWORD PTR [rdi+0x8]
            rcx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28f7b shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 28f7e je 0x28f8b
            if fl.e() { pc = 0x28f8b; continue; }
            pc = 0x28f80;
        }
        0x28f80 => {
            // block 28f80
            // 28f80 mov DWORD PTR [rax],esi
            m.w32(rax, ((rsi as u32 as u64)) as u32);
            // 28f82 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 28f86 dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // 28f89 jne 0x28f80
            if fl.ne() { pc = 0x28f80; continue; }
            pc = 0x28f8b;
        }
        0x28f8b => {
            // block 28f8b
            // 28f8b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28f8c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6819c size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006819c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6819c;
    loop {
        match pc {
        0x6819c => {
            // block 6819c
            // 6819c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6819d mov rbp,rsp
            rbp = rsp;
            // 681a0 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 681a2 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 681a5 mov rcx,QWORD PTR [rcx+0xbb8]
            rcx = m.r64(rcx.wrapping_add(0xbb8)) as u64;
            // 681ac mov edx,edx
            rdx = ((rdx as u32 as u64)) as u32 as u64;
            // 681ae lea rsi,[rip+0x99adb] # 0x101c90
            rsi = m.env.data_va(91229);
            // 681b5 mov esi,DWORD PTR [rsi+rdx*4]
            rsi = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 681b8 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 681ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 681bb jmp rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5ff4 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e5ff4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe5ff4;
    loop {
        match pc {
        0xe5ff4 => {
            // block e5ff4
            // e5ff4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5ff5 mov rbp,rsp
            rbp = rsp;
            // e5ff8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e5ff9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e5ffa mov rbx,rdi
            rbx = rdi;
            // e5ffd lea rax,[rbx+0x20]
            rax = rbx.wrapping_add(0x20);
            // e6001 mov rdi,rsi
            rdi = rsi;
            // e6004 mov rsi,rax
            rsi = rax;
            // e6007 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e600c mov al,BYTE PTR [rbx+0xc]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0xc)) as u64) as u8 as u64);
            // e600f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e6013 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e6014 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e6015 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe7c52 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7c52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe7c52;
    loop {
        match pc {
        0xe7c52 => {
            // block e7c52
            // e7c52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7c53 mov rbp,rsp
            rbp = rsp;
            // e7c56 cmp edi,0x1f
            fl = sub::<32>((rdi as u32 as u64), 0x1fu64).1;
            // e7c59 ja 0xe7c6b
            if fl.a() { pc = 0xe7c6b; continue; }
            pc = 0xe7c5b;
        }
        0xe7c5b => {
            // block e7c5b
            // e7c5b movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // e7c5e lea rcx,[rip+0x5e6cb] # 0x146330
            rcx = m.env.data_va(371453);
            // e7c65 mov rax,QWORD PTR [rcx+rax*8]
            rax = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // e7c69 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7c6a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe7c6b => {
            // block e7c6b
            // e7c6b lea rax,[rip+0x47f01] # 0x12fb73
            rax = m.env.data_va(279360);
            // e7c72 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7c73 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e0f0 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e0f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e0f0;
    loop {
        match pc {
        0x2e0f0 => {
            // block 2e0f0
            // 2e0f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e0f1 mov rbp,rsp
            rbp = rsp;
            // 2e0f4 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 2e0f6 lea rdx,[rip+0xd1c43] # 0xffd40
            rdx = m.env.data_va(83213);
            pc = 0x2e0fd;
        }
        0x2e0fd => {
            // block 2e0fd
            // 2e0fd cmp DWORD PTR [rdx+rcx*8],edi
            fl = sub::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(8))) as u64, (rdi as u32 as u64)).1;
            // 2e100 je 0x2e10e
            if fl.e() { pc = 0x2e10e; continue; }
            pc = 0x2e102;
        }
        0x2e102 => {
            // block 2e102
            // 2e102 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 2e105 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e107 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 2e10a jbe 0x2e0fd
            if fl.be() { pc = 0x2e0fd; continue; }
            pc = 0x2e10c;
        }
        0x2e10c => {
            // block 2e10c
            // 2e10c jmp 0x2e112
            pc = 0x2e112; continue;
        }
        0x2e10e => {
            // block 2e10e
            // 2e10e mov eax,DWORD PTR [rdx+rcx*8+0x4]
            rax = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4)) as u64) as u32 as u64;
            pc = 0x2e112;
        }
        0x2e112 => {
            // block 2e112
            // 2e112 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e113 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68106 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068106<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x68106;
    loop {
        match pc {
        0x68106 => {
            // block 68106
            // 68106 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68107 mov rbp,rsp
            rbp = rsp;
            // 6810a mov rax,rsi
            rax = rsi;
            // 6810d mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 68110 mov rcx,QWORD PTR [rcx+0xb8]
            rcx = m.r64(rcx.wrapping_add(0xb8)) as u64;
            // 68117 mov edx,edx
            rdx = ((rdx as u32 as u64)) as u32 as u64;
            // 68119 lea rsi,[rip+0x99b70] # 0x101c90
            rsi = m.env.data_va(91229);
            // 68120 mov esi,DWORD PTR [rsi+rdx*4]
            rsi = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 68123 mov rdx,rax
            rdx = rax;
            // 68126 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68127 jmp rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba93e size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba93e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba93e;
    loop {
        match pc {
        0xba93e => {
            // block ba93e
            // ba93e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba93f mov rbp,rsp
            rbp = rsp;
            // ba942 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ba946 mov DWORD PTR [rbp-0x8],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff8), ((rsi as u32 as u64)) as u32);
            // ba949 mov DWORD PTR [rbp-0x4],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), ((rdx as u32 as u64)) as u32);
            // ba94c add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // ba950 lea rsi,[rbp-0x8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff8);
            // ba954 call 0xebe04
            { let r = { let t = m.env.stub_va(0x5d1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba959 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ba95b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ba95f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba960 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe7c96 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7c96<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe7c96;
    loop {
        match pc {
        0xe7c96 => {
            // block e7c96
            // e7c96 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7c97 mov rbp,rsp
            rbp = rsp;
            // e7c9a dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // e7c9c cmp edi,0x11
            fl = sub::<32>((rdi as u32 as u64), 0x11u64).1;
            // e7c9f ja 0xe7cb1
            if fl.a() { pc = 0xe7cb1; continue; }
            pc = 0xe7ca1;
        }
        0xe7ca1 => {
            // block e7ca1
            // e7ca1 movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // e7ca4 lea rcx,[rip+0x5e815] # 0x1464c0
            rcx = m.env.data_va(371853);
            // e7cab mov rax,QWORD PTR [rcx+rax*8]
            rax = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // e7caf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7cb0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe7cb1 => {
            // block e7cb1
            // e7cb1 lea rax,[rip+0x51bc2] # 0x13987a
            rax = m.env.data_va(319559);
            // e7cb8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7cb9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x198a4 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000198a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x198a4;
    loop {
        match pc {
        0x198a4 => {
            // block 198a4
            // 198a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 198a5 mov rbp,rsp
            rbp = rsp;
            // 198a8 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 198ab call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 198ae mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 198b2 sub rcx,QWORD PTR [rax]
            { let (r, f) = sub::<64>(rcx, m.r64(rax) as u64);
              rcx = r;
              fl = f; }
            // 198b5 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 198b9 movabs rax,0xef7bdef7bdef7bdf
            rax = 0xef7b_def7_bdef_7bdfu64;
            // 198c3 imul rax,rcx
            { let (r, f) = imul::<64>(rax, rcx); rax = r; fl = f; }
            // 198c7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 198c8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a452 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a452<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5a452;
    loop {
        match pc {
        0x5a452 => {
            // block 5a452
            // 5a452 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a453 mov rbp,rsp
            rbp = rsp;
            // 5a456 dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 5a458 cmp edi,0x11
            fl = sub::<32>((rdi as u32 as u64), 0x11u64).1;
            // 5a45b ja 0x5a46e
            if fl.a() { pc = 0x5a46e; continue; }
            pc = 0x5a45d;
        }
        0x5a45d => {
            // block 5a45d
            // 5a45d movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // 5a460 lea rcx,[rip+0xa6859] # 0x100cc0
            rcx = m.env.data_va(87181);
            // 5a467 movsd xmm0,QWORD PTR [rcx+rax*8]
            x0 = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u128;
            // 5a46c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a46d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5a46e => {
            // block 5a46e
            // 5a46e movsd xmm0,QWORD PTR [rip+0xa630a] # 0x100780
            x0 = m.r64(m.env.data_va(85837)) as u128;
            // 5a476 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a477 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5e82 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e5e82<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe5e82;
    loop {
        match pc {
        0xe5e82 => {
            // block e5e82
            // e5e82 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5e83 mov rbp,rsp
            rbp = rsp;
            // e5e86 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // e5e88 jne 0xe5e8d
            if fl.ne() { pc = 0xe5e8d; continue; }
            pc = 0xe5e8a;
        }
        0xe5e8a => {
            // block e5e8a
            // e5e8a mov esi,DWORD PTR [rdi+0x8]
            rsi = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe5e8d;
        }
        0xe5e8d => {
            // block e5e8d
            // e5e8d dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e5e8f cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // e5e92 ja 0xe5ea3
            if fl.a() { pc = 0xe5ea3; continue; }
            pc = 0xe5e94;
        }
        0xe5e94 => {
            // block e5e94
            // e5e94 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // e5e97 lea rcx,[rip+0x43ea2] # 0x129d40
            rcx = m.env.data_va(255245);
            // e5e9e mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // e5ea1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5ea2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe5ea3 => {
            // block e5ea3
            // e5ea3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5ea5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5ea6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68400 size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068400<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x68400;
    loop {
        match pc {
        0x68400 => {
            // block 68400
            // 68400 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68401 mov rbp,rsp
            rbp = rsp;
            // 68404 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68405 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 68406 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 68408 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 6840a mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 6840c mov rbx,rdi
            rbx = rdi;
            // 6840f lea rsi,[rip+0x15ff6a] # 0x1c8380
            rsi = m.env.data_va(904013);
            // 68416 mov r8d,eax
            r8 = ((rax as u32 as u64)) as u32 as u64;
            // 68419 call 0x68d40
            { let r = { let t = m.env.code_va(0x68d40); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6841e mov rax,rbx
            rax = rbx;
            // 68421 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 68425 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68426 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68427 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4242e size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004242e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4242e;
    loop {
        match pc {
        0x4242e => {
            // block 4242e
            // 4242e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4242f mov rbp,rsp
            rbp = rsp;
            // 42432 mov r9,rsi
            r9 = rsi;
            // 42435 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 42438 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 4243c mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 4243e lea rdx,[rip+0xbdfab] # 0x1003f0
            rdx = m.env.data_va(84925);
            // 42445 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42448 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4244d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42450 mov rdx,r9
            rdx = r9;
            // 42453 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42454 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4247e size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004247e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4247e;
    loop {
        match pc {
        0x4247e => {
            // block 4247e
            // 4247e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4247f mov rbp,rsp
            rbp = rsp;
            // 42482 mov r9,rsi
            r9 = rsi;
            // 42485 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 42488 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 4248c mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 4248e lea rdx,[rip+0xbdf8b] # 0x100420
            rdx = m.env.data_va(84973);
            // 42495 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42498 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4249d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 424a0 mov rdx,r9
            rdx = r9;
            // 424a3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 424a4 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x424ce size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000424ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x424ce;
    loop {
        match pc {
        0x424ce => {
            // block 424ce
            // 424ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 424cf mov rbp,rsp
            rbp = rsp;
            // 424d2 mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 424d5 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 424d8 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 424dc mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 424de lea rdx,[rip+0xbdf8b] # 0x100470
            rdx = m.env.data_va(85053);
            // 424e5 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 424e8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 424ed xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 424f0 mov edx,r9d
            rdx = ((r9 as u32 as u64)) as u32 as u64;
            // 424f3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 424f4 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x119c8 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000119c8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x119c8;
    loop {
        match pc {
        0x119c8 => {
            // block 119c8
            // 119c8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 119c9 mov rbp,rsp
            rbp = rsp;
            // 119cc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 119cd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 119ce mov rbx,rsi
            rbx = rsi;
            // 119d1 mov rax,QWORD PTR [rdi+0x18]
            rax = m.r64(rdi.wrapping_add(0x18)) as u64;
            // 119d5 test rax,rax
            fl = logic::<64>(rax & rax);
            // 119d8 jne 0x119e6
            if fl.ne() { pc = 0x119e6; continue; }
            pc = 0x119da;
        }
        0x119da => {
            // block 119da
            // 119da mov rdi,QWORD PTR [rdi+0x8]
            rdi = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 119de mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 119e1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 119e3 call QWORD PTR [rax+0x48]
            { let r = { let t = m.r64(rax.wrapping_add(0x48)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x119e6;
        }
        0x119e6 => {
            // block 119e6
            // 119e6 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 119e9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 119eb add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 119ef pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 119f0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 119f1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x198d4 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000198d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x198d4;
    loop {
        match pc {
        0x198d4 => {
            // block 198d4
            // 198d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 198d5 mov rbp,rsp
            rbp = rsp;
            // 198d8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 198d9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 198da mov rbx,rdi
            rbx = rdi;
            // 198dd lea rax,[rip+0x127b34] # 0x141418
            rax = m.env.data_va(351205);
            // 198e4 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 198e7 lea rdi,[rbx+0x8]
            rdi = rbx.wrapping_add(0x8);
            // 198eb call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 198f0 mov rdi,rbx
            rdi = rbx;
            // 198f3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 198f7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 198f8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 198f9 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dea2 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dea2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2dea2;
    loop {
        match pc {
        0x2dea2 => {
            // block 2dea2
            // 2dea2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dea3 mov rbp,rsp
            rbp = rsp;
            // 2dea6 lea rdi,[rip+0x11923b] # 0x1470e8
            rdi = m.env.data_va(374965);
            // 2dead lea rsi,[rip+0xfffffffffffffe92] # 0x2dd46
            rsi = m.env.code_va(0x2dd46);
            // 2deb4 call 0xec266
            { let r = { let t = m.env.stub_va(0xa33); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2deb9 mov rax,QWORD PTR [rip+0x193b78] # 0x1c1a38
            rax = m.r64(m.env.data_va(877061)) as u64;
            // 2dec0 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2dec3 je 0x2dec8
            if fl.e() { pc = 0x2dec8; continue; }
            pc = 0x2dec5;
        }
        0x2dec5 => {
            // block 2dec5
            // 2dec5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dec6 jmp rax
            { let r = { let t = rax; m.call(t, &[], &[]) }; return Ret::from_call(r); }
        }
        0x2dec8 => {
            // block 2dec8
            // 2dec8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2deca pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2decb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2df32 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002df32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2df32;
    loop {
        match pc {
        0x2df32 => {
            // block 2df32
            // 2df32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2df33 mov rbp,rsp
            rbp = rsp;
            // 2df36 lea rdi,[rip+0x1191ab] # 0x1470e8
            rdi = m.env.data_va(374965);
            // 2df3d lea rsi,[rip+0xfffffffffffffe02] # 0x2dd46
            rsi = m.env.code_va(0x2dd46);
            // 2df44 call 0xec266
            { let r = { let t = m.env.stub_va(0xa33); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2df49 mov rax,QWORD PTR [rip+0x193b00] # 0x1c1a50
            rax = m.r64(m.env.data_va(877085)) as u64;
            // 2df50 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2df53 je 0x2df58
            if fl.e() { pc = 0x2df58; continue; }
            pc = 0x2df55;
        }
        0x2df55 => {
            // block 2df55
            // 2df55 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2df56 jmp rax
            { let r = { let t = rax; m.call(t, &[], &[]) }; return Ret::from_call(r); }
        }
        0x2df58 => {
            // block 2df58
            // 2df58 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2df5a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2df5b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2eb06 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002eb06<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2eb06;
    loop {
        match pc {
        0x2eb06 => {
            // block 2eb06
            // 2eb06 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2eb07 mov rbp,rsp
            rbp = rsp;
            // 2eb0a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2eb0b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2eb0c mov ebx,0xffffffff
            rbx = (0xffffffffu64) as u32 as u64;
            // 2eb11 lock xadd DWORD PTR [rdi+0x64],ebx
            { let a = m.r32(rdi.wrapping_add(0x64)) as u64; let b = (rbx as u32 as u64); let (r, f) = add::<32>(a, b); rbx = (a) as u32 as u64; m.w32(rdi.wrapping_add(0x64), (r) as u32); fl = f; }
            // 2eb16 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2eb19 je 0x2eb25
            if fl.e() { pc = 0x2eb25; continue; }
            pc = 0x2eb1b;
        }
        0x2eb1b => {
            // block 2eb1b
            // 2eb1b test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 2eb1d jne 0x2eb25
            if fl.ne() { pc = 0x2eb25; continue; }
            pc = 0x2eb1f;
        }
        0x2eb1f => {
            // block 2eb1f
            // 2eb1f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2eb22 call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2eb25;
        }
        0x2eb25 => {
            // block 2eb25
            // 2eb25 dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 2eb27 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 2eb29 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2eb2d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2eb2e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2eb2f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c762 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c762<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5c762;
    loop {
        match pc {
        0x5c762 => {
            // block 5c762
            // 5c762 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c763 mov rbp,rsp
            rbp = rsp;
            // 5c766 mov eax,0x1b
            rax = (0x1bu64) as u32 as u64;
            // 5c76b cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // 5c76e ja 0x5c78a
            if fl.a() { pc = 0x5c78a; continue; }
            pc = 0x5c770;
        }
        0x5c770 => {
            // block 5c770
            // 5c770 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 5c772 lea rcx,[rip+0xa4157] # 0x1008d0
            rcx = m.env.data_va(86173);
            // 5c779 lea rdx,[rip+0xa4130] # 0x1008b0
            rdx = m.env.data_va(86141);
            // 5c780 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // 5c783 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // 5c787 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5c78a;
        }
        0x5c78a => {
            // block 5c78a
            // 5c78a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c78b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbba33 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bba33<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbba33;
    loop {
        match pc {
        0xbba33 => {
            // block bba33
            // bba33 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bba34 mov rbp,rsp
            rbp = rsp;
            // bba37 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bba3c cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bba3f ja 0xbba5b
            if fl.a() { pc = 0xbba5b; continue; }
            pc = 0xbba41;
        }
        0xbba41 => {
            // block bba41
            // bba41 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bba43 lea rcx,[rip+0x46ff6] # 0x102a40
            rcx = m.env.data_va(94733);
            // bba4a lea rdx,[rip+0x46fcf] # 0x102a20
            rdx = m.env.data_va(94701);
            // bba51 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bba54 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bba58 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbba5b;
        }
        0xbba5b => {
            // block bba5b
            // bba5b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bba5c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbba5d size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bba5d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbba5d;
    loop {
        match pc {
        0xbba5d => {
            // block bba5d
            // bba5d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bba5e mov rbp,rsp
            rbp = rsp;
            // bba61 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bba66 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bba69 ja 0xbba85
            if fl.a() { pc = 0xbba85; continue; }
            pc = 0xbba6b;
        }
        0xbba6b => {
            // block bba6b
            // bba6b mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bba6d lea rcx,[rip+0x4700c] # 0x102a80
            rcx = m.env.data_va(94797);
            // bba74 lea rdx,[rip+0x46fe5] # 0x102a60
            rdx = m.env.data_va(94765);
            // bba7b test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bba7e cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bba82 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbba85;
        }
        0xbba85 => {
            // block bba85
            // bba85 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bba86 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbbbc size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbbbc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbbbbc;
    loop {
        match pc {
        0xbbbbc => {
            // block bbbbc
            // bbbbc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbbbd mov rbp,rsp
            rbp = rsp;
            // bbbc0 mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // bbbc5 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbbc8 ja 0xbbbe4
            if fl.a() { pc = 0xbbbe4; continue; }
            pc = 0xbbbca;
        }
        0xbbbca => {
            // block bbbca
            // bbbca mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbbcc lea rcx,[rip+0x4704d] # 0x102c20
            rcx = m.env.data_va(95213);
            // bbbd3 lea rdx,[rip+0x47026] # 0x102c00
            rdx = m.env.data_va(95181);
            // bbbda test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbbdd cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbbe1 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbbe4;
        }
        0xbbbe4 => {
            // block bbbe4
            // bbbe4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbbe5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbc9b size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbc9b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbbc9b;
    loop {
        match pc {
        0xbbc9b => {
            // block bbc9b
            // bbc9b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbc9c mov rbp,rsp
            rbp = rsp;
            // bbc9f mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bbca4 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbca7 ja 0xbbcc3
            if fl.a() { pc = 0xbbcc3; continue; }
            pc = 0xbbca9;
        }
        0xbbca9 => {
            // block bbca9
            // bbca9 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbcab lea rcx,[rip+0x4701e] # 0x102cd0
            rcx = m.env.data_va(95389);
            // bbcb2 lea rdx,[rip+0x46ff7] # 0x102cb0
            rdx = m.env.data_va(95357);
            // bbcb9 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbcbc cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbcc0 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbcc3;
        }
        0xbbcc3 => {
            // block bbcc3
            // bbcc3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbcc4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbcc5 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbcc5<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbbcc5;
    loop {
        match pc {
        0xbbcc5 => {
            // block bbcc5
            // bbcc5 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbcc6 mov rbp,rsp
            rbp = rsp;
            // bbcc9 mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // bbcce cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbcd1 ja 0xbbced
            if fl.a() { pc = 0xbbced; continue; }
            pc = 0xbbcd3;
        }
        0xbbcd3 => {
            // block bbcd3
            // bbcd3 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbcd5 lea rcx,[rip+0x47034] # 0x102d10
            rcx = m.env.data_va(95453);
            // bbcdc lea rdx,[rip+0x4700d] # 0x102cf0
            rdx = m.env.data_va(95421);
            // bbce3 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbce6 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbcea mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbced;
        }
        0xbbced => {
            // block bbced
            // bbced pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbcee ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbd19 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbd19<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbbd19;
    loop {
        match pc {
        0xbbd19 => {
            // block bbd19
            // bbd19 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbd1a mov rbp,rsp
            rbp = rsp;
            // bbd1d mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bbd22 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbd25 ja 0xbbd41
            if fl.a() { pc = 0xbbd41; continue; }
            pc = 0xbbd27;
        }
        0xbbd27 => {
            // block bbd27
            // bbd27 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbd29 lea rcx,[rip+0x47060] # 0x102d90
            rcx = m.env.data_va(95581);
            // bbd30 lea rdx,[rip+0x47039] # 0x102d70
            rdx = m.env.data_va(95549);
            // bbd37 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbd3a cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbd3e mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbd41;
        }
        0xbbd41 => {
            // block bbd41
            // bbd41 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbd42 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41ea6 size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041ea6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x41ea6;
    loop {
        match pc {
        0x41ea6 => {
            // block 41ea6
            // 41ea6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41ea7 mov rbp,rsp
            rbp = rsp;
            // 41eaa mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 41ead mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 41eb0 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 41eb4 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 41eb6 lea rdx,[rip+0xbe503] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 41ebd mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 41ec0 mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 41ec5 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 41ecb mov edx,r9d
            rdx = ((r9 as u32 as u64)) as u32 as u64;
            // 41ece pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41ecf jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x420d2 size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000420d2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x420d2;
    loop {
        match pc {
        0x420d2 => {
            // block 420d2
            // 420d2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 420d3 mov rbp,rsp
            rbp = rsp;
            // 420d6 mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 420d9 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 420dc mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 420e0 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 420e2 lea rdx,[rip+0xbe2d7] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 420e9 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 420ec mov ecx,0x80000
            rcx = (0x80000u64) as u32 as u64;
            // 420f1 mov r8d,0x13
            r8 = (0x13u64) as u32 as u64;
            // 420f7 mov edx,r9d
            rdx = ((r9 as u32 as u64)) as u32 as u64;
            // 420fa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 420fb jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43a6c size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043a6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43a6c;
    loop {
        match pc {
        0x43a6c => {
            // block 43a6c
            // 43a6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43a6d mov rbp,rsp
            rbp = rsp;
            // 43a70 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43a73 jle 0x43a79
            if fl.le() { pc = 0x43a79; continue; }
            pc = 0x43a75;
        }
        0x43a75 => {
            // block 43a75
            // 43a75 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43a77 jmp 0x43a96
            pc = 0x43a96; continue;
        }
        0x43a79 => {
            // block 43a79
            // 43a79 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43a7c mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 43a7e lea rsi,[rip+0xbc93b] # 0x1003c0
            rsi = m.env.data_va(84877);
            // 43a85 mov esi,DWORD PTR [rsi+rcx*4]
            rsi = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43a88 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // 43a8b mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 43a90 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 43a93 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43a96;
        }
        0x43a96 => {
            // block 43a96
            // 43a96 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43a97 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1951a size=45
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001951a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1951a;
    loop {
        match pc {
        0x1951a => {
            // block 1951a
            // 1951a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1951b mov rbp,rsp
            rbp = rsp;
            // 1951e sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 19522 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 19526 mov rsi,QWORD PTR [rdi+0x10]
            rsi = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 1952a lea rcx,[rip+0x16] # 0x19547
            rcx = m.env.code_va(0x19547);
            // 19531 mov QWORD PTR [rbp-0x8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff8), (rcx) as u64);
            // 19535 lea rdx,[rbp-0x8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff8);
            // 19539 mov rdi,rax
            rdi = rax;
            // 1953c call 0xeb996
            { let r = { let t = m.env.stub_va(0x163); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19541 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 19545 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19546 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2a2b2 size=14
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002a2b2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2a2b2;
    loop {
        match pc {
        0x2a2b2 => {
            // block 2a2b2
            // 2a2b2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2a2b3 mov rbp,rsp
            rbp = rsp;
            // 2a2b6 mov edi,DWORD PTR [rdi+0x20]
            rdi = (m.r32(rdi.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2a2b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2a2ba jmp 0x5c586
            { let r = { let t = m.env.code_va(0x5c586); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a6dc size=47
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a6dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5a6dc;
    loop {
        match pc {
        0x5a6dc => {
            // block 5a6dc
            // 5a6dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a6dd mov rbp,rsp
            rbp = rsp;
            // 5a6e0 dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 5a6e2 cmp edi,0x78
            fl = sub::<32>((rdi as u32 as u64), 0x78u64).1;
            // 5a6e5 ja 0x5a707
            if fl.a() { pc = 0x5a707; continue; }
            pc = 0x5a6e7;
        }
        0x5a6e7 => {
            // block 5a6e7
            // 5a6e7 movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // 5a6ea lea rcx,[rip+0xa6c9f] # 0x101390
            rcx = m.env.data_va(88925);
            // 5a6f1 mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5a6f4 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5a6f6 cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 5a6f9 ja 0x5a707
            if fl.a() { pc = 0x5a707; continue; }
            pc = 0x5a6fb;
        }
        0x5a6fb => {
            // block 5a6fb
            // 5a6fb mov eax,0x7a9
            rax = (0x7a9u64) as u32 as u64;
            // 5a700 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5a702 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a705 jmp 0x5a709
            pc = 0x5a709; continue;
        }
        0x5a707 => {
            // block 5a707
            // 5a707 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5a709;
        }
        0x5a709 => {
            // block 5a709
            // 5a709 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a70a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2c size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000e2c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2c;
    loop {
        match pc {
        0xe2c => {
            // block e2c
            // e2c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2d mov rbp,rsp
            rbp = rsp;
            // e30 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e31 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e32 mov rbx,rsi
            rbx = rsi;
            // e35 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e38 mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // e3a call QWORD PTR [rax+0x3c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3c0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e40 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e43 mov rcx,rax
            rcx = rax;
            // e46 shr rcx,0x20
            { let (r, f) = shr::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // e4a setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // e4d test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e4f setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // e52 and al,cl
            { let r = (rax as u8 as u64) & (rcx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e54 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e58 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e59 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
