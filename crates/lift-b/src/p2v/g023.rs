//! Mechanically lifted functions (scripts/lift_run.py), module g023. Each one is checked against the original
//! in the world harness (tests/world_g023.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x64c02 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064c02<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x64c02;
    loop {
        match pc {
        0x64c02 => {
            // block 64c02
            // 64c02 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64c03 mov rbp,rsp
            rbp = rsp;
            // 64c06 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 64c08 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 64c0a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 64c0c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 64c0d sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 64c11 mov r14,rsi
            r14 = rsi;
            // 64c14 mov r12,rdi
            r12 = rdi;
            // 64c17 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 64c1b mov rdx,r14
            rdx = r14;
            // 64c1e call 0xebb8e
            { let r = { let t = m.env.stub_va(0x35b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64c23 mov r15,rax
            r15 = rax;
            // 64c26 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 64c29 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 64c2c jne 0x64c5b
            if fl.ne() { pc = 0x64c5b; continue; }
            pc = 0x64c2e;
        }
        0x64c2e => {
            // block 64c2e
            // 64c2e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 64c32 mov rsi,r12
            rsi = r12;
            // 64c35 mov rdx,r14
            rdx = r14;
            // 64c38 call 0xebb94
            { let r = { let t = m.env.stub_va(0x361); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64c3d mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 64c41 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 64c45 mov rdi,r12
            rdi = r12;
            // 64c48 mov rdx,r15
            rdx = r15;
            // 64c4b mov rcx,rbx
            rcx = rbx;
            // 64c4e call 0xeb930
            { let r = { let t = m.env.stub_va(0xfd); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64c53 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            pc = 0x64c5b;
        }
        0x64c5b => {
            // block 64c5b
            // 64c5b add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 64c5f mov rax,rbx
            rax = rbx;
            // 64c62 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 64c66 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64c67 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 64c69 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 64c6b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 64c6d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64c6e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3ac8 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3ac8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3ac8;
    loop {
        match pc {
        0xc3ac8 => {
            // block c3ac8
            // c3ac8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3ac9 mov rbp,rsp
            rbp = rsp;
            // c3acc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3ace push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3ad0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3ad2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3ad3 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // c3ad7 mov r14,rsi
            r14 = rsi;
            // c3ada mov r12,rdi
            r12 = rdi;
            // c3add lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3ae1 mov rdx,r14
            rdx = r14;
            // c3ae4 call 0xebd3e
            { let r = { let t = m.env.stub_va(0x50b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3ae9 mov r15,rax
            r15 = rax;
            // c3aec mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3aef test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3af2 jne 0xc3b21
            if fl.ne() { pc = 0xc3b21; continue; }
            pc = 0xc3af4;
        }
        0xc3af4 => {
            // block c3af4
            // c3af4 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // c3af8 mov rsi,r12
            rsi = r12;
            // c3afb mov rdx,r14
            rdx = r14;
            // c3afe call 0xebd44
            { let r = { let t = m.env.stub_va(0x511); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3b03 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3b07 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // c3b0b mov rdi,r12
            rdi = r12;
            // c3b0e mov rdx,r15
            rdx = r15;
            // c3b11 mov rcx,rbx
            rcx = rbx;
            // c3b14 call 0xebe64
            { let r = { let t = m.env.stub_va(0x631); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3b19 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            pc = 0xc3b21;
        }
        0xc3b21 => {
            // block c3b21
            // c3b21 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // c3b25 mov rax,rbx
            rax = rbx;
            // c3b28 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // c3b2c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3b2d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3b2f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3b31 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3b33 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3b34 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3d56 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3d56<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3d56;
    loop {
        match pc {
        0xc3d56 => {
            // block c3d56
            // c3d56 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3d57 mov rbp,rsp
            rbp = rsp;
            // c3d5a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3d5c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3d5e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3d60 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3d61 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3d65 mov r15,rsi
            r15 = rsi;
            // c3d68 mov r14,rdi
            r14 = rdi;
            // c3d6b lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3d6f mov rdx,r15
            rdx = r15;
            // c3d72 call 0xebd0e
            { let r = { let t = m.env.stub_va(0x4db); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3d77 mov r12,rax
            r12 = rax;
            // c3d7a mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // c3d7e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3d81 jne 0xc3daf
            if fl.ne() { pc = 0xc3daf; continue; }
            pc = 0xc3d83;
        }
        0xc3d83 => {
            // block c3d83
            // c3d83 mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // c3d88 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3d8d mov rbx,rax
            rbx = rax;
            // c3d90 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // c3d93 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c3d96 mov DWORD PTR [rbx+0x20],0x0
            m.w32(rbx.wrapping_add(0x20), (0x0u64) as u32);
            // c3d9d mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3da1 mov rdi,r14
            rdi = r14;
            // c3da4 mov rdx,r12
            rdx = r12;
            // c3da7 mov rcx,rbx
            rcx = rbx;
            // c3daa call 0xebdec
            { let r = { let t = m.env.stub_va(0x5b9); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3daf;
        }
        0xc3daf => {
            // block c3daf
            // c3daf add rbx,0x20
            { let (r, f) = add::<64>(rbx, 0x20u64);
              rbx = r;
              fl = f; }
            // c3db3 mov rax,rbx
            rax = rbx;
            // c3db6 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3dba pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3dbb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3dbd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3dbf pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3dc1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3dc2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe30c4 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e30c4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe30c4;
    loop {
        match pc {
        0xe30c4 => {
            // block e30c4
            // e30c4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e30c5 mov rbp,rsp
            rbp = rsp;
            // e30c8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e30ca push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e30cc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e30cd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e30ce mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // e30d1 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e30d3 mov r15,rdi
            r15 = rdi;
            // e30d6 shl ebx,0x3
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x3u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e30d9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e30dc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e30df lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e30e3 mov esi,0x4001d
            rsi = (0x4001du64) as u32 as u64;
            // e30e8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e30ed xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e30f0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e30f3 mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // e30f8 mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e30fa shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e30fc not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // e30fe and eax,DWORD PTR [rbp-0x1c]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e3101 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // e3105 mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e3107 shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e3109 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e310b mov DWORD PTR [rbp-0x1c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rdx as u32 as u64)) as u32);
            // e310e mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e3111 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e3114 mov esi,0x4001d
            rsi = (0x4001du64) as u32 as u64;
            // e3119 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e311e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e3121 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3124 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e3126 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e312a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e312b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e312d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e312f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e3130 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8bdc size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008bdc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8bdc;
    loop {
        match pc {
        0x8bdc => {
            // block 8bdc
            // 8bdc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8bdd mov rbp,rsp
            rbp = rsp;
            // 8be0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8be2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8be4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8be5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 8be6 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 8be9 mov r14,rsi
            r14 = rsi;
            // 8bec mov rbx,rdi
            rbx = rdi;
            // 8bef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8bf2 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 8bf5 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8bfb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8bfd je 0x8c03
            if fl.e() { pc = 0x8c03; continue; }
            pc = 0x8bff;
        }
        0x8bff => {
            // block 8bff
            // 8bff xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8c01 jmp 0x8c41
            pc = 0x8c41; continue;
        }
        0x8c03 => {
            // block 8c03
            // 8c03 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 8c0a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8c0d mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 8c10 lea rcx,[rip+0xf6559] # 0xff170
            rcx = m.env.data_va(80189);
            // 8c17 mov esi,DWORD PTR [rcx+rdx*4]
            rsi = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8c1a lea rcx,[rip+0xf694f] # 0xff570
            rcx = m.env.data_va(81213);
            // 8c21 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8c24 lea rdi,[rip+0xf6975] # 0xff5a0
            rdi = m.env.data_va(81261);
            // 8c2b mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8c2f lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 8c33 mov rdi,rbx
            rdi = rbx;
            // 8c36 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8c39 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // 8c3d setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0x8c41;
        }
        0x8c41 => {
            // block 8c41
            // 8c41 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 8c45 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8c46 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8c48 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8c4a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8c4b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8cbc size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008cbc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8cbc;
    loop {
        match pc {
        0x8cbc => {
            // block 8cbc
            // 8cbc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8cbd mov rbp,rsp
            rbp = rsp;
            // 8cc0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8cc2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8cc4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8cc5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 8cc6 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 8cc9 mov r14,rsi
            r14 = rsi;
            // 8ccc mov rbx,rdi
            rbx = rdi;
            // 8ccf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8cd2 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 8cd5 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8cdb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8cdd je 0x8ce3
            if fl.e() { pc = 0x8ce3; continue; }
            pc = 0x8cdf;
        }
        0x8cdf => {
            // block 8cdf
            // 8cdf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8ce1 jmp 0x8d21
            pc = 0x8d21; continue;
        }
        0x8ce3 => {
            // block 8ce3
            // 8ce3 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 8cea mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8ced mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 8cf0 lea rcx,[rip+0xf6479] # 0xff170
            rcx = m.env.data_va(80189);
            // 8cf7 mov esi,DWORD PTR [rcx+rdx*4]
            rsi = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8cfa lea rcx,[rip+0xf692f] # 0xff630
            rcx = m.env.data_va(81405);
            // 8d01 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8d04 lea rdi,[rip+0xf6955] # 0xff660
            rdi = m.env.data_va(81453);
            // 8d0b mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8d0f lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 8d13 mov rdi,rbx
            rdi = rbx;
            // 8d16 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8d19 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // 8d1d setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0x8d21;
        }
        0x8d21 => {
            // block 8d21
            // 8d21 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 8d25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8d26 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8d28 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8d2a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8d2b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5ee2 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5ee2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5ee2;
    loop {
        match pc {
        0xb5ee2 => {
            // block b5ee2
            // b5ee2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5ee3 mov rbp,rsp
            rbp = rsp;
            // b5ee6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5ee8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5eea push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5eec push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5eed sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // b5ef1 mov r15,rsi
            r15 = rsi;
            // b5ef4 mov r14,rdi
            r14 = rdi;
            // b5ef7 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // b5efb mov rdx,r15
            rdx = r15;
            // b5efe call 0xebc96
            { let r = { let t = m.env.stub_va(0x463); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5f03 mov r12,rax
            r12 = rax;
            // b5f06 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // b5f0a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5f0d je 0xb5f13
            if fl.e() { pc = 0xb5f13; continue; }
            pc = 0xb5f0f;
        }
        0xb5f0f => {
            // block b5f0f
            // b5f0f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b5f11 jmp 0xb5f42
            pc = 0xb5f42; continue;
        }
        0xb5f13 => {
            // block b5f13
            // b5f13 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b5f17 mov rsi,r14
            rsi = r14;
            // b5f1a mov rdx,r15
            rdx = r15;
            // b5f1d call 0xebcae
            { let r = { let t = m.env.stub_va(0x47b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5f22 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b5f26 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b5f2a mov rdi,r14
            rdi = r14;
            // b5f2d mov rdx,r12
            rdx = r12;
            // b5f30 mov rcx,rbx
            rcx = rbx;
            // b5f33 call 0xebcb4
            { let r = { let t = m.env.stub_va(0x481); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5f38 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // b5f40 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xb5f42;
        }
        0xb5f42 => {
            // block b5f42
            // b5f42 mov rax,rbx
            rax = rbx;
            // b5f45 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // b5f49 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5f4a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b5f4c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5f4e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5f50 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5f51 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2d84 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2d84<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2d84;
    loop {
        match pc {
        0xe2d84 => {
            // block e2d84
            // e2d84 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2d85 mov rbp,rsp
            rbp = rsp;
            // e2d88 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2d8a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2d8c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2d8e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2d8f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2d93 mov r14,rcx
            r14 = rcx;
            // e2d96 mov r15,rdx
            r15 = rdx;
            // e2d99 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // e2d9c mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e2d9f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2da2 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // e2da6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2da8 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e2dad mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2db2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2db5 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2db8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2dba je 0xe2de5
            if fl.e() { pc = 0xe2de5; continue; }
            pc = 0xe2dbc;
        }
        0xe2dbc => {
            // block e2dbc
            // e2dbc add r12d,r12d
            { let (r, f) = add::<32>((r12 as u32 as u64), (r12 as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // e2dbf mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // e2dc4 mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e2dc7 shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e2dc9 and eax,DWORD PTR [rbp-0x24]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e2dcc mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e2dcf shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e2dd1 mov DWORD PTR [rbp-0x24],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rax as u32 as u64)) as u32);
            // e2dd4 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e2dd6 shr cl,1
            { let (r, f) = shr::<8>((rcx as u8 as u64), 0x1u64, fl); rcx = (rcx & !0xff) | ((r) as u8 as u64); fl = f; }
            // e2dd8 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e2ddb mov BYTE PTR [r15],cl
            m.w8(r15, ((rcx as u8 as u64)) as u8);
            // e2dde and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e2de0 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e2de3 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe2de5;
        }
        0xe2de5 => {
            // block e2de5
            // e2de5 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2de7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2deb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2dec pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2dee pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2df0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2df2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2df3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x975a size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000975a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x975a;
    loop {
        match pc {
        0x975a => {
            // block 975a
            // 975a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 975b mov rbp,rsp
            rbp = rsp;
            // 975e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9760 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9762 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9763 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 9764 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 9767 mov r14,rsi
            r14 = rsi;
            // 976a mov rbx,rdi
            rbx = rdi;
            // 976d mov DWORD PTR [r14],0x2
            m.w32(r14, (0x2u64) as u32);
            // 9774 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 9777 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 977a call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9780 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 9782 jne 0x97be
            if fl.ne() { pc = 0x97be; continue; }
            pc = 0x9784;
        }
        0x9784 => {
            // block 9784
            // 9784 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 978b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 978e mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 9791 lea rdx,[rip+0xf5f38] # 0xff6d0
            rdx = m.env.data_va(81565);
            // 9798 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 979b lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 979f mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 97a4 mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 97aa mov rdi,rbx
            rdi = rbx;
            // 97ad call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 97b0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 97b2 je 0x97be
            if fl.e() { pc = 0x97be; continue; }
            pc = 0x97b4;
        }
        0x97b4 => {
            // block 97b4
            // 97b4 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 97b7 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 97ba mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 97bc jmp 0x97c0
            pc = 0x97c0; continue;
        }
        0x97be => {
            // block 97be
            // 97be xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x97c0;
        }
        0x97c0 => {
            // block 97c0
            // 97c0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 97c4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 97c5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 97c7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 97c9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 97ca ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6530a size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006530a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6530a;
    loop {
        match pc {
        0x6530a => {
            // block 6530a
            // 6530a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6530b mov rbp,rsp
            rbp = rsp;
            // 6530e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 65310 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 65312 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65314 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65315 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 65319 mov r15,rsi
            r15 = rsi;
            // 6531c mov r14,rdi
            r14 = rdi;
            // 6531f lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 65323 mov rdx,r15
            rdx = r15;
            // 65326 call 0xebb1c
            { let r = { let t = m.env.stub_va(0x2e9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6532b mov r12,rax
            r12 = rax;
            // 6532e mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 65332 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 65335 je 0x6533b
            if fl.e() { pc = 0x6533b; continue; }
            pc = 0x65337;
        }
        0x65337 => {
            // block 65337
            // 65337 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65339 jmp 0x65362
            pc = 0x65362; continue;
        }
        0x6533b => {
            // block 6533b
            // 6533b mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 65340 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65345 mov rbx,rax
            rbx = rax;
            // 65348 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 6534b mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 6534e mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 65352 mov rdi,r14
            rdi = r14;
            // 65355 mov rdx,r12
            rdx = r12;
            // 65358 mov rcx,rbx
            rcx = rbx;
            // 6535b call 0xebcc6
            { let r = { let t = m.env.stub_va(0x493); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65360 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x65362;
        }
        0x65362 => {
            // block 65362
            // 65362 mov rax,rbx
            rax = rbx;
            // 65365 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 65369 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6536a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6536c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6536e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 65370 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 65371 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5770 size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5770<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5770;
    loop {
        match pc {
        0xb5770 => {
            // block b5770
            // b5770 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5771 mov rbp,rsp
            rbp = rsp;
            // b5774 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5776 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5778 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b577a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b577b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b577f mov r15,rsi
            r15 = rsi;
            // b5782 mov r14,rdi
            r14 = rdi;
            // b5785 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // b5789 mov rdx,r15
            rdx = r15;
            // b578c call 0xebbf4
            { let r = { let t = m.env.stub_va(0x3c1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5791 mov r12,rax
            r12 = rax;
            // b5794 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // b5798 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b579b jne 0xb57d0
            if fl.ne() { pc = 0xb57d0; continue; }
            pc = 0xb579d;
        }
        0xb579d => {
            // block b579d
            // b579d mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // b57a2 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b57a7 mov rbx,rax
            rbx = rax;
            // b57aa mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // b57ad mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // b57b0 mov DWORD PTR [rbx+0x20],0x0
            m.w32(rbx.wrapping_add(0x20), (0x0u64) as u32);
            // b57b7 mov DWORD PTR [rbx+0x24],0x0
            m.w32(rbx.wrapping_add(0x24), (0x0u64) as u32);
            // b57be mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b57c2 mov rdi,r14
            rdi = r14;
            // b57c5 mov rdx,r12
            rdx = r12;
            // b57c8 mov rcx,rbx
            rcx = rbx;
            // b57cb call 0xebc12
            { let r = { let t = m.env.stub_va(0x3df); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb57d0;
        }
        0xb57d0 => {
            // block b57d0
            // b57d0 add rbx,0x20
            { let (r, f) = add::<64>(rbx, 0x20u64);
              rbx = r;
              fl = f; }
            // b57d4 mov rax,rbx
            rax = rbx;
            // b57d7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b57db pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b57dc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b57de pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b57e0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b57e2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b57e3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd36d8 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d36d8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd36d8;
    loop {
        match pc {
        0xd36d8 => {
            // block d36d8
            // d36d8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d36d9 mov rbp,rsp
            rbp = rsp;
            // d36dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d36dd sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // d36e1 mov rbx,rdx
            rbx = rdx;
            // d36e4 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d36e7 je 0xd3744
            if fl.e() { pc = 0xd3744; continue; }
            pc = 0xd36e9;
        }
        0xd36e9 => {
            // block d36e9
            // d36e9 xorps xmm0,xmm0
            x0 = 0;
            // d36ec movaps XMMWORD PTR [rbp-0x20],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), x0);
            // d36f0 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // d36f4 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d36f8 mov DWORD PTR [rbp-0x40],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x2u64) as u32);
            // d36ff mov DWORD PTR [rbp-0x3c],0x30
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0x30u64) as u32);
            // d3706 mov DWORD PTR [rbp-0x28],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), ((rsi as u32 as u64)) as u32);
            // d3709 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // d3710 mov DWORD PTR [rbp-0x20],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u32);
            // d3717 mov DWORD PTR [rbp-0x1c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rcx as u32 as u64)) as u32);
            // d371a mov DWORD PTR [rbp-0x18],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), ((r8 as u32 as u64)) as u32);
            // d371e mov BYTE PTR [rbp-0x14],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u8);
            // d3722 mov BYTE PTR [rbp-0x13],0x1
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffed), (0x1u64) as u8);
            // d3726 mov BYTE PTR [rbp-0x12],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffee), (0x0u64) as u8);
            // d372a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d372d lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d3731 call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3737 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d3739 je 0xd3744
            if fl.e() { pc = 0xd3744; continue; }
            pc = 0xd373b;
        }
        0xd373b => {
            // block d373b
            // d373b mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // d373e mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // d3740 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d3742 jmp 0xd3746
            pc = 0xd3746; continue;
        }
        0xd3744 => {
            // block d3744
            // d3744 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3746;
        }
        0xd3746 => {
            // block d3746
            // d3746 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // d374a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d374b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d374c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2e82 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2e82<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2e82;
    loop {
        match pc {
        0xe2e82 => {
            // block e2e82
            // e2e82 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2e83 mov rbp,rsp
            rbp = rsp;
            // e2e86 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2e88 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2e8a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2e8c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2e8d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2e91 mov r14,rcx
            r14 = rcx;
            // e2e94 mov r15,rdx
            r15 = rdx;
            // e2e97 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // e2e9a mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e2e9d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2ea0 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // e2ea4 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2ea6 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e2eab mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2eb0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2eb3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2eb6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2eb8 je 0xe2ee8
            if fl.e() { pc = 0xe2ee8; continue; }
            pc = 0xe2eba;
        }
        0xe2eba => {
            // block e2eba
            // e2eba mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // e2ebd shr eax,0x8
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e2ec0 add r12d,r12d
            { let (r, f) = add::<32>((r12 as u32 as u64), (r12 as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // e2ec3 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // e2ec8 mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e2ecb shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e2ecd and edx,eax
            { let r = (rdx as u32 as u64) & (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e2ecf mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e2ed2 shr edx,cl
            { let (r, f) = shr::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e2ed4 mov DWORD PTR [rbp-0x24],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rdx as u32 as u64)) as u32);
            // e2ed7 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // e2ed9 shr al,1
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x1u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // e2edb and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e2edd mov BYTE PTR [r15],al
            m.w8(r15, ((rax as u8 as u64)) as u8);
            // e2ee0 and dl,0x1
            { let r = (rdx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e2ee3 mov BYTE PTR [r14],dl
            m.w8(r14, ((rdx as u8 as u64)) as u8);
            // e2ee6 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe2ee8;
        }
        0xe2ee8 => {
            // block e2ee8
            // e2ee8 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2eea add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2eee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2eef pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2ef1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2ef3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2ef5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2ef6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42f68 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042f68<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42f68;
    loop {
        match pc {
        0x42f68 => {
            // block 42f68
            // 42f68 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42f69 mov rbp,rsp
            rbp = rsp;
            // 42f6c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42f6e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42f70 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42f71 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42f72 mov r14,rdx
            r14 = rdx;
            // 42f75 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 42f78 mov rbx,rdi
            rbx = rdi;
            // 42f7b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42f7e call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42f84 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 42f86 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 42f88 cmp r15d,0xf
            fl = sub::<32>((r15 as u32 as u64), 0xfu64).1;
            // 42f8c ja 0x42fd5
            if fl.a() { pc = 0x42fd5; continue; }
            pc = 0x42f8e;
        }
        0x42f8e => {
            // block 42f8e
            // 42f8e test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 42f90 je 0x42fd5
            if fl.e() { pc = 0x42fd5; continue; }
            pc = 0x42f92;
        }
        0x42f92 => {
            // block 42f92
            // 42f92 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 42f99 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42f9c mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 42f9f lea rcx,[rip+0xbd52a] # 0x1004d0
            rcx = m.env.data_va(85149);
            // 42fa6 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42fa9 lea rsi,[rip+0xbd570] # 0x100520
            rsi = m.env.data_va(85229);
            // 42fb0 mov r8d,DWORD PTR [rsi+rdx*4]
            r8 = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42fb4 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 42fb8 mov esi,0x905
            rsi = (0x905u64) as u32 as u64;
            // 42fbd mov rdi,rbx
            rdi = rbx;
            // 42fc0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42fc3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42fc5 je 0x42fd3
            if fl.e() { pc = 0x42fd3; continue; }
            pc = 0x42fc7;
        }
        0x42fc7 => {
            // block 42fc7
            // 42fc7 cmp DWORD PTR [rbp-0x1c],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x1u64).1;
            // 42fcb sete BYTE PTR [r14]
            m.w8(r14, ((fl.e() as u64)) as u8);
            // 42fcf mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 42fd1 jmp 0x42fd5
            pc = 0x42fd5; continue;
        }
        0x42fd3 => {
            // block 42fd3
            // 42fd3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42fd5;
        }
        0x42fd5 => {
            // block 42fd5
            // 42fd5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42fd9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42fda pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42fdc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42fde pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42fdf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44fb6 size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044fb6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44fb6;
    loop {
        match pc {
        0x44fb6 => {
            // block 44fb6
            // 44fb6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44fb7 mov rbp,rsp
            rbp = rsp;
            // 44fba push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44fbc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44fbe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44fbf sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 44fc3 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // 44fc6 mov r14,rdx
            r14 = rdx;
            // 44fc9 mov r8d,esi
            r8 = ((rsi as u32 as u64)) as u32 as u64;
            // 44fcc mov rbx,rdi
            rbx = rdi;
            // 44fcf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 44fd1 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 44fd4 je 0x45027
            if fl.e() { pc = 0x45027; continue; }
            pc = 0x44fd6;
        }
        0x44fd6 => {
            // block 44fd6
            // 44fd6 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 44fd9 je 0x45027
            if fl.e() { pc = 0x45027; continue; }
            pc = 0x44fdb;
        }
        0x44fdb => {
            // block 44fdb
            // 44fdb mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 44fe2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 44fe5 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 44fe9 mov rdi,rbx
            rdi = rbx;
            // 44fec mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 44fee mov ecx,r8d
            rcx = ((r8 as u32 as u64)) as u32 as u64;
            // 44ff1 call QWORD PTR [rax+0x720]
            { let r = { let t = m.r64(rax.wrapping_add(0x720)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44ff7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44ff9 je 0x45025
            if fl.e() { pc = 0x45025; continue; }
            pc = 0x44ffb;
        }
        0x44ffb => {
            // block 44ffb
            // 44ffb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 44ffe mov r9d,DWORD PTR [rbp-0x1c]
            r9 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 45002 mov DWORD PTR [rsp],r15d
            m.w32(rsp, ((r15 as u32 as u64)) as u32);
            // 45006 mov DWORD PTR [rsp+0x8],0x1
            m.w32(rsp.wrapping_add(0x8), (0x1u64) as u32);
            // 4500e mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 45013 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 45018 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4501a mov rdi,rbx
            rdi = rbx;
            // 4501d mov r8,r14
            r8 = r14;
            // 45020 call QWORD PTR [rax+0x58]
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 45023 jmp 0x45027
            pc = 0x45027; continue;
        }
        0x45025 => {
            // block 45025
            // 45025 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x45027;
        }
        0x45027 => {
            // block 45027
            // 45027 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4502b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4502c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4502e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 45030 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 45031 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x9b4 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000009b4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x9b4;
    loop {
        match pc {
        0x9b4 => {
            // block 9b4
            // 9b4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 9b5 mov rbp,rsp
            rbp = rsp;
            // 9b8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9ba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9bc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 9be push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 9c0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9c1 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 9c5 mov QWORD PTR [rbp-0x38],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r9) as u64);
            // 9c9 mov r15,r8
            r15 = r8;
            // 9cc mov r12,rcx
            r12 = rcx;
            // 9cf mov r13,rdx
            r13 = rdx;
            // 9d2 mov rbx,rsi
            rbx = rsi;
            // 9d5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 9d8 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 9dc xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 9df xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 9e1 call QWORD PTR [rax+0x3b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3b0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9e7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 9e9 je 0xa1f
            if fl.e() { pc = 0xa1f; continue; }
            pc = 0x9eb;
        }
        0x9eb => {
            // block 9eb
            // 9eb mov rax,QWORD PTR [rbp+0x10]
            rax = m.r64(rbp.wrapping_add(0x10)) as u64;
            // 9ef mov DWORD PTR [rbx],0x1000
            m.w32(rbx, (0x1000u64) as u32);
            // 9f5 mov DWORD PTR [r13+0x0],0x800
            m.w32(r13, (0x800u64) as u32);
            // 9fd mov DWORD PTR [r12],0x1800
            m.w32(r12, (0x1800u64) as u32);
            // a05 mov DWORD PTR [r15],0x800
            m.w32(r15, (0x800u64) as u32);
            // a0c mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // a10 mov DWORD PTR [rcx],0x400
            m.w32(rcx, (0x400u64) as u32);
            // a16 mov DWORD PTR [rax],0xc00
            m.w32(rax, (0xc00u64) as u32);
            // a1c mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xa1f;
        }
        0xa1f => {
            // block a1f
            // a1f mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // a22 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // a26 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // a27 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // a29 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // a2b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // a2d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // a2f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a30 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3bd0 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3bd0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3bd0;
    loop {
        match pc {
        0xc3bd0 => {
            // block c3bd0
            // c3bd0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3bd1 mov rbp,rsp
            rbp = rsp;
            // c3bd4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3bd6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3bd8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3bda push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3bdb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3bdf mov r12,rsi
            r12 = rsi;
            // c3be2 mov r14,rdi
            r14 = rdi;
            // c3be5 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3be9 mov rdx,r12
            rdx = r12;
            // c3bec call 0xebd32
            { let r = { let t = m.env.stub_va(0x4ff); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3bf1 mov r15,rax
            r15 = rax;
            // c3bf4 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3bf7 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3bfa jne 0xc3c3a
            if fl.ne() { pc = 0xc3c3a; continue; }
            pc = 0xc3bfc;
        }
        0xc3bfc => {
            // block c3bfc
            // c3bfc mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c3c01 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3c06 mov rbx,rax
            rbx = rax;
            // c3c09 mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // c3c0d mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // c3c10 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // c3c18 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // c3c20 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // c3c28 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3c2c mov rdi,r14
            rdi = r14;
            // c3c2f mov rdx,r15
            rdx = r15;
            // c3c32 mov rcx,rbx
            rcx = rbx;
            // c3c35 call 0xebe4c
            { let r = { let t = m.env.stub_va(0x619); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3c3a;
        }
        0xc3c3a => {
            // block c3c3a
            // c3c3a add rbx,0x28
            { let (r, f) = add::<64>(rbx, 0x28u64);
              rbx = r;
              fl = f; }
            // c3c3e mov rax,rbx
            rax = rbx;
            // c3c41 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3c45 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3c46 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3c48 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3c4a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3c4c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3c4d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2a48 size=140
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002a48<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2a48;
    loop {
        match pc {
        0x2a48 => {
            // block 2a48
            // 2a48 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2a49 mov rbp,rsp
            rbp = rsp;
            // 2a4c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2a4e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2a4f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2a53 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 2a56 mov rbx,rdi
            rbx = rdi;
            // 2a59 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2a5c call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2a62 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2a64 jne 0x2ac9
            if fl.ne() { pc = 0x2ac9; continue; }
            pc = 0x2a66;
        }
        0x2a66 => {
            // block 2a66
            // 2a66 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2a69 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 2a6d mov rdi,rbx
            rdi = rbx;
            // 2a70 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 2a73 call QWORD PTR [rax+0x530]
            { let r = { let t = m.r64(rax.wrapping_add(0x530)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2a79 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2a7b je 0x2ac9
            if fl.e() { pc = 0x2ac9; continue; }
            pc = 0x2a7d;
        }
        0x2a7d => {
            // block 2a7d
            // 2a7d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2a80 lea rdx,[rbp-0x18]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 2a84 mov rdi,rbx
            rdi = rbx;
            // 2a87 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 2a8a call QWORD PTR [rax+0x548]
            { let r = { let t = m.r64(rax.wrapping_add(0x548)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2a90 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2a92 je 0x2ac9
            if fl.e() { pc = 0x2ac9; continue; }
            pc = 0x2a94;
        }
        0x2a94 => {
            // block 2a94
            // 2a94 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2a97 mov edx,DWORD PTR [rbp-0x14]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 2a9a mov rdi,rbx
            rdi = rbx;
            // 2a9d mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 2aa0 call QWORD PTR [rax+0x540]
            { let r = { let t = m.r64(rax.wrapping_add(0x540)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2aa6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2aa8 je 0x2ac9
            if fl.e() { pc = 0x2ac9; continue; }
            pc = 0x2aaa;
        }
        0x2aaa => {
            // block 2aaa
            // 2aaa mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2aad mov edx,DWORD PTR [rbp-0x18]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 2ab0 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 2ab5 mov rdi,rbx
            rdi = rbx;
            // 2ab8 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 2abb call QWORD PTR [rax+0x528]
            { let r = { let t = m.r64(rax.wrapping_add(0x528)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ac1 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2ac3 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2ac5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 2ac7 jne 0x2acb
            if fl.ne() { pc = 0x2acb; continue; }
            pc = 0x2ac9;
        }
        0x2ac9 => {
            // block 2ac9
            // 2ac9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2acb;
        }
        0x2acb => {
            // block 2acb
            // 2acb add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2acf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2ad0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2ad2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ad3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f10 size=144
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002f10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2f10;
    loop {
        match pc {
        0x2f10 => {
            // block 2f10
            // 2f10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f11 mov rbp,rsp
            rbp = rsp;
            // 2f14 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f17 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f1b mov r14,rsi
            r14 = rsi;
            // 2f1e mov rbx,rdi
            rbx = rdi;
            // 2f21 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 2f28 mov DWORD PTR [rbp-0x18],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0x0u64) as u32);
            // 2f2f mov DWORD PTR [rbp-0x1c],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x5u64) as u32);
            // 2f36 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2f39 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 2f3d mov esi,0x2880
            rsi = (0x2880u64) as u32 as u64;
            // 2f42 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2f47 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f4a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f4d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2f50 lea rdx,[rbp-0x18]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 2f54 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // 2f59 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2f5e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f61 mov rdi,rbx
            rdi = rbx;
            // 2f64 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f67 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2f6a lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 2f6e mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // 2f73 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2f78 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f7b mov rdi,rbx
            rdi = rbx;
            // 2f7e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f81 mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 2f84 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 2f87 mov eax,DWORD PTR [rbp-0x18]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 2f8a mov DWORD PTR [r14+0x4],eax
            m.w32(r14.wrapping_add(0x4), ((rax as u32 as u64)) as u32);
            // 2f8e mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 2f91 mov DWORD PTR [r14+0x8],eax
            m.w32(r14.wrapping_add(0x8), ((rax as u32 as u64)) as u32);
            // 2f95 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2f97 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f9b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f9c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f9e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f9f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3101e size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0003101e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3101e;
    loop {
        match pc {
        0x3101e => {
            // block 3101e
            // 3101e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3101f mov rbp,rsp
            rbp = rsp;
            // 31022 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31024 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31025 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 31029 mov r14,rdx
            r14 = rdx;
            // 3102c mov rbx,rdi
            rbx = rdi;
            // 3102f xorps xmm0,xmm0
            x0 = 0;
            // 31032 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // 31036 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 3103e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31041 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 31045 mov ecx,DWORD PTR [r14+0x8]
            rcx = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // 31049 sub ecx,DWORD PTR [r14]
            { let (r, f) = sub::<32>((rcx as u32 as u64), m.r32(r14) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3104c lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 31050 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31052 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31054 je 0x3108d
            if fl.e() { pc = 0x3108d; continue; }
            pc = 0x31056;
        }
        0x31056 => {
            // block 31056
            // 31056 movzx edx,BYTE PTR [rbx+0x8]
            rdx = (m.r8(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 3105a lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 3105e mov rdi,r14
            rdi = r14;
            // 31061 call 0x2eee0
            { let r = { let t = m.env.code_va(0x2eee0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31066 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 31068 jmp 0x3108f
            pc = 0x3108f; continue;
        }
        0x3106a => {
            // block 3106a
            // 3106a mov rbx,rax
            rbx = rax;
            // 3106d mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 31071 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31074 je 0x31085
            if fl.e() { pc = 0x31085; continue; }
            pc = 0x31076;
        }
        0x31076 => {
            // block 31076
            // 31076 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 3107a je 0x31080
            if fl.e() { pc = 0x31080; continue; }
            pc = 0x3107c;
        }
        0x3107c => {
            // block 3107c
            // 3107c mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x31080;
        }
        0x31080 => {
            // block 31080
            // 31080 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31085;
        }
        0x31085 => {
            // block 31085
            // 31085 mov rdi,rbx
            rdi = rbx;
            // 31088 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x3108d;
        }
        0x3108d => {
            // block 3108d
            // 3108d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x3108f;
        }
        0x3108f => {
            // block 3108f
            // 3108f mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 31093 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31096 je 0x310a7
            if fl.e() { pc = 0x310a7; continue; }
            pc = 0x31098;
        }
        0x31098 => {
            // block 31098
            // 31098 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 3109c je 0x310a2
            if fl.e() { pc = 0x310a2; continue; }
            pc = 0x3109e;
        }
        0x3109e => {
            // block 3109e
            // 3109e mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x310a2;
        }
        0x310a2 => {
            // block 310a2
            // 310a2 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x310a7;
        }
        0x310a7 => {
            // block 310a7
            // 310a7 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 310a9 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 310ad pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 310ae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 310b0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 310b1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2c64 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2c64<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2c64;
    loop {
        match pc {
        0xe2c64 => {
            // block e2c64
            // e2c64 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2c65 mov rbp,rsp
            rbp = rsp;
            // e2c68 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2c6a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2c6c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2c6d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e2c6e mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // e2c71 mov r15,rdi
            r15 = rdi;
            // e2c74 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e2c7b mov ebx,0x43864
            rbx = (0x43864u64) as u32 as u64;
            // e2c80 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // e2c83 ja 0xe2c9c
            if fl.a() { pc = 0xe2c9c; continue; }
            pc = 0xe2c85;
        }
        0xe2c85 => {
            // block e2c85
            // e2c85 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // e2c88 ja 0xe2c9c
            if fl.a() { pc = 0xe2c9c; continue; }
            pc = 0xe2c8a;
        }
        0xe2c8a => {
            // block e2c8a
            // e2c8a cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // e2c8d ja 0xe2c9c
            if fl.a() { pc = 0xe2c9c; continue; }
            pc = 0xe2c8f;
        }
        0xe2c8f => {
            // block e2c8f
            // e2c8f lea eax,[rdx+rsi*4]
            rax = (rdx.wrapping_add(rsi.wrapping_mul(4))) as u32 as u64;
            // e2c92 lea eax,[rcx+rax*4]
            rax = (rcx.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e2c95 lea ebx,[rax+rax*2+0x43864]
            rbx = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x43864)) as u32 as u64;
            pc = 0xe2c9c;
        }
        0xe2c9c => {
            // block e2c9c
            // e2c9c mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e2c9f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2ca2 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e2ca6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2cab xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2cae mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e2cb0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2cb3 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // e2cb7 je 0xe2cec
            if fl.e() { pc = 0xe2cec; continue; }
            pc = 0xe2cb9;
        }
        0xe2cb9 => {
            // block e2cb9
            // e2cb9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e2cbc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2cbf mov edx,0x80000000
            rdx = (0x80000000u64) as u32 as u64;
            // e2cc4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2cc9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2ccc mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e2cce call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2cd1 movzx eax,r14b
            rax = ((r14 as u8 as u64)) as u32 as u64;
            // e2cd5 lea edx,[rax+rax*1+0x1]
            rdx = (rax.wrapping_add(rax).wrapping_add(0x1)) as u32 as u64;
            // e2cd9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e2cdc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2cdf mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2ce4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2ce7 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e2ce9 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2cec;
        }
        0xe2cec => {
            // block e2cec
            // e2cec add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e2cf0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2cf1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2cf3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2cf5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2cf6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x682b0 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000682b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x682b0;
    loop {
        match pc {
        0x682b0 => {
            // block 682b0
            // 682b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 682b1 mov rbp,rsp
            rbp = rsp;
            // 682b4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 682b6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 682b8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 682ba push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 682bc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 682bd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 682be mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 682c1 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // 682c4 mov rbx,rdi
            rbx = rdi;
            // 682c7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 682ca mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 682cf mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 682d2 call QWORD PTR [rax+0xbd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 682d8 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 682db mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 682e2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 682e5 mov r13d,r14d
            r13 = ((r14 as u32 as u64)) as u32 as u64;
            // 682e8 lea rcx,[rip+0x99a01] # 0x101cf0
            rcx = m.env.data_va(91325);
            // 682ef mov esi,DWORD PTR [rcx+r13*4]
            rsi = (m.r32(rcx.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 682f3 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 682f7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 682fc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 682ff mov rdi,rbx
            rdi = rbx;
            // 68302 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68305 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 68308 lea rcx,[rip+0x99a11] # 0x101d20
            rcx = m.env.data_va(91373);
            // 6830f mov cl,BYTE PTR [rcx+r13*4]
            rcx = (rcx & !0xff) | ((m.r8(rcx.wrapping_add(r13.wrapping_mul(4))) as u64) as u8 as u64);
            // 68313 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 68315 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 68318 cmp eax,r12d
            fl = sub::<32>((rax as u32 as u64), (r12 as u32 as u64)).1;
            // 6831b je 0x68334
            if fl.e() { pc = 0x68334; continue; }
            pc = 0x6831d;
        }
        0x6831d => {
            // block 6831d
            // 6831d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 68320 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 68325 mov rdi,rbx
            rdi = rbx;
            // 68328 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 6832b call QWORD PTR [rax+0xbd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68331 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x68334;
        }
        0x68334 => {
            // block 68334
            // 68334 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 68337 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6833b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6833c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6833e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 68340 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 68342 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 68344 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68345 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3b36 size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3b36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3b36;
    loop {
        match pc {
        0xc3b36 => {
            // block c3b36
            // c3b36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3b37 mov rbp,rsp
            rbp = rsp;
            // c3b3a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3b3c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3b3e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3b40 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3b41 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3b45 mov r12,rsi
            r12 = rsi;
            // c3b48 mov r14,rdi
            r14 = rdi;
            // c3b4b lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3b4f mov rdx,r12
            rdx = r12;
            // c3b52 call 0xebd3e
            { let r = { let t = m.env.stub_va(0x50b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3b57 mov r15,rax
            r15 = rax;
            // c3b5a mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3b5d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3b60 jne 0xc3bbc
            if fl.ne() { pc = 0xc3bbc; continue; }
            pc = 0xc3b62;
        }
        0xc3b62 => {
            // block c3b62
            // c3b62 mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c3b67 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3b6c mov rbx,rax
            rbx = rax;
            // c3b6f mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // c3b74 mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // c3b78 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // c3b7c mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // c3b81 mov QWORD PTR [rbx+0x28],rcx
            m.w64(rbx.wrapping_add(0x28), (rcx) as u64);
            // c3b85 mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // c3b89 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // c3b92 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // c3b9b mov QWORD PTR [r12],0x0
            m.w64(r12, (0x0u64) as u64);
            // c3ba3 mov DWORD PTR [rbx+0x38],0x0
            m.w32(rbx.wrapping_add(0x38), (0x0u64) as u32);
            // c3baa mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3bae mov rdi,r14
            rdi = r14;
            // c3bb1 mov rdx,r15
            rdx = r15;
            // c3bb4 mov rcx,rbx
            rcx = rbx;
            // c3bb7 call 0xebe64
            { let r = { let t = m.env.stub_va(0x631); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3bbc;
        }
        0xc3bbc => {
            // block c3bbc
            // c3bbc add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // c3bc0 mov rax,rbx
            rax = rbx;
            // c3bc3 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3bc7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3bc8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3bca pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3bcc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3bce pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3bcf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5c8e size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5c8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd5c8e;
    loop {
        match pc {
        0xd5c8e => {
            // block d5c8e
            // d5c8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5c8f mov rbp,rsp
            rbp = rsp;
            // d5c92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5c93 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d5c94 mov rbx,rdi
            rbx = rdi;
            // d5c97 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // d5c9e mov DWORD PTR [rbp-0x10],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u32);
            // d5ca5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5ca8 lea rdx,[rbp-0x10]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // d5cac mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // d5cb1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5cb6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5cb9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5cbc mov ecx,DWORD PTR [rbp-0x10]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64) as u32 as u64;
            // d5cbf cmp ecx,0x106466ff
            fl = sub::<32>((rcx as u32 as u64), 0x106466ffu64).1;
            // d5cc5 jg 0xd5cfe
            if fl.g() { pc = 0xd5cfe; continue; }
            pc = 0xd5cc7;
        }
        0xd5cc7 => {
            // block d5cc7
            // d5cc7 cmp ecx,0x10518400
            fl = sub::<32>((rcx as u32 as u64), 0x10518400u64).1;
            // d5ccd je 0xd5cd7
            if fl.e() { pc = 0xd5cd7; continue; }
            pc = 0xd5ccf;
        }
        0xd5ccf => {
            // block d5ccf
            // d5ccf cmp ecx,0x10518450
            fl = sub::<32>((rcx as u32 as u64), 0x10518450u64).1;
            // d5cd5 jne 0xd5d21
            if fl.ne() { pc = 0xd5d21; continue; }
            pc = 0xd5cd7;
        }
        0xd5cd7 => {
            // block d5cd7
            // d5cd7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5cda lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d5cde mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // d5ce3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5ce8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5ceb mov rdi,rbx
            rdi = rbx;
            // d5cee call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5cf1 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // d5cf4 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d5cf6 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d5cf9 seta al
            rax = (rax & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // d5cfc jmp 0xd5d23
            pc = 0xd5d23; continue;
        }
        0xd5cfe => {
            // block d5cfe
            // d5cfe mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d5d00 lea edx,[rcx-0x10646700]
            rdx = (rcx.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d5d06 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // d5d09 jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d0b;
        }
        0xd5d0b => {
            // block d5d0b
            // d5d0b lea edx,[rcx-0x10646705]
            rdx = (rcx.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d5d11 cmp edx,0x2
            fl = sub::<32>((rdx as u32 as u64), 0x2u64).1;
            // d5d14 jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d16;
        }
        0xd5d16 => {
            // block d5d16
            // d5d16 add ecx,0xef8ef7b0
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xef8ef7b0u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d5d1c cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // d5d1f jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d21;
        }
        0xd5d21 => {
            // block d5d21
            // d5d21 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd5d23;
        }
        0xd5d23 => {
            // block d5d23
            // d5d23 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d5d27 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5d28 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5d29 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3914 size=172
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003914<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3914;
    loop {
        match pc {
        0x3914 => {
            // block 3914
            // 3914 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3915 mov rbp,rsp
            rbp = rsp;
            // 3918 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 391a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 391c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 391e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 3920 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3921 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3922 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 3925 mov r12,rdi
            r12 = rdi;
            // 3928 shl r14d,0xe
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0xeu64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 392c and r14d,0x3fffc000
            { let r = (r14 as u32 as u64) & 0x3fffc000u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 3933 je 0x39ab
            if fl.e() { pc = 0x39ab; continue; }
            pc = 0x3935;
        }
        0x3935 => {
            // block 3935
            // 3935 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 3938 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 393a xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x393d;
        }
        0x393d => {
            // block 393d
            // 393d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3941 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 3946 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 394b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 394e mov rdi,r12
            rdi = r12;
            // 3951 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 3953 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3956 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 395a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 395f mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 3964 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3969 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 396c mov rdi,r12
            rdi = r12;
            // 396f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3972 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3976 mov rdi,r12
            rdi = r12;
            // 3979 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 397f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3983 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // 3988 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 398d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3990 mov rdi,r12
            rdi = r12;
            // 3993 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 3997 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 399a cmp DWORD PTR [rbp-0x2c],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64, 0xffffffffu64).1;
            // 399e jne 0x39ae
            if fl.ne() { pc = 0x39ae; continue; }
            pc = 0x39a0;
        }
        0x39a0 => {
            // block 39a0
            // 39a0 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 39a3 add ebx,0x4
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x4u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 39a6 cmp r15d,r14d
            fl = sub::<32>((r15 as u32 as u64), (r14 as u32 as u64)).1;
            // 39a9 jb 0x393d
            if fl.b() { pc = 0x393d; continue; }
            pc = 0x39ab;
        }
        0x39ab => {
            // block 39ab
            // 39ab mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x39ae;
        }
        0x39ae => {
            // block 39ae
            // 39ae mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 39b1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 39b5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 39b6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 39b8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 39ba pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 39bc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 39be pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 39bf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c6 size=180
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000005c6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5c6;
    loop {
        match pc {
        0x5c6 => {
            // block 5c6
            // 5c6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c7 mov rbp,rsp
            rbp = rsp;
            // 5ca push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5cc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5ce push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5d0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5d1 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 5d5 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 5d8 mov r14,rsi
            r14 = rsi;
            // 5db mov rbx,rdi
            rbx = rdi;
            // 5de mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5e1 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5e7 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 5ea test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5ec cmovne r15d,r12d
            if fl.ne() { r15 = ((r12 as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // 5f0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5f3 lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 5f7 mov rdi,rbx
            rdi = rbx;
            // 5fa mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 5fd call QWORD PTR [rax+0x3e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x3e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 603 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 606 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 60a mov rdi,rbx
            rdi = rbx;
            // 60d mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 610 call QWORD PTR [rax+0x380]
            { let r = { let t = m.r64(rax.wrapping_add(0x380)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 616 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 619 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 61d mov rdi,rbx
            rdi = rbx;
            // 620 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 623 call QWORD PTR [rax+0x428]
            { let r = { let t = m.r64(rax.wrapping_add(0x428)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 629 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 62c lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 630 mov rdi,rbx
            rdi = rbx;
            // 633 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 636 call QWORD PTR [rax+0x440]
            { let r = { let t = m.r64(rax.wrapping_add(0x440)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 63f lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 643 mov rdi,rbx
            rdi = rbx;
            // 646 call QWORD PTR [rax+0x458]
            { let r = { let t = m.r64(rax.wrapping_add(0x458)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64c mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 64f mov edx,DWORD PTR [rbp-0x28]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 652 mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 655 mov r8d,DWORD PTR [rbp-0x30]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 659 cmp DWORD PTR [rbp-0x34],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64, 0x0u64).1;
            // 65d setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 660 movzx r9d,al
            r9 = ((rax as u8 as u64)) as u32 as u64;
            // 664 mov rdi,r14
            rdi = r14;
            // 667 call 0xd2260
            { let r = { let t = m.env.code_va(0xd2260); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 66c add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 670 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 671 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 673 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 675 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 677 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 678 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3ef4 size=188
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003ef4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3ef4;
    loop {
        match pc {
        0x3ef4 => {
            // block 3ef4
            // 3ef4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3ef5 mov rbp,rsp
            rbp = rsp;
            // 3ef8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 3efa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3efc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3efd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3efe mov r14,rcx
            r14 = rcx;
            // 3f01 mov r15,rdx
            r15 = rdx;
            // 3f04 mov rbx,rsi
            rbx = rsi;
            // 3f07 mov WORD PTR [r14],0x0
            m.w16(r14, (0x0u64) as u16);
            // 3f0d mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 3f13 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // 3f18 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 3f1f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 3f22 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 3f26 mov esi,0x59
            rsi = (0x59u64) as u32 as u64;
            // 3f2b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3f30 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3f33 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3f36 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3f38 je 0x3fa3
            if fl.e() { pc = 0x3fa3; continue; }
            pc = 0x3f3a;
        }
        0x3f3a => {
            // block 3f3a
            // 3f3a mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 3f3d mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 3f3f shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 3f42 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 3f44 shr edx,0x8
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x8u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 3f47 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 3f49 shr esi,0x14
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x14u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 3f4c and esi,0xf
            { let r = (rsi as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 3f4f lea esi,[rsi+rsi*4]
            rsi = (rsi.wrapping_add(rsi.wrapping_mul(4))) as u32 as u64;
            // 3f52 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3f55 lea ecx,[rcx+rsi*2]
            rcx = (rcx.wrapping_add(rsi.wrapping_mul(2))) as u32 as u64;
            // 3f58 mov WORD PTR [rbx],cx
            m.w16(rbx, ((rcx as u16 as u64)) as u16);
            // 3f5b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 3f5d shr ecx,0xc
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0xcu64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 3f60 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3f63 lea ecx,[rcx+rcx*4]
            rcx = (rcx.wrapping_add(rcx.wrapping_mul(4))) as u32 as u64;
            // 3f66 and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3f69 lea ecx,[rdx+rcx*2]
            rcx = (rdx.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // 3f6c mov WORD PTR [r15],cx
            m.w16(r15, ((rcx as u16 as u64)) as u16);
            // 3f70 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 3f72 shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 3f75 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3f78 lea ecx,[rcx+rcx*4]
            rcx = (rcx.wrapping_add(rcx.wrapping_mul(4))) as u32 as u64;
            // 3f7b and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3f7e lea eax,[rax+rcx*2]
            rax = (rax.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // 3f81 mov WORD PTR [r14],ax
            m.w16(r14, ((rax as u16 as u64)) as u16);
            // 3f85 movzx ecx,WORD PTR [rbx]
            rcx = (m.r16(rbx) as u64) as u32 as u64;
            // 3f88 cmp ecx,0x18
            fl = sub::<32>((rcx as u32 as u64), 0x18u64).1;
            // 3f8b sbb cl,cl
            { let (r, f) = sbb::<8>((rcx as u8 as u64), (rcx as u8 as u64), fl.cf);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3f8d movzx edx,WORD PTR [r15]
            rdx = (m.r16(r15) as u64) as u32 as u64;
            // 3f91 cmp edx,0x3c
            fl = sub::<32>((rdx as u32 as u64), 0x3cu64).1;
            // 3f94 sbb dl,dl
            { let (r, f) = sbb::<8>((rdx as u8 as u64), (rdx as u8 as u64), fl.cf);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3f96 and dl,cl
            { let r = (rdx as u8 as u64) & (rcx as u8 as u64); let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3f98 cmp eax,0x3c
            fl = sub::<32>((rax as u32 as u64), 0x3cu64).1;
            // 3f9b sbb al,al
            { let (r, f) = sbb::<8>((rax as u8 as u64), (rax as u8 as u64), fl.cf);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3f9d and al,dl
            { let r = (rax as u8 as u64) & (rdx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3f9f and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3fa1 jmp 0x3fa5
            pc = 0x3fa5; continue;
        }
        0x3fa3 => {
            // block 3fa3
            // 3fa3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x3fa5;
        }
        0x3fa5 => {
            // block 3fa5
            // 3fa5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 3fa9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3faa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3fac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3fae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3faf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4403e size=194
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004403e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4403e;
    loop {
        match pc {
        0x4403e => {
            // block 4403e
            // 4403e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4403f mov rbp,rsp
            rbp = rsp;
            // 44042 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44044 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44046 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 44048 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44049 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 4404d mov r15,rcx
            r15 = rcx;
            // 44050 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 44052 lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 44056 mov QWORD PTR [rbp-0x28],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u64);
            // 4405e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 44066 mov QWORD PTR [rbp-0x38],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r12) as u64);
            // 4406a mov BYTE PTR [r15],0x0
            m.w8(r15, (0x0u64) as u8);
            // 4406e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 44071 mov rax,QWORD PTR [rax+0x810]
            rax = m.r64(rax.wrapping_add(0x810)) as u64;
            // 44078 lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 4407c call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4407e mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 44082 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44084 je 0x440b4
            if fl.e() { pc = 0x440b4; continue; }
            pc = 0x44086;
        }
        0x44086 => {
            // block 44086
            // 44086 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 44089 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 4408c je 0x440cc
            if fl.e() { pc = 0x440cc; continue; }
            pc = 0x4408e;
        }
        0x4408e => {
            // block 4408e
            // 4408e mov rcx,r12
            rcx = r12;
            // 44091 mov rdx,rsi
            rdx = rsi;
            pc = 0x44094;
        }
        0x44094 => {
            // block 44094
            // 44094 mov rax,rdx
            rax = rdx;
            pc = 0x44097;
        }
        0x44097 => {
            // block 44097
            // 44097 cmp DWORD PTR [rax+0x1c],ebx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rbx as u32 as u64)).1;
            // 4409a jge 0x440a7
            if fl.ge() { pc = 0x440a7; continue; }
            pc = 0x4409c;
        }
        0x4409c => {
            // block 4409c
            // 4409c mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 440a0 test rax,rax
            fl = logic::<64>(rax & rax);
            // 440a3 jne 0x44097
            if fl.ne() { pc = 0x44097; continue; }
            pc = 0x440a5;
        }
        0x440a5 => {
            // block 440a5
            // 440a5 jmp 0x440b9
            pc = 0x440b9; continue;
        }
        0x440a7 => {
            // block 440a7
            // 440a7 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 440aa test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 440ad mov rcx,rax
            rcx = rax;
            // 440b0 jne 0x44094
            if fl.ne() { pc = 0x44094; continue; }
            pc = 0x440b2;
        }
        0x440b2 => {
            // block 440b2
            // 440b2 jmp 0x440bc
            pc = 0x440bc; continue;
        }
        0x440b4 => {
            // block 440b4
            // 440b4 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 440b7 jmp 0x440ce
            pc = 0x440ce; continue;
        }
        0x440b9 => {
            // block 440b9
            // 440b9 mov rax,rcx
            rax = rcx;
            pc = 0x440bc;
        }
        0x440bc => {
            // block 440bc
            // 440bc cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 440bf je 0x440ce
            if fl.e() { pc = 0x440ce; continue; }
            pc = 0x440c1;
        }
        0x440c1 => {
            // block 440c1
            // 440c1 cmp DWORD PTR [rax+0x1c],ebx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rbx as u32 as u64)).1;
            // 440c4 jg 0x440ce
            if fl.g() { pc = 0x440ce; continue; }
            pc = 0x440c6;
        }
        0x440c6 => {
            // block 440c6
            // 440c6 mov BYTE PTR [r15],0x1
            m.w8(r15, (0x1u64) as u8);
            // 440ca jmp 0x440ce
            pc = 0x440ce; continue;
        }
        0x440cc => {
            // block 440cc
            // 440cc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x440ce;
        }
        0x440ce => {
            // block 440ce
            // 440ce lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 440d2 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 440d7 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 440da add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 440de pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 440df pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 440e1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 440e3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 440e5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 440e6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x440e7 => {
            // block 440e7
            // 440e7 mov rbx,rax
            rbx = rax;
            // 440ea mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 440ee lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 440f2 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 440f7 mov rdi,rbx
            rdi = rbx;
            // 440fa call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44100 size=202
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044100<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44100;
    loop {
        match pc {
        0x44100 => {
            // block 44100
            // 44100 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44101 mov rbp,rsp
            rbp = rsp;
            // 44104 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44106 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44108 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4410a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4410b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4410f mov r12,rdx
            r12 = rdx;
            // 44112 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 44115 mov r14,rdi
            r14 = rdi;
            // 44118 lea rbx,[r12+0x8]
            rbx = r12.wrapping_add(0x8);
            // 4411d mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 44122 mov rdi,r12
            rdi = r12;
            // 44125 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4412a mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 44133 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 44137 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 44140 cmp r15d,0x7
            fl = sub::<32>((r15 as u32 as u64), 0x7u64).1;
            // 44144 ja 0x441ba
            if fl.a() { pc = 0x441ba; continue; }
            pc = 0x44146;
        }
        0x44146 => {
            // block 44146
            // 44146 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 4414d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 44150 mov r15d,r15d
            r15 = ((r15 as u32 as u64)) as u32 as u64;
            // 44153 lea rcx,[rip+0xbc4f6] # 0x100650
            rcx = m.env.data_va(85533);
            // 4415a mov esi,DWORD PTR [rcx+r15*4]
            rsi = (m.r32(rcx.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // 4415e lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 44162 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 44164 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44169 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4416c mov rdi,r14
            rdi = r14;
            // 4416f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44172 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44174 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 44179 je 0x441bc
            if fl.e() { pc = 0x441bc; continue; }
            pc = 0x4417b;
        }
        0x4417b => {
            // block 4417b
            // 4417b lea rax,[rip+0xbc4ee] # 0x100670
            rax = m.env.data_va(85565);
            // 44182 mov r15d,DWORD PTR [rax+r15*4]
            r15 = (m.r32(rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // 44186 mov DWORD PTR [rbp-0x28],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u32);
            // 4418d shl r15d,0x3
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x3u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 44191 lea r14,[rbp-0x28]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            pc = 0x44195;
        }
        0x44195 => {
            // block 44195
            // 44195 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 44198 lea ecx,[r15+rbx*1]
            rcx = (r15.wrapping_add(rbx)) as u32 as u64;
            // 4419c bt eax,ecx
            fl.cf = bit::<32>((rax as u32 as u64), (rcx as u32 as u64));
            // 4419f jae 0x441ac
            if fl.ae() { pc = 0x441ac; continue; }
            pc = 0x441a1;
        }
        0x441a1 => {
            // block 441a1
            // 441a1 mov rdi,r12
            rdi = r12;
            // 441a4 mov rsi,r14
            rsi = r14;
            // 441a7 call 0xebb52
            { let r = { let t = m.env.stub_va(0x31f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x441ac;
        }
        0x441ac => {
            // block 441ac
            // 441ac inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 441ae mov DWORD PTR [rbp-0x28],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), ((rbx as u32 as u64)) as u32);
            // 441b1 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 441b3 cmp ebx,0x8
            fl = sub::<32>((rbx as u32 as u64), 0x8u64).1;
            // 441b6 jb 0x44195
            if fl.b() { pc = 0x44195; continue; }
            pc = 0x441b8;
        }
        0x441b8 => {
            // block 441b8
            // 441b8 jmp 0x441bc
            pc = 0x441bc; continue;
        }
        0x441ba => {
            // block 441ba
            // 441ba xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x441bc;
        }
        0x441bc => {
            // block 441bc
            // 441bc add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 441c0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 441c1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 441c3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 441c5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 441c7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 441c8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3901 size=207
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3901<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3901;
    loop {
        match pc {
        0xc3901 => {
            // block c3901
            // c3901 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3902 mov rbp,rsp
            rbp = rsp;
            // c3905 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3907 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3909 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c390b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c390d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c390e sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // c3912 mov r14,rsi
            r14 = rsi;
            // c3915 mov rbx,rdi
            rbx = rdi;
            // c3918 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // c391b add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c391f cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // c3922 je 0xc39a9
            if fl.e() { pc = 0xc39a9; continue; }
            pc = 0xc3928;
        }
        0xc3928 => {
            // block c3928
            // c3928 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // c392c jmp 0xc3945
            pc = 0xc3945; continue;
        }
        0xc392e => {
            // block c392e
            // c392e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // c3933 mov rdi,rbx
            rdi = rbx;
            // c3936 lea rsi,[rip+0x66e26] # 0x12a763
            rsi = m.env.data_va(257840);
            // c393d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3942 mov r13,r12
            r13 = r12;
            pc = 0xc3945;
        }
        0xc3945 => {
            // block c3945
            // c3945 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c3949 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // c394e mov rdi,r15
            rdi = r15;
            // c3951 call 0x5f77c
            { let r = { let t = m.env.code_va(0x5f77c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3956 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // c395a test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // c395d jne 0xc3968
            if fl.ne() { pc = 0xc3968; continue; }
            pc = 0xc395f;
        }
        0xc395f => {
            // block c395f
            // c395f shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // c3962 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // c3966 jmp 0xc3970
            pc = 0xc3970; continue;
        }
        0xc3968 => {
            // block c3968
            // c3968 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c396c mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xc3970;
        }
        0xc3970 => {
            // block c3970
            // c3970 mov rdi,rbx
            rdi = rbx;
            // c3973 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3978 mov rdi,r15
            rdi = r15;
            // c397b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3980 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // c3984 test rax,rax
            fl = logic::<64>(rax & rax);
            // c3987 je 0xc3997
            if fl.e() { pc = 0xc3997; continue; }
            pc = 0xc3989;
        }
        0xc3989 => {
            // block c3989
            // c3989 mov r12,rax
            r12 = rax;
            // c398c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // c3990 test rax,rax
            fl = logic::<64>(rax & rax);
            // c3993 jne 0xc3989
            if fl.ne() { pc = 0xc3989; continue; }
            pc = 0xc3995;
        }
        0xc3995 => {
            // block c3995
            // c3995 jmp 0xc39a4
            pc = 0xc39a4; continue;
        }
        0xc3997 => {
            // block c3997
            // c3997 mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // c399b cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // c399f mov r13,r12
            r13 = r12;
            // c39a2 jne 0xc3997
            if fl.ne() { pc = 0xc3997; continue; }
            pc = 0xc39a4;
        }
        0xc39a4 => {
            // block c39a4
            // c39a4 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // c39a7 jne 0xc392e
            if fl.ne() { pc = 0xc392e; continue; }
            pc = 0xc39a9;
        }
        0xc39a9 => {
            // block c39a9
            // c39a9 mov rax,rbx
            rax = rbx;
            // c39ac add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // c39b0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c39b1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c39b3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c39b5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c39b7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c39b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c39ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc39bb => {
            // block c39bb
            // c39bb mov rbx,rax
            rbx = rax;
            // c39be lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // c39c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c39c7 mov rdi,rbx
            rdi = rbx;
            // c39ca call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7e5e size=210
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7e5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb7e5e;
    loop {
        match pc {
        0xb7e5e => {
            // block b7e5e
            // b7e5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7e5f mov rbp,rsp
            rbp = rsp;
            // b7e62 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7e64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7e65 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7e69 mov rbx,rdi
            rbx = rdi;
            // b7e6c mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // b7e73 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7e76 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7e7c xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b7e7f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7e81 je 0xb7f23
            if fl.e() { pc = 0xb7f23; continue; }
            pc = 0xb7e87;
        }
        0xb7e87 => {
            // block b7e87
            // b7e87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7e8a lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // b7e8e xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b7e91 mov esi,0x18
            rsi = (0x18u64) as u32 as u64;
            // b7e96 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b7e9b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b7e9e mov rdi,rbx
            rdi = rbx;
            // b7ea1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7ea4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7ea6 je 0xb7f23
            if fl.e() { pc = 0xb7f23; continue; }
            pc = 0xb7ea8;
        }
        0xb7ea8 => {
            // block b7ea8
            // b7ea8 mov eax,DWORD PTR [rbx+0x10]
            rax = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b7eab cmp eax,0x104782ff
            fl = sub::<32>((rax as u32 as u64), 0x104782ffu64).1;
            // b7eb0 jg 0xb7ed2
            if fl.g() { pc = 0xb7ed2; continue; }
            pc = 0xb7eb2;
        }
        0xb7eb2 => {
            // block b7eb2
            // b7eb2 cmp eax,0x1032294f
            fl = sub::<32>((rax as u32 as u64), 0x1032294fu64).1;
            // b7eb7 jg 0xb7ee9
            if fl.g() { pc = 0xb7ee9; continue; }
            pc = 0xb7eb9;
        }
        0xb7eb9 => {
            // block b7eb9
            // b7eb9 cmp eax,0x10266400
            fl = sub::<32>((rax as u32 as u64), 0x10266400u64).1;
            // b7ebe jne 0xb7f09
            if fl.ne() { pc = 0xb7f09; continue; }
            pc = 0xb7ec0;
        }
        0xb7ec0 => {
            // block b7ec0
            // b7ec0 mov al,BYTE PTR [rbp-0x11]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u8 as u64);
            // b7ec3 and al,0x8
            { let r = (rax as u8 as u64) & 0x8u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7ec5 shr al,0x3
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x3u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // b7ec8 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b7ecb lea r14d,[rax+rax*4+0x1]
            r14 = (rax.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1)) as u32 as u64;
            // b7ed0 jmp 0xb7f23
            pc = 0xb7f23; continue;
        }
        0xb7ed2 => {
            // block b7ed2
            // b7ed2 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // b7ed7 je 0xb7f10
            if fl.e() { pc = 0xb7f10; continue; }
            pc = 0xb7ed9;
        }
        0xb7ed9 => {
            // block b7ed9
            // b7ed9 cmp eax,0x10518400
            fl = sub::<32>((rax as u32 as u64), 0x10518400u64).1;
            // b7ede je 0xb7f10
            if fl.e() { pc = 0xb7f10; continue; }
            pc = 0xb7ee0;
        }
        0xb7ee0 => {
            // block b7ee0
            // b7ee0 cmp eax,0x10518450
            fl = sub::<32>((rax as u32 as u64), 0x10518450u64).1;
            // b7ee5 jne 0xb7f23
            if fl.ne() { pc = 0xb7f23; continue; }
            pc = 0xb7ee7;
        }
        0xb7ee7 => {
            // block b7ee7
            // b7ee7 jmp 0xb7f10
            pc = 0xb7f10; continue;
        }
        0xb7ee9 => {
            // block b7ee9
            // b7ee9 cmp eax,0x10322950
            fl = sub::<32>((rax as u32 as u64), 0x10322950u64).1;
            // b7eee je 0xb7f10
            if fl.e() { pc = 0xb7f10; continue; }
            pc = 0xb7ef0;
        }
        0xb7ef0 => {
            // block b7ef0
            // b7ef0 cmp eax,0x10352300
            fl = sub::<32>((rax as u32 as u64), 0x10352300u64).1;
            // b7ef5 jne 0xb7f23
            if fl.ne() { pc = 0xb7f23; continue; }
            pc = 0xb7ef7;
        }
        0xb7ef7 => {
            // block b7ef7
            // b7ef7 mov al,BYTE PTR [rbp-0x11]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u8 as u64);
            // b7efa and al,0x8
            { let r = (rax as u8 as u64) & 0x8u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7efc shr al,0x3
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x3u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // b7eff movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b7f02 lea r14d,[rax+rax*2+0x1]
            r14 = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x1)) as u32 as u64;
            // b7f07 jmp 0xb7f23
            pc = 0xb7f23; continue;
        }
        0xb7f09 => {
            // block b7f09
            // b7f09 cmp eax,0x10294700
            fl = sub::<32>((rax as u32 as u64), 0x10294700u64).1;
            // b7f0e jne 0xb7f23
            if fl.ne() { pc = 0xb7f23; continue; }
            pc = 0xb7f10;
        }
        0xb7f10 => {
            // block b7f10
            // b7f10 test BYTE PTR [rbp-0x11],0x8
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64 & 0x8u64);
            // b7f14 mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // b7f19 mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // b7f1f cmove r14d,eax
            if fl.e() { r14 = ((rax as u32 as u64)) as u32 as u64; } else { r14 = r14 as u32 as u64; }
            pc = 0xb7f23;
        }
        0xb7f23 => {
            // block b7f23
            // b7f23 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // b7f26 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7f2a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7f2b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7f2d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7f2e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x69246 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00069246<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x69246;
    loop {
        match pc {
        0x69246 => {
            // block 69246
            // 69246 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 69247 mov rbp,rsp
            rbp = rsp;
            // 6924a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6924c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6924e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 69250 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 69252 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 69253 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 69257 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 6925a mov r12,rsi
            r12 = rsi;
            // 6925d mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 69261 lea rax,[rdi+0x8]
            rax = rdi.wrapping_add(0x8);
            // 69265 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 69269 mov QWORD PTR [rdi+0x10],0x0
            m.w64(rdi.wrapping_add(0x10), (0x0u64) as u64);
            // 69271 mov QWORD PTR [rdi+0x8],0x0
            m.w64(rdi.wrapping_add(0x8), (0x0u64) as u64);
            // 69279 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 6927c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 69280 call 0x6911e
            { let r = { let t = m.env.code_va(0x6911e); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69285 mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 69289 lea r13,[rbp-0x38]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 6928d jmp 0x692d6
            pc = 0x692d6; continue;
        }
        0x6928f => {
            // block 6928f
            // 6928f lea rbx,[r14+0x20]
            rbx = r14.wrapping_add(0x20);
            // 69293 mov rdi,r12
            rdi = r12;
            // 69296 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 69299 mov rdx,rbx
            rdx = rbx;
            // 6929c call 0x68fea
            { let r = { let t = m.env.code_va(0x68fea); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 692a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 692a3 je 0x692b1
            if fl.e() { pc = 0x692b1; continue; }
            pc = 0x692a5;
        }
        0x692a5 => {
            // block 692a5
            // 692a5 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 692a9 mov rsi,rbx
            rsi = rbx;
            // 692ac call 0xebca8
            { let r = { let t = m.env.stub_va(0x475); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x692b1;
        }
        0x692b1 => {
            // block 692b1
            // 692b1 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 692b5 test rax,rax
            fl = logic::<64>(rax & rax);
            // 692b8 je 0x692c7
            if fl.e() { pc = 0x692c7; continue; }
            pc = 0x692ba;
        }
        0x692ba => {
            // block 692ba
            // 692ba mov r14,rax
            r14 = rax;
            // 692bd mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 692c0 test rax,rax
            fl = logic::<64>(rax & rax);
            // 692c3 jne 0x692ba
            if fl.ne() { pc = 0x692ba; continue; }
            pc = 0x692c5;
        }
        0x692c5 => {
            // block 692c5
            // 692c5 jmp 0x692d6
            pc = 0x692d6; continue;
        }
        0x692c7 => {
            // block 692c7
            // 692c7 mov rax,r14
            rax = r14;
            pc = 0x692ca;
        }
        0x692ca => {
            // block 692ca
            // 692ca mov r14,QWORD PTR [rax+0x10]
            r14 = m.r64(rax.wrapping_add(0x10)) as u64;
            // 692ce cmp QWORD PTR [r14],rax
            fl = sub::<64>(m.r64(r14) as u64, rax).1;
            // 692d1 mov rax,r14
            rax = r14;
            // 692d4 jne 0x692ca
            if fl.ne() { pc = 0x692ca; continue; }
            pc = 0x692d6;
        }
        0x692d6 => {
            // block 692d6
            // 692d6 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 692d9 jne 0x6928f
            if fl.ne() { pc = 0x6928f; continue; }
            pc = 0x692db;
        }
        0x692db => {
            // block 692db
            // 692db mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 692df lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 692e3 call 0xebcc0
            { let r = { let t = m.env.stub_va(0x48d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 692e8 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 692ec add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 692f0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 692f1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 692f3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 692f5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 692f7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 692f9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 692fa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x692fb => {
            // block 692fb
            // 692fb mov rbx,rax
            rbx = rax;
            // 692fe mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 69302 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 69306 call 0xebcc0
            { let r = { let t = m.env.stub_va(0x48d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6930b;
        }
        0x6930b => {
            // block 6930b
            // 6930b mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 6930f mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 69312 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 69316 call 0xebcc0
            { let r = { let t = m.env.stub_va(0x48d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6931b mov rdi,rbx
            rdi = rbx;
            // 6931e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69323 mov rbx,rax
            rbx = rax;
            // 69326 jmp 0x6930b
            pc = 0x6930b; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xec66 size=230
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ec66<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xec66;
    loop {
        match pc {
        0xec66 => {
            // block ec66
            // ec66 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ec67 mov rbp,rsp
            rbp = rsp;
            // ec6a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ec6c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ec6e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ec6f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ec70 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // ec73 mov r14,rsi
            r14 = rsi;
            // ec76 mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // ec7d mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // ec84 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // ec87 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // ec8b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // ec8d mov esi,0x77
            rsi = (0x77u64) as u32 as u64;
            // ec92 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ec97 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ec9a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ec9d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // ec9f je 0xed2c
            if fl.e() { pc = 0xed2c; continue; }
            pc = 0xeca5;
        }
        0xeca5 => {
            // block eca5
            // eca5 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // eca8 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // ecac jbe 0xecb2
            if fl.be() { pc = 0xecb2; continue; }
            pc = 0xecae;
        }
        0xecae => {
            // block ecae
            // ecae xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // ecb0 jmp 0xed2c
            pc = 0xed2c; continue;
        }
        0xecb2 => {
            // block ecb2
            // ecb2 mov ecx,0xffc0
            rcx = (0xffc0u64) as u32 as u64;
            // ecb7 and ecx,DWORD PTR [rbp-0x1c]
            { let r = (rcx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // ecba shr ecx,0x6
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x6u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // ecbd cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // ecc1 mulsd xmm0,QWORD PTR [rip+0xefc17] # 0xfe8e0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(77997)) as u128))).to_bits());
            // ecc9 mulsd xmm0,QWORD PTR [rip+0xefc17] # 0xfe8e8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(78005)) as u128))).to_bits());
            // ecd1 addsd xmm0,QWORD PTR [rip+0xefc17] # 0xfe8f0
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(78013)) as u128))).to_bits());
            // ecd9 lea rcx,[rip+0x5c] # 0xed3c
            rcx = m.env.code_va(0xed3c);
            // ece0 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // ece4 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // ece7 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xece9 => {
            // block ece9
            // ece9 mulsd xmm0,QWORD PTR [rip+0xefc0f] # 0xfe900
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(78029)) as u128))).to_bits());
            // ecf1 divsd xmm0,QWORD PTR [rip+0xefc0f] # 0xfe908
            x0 = set_lo64(x0, (f64_of(x0) / f64_of((m.r64(m.env.data_va(78037)) as u128))).to_bits());
            // ecf9 addsd xmm0,QWORD PTR [rip+0xefc0f] # 0xfe910
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(78045)) as u128))).to_bits());
            // ed01 jmp 0xed25
            pc = 0xed25; continue;
        }
        0xed03 => {
            // block ed03
            // ed03 addsd xmm0,QWORD PTR [rip+0xefbed] # 0xfe8f8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(78021)) as u128))).to_bits());
            // ed0b jmp 0xed25
            pc = 0xed25; continue;
        }
        0xed0d => {
            // block ed0d
            // ed0d addsd xmm0,QWORD PTR [rip+0xefbe3] # 0xfe8f8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(78021)) as u128))).to_bits());
            // ed15 mulsd xmm0,QWORD PTR [rip+0xefbe3] # 0xfe900
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(78029)) as u128))).to_bits());
            // ed1d divsd xmm0,QWORD PTR [rip+0xefbe3] # 0xfe908
            x0 = set_lo64(x0, (f64_of(x0) / f64_of((m.r64(m.env.data_va(78037)) as u128))).to_bits());
            pc = 0xed25;
        }
        0xed25 => {
            // block ed25
            // ed25 movsd QWORD PTR [r14],xmm0
            m.w64(r14, x0 as u64);
            // ed2a mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xed2c;
        }
        0xed2c => {
            // block ed2c
            // ed2c mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // ed2e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ed32 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ed33 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ed35 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ed37 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ed38 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5d10 size=236
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005d10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5d10;
    loop {
        match pc {
        0x5d10 => {
            // block 5d10
            // 5d10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5d11 mov rbp,rsp
            rbp = rsp;
            // 5d14 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5d16 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5d18 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5d1a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5d1b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5d1f mov r14,rdx
            r14 = rdx;
            // 5d22 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5d25 mov rbx,rdi
            rbx = rdi;
            // 5d28 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d2b call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d31 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d33 jne 0x5ddf
            if fl.ne() { pc = 0x5ddf; continue; }
            pc = 0x5d39;
        }
        0x5d39 => {
            // block 5d39
            // 5d39 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d3c lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 5d40 mov rdi,rbx
            rdi = rbx;
            // 5d43 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 5d46 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d4c mov r12d,DWORD PTR [rbp-0x24]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 5d50 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d53 mov rdi,rbx
            rdi = rbx;
            // 5d56 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d5c cmp r12d,eax
            fl = sub::<32>((r12 as u32 as u64), (rax as u32 as u64)).1;
            // 5d5f jbe 0x5d68
            if fl.be() { pc = 0x5d68; continue; }
            pc = 0x5d61;
        }
        0x5d61 => {
            // block 5d61
            // 5d61 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            pc = 0x5d68;
        }
        0x5d68 => {
            // block 5d68
            // 5d68 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d6b mov rdi,rbx
            rdi = rbx;
            // 5d6e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d74 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5d76 call 0xcdd26
            { let r = { let t = m.env.code_va(0xcdd26); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d7b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d7d je 0x5db7
            if fl.e() { pc = 0x5db7; continue; }
            pc = 0x5d7f;
        }
        0x5d7f => {
            // block 5d7f
            // 5d7f mov rax,QWORD PTR [rbx+0x210]
            rax = m.r64(rbx.wrapping_add(0x210)) as u64;
            // 5d86 test rax,rax
            fl = logic::<64>(rax & rax);
            // 5d89 jne 0x5da2
            if fl.ne() { pc = 0x5da2; continue; }
            pc = 0x5d8b;
        }
        0x5d8b => {
            // block 5d8b
            // 5d8b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d8e mov rdi,rbx
            rdi = rbx;
            // 5d91 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d97 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d99 je 0x5ddf
            if fl.e() { pc = 0x5ddf; continue; }
            pc = 0x5d9b;
        }
        0x5d9b => {
            // block 5d9b
            // 5d9b mov rax,QWORD PTR [rbx+0x210]
            rax = m.r64(rbx.wrapping_add(0x210)) as u64;
            pc = 0x5da2;
        }
        0x5da2 => {
            // block 5da2
            // 5da2 mov ecx,DWORD PTR [rbx+0x24c]
            rcx = (m.r32(rbx.wrapping_add(0x24c)) as u64) as u32 as u64;
            // 5da8 imul ecx,DWORD PTR [rbp-0x24]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // 5dac and ecx,0xfffffffc
            { let r = (rcx as u32 as u64) & 0xfffffffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5daf add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 5db2 mov QWORD PTR [r14],rcx
            m.w64(r14, (rcx) as u64);
            // 5db5 jmp 0x5ded
            pc = 0x5ded; continue;
        }
        0x5db7 => {
            // block 5db7
            // 5db7 cmp QWORD PTR [rbx+0x218],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x218)) as u64, 0x0u64).1;
            // 5dbf jne 0x5dd1
            if fl.ne() { pc = 0x5dd1; continue; }
            pc = 0x5dc1;
        }
        0x5dc1 => {
            // block 5dc1
            // 5dc1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5dc4 mov rdi,rbx
            rdi = rbx;
            // 5dc7 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5dcd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5dcf je 0x5ddf
            if fl.e() { pc = 0x5ddf; continue; }
            pc = 0x5dd1;
        }
        0x5dd1 => {
            // block 5dd1
            // 5dd1 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 5dd4 je 0x5de3
            if fl.e() { pc = 0x5de3; continue; }
            pc = 0x5dd6;
        }
        0x5dd6 => {
            // block 5dd6
            // 5dd6 mov rax,QWORD PTR [rbx+0x220]
            rax = m.r64(rbx.wrapping_add(0x220)) as u64;
            // 5ddd jmp 0x5dea
            pc = 0x5dea; continue;
        }
        0x5ddf => {
            // block 5ddf
            // 5ddf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5de1 jmp 0x5def
            pc = 0x5def; continue;
        }
        0x5de3 => {
            // block 5de3
            // 5de3 mov rax,QWORD PTR [rbx+0x218]
            rax = m.r64(rbx.wrapping_add(0x218)) as u64;
            pc = 0x5dea;
        }
        0x5dea => {
            // block 5dea
            // 5dea mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            pc = 0x5ded;
        }
        0x5ded => {
            // block 5ded
            // 5ded mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x5def;
        }
        0x5def => {
            // block 5def
            // 5def add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5df3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5df4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5df6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5df8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5dfa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5dfb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8e79 size=241
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8e79<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb8e79;
    loop {
        match pc {
        0xb8e79 => {
            // block b8e79
            // b8e79 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8e7a mov rbp,rsp
            rbp = rsp;
            // b8e7d push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b8e7f push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8e81 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b8e83 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b8e85 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8e86 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b8e8a mov r12,rsi
            r12 = rsi;
            // b8e8d mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // b8e91 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8e95 lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // b8e9a cmp rcx,r13
            fl = sub::<64>(rcx, r13).1;
            // b8e9d je 0xb8f43
            if fl.e() { pc = 0xb8f43; continue; }
            pc = 0xb8ea3;
        }
        0xb8ea3 => {
            // block b8ea3
            // b8ea3 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8ea7 mov r14,rcx
            r14 = rcx;
            // b8eaa jmp 0xb8eb3
            pc = 0xb8eb3; continue;
        }
        0xb8eac => {
            // block b8eac
            // b8eac mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8eb0 mov r14,rax
            r14 = rax;
            pc = 0xb8eb3;
        }
        0xb8eb3 => {
            // block b8eb3
            // b8eb3 cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // b8eb6 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // b8eb9 lea rsi,[rip+0x7fe08] # 0x138cc8
            rsi = m.env.data_va(316565);
            // b8ec0 lea rcx,[rip+0x715d6] # 0x12a49d
            rcx = m.env.data_va(257130);
            // b8ec7 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b8ecb movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // b8ece add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // b8ed1 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b8ed5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8eda mov rbx,rax
            rbx = rax;
            // b8edd mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b8ee1 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b8ee6 mov rdi,r15
            rdi = r15;
            // b8ee9 call 0x60ec3
            { let r = { let t = m.env.code_va(0x60ec3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8eee movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // b8ef2 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b8ef5 jne 0xb8f00
            if fl.ne() { pc = 0xb8f00; continue; }
            pc = 0xb8ef7;
        }
        0xb8ef7 => {
            // block b8ef7
            // b8ef7 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b8efa lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // b8efe jmp 0xb8f08
            pc = 0xb8f08; continue;
        }
        0xb8f00 => {
            // block b8f00
            // b8f00 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b8f04 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xb8f08;
        }
        0xb8f08 => {
            // block b8f08
            // b8f08 mov rdi,rbx
            rdi = rbx;
            // b8f0b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8f10 mov rdi,r15
            rdi = r15;
            // b8f13 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8f18 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // b8f1c test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b8f1f je 0xb8f2e
            if fl.e() { pc = 0xb8f2e; continue; }
            pc = 0xb8f21;
        }
        0xb8f21 => {
            // block b8f21
            // b8f21 mov rax,rcx
            rax = rcx;
            // b8f24 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b8f27 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b8f2a jne 0xb8f21
            if fl.ne() { pc = 0xb8f21; continue; }
            pc = 0xb8f2c;
        }
        0xb8f2c => {
            // block b8f2c
            // b8f2c jmp 0xb8f3a
            pc = 0xb8f3a; continue;
        }
        0xb8f2e => {
            // block b8f2e
            // b8f2e mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // b8f32 cmp QWORD PTR [rax],r14
            fl = sub::<64>(m.r64(rax) as u64, r14).1;
            // b8f35 mov r14,rax
            r14 = rax;
            // b8f38 jne 0xb8f2e
            if fl.ne() { pc = 0xb8f2e; continue; }
            pc = 0xb8f3a;
        }
        0xb8f3a => {
            // block b8f3a
            // b8f3a cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // b8f3d jne 0xb8eac
            if fl.ne() { pc = 0xb8eac; continue; }
            pc = 0xb8f43;
        }
        0xb8f43 => {
            // block b8f43
            // b8f43 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b8f47 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b8f4b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8f4c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b8f4e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b8f50 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8f52 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b8f54 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8f55 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb8f56 => {
            // block b8f56
            // b8f56 mov rbx,rax
            rbx = rax;
            // b8f59 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8f5d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8f62 mov rdi,rbx
            rdi = rbx;
            // b8f65 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
