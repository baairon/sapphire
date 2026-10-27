//! Mechanically lifted functions (scripts/lift_run.py), module g014. Each one is checked against the original
//! in the world harness (tests/world_g014.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe42ac size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e42ac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe42ac;
    loop {
        match pc {
        0xe42ac => {
            // block e42ac
            // e42ac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e42ad mov rbp,rsp
            rbp = rsp;
            // e42b0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e42b1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e42b2 mov rbx,rdi
            rbx = rdi;
            // e42b5 mov QWORD PTR [rbx],rsi
            m.w64(rbx, (rsi) as u64);
            // e42b8 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // e42c0 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // e42c8 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // e42d0 mov DWORD PTR [rbx+0x30],0x41c00
            m.w32(rbx.wrapping_add(0x30), (0x41c00u64) as u32);
            // e42d7 lea rdi,[rbx+0x34]
            rdi = rbx.wrapping_add(0x34);
            // e42db mov esi,0x2008
            rsi = (0x2008u64) as u32 as u64;
            // e42e0 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e42e5 mov DWORD PTR [rbx+0x2048],0x1
            m.w32(rbx.wrapping_add(0x2048), (0x1u64) as u32);
            // e42ef add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e42f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e42f4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e42f5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe73a0 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e73a0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe73a0;
    loop {
        match pc {
        0xe73a0 => {
            // block e73a0
            // e73a0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e73a1 mov rbp,rsp
            rbp = rsp;
            // e73a4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e73a6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e73a8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e73aa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e73ac push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e73ad push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e73ae mov r15,r8
            r15 = r8;
            // e73b1 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // e73b4 mov rbx,rsi
            rbx = rsi;
            // e73b7 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // e73ba jle 0xe73db
            if fl.le() { pc = 0xe73db; continue; }
            pc = 0xe73bc;
        }
        0xe73bc => {
            // block e73bc
            // e73bc movsxd r12,edx
            r12 = sx::<32>((rdx as u32 as u64)) as u64;
            // e73bf movsxd r13,r9d
            r13 = sx::<32>((r9 as u32 as u64)) as u64;
            pc = 0xe73c2;
        }
        0xe73c2 => {
            // block e73c2
            // e73c2 mov rdi,r15
            rdi = r15;
            // e73c5 mov rsi,rbx
            rsi = rbx;
            // e73c8 mov rdx,r12
            rdx = r12;
            // e73cb call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e73d0 add rbx,r12
            { let (r, f) = add::<64>(rbx, r12);
              rbx = r;
              fl = f; }
            // e73d3 add r15,r13
            { let (r, f) = add::<64>(r15, r13);
              r15 = r;
              fl = f; }
            // e73d6 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // e73d9 jne 0xe73c2
            if fl.ne() { pc = 0xe73c2; continue; }
            pc = 0xe73db;
        }
        0xe73db => {
            // block e73db
            // e73db add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e73df pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e73e0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e73e2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e73e4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e73e6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e73e8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e73e9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5010 size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005010<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5010;
    loop {
        match pc {
        0x5010 => {
            // block 5010
            // 5010 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5011 mov rbp,rsp
            rbp = rsp;
            // 5014 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5016 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5017 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 5019 mov r14,rdi
            r14 = rdi;
            // 501c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 501f call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5025 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 5027 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5029 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 502c ja 0x5058
            if fl.a() { pc = 0x5058; continue; }
            pc = 0x502e;
        }
        0x502e => {
            // block 502e
            // 502e xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 5031 je 0x5058
            if fl.e() { pc = 0x5058; continue; }
            pc = 0x5033;
        }
        0x5033 => {
            // block 5033
            // 5033 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 5036 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 5038 lea rdx,[rip+0xfa2b1] # 0xff2f0
            rdx = m.env.data_va(80573);
            // 503f mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5042 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 5047 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 504c mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            // 5052 mov rdi,r14
            rdi = r14;
            // 5055 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5058;
        }
        0x5058 => {
            // block 5058
            // 5058 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5059 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 505b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 505c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a11a size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001a11a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a11a;
    loop {
        match pc {
        0x1a11a => {
            // block 1a11a
            // 1a11a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a11b mov rbp,rsp
            rbp = rsp;
            // 1a11e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a120 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a121 mov r14,rsi
            r14 = rsi;
            // 1a124 mov rbx,rdi
            rbx = rdi;
            // 1a127 mov rax,r14
            rax = r14;
            // 1a12a shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1a12e je 0x1a138
            if fl.e() { pc = 0x1a138; continue; }
            pc = 0x1a130;
        }
        0x1a130 => {
            // block 1a130
            // 1a130 mov rdi,rbx
            rdi = rbx;
            // 1a133 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x1a138 => {
            // block 1a138
            // 1a138 lea rdi,[r14*4+0x0]
            rdi = r14.wrapping_mul(4);
            // 1a140 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a145 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 1a149 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1a14c lea rax,[rax+r14*4]
            rax = rax.wrapping_add(r14.wrapping_mul(4));
            // 1a150 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 1a154 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a155 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a157 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a158 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba9ca size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba9ca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba9ca;
    loop {
        match pc {
        0xba9ca => {
            // block ba9ca
            // ba9ca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba9cb mov rbp,rsp
            rbp = rsp;
            // ba9ce push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba9d0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba9d2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba9d3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ba9d4 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // ba9d7 mov rbx,rdi
            rbx = rdi;
            // ba9da mov DWORD PTR [rbp-0x1c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rsi as u32 as u64)) as u32);
            // ba9dd lea r15,[rbx+0x8]
            r15 = rbx.wrapping_add(0x8);
            // ba9e1 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // ba9e5 mov rdi,r15
            rdi = r15;
            // ba9e8 call 0xebe16
            { let r = { let t = m.env.stub_va(0x5e3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba9ed add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // ba9f1 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // ba9f4 je 0xbaa0b
            if fl.e() { pc = 0xbaa0b; continue; }
            pc = 0xba9f6;
        }
        0xba9f6 => {
            // block ba9f6
            // ba9f6 cmp DWORD PTR [rax+0x20],r14d
            fl = sub::<32>(m.r32(rax.wrapping_add(0x20)) as u64, (r14 as u32 as u64)).1;
            // ba9fa jne 0xbaa0b
            if fl.ne() { pc = 0xbaa0b; continue; }
            pc = 0xba9fc;
        }
        0xba9fc => {
            // block ba9fc
            // ba9fc mov rdi,r15
            rdi = r15;
            // ba9ff mov rsi,rax
            rsi = rax;
            // baa02 call 0xebe1c
            { let r = { let t = m.env.stub_va(0x5e9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baa07 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // baa09 jmp 0xbaa0d
            pc = 0xbaa0d; continue;
        }
        0xbaa0b => {
            // block baa0b
            // baa0b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xbaa0d;
        }
        0xbaa0d => {
            // block baa0d
            // baa0d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // baa11 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // baa12 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // baa14 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // baa16 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // baa17 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ad4 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002ad4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2ad4;
    loop {
        match pc {
        0x2ad4 => {
            // block 2ad4
            // 2ad4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ad5 mov rbp,rsp
            rbp = rsp;
            // 2ad8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2ada push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2adc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2add push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2ade mov r14,rdx
            r14 = rdx;
            // 2ae1 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2ae4 mov rbx,rdi
            rbx = rdi;
            // 2ae7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2aea call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2af0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2af2 je 0x2af8
            if fl.e() { pc = 0x2af8; continue; }
            pc = 0x2af4;
        }
        0x2af4 => {
            // block 2af4
            // 2af4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2af6 jmp 0x2b19
            pc = 0x2b19; continue;
        }
        0x2af8 => {
            // block 2af8
            // 2af8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2afb mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 2afe lea rdx,[rip+0xfc5ab] # 0xff0b0
            rdx = m.env.data_va(79997);
            // 2b05 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2b08 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2b0d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2b10 mov rdi,rbx
            rdi = rbx;
            // 2b13 mov rdx,r14
            rdx = r14;
            // 2b16 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2b19;
        }
        0x2b19 => {
            // block 2b19
            // 2b19 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2b1d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2b1e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2b20 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2b22 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b23 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bc4 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002bc4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2bc4;
    loop {
        match pc {
        0x2bc4 => {
            // block 2bc4
            // 2bc4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bc5 mov rbp,rsp
            rbp = rsp;
            // 2bc8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bca push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bcc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bcd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2bce mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 2bd1 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2bd4 mov rbx,rdi
            rbx = rdi;
            // 2bd7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2bda call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2be0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2be2 je 0x2be8
            if fl.e() { pc = 0x2be8; continue; }
            pc = 0x2be4;
        }
        0x2be4 => {
            // block 2be4
            // 2be4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2be6 jmp 0x2c09
            pc = 0x2c09; continue;
        }
        0x2be8 => {
            // block 2be8
            // 2be8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2beb mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 2bee lea rdx,[rip+0xfc51b] # 0xff110
            rdx = m.env.data_va(80093);
            // 2bf5 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2bf8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2bfd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2c00 mov rdi,rbx
            rdi = rbx;
            // 2c03 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2c06 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c09;
        }
        0x2c09 => {
            // block 2c09
            // 2c09 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2c0d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2c0e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2c10 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2c12 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2c13 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x693c size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000693c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x693c;
    loop {
        match pc {
        0x693c => {
            // block 693c
            // 693c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 693d mov rbp,rsp
            rbp = rsp;
            // 6940 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6942 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6944 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6945 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6946 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 6949 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 694c mov rbx,rdi
            rbx = rdi;
            // 694f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6952 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6958 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 695a je 0x6960
            if fl.e() { pc = 0x6960; continue; }
            pc = 0x695c;
        }
        0x695c => {
            // block 695c
            // 695c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 695e jmp 0x6980
            pc = 0x6980; continue;
        }
        0x6960 => {
            // block 6960
            // 6960 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6963 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 6966 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 6969 movzx esi,cl
            rsi = ((rcx as u8 as u64)) as u32 as u64;
            // 696c or esi,0x44
            { let r = (rsi as u32 as u64) | 0x44u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 696f mov ecx,0x3ff
            rcx = (0x3ffu64) as u32 as u64;
            // 6974 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6977 mov rdi,rbx
            rdi = rbx;
            // 697a mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 697d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6980;
        }
        0x6980 => {
            // block 6980
            // 6980 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6984 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6985 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6987 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6989 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 698a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x102e0 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000102e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x102e0;
    loop {
        match pc {
        0x102e0 => {
            // block 102e0
            // 102e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 102e1 mov rbp,rsp
            rbp = rsp;
            // 102e4 mov rax,QWORD PTR [rdi+0x30]
            rax = m.r64(rdi.wrapping_add(0x30)) as u64;
            // 102e8 mov rcx,QWORD PTR [rdi+0x58]
            rcx = m.r64(rdi.wrapping_add(0x58)) as u64;
            // 102ec cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 102ef jae 0x102f8
            if fl.ae() { pc = 0x102f8; continue; }
            pc = 0x102f1;
        }
        0x102f1 => {
            // block 102f1
            // 102f1 mov QWORD PTR [rdi+0x58],rax
            m.w64(rdi.wrapping_add(0x58), (rax) as u64);
            // 102f5 mov rcx,rax
            rcx = rax;
            pc = 0x102f8;
        }
        0x102f8 => {
            // block 102f8
            // 102f8 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 102fd test BYTE PTR [rdi+0x60],0x8
            fl = logic::<8>(m.r8(rdi.wrapping_add(0x60)) as u64 & 0x8u64);
            // 10301 je 0x10325
            if fl.e() { pc = 0x10325; continue; }
            pc = 0x10303;
        }
        0x10303 => {
            // block 10303
            // 10303 mov rsi,QWORD PTR [rdi+0x20]
            rsi = m.r64(rdi.wrapping_add(0x20)) as u64;
            // 10307 cmp rsi,rcx
            fl = sub::<64>(rsi, rcx).1;
            // 1030a jae 0x10319
            if fl.ae() { pc = 0x10319; continue; }
            pc = 0x1030c;
        }
        0x1030c => {
            // block 1030c
            // 1030c mov rdx,QWORD PTR [rdi+0x18]
            rdx = m.r64(rdi.wrapping_add(0x18)) as u64;
            // 10310 mov QWORD PTR [rdi+0x20],rcx
            m.w64(rdi.wrapping_add(0x20), (rcx) as u64);
            // 10314 mov rsi,rcx
            rsi = rcx;
            // 10317 jmp 0x1031d
            pc = 0x1031d; continue;
        }
        0x10319 => {
            // block 10319
            // 10319 mov rdx,QWORD PTR [rdi+0x18]
            rdx = m.r64(rdi.wrapping_add(0x18)) as u64;
            pc = 0x1031d;
        }
        0x1031d => {
            // block 1031d
            // 1031d cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 10320 jae 0x10325
            if fl.ae() { pc = 0x10325; continue; }
            pc = 0x10322;
        }
        0x10322 => {
            // block 10322
            // 10322 movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            pc = 0x10325;
        }
        0x10325 => {
            // block 10325
            // 10325 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10326 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x12fb4 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012fb4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x12fb4;
    loop {
        match pc {
        0x12fb4 => {
            // block 12fb4
            // 12fb4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12fb5 mov rbp,rsp
            rbp = rsp;
            // 12fb8 mov rax,QWORD PTR [rdi+0x248]
            rax = m.r64(rdi.wrapping_add(0x248)) as u64;
            // 12fbf mov rcx,QWORD PTR [rdi+0x250]
            rcx = m.r64(rdi.wrapping_add(0x250)) as u64;
            // 12fc6 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 12fc8 sub edx,eax
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 12fca movsxd rdx,edx
            rdx = sx::<32>((rdx as u32 as u64)) as u64;
            // 12fcd cmp rdx,0x6
            fl = sub::<64>(rdx, 0x6u64).1;
            // 12fd1 jb 0x12fff
            if fl.b() { pc = 0x12fff; continue; }
            pc = 0x12fd3;
        }
        0x12fd3 => {
            // block 12fd3
            // 12fd3 sub ecx,eax
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // 12fd5 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // 12fd8 lea rcx,[rcx+rax*1-0x6]
            rcx = rcx.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_fffa);
            // 12fdd xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 12fdf lea rsi,[rip+0xecc1a] # 0xffc00
            rsi = m.env.data_va(82893);
            pc = 0x12fe6;
        }
        0x12fe6 => {
            // block 12fe6
            // 12fe6 movzx eax,BYTE PTR [rcx+rdx*1]
            rax = (m.r8(rcx.wrapping_add(rdx)) as u64) as u32 as u64;
            // 12fea movzx edi,BYTE PTR [rdx+rsi*1]
            rdi = (m.r8(rdx.wrapping_add(rsi)) as u64) as u32 as u64;
            // 12fee cmp edi,eax
            fl = sub::<32>((rdi as u32 as u64), (rax as u32 as u64)).1;
            // 12ff0 jne 0x12fff
            if fl.ne() { pc = 0x12fff; continue; }
            pc = 0x12ff2;
        }
        0x12ff2 => {
            // block 12ff2
            // 12ff2 inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // 12ff5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 12ff7 cmp rdx,0x5
            fl = sub::<64>(rdx, 0x5u64).1;
            // 12ffb jbe 0x12fe6
            if fl.be() { pc = 0x12fe6; continue; }
            pc = 0x12ffd;
        }
        0x12ffd => {
            // block 12ffd
            // 12ffd jmp 0x13001
            pc = 0x13001; continue;
        }
        0x12fff => {
            // block 12fff
            // 12fff xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x13001;
        }
        0x13001 => {
            // block 13001
            // 13001 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 13002 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d590 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d590<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2d590;
    loop {
        match pc {
        0x2d590 => {
            // block 2d590
            // 2d590 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d591 mov rbp,rsp
            rbp = rsp;
            // 2d594 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d596 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d597 mov rbx,rdi
            rbx = rdi;
            // 2d59a mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            // 2d59d test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2d5a0 je 0x2d5d0
            if fl.e() { pc = 0x2d5d0; continue; }
            pc = 0x2d5a2;
        }
        0x2d5a2 => {
            // block 2d5a2
            // 2d5a2 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2d5a6 cmp rdi,r14
            fl = sub::<64>(rdi, r14).1;
            // 2d5a9 je 0x2d5c4
            if fl.e() { pc = 0x2d5c4; continue; }
            pc = 0x2d5ab;
        }
        0x2d5ab => {
            // block 2d5ab
            // 2d5ab add rdi,0xffffffffffffffe8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffe8u64);
              rdi = r;
              fl = f; }
            // 2d5af mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 2d5b3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d5b8 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2d5bc cmp rdi,r14
            fl = sub::<64>(rdi, r14).1;
            // 2d5bf jne 0x2d5ab
            if fl.ne() { pc = 0x2d5ab; continue; }
            pc = 0x2d5c1;
        }
        0x2d5c1 => {
            // block 2d5c1
            // 2d5c1 mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            pc = 0x2d5c4;
        }
        0x2d5c4 => {
            // block 2d5c4
            // 2d5c4 mov rdi,r14
            rdi = r14;
            // 2d5c7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d5c8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d5ca pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d5cb jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2d5d0 => {
            // block 2d5d0
            // 2d5d0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d5d1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d5d3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d5d4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9fd8 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9fd8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9fd8;
    loop {
        match pc {
        0xe9fd8 => {
            // block e9fd8
            // e9fd8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9fd9 mov rbp,rsp
            rbp = rsp;
            // e9fdc mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // e9fe1 cmp QWORD PTR [rdi+0x78],0x0
            fl = sub::<64>(m.r64(rdi.wrapping_add(0x78)) as u64, 0x0u64).1;
            // e9fe6 je 0xea025
            if fl.e() { pc = 0xea025; continue; }
            pc = 0xe9fe8;
        }
        0xe9fe8 => {
            // block e9fe8
            // e9fe8 mov rcx,QWORD PTR [rdi+0x18]
            rcx = m.r64(rdi.wrapping_add(0x18)) as u64;
            // e9fec cmp QWORD PTR [rdi+0x10],rcx
            fl = sub::<64>(m.r64(rdi.wrapping_add(0x10)) as u64, rcx).1;
            // e9ff0 jae 0xea025
            if fl.ae() { pc = 0xea025; continue; }
            pc = 0xe9ff2;
        }
        0xe9ff2 => {
            // block e9ff2
            // e9ff2 cmp esi,0xffffffff
            fl = sub::<32>((rsi as u32 as u64), 0xffffffffu64).1;
            // e9ff5 je 0xea01c
            if fl.e() { pc = 0xea01c; continue; }
            pc = 0xe9ff7;
        }
        0xe9ff7 => {
            // block e9ff7
            // e9ff7 test BYTE PTR [rdi+0x188],0x10
            fl = logic::<8>(m.r8(rdi.wrapping_add(0x188)) as u64 & 0x10u64);
            // e9ffe jne 0xea00e
            if fl.ne() { pc = 0xea00e; continue; }
            pc = 0xea000;
        }
        0xea000 => {
            // block ea000
            // ea000 movzx r8d,BYTE PTR [rcx-0x1]
            r8 = (m.r8(rcx.wrapping_add(0xffff_ffff_ffff_ffff)) as u64) as u32 as u64;
            // ea005 movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // ea009 cmp edx,r8d
            fl = sub::<32>((rdx as u32 as u64), (r8 as u32 as u64)).1;
            // ea00c jne 0xea025
            if fl.ne() { pc = 0xea025; continue; }
            pc = 0xea00e;
        }
        0xea00e => {
            // block ea00e
            // ea00e dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // ea011 mov QWORD PTR [rdi+0x18],rcx
            m.w64(rdi.wrapping_add(0x18), (rcx) as u64);
            // ea015 mov BYTE PTR [rcx],sil
            m.w8(rcx, ((rsi as u8 as u64)) as u8);
            // ea018 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // ea01a jmp 0xea025
            pc = 0xea025; continue;
        }
        0xea01c => {
            // block ea01c
            // ea01c dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // ea01f mov QWORD PTR [rdi+0x18],rcx
            m.w64(rdi.wrapping_add(0x18), (rcx) as u64);
            // ea023 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xea025;
        }
        0xea025 => {
            // block ea025
            // ea025 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea026 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5e3a size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005e3a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5e3a;
    loop {
        match pc {
        0x5e3a => {
            // block 5e3a
            // 5e3a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5e3b mov rbp,rsp
            rbp = rsp;
            // 5e3e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5e40 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5e42 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5e43 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5e44 mov r14,rdx
            r14 = rdx;
            // 5e47 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5e4a mov rbx,rdi
            rbx = rdi;
            // 5e4d mov rax,QWORD PTR [rbx+0x228]
            rax = m.r64(rbx.wrapping_add(0x228)) as u64;
            // 5e54 test rax,rax
            fl = logic::<64>(rax & rax);
            // 5e57 jne 0x5e70
            if fl.ne() { pc = 0x5e70; continue; }
            pc = 0x5e59;
        }
        0x5e59 => {
            // block 5e59
            // 5e59 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5e5c mov rdi,rbx
            rdi = rbx;
            // 5e5f call QWORD PTR [rax+0x90]
            { let r = { let t = m.r64(rax.wrapping_add(0x90)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5e65 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5e67 je 0x5e7e
            if fl.e() { pc = 0x5e7e; continue; }
            pc = 0x5e69;
        }
        0x5e69 => {
            // block 5e69
            // 5e69 mov rax,QWORD PTR [rbx+0x228]
            rax = m.r64(rbx.wrapping_add(0x228)) as u64;
            pc = 0x5e70;
        }
        0x5e70 => {
            // block 5e70
            // 5e70 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 5e73 lea rax,[rax+rcx*4]
            rax = rax.wrapping_add(rcx.wrapping_mul(4));
            // 5e77 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 5e7a mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 5e7c jmp 0x5e80
            pc = 0x5e80; continue;
        }
        0x5e7e => {
            // block 5e7e
            // 5e7e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5e80;
        }
        0x5e80 => {
            // block 5e80
            // 5e80 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5e84 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5e85 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5e87 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5e89 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5e8a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5f1c size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005f1c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5f1c;
    loop {
        match pc {
        0x5f1c => {
            // block 5f1c
            // 5f1c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5f1d mov rbp,rsp
            rbp = rsp;
            // 5f20 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5f22 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5f24 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5f25 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5f26 mov r14,rdx
            r14 = rdx;
            // 5f29 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5f2c mov rbx,rdi
            rbx = rdi;
            // 5f2f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5f32 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5f38 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5f3a je 0x5f40
            if fl.e() { pc = 0x5f40; continue; }
            pc = 0x5f3c;
        }
        0x5f3c => {
            // block 5f3c
            // 5f3c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5f3e jmp 0x5f63
            pc = 0x5f63; continue;
        }
        0x5f40 => {
            // block 5f40
            // 5f40 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5f43 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 5f46 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 5f49 movzx esi,cl
            rsi = ((rcx as u8 as u64)) as u32 as u64;
            // 5f4c or esi,0x44
            { let r = (rsi as u32 as u64) | 0x44u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5f4f mov ecx,0x60000
            rcx = (0x60000u64) as u32 as u64;
            // 5f54 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 5f5a mov rdi,rbx
            rdi = rbx;
            // 5f5d mov rdx,r14
            rdx = r14;
            // 5f60 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5f63;
        }
        0x5f63 => {
            // block 5f63
            // 5f63 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5f67 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5f68 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5f6a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5f6c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5f6d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14c40 size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014c40<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x14c40;
    loop {
        match pc {
        0x14c40 => {
            // block 14c40
            // 14c40 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14c41 mov rbp,rsp
            rbp = rsp;
            // 14c44 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 14c46 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14c47 mov rbx,rdi
            rbx = rdi;
            // 14c4a mov rax,QWORD PTR [rip+0x12b74f] # 0x1403a0
            rax = m.r64(m.env.data_va(346989)) as u64;
            // 14c51 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 14c55 mov QWORD PTR [rbx],rcx
            m.w64(rbx, (rcx) as u64);
            // 14c58 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 14c5c mov QWORD PTR [rbx+0x1a8],rax
            m.w64(rbx.wrapping_add(0x1a8), (rax) as u64);
            // 14c63 lea rdi,[rbx+0x10]
            rdi = rbx.wrapping_add(0x10);
            // 14c67 call 0xeb912
            { let r = { let t = m.env.stub_va(0xdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14c6c mov rsi,QWORD PTR [rip+0x12b57d] # 0x1401f0
            rsi = m.r64(m.env.data_va(346557)) as u64;
            // 14c73 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 14c77 lea r14,[rbx+0x1a8]
            r14 = rbx.wrapping_add(0x1a8);
            // 14c7e mov rdi,rbx
            rdi = rbx;
            // 14c81 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14c86 mov rdi,r14
            rdi = r14;
            // 14c89 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14c8a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 14c8c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14c8d jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xff5a size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ff5a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xff5a;
    loop {
        match pc {
        0xff5a => {
            // block ff5a
            // ff5a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ff5b mov rbp,rsp
            rbp = rsp;
            // ff5e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ff60 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ff61 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // ff65 mov rbx,rdi
            rbx = rdi;
            // ff68 lea r14,[rbp-0x30]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ff6c mov rdi,r14
            rdi = r14;
            // ff6f call 0x6474a
            { let r = { let t = m.env.code_va(0x6474a); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ff74 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ff77 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ff79 mov rdi,rbx
            rdi = rbx;
            // ff7c call QWORD PTR [rax+0x1bd8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1bd8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ff82 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ff85 mov rdi,rbx
            rdi = rbx;
            // ff88 mov rsi,r14
            rsi = r14;
            // ff8b call QWORD PTR [rax+0x1cf0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1cf0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ff91 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ff94 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // ff99 mov rdi,rbx
            rdi = rbx;
            // ff9c call QWORD PTR [rax+0x1bd8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1bd8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ffa2 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ffa4 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // ffa8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ffa9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ffab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ffac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xffae size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ffae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xffae;
    loop {
        match pc {
        0xffae => {
            // block ffae
            // ffae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ffaf mov rbp,rsp
            rbp = rsp;
            // ffb2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ffb4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ffb5 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // ffb9 mov rbx,rdi
            rbx = rdi;
            // ffbc lea r14,[rbp-0x30]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // ffc0 mov rdi,r14
            rdi = r14;
            // ffc3 call 0x6479d
            { let r = { let t = m.env.code_va(0x6479d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ffc8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ffcb xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ffcd mov rdi,rbx
            rdi = rbx;
            // ffd0 call QWORD PTR [rax+0x1bd8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1bd8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ffd6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ffd9 mov rdi,rbx
            rdi = rbx;
            // ffdc mov rsi,r14
            rsi = r14;
            // ffdf call QWORD PTR [rax+0x1cf0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1cf0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ffe5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ffe8 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // ffed mov rdi,rbx
            rdi = rbx;
            // fff0 call QWORD PTR [rax+0x1bd8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1bd8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fff6 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // fff8 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // fffc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fffd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ffff pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10000 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4ea0 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004ea0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4ea0;
    loop {
        match pc {
        0x4ea0 => {
            // block 4ea0
            // 4ea0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4ea1 mov rbp,rsp
            rbp = rsp;
            // 4ea4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4ea6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4ea8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4ea9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4eaa mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 4ead mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 4eb0 mov rbx,rdi
            rbx = rdi;
            // 4eb3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4eb6 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ebc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4ebe je 0x4ec4
            if fl.e() { pc = 0x4ec4; continue; }
            pc = 0x4ec0;
        }
        0x4ec0 => {
            // block 4ec0
            // 4ec0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4ec2 jmp 0x4ee8
            pc = 0x4ee8; continue;
        }
        0x4ec4 => {
            // block 4ec4
            // 4ec4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4ec7 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 4eca lea rdx,[rip+0xfa41f] # 0xff2f0
            rdx = m.env.data_va(80573);
            // 4ed1 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4ed4 mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // 4ed9 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // 4edf mov rdi,rbx
            rdi = rbx;
            // 4ee2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 4ee5 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4ee8;
        }
        0x4ee8 => {
            // block 4ee8
            // 4ee8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4eec pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4eed pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4eef pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4ef1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4ef2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5700 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005700<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5700;
    loop {
        match pc {
        0x5700 => {
            // block 5700
            // 5700 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5701 mov rbp,rsp
            rbp = rsp;
            // 5704 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5706 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5708 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5709 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 570a mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 570d mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5710 mov rbx,rdi
            rbx = rdi;
            // 5713 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5716 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 571c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 571e je 0x5724
            if fl.e() { pc = 0x5724; continue; }
            pc = 0x5720;
        }
        0x5720 => {
            // block 5720
            // 5720 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5722 jmp 0x5748
            pc = 0x5748; continue;
        }
        0x5724 => {
            // block 5724
            // 5724 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5727 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 572a lea rdx,[rip+0xf994f] # 0xff080
            rdx = m.env.data_va(79949);
            // 5731 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5734 mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 5739 mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 573f mov rdi,rbx
            rdi = rbx;
            // 5742 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 5745 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5748;
        }
        0x5748 => {
            // block 5748
            // 5748 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 574c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 574d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 574f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5751 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5752 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6aea size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006aea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6aea;
    loop {
        match pc {
        0x6aea => {
            // block 6aea
            // 6aea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6aeb mov rbp,rsp
            rbp = rsp;
            // 6aee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6af0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6af2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6af3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6af4 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 6af7 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 6afa mov rbx,rdi
            rbx = rdi;
            // 6afd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6b00 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6b06 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6b08 je 0x6b0e
            if fl.e() { pc = 0x6b0e; continue; }
            pc = 0x6b0a;
        }
        0x6b0a => {
            // block 6b0a
            // 6b0a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6b0c jmp 0x6b32
            pc = 0x6b32; continue;
        }
        0x6b0e => {
            // block 6b0e
            // 6b0e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6b11 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 6b14 lea rdx,[rip+0xf8565] # 0xff080
            rdx = m.env.data_va(79949);
            // 6b1b mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6b1e mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 6b23 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 6b29 mov rdi,rbx
            rdi = rbx;
            // 6b2c mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 6b2f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6b32;
        }
        0x6b32 => {
            // block 6b32
            // 6b32 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6b36 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6b37 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6b39 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6b3b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6b3c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x260a size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000260a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x260a;
    loop {
        match pc {
        0x260a => {
            // block 260a
            // 260a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 260b mov rbp,rsp
            rbp = rsp;
            // 260e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2610 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2612 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2613 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2614 mov r14,rdx
            r14 = rdx;
            // 2617 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 261a mov rbx,rdi
            rbx = rdi;
            // 261d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2620 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2626 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2628 je 0x262e
            if fl.e() { pc = 0x262e; continue; }
            pc = 0x262a;
        }
        0x262a => {
            // block 262a
            // 262a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 262c jmp 0x2652
            pc = 0x2652; continue;
        }
        0x262e => {
            // block 262e
            // 262e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2631 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 2634 lea rdx,[rip+0xfca45] # 0xff080
            rdx = m.env.data_va(79949);
            // 263b mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 263e mov ecx,0x800
            rcx = (0x800u64) as u32 as u64;
            // 2643 mov r8d,0xb
            r8 = (0xbu64) as u32 as u64;
            // 2649 mov rdi,rbx
            rdi = rbx;
            // 264c mov rdx,r14
            rdx = r14;
            // 264f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2652;
        }
        0x2652 => {
            // block 2652
            // 2652 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2656 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2657 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2659 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 265b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 265c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x27d2 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000027d2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x27d2;
    loop {
        match pc {
        0x27d2 => {
            // block 27d2
            // 27d2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 27d3 mov rbp,rsp
            rbp = rsp;
            // 27d6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 27d8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 27da push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 27db push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 27dc mov r14,rdx
            r14 = rdx;
            // 27df mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 27e2 mov rbx,rdi
            rbx = rdi;
            // 27e5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 27e8 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27ee test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 27f0 je 0x27f6
            if fl.e() { pc = 0x27f6; continue; }
            pc = 0x27f2;
        }
        0x27f2 => {
            // block 27f2
            // 27f2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 27f4 jmp 0x281a
            pc = 0x281a; continue;
        }
        0x27f6 => {
            // block 27f6
            // 27f6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 27f9 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 27fc lea rdx,[rip+0xfc87d] # 0xff080
            rdx = m.env.data_va(79949);
            // 2803 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2806 mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 280b mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 2811 mov rdi,rbx
            rdi = rbx;
            // 2814 mov rdx,r14
            rdx = r14;
            // 2817 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x281a;
        }
        0x281a => {
            // block 281a
            // 281a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 281e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 281f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2821 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2823 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2824 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x287a size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000287a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x287a;
    loop {
        match pc {
        0x287a => {
            // block 287a
            // 287a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 287b mov rbp,rsp
            rbp = rsp;
            // 287e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2880 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2882 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2883 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2884 mov r14,rdx
            r14 = rdx;
            // 2887 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 288a mov rbx,rdi
            rbx = rdi;
            // 288d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2890 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2896 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2898 je 0x289e
            if fl.e() { pc = 0x289e; continue; }
            pc = 0x289a;
        }
        0x289a => {
            // block 289a
            // 289a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 289c jmp 0x28c2
            pc = 0x28c2; continue;
        }
        0x289e => {
            // block 289e
            // 289e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28a1 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 28a4 lea rdx,[rip+0xfc7d5] # 0xff080
            rdx = m.env.data_va(79949);
            // 28ab mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 28ae mov ecx,0x400
            rcx = (0x400u64) as u32 as u64;
            // 28b3 mov r8d,0xa
            r8 = (0xau64) as u32 as u64;
            // 28b9 mov rdi,rbx
            rdi = rbx;
            // 28bc mov rdx,r14
            rdx = r14;
            // 28bf call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x28c2;
        }
        0x28c2 => {
            // block 28c2
            // 28c2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 28c6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28c7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28c9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28cb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28cc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd860 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d860<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd860;
    loop {
        match pc {
        0xd860 => {
            // block d860
            // d860 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d861 mov rbp,rsp
            rbp = rsp;
            // d864 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d866 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d868 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d869 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d86a mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d86d mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d870 mov rbx,rdi
            rbx = rdi;
            // d873 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d876 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d87c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d87e je 0xd884
            if fl.e() { pc = 0xd884; continue; }
            pc = 0xd880;
        }
        0xd880 => {
            // block d880
            // d880 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d882 jmp 0xd8a9
            pc = 0xd8a9; continue;
        }
        0xd884 => {
            // block d884
            // d884 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d887 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // d88a lea rdx,[rip+0xf1b2f] # 0xff3c0
            rdx = m.env.data_va(80781);
            // d891 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // d894 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // d898 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // d89d mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // d8a3 mov rdi,rbx
            rdi = rbx;
            // d8a6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd8a9;
        }
        0xd8a9 => {
            // block d8a9
            // d8a9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d8ad pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d8ae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d8b0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d8b2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8b3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd9d0 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d9d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd9d0;
    loop {
        match pc {
        0xd9d0 => {
            // block d9d0
            // d9d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d9d1 mov rbp,rsp
            rbp = rsp;
            // d9d4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d9d6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d9d8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d9d9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d9da mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d9dd mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d9e0 mov rbx,rdi
            rbx = rdi;
            // d9e3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d9e6 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9ec test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d9ee je 0xd9f4
            if fl.e() { pc = 0xd9f4; continue; }
            pc = 0xd9f0;
        }
        0xd9f0 => {
            // block d9f0
            // d9f0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d9f2 jmp 0xda19
            pc = 0xda19; continue;
        }
        0xd9f4 => {
            // block d9f4
            // d9f4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d9f7 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // d9fa lea rdx,[rip+0xf19bf] # 0xff3c0
            rdx = m.env.data_va(80781);
            // da01 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // da04 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // da08 mov ecx,0x2000000
            rcx = (0x2000000u64) as u32 as u64;
            // da0d mov r8d,0x19
            r8 = (0x19u64) as u32 as u64;
            // da13 mov rdi,rbx
            rdi = rbx;
            // da16 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xda19;
        }
        0xda19 => {
            // block da19
            // da19 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // da1d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // da1e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // da20 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // da22 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // da23 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18afc size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018afc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x18afc;
    loop {
        match pc {
        0x18afc => {
            // block 18afc
            // 18afc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18afd mov rbp,rsp
            rbp = rsp;
            // 18b00 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18b02 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18b04 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 18b06 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18b07 mov r12,rsi
            r12 = rsi;
            // 18b0a mov r14,rdi
            r14 = rdi;
            // 18b0d mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 18b11 cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 18b16 je 0x18b44
            if fl.e() { pc = 0x18b44; continue; }
            pc = 0x18b18;
        }
        0x18b18 => {
            // block 18b18
            // 18b18 lea r15,[rip+0x111775] # 0x12a294
            r15 = m.env.data_va(256609);
            pc = 0x18b1f;
        }
        0x18b1f => {
            // block 18b1f
            // 18b1f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 18b24 mov rdi,r14
            rdi = r14;
            // 18b27 mov rsi,r15
            rsi = r15;
            // 18b2a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18b2f mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 18b31 mov rdi,rax
            rdi = rax;
            // 18b34 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18b39 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 18b3d cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 18b42 jne 0x18b1f
            if fl.ne() { pc = 0x18b1f; continue; }
            pc = 0x18b44;
        }
        0x18b44 => {
            // block 18b44
            // 18b44 mov rax,r14
            rax = r14;
            // 18b47 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18b48 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18b4a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18b4c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18b4e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18b4f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x32210 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00032210<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x32210;
    loop {
        match pc {
        0x32210 => {
            // block 32210
            // 32210 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 32211 mov rbp,rsp
            rbp = rsp;
            // 32214 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 32216 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 32218 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 32219 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3221a mov r14,rsi
            r14 = rsi;
            // 3221d mov r15,rdi
            r15 = rdi;
            // 32220 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 32225 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3222a mov rbx,rax
            rbx = rax;
            // 3222d mov eax,DWORD PTR [r14]
            rax = (m.r32(r14) as u64) as u32 as u64;
            // 32230 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 32233 mov rdi,r15
            rdi = r15;
            // 32236 mov rsi,rbx
            rsi = rbx;
            // 32239 call 0xebb28
            { let r = { let t = m.env.stub_va(0x2f5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3223e mov r14,rax
            r14 = rax;
            // 32241 mov r15b,dl
            r15 = (r15 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 32244 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // 32248 jne 0x32252
            if fl.ne() { pc = 0x32252; continue; }
            pc = 0x3224a;
        }
        0x3224a => {
            // block 3224a
            // 3224a mov rdi,rbx
            rdi = rbx;
            // 3224d call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x32252;
        }
        0x32252 => {
            // block 32252
            // 32252 mov rax,r14
            rax = r14;
            // 32255 mov dl,r15b
            rdx = (rdx & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 32258 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 3225c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3225d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3225f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 32261 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32262 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44abc size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044abc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44abc;
    loop {
        match pc {
        0x44abc => {
            // block 44abc
            // 44abc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44abd mov rbp,rsp
            rbp = rsp;
            // 44ac0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44ac2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44ac4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44ac5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 44ac6 mov r14,rsi
            r14 = rsi;
            // 44ac9 mov r15,rdi
            r15 = rdi;
            // 44acc mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 44ad1 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44ad6 mov rbx,rax
            rbx = rax;
            // 44ad9 mov eax,DWORD PTR [r14]
            rax = (m.r32(r14) as u64) as u32 as u64;
            // 44adc mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 44adf mov rdi,r15
            rdi = r15;
            // 44ae2 mov rsi,rbx
            rsi = rbx;
            // 44ae5 call 0xebb64
            { let r = { let t = m.env.stub_va(0x331); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44aea mov r14,rax
            r14 = rax;
            // 44aed mov r15b,dl
            r15 = (r15 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 44af0 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // 44af4 jne 0x44afe
            if fl.ne() { pc = 0x44afe; continue; }
            pc = 0x44af6;
        }
        0x44af6 => {
            // block 44af6
            // 44af6 mov rdi,rbx
            rdi = rbx;
            // 44af9 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x44afe;
        }
        0x44afe => {
            // block 44afe
            // 44afe mov rax,r14
            rax = r14;
            // 44b01 mov dl,r15b
            rdx = (rdx & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 44b04 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 44b08 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44b09 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44b0b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 44b0d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44b0e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfb8 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000fb8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xfb8;
    loop {
        match pc {
        0xfb8 => {
            // block fb8
            // fb8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fb9 mov rbp,rsp
            rbp = rsp;
            // fbc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // fbe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // fc0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fc1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // fc2 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // fc5 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // fc8 mov rbx,rdi
            rbx = rdi;
            // fcb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fce call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd4 mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // fd7 mov r9,QWORD PTR [rcx+0x30]
            r9 = m.r64(rcx.wrapping_add(0x30)) as u64;
            // fdb mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // fde xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // fe0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // fe2 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // fe6 lea rax,[rip+0xfe003] # 0xfeff0
            rax = m.env.data_va(79805);
            // fed mov esi,DWORD PTR [rax+rdx*4]
            rsi = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // ff0 mov ecx,0x380
            rcx = (0x380u64) as u32 as u64;
            // ff5 mov r8d,0x7
            r8 = (0x7u64) as u32 as u64;
            // ffb mov rdi,rbx
            rdi = rbx;
            // ffe mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 1001 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1005 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1006 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1008 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 100a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 100b jmp r9
            { let r = { let t = r9; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x100e size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000100e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x100e;
    loop {
        match pc {
        0x100e => {
            // block 100e
            // 100e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 100f mov rbp,rsp
            rbp = rsp;
            // 1012 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1014 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1016 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1017 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1018 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 101b mov r14,rsi
            r14 = rsi;
            // 101e mov rbx,rdi
            rbx = rdi;
            // 1021 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1024 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 102a mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 102d mov r9,QWORD PTR [rcx+0x38]
            r9 = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 1031 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 1034 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1036 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1038 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // 103c lea rax,[rip+0xfdfad] # 0xfeff0
            rax = m.env.data_va(79805);
            // 1043 mov esi,DWORD PTR [rax+rdx*4]
            rsi = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 1046 mov ecx,0x380
            rcx = (0x380u64) as u32 as u64;
            // 104b mov r8d,0x7
            r8 = (0x7u64) as u32 as u64;
            // 1051 mov rdi,rbx
            rdi = rbx;
            // 1054 mov rdx,r14
            rdx = r14;
            // 1057 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 105b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 105c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 105e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1060 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1061 jmp r9
            { let r = { let t = r9; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x75a2 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000075a2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x75a2;
    loop {
        match pc {
        0x75a2 => {
            // block 75a2
            // 75a2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 75a3 mov rbp,rsp
            rbp = rsp;
            // 75a6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 75a8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 75aa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 75ab push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 75ac mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 75af mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 75b2 mov rbx,rdi
            rbx = rdi;
            // 75b5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 75b8 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 75bb call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 75c1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 75c3 je 0x75c9
            if fl.e() { pc = 0x75c9; continue; }
            pc = 0x75c5;
        }
        0x75c5 => {
            // block 75c5
            // 75c5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 75c7 jmp 0x75ed
            pc = 0x75ed; continue;
        }
        0x75c9 => {
            // block 75c9
            // 75c9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 75cc mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 75cf lea rdx,[rip+0xf7eaa] # 0xff480
            rdx = m.env.data_va(80973);
            // 75d6 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 75d9 mov ecx,0x40000000
            rcx = (0x40000000u64) as u32 as u64;
            // 75de mov r8d,0x1e
            r8 = (0x1eu64) as u32 as u64;
            // 75e4 mov rdi,rbx
            rdi = rbx;
            // 75e7 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 75ea call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x75ed;
        }
        0x75ed => {
            // block 75ed
            // 75ed add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 75f1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 75f2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 75f4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 75f6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 75f7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x97cc size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000097cc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x97cc;
    loop {
        match pc {
        0x97cc => {
            // block 97cc
            // 97cc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 97cd mov rbp,rsp
            rbp = rsp;
            // 97d0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 97d2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 97d4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 97d5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 97d6 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 97d9 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 97dc mov rbx,rdi
            rbx = rdi;
            // 97df mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 97e2 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 97e5 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 97eb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 97ed je 0x97f3
            if fl.e() { pc = 0x97f3; continue; }
            pc = 0x97ef;
        }
        0x97ef => {
            // block 97ef
            // 97ef xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 97f1 jmp 0x9817
            pc = 0x9817; continue;
        }
        0x97f3 => {
            // block 97f3
            // 97f3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 97f6 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 97f9 lea rdx,[rip+0xf5c80] # 0xff480
            rdx = m.env.data_va(80973);
            // 9800 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 9803 mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 9808 mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 980e mov rdi,rbx
            rdi = rbx;
            // 9811 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 9814 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9817;
        }
        0x9817 => {
            // block 9817
            // 9817 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 981b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 981c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 981e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 9820 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9821 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x9878 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00009878<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x9878;
    loop {
        match pc {
        0x9878 => {
            // block 9878
            // 9878 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 9879 mov rbp,rsp
            rbp = rsp;
            // 987c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 987e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9880 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9881 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 9882 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 9885 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 9888 mov rbx,rdi
            rbx = rdi;
            // 988b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 988e mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 9891 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9897 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 9899 je 0x989f
            if fl.e() { pc = 0x989f; continue; }
            pc = 0x989b;
        }
        0x989b => {
            // block 989b
            // 989b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 989d jmp 0x98c3
            pc = 0x98c3; continue;
        }
        0x989f => {
            // block 989f
            // 989f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 98a2 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 98a5 lea rdx,[rip+0xf5bd4] # 0xff480
            rdx = m.env.data_va(80973);
            // 98ac mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 98af mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 98b4 mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 98ba mov rdi,rbx
            rdi = rbx;
            // 98bd mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 98c0 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x98c3;
        }
        0x98c3 => {
            // block 98c3
            // 98c3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 98c7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 98c8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 98ca pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 98cc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 98cd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e174 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e174<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e174;
    loop {
        match pc {
        0x2e174 => {
            // block 2e174
            // 2e174 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e175 mov rbp,rsp
            rbp = rsp;
            // 2e178 lea rax,[rip+0x113371] # 0x1414f0
            rax = m.env.data_va(351421);
            // 2e17f mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 2e182 mov WORD PTR [rdi+0x8],0x0
            m.w16(rdi.wrapping_add(0x8), (0x0u64) as u16);
            // 2e188 mov QWORD PTR [rdi+0x48],0x0
            m.w64(rdi.wrapping_add(0x48), (0x0u64) as u64);
            // 2e190 mov QWORD PTR [rdi+0x40],0x0
            m.w64(rdi.wrapping_add(0x40), (0x0u64) as u64);
            // 2e198 mov QWORD PTR [rdi+0x38],0x0
            m.w64(rdi.wrapping_add(0x38), (0x0u64) as u64);
            // 2e1a0 mov QWORD PTR [rdi+0x30],0x0
            m.w64(rdi.wrapping_add(0x30), (0x0u64) as u64);
            // 2e1a8 mov QWORD PTR [rdi+0x28],0x0
            m.w64(rdi.wrapping_add(0x28), (0x0u64) as u64);
            // 2e1b0 mov QWORD PTR [rdi+0x20],0x0
            m.w64(rdi.wrapping_add(0x20), (0x0u64) as u64);
            // 2e1b8 mov QWORD PTR [rdi+0x18],0x0
            m.w64(rdi.wrapping_add(0x18), (0x0u64) as u64);
            // 2e1c0 mov QWORD PTR [rdi+0x10],0x0
            m.w64(rdi.wrapping_add(0x10), (0x0u64) as u64);
            // 2e1c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e1c9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdf1a4 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000df1a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdf1a4;
    loop {
        match pc {
        0xdf1a4 => {
            // block df1a4
            // df1a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df1a5 mov rbp,rsp
            rbp = rsp;
            // df1a8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df1a9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // df1aa mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // df1ac call 0xec290
            { let r = { let t = m.env.stub_va(0xa5d); m.call(t, &[], &[x0 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df1b1 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // df1b3 je 0xdf1cf
            if fl.e() { pc = 0xdf1cf; continue; }
            pc = 0xdf1b5;
        }
        0xdf1b5 => {
            // block df1b5
            // df1b5 mulsd xmm0,QWORD PTR [rip+0x23dfb] # 0x102fb8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(96133)) as u128))).to_bits());
            // df1bd mulsd xmm0,QWORD PTR [rip+0x23de3] # 0x102fa8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(96117)) as u128))).to_bits());
            // df1c5 addsd xmm0,QWORD PTR [rip+0x23df3] # 0x102fc0
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(96141)) as u128))).to_bits());
            // df1cd jmp 0xdf1e7
            pc = 0xdf1e7; continue;
        }
        0xdf1cf => {
            // block df1cf
            // df1cf mulsd xmm0,QWORD PTR [rip+0x23dc9] # 0x102fa0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(96109)) as u128))).to_bits());
            // df1d7 mulsd xmm0,QWORD PTR [rip+0x23dc9] # 0x102fa8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(96117)) as u128))).to_bits());
            // df1df addsd xmm0,QWORD PTR [rip+0x23dc9] # 0x102fb0
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(96125)) as u128))).to_bits());
            pc = 0xdf1e7;
        }
        0xdf1e7 => {
            // block df1e7
            // df1e7 addsd xmm0,QWORD PTR [rip+0x1f6e9] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // df1ef cvttsd2si eax,xmm0
            rax = (cvtt32(f64_of(x0)) as u64) as u32 as u64;
            // df1f3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // df1f7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // df1f8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // df1f9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
