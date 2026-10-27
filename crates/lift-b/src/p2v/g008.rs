//! Mechanically lifted functions (scripts/lift_run.py), module g008. Each one is checked against the original
//! in the world harness (tests/world_g008.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x5c6a8 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c6a8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5c6a8;
    loop {
        match pc {
        0x5c6a8 => {
            // block 5c6a8
            // 5c6a8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c6a9 mov rbp,rsp
            rbp = rsp;
            // 5c6ac mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // 5c6b1 cmp edi,0x9
            fl = sub::<32>((rdi as u32 as u64), 0x9u64).1;
            // 5c6b4 jg 0x5c6c2
            if fl.g() { pc = 0x5c6c2; continue; }
            pc = 0x5c6b6;
        }
        0x5c6b6 => {
            // block 5c6b6
            // 5c6b6 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 5c6b8 lea rcx,[rip+0xa4101] # 0x1007c0
            rcx = m.env.data_va(85901);
            // 5c6bf mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5c6c2;
        }
        0x5c6c2 => {
            // block 5c6c2
            // 5c6c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c6c3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c822 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c822<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5c822;
    loop {
        match pc {
        0x5c822 => {
            // block 5c822
            // 5c822 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c823 mov rbp,rsp
            rbp = rsp;
            // 5c826 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // 5c82b cmp edi,0x9
            fl = sub::<32>((rdi as u32 as u64), 0x9u64).1;
            // 5c82e jg 0x5c83c
            if fl.g() { pc = 0x5c83c; continue; }
            pc = 0x5c830;
        }
        0x5c830 => {
            // block 5c830
            // 5c830 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 5c832 lea rcx,[rip+0xa4217] # 0x100a50
            rcx = m.env.data_va(86557);
            // 5c839 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5c83c;
        }
        0x5c83c => {
            // block 5c83c
            // 5c83c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c83d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9ff6 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9ff6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb9ff6;
    loop {
        match pc {
        0xb9ff6 => {
            // block b9ff6
            // b9ff6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9ff7 mov rbp,rsp
            rbp = rsp;
            // b9ffa test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9ffd je 0xba00d
            if fl.e() { pc = 0xba00d; continue; }
            pc = 0xb9fff;
        }
        0xb9fff => {
            // block b9fff
            // b9fff mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // ba002 movzx esi,si
            rsi = ((rsi as u16 as u64)) as u32 as u64;
            // ba005 call QWORD PTR [rax+0x17a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x17a8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba00b jmp 0xba00f
            pc = 0xba00f; continue;
        }
        0xba00d => {
            // block ba00d
            // ba00d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xba00f;
        }
        0xba00f => {
            // block ba00f
            // ba00f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba010 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3e32 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3e32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3e32;
    loop {
        match pc {
        0xc3e32 => {
            // block c3e32
            // c3e32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3e33 mov rbp,rsp
            rbp = rsp;
            // c3e36 lea rax,[rip+0x8039b] # 0x1441d8
            rax = m.env.data_va(362917);
            // c3e3d mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // c3e40 mov rsi,QWORD PTR [rdi+0x10]
            rsi = m.r64(rdi.wrapping_add(0x10)) as u64;
            // c3e44 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c3e48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3e49 jmp 0xebe22
            { let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3c size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000003c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3c;
    loop {
        match pc {
        0x3c => {
            // block 3c
            // 3c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3d mov rbp,rsp
            rbp = rsp;
            // 40 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 42 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 45 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 49 mov esi,0x2820
            rsi = (0x2820u64) as u32 as u64;
            // 4e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 53 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 56 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 57 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x16b2 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000016b2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x16b2;
    loop {
        match pc {
        0x16b2 => {
            // block 16b2
            // 16b2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 16b3 mov rbp,rsp
            rbp = rsp;
            // 16b6 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 16b8 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 16bb mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 16bf mov esi,0x2814
            rsi = (0x2814u64) as u32 as u64;
            // 16c4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 16c9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 16cc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 16cd jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc206 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c206<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc206;
    loop {
        match pc {
        0xc206 => {
            // block c206
            // c206 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c207 mov rbp,rsp
            rbp = rsp;
            // c20a mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // c20c mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c20f mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // c213 mov esi,0x27d9
            rsi = (0x27d9u64) as u32 as u64;
            // c218 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // c21d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c220 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c221 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc242 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c242<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc242;
    loop {
        match pc {
        0xc242 => {
            // block c242
            // c242 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c243 mov rbp,rsp
            rbp = rsp;
            // c246 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // c248 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c24b mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // c24f mov esi,0x27db
            rsi = (0x27dbu64) as u32 as u64;
            // c254 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // c259 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c25c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c25d jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc27e size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c27e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc27e;
    loop {
        match pc {
        0xc27e => {
            // block c27e
            // c27e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c27f mov rbp,rsp
            rbp = rsp;
            // c282 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // c284 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c287 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // c28b mov esi,0x27dd
            rsi = (0x27ddu64) as u32 as u64;
            // c290 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // c295 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c298 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c299 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc2ba size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c2ba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc2ba;
    loop {
        match pc {
        0xc2ba => {
            // block c2ba
            // c2ba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c2bb mov rbp,rsp
            rbp = rsp;
            // c2be mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // c2c0 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c2c3 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // c2c7 mov esi,0x27ec
            rsi = (0x27ecu64) as u32 as u64;
            // c2cc mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // c2d1 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c2d4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c2d5 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc2f6 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c2f6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc2f6;
    loop {
        match pc {
        0xc2f6 => {
            // block c2f6
            // c2f6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c2f7 mov rbp,rsp
            rbp = rsp;
            // c2fa mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // c2fc mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c2ff mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // c303 mov esi,0x27ee
            rsi = (0x27eeu64) as u32 as u64;
            // c308 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // c30d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c310 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c311 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc332 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c332<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc332;
    loop {
        match pc {
        0xc332 => {
            // block c332
            // c332 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c333 mov rbp,rsp
            rbp = rsp;
            // c336 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // c338 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c33b mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // c33f mov esi,0x27f0
            rsi = (0x27f0u64) as u32 as u64;
            // c344 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // c349 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c34c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c34d jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc36e size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c36e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc36e;
    loop {
        match pc {
        0xc36e => {
            // block c36e
            // c36e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c36f mov rbp,rsp
            rbp = rsp;
            // c372 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // c374 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c377 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // c37b mov esi,0x27f3
            rsi = (0x27f3u64) as u32 as u64;
            // c380 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // c385 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c388 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c389 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc56e size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c56e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc56e;
    loop {
        match pc {
        0xc56e => {
            // block c56e
            // c56e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c56f mov rbp,rsp
            rbp = rsp;
            // c572 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // c574 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c577 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // c57b mov esi,0x55
            rsi = (0x55u64) as u32 as u64;
            // c580 mov ecx,0xfff
            rcx = (0xfffu64) as u32 as u64;
            // c585 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c588 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c589 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcae8 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cae8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xcae8;
    loop {
        match pc {
        0xcae8 => {
            // block cae8
            // cae8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cae9 mov rbp,rsp
            rbp = rsp;
            // caec mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // caee mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // caf1 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // caf5 mov esi,0xc0
            rsi = (0xc0u64) as u32 as u64;
            // cafa mov ecx,0xf
            rcx = (0xfu64) as u32 as u64;
            // caff xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // cb02 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cb03 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcd34 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cd34<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xcd34;
    loop {
        match pc {
        0xcd34 => {
            // block cd34
            // cd34 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cd35 mov rbp,rsp
            rbp = rsp;
            // cd38 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // cd3a mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cd3d mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // cd41 mov esi,0x52
            rsi = (0x52u64) as u32 as u64;
            // cd46 mov ecx,0x1f
            rcx = (0x1fu64) as u32 as u64;
            // cd4b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // cd4e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cd4f jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf842 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f842<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf842;
    loop {
        match pc {
        0xf842 => {
            // block f842
            // f842 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f843 mov rbp,rsp
            rbp = rsp;
            // f846 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // f848 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // f84b mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // f84f mov esi,0x14e
            rsi = (0x14eu64) as u32 as u64;
            // f854 mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // f859 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // f85c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f85d jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51db4 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051db4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51db4;
    loop {
        match pc {
        0x51db4 => {
            // block 51db4
            // 51db4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51db5 mov rbp,rsp
            rbp = rsp;
            // 51db8 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51dba mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51dbd mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51dc1 mov esi,0x283c
            rsi = (0x283cu64) as u32 as u64;
            // 51dc6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51dcb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51dce pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51dcf jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51df0 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051df0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51df0;
    loop {
        match pc {
        0x51df0 => {
            // block 51df0
            // 51df0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51df1 mov rbp,rsp
            rbp = rsp;
            // 51df4 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51df6 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51df9 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51dfd mov esi,0x283d
            rsi = (0x283du64) as u32 as u64;
            // 51e02 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51e07 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51e0a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51e0b jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51e2c size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051e2c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51e2c;
    loop {
        match pc {
        0x51e2c => {
            // block 51e2c
            // 51e2c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51e2d mov rbp,rsp
            rbp = rsp;
            // 51e30 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51e32 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51e35 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51e39 mov esi,0x283e
            rsi = (0x283eu64) as u32 as u64;
            // 51e3e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51e43 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51e46 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51e47 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51e68 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051e68<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51e68;
    loop {
        match pc {
        0x51e68 => {
            // block 51e68
            // 51e68 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51e69 mov rbp,rsp
            rbp = rsp;
            // 51e6c mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51e6e mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51e71 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51e75 mov esi,0x283f
            rsi = (0x283fu64) as u32 as u64;
            // 51e7a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51e7f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51e82 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51e83 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51ea4 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051ea4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51ea4;
    loop {
        match pc {
        0x51ea4 => {
            // block 51ea4
            // 51ea4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51ea5 mov rbp,rsp
            rbp = rsp;
            // 51ea8 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51eaa mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51ead mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51eb1 mov esi,0x2840
            rsi = (0x2840u64) as u32 as u64;
            // 51eb6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51ebb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51ebe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51ebf jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51ee0 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051ee0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51ee0;
    loop {
        match pc {
        0x51ee0 => {
            // block 51ee0
            // 51ee0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51ee1 mov rbp,rsp
            rbp = rsp;
            // 51ee4 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51ee6 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51ee9 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51eed mov esi,0x27c5
            rsi = (0x27c5u64) as u32 as u64;
            // 51ef2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51ef7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51efa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51efb jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51f1c size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051f1c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51f1c;
    loop {
        match pc {
        0x51f1c => {
            // block 51f1c
            // 51f1c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51f1d mov rbp,rsp
            rbp = rsp;
            // 51f20 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51f22 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51f25 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51f29 mov esi,0x27c6
            rsi = (0x27c6u64) as u32 as u64;
            // 51f2e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51f33 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51f36 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51f37 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba156 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba156<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba156;
    loop {
        match pc {
        0xba156 => {
            // block ba156
            // ba156 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba157 mov rbp,rsp
            rbp = rsp;
            // ba15a mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba15c mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba15f mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba163 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // ba168 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba16d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba170 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba171 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba192 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba192<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba192;
    loop {
        match pc {
        0xba192 => {
            // block ba192
            // ba192 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba193 mov rbp,rsp
            rbp = rsp;
            // ba196 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba198 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba19b mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba19f mov esi,0xc
            rsi = (0xcu64) as u32 as u64;
            // ba1a4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba1a9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba1ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba1ad jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba1ce size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba1ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba1ce;
    loop {
        match pc {
        0xba1ce => {
            // block ba1ce
            // ba1ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba1cf mov rbp,rsp
            rbp = rsp;
            // ba1d2 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba1d4 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba1d7 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba1db mov esi,0x9
            rsi = (0x9u64) as u32 as u64;
            // ba1e0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba1e5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba1e8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba1e9 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba20a size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba20a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba20a;
    loop {
        match pc {
        0xba20a => {
            // block ba20a
            // ba20a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba20b mov rbp,rsp
            rbp = rsp;
            // ba20e mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba210 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba213 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba217 mov esi,0xf7
            rsi = (0xf7u64) as u32 as u64;
            // ba21c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba221 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba224 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba225 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba246 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba246<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba246;
    loop {
        match pc {
        0xba246 => {
            // block ba246
            // ba246 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba247 mov rbp,rsp
            rbp = rsp;
            // ba24a mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba24c mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba24f mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba253 mov esi,0x1e5
            rsi = (0x1e5u64) as u32 as u64;
            // ba258 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba25d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba260 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba261 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba282 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba282<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba282;
    loop {
        match pc {
        0xba282 => {
            // block ba282
            // ba282 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba283 mov rbp,rsp
            rbp = rsp;
            // ba286 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba288 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba28b mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba28f mov esi,0x1e8
            rsi = (0x1e8u64) as u32 as u64;
            // ba294 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba299 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba29c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba29d jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba2be size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba2be<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba2be;
    loop {
        match pc {
        0xba2be => {
            // block ba2be
            // ba2be push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba2bf mov rbp,rsp
            rbp = rsp;
            // ba2c2 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba2c4 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba2c7 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba2cb mov esi,0xb
            rsi = (0xbu64) as u32 as u64;
            // ba2d0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba2d5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba2d8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba2d9 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba2fa size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba2fa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba2fa;
    loop {
        match pc {
        0xba2fa => {
            // block ba2fa
            // ba2fa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba2fb mov rbp,rsp
            rbp = rsp;
            // ba2fe mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba300 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba303 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba307 mov esi,0xf8
            rsi = (0xf8u64) as u32 as u64;
            // ba30c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba311 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba314 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba315 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba336 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba336<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba336;
    loop {
        match pc {
        0xba336 => {
            // block ba336
            // ba336 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba337 mov rbp,rsp
            rbp = rsp;
            // ba33a mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba33c mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba33f mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba343 mov esi,0x1e6
            rsi = (0x1e6u64) as u32 as u64;
            // ba348 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba34d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba350 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba351 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba372 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba372<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba372;
    loop {
        match pc {
        0xba372 => {
            // block ba372
            // ba372 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba373 mov rbp,rsp
            rbp = rsp;
            // ba376 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba378 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba37b mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba37f mov esi,0x1e9
            rsi = (0x1e9u64) as u32 as u64;
            // ba384 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba389 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba38c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba38d jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba3ae size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba3ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba3ae;
    loop {
        match pc {
        0xba3ae => {
            // block ba3ae
            // ba3ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba3af mov rbp,rsp
            rbp = rsp;
            // ba3b2 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // ba3b4 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ba3b7 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // ba3bb mov esi,0xd
            rsi = (0xdu64) as u32 as u64;
            // ba3c0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ba3c5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ba3c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba3c9 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
