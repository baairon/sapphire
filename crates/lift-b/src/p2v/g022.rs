//! Mechanically lifted functions (scripts/lift_run.py), module g022. Each one is checked against the original
//! in the world harness (tests/world_g022.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x42360 size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042360<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42360;
    loop {
        match pc {
        0x42360 => {
            // block 42360
            // 42360 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42361 mov rbp,rsp
            rbp = rsp;
            // 42364 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42365 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42366 mov rbx,rsi
            rbx = rsi;
            // 42369 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 42370 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 42373 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 42375 lea rdx,[rip+0xbe0f4] # 0x100470
            rdx = m.env.data_va(85053);
            // 4237c mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4237f lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 42383 mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 42388 mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 4238e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42391 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42393 je 0x4239a
            if fl.e() { pc = 0x4239a; continue; }
            pc = 0x42395;
        }
        0x42395 => {
            // block 42395
            // 42395 mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 42398 mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            pc = 0x4239a;
        }
        0x4239a => {
            // block 4239a
            // 4239a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4239e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4239f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 423a0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5c4c size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5c4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd5c4c;
    loop {
        match pc {
        0xd5c4c => {
            // block d5c4c
            // d5c4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5c4d mov rbp,rsp
            rbp = rsp;
            // d5c50 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5c51 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d5c52 mov rbx,rdi
            rbx = rdi;
            // d5c55 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5c58 call QWORD PTR [rax+0x100]
            { let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5c5e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5c60 je 0xd5c85
            if fl.e() { pc = 0xd5c85; continue; }
            pc = 0xd5c62;
        }
        0xd5c62 => {
            // block d5c62
            // d5c62 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5c65 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d5c69 mov esi,0x40005
            rsi = (0x40005u64) as u32 as u64;
            // d5c6e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5c73 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5c76 mov rdi,rbx
            rdi = rbx;
            // d5c79 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5c7c cmp DWORD PTR [rbp-0xc],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x1u64).1;
            // d5c80 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d5c83 jmp 0xd5c87
            pc = 0xd5c87; continue;
        }
        0xd5c85 => {
            // block d5c85
            // d5c85 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd5c87;
        }
        0xd5c87 => {
            // block d5c87
            // d5c87 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d5c8b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5c8c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5c8d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0bee size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0bee<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0bee;
    loop {
        match pc {
        0xe0bee => {
            // block e0bee
            // e0bee push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0bef mov rbp,rsp
            rbp = rsp;
            // e0bf2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0bf4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0bf5 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0bf9 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e0bfb mov r14,rdi
            r14 = rdi;
            // e0bfe mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0c01 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0c05 call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0c0b mov eax,0xffffff0f
            rax = (0xffffff0fu64) as u32 as u64;
            // e0c10 and eax,DWORD PTR [rbp-0x14]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0c13 shl ebx,0x4
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x4u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0c16 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0c18 mov DWORD PTR [rbp-0x14],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rbx as u32 as u64)) as u32);
            // e0c1b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0c1e mov rdi,r14
            rdi = r14;
            // e0c21 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e0c23 call QWORD PTR [rax+0x1ac0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0c29 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0c2d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0c2e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0c30 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0c31 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0e4a size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0e4a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0e4a;
    loop {
        match pc {
        0xe0e4a => {
            // block e0e4a
            // e0e4a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0e4b mov rbp,rsp
            rbp = rsp;
            // e0e4e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0e50 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0e51 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e55 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e0e57 mov r14,rdi
            r14 = rdi;
            // e0e5a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0e5d lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0e61 call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0e67 mov eax,0x3fffffff
            rax = (0x3fffffffu64) as u32 as u64;
            // e0e6c and eax,DWORD PTR [rbp-0x14]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0e6f shl ebx,0x1e
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x1eu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0e72 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0e74 mov DWORD PTR [rbp-0x14],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rbx as u32 as u64)) as u32);
            // e0e77 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0e7a mov rdi,r14
            rdi = r14;
            // e0e7d mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e0e7f call QWORD PTR [rax+0x1ad0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0e85 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e89 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0e8a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0e8c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0e8d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6aa4 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006aa4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6aa4;
    loop {
        match pc {
        0x6aa4 => {
            // block 6aa4
            // 6aa4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6aa5 mov rbp,rsp
            rbp = rsp;
            // 6aa8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6aaa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6aab sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 6aaf mov r14,rsi
            r14 = rsi;
            // 6ab2 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 6ab9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 6abc xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 6abe lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 6ac2 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 6ac4 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 6ac9 mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            // 6acf call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6ad2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6ad4 je 0x6add
            if fl.e() { pc = 0x6add; continue; }
            pc = 0x6ad6;
        }
        0x6ad6 => {
            // block 6ad6
            // 6ad6 cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // 6ada setne bl
            rbx = (rbx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x6add;
        }
        0x6add => {
            // block 6add
            // 6add mov BYTE PTR [r14],bl
            m.w8(r14, ((rbx as u8 as u64)) as u8);
            // 6ae0 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 6ae4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6ae5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6ae7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6ae8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x423e8 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000423e8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x423e8;
    loop {
        match pc {
        0x423e8 => {
            // block 423e8
            // 423e8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 423e9 mov rbp,rsp
            rbp = rsp;
            // 423ec push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 423ed push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 423ee mov rbx,rsi
            rbx = rsi;
            // 423f1 mov DWORD PTR [rbp-0xc],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x2u64) as u32);
            // 423f8 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 423fb lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 423ff call QWORD PTR [rax+0x6a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x6a8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42405 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42407 je 0x42419
            if fl.e() { pc = 0x42419; continue; }
            pc = 0x42409;
        }
        0x42409 => {
            // block 42409
            // 42409 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 4240c cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 4240f jne 0x4241d
            if fl.ne() { pc = 0x4241d; continue; }
            pc = 0x42411;
        }
        0x42411 => {
            // block 42411
            // 42411 mov DWORD PTR [rbx],0x400000
            m.w32(rbx, (0x400000u64) as u32);
            // 42417 jmp 0x42425
            pc = 0x42425; continue;
        }
        0x42419 => {
            // block 42419
            // 42419 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4241b jmp 0x42427
            pc = 0x42427; continue;
        }
        0x4241d => {
            // block 4241d
            // 4241d test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4241f mov DWORD PTR [rbx],0x100000
            m.w32(rbx, (0x100000u64) as u32);
            pc = 0x42425;
        }
        0x42425 => {
            // block 42425
            // 42425 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x42427;
        }
        0x42427 => {
            // block 42427
            // 42427 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4242b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4242c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4242d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43a98 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043a98<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43a98;
    loop {
        match pc {
        0x43a98 => {
            // block 43a98
            // 43a98 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43a99 mov rbp,rsp
            rbp = rsp;
            // 43a9c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43a9d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43a9e mov rbx,rdx
            rbx = rdx;
            // 43aa1 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43aa4 jle 0x43aaa
            if fl.le() { pc = 0x43aaa; continue; }
            pc = 0x43aa6;
        }
        0x43aa6 => {
            // block 43aa6
            // 43aa6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43aa8 jmp 0x43ad6
            pc = 0x43ad6; continue;
        }
        0x43aaa => {
            // block 43aaa
            // 43aaa mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 43ab1 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43ab4 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 43ab6 lea rdx,[rip+0xbc903] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 43abd mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43ac0 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 43ac4 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 43ac9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 43acc call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43acf cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 43ad3 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x43ad6;
        }
        0x43ad6 => {
            // block 43ad6
            // 43ad6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43ada pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43adb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43adc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x438bc size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000438bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x438bc;
    loop {
        match pc {
        0x438bc => {
            // block 438bc
            // 438bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 438bd mov rbp,rsp
            rbp = rsp;
            // 438c0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 438c1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 438c2 mov rbx,rdx
            rbx = rdx;
            // 438c5 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 438c8 jle 0x438ce
            if fl.le() { pc = 0x438ce; continue; }
            pc = 0x438ca;
        }
        0x438ca => {
            // block 438ca
            // 438ca xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 438cc jmp 0x438fd
            pc = 0x438fd; continue;
        }
        0x438ce => {
            // block 438ce
            // 438ce mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 438d5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 438d8 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 438da lea rdx,[rip+0xbcadf] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 438e1 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 438e4 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 438e8 mov ecx,0x200
            rcx = (0x200u64) as u32 as u64;
            // 438ed mov r8d,0x9
            r8 = (0x9u64) as u32 as u64;
            // 438f3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 438f6 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 438fa setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x438fd;
        }
        0x438fd => {
            // block 438fd
            // 438fd add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43901 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43902 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43903 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x439ac size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000439ac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x439ac;
    loop {
        match pc {
        0x439ac => {
            // block 439ac
            // 439ac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 439ad mov rbp,rsp
            rbp = rsp;
            // 439b0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 439b1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 439b2 mov rbx,rdx
            rbx = rdx;
            // 439b5 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 439b8 jle 0x439be
            if fl.le() { pc = 0x439be; continue; }
            pc = 0x439ba;
        }
        0x439ba => {
            // block 439ba
            // 439ba xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 439bc jmp 0x439ed
            pc = 0x439ed; continue;
        }
        0x439be => {
            // block 439be
            // 439be mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 439c5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 439c8 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 439ca lea rdx,[rip+0xbc9ef] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 439d1 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 439d4 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 439d8 mov ecx,0x800
            rcx = (0x800u64) as u32 as u64;
            // 439dd mov r8d,0xb
            r8 = (0xbu64) as u32 as u64;
            // 439e3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 439e6 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 439ea setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x439ed;
        }
        0x439ed => {
            // block 439ed
            // 439ed add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 439f1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 439f2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 439f3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x298c size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000298c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x298c;
    loop {
        match pc {
        0x298c => {
            // block 298c
            // 298c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 298d mov rbp,rsp
            rbp = rsp;
            // 2990 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2991 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2992 mov rbx,rdx
            rbx = rdx;
            // 2995 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 299c mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 299f mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 29a1 lea rdx,[rip+0xfc6d8] # 0xff080
            rdx = m.env.data_va(79949);
            // 29a8 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 29ab lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 29af mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // 29b4 mov r8d,0x7
            r8 = (0x7u64) as u32 as u64;
            // 29ba call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29bd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 29bf je 0x29cc
            if fl.e() { pc = 0x29cc; continue; }
            pc = 0x29c1;
        }
        0x29c1 => {
            // block 29c1
            // 29c1 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 29c5 sete BYTE PTR [rbx]
            m.w8(rbx, ((fl.e() as u64)) as u8);
            // 29c8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 29ca jmp 0x29ce
            pc = 0x29ce; continue;
        }
        0x29cc => {
            // block 29cc
            // 29cc xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x29ce;
        }
        0x29ce => {
            // block 29ce
            // 29ce add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 29d2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29d3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29d4 ret
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // e0da7 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // e0daa jne 0xe0db0
            if fl.ne() { pc = 0xe0db0; continue; }
            pc = 0xe0dac;
        }
        0xe0dac => {
            // block e0dac
            // e0dac xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0dae jmp 0xe0dbd
            pc = 0xe0dbd; continue;
        }
        0xe0db0 => {
            // block e0db0
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

/// parity: P2v @0x44b10 size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044b10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44b10;
    loop {
        match pc {
        0x44b10 => {
            // block 44b10
            // 44b10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44b11 mov rbp,rsp
            rbp = rsp;
            // 44b14 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44b16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44b17 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44b1b mov r14,rsi
            r14 = rsi;
            // 44b1e mov rbx,rdi
            rbx = rdi;
            // 44b21 lea rdx,[r14+0x1c]
            rdx = r14.wrapping_add(0x1c);
            // 44b25 lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 44b29 call 0xebb46
            { let r = { let t = m.env.stub_va(0x313); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44b2e mov rcx,rax
            rcx = rax;
            // 44b31 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 44b34 test rax,rax
            fl = logic::<64>(rax & rax);
            // 44b37 je 0x44b3d
            if fl.e() { pc = 0x44b3d; continue; }
            pc = 0x44b39;
        }
        0x44b39 => {
            // block 44b39
            // 44b39 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 44b3b jmp 0x44b54
            pc = 0x44b54; continue;
        }
        0x44b3d => {
            // block 44b3d
            // 44b3d mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 44b41 mov rdi,rbx
            rdi = rbx;
            // 44b44 mov rdx,rcx
            rdx = rcx;
            // 44b47 mov rcx,r14
            rcx = r14;
            // 44b4a call 0xebb5e
            { let r = { let t = m.env.stub_va(0x32b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44b4f mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            // 44b51 mov rax,r14
            rax = r14;
            pc = 0x44b54;
        }
        0x44b54 => {
            // block 44b54
            // 44b54 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44b58 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44b59 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44b5b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44b5c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb51e0 size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b51e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb51e0;
    loop {
        match pc {
        0xb51e0 => {
            // block b51e0
            // b51e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b51e1 mov rbp,rsp
            rbp = rsp;
            // b51e4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b51e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b51e7 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b51eb mov r14,rsi
            r14 = rsi;
            // b51ee mov rbx,rdi
            rbx = rdi;
            // b51f1 lea rdx,[r14+0x20]
            rdx = r14.wrapping_add(0x20);
            // b51f5 lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // b51f9 call 0xebc4e
            { let r = { let t = m.env.stub_va(0x41b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b51fe mov rcx,rax
            rcx = rax;
            // b5201 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // b5204 test rax,rax
            fl = logic::<64>(rax & rax);
            // b5207 je 0xb520d
            if fl.e() { pc = 0xb520d; continue; }
            pc = 0xb5209;
        }
        0xb5209 => {
            // block b5209
            // b5209 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b520b jmp 0xb5224
            pc = 0xb5224; continue;
        }
        0xb520d => {
            // block b520d
            // b520d mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b5211 mov rdi,rbx
            rdi = rbx;
            // b5214 mov rdx,rcx
            rdx = rcx;
            // b5217 mov rcx,r14
            rcx = r14;
            // b521a call 0xebc5a
            { let r = { let t = m.env.stub_va(0x427); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b521f mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            // b5221 mov rax,r14
            rax = r14;
            pc = 0xb5224;
        }
        0xb5224 => {
            // block b5224
            // b5224 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b5228 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5229 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b522b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b522c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe468 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e468<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe468;
    loop {
        match pc {
        0xe468 => {
            // block e468
            // e468 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e469 mov rbp,rsp
            rbp = rsp;
            // e46c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e46e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e46f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e473 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e476 mov rbx,rdi
            rbx = rdi;
            // e479 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e47c call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e482 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e484 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e486 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e488 jne 0xe4af
            if fl.ne() { pc = 0xe4af; continue; }
            pc = 0xe48a;
        }
        0xe48a => {
            // block e48a
            // e48a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e48d mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // e490 lea rdx,[rip+0xf1269] # 0xff700
            rdx = m.env.data_va(81613);
            // e497 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e49a lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e49e mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // e4a3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4a6 mov rdi,rbx
            rdi = rbx;
            // e4a9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4ac mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            pc = 0xe4af;
        }
        0xe4af => {
            // block e4af
            // e4af add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e4b3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4b4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4b6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4b7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43b12 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043b12<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43b12;
    loop {
        match pc {
        0x43b12 => {
            // block 43b12
            // 43b12 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43b13 mov rbp,rsp
            rbp = rsp;
            // 43b16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43b17 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43b18 mov rbx,rdx
            rbx = rdx;
            // 43b1b mov BYTE PTR [rbx],0x0
            m.w8(rbx, (0x0u64) as u8);
            // 43b1e cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43b21 jle 0x43b27
            if fl.le() { pc = 0x43b27; continue; }
            pc = 0x43b23;
        }
        0x43b23 => {
            // block 43b23
            // 43b23 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43b25 jmp 0x43b5b
            pc = 0x43b5b; continue;
        }
        0x43b27 => {
            // block 43b27
            // 43b27 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 43b2e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43b31 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 43b33 lea rcx,[rip+0xbca86] # 0x1005c0
            rcx = m.env.data_va(85389);
            // 43b3a mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43b3d lea rsi,[rip+0xbcaac] # 0x1005f0
            rsi = m.env.data_va(85437);
            // 43b44 mov r8d,DWORD PTR [rsi+rdx*4]
            r8 = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43b48 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 43b4c mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 43b51 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43b54 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 43b58 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x43b5b;
        }
        0x43b5b => {
            // block 43b5b
            // 43b5b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43b5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43b60 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43b61 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42080 size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042080<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42080;
    loop {
        match pc {
        0x42080 => {
            // block 42080
            // 42080 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42081 mov rbp,rsp
            rbp = rsp;
            // 42084 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42085 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42086 mov rbx,rsi
            rbx = rsi;
            // 42089 mov DWORD PTR [rbx],0x2
            m.w32(rbx, (0x2u64) as u32);
            // 4208f cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // 42092 ja 0x420c9
            if fl.a() { pc = 0x420c9; continue; }
            pc = 0x42094;
        }
        0x42094 => {
            // block 42094
            // 42094 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 4209b mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 4209e mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 420a0 lea rdx,[rip+0xbe319] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 420a7 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 420aa lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 420ae mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 420b3 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 420b9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 420bc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 420be je 0x420c9
            if fl.e() { pc = 0x420c9; continue; }
            pc = 0x420c0;
        }
        0x420c0 => {
            // block 420c0
            // 420c0 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 420c3 mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // 420c5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 420c7 jmp 0x420cb
            pc = 0x420cb; continue;
        }
        0x420c9 => {
            // block 420c9
            // 420c9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x420cb;
        }
        0x420cb => {
            // block 420cb
            // 420cb add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 420cf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 420d0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 420d1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x432dc size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000432dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x432dc;
    loop {
        match pc {
        0x432dc => {
            // block 432dc
            // 432dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 432dd mov rbp,rsp
            rbp = rsp;
            // 432e0 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 432e4 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 432e7 jle 0x432ed
            if fl.le() { pc = 0x432ed; continue; }
            pc = 0x432e9;
        }
        0x432e9 => {
            // block 432e9
            // 432e9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 432eb jmp 0x43327
            pc = 0x43327; continue;
        }
        0x432ed => {
            // block 432ed
            // 432ed mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // 432f4 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 432f6 mov eax,0xffff
            rax = (0xffffu64) as u32 as u64;
            // 432fb mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // 43300 cmove ecx,eax
            if fl.e() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 43303 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 43306 movzx r8d,al
            r8 = ((rax as u8 as u64)) as u32 as u64;
            // 4330a shl r8d,0x4
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x4u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // 4330e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43311 lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // 43315 mov esi,0x90f
            rsi = (0x90fu64) as u32 as u64;
            // 4331a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4331d xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4331f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43321 cmovne ecx,DWORD PTR [rbp-0x4]
            if fl.ne() { rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 43325 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x43327;
        }
        0x43327 => {
            // block 43327
            // 43327 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4332b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4332c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5b4 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e5b4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe5b4;
    loop {
        match pc {
        0xe5b4 => {
            // block e5b4
            // e5b4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5b5 mov rbp,rsp
            rbp = rsp;
            // e5b8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e5ba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e5bb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e5bf mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e5c2 mov rbx,rdi
            rbx = rdi;
            // e5c5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e5c8 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e5ce mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e5d0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5d2 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e5d4 jne 0xe5fe
            if fl.ne() { pc = 0xe5fe; continue; }
            pc = 0xe5d6;
        }
        0xe5d6 => {
            // block e5d6
            // e5d6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e5d9 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // e5dc lea rdx,[rip+0xf114d] # 0xff730
            rdx = m.env.data_va(81661);
            // e5e3 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e5e6 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e5ea mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // e5ef mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // e5f5 mov rdi,rbx
            rdi = rbx;
            // e5f8 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e5fb mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            pc = 0xe5fe;
        }
        0xe5fe => {
            // block e5fe
            // e5fe add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e602 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e603 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e605 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e606 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2f90 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2f90<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2f90;
    loop {
        match pc {
        0xe2f90 => {
            // block e2f90
            // e2f90 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2f91 mov rbp,rsp
            rbp = rsp;
            // e2f94 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2f96 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2f97 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2f9b mov r14,rcx
            r14 = rcx;
            // e2f9e mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e2fa0 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e2fa3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2fa6 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // e2faa test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e2fac je 0xe2fb9
            if fl.e() { pc = 0xe2fb9; continue; }
            pc = 0xe2fae;
        }
        0xe2fae => {
            // block e2fae
            // e2fae lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e2fb2 mov esi,0x4001c
            rsi = (0x4001cu64) as u32 as u64;
            // e2fb7 jmp 0xe2fc2
            pc = 0xe2fc2; continue;
        }
        0xe2fb9 => {
            // block e2fb9
            // e2fb9 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e2fbd mov esi,0x4001b
            rsi = (0x4001bu64) as u32 as u64;
            pc = 0xe2fc2;
        }
        0xe2fc2 => {
            // block e2fc2
            // e2fc2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2fc7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2fca call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2fcc shl ebx,0x3
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x3u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e2fcf mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // e2fd2 mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2fd4 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e2fd6 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e2fd9 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e2fdb add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2fdf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2fe0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2fe2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2fe3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1fa2 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001fa2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1fa2;
    loop {
        match pc {
        0x1fa2 => {
            // block 1fa2
            // 1fa2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1fa3 mov rbp,rsp
            rbp = rsp;
            // 1fa6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1fa8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1fa9 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 1fad mov r14,rdx
            r14 = rdx;
            // 1fb0 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 1fb3 ja 0x1fea
            if fl.a() { pc = 0x1fea; continue; }
            pc = 0x1fb5;
        }
        0x1fb5 => {
            // block 1fb5
            // 1fb5 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 1fbc mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 1fbe lea rcx,[rip+0xfd0bb] # 0xff080
            rcx = m.env.data_va(79949);
            // 1fc5 mov esi,DWORD PTR [rcx+rax*4]
            rsi = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 1fc8 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1fcb lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 1fcf xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1fd1 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 1fd6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 1fd9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1fdc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1fde je 0x1fec
            if fl.e() { pc = 0x1fec; continue; }
            pc = 0x1fe0;
        }
        0x1fe0 => {
            // block 1fe0
            // 1fe0 mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 1fe3 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 1fe6 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // 1fe8 jmp 0x1fec
            pc = 0x1fec; continue;
        }
        0x1fea => {
            // block 1fea
            // 1fea xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x1fec;
        }
        0x1fec => {
            // block 1fec
            // 1fec mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 1fee add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 1ff2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1ff3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1ff5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1ff6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe3b8 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e3b8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe3b8;
    loop {
        match pc {
        0xe3b8 => {
            // block e3b8
            // e3b8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e3b9 mov rbp,rsp
            rbp = rsp;
            // e3bc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e3be push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e3bf sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e3c3 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e3c6 mov rbx,rdi
            rbx = rdi;
            // e3c9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e3cc call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3d2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e3d4 je 0xe3da
            if fl.e() { pc = 0xe3da; continue; }
            pc = 0xe3d6;
        }
        0xe3d6 => {
            // block e3d6
            // e3d6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e3d8 jmp 0xe406
            pc = 0xe406; continue;
        }
        0xe3da => {
            // block e3da
            // e3da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e3dd mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // e3e0 lea rdx,[rip+0xf1319] # 0xff700
            rdx = m.env.data_va(81613);
            // e3e7 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e3ea lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e3ee mov ecx,0x1000000
            rcx = (0x1000000u64) as u32 as u64;
            // e3f3 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // e3f9 mov rdi,rbx
            rdi = rbx;
            // e3fc call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3ff cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // e403 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0xe406;
        }
        0xe406 => {
            // block e406
            // e406 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e40a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e40b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e40d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e40e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe50c size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e50c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe50c;
    loop {
        match pc {
        0xe50c => {
            // block e50c
            // e50c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e50d mov rbp,rsp
            rbp = rsp;
            // e510 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e512 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e513 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e517 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e51a mov rbx,rdi
            rbx = rdi;
            // e51d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e520 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e526 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e528 je 0xe52e
            if fl.e() { pc = 0xe52e; continue; }
            pc = 0xe52a;
        }
        0xe52a => {
            // block e52a
            // e52a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e52c jmp 0xe55a
            pc = 0xe55a; continue;
        }
        0xe52e => {
            // block e52e
            // e52e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e531 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // e534 lea rdx,[rip+0xf11c5] # 0xff700
            rdx = m.env.data_va(81613);
            // e53b mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e53e lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e542 mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // e547 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // e54d mov rdi,rbx
            rdi = rbx;
            // e550 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e553 cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // e557 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0xe55a;
        }
        0xe55a => {
            // block e55a
            // e55a add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e55e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e55f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e561 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e562 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8328 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d8328<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd8328;
    loop {
        match pc {
        0xd8328 => {
            // block d8328
            // d8328 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8329 mov rbp,rsp
            rbp = rsp;
            // d832c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d832e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d832f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d8333 mov rbx,rdi
            rbx = rdi;
            // d8336 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8339 lea r14,[rbp-0x14]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // d833d mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // d8342 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8347 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d834a mov rdx,r14
            rdx = r14;
            // d834d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8350 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            pc = 0xd8357;
        }
        0xd8357 => {
            // block d8357
            // d8357 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d835a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d835f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8364 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8367 mov rdi,rbx
            rdi = rbx;
            // d836a mov rdx,r14
            rdx = r14;
            // d836d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8370 test BYTE PTR [rbp-0x13],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffed)) as u64 & 0x1u64);
            // d8374 jne 0xd8357
            if fl.ne() { pc = 0xd8357; continue; }
            pc = 0xd8376;
        }
        0xd8376 => {
            // block d8376
            // d8376 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d8378 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d837c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d837d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d837f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8380 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x430e6 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000430e6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x430e6;
    loop {
        match pc {
        0x430e6 => {
            // block 430e6
            // 430e6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 430e7 mov rbp,rsp
            rbp = rsp;
            // 430ea push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 430ec push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 430ed sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 430f1 mov r14,rsi
            r14 = rsi;
            // 430f4 mov rbx,rdi
            rbx = rdi;
            // 430f7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 430fa call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43100 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43102 je 0x43138
            if fl.e() { pc = 0x43138; continue; }
            pc = 0x43104;
        }
        0x43104 => {
            // block 43104
            // 43104 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 4310b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4310e lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 43112 mov esi,0x905
            rsi = (0x905u64) as u32 as u64;
            // 43117 mov ecx,0xc0000
            rcx = (0xc0000u64) as u32 as u64;
            // 4311c mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 43122 mov rdi,rbx
            rdi = rbx;
            // 43125 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43128 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4312a je 0x43138
            if fl.e() { pc = 0x43138; continue; }
            pc = 0x4312c;
        }
        0x4312c => {
            // block 4312c
            // 4312c cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // 43130 sete BYTE PTR [r14]
            m.w8(r14, ((fl.e() as u64)) as u8);
            // 43134 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 43136 jmp 0x4313a
            pc = 0x4313a; continue;
        }
        0x43138 => {
            // block 43138
            // 43138 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4313a;
        }
        0x4313a => {
            // block 4313a
            // 4313a add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4313e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4313f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43141 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43142 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc188 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c188<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc188;
    loop {
        match pc {
        0xc188 => {
            // block c188
            // c188 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c189 mov rbp,rsp
            rbp = rsp;
            // c18c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c18e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c190 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c191 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c192 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // c195 mov r14,rsi
            r14 = rsi;
            // c198 mov rbx,rdi
            rbx = rdi;
            // c19b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c19e mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // c1a1 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c1a7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c1a9 je 0xc1af
            if fl.e() { pc = 0xc1af; continue; }
            pc = 0xc1ab;
        }
        0xc1ab => {
            // block c1ab
            // c1ab xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c1ad jmp 0xc1dc
            pc = 0xc1dc; continue;
        }
        0xc1af => {
            // block c1af
            // c1af mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c1b2 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // c1b5 lea rdx,[rip+0xf32c4] # 0xff480
            rdx = m.env.data_va(80973);
            // c1bc mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // c1bf lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // c1c3 mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            // c1c8 mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // c1ce mov rdi,rbx
            rdi = rbx;
            // c1d1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c1d4 cmp DWORD PTR [rbp-0x1c],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x1u64).1;
            // c1d8 sete BYTE PTR [r14]
            m.w8(r14, ((fl.e() as u64)) as u8);
            pc = 0xc1dc;
        }
        0xc1dc => {
            // block c1dc
            // c1dc add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c1e0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c1e1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c1e3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c1e5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c1e6 ret
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d01 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 3d08 lea r14,[rbp-0x18]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            pc = 0x3d0c;
        }
        0x3d0c => {
            // block 3d0c
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d2c test BYTE PTR [rbp-0x17],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe9)) as u64 & 0x1u64);
            // 3d30 jne 0x3d0c
            if fl.ne() { pc = 0x3d0c; continue; }
            pc = 0x3d32;
        }
        0x3d32 => {
            // block 3d32
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

/// parity: P2v @0x42c6c size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042c6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42c6c;
    loop {
        match pc {
        0x42c6c => {
            // block 42c6c
            // 42c6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42c6d mov rbp,rsp
            rbp = rsp;
            // 42c70 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42c72 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42c73 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 42c77 mov r14,rsi
            r14 = rsi;
            // 42c7a mov rbx,rdi
            rbx = rdi;
            // 42c7d mov DWORD PTR [r14],0x8
            m.w32(r14, (0x8u64) as u32);
            // 42c84 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42c87 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42c8d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42c8f je 0x42cc3
            if fl.e() { pc = 0x42cc3; continue; }
            pc = 0x42c91;
        }
        0x42c91 => {
            // block 42c91
            // 42c91 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 42c98 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42c9b lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 42c9f mov esi,0x900
            rsi = (0x900u64) as u32 as u64;
            // 42ca4 mov ecx,0x70
            rcx = (0x70u64) as u32 as u64;
            // 42ca9 mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // 42caf mov rdi,rbx
            rdi = rbx;
            // 42cb2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42cb5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42cb7 je 0x42cc3
            if fl.e() { pc = 0x42cc3; continue; }
            pc = 0x42cb9;
        }
        0x42cb9 => {
            // block 42cb9
            // 42cb9 mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 42cbc mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 42cbf mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 42cc1 jmp 0x42cc5
            pc = 0x42cc5; continue;
        }
        0x42cc3 => {
            // block 42cc3
            // 42cc3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42cc5;
        }
        0x42cc5 => {
            // block 42cc5
            // 42cc5 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 42cc9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42cca pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42ccc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42ccd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0dc4 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0dc4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0dc4;
    loop {
        match pc {
        0xe0dc4 => {
            // block e0dc4
            // e0dc4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0dc5 mov rbp,rsp
            rbp = rsp;
            // e0dc8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0dca push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0dcb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0dcf mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e0dd2 mov rbx,rdi
            rbx = rdi;
            // e0dd5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0dd8 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0ddc call QWORD PTR [rax+0x1ae8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0de2 mov esi,0xfffffcff
            rsi = (0xfffffcffu64) as u32 as u64;
            // e0de7 and esi,DWORD PTR [rbp-0x14]
            { let r = (rsi as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0dea mov DWORD PTR [rbp-0x14],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rsi as u32 as u64)) as u32);
            // e0ded cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // e0df1 je 0xe0e1c
            if fl.e() { pc = 0xe0e1c; continue; }
            pc = 0xe0df3;
        }
        0xe0df3 => {
            // block e0df3
            // e0df3 cmp r14d,0x2
            fl = sub::<32>((r14 as u32 as u64), 0x2u64).1;
            // e0df7 jne 0xe0e01
            if fl.ne() { pc = 0xe0e01; continue; }
            pc = 0xe0df9;
        }
        0xe0df9 => {
            // block e0df9
            // e0df9 or esi,0x200
            { let r = (rsi as u32 as u64) | 0x200u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0dff jmp 0xe0e0d
            pc = 0xe0e0d; continue;
        }
        0xe0e01 => {
            // block e0e01
            // e0e01 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // e0e05 jne 0xe0e10
            if fl.ne() { pc = 0xe0e10; continue; }
            pc = 0xe0e07;
        }
        0xe0e07 => {
            // block e0e07
            // e0e07 or esi,0x100
            { let r = (rsi as u32 as u64) | 0x100u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xe0e0d;
        }
        0xe0e0d => {
            // block e0e0d
            // e0e0d mov DWORD PTR [rbp-0x14],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rsi as u32 as u64)) as u32);
            pc = 0xe0e10;
        }
        0xe0e10 => {
            // block e0e10
            // e0e10 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0e13 mov rdi,rbx
            rdi = rbx;
            // e0e16 call QWORD PTR [rax+0x1ae0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe0e1c;
        }
        0xe0e1c => {
            // block e0e1c
            // e0e1c add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e20 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0e21 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0e23 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0e24 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8b4 size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d8b4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd8b4;
    loop {
        match pc {
        0xd8b4 => {
            // block d8b4
            // d8b4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8b5 mov rbp,rsp
            rbp = rsp;
            // d8b8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d8ba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d8bc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d8bd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d8be mov r14,rdx
            r14 = rdx;
            // d8c1 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d8c4 mov rbx,rdi
            rbx = rdi;
            // d8c7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8ca call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8d0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d8d2 je 0xd8d8
            if fl.e() { pc = 0xd8d8; continue; }
            pc = 0xd8d4;
        }
        0xd8d4 => {
            // block d8d4
            // d8d4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8d6 jmp 0xd90c
            pc = 0xd90c; continue;
        }
        0xd8d8 => {
            // block d8d8
            // d8d8 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // d8df mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8e2 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // d8e5 lea rdx,[rip+0xf1ad4] # 0xff3c0
            rdx = m.env.data_va(80781);
            // d8ec mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // d8ef lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d8f3 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // d8f8 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // d8fe mov rdi,rbx
            rdi = rbx;
            // d901 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d904 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // d908 setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0xd90c;
        }
        0xd90c => {
            // block d90c
            // d90c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d910 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d911 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d913 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d915 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d916 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd96c size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d96c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd96c;
    loop {
        match pc {
        0xd96c => {
            // block d96c
            // d96c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d96d mov rbp,rsp
            rbp = rsp;
            // d970 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d972 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d974 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d975 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d976 mov r14,rdx
            r14 = rdx;
            // d979 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d97c mov rbx,rdi
            rbx = rdi;
            // d97f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d982 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d988 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d98a je 0xd990
            if fl.e() { pc = 0xd990; continue; }
            pc = 0xd98c;
        }
        0xd98c => {
            // block d98c
            // d98c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d98e jmp 0xd9c4
            pc = 0xd9c4; continue;
        }
        0xd990 => {
            // block d990
            // d990 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // d997 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d99a mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // d99d lea rdx,[rip+0xf1a1c] # 0xff3c0
            rdx = m.env.data_va(80781);
            // d9a4 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // d9a7 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d9ab mov ecx,0x1000000
            rcx = (0x1000000u64) as u32 as u64;
            // d9b0 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // d9b6 mov rdi,rbx
            rdi = rbx;
            // d9b9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9bc cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // d9c0 setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0xd9c4;
        }
        0xd9c4 => {
            // block d9c4
            // d9c4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d9c8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d9c9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d9cb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d9cd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d9ce ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1dca size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e1dca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe1dca;
    loop {
        match pc {
        0xe1dca => {
            // block e1dca
            // e1dca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1dcb mov rbp,rsp
            rbp = rsp;
            // e1dce push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1dd0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1dd1 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e1dd5 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e1dd8 mov rbx,rdi
            rbx = rdi;
            // e1ddb mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e1dde mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1de1 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e1de5 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e1dea mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e1def xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1df2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1df5 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // e1df9 mov eax,0xbfffffff
            rax = (0xbfffffffu64) as u32 as u64;
            // e1dfe mov edx,0x7fffffff
            rdx = (0x7fffffffu64) as u32 as u64;
            // e1e03 cmove edx,eax
            if fl.e() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // e1e06 and edx,DWORD PTR [rbp-0x14]
            { let r = (rdx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e1e09 mov DWORD PTR [rbp-0x14],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rdx as u32 as u64)) as u32);
            // e1e0c mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e1e0f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1e12 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e1e17 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e1e1c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1e1f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1e22 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e1e24 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e1e28 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e1e29 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e1e2b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1e2c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba672 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba672<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba672;
    loop {
        match pc {
        0xba672 => {
            // block ba672
            // ba672 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba673 mov rbp,rsp
            rbp = rsp;
            // ba676 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba678 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba67a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ba67c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba67d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ba681 mov r15,rsi
            r15 = rsi;
            // ba684 mov r14,rdi
            r14 = rdi;
            // ba687 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // ba68b mov rdx,r15
            rdx = r15;
            // ba68e call 0xebcde
            { let r = { let t = m.env.stub_va(0x4ab); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba693 mov r12,rax
            r12 = rax;
            // ba696 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // ba69a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // ba69d je 0xba6a3
            if fl.e() { pc = 0xba6a3; continue; }
            pc = 0xba69f;
        }
        0xba69f => {
            // block ba69f
            // ba69f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ba6a1 jmp 0xba6ca
            pc = 0xba6ca; continue;
        }
        0xba6a3 => {
            // block ba6a3
            // ba6a3 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // ba6a8 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba6ad mov rbx,rax
            rbx = rax;
            // ba6b0 mov al,BYTE PTR [r15]
            rax = (rax & !0xff) | ((m.r8(r15) as u64) as u8 as u64);
            // ba6b3 mov BYTE PTR [rbx+0x19],al
            m.w8(rbx.wrapping_add(0x19), ((rax as u8 as u64)) as u8);
            // ba6b6 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // ba6ba mov rdi,r14
            rdi = r14;
            // ba6bd mov rdx,r12
            rdx = r12;
            // ba6c0 mov rcx,rbx
            rcx = rbx;
            // ba6c3 call 0xebcea
            { let r = { let t = m.env.stub_va(0x4b7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba6c8 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xba6ca;
        }
        0xba6ca => {
            // block ba6ca
            // ba6ca mov rax,rbx
            rax = rbx;
            // ba6cd add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ba6d1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba6d2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ba6d4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba6d6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba6d8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba6d9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6182 size=106
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6182<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb6182;
    loop {
        match pc {
        0xb6182 => {
            // block b6182
            // b6182 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6183 mov rbp,rsp
            rbp = rsp;
            // b6186 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b6188 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b618a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b618c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b618d sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // b6191 mov r14,rdx
            r14 = rdx;
            // b6194 mov r12,rdi
            r12 = rdi;
            // b6197 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // b619b mov rcx,r14
            rcx = r14;
            // b619e call 0xebc90
            { let r = { let t = m.env.stub_va(0x45d); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b61a3 mov r15,rax
            r15 = rax;
            // b61a6 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // b61a9 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b61ac jne 0xb61db
            if fl.ne() { pc = 0xb61db; continue; }
            pc = 0xb61ae;
        }
        0xb61ae => {
            // block b61ae
            // b61ae lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b61b2 mov rsi,r12
            rsi = r12;
            // b61b5 mov rdx,r14
            rdx = r14;
            // b61b8 call 0xebcae
            { let r = { let t = m.env.stub_va(0x47b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b61bd mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b61c1 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b61c5 mov rdi,r12
            rdi = r12;
            // b61c8 mov rdx,r15
            rdx = r15;
            // b61cb mov rcx,rbx
            rcx = rbx;
            // b61ce call 0xebcb4
            { let r = { let t = m.env.stub_va(0x481); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b61d3 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            pc = 0xb61db;
        }
        0xb61db => {
            // block b61db
            // b61db mov rax,rbx
            rax = rbx;
            // b61de add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // b61e2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b61e3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b61e5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b61e7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b61e9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b61ea ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1100 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001100<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1100;
    loop {
        match pc {
        0x1100 => {
            // block 1100
            // 1100 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1101 mov rbp,rsp
            rbp = rsp;
            // 1104 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1106 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1108 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 110a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 110b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 110f mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 1112 mov r14,rsi
            r14 = rsi;
            // 1115 mov rbx,rdi
            rbx = rdi;
            // 1118 mov DWORD PTR [rbp-0x24],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0xcu64) as u32);
            // 111f mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // 1123 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1126 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112c xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 112f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1131 cmove r12d,r15d
            if fl.e() { r12 = ((r15 as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // 1135 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1138 lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 113c mov rdi,rbx
            rdi = rbx;
            // 113f mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 1142 call QWORD PTR [rax+0x3e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x3e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1148 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 114a je 0x115e
            if fl.e() { pc = 0x115e; continue; }
            pc = 0x114c;
        }
        0x114c => {
            // block 114c
            // 114c mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 114f and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 1152 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 1155 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 1158 jne 0x115e
            if fl.ne() { pc = 0x115e; continue; }
            pc = 0x115a;
        }
        0x115a => {
            // block 115a
            // 115a mov BYTE PTR [r14],0x1
            m.w8(r14, (0x1u64) as u8);
            pc = 0x115e;
        }
        0x115e => {
            // block 115e
            // 115e mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 1161 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 1165 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1166 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1168 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 116a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 116c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 116d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8dc2 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008dc2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8dc2;
    loop {
        match pc {
        0x8dc2 => {
            // block 8dc2
            // 8dc2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8dc3 mov rbp,rsp
            rbp = rsp;
            // 8dc6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8dc8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8dca push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 8dcc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8dcd sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8dd1 mov r15,rdx
            r15 = rdx;
            // 8dd4 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 8dd7 mov rbx,rdi
            rbx = rdi;
            // 8dda mov DWORD PTR [r15],0x1
            m.w32(r15, (0x1u64) as u32);
            // 8de1 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 8de6 lea r12,[rbp-0x24]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            pc = 0x8dea;
        }
        0x8dea => {
            // block 8dea
            // 8dea mov DWORD PTR [rbp-0x24],0xff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0xffu64) as u32);
            // 8df1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8df4 mov rdi,rbx
            rdi = rbx;
            // 8df7 mov rdx,r12
            rdx = r12;
            // 8dfa call QWORD PTR [rax+0x10c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x10c8)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8e00 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8e02 je 0x8e0a
            if fl.e() { pc = 0x8e0a; continue; }
            pc = 0x8e04;
        }
        0x8e04 => {
            // block 8e04
            // 8e04 cmp DWORD PTR [rbp-0x24],r14d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64, (r14 as u32 as u64)).1;
            // 8e08 je 0x8e21
            if fl.e() { pc = 0x8e21; continue; }
            pc = 0x8e0a;
        }
        0x8e0a => {
            // block 8e0a
            // 8e0a mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 8e0d inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 8e0f mov DWORD PTR [r15],esi
            m.w32(r15, ((rsi as u32 as u64)) as u32);
            // 8e12 cmp esi,0x86
            fl = sub::<32>((rsi as u32 as u64), 0x86u64).1;
            // 8e18 jb 0x8dea
            if fl.b() { pc = 0x8dea; continue; }
            pc = 0x8e1a;
        }
        0x8e1a => {
            // block 8e1a
            // 8e1a mov DWORD PTR [r15],0xffffffff
            m.w32(r15, (0xffffffffu64) as u32);
            pc = 0x8e21;
        }
        0x8e21 => {
            // block 8e21
            // 8e21 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 8e23 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8e27 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8e28 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 8e2a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8e2c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8e2e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8e2f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
