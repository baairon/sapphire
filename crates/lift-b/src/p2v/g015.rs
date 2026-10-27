//! Mechanically lifted functions (scripts/lift_run.py), module g015. Each one is checked against the original
//! in the world harness (tests/world_g015.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe126c size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e126c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe126c;
    loop {
        match pc {
        0xe126c => {
            // block e126c
            // e126c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e126d mov rbp,rsp
            rbp = rsp;
            // e1270 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e1272 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1274 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1275 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e1276 mov r14,rdx
            r14 = rdx;
            // e1279 mov rbx,rdi
            rbx = rdi;
            // e127c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e127f mov edx,DWORD PTR [rsi]
            rdx = (m.r32(rsi) as u64) as u32 as u64;
            // e1281 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // e1284 mov esi,0x14
            rsi = (0x14u64) as u32 as u64;
            // e1289 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e128e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1291 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1294 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e1296 je 0xe12b4
            if fl.e() { pc = 0xe12b4; continue; }
            pc = 0xe1298;
        }
        0xe1298 => {
            // block e1298
            // e1298 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e129b mov edx,DWORD PTR [r14]
            rdx = (m.r32(r14) as u64) as u32 as u64;
            // e129e mov esi,0x10a
            rsi = (0x10au64) as u32 as u64;
            // e12a3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e12a8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e12ab mov rdi,rbx
            rdi = rbx;
            // e12ae call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e12b1 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xe12b4;
        }
        0xe12b4 => {
            // block e12b4
            // e12b4 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // e12b7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e12bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e12bc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e12be pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e12c0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e12c1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x57a8 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000057a8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x57a8;
    loop {
        match pc {
        0x57a8 => {
            // block 57a8
            // 57a8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 57a9 mov rbp,rsp
            rbp = rsp;
            // 57ac push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 57ae push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 57b0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 57b1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 57b2 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 57b5 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 57b8 mov rbx,rdi
            rbx = rdi;
            // 57bb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 57be call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 57c4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 57c6 je 0x57cc
            if fl.e() { pc = 0x57cc; continue; }
            pc = 0x57c8;
        }
        0x57c8 => {
            // block 57c8
            // 57c8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 57ca jmp 0x57f4
            pc = 0x57f4; continue;
        }
        0x57cc => {
            // block 57cc
            // 57cc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 57cf cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 57d3 sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 57d6 movzx esi,cl
            rsi = ((rcx as u8 as u64)) as u32 as u64;
            // 57d9 add esi,0xa5
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xa5u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 57df movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // 57e3 mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // 57e8 mov r8d,0x2
            r8 = (0x2u64) as u32 as u64;
            // 57ee mov rdi,rbx
            rdi = rbx;
            // 57f1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x57f4;
        }
        0x57f4 => {
            // block 57f4
            // 57f4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 57f8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 57f9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 57fb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 57fd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 57fe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3680 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3680<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd3680;
    loop {
        match pc {
        0xd3680 => {
            // block d3680
            // d3680 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3681 mov rbp,rsp
            rbp = rsp;
            // d3684 sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // d3688 xorps xmm0,xmm0
            x0 = 0;
            // d368b movaps XMMWORD PTR [rbp-0x10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fff0), x0);
            // d368f movaps XMMWORD PTR [rbp-0x20],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), x0);
            // d3693 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // d3697 mov DWORD PTR [rbp-0x30],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x2u64) as u32);
            // d369e mov DWORD PTR [rbp-0x2c],0x30
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x30u64) as u32);
            // d36a5 mov DWORD PTR [rbp-0x18],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), ((rsi as u32 as u64)) as u32);
            // d36a8 mov DWORD PTR [rbp-0x14],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rdx as u32 as u64)) as u32);
            // d36ab mov DWORD PTR [rbp-0x10],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u32);
            // d36b2 mov DWORD PTR [rbp-0xc],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rcx as u32 as u64)) as u32);
            // d36b5 mov DWORD PTR [rbp-0x8],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff8), ((r8 as u32 as u64)) as u32);
            // d36b9 mov BYTE PTR [rbp-0x4],0x1
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x1u64) as u8);
            // d36bd mov BYTE PTR [rbp-0x3],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fffd), (0x0u64) as u8);
            // d36c1 mov BYTE PTR [rbp-0x2],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fffe), (0x0u64) as u8);
            // d36c5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d36c8 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d36cc call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d36d2 add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // d36d6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d36d7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2902e size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002902e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2902e;
    loop {
        match pc {
        0x2902e => {
            // block 2902e
            // 2902e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2902f mov rbp,rsp
            rbp = rsp;
            // 29032 mov rax,rsi
            rax = rsi;
            // 29035 mov r9,rdi
            r9 = rdi;
            // 29038 mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 2903b test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2903e je 0x29084
            if fl.e() { pc = 0x29084; continue; }
            pc = 0x29040;
        }
        0x29040 => {
            // block 29040
            // 29040 mov r10d,DWORD PTR [rax+0x8]
            r10 = (m.r32(rax.wrapping_add(0x8)) as u64) as u32 as u64;
            // 29044 test r10d,r10d
            fl = logic::<32>((r10 as u32 as u64) & (r10 as u32 as u64));
            // 29047 je 0x29084
            if fl.e() { pc = 0x29084; continue; }
            pc = 0x29049;
        }
        0x29049 => {
            // block 29049
            // 29049 mov rdi,QWORD PTR [r9]
            rdi = m.r64(r9) as u64;
            // 2904c test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2904f je 0x29084
            if fl.e() { pc = 0x29084; continue; }
            pc = 0x29051;
        }
        0x29051 => {
            // block 29051
            // 29051 mov r9d,DWORD PTR [r9+0x8]
            r9 = (m.r32(r9.wrapping_add(0x8)) as u64) as u32 as u64;
            // 29055 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 29057 test r9d,r9d
            fl = logic::<32>((r9 as u32 as u64) & (r9 as u32 as u64));
            // 2905a je 0x29086
            if fl.e() { pc = 0x29086; continue; }
            pc = 0x2905c;
        }
        0x2905c => {
            // block 2905c
            // 2905c lea r11d,[r8+rdx*1]
            r11 = (r8.wrapping_add(rdx)) as u32 as u64;
            // 29060 cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // 29063 ja 0x29086
            if fl.a() { pc = 0x29086; continue; }
            pc = 0x29065;
        }
        0x29065 => {
            // block 29065
            // 29065 lea r10d,[r8+rcx*1]
            r10 = (r8.wrapping_add(rcx)) as u32 as u64;
            // 29069 cmp r10d,r9d
            fl = sub::<32>((r10 as u32 as u64), (r9 as u32 as u64)).1;
            // 2906c ja 0x29086
            if fl.a() { pc = 0x29086; continue; }
            pc = 0x2906e;
        }
        0x2906e => {
            // block 2906e
            // 2906e mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 29070 add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 29073 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 29075 add rdi,rax
            { let (r, f) = add::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 29078 mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            // 2907b call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29080 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 29082 jmp 0x29086
            pc = 0x29086; continue;
        }
        0x29084 => {
            // block 29084
            // 29084 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x29086;
        }
        0x29086 => {
            // block 29086
            // 29086 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29087 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xda88 size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000da88<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xda88;
    loop {
        match pc {
        0xda88 => {
            // block da88
            // da88 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // da89 mov rbp,rsp
            rbp = rsp;
            // da8c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // da8e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // da8f mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // da92 mov rbx,rdi
            rbx = rdi;
            // da95 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // da98 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // da9e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // daa0 je 0xdaa6
            if fl.e() { pc = 0xdaa6; continue; }
            pc = 0xdaa2;
        }
        0xdaa2 => {
            // block daa2
            // daa2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // daa4 jmp 0xdade
            pc = 0xdade; continue;
        }
        0xdaa6 => {
            // block daa6
            // daa6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // daa9 mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // daae xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dab0 mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // dab5 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // dabb mov rdi,rbx
            rdi = rbx;
            // dabe call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dac1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dac4 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // dac8 mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // dacd mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // dad2 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // dad8 mov rdi,rbx
            rdi = rbx;
            // dadb call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdade;
        }
        0xdade => {
            // block dade
            // dade pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dadf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dae1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dae2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdb6a size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000db6a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdb6a;
    loop {
        match pc {
        0xdb6a => {
            // block db6a
            // db6a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // db6b mov rbp,rsp
            rbp = rsp;
            // db6e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // db70 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // db71 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // db74 mov rbx,rdi
            rbx = rdi;
            // db77 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db7a call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db80 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // db82 je 0xdb88
            if fl.e() { pc = 0xdb88; continue; }
            pc = 0xdb84;
        }
        0xdb84 => {
            // block db84
            // db84 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db86 jmp 0xdbc0
            pc = 0xdbc0; continue;
        }
        0xdb88 => {
            // block db88
            // db88 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db8b mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // db90 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db92 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // db97 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // db9d mov rdi,rbx
            rdi = rbx;
            // dba0 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dba3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dba6 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // dbaa mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // dbaf mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // dbb4 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // dbba mov rdi,rbx
            rdi = rbx;
            // dbbd call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdbc0;
        }
        0xdbc0 => {
            // block dbc0
            // dbc0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dbc1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dbc3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dbc4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5388 size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5388<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5388;
    loop {
        match pc {
        0xb5388 => {
            // block b5388
            // b5388 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5389 mov rbp,rsp
            rbp = rsp;
            // b538c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b538e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5390 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5391 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b5392 mov r14,rsi
            r14 = rsi;
            // b5395 mov r15,rdi
            r15 = rdi;
            // b5398 mov edi,0x30
            rdi = (0x30u64) as u32 as u64;
            // b539d call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b53a2 mov rbx,rax
            rbx = rax;
            // b53a5 mov eax,DWORD PTR [r14]
            rax = (m.r32(r14) as u64) as u32 as u64;
            // b53a8 mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // b53ab mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // b53af mov QWORD PTR [rbx+0x28],rax
            m.w64(rbx.wrapping_add(0x28), (rax) as u64);
            // b53b3 mov rdi,r15
            rdi = r15;
            // b53b6 mov rsi,rbx
            rsi = rbx;
            // b53b9 call 0xebc84
            { let r = { let t = m.env.stub_va(0x451); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b53be mov r14,rax
            r14 = rax;
            // b53c1 mov r15b,dl
            r15 = (r15 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // b53c4 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // b53c8 jne 0xb53d2
            if fl.ne() { pc = 0xb53d2; continue; }
            pc = 0xb53ca;
        }
        0xb53ca => {
            // block b53ca
            // b53ca mov rdi,rbx
            rdi = rbx;
            // b53cd call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb53d2;
        }
        0xb53d2 => {
            // block b53d2
            // b53d2 mov rax,r14
            rax = r14;
            // b53d5 mov dl,r15b
            rdx = (rdx & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b53d8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b53dc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b53dd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b53df pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b53e1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b53e2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28a4f size=93
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028a4f<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28a4f;
    loop {
        match pc {
        0x28a4f => {
            // block 28a4f
            // 28a4f push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28a50 mov rbp,rsp
            rbp = rsp;
            // 28a53 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28a55 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28a57 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28a58 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 28a59 mov r15,rsi
            r15 = rsi;
            // 28a5c mov r14,rdi
            r14 = rdi;
            // 28a5f mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 28a62 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 28a66 jmp 0x28a99
            pc = 0x28a99; continue;
        }
        0x28a68 => {
            // block 28a68
            // 28a68 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 28a6c mov rdi,r14
            rdi = r14;
            // 28a6f call 0xebace
            { let r = { let t = m.env.stub_va(0x29b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28a74 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 28a78 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28a7b je 0x28a8a
            if fl.e() { pc = 0x28a8a; continue; }
            pc = 0x28a7d;
        }
        0x28a7d => {
            // block 28a7d
            // 28a7d mov rbx,rax
            rbx = rax;
            // 28a80 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28a83 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28a86 jne 0x28a7d
            if fl.ne() { pc = 0x28a7d; continue; }
            pc = 0x28a88;
        }
        0x28a88 => {
            // block 28a88
            // 28a88 jmp 0x28a99
            pc = 0x28a99; continue;
        }
        0x28a8a => {
            // block 28a8a
            // 28a8a mov rax,rbx
            rax = rbx;
            pc = 0x28a8d;
        }
        0x28a8d => {
            // block 28a8d
            // 28a8d mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 28a91 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 28a94 mov rax,rbx
            rax = rbx;
            // 28a97 jne 0x28a8d
            if fl.ne() { pc = 0x28a8d; continue; }
            pc = 0x28a99;
        }
        0x28a99 => {
            // block 28a99
            // 28a99 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 28a9c jne 0x28a68
            if fl.ne() { pc = 0x28a68; continue; }
            pc = 0x28a9e;
        }
        0x28a9e => {
            // block 28a9e
            // 28a9e mov rax,r14
            rax = r14;
            // 28aa1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 28aa5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28aa6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28aa8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28aaa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28aab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4c82 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004c82<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4c82;
    loop {
        match pc {
        0x4c82 => {
            // block 4c82
            // 4c82 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4c83 mov rbp,rsp
            rbp = rsp;
            // 4c86 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4c88 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4c8a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4c8b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4c8c mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 4c8f mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 4c92 mov rbx,rdi
            rbx = rdi;
            // 4c95 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4c98 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c9e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4ca0 je 0x4ca6
            if fl.e() { pc = 0x4ca6; continue; }
            pc = 0x4ca2;
        }
        0x4ca2 => {
            // block 4ca2
            // 4ca2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4ca4 jmp 0x4cd4
            pc = 0x4cd4; continue;
        }
        0x4ca6 => {
            // block 4ca6
            // 4ca6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4ca9 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 4cac lea rcx,[rip+0xfa5ad] # 0xff260
            rcx = m.env.data_va(80429);
            // 4cb3 mov esi,DWORD PTR [rcx+rdx*4]
            rsi = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4cb6 lea rcx,[rip+0xfa5d3] # 0xff290
            rcx = m.env.data_va(80477);
            // 4cbd mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4cc0 lea rdi,[rip+0xfa5f9] # 0xff2c0
            rdi = m.env.data_va(80525);
            // 4cc7 mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4ccb mov rdi,rbx
            rdi = rbx;
            // 4cce mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 4cd1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4cd4;
        }
        0x4cd4 => {
            // block 4cd4
            // 4cd4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4cd8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4cd9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4cdb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4cdd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4cde ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28bc0 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028bc0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28bc0;
    loop {
        match pc {
        0x28bc0 => {
            // block 28bc0
            // 28bc0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28bc1 mov rbp,rsp
            rbp = rsp;
            // 28bc4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28bc6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28bc8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28bc9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 28bca mov r15,rsi
            r15 = rsi;
            // 28bcd mov r14,rdi
            r14 = rdi;
            // 28bd0 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 28bd3 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 28bd7 jmp 0x28c0a
            pc = 0x28c0a; continue;
        }
        0x28bd9 => {
            // block 28bd9
            // 28bd9 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 28bdd mov rdi,r14
            rdi = r14;
            // 28be0 call 0xebada
            { let r = { let t = m.env.stub_va(0x2a7); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28be5 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 28be9 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28bec je 0x28bfb
            if fl.e() { pc = 0x28bfb; continue; }
            pc = 0x28bee;
        }
        0x28bee => {
            // block 28bee
            // 28bee mov rbx,rax
            rbx = rax;
            // 28bf1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28bf4 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28bf7 jne 0x28bee
            if fl.ne() { pc = 0x28bee; continue; }
            pc = 0x28bf9;
        }
        0x28bf9 => {
            // block 28bf9
            // 28bf9 jmp 0x28c0a
            pc = 0x28c0a; continue;
        }
        0x28bfb => {
            // block 28bfb
            // 28bfb mov rax,rbx
            rax = rbx;
            pc = 0x28bfe;
        }
        0x28bfe => {
            // block 28bfe
            // 28bfe mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 28c02 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 28c05 mov rax,rbx
            rax = rbx;
            // 28c08 jne 0x28bfe
            if fl.ne() { pc = 0x28bfe; continue; }
            pc = 0x28c0a;
        }
        0x28c0a => {
            // block 28c0a
            // 28c0a cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 28c0d jne 0x28bd9
            if fl.ne() { pc = 0x28bd9; continue; }
            pc = 0x28c0f;
        }
        0x28c0f => {
            // block 28c0f
            // 28c0f mov rax,r14
            rax = r14;
            // 28c12 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 28c16 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28c17 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28c19 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28c1b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28c1c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x681f4 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000681f4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x681f4;
    loop {
        match pc {
        0x681f4 => {
            // block 681f4
            // 681f4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 681f5 mov rbp,rsp
            rbp = rsp;
            // 681f8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 681fa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 681fc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 681fd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 681fe mov r15,rdi
            r15 = rdi;
            // 68201 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 68203 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 68206 ja 0x68246
            if fl.a() { pc = 0x68246; continue; }
            pc = 0x68208;
        }
        0x68208 => {
            // block 68208
            // 68208 test dx,dx
            fl = logic::<16>((rdx as u16 as u64) & (rdx as u16 as u64));
            // 6820b je 0x68246
            if fl.e() { pc = 0x68246; continue; }
            pc = 0x6820d;
        }
        0x6820d => {
            // block 6820d
            // 6820d mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 6820f lea rcx,[rip+0x99a7a] # 0x101c90
            rcx = m.env.data_va(91229);
            // 68216 mov r14d,DWORD PTR [rcx+rax*4]
            r14 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 6821a mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 6821f sub ebx,edx
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rdx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x68221;
        }
        0x68221 => {
            // block 68221
            // 68221 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 68224 mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // 68229 mov rdi,r15
            rdi = r15;
            // 6822c mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 6822f call QWORD PTR [rax+0xc0]
            { let r = { let t = m.r64(rax.wrapping_add(0xc0)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68235 test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // 68238 je 0x68246
            if fl.e() { pc = 0x68246; continue; }
            pc = 0x6823a;
        }
        0x6823a => {
            // block 6823a
            // 6823a mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 6823c xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6823f lea ebx,[rbx+0x1]
            rbx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 68242 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 68244 je 0x68221
            if fl.e() { pc = 0x68221; continue; }
            pc = 0x68246;
        }
        0x68246 => {
            // block 68246
            // 68246 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6824a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6824b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6824d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6824f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68250 ret
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
            { let r = { let t = m.env.stub_va(0xa57); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d31c7 cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // d31ca je 0xd31da
            if fl.e() { pc = 0xd31da; continue; }
            pc = 0xd31cc;
        }
        0xd31cc => {
            // block d31cc
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
            // d31d8 jmp 0xd31df
            pc = 0xd31df; continue;
        }
        0xd31da => {
            // block d31da
            // d31da mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            pc = 0xd31df;
        }
        0xd31df => {
            // block d31df
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

/// parity: P2v @0x15f2 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000015f2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x15f2;
    loop {
        match pc {
        0x15f2 => {
            // block 15f2
            // 15f2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15f3 mov rbp,rsp
            rbp = rsp;
            // 15f6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15f8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15fa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15fb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15fc mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 15ff mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 1602 mov rbx,rdi
            rbx = rdi;
            // 1605 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1608 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 160e mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 1611 mov r9,QWORD PTR [rcx+0x30]
            r9 = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 1615 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 1618 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 161a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 161c cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // 1620 lea rax,[rip+0xfdc39] # 0xff260
            rax = m.env.data_va(80429);
            // 1627 mov esi,DWORD PTR [rax+rdx*4]
            rsi = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 162a lea rax,[rip+0xfd9ef] # 0xff020
            rax = m.env.data_va(79853);
            // 1631 mov ecx,DWORD PTR [rax+rdx*4]
            rcx = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 1634 lea rax,[rip+0xfda15] # 0xff050
            rax = m.env.data_va(79901);
            // 163b mov r8d,DWORD PTR [rax+rdx*4]
            r8 = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 163f mov rdi,rbx
            rdi = rbx;
            // 1642 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 1645 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1649 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 164a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 164c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 164e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 164f jmp r9
            { let r = { let t = r9; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e114 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e114<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e114;
    loop {
        match pc {
        0x2e114 => {
            // block 2e114
            // 2e114 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e115 mov rbp,rsp
            rbp = rsp;
            // 2e118 mov edi,0x50
            rdi = (0x50u64) as u32 as u64;
            // 2e11d call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e122 lea rcx,[rip+0x1133c7] # 0x1414f0
            rcx = m.env.data_va(351421);
            // 2e129 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 2e12c mov WORD PTR [rax+0x8],0x0
            m.w16(rax.wrapping_add(0x8), (0x0u64) as u16);
            // 2e132 mov QWORD PTR [rax+0x48],0x0
            m.w64(rax.wrapping_add(0x48), (0x0u64) as u64);
            // 2e13a mov QWORD PTR [rax+0x40],0x0
            m.w64(rax.wrapping_add(0x40), (0x0u64) as u64);
            // 2e142 mov QWORD PTR [rax+0x38],0x0
            m.w64(rax.wrapping_add(0x38), (0x0u64) as u64);
            // 2e14a mov QWORD PTR [rax+0x30],0x0
            m.w64(rax.wrapping_add(0x30), (0x0u64) as u64);
            // 2e152 mov QWORD PTR [rax+0x28],0x0
            m.w64(rax.wrapping_add(0x28), (0x0u64) as u64);
            // 2e15a mov QWORD PTR [rax+0x20],0x0
            m.w64(rax.wrapping_add(0x20), (0x0u64) as u64);
            // 2e162 mov QWORD PTR [rax+0x18],0x0
            m.w64(rax.wrapping_add(0x18), (0x0u64) as u64);
            // 2e16a mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2e172 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e173 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba6da size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba6da<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba6da;
    loop {
        match pc {
        0xba6da => {
            // block ba6da
            // ba6da push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba6db mov rbp,rsp
            rbp = rsp;
            // ba6de mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // ba6e2 test rax,rax
            fl = logic::<64>(rax & rax);
            // ba6e5 je 0xba71d
            if fl.e() { pc = 0xba71d; continue; }
            pc = 0xba6e7;
        }
        0xba6e7 => {
            // block ba6e7
            // ba6e7 movzx ecx,BYTE PTR [rdx]
            rcx = (m.r8(rdx) as u64) as u32 as u64;
            // ba6ea jmp 0xba6ef
            pc = 0xba6ef; continue;
        }
        0xba6ec => {
            // block ba6ec
            // ba6ec mov rax,rdx
            rax = rdx;
            pc = 0xba6ef;
        }
        0xba6ef => {
            // block ba6ef
            // ba6ef movzx edx,BYTE PTR [rax+0x19]
            rdx = (m.r8(rax.wrapping_add(0x19)) as u64) as u32 as u64;
            // ba6f3 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // ba6f5 jae 0xba701
            if fl.ae() { pc = 0xba701; continue; }
            pc = 0xba6f7;
        }
        0xba6f7 => {
            // block ba6f7
            // ba6f7 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // ba6fa test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // ba6fd jne 0xba6ec
            if fl.ne() { pc = 0xba6ec; continue; }
            pc = 0xba6ff;
        }
        0xba6ff => {
            // block ba6ff
            // ba6ff jmp 0xba729
            pc = 0xba729; continue;
        }
        0xba701 => {
            // block ba701
            // ba701 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // ba704 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // ba706 jae 0xba731
            if fl.ae() { pc = 0xba731; continue; }
            pc = 0xba708;
        }
        0xba708 => {
            // block ba708
            // ba708 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // ba70c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // ba70f jne 0xba6ec
            if fl.ne() { pc = 0xba6ec; continue; }
            pc = 0xba711;
        }
        0xba711 => {
            // block ba711
            // ba711 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // ba714 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // ba718 mov rsi,rax
            rsi = rax;
            // ba71b jmp 0xba734
            pc = 0xba734; continue;
        }
        0xba71d => {
            // block ba71d
            // ba71d add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // ba721 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // ba724 mov rsi,rdi
            rsi = rdi;
            // ba727 jmp 0xba734
            pc = 0xba734; continue;
        }
        0xba729 => {
            // block ba729
            // ba729 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // ba72c mov rsi,rax
            rsi = rax;
            // ba72f jmp 0xba734
            pc = 0xba734; continue;
        }
        0xba731 => {
            // block ba731
            // ba731 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xba734;
        }
        0xba734 => {
            // block ba734
            // ba734 mov rax,rsi
            rax = rsi;
            // ba737 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba738 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4bd4 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4bd4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4bd4;
    loop {
        match pc {
        0xe4bd4 => {
            // block e4bd4
            // e4bd4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4bd5 mov rbp,rsp
            rbp = rsp;
            // e4bd8 lea rax,[rip+0x616c9] # 0x1462a8
            rax = m.env.data_va(371317);
            // e4bdf mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // e4be2 mov QWORD PTR [rdi+0x90],0x0
            m.w64(rdi.wrapping_add(0x90), (0x0u64) as u64);
            // e4bed mov BYTE PTR [rdi+0x8c],0x0
            m.w8(rdi.wrapping_add(0x8c), (0x0u64) as u8);
            // e4bf4 mov WORD PTR [rdi+0xc],0x0
            m.w16(rdi.wrapping_add(0xc), (0x0u64) as u16);
            // e4bfa mov DWORD PTR [rdi+0x8],0x0
            m.w32(rdi.wrapping_add(0x8), (0x0u64) as u32);
            // e4c01 mov QWORD PTR [rdi+0x48],0x0
            m.w64(rdi.wrapping_add(0x48), (0x0u64) as u64);
            // e4c09 mov QWORD PTR [rdi+0x40],0x0
            m.w64(rdi.wrapping_add(0x40), (0x0u64) as u64);
            // e4c11 mov QWORD PTR [rdi+0x38],0x0
            m.w64(rdi.wrapping_add(0x38), (0x0u64) as u64);
            // e4c19 mov QWORD PTR [rdi+0x30],0x0
            m.w64(rdi.wrapping_add(0x30), (0x0u64) as u64);
            // e4c21 mov QWORD PTR [rdi+0x28],0x0
            m.w64(rdi.wrapping_add(0x28), (0x0u64) as u64);
            // e4c29 mov QWORD PTR [rdi+0x20],0x0
            m.w64(rdi.wrapping_add(0x20), (0x0u64) as u64);
            // e4c31 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4c32 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44c68 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044c68<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44c68;
    loop {
        match pc {
        0x44c68 => {
            // block 44c68
            // 44c68 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44c69 mov rbp,rsp
            rbp = rsp;
            // 44c6c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44c6e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44c70 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44c71 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 44c72 mov r14,rsi
            r14 = rsi;
            // 44c75 mov rbx,rdi
            rbx = rdi;
            // 44c78 lea r15,[rbx+0x8]
            r15 = rbx.wrapping_add(0x8);
            // 44c7c mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 44c80 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44c85 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 44c88 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 44c8b mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 44c8f mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 44c93 mov rcx,QWORD PTR [r14+0x10]
            rcx = m.r64(r14.wrapping_add(0x10)) as u64;
            // 44c97 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // 44c9b test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 44c9e je 0x44cbc
            if fl.e() { pc = 0x44cbc; continue; }
            pc = 0x44ca0;
        }
        0x44ca0 => {
            // block 44ca0
            // 44ca0 lea rcx,[r14+0x8]
            rcx = r14.wrapping_add(0x8);
            // 44ca4 mov QWORD PTR [rax+0x10],r15
            m.w64(rax.wrapping_add(0x10), (r15) as u64);
            // 44ca8 mov QWORD PTR [r14],rcx
            m.w64(r14, (rcx) as u64);
            // 44cab mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 44cb3 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 44cba jmp 0x44cbf
            pc = 0x44cbf; continue;
        }
        0x44cbc => {
            // block 44cbc
            // 44cbc mov QWORD PTR [rbx],r15
            m.w64(rbx, (r15) as u64);
            pc = 0x44cbf;
        }
        0x44cbf => {
            // block 44cbf
            // 44cbf add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 44cc3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44cc4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44cc6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 44cc8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44cc9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4201c size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004201c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4201c;
    loop {
        match pc {
        0x4201c => {
            // block 4201c
            // 4201c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4201d mov rbp,rsp
            rbp = rsp;
            // 42020 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42022 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42024 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42025 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42026 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 42028 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 4202b mov r14,rdi
            r14 = rdi;
            // 4202e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 42030 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 42034 ja 0x42074
            if fl.a() { pc = 0x42074; continue; }
            pc = 0x42036;
        }
        0x42036 => {
            // block 42036
            // 42036 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 42039 ja 0x42074
            if fl.a() { pc = 0x42074; continue; }
            pc = 0x4203b;
        }
        0x4203b => {
            // block 4203b
            // 4203b cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 4203f jne 0x42051
            if fl.ne() { pc = 0x42051; continue; }
            pc = 0x42041;
        }
        0x42041 => {
            // block 42041
            // 42041 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 42044 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 42046 mov rdi,r14
            rdi = r14;
            // 42049 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 4204b call QWORD PTR [rax+0x700]
            { let r = { let t = m.r64(rax.wrapping_add(0x700)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x42051;
        }
        0x42051 => {
            // block 42051
            // 42051 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 42054 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 42056 lea rdx,[rip+0xbe363] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 4205d mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42060 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 42065 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 4206b mov rdi,r14
            rdi = r14;
            // 4206e mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 42071 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x42074;
        }
        0x42074 => {
            // block 42074
            // 42074 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42078 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42079 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4207b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4207d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4207e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb60d4 size=102
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b60d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb60d4;
    loop {
        match pc {
        0xb60d4 => {
            // block b60d4
            // b60d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b60d5 mov rbp,rsp
            rbp = rsp;
            // b60d8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b60da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b60dc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b60de push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b60df mov r15,rdx
            r15 = rdx;
            // b60e2 mov r12,rsi
            r12 = rsi;
            // b60e5 mov rbx,rdi
            rbx = rdi;
            // b60e8 add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // b60ec mov edi,0x38
            rdi = (0x38u64) as u32 as u64;
            // b60f1 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b60f6 mov r14,rax
            r14 = rax;
            // b60f9 mov QWORD PTR [rbx],r14
            m.w64(rbx, (r14) as u64);
            // b60fc mov QWORD PTR [rbx+0x8],r12
            m.w64(rbx.wrapping_add(0x8), (r12) as u64);
            // b6100 mov BYTE PTR [rbx+0x10],0x0
            m.w8(rbx.wrapping_add(0x10), (0x0u64) as u8);
            // b6104 lea rdi,[r14+0x20]
            rdi = r14.wrapping_add(0x20);
            // b6108 mov rsi,r15
            rsi = r15;
            // b610b call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6110 mov BYTE PTR [rbx+0x10],0x1
            m.w8(rbx.wrapping_add(0x10), (0x1u64) as u8);
            // b6114 mov rax,rbx
            rax = rbx;
            // b6117 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b6118 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b611a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b611c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b611e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b611f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb6120 => {
            // block b6120
            // b6120 mov r15,rax
            r15 = rax;
            // b6123 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // b612a mov rdi,r14
            rdi = r14;
            // b612d call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6132 mov rdi,r15
            rdi = r15;
            // b6135 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14d94 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014d94<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x14d94;
    loop {
        match pc {
        0x14d94 => {
            // block 14d94
            // 14d94 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14d95 mov rbp,rsp
            rbp = rsp;
            // 14d98 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 14d9a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14d9b mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 14d9e mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 14da2 lea rbx,[rdi+rax*1]
            rbx = rdi.wrapping_add(rax);
            // 14da6 mov rcx,QWORD PTR [rip+0x12b5f3] # 0x1403a0
            rcx = m.r64(m.env.data_va(346989)) as u64;
            // 14dad lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // 14db1 mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // 14db5 lea r14,[rdi+rax*1+0x1a8]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x1a8);
            // 14dbd add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // 14dc1 mov QWORD PTR [rdi+rax*1+0x1a8],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x1a8), (rcx) as u64);
            // 14dc9 lea rdi,[rdi+rax*1+0x10]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x10);
            // 14dce call 0xeb912
            { let r = { let t = m.env.stub_va(0xdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14dd3 mov rsi,QWORD PTR [rip+0x12b416] # 0x1401f0
            rsi = m.r64(m.env.data_va(346557)) as u64;
            // 14dda add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 14dde mov rdi,rbx
            rdi = rbx;
            // 14de1 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14de6 mov rdi,r14
            rdi = r14;
            // 14de9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14dea pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 14dec pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14ded jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc533c size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c533c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc533c;
    loop {
        match pc {
        0xc533c => {
            // block c533c
            // c533c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c533d mov rbp,rsp
            rbp = rsp;
            // c5340 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c5342 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c5344 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c5346 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c5347 mov r12,rdx
            r12 = rdx;
            // c534a mov r14,rsi
            r14 = rsi;
            // c534d mov rbx,rdi
            rbx = rdi;
            // c5350 mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c5355 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c535a mov r15,rax
            r15 = rax;
            // c535d lea rdi,[r15+0x20]
            rdi = r15.wrapping_add(0x20);
            // c5361 mov rsi,r12
            rsi = r12;
            // c5364 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5369 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c536d mov DWORD PTR [r15+0x38],0x0
            m.w32(r15.wrapping_add(0x38), (0x0u64) as u32);
            // c5375 mov QWORD PTR [rbx],r15
            m.w64(rbx, (r15) as u64);
            // c5378 mov QWORD PTR [rbx+0x8],r14
            m.w64(rbx.wrapping_add(0x8), (r14) as u64);
            // c537c mov QWORD PTR [rbx+0x10],0x101
            m.w64(rbx.wrapping_add(0x10), (0x101u64) as u64);
            // c5384 mov rax,rbx
            rax = rbx;
            // c5387 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5388 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c538a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c538c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c538e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c538f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc5390 => {
            // block c5390
            // c5390 mov rbx,rax
            rbx = rax;
            // c5393 mov rdi,r15
            rdi = r15;
            // c5396 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c539b mov rdi,rbx
            rdi = rbx;
            // c539e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe99dc size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e99dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe99dc;
    loop {
        match pc {
        0xe99dc => {
            // block e99dc
            // e99dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e99dd mov rbp,rsp
            rbp = rsp;
            // e99e0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e99e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e99e3 mov rbx,rdi
            rbx = rdi;
            // e99e6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e99e9 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e99ed mov rdx,rdi
            rdx = rdi;
            // e99f0 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // e99f3 sar rdx,0x3
            { let (r, f) = sar::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // e99f7 movabs rcx,0xaaaaaaaaaaaaaaab
            rcx = 0xaaaa_aaaa_aaaa_aaabu64;
            // e9a01 imul rcx,rdx
            { let (r, f) = imul::<64>(rcx, rdx); rcx = r; fl = f; }
            // e9a05 cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // e9a08 jae 0xe9a19
            if fl.ae() { pc = 0xe9a19; continue; }
            pc = 0xe9a0a;
        }
        0xe9a0a => {
            // block e9a0a
            // e9a0a sub rsi,rcx
            { let (r, f) = sub::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // e9a0d mov rdi,rbx
            rdi = rbx;
            // e9a10 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9a11 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9a13 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9a14 jmp 0xebf1e
            { let r = { let t = m.env.stub_va(0x6eb); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xe9a19 => {
            // block e9a19
            // e9a19 jbe 0xe9a3b
            if fl.be() { pc = 0xe9a3b; continue; }
            pc = 0xe9a1b;
        }
        0xe9a1b => {
            // block e9a1b
            // e9a1b lea rcx,[rsi+rsi*2]
            rcx = rsi.wrapping_add(rsi.wrapping_mul(2));
            // e9a1f lea r14,[rax+rcx*8]
            r14 = rax.wrapping_add(rcx.wrapping_mul(8));
            // e9a23 jmp 0xe9a36
            pc = 0xe9a36; continue;
        }
        0xe9a25 => {
            // block e9a25
            // e9a25 add rdi,0xffffffffffffffe8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffe8u64);
              rdi = r;
              fl = f; }
            // e9a29 mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // e9a2d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9a32 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0xe9a36;
        }
        0xe9a36 => {
            // block e9a36
            // e9a36 cmp rdi,r14
            fl = sub::<64>(rdi, r14).1;
            // e9a39 jne 0xe9a25
            if fl.ne() { pc = 0xe9a25; continue; }
            pc = 0xe9a3b;
        }
        0xe9a3b => {
            // block e9a3b
            // e9a3b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9a3c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9a3e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9a3f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14ce4 size=106
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014ce4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x14ce4;
    loop {
        match pc {
        0x14ce4 => {
            // block 14ce4
            // 14ce4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14ce5 mov rbp,rsp
            rbp = rsp;
            // 14ce8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14ce9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 14cea mov rax,QWORD PTR [rip+0x12b52f] # 0x140220
            rax = m.r64(m.env.data_va(346605)) as u64;
            // 14cf1 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 14cf5 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 14cf8 lea rbx,[rip+0x1acd09] # 0x1c1a08
            rbx = m.env.data_va(877013);
            // 14cff mov rsi,QWORD PTR [rip+0x1acd02] # 0x1c1a08
            rsi = m.r64(m.env.data_va(877013)) as u64;
            // 14d06 lea rdi,[rip+0x1accf3] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 14d0d call 0xeb942
            { let r = { let t = m.env.stub_va(0x10f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14d12 mov QWORD PTR [rip+0x1accf3],0x0 # 0x1c1a10
            m.w64(m.env.data_va(877021), (0x0u64) as u64);
            // 14d1d mov QWORD PTR [rip+0x1accdc],rbx # 0x1c1a00
            m.w64(m.env.data_va(877005), (rbx) as u64);
            // 14d24 mov QWORD PTR [rip+0x1accd9],0x0 # 0x1c1a08
            m.w64(m.env.data_va(877013), (0x0u64) as u64);
            // 14d2f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 14d33 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14d34 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14d35 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4b54 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004b54<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4b54;
    loop {
        match pc {
        0x4b54 => {
            // block 4b54
            // 4b54 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4b55 mov rbp,rsp
            rbp = rsp;
            // 4b58 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4b5a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4b5c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4b5d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4b5e mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 4b61 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 4b64 mov rbx,rdi
            rbx = rdi;
            // 4b67 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b6a mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 4b6d call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b73 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4b75 je 0x4b7b
            if fl.e() { pc = 0x4b7b; continue; }
            pc = 0x4b77;
        }
        0x4b77 => {
            // block 4b77
            // 4b77 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4b79 jmp 0x4bb4
            pc = 0x4bb4; continue;
        }
        0x4b7b => {
            // block 4b7b
            // 4b7b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b7e mov rdi,rbx
            rdi = rbx;
            // 4b81 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b87 mov r9,QWORD PTR [rbx]
            r9 = m.r64(rbx) as u64;
            // 4b8a mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 4b8d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4b8f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4b91 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // 4b95 lea rax,[rip+0xfa454] # 0xfeff0
            rax = m.env.data_va(79805);
            // 4b9c mov esi,DWORD PTR [rax+rdx*4]
            rsi = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4b9f mov ecx,0x300000
            rcx = (0x300000u64) as u32 as u64;
            // 4ba4 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 4baa mov rdi,rbx
            rdi = rbx;
            // 4bad mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 4bb0 call QWORD PTR [r9+0x30]
            { let r = { let t = m.r64(r9.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4bb4;
        }
        0x4bb4 => {
            // block 4bb4
            // 4bb4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4bb8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4bb9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4bbb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4bbd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4bbe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9548 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9548<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb9548;
    loop {
        match pc {
        0xb9548 => {
            // block b9548
            // b9548 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9549 mov rbp,rsp
            rbp = rsp;
            // b954c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b954e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b9550 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9551 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b9552 mov r15,rdi
            r15 = rdi;
            // b9555 lea rax,[r15+0x8]
            rax = r15.wrapping_add(0x8);
            // b9559 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // b9561 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // b9569 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // b956c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b956e lea r14,[rip+0x4946b] # 0x1029e0
            r14 = m.env.data_va(94637);
            pc = 0xb9575;
        }
        0xb9575 => {
            // block b9575
            // b9575 lea rsi,[r14+rbx*1]
            rsi = r14.wrapping_add(rbx);
            // b9579 mov rdi,r15
            rdi = r15;
            // b957c call 0xebce4
            { let r = { let t = m.env.stub_va(0x4b1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9581 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // b9584 cmp rbx,0x10
            fl = sub::<64>(rbx, 0x10u64).1;
            // b9588 jne 0xb9575
            if fl.ne() { pc = 0xb9575; continue; }
            pc = 0xb958a;
        }
        0xb958a => {
            // block b958a
            // b958a mov rax,r15
            rax = r15;
            // b958d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b9591 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9592 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9594 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9596 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9597 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb9598 => {
            // block b9598
            // b9598 mov r14,rax
            r14 = rax;
            // b959b mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // b959f mov rdi,r15
            rdi = r15;
            // b95a2 call 0xebcf0
            { let r = { let t = m.env.stub_va(0x4bd); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b95a7 mov rdi,r14
            rdi = r14;
            // b95aa call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9ab8 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9ab8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9ab8;
    loop {
        match pc {
        0xe9ab8 => {
            // block e9ab8
            // e9ab8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9ab9 mov rbp,rsp
            rbp = rsp;
            // e9abc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9abe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9ac0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9ac1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e9ac2 mov rbx,rdi
            rbx = rdi;
            // e9ac5 mov r14,QWORD PTR [rbx+0x78]
            r14 = m.r64(rbx.wrapping_add(0x78)) as u64;
            // e9ac9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9acb test r14,r14
            fl = logic::<64>(r14 & r14);
            // e9ace je 0xe9b00
            if fl.e() { pc = 0xe9b00; continue; }
            pc = 0xe9ad0;
        }
        0xe9ad0 => {
            // block e9ad0
            // e9ad0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e9ad3 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e9ad7 mov rdi,rbx
            rdi = rbx;
            // e9ada call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9adc mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // e9adf mov rdi,r14
            rdi = r14;
            // e9ae2 call 0xec1b8
            { let r = { let t = m.env.stub_va(0x985); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9ae7 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // e9ae9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9aeb test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // e9aed jne 0xe9b00
            if fl.ne() { pc = 0xe9b00; continue; }
            pc = 0xe9aef;
        }
        0xe9aef => {
            // block e9aef
            // e9aef xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9af1 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // e9af4 cmove rax,rbx
            if fl.e() { rax = rbx; }
            // e9af8 mov QWORD PTR [rbx+0x78],0x0
            m.w64(rbx.wrapping_add(0x78), (0x0u64) as u64);
            pc = 0xe9b00;
        }
        0xe9b00 => {
            // block e9b00
            // e9b00 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e9b04 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9b05 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9b07 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e9b09 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9b0a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe9b0b => {
            // block e9b0b
            // e9b0b mov rbx,rax
            rbx = rax;
            // e9b0e mov rdi,r14
            rdi = r14;
            // e9b11 call 0xec1b8
            { let r = { let t = m.env.stub_va(0x985); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9b16 mov rdi,rbx
            rdi = rbx;
            // e9b19 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a270 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a270<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a270;
    loop {
        match pc {
        0x1a270 => {
            // block 1a270
            // 1a270 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a271 mov rbp,rsp
            rbp = rsp;
            // 1a274 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a276 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a277 mov r14,rsi
            r14 = rsi;
            // 1a27a mov rbx,rdi
            rbx = rdi;
            // 1a27d mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1a285 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1a28d mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 1a294 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a298 sub rsi,QWORD PTR [r14]
            { let (r, f) = sub::<64>(rsi, m.r64(r14) as u64);
              rsi = r;
              fl = f; }
            // 1a29b sar rsi,0x2
            { let (r, f) = sar::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 1a29f je 0x1a2c7
            if fl.e() { pc = 0x1a2c7; continue; }
            pc = 0x1a2a1;
        }
        0x1a2a1 => {
            // block 1a2a1
            // 1a2a1 mov rdi,rbx
            rdi = rbx;
            // 1a2a4 call 0xeba20
            { let r = { let t = m.env.stub_va(0x1ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a2a9 mov rsi,QWORD PTR [r14]
            rsi = m.r64(r14) as u64;
            // 1a2ac mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a2b0 sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // 1a2b3 mov r14,rdx
            r14 = rdx;
            // 1a2b6 and r14,0xfffffffffffffffc
            { let r = r14 & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              r14 = r;
              fl = f; }
            // 1a2ba mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1a2be call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a2c3 add QWORD PTR [rbx+0x8],r14
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, r14);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            pc = 0x1a2c7;
        }
        0x1a2c7 => {
            // block 1a2c7
            // 1a2c7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a2c8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a2ca pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a2cb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1a2cc => {
            // block 1a2cc
            // 1a2cc mov r14,rax
            r14 = rax;
            // 1a2cf mov rdi,rbx
            rdi = rbx;
            // 1a2d2 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a2d7 mov rdi,r14
            rdi = r14;
            // 1a2da call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a350 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a350<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a350;
    loop {
        match pc {
        0x1a350 => {
            // block 1a350
            // 1a350 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a351 mov rbp,rsp
            rbp = rsp;
            // 1a354 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a356 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a357 mov r14,rsi
            r14 = rsi;
            // 1a35a mov rbx,rdi
            rbx = rdi;
            // 1a35d mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1a365 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1a36d mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 1a374 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a378 sub rsi,QWORD PTR [r14]
            { let (r, f) = sub::<64>(rsi, m.r64(r14) as u64);
              rsi = r;
              fl = f; }
            // 1a37b sar rsi,0x2
            { let (r, f) = sar::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 1a37f je 0x1a3a7
            if fl.e() { pc = 0x1a3a7; continue; }
            pc = 0x1a381;
        }
        0x1a381 => {
            // block 1a381
            // 1a381 mov rdi,rbx
            rdi = rbx;
            // 1a384 call 0xeba44
            { let r = { let t = m.env.stub_va(0x211); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a389 mov rsi,QWORD PTR [r14]
            rsi = m.r64(r14) as u64;
            // 1a38c mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a390 sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // 1a393 mov r14,rdx
            r14 = rdx;
            // 1a396 and r14,0xfffffffffffffffc
            { let r = r14 & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              r14 = r;
              fl = f; }
            // 1a39a mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1a39e call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a3a3 add QWORD PTR [rbx+0x8],r14
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, r14);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            pc = 0x1a3a7;
        }
        0x1a3a7 => {
            // block 1a3a7
            // 1a3a7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a3a8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a3aa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a3ab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1a3ac => {
            // block 1a3ac
            // 1a3ac mov r14,rax
            r14 = rax;
            // 1a3af mov rdi,rbx
            rdi = rbx;
            // 1a3b2 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a3b7 mov rdi,r14
            rdi = r14;
            // 1a3ba call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e528 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e528<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e528;
    loop {
        match pc {
        0x2e528 => {
            // block 2e528
            // 2e528 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e529 mov rbp,rsp
            rbp = rsp;
            // 2e52c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e52e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e530 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e532 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e533 mov r14,rdx
            r14 = rdx;
            // 2e536 mov r12,rsi
            r12 = rsi;
            // 2e539 mov r15,rdi
            r15 = rdi;
            // 2e53c test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2e53f je 0x2e58c
            if fl.e() { pc = 0x2e58c; continue; }
            pc = 0x2e541;
        }
        0x2e541 => {
            // block 2e541
            // 2e541 mov rbx,QWORD PTR [r15+0x10]
            rbx = m.r64(r15.wrapping_add(0x10)) as u64;
            // 2e545 jmp 0x2e54b
            pc = 0x2e54b; continue;
        }
        0x2e547 => {
            // block 2e547
            // 2e547 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            pc = 0x2e54b;
        }
        0x2e54b => {
            // block 2e54b
            // 2e54b cmp rbx,QWORD PTR [r15+0x18]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x18)) as u64).1;
            // 2e54f je 0x2e58c
            if fl.e() { pc = 0x2e58c; continue; }
            pc = 0x2e551;
        }
        0x2e551 => {
            // block 2e551
            // 2e551 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 2e554 je 0x2e575
            if fl.e() { pc = 0x2e575; continue; }
            pc = 0x2e556;
        }
        0x2e556 => {
            // block 2e556
            // 2e556 cmp BYTE PTR [r12],0x0
            fl = sub::<8>(m.r8(r12) as u64, 0x0u64).1;
            // 2e55b je 0x2e575
            if fl.e() { pc = 0x2e575; continue; }
            pc = 0x2e55d;
        }
        0x2e55d => {
            // block 2e55d
            // 2e55d mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 2e560 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2e563 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e566 mov rdi,r12
            rdi = r12;
            // 2e569 mov rsi,rax
            rsi = rax;
            // 2e56c call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e571 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e573 jne 0x2e547
            if fl.ne() { pc = 0x2e547; continue; }
            pc = 0x2e575;
        }
        0x2e575 => {
            // block 2e575
            // 2e575 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 2e578 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2e57b mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 2e57f mov rsi,r14
            rsi = r14;
            // 2e582 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e583 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e585 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e587 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e589 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e58a jmp rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2e58c => {
            // block 2e58c
            // 2e58c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e58e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e58f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e591 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e593 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e595 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e596 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42ef4 size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042ef4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42ef4;
    loop {
        match pc {
        0x42ef4 => {
            // block 42ef4
            // 42ef4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42ef5 mov rbp,rsp
            rbp = rsp;
            // 42ef8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42efa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42efc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42efd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42efe mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 42f01 mov rbx,rdi
            rbx = rdi;
            // 42f04 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42f07 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42f0d xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42f10 cmp r14d,0x1ffff
            fl = sub::<32>((r14 as u32 as u64), 0x1ffffu64).1;
            // 42f17 ja 0x42f59
            if fl.a() { pc = 0x42f59; continue; }
            pc = 0x42f19;
        }
        0x42f19 => {
            // block 42f19
            // 42f19 xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 42f1b jne 0x42f59
            if fl.ne() { pc = 0x42f59; continue; }
            pc = 0x42f1d;
        }
        0x42f1d => {
            // block 42f1d
            // 42f1d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42f20 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42f23 mov esi,0x903
            rsi = (0x903u64) as u32 as u64;
            // 42f28 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42f2d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42f30 mov rdi,rbx
            rdi = rbx;
            // 42f33 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42f36 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42f39 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42f3b je 0x42f59
            if fl.e() { pc = 0x42f59; continue; }
            pc = 0x42f3d;
        }
        0x42f3d => {
            // block 42f3d
            // 42f3d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42f40 mov esi,0x907
            rsi = (0x907u64) as u32 as u64;
            // 42f45 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42f4a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42f4d mov rdi,rbx
            rdi = rbx;
            // 42f50 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42f53 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42f56 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x42f59;
        }
        0x42f59 => {
            // block 42f59
            // 42f59 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 42f5c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42f60 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42f61 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42f63 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42f65 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42f66 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa28c0 size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a28c0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xa28c0;
    loop {
        match pc {
        0xa28c0 => {
            // block a28c0
            // a28c0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a28c1 mov rbp,rsp
            rbp = rsp;
            // a28c4 mov rax,QWORD PTR [rdi+0x30]
            rax = m.r64(rdi.wrapping_add(0x30)) as u64;
            // a28c8 mov r10,QWORD PTR [rdi+0x58]
            r10 = m.r64(rdi.wrapping_add(0x58)) as u64;
            // a28cc cmp r10,rax
            fl = sub::<64>(r10, rax).1;
            // a28cf jae 0xa28d8
            if fl.ae() { pc = 0xa28d8; continue; }
            pc = 0xa28d1;
        }
        0xa28d1 => {
            // block a28d1
            // a28d1 mov QWORD PTR [rdi+0x58],rax
            m.w64(rdi.wrapping_add(0x58), (rax) as u64);
            // a28d5 mov r10,rax
            r10 = rax;
            pc = 0xa28d8;
        }
        0xa28d8 => {
            // block a28d8
            // a28d8 mov r8,QWORD PTR [rdi+0x10]
            r8 = m.r64(rdi.wrapping_add(0x10)) as u64;
            // a28dc mov rdx,QWORD PTR [rdi+0x18]
            rdx = m.r64(rdi.wrapping_add(0x18)) as u64;
            // a28e0 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // a28e5 cmp r8,rdx
            fl = sub::<64>(r8, rdx).1;
            // a28e8 jae 0xa292a
            if fl.ae() { pc = 0xa292a; continue; }
            pc = 0xa28ea;
        }
        0xa28ea => {
            // block a28ea
            // a28ea cmp esi,0xffffffff
            fl = sub::<32>((rsi as u32 as u64), 0xffffffffu64).1;
            // a28ed je 0xa2919
            if fl.e() { pc = 0xa2919; continue; }
            pc = 0xa28ef;
        }
        0xa28ef => {
            // block a28ef
            // a28ef test BYTE PTR [rdi+0x60],0x10
            fl = logic::<8>(m.r8(rdi.wrapping_add(0x60)) as u64 & 0x10u64);
            // a28f3 jne 0xa2903
            if fl.ne() { pc = 0xa2903; continue; }
            pc = 0xa28f5;
        }
        0xa28f5 => {
            // block a28f5
            // a28f5 movzx r9d,BYTE PTR [rdx-0x1]
            r9 = (m.r8(rdx.wrapping_add(0xffff_ffff_ffff_ffff)) as u64) as u32 as u64;
            // a28fa movzx ecx,sil
            rcx = ((rsi as u8 as u64)) as u32 as u64;
            // a28fe cmp ecx,r9d
            fl = sub::<32>((rcx as u32 as u64), (r9 as u32 as u64)).1;
            // a2901 jne 0xa292a
            if fl.ne() { pc = 0xa292a; continue; }
            pc = 0xa2903;
        }
        0xa2903 => {
            // block a2903
            // a2903 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // a2906 mov QWORD PTR [rdi+0x10],r8
            m.w64(rdi.wrapping_add(0x10), (r8) as u64);
            // a290a mov QWORD PTR [rdi+0x18],rdx
            m.w64(rdi.wrapping_add(0x18), (rdx) as u64);
            // a290e mov QWORD PTR [rdi+0x20],r10
            m.w64(rdi.wrapping_add(0x20), (r10) as u64);
            // a2912 mov BYTE PTR [rdx],sil
            m.w8(rdx, ((rsi as u8 as u64)) as u8);
            // a2915 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // a2917 jmp 0xa292a
            pc = 0xa292a; continue;
        }
        0xa2919 => {
            // block a2919
            // a2919 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // a291c mov QWORD PTR [rdi+0x10],r8
            m.w64(rdi.wrapping_add(0x10), (r8) as u64);
            // a2920 mov QWORD PTR [rdi+0x18],rdx
            m.w64(rdi.wrapping_add(0x18), (rdx) as u64);
            // a2924 mov QWORD PTR [rdi+0x20],r10
            m.w64(rdi.wrapping_add(0x20), (r10) as u64);
            // a2928 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xa292a;
        }
        0xa292a => {
            // block a292a
            // a292a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a292b ret
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
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // c5795 lea rsi,[r14+0x8]
            rsi = r14.wrapping_add(0x8);
            // c5799 jmp 0xc57bc
            pc = 0xc57bc; continue;
        }
        0xc579b => {
            // block c579b
            // c579b mov rsi,rcx
            rsi = rcx;
            pc = 0xc579e;
        }
        0xc579e => {
            // block c579e
            // c579e cmp eax,DWORD PTR [rsi+0x1c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rsi.wrapping_add(0x1c)) as u64).1;
            // c57a1 jge 0xc57ad
            if fl.ge() { pc = 0xc57ad; continue; }
            pc = 0xc57a3;
        }
        0xc57a3 => {
            // block c57a3
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
            // c57ab jmp 0xc57bc
            pc = 0xc57bc; continue;
        }
        0xc57ad => {
            // block c57ad
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
            // c57b6 lea rdx,[rsi+0x8]
            rdx = rsi.wrapping_add(0x8);
            // c57ba jmp 0xc57bf
            pc = 0xc57bf; continue;
        }
        0xc57bc => {
            // block c57bc
            // c57bc mov rdx,rsi
            rdx = rsi;
            pc = 0xc57bf;
        }
        0xc57bf => {
            // block c57bf
            // c57bf mov rdi,r14
            rdi = r14;
            // c57c2 mov rcx,r15
            rcx = r15;
            // c57c5 call 0xebdc2
            { let r = { let t = m.env.stub_va(0x58f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x5d32b size=119
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005d32b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5d32b;
    loop {
        match pc {
        0x5d32b => {
            // block 5d32b
            // 5d32b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5d32c mov rbp,rsp
            rbp = rsp;
            // 5d32f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5d330 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5d331 mov rbx,rdi
            rbx = rdi;
            // 5d334 cmp esi,0x2
            fl = sub::<32>((rsi as u32 as u64), 0x2u64).1;
            // 5d337 je 0x5d363
            if fl.e() { pc = 0x5d363; continue; }
            pc = 0x5d339;
        }
        0x5d339 => {
            // block 5d339
            // 5d339 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 5d33c je 0x5d342
            if fl.e() { pc = 0x5d342; continue; }
            pc = 0x5d33e;
        }
        0x5d33e => {
            // block 5d33e
            // 5d33e test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 5d340 jne 0x5d387
            if fl.ne() { pc = 0x5d387; continue; }
            pc = 0x5d342;
        }
        0x5d342 => {
            // block 5d342
            // 5d342 lea rax,[rip+0xce987] # 0x12bcd0
            rax = m.env.data_va(263325);
            // 5d349 lea rsi,[rip+0xce986] # 0x12bcd6
            rsi = m.env.data_va(263331);
            // 5d350 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5d352 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 5d356 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5d359 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 5d35c lea rdx,[rax+rax*8+0x5]
            rdx = rax.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x5);
            // 5d361 jmp 0x5d390
            pc = 0x5d390; continue;
        }
        0x5d363 => {
            // block 5d363
            // 5d363 lea rax,[rip+0xce87e] # 0x12bbe8
            rax = m.env.data_va(263093);
            // 5d36a lea rsi,[rip+0xce974] # 0x12bce5
            rsi = m.env.data_va(263346);
            // 5d371 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5d373 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 5d377 mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 5d37c mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 5d381 cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            // 5d385 jmp 0x5d390
            pc = 0x5d390; continue;
        }
        0x5d387 => {
            // block 5d387
            // 5d387 lea rsi,[rip+0xdb93a] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 5d38e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x5d390;
        }
        0x5d390 => {
            // block 5d390
            // 5d390 mov rdi,rbx
            rdi = rbx;
            // 5d393 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d398 mov rax,rbx
            rax = rbx;
            // 5d39b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5d39f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5d3a0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5d3a1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd9f72 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d9f72<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd9f72;
    loop {
        match pc {
        0xd9f72 => {
            // block d9f72
            // d9f72 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d9f73 mov rbp,rsp
            rbp = rsp;
            // d9f76 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d9f78 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d9f7a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d9f7b sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d9f7f mov r14,rsi
            r14 = rsi;
            // d9f82 mov rbx,rdi
            rbx = rdi;
            // d9f85 lea rdi,[rip+0x5f0b4] # 0x139040
            rdi = m.env.data_va(317453);
            // d9f8c call 0xec272
            { let r = { let t = m.env.stub_va(0xa3f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9f91 add rbx,0x380
            { let (r, f) = add::<64>(rbx, 0x380u64);
              rbx = r;
              fl = f; }
            // d9f98 mov rdi,r14
            rdi = r14;
            // d9f9b call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fa0 lea r15,[rbp-0x30]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d9fa4 mov rdi,r15
            rdi = r15;
            // d9fa7 mov rsi,r14
            rsi = r14;
            // d9faa mov rdx,rax
            rdx = rax;
            // d9fad call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fb2 mov rdi,rbx
            rdi = rbx;
            // d9fb5 mov rsi,r15
            rsi = r15;
            // d9fb8 call 0xe8100
            { let r = { let t = m.env.code_va(0xe8100); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fbd mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d9fbf lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d9fc3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fc8 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d9fca add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d9fce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d9fcf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d9fd1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d9fd3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d9fd4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd9fd5 => {
            // block d9fd5
            // d9fd5 mov rbx,rax
            rbx = rax;
            // d9fd8 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d9fdc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9fe1 mov rdi,rbx
            rdi = rbx;
            // d9fe4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a168 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a168<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a168;
    loop {
        match pc {
        0x1a168 => {
            // block 1a168
            // 1a168 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a169 mov rbp,rsp
            rbp = rsp;
            // 1a16c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1a16e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a170 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a171 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1a172 mov rbx,rdi
            rbx = rdi;
            // 1a175 mov rax,QWORD PTR [rip+0x12623c] # 0x1403b8
            rax = m.r64(m.env.data_va(347013)) as u64;
            // 1a17c lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 1a180 mov QWORD PTR [rbx],rcx
            m.w64(rbx, (rcx) as u64);
            // 1a183 lea r14,[rbx+0x78]
            r14 = rbx.wrapping_add(0x78);
            // 1a187 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 1a18b mov QWORD PTR [rbx+0x78],rax
            m.w64(rbx.wrapping_add(0x78), (rax) as u64);
            // 1a18f lea r15,[rbx+0x10]
            r15 = rbx.wrapping_add(0x10);
            // 1a193 mov rax,QWORD PTR [rip+0x12620e] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 1a19a add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 1a19e mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 1a1a2 lea rdi,[rbx+0x50]
            rdi = rbx.wrapping_add(0x50);
            // 1a1a6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a1ab mov rdi,r15
            rdi = r15;
            // 1a1ae call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a1b3 mov rsi,QWORD PTR [rip+0x126046] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // 1a1ba add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 1a1be mov rdi,rbx
            rdi = rbx;
            // 1a1c1 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a1c6 mov rdi,r14
            rdi = r14;
            // 1a1c9 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a1ce mov rdi,rbx
            rdi = rbx;
            // 1a1d1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1a1d5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a1d6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a1d8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1a1da pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a1db jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
