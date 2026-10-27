//! Mechanically lifted functions (scripts/lift_run.py), module b002. Each one is checked against the original
//! in the world harness (tests/world_b002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xf70c size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f70c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf70c;
    loop {
        match pc {
        0xf70c => {
            // block f70c
            #[cfg(feature = "trace")] eprintln!("f_0000f70c pc f70c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f70c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f70d mov rbp,rsp
            rbp = rsp;
            // f710 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f712 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f713 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // f716 mov rbx,rdi
            rbx = rdi;
            // f719 cmp r14d,0xc351
            fl = sub::<32>((r14 as u32 as u64), 0xc351u64).1;
            // f720 jb 0xf732
            if fl.b() { pc = 0xf732; continue; }
            pc = 0xf722;
        }
        0xf722 => {
            // block f722
            #[cfg(feature = "trace")] eprintln!("f_0000f70c pc f722 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f722 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f725 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f72a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f72c je 0xf732
            if fl.e() { pc = 0xf732; continue; }
            pc = 0xf72e;
        }
        0xf72e => {
            // block f72e
            #[cfg(feature = "trace")] eprintln!("f_0000f70c pc f72e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f72e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // f730 jmp 0xf74b
            pc = 0xf74b; continue;
        }
        0xf732 => {
            // block f732
            #[cfg(feature = "trace")] eprintln!("f_0000f70c pc f732 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f732 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f735 mov esi,0x14d
            rsi = (0x14du64) as u32 as u64;
            // f73a mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // f73f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // f742 mov rdi,rbx
            rdi = rbx;
            // f745 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // f748 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xf74b;
        }
        0xf74b => {
            // block f74b
            #[cfg(feature = "trace")] eprintln!("f_0000f70c pc f74b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f74b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f74c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f74e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f74f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43ce2 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043ce2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43ce2;
    loop {
        match pc {
        0x43ce2 => {
            // block 43ce2
            #[cfg(feature = "trace")] eprintln!("f_00043ce2 pc 43ce2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43ce2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43ce3 mov rbp,rsp
            rbp = rsp;
            // 43ce6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43ce7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43ce8 mov rbx,rdx
            rbx = rdx;
            // 43ceb mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 43cf2 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43cf5 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 43cf7 lea rdx,[rip+0xbc772] # 0x100470
            rdx = m.env.data_va(85053);
            // 43cfe mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43d01 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 43d05 mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // 43d0a mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 43d10 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43d13 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43d15 je 0x43d1e
            if fl.e() { pc = 0x43d1e; continue; }
            pc = 0x43d17;
        }
        0x43d17 => {
            // block 43d17
            #[cfg(feature = "trace")] eprintln!("f_00043ce2 pc 43d17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43d17 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 43d1b setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x43d1e;
        }
        0x43d1e => {
            // block 43d1e
            #[cfg(feature = "trace")] eprintln!("f_00043ce2 pc 43d1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43d1e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43d22 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43d23 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43d24 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0baa size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0baa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0baa;
    loop {
        match pc {
        0xe0baa => {
            // block e0baa
            #[cfg(feature = "trace")] eprintln!("f_000e0baa pc e0baa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0baa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0bab mov rbp,rsp
            rbp = rsp;
            // e0bae push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0bb0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0bb1 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0bb5 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e0bb7 mov r14,rdi
            r14 = rdi;
            // e0bba mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0bbd lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0bc1 call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0bc7 mov eax,0xfffff0ff
            rax = (0xfffff0ffu64) as u32 as u64;
            // e0bcc and eax,DWORD PTR [rbp-0x14]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0bcf shl ebx,0x8
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x8u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0bd2 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0bd4 mov DWORD PTR [rbp-0x14],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rbx as u32 as u64)) as u32);
            // e0bd7 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0bda mov rdi,r14
            rdi = r14;
            // e0bdd mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e0bdf call QWORD PTR [rax+0x1ac0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0be5 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0be9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0bea pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0bec pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0bed ret
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
            #[cfg(feature = "trace")] eprintln!("f_000e0bee pc e0bee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xe0c32 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0c32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0c32;
    loop {
        match pc {
        0xe0c32 => {
            // block e0c32
            #[cfg(feature = "trace")] eprintln!("f_000e0c32 pc e0c32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0c32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0c33 mov rbp,rsp
            rbp = rsp;
            // e0c36 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0c38 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0c39 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0c3d mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e0c3f mov r14,rdi
            r14 = rdi;
            // e0c42 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0c45 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0c49 call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0c4f mov eax,0xffff0fff
            rax = (0xffff0fffu64) as u32 as u64;
            // e0c54 and eax,DWORD PTR [rbp-0x14]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0c57 shl ebx,0xc
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0xcu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0c5a or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0c5c mov DWORD PTR [rbp-0x14],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rbx as u32 as u64)) as u32);
            // e0c5f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0c62 mov rdi,r14
            rdi = r14;
            // e0c65 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e0c67 call QWORD PTR [rax+0x1ac0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0c6d add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0c71 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0c72 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0c74 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0c75 ret
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
            #[cfg(feature = "trace")] eprintln!("f_000e0e4a pc e0e4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3006 mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // e3009 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e300b cmp ebx,0x1
            fl = sub::<32>((rbx as u32 as u64), 0x1u64).1;
            // e300e jne 0xe301a
            if fl.ne() { pc = 0xe301a; continue; }
            pc = 0xe3010;
        }
        0xe3010 => {
            // block e3010
            #[cfg(feature = "trace")] eprintln!("f_000e2fe4 pc e3010 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3010 test ecx,0x40000000
            fl = logic::<32>((rcx as u32 as u64) & 0x40000000u64);
            // e3016 je 0xe301e
            if fl.e() { pc = 0xe301e; continue; }
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
            if fl.s() { pc = 0xe3020; continue; }
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

/// parity: P2v @0xe45f8 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e45f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe45f8;
    loop {
        match pc {
        0xe45f8 => {
            // block e45f8
            #[cfg(feature = "trace")] eprintln!("f_000e45f8 pc e45f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e45f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e45f9 mov rbp,rsp
            rbp = rsp;
            // e45fc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e45fe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4600 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4601 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e4602 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e4605 mov rbx,rdi
            rbx = rdi;
            // e4608 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e460f lea r15,[rbp-0x1c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            pc = 0xe4613;
        }
        0xe4613 => {
            // block e4613
            #[cfg(feature = "trace")] eprintln!("f_000e45f8 pc e4613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4613 mov rdi,rbx
            rdi = rbx;
            // e4616 mov rsi,r15
            rsi = r15;
            // e4619 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // e461c call 0xe463c
            { let r = { let t = m.env.code_va(0xe463c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4621 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e4623 je 0xe462f
            if fl.e() { pc = 0xe462f; continue; }
            pc = 0xe4625;
        }
        0xe4625 => {
            // block e4625
            #[cfg(feature = "trace")] eprintln!("f_000e45f8 pc e4625 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4625 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e4627 cmp DWORD PTR [rbp-0x1c],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0xffffffffu64).1;
            // e462b jne 0xe4613
            if fl.ne() { pc = 0xe4613; continue; }
            pc = 0xe462d;
        }
        0xe462d => {
            // block e462d
            #[cfg(feature = "trace")] eprintln!("f_000e45f8 pc e462d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e462d jmp 0xe4631
            pc = 0xe4631; continue;
        }
        0xe462f => {
            // block e462f
            #[cfg(feature = "trace")] eprintln!("f_000e45f8 pc e462f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e462f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4631;
        }
        0xe4631 => {
            // block e4631
            #[cfg(feature = "trace")] eprintln!("f_000e45f8 pc e4631 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4631 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e4635 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4636 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4638 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e463a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e463b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6a3e size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006a3e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6a3e;
    loop {
        match pc {
        0x6a3e => {
            // block 6a3e
            #[cfg(feature = "trace")] eprintln!("f_00006a3e pc 6a3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6a3e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6a3f mov rbp,rsp
            rbp = rsp;
            // 6a42 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6a44 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6a45 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 6a49 mov r14,rsi
            r14 = rsi;
            // 6a4c mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 6a53 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 6a56 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 6a58 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 6a5c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 6a5e mov ecx,0x8000000
            rcx = (0x8000000u64) as u32 as u64;
            // 6a63 mov r8d,0x1b
            r8 = (0x1bu64) as u32 as u64;
            // 6a69 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6a6c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6a6e je 0x6a77
            if fl.e() { pc = 0x6a77; continue; }
            pc = 0x6a70;
        }
        0x6a70 => {
            // block 6a70
            #[cfg(feature = "trace")] eprintln!("f_00006a3e pc 6a70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6a70 cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // 6a74 setne bl
            rbx = (rbx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x6a77;
        }
        0x6a77 => {
            // block 6a77
            #[cfg(feature = "trace")] eprintln!("f_00006a3e pc 6a77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6a77 mov BYTE PTR [r14],bl
            m.w8(r14, ((rbx as u8 as u64)) as u8);
            // 6a7a add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 6a7e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6a7f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6a81 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6a82 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdd26 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000dd26<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdd26;
    loop {
        match pc {
        0xdd26 => {
            // block dd26
            #[cfg(feature = "trace")] eprintln!("f_0000dd26 pc dd26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd26 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dd27 mov rbp,rsp
            rbp = rsp;
            // dd2a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dd2c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dd2d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // dd31 mov r14,rsi
            r14 = rsi;
            // dd34 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dd37 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // dd3b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // dd3d mov esi,0x136
            rsi = (0x136u64) as u32 as u64;
            // dd42 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dd47 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dd4a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd4d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dd4f je 0xdd60
            if fl.e() { pc = 0xdd60; continue; }
            pc = 0xdd51;
        }
        0xdd51 => {
            // block dd51
            #[cfg(feature = "trace")] eprintln!("f_0000dd26 pc dd51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd51 cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // dd55 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // dd58 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // dd5b mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // dd5e mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xdd60;
        }
        0xdd60 => {
            // block dd60
            #[cfg(feature = "trace")] eprintln!("f_0000dd26 pc dd60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd60 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // dd62 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // dd66 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dd67 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dd69 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dd6a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x423a2 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000423a2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x423a2;
    loop {
        match pc {
        0x423a2 => {
            // block 423a2
            #[cfg(feature = "trace")] eprintln!("f_000423a2 pc 423a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 423a2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 423a3 mov rbp,rsp
            rbp = rsp;
            // 423a6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 423a7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 423a8 mov rbx,rsi
            rbx = rsi;
            // 423ab mov DWORD PTR [rbp-0xc],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x2u64) as u32);
            // 423b2 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 423b5 lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 423b9 call QWORD PTR [rax+0x6a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x6a8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 423bf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 423c1 je 0x423d3
            if fl.e() { pc = 0x423d3; continue; }
            pc = 0x423c3;
        }
        0x423c3 => {
            // block 423c3
            #[cfg(feature = "trace")] eprintln!("f_000423a2 pc 423c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 423c3 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 423c6 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 423c9 jne 0x423d7
            if fl.ne() { pc = 0x423d7; continue; }
            pc = 0x423cb;
        }
        0x423cb => {
            // block 423cb
            #[cfg(feature = "trace")] eprintln!("f_000423a2 pc 423cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 423cb mov DWORD PTR [rbx],0x3fc000
            m.w32(rbx, (0x3fc000u64) as u32);
            // 423d1 jmp 0x423df
            pc = 0x423df; continue;
        }
        0x423d3 => {
            // block 423d3
            #[cfg(feature = "trace")] eprintln!("f_000423a2 pc 423d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 423d3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 423d5 jmp 0x423e1
            pc = 0x423e1; continue;
        }
        0x423d7 => {
            // block 423d7
            #[cfg(feature = "trace")] eprintln!("f_000423a2 pc 423d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 423d7 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 423d9 mov DWORD PTR [rbx],0xff000
            m.w32(rbx, (0xff000u64) as u32);
            pc = 0x423df;
        }
        0x423df => {
            // block 423df
            #[cfg(feature = "trace")] eprintln!("f_000423a2 pc 423df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 423df mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x423e1;
        }
        0x423e1 => {
            // block 423e1
            #[cfg(feature = "trace")] eprintln!("f_000423a2 pc 423e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 423e1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 423e5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 423e6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 423e7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x210e size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000210e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x210e;
    loop {
        match pc {
        0x210e => {
            // block 210e
            #[cfg(feature = "trace")] eprintln!("f_0000210e pc 210e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 210e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 210f mov rbp,rsp
            rbp = rsp;
            // 2112 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2113 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2114 mov rbx,rdi
            rbx = rdi;
            // 2117 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 211a call 0xce188
            { let r = { let t = m.env.code_va(0xce188); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 211f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2121 je 0x214d
            if fl.e() { pc = 0x214d; continue; }
            pc = 0x2123;
        }
        0x2123 => {
            // block 2123
            #[cfg(feature = "trace")] eprintln!("f_0000210e pc 2123 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2123 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2126 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 212a mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 212f mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 2134 mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 213a mov rdi,rbx
            rdi = rbx;
            // 213d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2140 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2142 je 0x214d
            if fl.e() { pc = 0x214d; continue; }
            pc = 0x2144;
        }
        0x2144 => {
            // block 2144
            #[cfg(feature = "trace")] eprintln!("f_0000210e pc 2144 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2144 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 2148 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 214b jmp 0x214f
            pc = 0x214f; continue;
        }
        0x214d => {
            // block 214d
            #[cfg(feature = "trace")] eprintln!("f_0000210e pc 214d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 214d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x214f;
        }
        0x214f => {
            // block 214f
            #[cfg(feature = "trace")] eprintln!("f_0000210e pc 214f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 214f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2153 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2154 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2155 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf402 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f402<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf402;
    loop {
        match pc {
        0xf402 => {
            // block f402
            #[cfg(feature = "trace")] eprintln!("f_0000f402 pc f402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f402 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f403 mov rbp,rsp
            rbp = rsp;
            // f406 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f408 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f409 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // f40c mov rbx,rdi
            rbx = rdi;
            // f40f cmp r14d,0xc351
            fl = sub::<32>((r14 as u32 as u64), 0xc351u64).1;
            // f416 jb 0xf428
            if fl.b() { pc = 0xf428; continue; }
            pc = 0xf418;
        }
        0xf418 => {
            // block f418
            #[cfg(feature = "trace")] eprintln!("f_0000f402 pc f418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f418 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f41b call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f420 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f422 je 0xf428
            if fl.e() { pc = 0xf428; continue; }
            pc = 0xf424;
        }
        0xf424 => {
            // block f424
            #[cfg(feature = "trace")] eprintln!("f_0000f402 pc f424 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f424 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // f426 jmp 0xf444
            pc = 0xf444; continue;
        }
        0xf428 => {
            // block f428
            #[cfg(feature = "trace")] eprintln!("f_0000f402 pc f428 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f428 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f42b mov esi,0x14a
            rsi = (0x14au64) as u32 as u64;
            // f430 mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // f435 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // f43b mov rdi,rbx
            rdi = rbx;
            // f43e mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // f441 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xf444;
        }
        0xf444 => {
            // block f444
            #[cfg(feature = "trace")] eprintln!("f_0000f402 pc f444 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f444 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f445 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f447 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f448 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf538 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f538<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf538;
    loop {
        match pc {
        0xf538 => {
            // block f538
            #[cfg(feature = "trace")] eprintln!("f_0000f538 pc f538 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f538 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f539 mov rbp,rsp
            rbp = rsp;
            // f53c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f53e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f53f mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // f542 mov rbx,rdi
            rbx = rdi;
            // f545 cmp r14d,0xc351
            fl = sub::<32>((r14 as u32 as u64), 0xc351u64).1;
            // f54c jb 0xf55e
            if fl.b() { pc = 0xf55e; continue; }
            pc = 0xf54e;
        }
        0xf54e => {
            // block f54e
            #[cfg(feature = "trace")] eprintln!("f_0000f538 pc f54e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f54e mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f551 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f556 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f558 je 0xf55e
            if fl.e() { pc = 0xf55e; continue; }
            pc = 0xf55a;
        }
        0xf55a => {
            // block f55a
            #[cfg(feature = "trace")] eprintln!("f_0000f538 pc f55a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f55a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // f55c jmp 0xf57a
            pc = 0xf57a; continue;
        }
        0xf55e => {
            // block f55e
            #[cfg(feature = "trace")] eprintln!("f_0000f538 pc f55e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f55e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f561 mov esi,0x14b
            rsi = (0x14bu64) as u32 as u64;
            // f566 mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // f56b mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // f571 mov rdi,rbx
            rdi = rbx;
            // f574 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // f577 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xf57a;
        }
        0xf57a => {
            // block f57a
            #[cfg(feature = "trace")] eprintln!("f_0000f538 pc f57a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f57a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f57b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f57d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f57e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf66e size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f66e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf66e;
    loop {
        match pc {
        0xf66e => {
            // block f66e
            #[cfg(feature = "trace")] eprintln!("f_0000f66e pc f66e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f66e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f66f mov rbp,rsp
            rbp = rsp;
            // f672 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f674 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f675 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // f678 mov rbx,rdi
            rbx = rdi;
            // f67b cmp r14d,0xc351
            fl = sub::<32>((r14 as u32 as u64), 0xc351u64).1;
            // f682 jb 0xf694
            if fl.b() { pc = 0xf694; continue; }
            pc = 0xf684;
        }
        0xf684 => {
            // block f684
            #[cfg(feature = "trace")] eprintln!("f_0000f66e pc f684 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f684 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f687 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f68c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f68e je 0xf694
            if fl.e() { pc = 0xf694; continue; }
            pc = 0xf690;
        }
        0xf690 => {
            // block f690
            #[cfg(feature = "trace")] eprintln!("f_0000f66e pc f690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f690 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // f692 jmp 0xf6b0
            pc = 0xf6b0; continue;
        }
        0xf694 => {
            // block f694
            #[cfg(feature = "trace")] eprintln!("f_0000f66e pc f694 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f694 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f697 mov esi,0x14c
            rsi = (0x14cu64) as u32 as u64;
            // f69c mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // f6a1 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // f6a7 mov rdi,rbx
            rdi = rbx;
            // f6aa mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // f6ad call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xf6b0;
        }
        0xf6b0 => {
            // block f6b0
            #[cfg(feature = "trace")] eprintln!("f_0000f66e pc f6b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f6b0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f6b1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f6b3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f6b4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf7a4 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f7a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf7a4;
    loop {
        match pc {
        0xf7a4 => {
            // block f7a4
            #[cfg(feature = "trace")] eprintln!("f_0000f7a4 pc f7a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f7a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f7a5 mov rbp,rsp
            rbp = rsp;
            // f7a8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f7aa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f7ab mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // f7ae mov rbx,rdi
            rbx = rdi;
            // f7b1 cmp r14d,0xc351
            fl = sub::<32>((r14 as u32 as u64), 0xc351u64).1;
            // f7b8 jb 0xf7ca
            if fl.b() { pc = 0xf7ca; continue; }
            pc = 0xf7ba;
        }
        0xf7ba => {
            // block f7ba
            #[cfg(feature = "trace")] eprintln!("f_0000f7a4 pc f7ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f7ba mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f7bd call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f7c2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f7c4 je 0xf7ca
            if fl.e() { pc = 0xf7ca; continue; }
            pc = 0xf7c6;
        }
        0xf7c6 => {
            // block f7c6
            #[cfg(feature = "trace")] eprintln!("f_0000f7a4 pc f7c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f7c6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // f7c8 jmp 0xf7e6
            pc = 0xf7e6; continue;
        }
        0xf7ca => {
            // block f7ca
            #[cfg(feature = "trace")] eprintln!("f_0000f7a4 pc f7ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f7ca mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f7cd mov esi,0x14d
            rsi = (0x14du64) as u32 as u64;
            // f7d2 mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // f7d7 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // f7dd mov rdi,rbx
            rdi = rbx;
            // f7e0 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // f7e3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xf7e6;
        }
        0xf7e6 => {
            // block f7e6
            #[cfg(feature = "trace")] eprintln!("f_0000f7a4 pc f7e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f7e6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f7e7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f7e9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f7ea ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfa4e size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fa4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xfa4e;
    loop {
        match pc {
        0xfa4e => {
            // block fa4e
            #[cfg(feature = "trace")] eprintln!("f_0000fa4e pc fa4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fa4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fa4f mov rbp,rsp
            rbp = rsp;
            // fa52 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fa53 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // fa54 mov rbx,rdi
            rbx = rdi;
            // fa57 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // fa5a call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fa5f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // fa61 je 0xfa8d
            if fl.e() { pc = 0xfa8d; continue; }
            pc = 0xfa63;
        }
        0xfa63 => {
            // block fa63
            #[cfg(feature = "trace")] eprintln!("f_0000fa4e pc fa63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fa63 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // fa6a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fa6d lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // fa71 mov esi,0x150
            rsi = (0x150u64) as u32 as u64;
            // fa76 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // fa7b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // fa7e mov rdi,rbx
            rdi = rbx;
            // fa81 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fa84 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // fa88 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // fa8b jmp 0xfa8f
            pc = 0xfa8f; continue;
        }
        0xfa8d => {
            // block fa8d
            #[cfg(feature = "trace")] eprintln!("f_0000fa4e pc fa8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fa8d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xfa8f;
        }
        0xfa8f => {
            // block fa8f
            #[cfg(feature = "trace")] eprintln!("f_0000fa4e pc fa8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fa8f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // fa93 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fa94 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fa95 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43934 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043934<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43934;
    loop {
        match pc {
        0x43934 => {
            // block 43934
            #[cfg(feature = "trace")] eprintln!("f_00043934 pc 43934 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43934 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43935 mov rbp,rsp
            rbp = rsp;
            // 43938 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43939 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4393a mov rbx,rdx
            rbx = rdx;
            // 4393d cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43940 jle 0x43946
            if fl.le() { pc = 0x43946; continue; }
            pc = 0x43942;
        }
        0x43942 => {
            // block 43942
            #[cfg(feature = "trace")] eprintln!("f_00043934 pc 43942 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43942 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43944 jmp 0x43975
            pc = 0x43975; continue;
        }
        0x43946 => {
            // block 43946
            #[cfg(feature = "trace")] eprintln!("f_00043934 pc 43946 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43946 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 4394d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43950 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 43952 lea rdx,[rip+0xbca67] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 43959 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4395c lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 43960 mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // 43965 mov r8d,0x2
            r8 = (0x2u64) as u32 as u64;
            // 4396b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4396e cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 43972 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x43975;
        }
        0x43975 => {
            // block 43975
            #[cfg(feature = "trace")] eprintln!("f_00043934 pc 43975 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43975 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43979 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4397a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4397b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43a24 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043a24<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43a24;
    loop {
        match pc {
        0x43a24 => {
            // block 43a24
            #[cfg(feature = "trace")] eprintln!("f_00043a24 pc 43a24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a24 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43a25 mov rbp,rsp
            rbp = rsp;
            // 43a28 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43a29 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43a2a mov rbx,rdx
            rbx = rdx;
            // 43a2d cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43a30 jle 0x43a36
            if fl.le() { pc = 0x43a36; continue; }
            pc = 0x43a32;
        }
        0x43a32 => {
            // block 43a32
            #[cfg(feature = "trace")] eprintln!("f_00043a24 pc 43a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a32 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43a34 jmp 0x43a65
            pc = 0x43a65; continue;
        }
        0x43a36 => {
            // block 43a36
            #[cfg(feature = "trace")] eprintln!("f_00043a24 pc 43a36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a36 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 43a3d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43a40 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 43a42 lea rdx,[rip+0xbc977] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 43a49 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43a4c lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 43a50 mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // 43a55 mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // 43a5b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43a5e cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 43a62 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x43a65;
        }
        0x43a65 => {
            // block 43a65
            #[cfg(feature = "trace")] eprintln!("f_00043a24 pc 43a65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a65 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43a69 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43a6a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43a6b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44de4 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044de4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44de4;
    loop {
        match pc {
        0x44de4 => {
            // block 44de4
            #[cfg(feature = "trace")] eprintln!("f_00044de4 pc 44de4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44de4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44de5 mov rbp,rsp
            rbp = rsp;
            // 44de8 sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // 44dec mov r10d,ecx
            r10 = ((rcx as u32 as u64)) as u32 as u64;
            // 44def mov rax,rdx
            rax = rdx;
            // 44df2 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 44df4 mov edx,DWORD PTR [rbp+0x10]
            rdx = (m.r32(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 44df7 mov esi,DWORD PTR [rbp+0x18]
            rsi = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 44dfa mov DWORD PTR [rsp+0x18],esi
            m.w32(rsp.wrapping_add(0x18), ((rsi as u32 as u64)) as u32);
            // 44dfe mov DWORD PTR [rsp+0x10],edx
            m.w32(rsp.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // 44e02 mov DWORD PTR [rsp+0x8],r9d
            m.w32(rsp.wrapping_add(0x8), ((r9 as u32 as u64)) as u32);
            // 44e07 mov DWORD PTR [rsp],r8d
            m.w32(rsp, ((r8 as u32 as u64)) as u32);
            // 44e0b mov DWORD PTR [rsp+0x20],0x1
            m.w32(rsp.wrapping_add(0x20), (0x1u64) as u32);
            // 44e13 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 44e18 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 44e1a mov r8,rax
            r8 = rax;
            // 44e1d mov r9d,r10d
            r9 = ((r10 as u32 as u64)) as u32 as u64;
            // 44e20 call 0x4e8a2
            { let r = { let t = m.env.code_va(0x4e8a2); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44e25 add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // 44e29 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44e2a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
