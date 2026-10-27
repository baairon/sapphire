//! Mechanically lifted functions (scripts/lift_run.py), module u2001. Each one is checked against the original
//! in the world harness (tests/world_u2001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xfc60 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fc60<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xfc60;
    loop {
        match pc {
        0xfc60 => {
            // block fc60
            #[cfg(feature = "trace")] eprintln!("f_0000fc60 pc fc60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fc60 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fc61 mov rbp,rsp
            rbp = rsp;
            // fc64 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // fc66 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fc67 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // fc6a mov rbx,rdi
            rbx = rdi;
            // fc6d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // fc70 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fc75 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // fc77 je 0xfcab
            if fl.e() { pc = 0xfcab; continue; }
            pc = 0xfc79;
        }
        0xfc79 => {
            // block fc79
            #[cfg(feature = "trace")] eprintln!("f_0000fc60 pc fc79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fc79 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fc7c movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // fc80 mov esi,0x150
            rsi = (0x150u64) as u32 as u64;
            // fc85 mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // fc8a mov r8d,0x6
            r8 = (0x6u64) as u32 as u64;
            // fc90 mov rdi,rbx
            rdi = rbx;
            // fc93 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fc96 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // fc99 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fc9c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // fc9e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // fca0 mov rdi,rbx
            rdi = rbx;
            // fca3 call QWORD PTR [rax+0xbd8]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd8)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fca9 jmp 0xfcae
            pc = 0xfcae; continue;
        }
        0xfcab => {
            // block fcab
            #[cfg(feature = "trace")] eprintln!("f_0000fc60 pc fcab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fcab xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xfcae;
        }
        0xfcae => {
            // block fcae
            #[cfg(feature = "trace")] eprintln!("f_0000fc60 pc fcae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fcae mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // fcb1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fcb2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // fcb4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fcb5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x115fe size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000115fe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x115fe;
    loop {
        match pc {
        0x115fe => {
            // block 115fe
            #[cfg(feature = "trace")] eprintln!("f_000115fe pc 115fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 115fe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 115ff mov rbp,rsp
            rbp = rsp;
            // 11602 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11604 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11606 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11607 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 11608 mov r14,rdi
            r14 = rdi;
            // 1160b mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 11613 mov rdi,QWORD PTR [r14+0x38]
            rdi = m.r64(r14.wrapping_add(0x38)) as u64;
            // 11617 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1161a lea rsi,[rbp-0x20]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // 1161e call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11621 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 11623 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 11625 jne 0x11647
            if fl.ne() { pc = 0x11647; continue; }
            pc = 0x11627;
        }
        0x11627 => {
            // block 11627
            #[cfg(feature = "trace")] eprintln!("f_000115fe pc 11627 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11627 mov rdi,QWORD PTR [rbp-0x20]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 1162b call 0x1153a
            { let r = { let t = m.env.code_va(0x1153a); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11630 mov r15,rax
            r15 = rax;
            // 11633 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 11636 mov rdi,r14
            rdi = r14;
            // 11639 mov rsi,r15
            rsi = r15;
            // 1163c call QWORD PTR [rax+0x60]
            { let r = { let t = m.r64(rax.wrapping_add(0x60)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1163f mov rdi,r15
            rdi = r15;
            // 11642 call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x11647;
        }
        0x11647 => {
            // block 11647
            #[cfg(feature = "trace")] eprintln!("f_000115fe pc 11647 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11647 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 11649 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1164d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1164e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11650 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11652 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11653 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdbc6 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000dbc6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdbc6;
    loop {
        match pc {
        0xdbc6 => {
            // block dbc6
            #[cfg(feature = "trace")] eprintln!("f_0000dbc6 pc dbc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbc6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dbc7 mov rbp,rsp
            rbp = rsp;
            // dbca push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dbcc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dbcd sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // dbd1 mov r14,rdx
            r14 = rdx;
            // dbd4 mov rbx,rdi
            rbx = rdi;
            // dbd7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dbda call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dbe0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dbe2 je 0xdbe8
            if fl.e() { pc = 0xdbe8; continue; }
            pc = 0xdbe4;
        }
        0xdbe4 => {
            // block dbe4
            #[cfg(feature = "trace")] eprintln!("f_0000dbc6 pc dbe4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dbe6 jmp 0xdc14
            pc = 0xdc14; continue;
        }
        0xdbe8 => {
            // block dbe8
            #[cfg(feature = "trace")] eprintln!("f_0000dbc6 pc dbe8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dbe8 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // dbef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dbf2 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // dbf6 mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // dbfb mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // dc00 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // dc06 mov rdi,rbx
            rdi = rbx;
            // dc09 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc0c cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // dc10 setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0xdc14;
        }
        0xdc14 => {
            // block dc14
            #[cfg(feature = "trace")] eprintln!("f_0000dbc6 pc dc14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc14 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // dc18 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc19 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc1b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc1c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe410 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e410<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe410;
    loop {
        match pc {
        0xe410 => {
            // block e410
            #[cfg(feature = "trace")] eprintln!("f_0000e410 pc e410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e410 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e411 mov rbp,rsp
            rbp = rsp;
            // e414 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e416 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e417 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e41b mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e41e mov rbx,rdi
            rbx = rdi;
            // e421 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e424 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e42a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e42c je 0xe432
            if fl.e() { pc = 0xe432; continue; }
            pc = 0xe42e;
        }
        0xe42e => {
            // block e42e
            #[cfg(feature = "trace")] eprintln!("f_0000e410 pc e42e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e42e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e430 jmp 0xe45e
            pc = 0xe45e; continue;
        }
        0xe432 => {
            // block e432
            #[cfg(feature = "trace")] eprintln!("f_0000e410 pc e432 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e432 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e435 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // e438 lea rdx,[rip+0xf12c1] # 0xff700
            rdx = m.env.data_va(81613);
            // e43f mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e442 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e446 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // e44b mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // e451 mov rdi,rbx
            rdi = rbx;
            // e454 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e457 cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // e45b setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0xe45e;
        }
        0xe45e => {
            // block e45e
            #[cfg(feature = "trace")] eprintln!("f_0000e410 pc e45e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e45e add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e462 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e463 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e465 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e466 ret
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
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xb6914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x41f14 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041f14<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x41f14;
    loop {
        match pc {
        0x41f14 => {
            // block 41f14
            #[cfg(feature = "trace")] eprintln!("f_00041f14 pc 41f14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41f14 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41f15 mov rbp,rsp
            rbp = rsp;
            // 41f18 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 41f1a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 41f1c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 41f1d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 41f1e mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 41f21 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 41f24 mov rbx,rdi
            rbx = rdi;
            // 41f27 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // 41f2b je 0x41f3d
            if fl.e() { pc = 0x41f3d; continue; }
            pc = 0x41f2d;
        }
        0x41f2d => {
            // block 41f2d
            #[cfg(feature = "trace")] eprintln!("f_00041f14 pc 41f2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41f2d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 41f30 call 0xcd038
            { let r = { let t = m.env.code_va(0xcd038); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41f35 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 41f37 je 0x41f3d
            if fl.e() { pc = 0x41f3d; continue; }
            pc = 0x41f39;
        }
        0x41f39 => {
            // block 41f39
            #[cfg(feature = "trace")] eprintln!("f_00041f14 pc 41f39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41f39 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 41f3b jmp 0x41f61
            pc = 0x41f61; continue;
        }
        0x41f3d => {
            // block 41f3d
            #[cfg(feature = "trace")] eprintln!("f_00041f14 pc 41f3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41f3d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 41f40 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 41f43 lea rdx,[rip+0xbe476] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 41f4a mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 41f4d mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 41f52 mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 41f58 mov rdi,rbx
            rdi = rbx;
            // 41f5b mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 41f5e call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x41f61;
        }
        0x41f61 => {
            // block 41f61
            #[cfg(feature = "trace")] eprintln!("f_00041f14 pc 41f61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41f61 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 41f65 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 41f66 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 41f68 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 41f6a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41f6b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43232 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043232<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43232;
    loop {
        match pc {
        0x43232 => {
            // block 43232
            #[cfg(feature = "trace")] eprintln!("f_00043232 pc 43232 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43232 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43233 mov rbp,rsp
            rbp = rsp;
            // 43236 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4323a mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 4323c cmp eax,0xf
            fl = sub::<32>((rax as u32 as u64), 0xfu64).1;
            // 4323f jle 0x43245
            if fl.le() { pc = 0x43245; continue; }
            pc = 0x43241;
        }
        0x43241 => {
            // block 43241
            #[cfg(feature = "trace")] eprintln!("f_00043232 pc 43241 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43241 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43243 jmp 0x43283
            pc = 0x43283; continue;
        }
        0x43245 => {
            // block 43245
            #[cfg(feature = "trace")] eprintln!("f_00043232 pc 43245 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43245 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // 4324c mov r9,QWORD PTR [rdi]
            r9 = m.r64(rdi) as u64;
            // 4324f mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 43251 lea rdx,[rip+0xbd318] # 0x100570
            rdx = m.env.data_va(85309);
            // 43258 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4325b and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4325e mov edx,0xffff
            rdx = (0xffffu64) as u32 as u64;
            // 43263 mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // 43268 cmove ecx,edx
            if fl.e() { rcx = ((rdx as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 4326b shl eax,0x4
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 4326e lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // 43272 mov r8d,eax
            r8 = ((rax as u32 as u64)) as u32 as u64;
            // 43275 call QWORD PTR [r9+0x38]
            { let r = { let t = m.r64(r9.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43279 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4327b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4327d cmovne ecx,DWORD PTR [rbp-0x4]
            if fl.ne() { rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 43281 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x43283;
        }
        0x43283 => {
            // block 43283
            #[cfg(feature = "trace")] eprintln!("f_00043232 pc 43283 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43283 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43287 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43288 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43c0a size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043c0a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43c0a;
    loop {
        match pc {
        0x43c0a => {
            // block 43c0a
            #[cfg(feature = "trace")] eprintln!("f_00043c0a pc 43c0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43c0a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43c0b mov rbp,rsp
            rbp = rsp;
            // 43c0e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43c10 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43c12 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43c13 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43c14 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 43c17 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 43c19 mov r15,rdi
            r15 = rdi;
            // 43c1c mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43c20 call 0xcbb7b
            { let r = { let t = m.env.code_va(0xcbb7b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43c25 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 43c27 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43c29 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 43c2c jg 0x43c56
            if fl.g() { pc = 0x43c56; continue; }
            pc = 0x43c2e;
        }
        0x43c2e => {
            // block 43c2e
            #[cfg(feature = "trace")] eprintln!("f_00043c0a pc 43c2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43c2e xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 43c31 jne 0x43c56
            if fl.ne() { pc = 0x43c56; continue; }
            pc = 0x43c33;
        }
        0x43c33 => {
            // block 43c33
            #[cfg(feature = "trace")] eprintln!("f_00043c0a pc 43c33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43c33 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43c36 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 43c38 lea rdx,[rip+0xbc9e1] # 0x100620
            rdx = m.env.data_va(85485);
            // 43c3f mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43c42 mov ecx,0x1fff0000
            rcx = (0x1fff0000u64) as u32 as u64;
            // 43c47 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 43c4d mov rdi,r15
            rdi = r15;
            // 43c50 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 43c53 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43c56;
        }
        0x43c56 => {
            // block 43c56
            #[cfg(feature = "trace")] eprintln!("f_00043c0a pc 43c56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43c56 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43c5a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43c5b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43c5d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43c5f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43c60 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43c62 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043c62<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43c62;
    loop {
        match pc {
        0x43c62 => {
            // block 43c62
            #[cfg(feature = "trace")] eprintln!("f_00043c62 pc 43c62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43c62 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43c63 mov rbp,rsp
            rbp = rsp;
            // 43c66 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43c68 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43c6a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43c6b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43c6c mov r14,rdx
            r14 = rdx;
            // 43c6f mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 43c71 mov r15,rdi
            r15 = rdi;
            // 43c74 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43c78 call 0xcbb7b
            { let r = { let t = m.env.code_va(0xcbb7b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43c7d mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 43c7f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43c81 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 43c84 jg 0x43cae
            if fl.g() { pc = 0x43cae; continue; }
            pc = 0x43c86;
        }
        0x43c86 => {
            // block 43c86
            #[cfg(feature = "trace")] eprintln!("f_00043c62 pc 43c86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43c86 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 43c89 jne 0x43cae
            if fl.ne() { pc = 0x43cae; continue; }
            pc = 0x43c8b;
        }
        0x43c8b => {
            // block 43c8b
            #[cfg(feature = "trace")] eprintln!("f_00043c62 pc 43c8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43c8b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43c8e mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 43c90 lea rdx,[rip+0xbc989] # 0x100620
            rdx = m.env.data_va(85485);
            // 43c97 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43c9a mov ecx,0x1fff0000
            rcx = (0x1fff0000u64) as u32 as u64;
            // 43c9f mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 43ca5 mov rdi,r15
            rdi = r15;
            // 43ca8 mov rdx,r14
            rdx = r14;
            // 43cab call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43cae;
        }
        0x43cae => {
            // block 43cae
            #[cfg(feature = "trace")] eprintln!("f_00043c62 pc 43cae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43cae add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43cb2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43cb3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43cb5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43cb7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43cb8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8ab2 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008ab2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8ab2;
    loop {
        match pc {
        0x8ab2 => {
            // block 8ab2
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8ab2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8ab2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8ab3 mov rbp,rsp
            rbp = rsp;
            // 8ab6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8ab7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 8ab8 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 8abb lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 8abf xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8ac1 mov esi,0x51
            rsi = (0x51u64) as u32 as u64;
            // 8ac6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8acb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8ace call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8ad1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8ad3 je 0x8b03
            if fl.e() { pc = 0x8b03; continue; }
            pc = 0x8ad5;
        }
        0x8ad5 => {
            // block 8ad5
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8ad5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8ad5 mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 8ad8 test ecx,0x10000
            fl = logic::<32>((rcx as u32 as u64) & 0x10000u64);
            // 8ade je 0x8b03
            if fl.e() { pc = 0x8b03; continue; }
            pc = 0x8ae0;
        }
        0x8ae0 => {
            // block 8ae0
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8ae0 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 8ae2 and eax,0x8000
            { let r = (rax as u32 as u64) & 0x8000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8ae7 test ecx,0x40000
            fl = logic::<32>((rcx as u32 as u64) & 0x40000u64);
            // 8aed jne 0x8af9
            if fl.ne() { pc = 0x8af9; continue; }
            pc = 0x8aef;
        }
        0x8aef => {
            // block 8aef
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8aef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8aef shr eax,0xa
            { let (r, f) = shr::<32>((rax as u32 as u64), 0xau64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8af2 xor eax,0x20
            { let r = (rax as u32 as u64) ^ 0x20u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8af5 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 8af7 jmp 0x8b03
            pc = 0x8b03; continue;
        }
        0x8af9 => {
            // block 8af9
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8af9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8af9 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 8afb mov ebx,0x21
            rbx = (0x21u64) as u32 as u64;
            // 8b00 cmove ebx,eax
            if fl.e() { rbx = ((rax as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            pc = 0x8b03;
        }
        0x8b03 => {
            // block 8b03
            #[cfg(feature = "trace")] eprintln!("f_00008ab2 pc 8b03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8b03 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 8b05 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 8b09 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8b0a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8b0b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe90c size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e90c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe90c;
    loop {
        match pc {
        0xe90c => {
            // block e90c
            #[cfg(feature = "trace")] eprintln!("f_0000e90c pc e90c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e90c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e90d mov rbp,rsp
            rbp = rsp;
            // e910 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e912 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e913 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e917 mov r14,rsi
            r14 = rsi;
            // e91a mov rbx,rdi
            rbx = rdi;
            // e91d mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // e924 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // e927 call 0xcc7a3
            { let r = { let t = m.env.code_va(0xcc7a3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e92e je 0xe956
            if fl.e() { pc = 0xe956; continue; }
            pc = 0xe930;
        }
        0xe930 => {
            // block e930
            #[cfg(feature = "trace")] eprintln!("f_0000e90c pc e930 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e930 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e933 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e937 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // e93c mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // e941 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // e947 mov rdi,rbx
            rdi = rbx;
            // e94a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e94d cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // e951 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // e954 jmp 0xe95a
            pc = 0xe95a; continue;
        }
        0xe956 => {
            // block e956
            #[cfg(feature = "trace")] eprintln!("f_0000e90c pc e956 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e956 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e958 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe95a;
        }
        0xe95a => {
            // block e95a
            #[cfg(feature = "trace")] eprintln!("f_0000e90c pc e95a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e95a mov BYTE PTR [r14],cl
            m.w8(r14, ((rcx as u8 as u64)) as u8);
            // e95d add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e961 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e962 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e964 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e965 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf2d0 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f2d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf2d0;
    loop {
        match pc {
        0xf2d0 => {
            // block f2d0
            #[cfg(feature = "trace")] eprintln!("f_0000f2d0 pc f2d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f2d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f2d1 mov rbp,rsp
            rbp = rsp;
            // f2d4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f2d6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f2d7 sub rsp,0x60
            { let (r, f) = sub::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // f2db mov rbx,rdi
            rbx = rdi;
            // f2de lea r14,[rbp-0x68]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // f2e2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // f2e5 mov rdi,r14
            rdi = r14;
            // f2e8 call 0x2ceb8
            { let r = { let t = m.env.code_va(0x2ceb8); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f2ed mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f2f0 mov rax,QWORD PTR [rax+0xd0]
            rax = m.r64(rax.wrapping_add(0xd0)) as u64;
            // f2f7 mov rdi,rbx
            rdi = rbx;
            // f2fa mov rsi,r14
            rsi = r14;
            // f2fd call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f2ff mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // f301 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // f305 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f30a mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // f30c add rsp,0x60
            { let (r, f) = add::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // f310 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f311 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f313 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f314 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xf315 => {
            // block f315
            #[cfg(feature = "trace")] eprintln!("f_0000f2d0 pc f315 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f315 mov rbx,rax
            rbx = rax;
            // f318 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // f31c call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f321 mov rdi,rbx
            rdi = rbx;
            // f324 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x437e0 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000437e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x437e0;
    loop {
        match pc {
        0x437e0 => {
            // block 437e0
            #[cfg(feature = "trace")] eprintln!("f_000437e0 pc 437e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 437e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 437e1 mov rbp,rsp
            rbp = rsp;
            // 437e4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 437e6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 437e8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 437e9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 437ea mov r14,rdx
            r14 = rdx;
            // 437ed mov r15,rsi
            r15 = rsi;
            // 437f0 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 437f7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 437fa lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 437fe mov esi,0xbe
            rsi = (0xbeu64) as u32 as u64;
            // 43803 mov ecx,0x3f0000
            rcx = (0x3f0000u64) as u32 as u64;
            // 43808 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 4380e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43811 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 43813 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // 43815 je 0x4382d
            if fl.e() { pc = 0x4382d; continue; }
            pc = 0x43817;
        }
        0x43817 => {
            // block 43817
            #[cfg(feature = "trace")] eprintln!("f_000437e0 pc 43817 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43817 mov edi,DWORD PTR [rbp-0x1c]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 4381a mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 4381c and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4381f mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            // 43822 shr edi,0x4
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x4u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // 43825 call 0x5c564
            { let r = { let t = m.env.code_va(0x5c564); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4382a mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            pc = 0x4382d;
        }
        0x4382d => {
            // block 4382d
            #[cfg(feature = "trace")] eprintln!("f_000437e0 pc 4382d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4382d mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 4382f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43833 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43834 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43836 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43838 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43839 ret
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
            #[cfg(feature = "trace")] eprintln!("f_000d8328 pc d8328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d8328 pc d8357 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d8328 pc d8376 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x5bd0 size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005bd0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5bd0;
    loop {
        match pc {
        0x5bd0 => {
            // block 5bd0
            #[cfg(feature = "trace")] eprintln!("f_00005bd0 pc 5bd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bd0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5bd1 mov rbp,rsp
            rbp = rsp;
            // 5bd4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5bd6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5bd8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5bd9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5bda mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 5bdd mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5be0 mov rbx,rdi
            rbx = rdi;
            // 5be3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5be6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5bec mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5bee call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5bf3 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 5bf6 cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 5bf9 jbe 0x5c1f
            if fl.be() { pc = 0x5c1f; continue; }
            pc = 0x5bfb;
        }
        0x5bfb => {
            // block 5bfb
            #[cfg(feature = "trace")] eprintln!("f_00005bd0 pc 5bfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5bfe movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 5c02 lea rdx,[rip+0xf9797] # 0xff3a0
            rdx = m.env.data_va(80749);
            // 5c09 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5c0c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 5c11 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5c14 mov rdi,rbx
            rdi = rbx;
            // 5c17 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 5c1a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5c1d jmp 0x5c21
            pc = 0x5c21; continue;
        }
        0x5c1f => {
            // block 5c1f
            #[cfg(feature = "trace")] eprintln!("f_00005bd0 pc 5c1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c1f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5c21;
        }
        0x5c21 => {
            // block 5c21
            #[cfg(feature = "trace")] eprintln!("f_00005bd0 pc 5c21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c21 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5c25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5c26 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5c28 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5c2a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c2b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf274 size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f274<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf274;
    loop {
        match pc {
        0xf274 => {
            // block f274
            #[cfg(feature = "trace")] eprintln!("f_0000f274 pc f274 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f274 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f275 mov rbp,rsp
            rbp = rsp;
            // f278 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f27a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f27b sub rsp,0x60
            { let (r, f) = sub::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // f27f mov rbx,rdi
            rbx = rdi;
            // f282 lea r14,[rbp-0x68]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // f286 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // f28c mov rdi,r14
            rdi = r14;
            // f28f call 0x2ceb8
            { let r = { let t = m.env.code_va(0x2ceb8); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f294 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f297 mov rax,QWORD PTR [rax+0xd0]
            rax = m.r64(rax.wrapping_add(0xd0)) as u64;
            // f29e mov rdi,rbx
            rdi = rbx;
            // f2a1 mov rsi,r14
            rsi = r14;
            // f2a4 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f2a6 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // f2a8 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // f2ac call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f2b1 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // f2b3 add rsp,0x60
            { let (r, f) = add::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // f2b7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f2b8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f2ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f2bb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xf2bc => {
            // block f2bc
            #[cfg(feature = "trace")] eprintln!("f_0000f274 pc f2bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f2bc mov rbx,rax
            rbx = rax;
            // f2bf lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // f2c3 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f2c8 mov rdi,rbx
            rdi = rbx;
            // f2cb call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42ae0 size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042ae0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42ae0;
    loop {
        match pc {
        0x42ae0 => {
            // block 42ae0
            #[cfg(feature = "trace")] eprintln!("f_00042ae0 pc 42ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42ae0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42ae1 mov rbp,rsp
            rbp = rsp;
            // 42ae4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42ae6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42ae8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42ae9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42aea mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 42aed mov rbx,rdi
            rbx = rdi;
            // 42af0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42af3 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42af9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42afb je 0x42b2e
            if fl.e() { pc = 0x42b2e; continue; }
            pc = 0x42afd;
        }
        0x42afd => {
            // block 42afd
            #[cfg(feature = "trace")] eprintln!("f_00042ae0 pc 42afd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42afd movzx r15d,r14w
            r15 = ((r14 as u16 as u64)) as u32 as u64;
            // 42b01 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 42b04 call 0xcf820
            { let r = { let t = m.env.code_va(0xcf820); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42b09 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 42b0c inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 42b0e cmp r15d,eax
            fl = sub::<32>((r15 as u32 as u64), (rax as u32 as u64)).1;
            // 42b11 jae 0x42b2e
            if fl.ae() { pc = 0x42b2e; continue; }
            pc = 0x42b13;
        }
        0x42b13 => {
            // block 42b13
            #[cfg(feature = "trace")] eprintln!("f_00042ae0 pc 42b13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42b13 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42b16 mov esi,0x900
            rsi = (0x900u64) as u32 as u64;
            // 42b1b mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // 42b20 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42b23 mov rdi,rbx
            rdi = rbx;
            // 42b26 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42b29 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42b2c jmp 0x42b30
            pc = 0x42b30; continue;
        }
        0x42b2e => {
            // block 42b2e
            #[cfg(feature = "trace")] eprintln!("f_00042ae0 pc 42b2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42b2e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42b30;
        }
        0x42b30 => {
            // block 42b30
            #[cfg(feature = "trace")] eprintln!("f_00042ae0 pc 42b30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42b30 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42b34 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42b35 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42b37 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42b39 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42b3a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd42bc size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d42bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd42bc;
    loop {
        match pc {
        0xd42bc => {
            // block d42bc
            #[cfg(feature = "trace")] eprintln!("f_000d42bc pc d42bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d42bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d42bd mov rbp,rsp
            rbp = rsp;
            // d42c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d42c2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d42c3 sub rsp,0x60
            { let (r, f) = sub::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // d42c7 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d42ca mov rbx,rdi
            rbx = rdi;
            // d42cd lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // d42d1 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d42d6 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // d42db call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d42e0 mov DWORD PTR [rbp-0x50],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u32);
            // d42e7 lea rdx,[rbp-0x6c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff94);
            // d42eb mov rdi,rbx
            rdi = rbx;
            // d42ee mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d42f1 call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d42f6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d42f8 je 0xd430c
            if fl.e() { pc = 0xd430c; continue; }
            pc = 0xd42fa;
        }
        0xd42fa => {
            // block d42fa
            #[cfg(feature = "trace")] eprintln!("f_000d42bc pc d42fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d42fa mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d42fd lea rsi,[rbp-0x70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // d4301 mov rdi,rbx
            rdi = rbx;
            // d4304 call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d430a jmp 0xd430e
            pc = 0xd430e; continue;
        }
        0xd430c => {
            // block d430c
            #[cfg(feature = "trace")] eprintln!("f_000d42bc pc d430c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d430c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd430e;
        }
        0xd430e => {
            // block d430e
            #[cfg(feature = "trace")] eprintln!("f_000d42bc pc d430e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d430e add rsp,0x60
            { let (r, f) = add::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // d4312 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4313 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4315 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4316 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42cce size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042cce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42cce;
    loop {
        match pc {
        0x42cce => {
            // block 42cce
            #[cfg(feature = "trace")] eprintln!("f_00042cce pc 42cce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42cce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42ccf mov rbp,rsp
            rbp = rsp;
            // 42cd2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42cd4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42cd6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42cd7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42cd8 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 42cdb mov rbx,rdi
            rbx = rdi;
            // 42cde mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42ce1 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42ce7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42ce9 je 0x42d1f
            if fl.e() { pc = 0x42d1f; continue; }
            pc = 0x42ceb;
        }
        0x42ceb => {
            // block 42ceb
            #[cfg(feature = "trace")] eprintln!("f_00042cce pc 42ceb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42ceb movzx r15d,r14w
            r15 = ((r14 as u16 as u64)) as u32 as u64;
            // 42cef mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 42cf2 call 0xcf820
            { let r = { let t = m.env.code_va(0xcf820); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42cf7 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 42cfa inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 42cfc cmp r15d,eax
            fl = sub::<32>((r15 as u32 as u64), (rax as u32 as u64)).1;
            // 42cff jae 0x42d1f
            if fl.ae() { pc = 0x42d1f; continue; }
            pc = 0x42d01;
        }
        0x42d01 => {
            // block 42d01
            #[cfg(feature = "trace")] eprintln!("f_00042cce pc 42d01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42d01 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42d04 mov esi,0x900
            rsi = (0x900u64) as u32 as u64;
            // 42d09 mov ecx,0x70
            rcx = (0x70u64) as u32 as u64;
            // 42d0e mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // 42d14 mov rdi,rbx
            rdi = rbx;
            // 42d17 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42d1a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42d1d jmp 0x42d21
            pc = 0x42d21; continue;
        }
        0x42d1f => {
            // block 42d1f
            #[cfg(feature = "trace")] eprintln!("f_00042cce pc 42d1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42d1f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42d21;
        }
        0x42d21 => {
            // block 42d21
            #[cfg(feature = "trace")] eprintln!("f_00042cce pc 42d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42d21 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42d25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42d26 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42d28 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42d2a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42d2b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42e5a size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042e5a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42e5a;
    loop {
        match pc {
        0x42e5a => {
            // block 42e5a
            #[cfg(feature = "trace")] eprintln!("f_00042e5a pc 42e5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42e5a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42e5b mov rbp,rsp
            rbp = rsp;
            // 42e5e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42e60 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42e62 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42e63 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42e64 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 42e67 mov rbx,rdi
            rbx = rdi;
            // 42e6a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42e6d call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42e73 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42e75 je 0x42eab
            if fl.e() { pc = 0x42eab; continue; }
            pc = 0x42e77;
        }
        0x42e77 => {
            // block 42e77
            #[cfg(feature = "trace")] eprintln!("f_00042e5a pc 42e77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42e77 movzx r15d,r14w
            r15 = ((r14 as u16 as u64)) as u32 as u64;
            // 42e7b mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 42e7e call 0xcf820
            { let r = { let t = m.env.code_va(0xcf820); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42e83 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 42e86 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 42e88 cmp r15d,eax
            fl = sub::<32>((r15 as u32 as u64), (rax as u32 as u64)).1;
            // 42e8b jae 0x42eab
            if fl.ae() { pc = 0x42eab; continue; }
            pc = 0x42e8d;
        }
        0x42e8d => {
            // block 42e8d
            #[cfg(feature = "trace")] eprintln!("f_00042e5a pc 42e8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42e8d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42e90 mov esi,0x900
            rsi = (0x900u64) as u32 as u64;
            // 42e95 mov ecx,0x700
            rcx = (0x700u64) as u32 as u64;
            // 42e9a mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // 42ea0 mov rdi,rbx
            rdi = rbx;
            // 42ea3 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42ea6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42ea9 jmp 0x42ead
            pc = 0x42ead; continue;
        }
        0x42eab => {
            // block 42eab
            #[cfg(feature = "trace")] eprintln!("f_00042e5a pc 42eab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42eab xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42ead;
        }
        0x42ead => {
            // block 42ead
            #[cfg(feature = "trace")] eprintln!("f_00042e5a pc 42ead rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42ead add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42eb1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42eb2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42eb4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42eb6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42eb7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43040 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043040<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43040;
    loop {
        match pc {
        0x43040 => {
            // block 43040
            #[cfg(feature = "trace")] eprintln!("f_00043040 pc 43040 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43040 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43041 mov rbp,rsp
            rbp = rsp;
            // 43044 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43046 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43047 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4304b mov r14,rsi
            r14 = rsi;
            // 4304e mov rbx,rdi
            rbx = rdi;
            // 43051 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 43054 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4305a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4305c je 0x43092
            if fl.e() { pc = 0x43092; continue; }
            pc = 0x4305e;
        }
        0x4305e => {
            // block 4305e
            #[cfg(feature = "trace")] eprintln!("f_00043040 pc 4305e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4305e mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 43065 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 43068 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 4306c mov esi,0x905
            rsi = (0x905u64) as u32 as u64;
            // 43071 mov ecx,0x30000
            rcx = (0x30000u64) as u32 as u64;
            // 43076 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 4307c mov rdi,rbx
            rdi = rbx;
            // 4307f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43082 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43084 je 0x43092
            if fl.e() { pc = 0x43092; continue; }
            pc = 0x43086;
        }
        0x43086 => {
            // block 43086
            #[cfg(feature = "trace")] eprintln!("f_00043040 pc 43086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43086 cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // 4308a sete BYTE PTR [r14]
            m.w8(r14, ((fl.e() as u64)) as u8);
            // 4308e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 43090 jmp 0x43094
            pc = 0x43094; continue;
        }
        0x43092 => {
            // block 43092
            #[cfg(feature = "trace")] eprintln!("f_00043040 pc 43092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43092 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x43094;
        }
        0x43094 => {
            // block 43094
            #[cfg(feature = "trace")] eprintln!("f_00043040 pc 43094 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43094 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43098 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43099 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4309b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4309c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4318c size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004318c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4318c;
    loop {
        match pc {
        0x4318c => {
            // block 4318c
            #[cfg(feature = "trace")] eprintln!("f_0004318c pc 4318c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4318c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4318d mov rbp,rsp
            rbp = rsp;
            // 43190 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43192 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43193 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43197 mov r14,rsi
            r14 = rsi;
            // 4319a mov rbx,rdi
            rbx = rdi;
            // 4319d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 431a0 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 431a6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 431a8 je 0x431de
            if fl.e() { pc = 0x431de; continue; }
            pc = 0x431aa;
        }
        0x431aa => {
            // block 431aa
            #[cfg(feature = "trace")] eprintln!("f_0004318c pc 431aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 431aa mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 431b1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 431b4 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 431b8 mov esi,0x905
            rsi = (0x905u64) as u32 as u64;
            // 431bd mov ecx,0x300000
            rcx = (0x300000u64) as u32 as u64;
            // 431c2 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 431c8 mov rdi,rbx
            rdi = rbx;
            // 431cb call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 431ce test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 431d0 je 0x431de
            if fl.e() { pc = 0x431de; continue; }
            pc = 0x431d2;
        }
        0x431d2 => {
            // block 431d2
            #[cfg(feature = "trace")] eprintln!("f_0004318c pc 431d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 431d2 cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // 431d6 sete BYTE PTR [r14]
            m.w8(r14, ((fl.e() as u64)) as u8);
            // 431da mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 431dc jmp 0x431e0
            pc = 0x431e0; continue;
        }
        0x431de => {
            // block 431de
            #[cfg(feature = "trace")] eprintln!("f_0004318c pc 431de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 431de xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x431e0;
        }
        0x431e0 => {
            // block 431e0
            #[cfg(feature = "trace")] eprintln!("f_0004318c pc 431e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 431e0 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 431e4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 431e5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 431e7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 431e8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7836 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7836<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb7836;
    loop {
        match pc {
        0xb7836 => {
            // block b7836
            #[cfg(feature = "trace")] eprintln!("f_000b7836 pc b7836 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7836 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7837 mov rbp,rsp
            rbp = rsp;
            // b783a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b783c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b783e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b783f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b7840 mov r14,rcx
            r14 = rcx;
            // b7843 mov r15,rdx
            r15 = rdx;
            // b7846 mov rbx,rsi
            rbx = rsi;
            // b7849 mov WORD PTR [r8],0x0
            m.w16(r8, (0x0u64) as u16);
            // b784f mov WORD PTR [r14],0x0
            m.w16(r14, (0x0u64) as u16);
            // b7855 mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // b785b mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // b7860 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // b7867 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // b786b call 0x480fa
            { let r = { let t = m.env.code_va(0x480fa); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7870 movzx eax,BYTE PTR [rbp-0x19]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe7)) as u64) as u32 as u64;
            // b7874 mov WORD PTR [rbx],ax
            m.w16(rbx, ((rax as u16 as u64)) as u16);
            // b7877 movzx eax,BYTE PTR [rbp-0x1a]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6)) as u64) as u32 as u64;
            // b787b mov WORD PTR [r15],ax
            m.w16(r15, ((rax as u16 as u64)) as u16);
            // b787f movzx eax,BYTE PTR [rbp-0x1b]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe5)) as u64) as u32 as u64;
            // b7883 mov WORD PTR [r14],ax
            m.w16(r14, ((rax as u16 as u64)) as u16);
            // b7887 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // b7889 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b788d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b788e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7890 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b7892 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7893 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x586c size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000586c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x586c;
    loop {
        match pc {
        0x586c => {
            // block 586c
            #[cfg(feature = "trace")] eprintln!("f_0000586c pc 586c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 586d mov rbp,rsp
            rbp = rsp;
            // 5870 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5872 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5874 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5875 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5876 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 5879 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 587c mov rbx,rdi
            rbx = rdi;
            // 587f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5882 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5888 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 588a call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 588f movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 5892 cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 5895 jbe 0x58bf
            if fl.be() { pc = 0x58bf; continue; }
            pc = 0x5897;
        }
        0x5897 => {
            // block 5897
            #[cfg(feature = "trace")] eprintln!("f_0000586c pc 5897 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5897 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 589a movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 589e lea rdx,[rip+0xf9adb] # 0xff380
            rdx = m.env.data_va(80717);
            // 58a5 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 58a8 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // 58ac mov ecx,0x2000
            rcx = (0x2000u64) as u32 as u64;
            // 58b1 mov r8d,0xd
            r8 = (0xdu64) as u32 as u64;
            // 58b7 mov rdi,rbx
            rdi = rbx;
            // 58ba call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58bd jmp 0x58c1
            pc = 0x58c1; continue;
        }
        0x58bf => {
            // block 58bf
            #[cfg(feature = "trace")] eprintln!("f_0000586c pc 58bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58bf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58c1;
        }
        0x58c1 => {
            // block 58c1
            #[cfg(feature = "trace")] eprintln!("f_0000586c pc 58c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58c1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 58c5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58c6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58c8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58ca pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58cb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5942 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005942<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5942;
    loop {
        match pc {
        0x5942 => {
            // block 5942
            #[cfg(feature = "trace")] eprintln!("f_00005942 pc 5942 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5942 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5943 mov rbp,rsp
            rbp = rsp;
            // 5946 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5948 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 594a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 594b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 594c mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 594f mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5952 mov rbx,rdi
            rbx = rdi;
            // 5955 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5958 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 595e mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5960 call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5965 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 5968 cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 596b jbe 0x5994
            if fl.be() { pc = 0x5994; continue; }
            pc = 0x596d;
        }
        0x596d => {
            // block 596d
            #[cfg(feature = "trace")] eprintln!("f_00005942 pc 596d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 596d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5970 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 5974 lea rdx,[rip+0xf9a05] # 0xff380
            rdx = m.env.data_va(80717);
            // 597b mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 597e mov ecx,0x300000
            rcx = (0x300000u64) as u32 as u64;
            // 5983 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 5989 mov rdi,rbx
            rdi = rbx;
            // 598c mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 598f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5992 jmp 0x5996
            pc = 0x5996; continue;
        }
        0x5994 => {
            // block 5994
            #[cfg(feature = "trace")] eprintln!("f_00005942 pc 5994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5994 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5996;
        }
        0x5996 => {
            // block 5996
            #[cfg(feature = "trace")] eprintln!("f_00005942 pc 5996 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5996 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 599a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 599b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 599d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 599f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 59a0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a1c size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005a1c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5a1c;
    loop {
        match pc {
        0x5a1c => {
            // block 5a1c
            #[cfg(feature = "trace")] eprintln!("f_00005a1c pc 5a1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a1c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a1d mov rbp,rsp
            rbp = rsp;
            // 5a20 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5a22 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5a24 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5a25 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5a26 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 5a29 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5a2c mov rbx,rdi
            rbx = rdi;
            // 5a2f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5a32 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5a38 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5a3a call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5a3f movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 5a42 cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 5a45 jbe 0x5a6e
            if fl.be() { pc = 0x5a6e; continue; }
            pc = 0x5a47;
        }
        0x5a47 => {
            // block 5a47
            #[cfg(feature = "trace")] eprintln!("f_00005a1c pc 5a47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a47 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5a4a movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 5a4e lea rdx,[rip+0xf992b] # 0xff380
            rdx = m.env.data_va(80717);
            // 5a55 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5a58 mov ecx,0xc00000
            rcx = (0xc00000u64) as u32 as u64;
            // 5a5d mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 5a63 mov rdi,rbx
            rdi = rbx;
            // 5a66 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 5a69 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5a6c jmp 0x5a70
            pc = 0x5a70; continue;
        }
        0x5a6e => {
            // block 5a6e
            #[cfg(feature = "trace")] eprintln!("f_00005a1c pc 5a6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a6e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5a70;
        }
        0x5a70 => {
            // block 5a70
            #[cfg(feature = "trace")] eprintln!("f_00005a1c pc 5a70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a70 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5a74 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5a75 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5a77 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5a79 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a7a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5af6 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005af6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5af6;
    loop {
        match pc {
        0x5af6 => {
            // block 5af6
            #[cfg(feature = "trace")] eprintln!("f_00005af6 pc 5af6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5af6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5af7 mov rbp,rsp
            rbp = rsp;
            // 5afa push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5afc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5afe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5aff push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5b00 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 5b03 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5b06 mov rbx,rdi
            rbx = rdi;
            // 5b09 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5b0c call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b12 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5b14 call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b19 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 5b1c cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 5b1f jbe 0x5b48
            if fl.be() { pc = 0x5b48; continue; }
            pc = 0x5b21;
        }
        0x5b21 => {
            // block 5b21
            #[cfg(feature = "trace")] eprintln!("f_00005af6 pc 5b21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b21 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5b24 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 5b28 lea rdx,[rip+0xf9851] # 0xff380
            rdx = m.env.data_va(80717);
            // 5b2f mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5b32 mov ecx,0x3000000
            rcx = (0x3000000u64) as u32 as u64;
            // 5b37 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // 5b3d mov rdi,rbx
            rdi = rbx;
            // 5b40 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 5b43 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5b46 jmp 0x5b4a
            pc = 0x5b4a; continue;
        }
        0x5b48 => {
            // block 5b48
            #[cfg(feature = "trace")] eprintln!("f_00005af6 pc 5b48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b48 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5b4a;
        }
        0x5b4a => {
            // block 5b4a
            #[cfg(feature = "trace")] eprintln!("f_00005af6 pc 5b4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b4a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5b4e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5b4f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5b51 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5b53 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5b54 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8b0c size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008b0c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8b0c;
    loop {
        match pc {
        0x8b0c => {
            // block 8b0c
            #[cfg(feature = "trace")] eprintln!("f_00008b0c pc 8b0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8b0c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8b0d mov rbp,rsp
            rbp = rsp;
            // 8b10 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8b12 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8b13 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8b17 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 8b1a lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 8b1e call QWORD PTR [rax+0xa40]
            { let r = { let t = m.r64(rax.wrapping_add(0xa40)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8b24 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 8b26 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8b28 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 8b2a je 0x8b63
            if fl.e() { pc = 0x8b63; continue; }
            pc = 0x8b2c;
        }
        0x8b2c => {
            // block 8b2c
            #[cfg(feature = "trace")] eprintln!("f_00008b0c pc 8b2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8b2c mov esi,DWORD PTR [rbp-0x14]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 8b2f mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 8b32 shr r14d,0x10
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x10u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 8b36 and r14d,0xf
            { let r = (r14 as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 8b3a mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 8b3c shr ebx,0x14
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x14u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 8b3f and esi,0x800000
            { let r = (rsi as u32 as u64) & 0x800000u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8b45 shr esi,0x17
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x17u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 8b48 and ebx,0x7
            { let r = (rbx as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8b4b mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // 8b4d call 0x5a594
            { let r = { let t = m.env.code_va(0x5a594); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8b52 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 8b54 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8b57 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // 8b5a mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 8b5c mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 8b5e call 0x6d8
            { let r = { let t = m.env.code_va(0x6d8); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x8b63;
        }
        0x8b63 => {
            // block 8b63
            #[cfg(feature = "trace")] eprintln!("f_00008b0c pc 8b63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8b63 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8b67 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8b68 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8b6a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8b6b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2cf2e size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002cf2e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2cf2e;
    loop {
        match pc {
        0x2cf2e => {
            // block 2cf2e
            #[cfg(feature = "trace")] eprintln!("f_0002cf2e pc 2cf2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cf2e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2cf2f mov rbp,rsp
            rbp = rsp;
            // 2cf32 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2cf34 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2cf35 mov r14,rsi
            r14 = rsi;
            // 2cf38 mov rbx,rdi
            rbx = rdi;
            // 2cf3b call 0x26172
            { let r = { let t = m.env.code_va(0x26172); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cf40 lea rsi,[rip+0xfda80] # 0x12a9c7
            rsi = m.env.data_va(258452);
            // 2cf47 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 2cf4c mov rdi,rax
            rdi = rax;
            // 2cf4f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cf54 mov esi,DWORD PTR [rbx+0x20]
            rsi = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2cf57 mov rdi,rax
            rdi = rax;
            // 2cf5a call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cf5f lea rsi,[rip+0xfda69] # 0x12a9cf
            rsi = m.env.data_va(258460);
            // 2cf66 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 2cf6b mov rdi,rax
            rdi = rax;
            // 2cf6e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cf73 mov esi,DWORD PTR [rbx+0x24]
            rsi = (m.r32(rbx.wrapping_add(0x24)) as u64) as u32 as u64;
            // 2cf76 mov rdi,rax
            rdi = rax;
            // 2cf79 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cf7e mov rax,r14
            rax = r14;
            // 2cf81 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2cf82 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2cf84 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2cf85 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd43e4 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d43e4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd43e4;
    loop {
        match pc {
        0xd43e4 => {
            // block d43e4
            #[cfg(feature = "trace")] eprintln!("f_000d43e4 pc d43e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d43e4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d43e5 mov rbp,rsp
            rbp = rsp;
            // d43e8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d43ea push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d43ec push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d43ed sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d43f1 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d43f4 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d43f7 mov rbx,rdi
            rbx = rdi;
            // d43fa lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d43fe mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // d4403 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // d4408 call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d440d mov DWORD PTR [rbp-0x70],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), ((r14 as u32 as u64)) as u32);
            // d4411 lea rdx,[rbp-0x74]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // d4415 mov rdi,rbx
            rdi = rbx;
            // d4418 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d441b call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4420 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d4422 je 0xd4436
            if fl.e() { pc = 0xd4436; continue; }
            pc = 0xd4424;
        }
        0xd4424 => {
            // block d4424
            #[cfg(feature = "trace")] eprintln!("f_000d43e4 pc d4424 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4424 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d4427 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d442b mov rdi,rbx
            rdi = rbx;
            // d442e call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4434 jmp 0xd4438
            pc = 0xd4438; continue;
        }
        0xd4436 => {
            // block d4436
            #[cfg(feature = "trace")] eprintln!("f_000d43e4 pc d4436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4436 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd4438;
        }
        0xd4438 => {
            // block d4438
            #[cfg(feature = "trace")] eprintln!("f_000d43e4 pc d4438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4438 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d443c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d443d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d443f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d4441 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4442 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4630 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4630<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4630;
    loop {
        match pc {
        0xd4630 => {
            // block d4630
            #[cfg(feature = "trace")] eprintln!("f_000d4630 pc d4630 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4630 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4631 mov rbp,rsp
            rbp = rsp;
            // d4634 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4636 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4638 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4639 sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d463d mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d4640 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d4643 mov rbx,rdi
            rbx = rdi;
            // d4646 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d464a mov esi,0xf
            rsi = (0xfu64) as u32 as u64;
            // d464f mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // d4654 call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4659 mov DWORD PTR [rbp-0x70],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), ((r14 as u32 as u64)) as u32);
            // d465d lea rdx,[rbp-0x74]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // d4661 mov rdi,rbx
            rdi = rbx;
            // d4664 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d4667 call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d466c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d466e je 0xd4682
            if fl.e() { pc = 0xd4682; continue; }
            pc = 0xd4670;
        }
        0xd4670 => {
            // block d4670
            #[cfg(feature = "trace")] eprintln!("f_000d4630 pc d4670 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4670 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d4673 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d4677 mov rdi,rbx
            rdi = rbx;
            // d467a call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4680 jmp 0xd4684
            pc = 0xd4684; continue;
        }
        0xd4682 => {
            // block d4682
            #[cfg(feature = "trace")] eprintln!("f_000d4630 pc d4682 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4682 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd4684;
        }
        0xd4684 => {
            // block d4684
            #[cfg(feature = "trace")] eprintln!("f_000d4630 pc d4684 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4684 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d4688 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4689 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d468b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d468d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d468e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29ec6 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029ec6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x29ec6;
    loop {
        match pc {
        0x29ec6 => {
            // block 29ec6
            #[cfg(feature = "trace")] eprintln!("f_00029ec6 pc 29ec6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29ec6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29ec7 mov rbp,rsp
            rbp = rsp;
            // 29eca push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 29ecc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 29ece push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29ecf push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 29ed0 mov r14,rsi
            r14 = rsi;
            // 29ed3 mov rbx,rdi
            rbx = rdi;
            // 29ed6 call 0x26172
            { let r = { let t = m.env.code_va(0x26172); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29edb lea r15,[rip+0x1005bb] # 0x12a49d
            r15 = m.env.data_va(257130);
            // 29ee2 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 29ee7 mov rdi,rax
            rdi = rax;
            // 29eea mov rsi,r15
            rsi = r15;
            // 29eed call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29ef2 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 29ef6 mov rsi,rax
            rsi = rax;
            // 29ef9 call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29efe mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 29f03 mov rdi,rax
            rdi = rax;
            // 29f06 mov rsi,r15
            rsi = r15;
            // 29f09 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29f0e lea rsi,[rbx+0x48]
            rsi = rbx.wrapping_add(0x48);
            // 29f12 mov rdi,rax
            rdi = rax;
            // 29f15 call 0x26804
            { let r = { let t = m.env.code_va(0x26804); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29f1a mov rax,r14
            rax = r14;
            // 29f1d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 29f21 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29f22 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 29f24 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 29f26 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29f27 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42b3c size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042b3c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42b3c;
    loop {
        match pc {
        0x42b3c => {
            // block 42b3c
            #[cfg(feature = "trace")] eprintln!("f_00042b3c pc 42b3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42b3c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42b3d mov rbp,rsp
            rbp = rsp;
            // 42b40 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42b42 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42b44 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42b45 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42b46 mov r14,rsi
            r14 = rsi;
            // 42b49 mov rbx,rdi
            rbx = rdi;
            // 42b4c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42b4f call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42b55 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42b57 je 0x42b8c
            if fl.e() { pc = 0x42b8c; continue; }
            pc = 0x42b59;
        }
        0x42b59 => {
            // block 42b59
            #[cfg(feature = "trace")] eprintln!("f_00042b3c pc 42b59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42b59 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 42b60 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42b63 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 42b67 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42b6a mov esi,0x904
            rsi = (0x904u64) as u32 as u64;
            // 42b6f mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // 42b74 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42b77 mov rdi,rbx
            rdi = rbx;
            // 42b7a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42b7d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42b7f je 0x42b8f
            if fl.e() { pc = 0x42b8f; continue; }
            pc = 0x42b81;
        }
        0x42b81 => {
            // block 42b81
            #[cfg(feature = "trace")] eprintln!("f_00042b3c pc 42b81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42b81 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 42b84 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 42b87 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 42b8a jmp 0x42b8f
            pc = 0x42b8f; continue;
        }
        0x42b8c => {
            // block 42b8c
            #[cfg(feature = "trace")] eprintln!("f_00042b3c pc 42b8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42b8c xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x42b8f;
        }
        0x42b8f => {
            // block 42b8f
            #[cfg(feature = "trace")] eprintln!("f_00042b3c pc 42b8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42b8f mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 42b92 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42b96 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42b97 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42b99 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42b9b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42b9c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42df8 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042df8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42df8;
    loop {
        match pc {
        0x42df8 => {
            // block 42df8
            #[cfg(feature = "trace")] eprintln!("f_00042df8 pc 42df8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42df8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42df9 mov rbp,rsp
            rbp = rsp;
            // 42dfc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42dfe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42dff sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 42e03 mov r14,rsi
            r14 = rsi;
            // 42e06 mov rbx,rdi
            rbx = rdi;
            // 42e09 mov DWORD PTR [r14],0x8
            m.w32(r14, (0x8u64) as u32);
            // 42e10 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42e13 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42e19 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42e1b je 0x42e4f
            if fl.e() { pc = 0x42e4f; continue; }
            pc = 0x42e1d;
        }
        0x42e1d => {
            // block 42e1d
            #[cfg(feature = "trace")] eprintln!("f_00042df8 pc 42e1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42e1d mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 42e24 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42e27 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 42e2b mov esi,0x900
            rsi = (0x900u64) as u32 as u64;
            // 42e30 mov ecx,0x700
            rcx = (0x700u64) as u32 as u64;
            // 42e35 mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // 42e3b mov rdi,rbx
            rdi = rbx;
            // 42e3e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42e41 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42e43 je 0x42e4f
            if fl.e() { pc = 0x42e4f; continue; }
            pc = 0x42e45;
        }
        0x42e45 => {
            // block 42e45
            #[cfg(feature = "trace")] eprintln!("f_00042df8 pc 42e45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42e45 mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 42e48 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 42e4b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 42e4d jmp 0x42e51
            pc = 0x42e51; continue;
        }
        0x42e4f => {
            // block 42e4f
            #[cfg(feature = "trace")] eprintln!("f_00042df8 pc 42e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42e4f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42e51;
        }
        0x42e51 => {
            // block 42e51
            #[cfg(feature = "trace")] eprintln!("f_00042df8 pc 42e51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42e51 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 42e55 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42e56 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42e58 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42e59 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4e4e size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4e4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4e4e;
    loop {
        match pc {
        0xc4e4e => {
            // block c4e4e
            #[cfg(feature = "trace")] eprintln!("f_000c4e4e pc c4e4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4e4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4e4f mov rbp,rsp
            rbp = rsp;
            // c4e52 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c4e54 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4e56 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c4e58 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4e59 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c4e5d mov r15,rdx
            r15 = rdx;
            // c4e60 mov r14,rdi
            r14 = rdi;
            // c4e63 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c4e67 mov rcx,r15
            rcx = r15;
            // c4e6a call 0xebd86
            { let r = { let t = m.env.stub_va(0x553); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4e6f mov r12,rax
            r12 = rax;
            // c4e72 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // c4e76 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c4e79 jne 0xc4ea0
            if fl.ne() { pc = 0xc4ea0; continue; }
            pc = 0xc4e7b;
        }
        0xc4e7b => {
            // block c4e7b
            #[cfg(feature = "trace")] eprintln!("f_000c4e4e pc c4e7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4e7b mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // c4e80 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4e85 mov rbx,rax
            rbx = rax;
            // c4e88 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // c4e8b mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c4e8e mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c4e92 mov rdi,r14
            rdi = r14;
            // c4e95 mov rdx,r12
            rdx = r12;
            // c4e98 mov rcx,rbx
            rcx = rbx;
            // c4e9b call 0xebd9e
            { let r = { let t = m.env.stub_va(0x56b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc4ea0;
        }
        0xc4ea0 => {
            // block c4ea0
            #[cfg(feature = "trace")] eprintln!("f_000c4e4e pc c4ea0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ea0 mov rax,rbx
            rax = rbx;
            // c4ea3 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c4ea7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4ea8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c4eaa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4eac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c4eae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4eaf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4382 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4382<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4382;
    loop {
        match pc {
        0xd4382 => {
            // block d4382
            #[cfg(feature = "trace")] eprintln!("f_000d4382 pc d4382 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4382 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4383 mov rbp,rsp
            rbp = rsp;
            // d4386 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4388 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d438a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d438b sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d438f mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d4392 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d4395 mov rbx,rdi
            rbx = rdi;
            // d4398 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d439c mov esi,0x6
            rsi = (0x6u64) as u32 as u64;
            // d43a1 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // d43a6 call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d43ab movzx eax,r14b
            rax = ((r14 as u8 as u64)) as u32 as u64;
            // d43af mov DWORD PTR [rbp-0x58],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((rax as u32 as u64)) as u32);
            // d43b2 lea rdx,[rbp-0x74]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // d43b6 mov rdi,rbx
            rdi = rbx;
            // d43b9 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d43bc call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d43c1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d43c3 je 0xd43d7
            if fl.e() { pc = 0xd43d7; continue; }
            pc = 0xd43c5;
        }
        0xd43c5 => {
            // block d43c5
            #[cfg(feature = "trace")] eprintln!("f_000d4382 pc d43c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d43c5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d43c8 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d43cc mov rdi,rbx
            rdi = rbx;
            // d43cf call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d43d5 jmp 0xd43d9
            pc = 0xd43d9; continue;
        }
        0xd43d7 => {
            // block d43d7
            #[cfg(feature = "trace")] eprintln!("f_000d4382 pc d43d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d43d7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd43d9;
        }
        0xd43d9 => {
            // block d43d9
            #[cfg(feature = "trace")] eprintln!("f_000d4382 pc d43d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d43d9 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d43dd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d43de pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d43e0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d43e2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d43e3 ret
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
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0dc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0df3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0df3 cmp r14d,0x2
            fl = sub::<32>((r14 as u32 as u64), 0x2u64).1;
            // e0df7 jne 0xe0e01
            if fl.ne() { pc = 0xe0e01; continue; }
            pc = 0xe0df9;
        }
        0xe0df9 => {
            // block e0df9
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0df9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0df9 or esi,0x200
            { let r = (rsi as u32 as u64) | 0x200u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0dff jmp 0xe0e0d
            pc = 0xe0e0d; continue;
        }
        0xe0e01 => {
            // block e0e01
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0e01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e01 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // e0e05 jne 0xe0e10
            if fl.ne() { pc = 0xe0e10; continue; }
            pc = 0xe0e07;
        }
        0xe0e07 => {
            // block e0e07
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0e07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e07 or esi,0x100
            { let r = (rsi as u32 as u64) | 0x100u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xe0e0d;
        }
        0xe0e0d => {
            // block e0e0d
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0e0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e0d mov DWORD PTR [rbp-0x14],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rsi as u32 as u64)) as u32);
            pc = 0xe0e10;
        }
        0xe0e10 => {
            // block e0e10
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0e10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0e1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xe47f2 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e47f2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe47f2;
    loop {
        match pc {
        0xe47f2 => {
            // block e47f2
            #[cfg(feature = "trace")] eprintln!("f_000e47f2 pc e47f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e47f2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e47f3 mov rbp,rsp
            rbp = rsp;
            // e47f6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e47f7 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // e47fb mov rbx,rdx
            rbx = rdx;
            // e47fe mov DWORD PTR [rbp-0x18],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), ((rsi as u32 as u64)) as u32);
            // e4801 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // e4808 mov DWORD PTR [rbp-0x10],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff0), ((rcx as u32 as u64)) as u32);
            // e480b mov DWORD PTR [rbp-0xc],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((r8 as u32 as u64)) as u32);
            // e480f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e4812 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e4815 mov rcx,QWORD PTR [rcx+0xc98]
            rcx = m.r64(rcx.wrapping_add(0xc98)) as u64;
            // e481c mov edx,DWORD PTR [rdi+0x28]
            rdx = (m.r32(rdi.wrapping_add(0x28)) as u64) as u32 as u64;
            // e481f mov esi,DWORD PTR [rdi+0x2c]
            rsi = (m.r32(rdi.wrapping_add(0x2c)) as u64) as u32 as u64;
            // e4822 mov DWORD PTR [rbp-0x28],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), ((rdx as u32 as u64)) as u32);
            // e4825 mov DWORD PTR [rbp-0x24],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rsi as u32 as u64)) as u32);
            // e4828 mov DWORD PTR [rbp-0x20],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0xffffffffu64) as u32);
            // e482f mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e4836 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // e483a lea rdx,[rbp-0x18]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // e483e mov rdi,rax
            rdi = rax;
            // e4841 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4843 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e4845 je 0xe484c
            if fl.e() { pc = 0xe484c; continue; }
            pc = 0xe4847;
        }
        0xe4847 => {
            // block e4847
            #[cfg(feature = "trace")] eprintln!("f_000e47f2 pc e4847 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4847 mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // e484a mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            pc = 0xe484c;
        }
        0xe484c => {
            // block e484c
            #[cfg(feature = "trace")] eprintln!("f_000e47f2 pc e484c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e484c add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // e4850 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4851 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4852 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe6182 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e6182<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe6182;
    loop {
        match pc {
        0xe6182 => {
            // block e6182
            #[cfg(feature = "trace")] eprintln!("f_000e6182 pc e6182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6182 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e6183 mov rbp,rsp
            rbp = rsp;
            // e6186 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e6188 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e6189 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e618d mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e6190 mov rbx,rdi
            rbx = rdi;
            // e6193 mov ecx,DWORD PTR [rbx+0x5c]
            rcx = (m.r32(rbx.wrapping_add(0x5c)) as u64) as u32 as u64;
            // e6196 mov r8d,DWORD PTR [rbx+0x58]
            r8 = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // e619a mov r9d,DWORD PTR [rbx+0x54]
            r9 = (m.r32(rbx.wrapping_add(0x54)) as u64) as u32 as u64;
            // e619e mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // e61a1 mov eax,DWORD PTR [rbx+0x50]
            rax = (m.r32(rbx.wrapping_add(0x50)) as u64) as u32 as u64;
            // e61a4 mov DWORD PTR [rsp],eax
            m.w32(rsp, ((rax as u32 as u64)) as u32);
            // e61a7 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e61ab call 0xe605e
            { let r = { let t = m.env.code_va(0xe605e); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e61b0 add r14d,DWORD PTR [rbp-0x14]
            { let (r, f) = add::<32>((r14 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e61b4 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e61b6 mov rdi,rbx
            rdi = rbx;
            // e61b9 call 0xe61e4
            { let r = { let t = m.env.code_va(0xe61e4); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e61be mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // e61c0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e61c2 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // e61c5 div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // e61c7 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // e61ca mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // e61cd mov rdi,rbx
            rdi = rbx;
            // e61d0 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // e61d3 call 0xe54b0
            { let r = { let t = m.env.code_va(0xe54b0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e61d8 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // e61db add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e61df pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e61e0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e61e2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e61e3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2046 size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002046<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2046;
    loop {
        match pc {
        0x2046 => {
            // block 2046
            #[cfg(feature = "trace")] eprintln!("f_00002046 pc 2046 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2046 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2047 mov rbp,rsp
            rbp = rsp;
            // 204a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 204c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 204e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2050 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2051 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // 2054 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // 2057 mov rbx,rdi
            rbx = rdi;
            // 205a mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 205d mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // 205f mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 2062 call 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2067 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 206a mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 206d mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 2070 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 2073 call 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2078 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 207b mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 207e call 0xcb406
            { let r = { let t = m.env.code_va(0xcb406); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2083 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2086 cmp r12d,r14d
            fl = sub::<32>((r12 as u32 as u64), (r14 as u32 as u64)).1;
            // 2089 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 208c and r15b,al
            { let r = (r15 as u8 as u64) & (rax as u8 as u64); let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 208f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2092 mov rdi,rbx
            rdi = rbx;
            // 2095 call QWORD PTR [rax+0x418]
            { let r = { let t = m.r64(rax.wrapping_add(0x418)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 209b xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 209d and al,r15b
            { let r = (rax as u8 as u64) & (r15 as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 20a0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 20a1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 20a3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 20a5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 20a7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 20a8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x20aa size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000020aa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x20aa;
    loop {
        match pc {
        0x20aa => {
            // block 20aa
            #[cfg(feature = "trace")] eprintln!("f_000020aa pc 20aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 20aa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 20ab mov rbp,rsp
            rbp = rsp;
            // 20ae push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 20b0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 20b2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 20b4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 20b5 mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // 20b8 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 20bb mov rbx,rdi
            rbx = rdi;
            // 20be mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 20c1 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 20c4 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 20c6 call 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 20cb mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 20ce mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 20d1 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 20d4 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 20d7 call 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 20dc mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 20df mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 20e2 call 0xcb406
            { let r = { let t = m.env.code_va(0xcb406); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 20e7 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 20ea cmp r12d,r14d
            fl = sub::<32>((r12 as u32 as u64), (r14 as u32 as u64)).1;
            // 20ed setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 20f0 and r15b,al
            { let r = (r15 as u8 as u64) & (rax as u8 as u64); let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 20f3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 20f6 mov rdi,rbx
            rdi = rbx;
            // 20f9 call QWORD PTR [rax+0x418]
            { let r = { let t = m.r64(rax.wrapping_add(0x418)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 20ff xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 2101 and al,r15b
            { let r = (rax as u8 as u64) & (r15 as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 2104 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2105 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2107 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2109 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 210b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 210c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c2c size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005c2c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5c2c;
    loop {
        match pc {
        0x5c2c => {
            // block 5c2c
            #[cfg(feature = "trace")] eprintln!("f_00005c2c pc 5c2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c2c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c2d mov rbp,rsp
            rbp = rsp;
            // 5c30 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5c32 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5c34 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5c35 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5c36 mov r14,rdx
            r14 = rdx;
            // 5c39 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5c3c mov rbx,rdi
            rbx = rdi;
            // 5c3f mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // 5c46 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5c49 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5c4f mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5c51 call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5c56 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 5c59 cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 5c5c jbe 0x5c82
            if fl.be() { pc = 0x5c82; continue; }
            pc = 0x5c5e;
        }
        0x5c5e => {
            // block 5c5e
            #[cfg(feature = "trace")] eprintln!("f_00005c2c pc 5c5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c5e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5c61 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 5c65 lea rdx,[rip+0xf9734] # 0xff3a0
            rdx = m.env.data_va(80749);
            // 5c6c mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5c6f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 5c74 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5c77 mov rdi,rbx
            rdi = rbx;
            // 5c7a mov rdx,r14
            rdx = r14;
            // 5c7d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5c80 jmp 0x5c84
            pc = 0x5c84; continue;
        }
        0x5c82 => {
            // block 5c82
            #[cfg(feature = "trace")] eprintln!("f_00005c2c pc 5c82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c82 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5c84;
        }
        0x5c84 => {
            // block 5c84
            #[cfg(feature = "trace")] eprintln!("f_00005c2c pc 5c84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c84 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5c88 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5c89 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5c8b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5c8d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c8e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xda24 size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000da24<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xda24;
    loop {
        match pc {
        0xda24 => {
            // block da24
            #[cfg(feature = "trace")] eprintln!("f_0000da24 pc da24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // da24 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // da25 mov rbp,rsp
            rbp = rsp;
            // da28 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // da2a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // da2c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // da2d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // da2e mov r14,rdx
            r14 = rdx;
            // da31 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // da34 mov rbx,rdi
            rbx = rdi;
            // da37 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // da3a call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // da40 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // da42 je 0xda48
            if fl.e() { pc = 0xda48; continue; }
            pc = 0xda44;
        }
        0xda44 => {
            // block da44
            #[cfg(feature = "trace")] eprintln!("f_0000da24 pc da44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // da44 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // da46 jmp 0xda7c
            pc = 0xda7c; continue;
        }
        0xda48 => {
            // block da48
            #[cfg(feature = "trace")] eprintln!("f_0000da24 pc da48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // da48 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // da4f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // da52 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // da55 lea rdx,[rip+0xf1964] # 0xff3c0
            rdx = m.env.data_va(80781);
            // da5c mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // da5f lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // da63 mov ecx,0x2000000
            rcx = (0x2000000u64) as u32 as u64;
            // da68 mov r8d,0x19
            r8 = (0x19u64) as u32 as u64;
            // da6e mov rdi,rbx
            rdi = rbx;
            // da71 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // da74 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // da78 setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0xda7c;
        }
        0xda7c => {
            // block da7c
            #[cfg(feature = "trace")] eprintln!("f_0000da24 pc da7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // da7c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // da80 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // da81 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // da83 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // da85 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // da86 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ad3a size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002ad3a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2ad3a;
    loop {
        match pc {
        0x2ad3a => {
            // block 2ad3a
            #[cfg(feature = "trace")] eprintln!("f_0002ad3a pc 2ad3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ad3a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ad3b mov rbp,rsp
            rbp = rsp;
            // 2ad3e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2ad40 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2ad42 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2ad44 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2ad45 mov r12,rcx
            r12 = rcx;
            // 2ad48 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 2ad4b mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2ad4e mov rbx,rdi
            rbx = rdi;
            // 2ad51 call 0x2ace2
            { let r = { let t = m.env.code_va(0x2ace2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ad56 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 2ad5a ja 0x2ad93
            if fl.a() { pc = 0x2ad93; continue; }
            pc = 0x2ad5c;
        }
        0x2ad5c => {
            // block 2ad5c
            #[cfg(feature = "trace")] eprintln!("f_0002ad3a pc 2ad5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ad5c test r12,r12
            fl = logic::<64>(r12 & r12);
            // 2ad5f je 0x2ad7f
            if fl.e() { pc = 0x2ad7f; continue; }
            pc = 0x2ad61;
        }
        0x2ad61 => {
            // block 2ad61
            #[cfg(feature = "trace")] eprintln!("f_0002ad3a pc 2ad61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ad61 lea r12,[rbx+0x8]
            r12 = rbx.wrapping_add(0x8);
            // 2ad65 mov edi,0x1800
            rdi = (0x1800u64) as u32 as u64;
            // 2ad6a call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ad6f mov edx,0x1800
            rdx = (0x1800u64) as u32 as u64;
            // 2ad74 mov rdi,r12
            rdi = r12;
            // 2ad77 mov rsi,rax
            rsi = rax;
            // 2ad7a call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2ad7f;
        }
        0x2ad7f => {
            // block 2ad7f
            #[cfg(feature = "trace")] eprintln!("f_0002ad3a pc 2ad7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ad7f mov DWORD PTR [rbx],r15d
            m.w32(rbx, ((r15 as u32 as u64)) as u32);
            // 2ad82 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2ad84 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 2ad88 cmove eax,r14d
            if fl.e() { rax = ((r14 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 2ad8c mov DWORD PTR [rbx+0x4],eax
            m.w32(rbx.wrapping_add(0x4), ((rax as u32 as u64)) as u32);
            // 2ad8f mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2ad91 jmp 0x2ad95
            pc = 0x2ad95; continue;
        }
        0x2ad93 => {
            // block 2ad93
            #[cfg(feature = "trace")] eprintln!("f_0002ad3a pc 2ad93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ad93 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2ad95;
        }
        0x2ad95 => {
            // block 2ad95
            #[cfg(feature = "trace")] eprintln!("f_0002ad3a pc 2ad95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ad95 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2ad96 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2ad98 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2ad9a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2ad9c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ad9d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1a86 size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e1a86<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe1a86;
    loop {
        match pc {
        0xe1a86 => {
            // block e1a86
            #[cfg(feature = "trace")] eprintln!("f_000e1a86 pc e1a86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a86 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1a87 mov rbp,rsp
            rbp = rsp;
            // e1a8a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1a8c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1a8d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e1a91 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e1a94 mov rbx,rdi
            rbx = rdi;
            // e1a97 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e1a9a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1a9d lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e1aa1 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e1aa6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e1aab xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1aae call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1ab1 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // e1ab5 mov eax,0x40000000
            rax = (0x40000000u64) as u32 as u64;
            // e1aba mov edx,0x80000000
            rdx = (0x80000000u64) as u32 as u64;
            // e1abf cmove edx,eax
            if fl.e() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // e1ac2 or edx,DWORD PTR [rbp-0x14]
            { let r = (rdx as u32 as u64) | m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e1ac5 mov DWORD PTR [rbp-0x14],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rdx as u32 as u64)) as u32);
            // e1ac8 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e1acb mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1ace mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e1ad3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e1ad8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1adb call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1ade mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e1ae0 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e1ae4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e1ae5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e1ae7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1ae8 ret
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
            #[cfg(feature = "trace")] eprintln!("f_000e1dca pc e1dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xc36a2 size=102
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c36a2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc36a2;
    loop {
        match pc {
        0xc36a2 => {
            // block c36a2
            #[cfg(feature = "trace")] eprintln!("f_000c36a2 pc c36a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c36a2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c36a3 mov rbp,rsp
            rbp = rsp;
            // c36a6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c36a8 cmp QWORD PTR [rip+0x104f70],0x0 # 0x1c8620
            fl = sub::<64>(m.r64(m.env.data_va(904685)) as u64, 0x0u64).1;
            // c36b0 je 0xc3705
            if fl.e() { pc = 0xc3705; continue; }
            pc = 0xc36b2;
        }
        0xc36b2 => {
            // block c36b2
            #[cfg(feature = "trace")] eprintln!("f_000c36a2 pc c36b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c36b2 mov rcx,QWORD PTR [rip+0x104f4f] # 0x1c8608
            rcx = m.r64(m.env.data_va(904661)) as u64;
            // c36b9 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c36bc je 0xc3705
            if fl.e() { pc = 0xc3705; continue; }
            pc = 0xc36be;
        }
        0xc36be => {
            // block c36be
            #[cfg(feature = "trace")] eprintln!("f_000c36a2 pc c36be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c36be mov rcx,QWORD PTR [rip+0x104f73] # 0x1c8638
            rcx = m.r64(m.env.data_va(904709)) as u64;
            // c36c5 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c36c8 je 0xc3705
            if fl.e() { pc = 0xc3705; continue; }
            pc = 0xc36ca;
        }
        0xc36ca => {
            // block c36ca
            #[cfg(feature = "trace")] eprintln!("f_000c36a2 pc c36ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c36ca mov rcx,QWORD PTR [rip+0x104fdf] # 0x1c86b0
            rcx = m.r64(m.env.data_va(904829)) as u64;
            // c36d1 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c36d4 je 0xc3705
            if fl.e() { pc = 0xc3705; continue; }
            pc = 0xc36d6;
        }
        0xc36d6 => {
            // block c36d6
            #[cfg(feature = "trace")] eprintln!("f_000c36a2 pc c36d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c36d6 mov rcx,QWORD PTR [rip+0x104f8b] # 0x1c8668
            rcx = m.r64(m.env.data_va(904757)) as u64;
            // c36dd test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c36e0 je 0xc3705
            if fl.e() { pc = 0xc3705; continue; }
            pc = 0xc36e2;
        }
        0xc36e2 => {
            // block c36e2
            #[cfg(feature = "trace")] eprintln!("f_000c36a2 pc c36e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c36e2 mov rcx,QWORD PTR [rip+0x104f67] # 0x1c8650
            rcx = m.r64(m.env.data_va(904733)) as u64;
            // c36e9 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c36ec je 0xc3705
            if fl.e() { pc = 0xc3705; continue; }
            pc = 0xc36ee;
        }
        0xc36ee => {
            // block c36ee
            #[cfg(feature = "trace")] eprintln!("f_000c36a2 pc c36ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c36ee mov rcx,QWORD PTR [rip+0x104f8b] # 0x1c8680
            rcx = m.r64(m.env.data_va(904781)) as u64;
            // c36f5 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c36f8 je 0xc3705
            if fl.e() { pc = 0xc3705; continue; }
            pc = 0xc36fa;
        }
        0xc36fa => {
            // block c36fa
            #[cfg(feature = "trace")] eprintln!("f_000c36a2 pc c36fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c36fa cmp QWORD PTR [rip+0x104f96],0x0 # 0x1c8698
            fl = sub::<64>(m.r64(m.env.data_va(904805)) as u64, 0x0u64).1;
            // c3702 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0xc3705;
        }
        0xc3705 => {
            // block c3705
            #[cfg(feature = "trace")] eprintln!("f_000c36a2 pc c3705 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3705 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3706 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe3028 size=102
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e3028<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe3028;
    loop {
        match pc {
        0xe3028 => {
            // block e3028
            #[cfg(feature = "trace")] eprintln!("f_000e3028 pc e3028 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3028 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e3029 mov rbp,rsp
            rbp = rsp;
            // e302c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e302e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e302f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e3033 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e3036 mov rbx,rdi
            rbx = rdi;
            // e3039 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e303c mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e303f lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e3043 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e3048 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e304d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e3050 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3053 mov edx,DWORD PTR [rbp-0x14]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // e3056 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e3058 and eax,0xdfffffff
            { let r = (rax as u32 as u64) & 0xdfffffffu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e305d or edx,0x20000000
            { let r = (rdx as u32 as u64) | 0x20000000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e3063 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // e3066 cmove edx,eax
            if fl.e() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // e3069 mov DWORD PTR [rbp-0x14],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rdx as u32 as u64)) as u32);
            // e306c mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e306f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e3072 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e3077 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e307c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e307f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3082 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e3084 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e3088 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e3089 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e308b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e308c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe70c size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e70c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe70c;
    loop {
        match pc {
        0xe70c => {
            // block e70c
            #[cfg(feature = "trace")] eprintln!("f_0000e70c pc e70c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e70c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e70d mov rbp,rsp
            rbp = rsp;
            // e710 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e712 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e714 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e715 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e716 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // e719 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e71c mov rbx,rdi
            rbx = rdi;
            // e71f mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // e722 call 0xcb584
            { let r = { let t = m.env.code_va(0xcb584); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e727 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e729 je 0xe742
            if fl.e() { pc = 0xe742; continue; }
            pc = 0xe72b;
        }
        0xe72b => {
            // block e72b
            #[cfg(feature = "trace")] eprintln!("f_0000e70c pc e72b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e72b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e72e movzx r15d,r15w
            r15 = ((r15 as u16 as u64)) as u32 as u64;
            // e732 mov rdi,rbx
            rdi = rbx;
            // e735 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e738 call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e73e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e740 je 0xe746
            if fl.e() { pc = 0xe746; continue; }
            pc = 0xe742;
        }
        0xe742 => {
            // block e742
            #[cfg(feature = "trace")] eprintln!("f_0000e70c pc e742 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e742 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e744 jmp 0xe769
            pc = 0xe769; continue;
        }
        0xe746 => {
            // block e746
            #[cfg(feature = "trace")] eprintln!("f_0000e70c pc e746 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e746 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e749 lea rcx,[rip+0xf0c70] # 0xff3c0
            rcx = m.env.data_va(80781);
            // e750 mov esi,DWORD PTR [rcx+r15*4]
            rsi = (m.r32(rcx.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // e754 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // e758 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // e75d mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            // e763 mov rdi,rbx
            rdi = rbx;
            // e766 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe769;
        }
        0xe769 => {
            // block e769
            #[cfg(feature = "trace")] eprintln!("f_0000e70c pc e769 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e769 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e76d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e76e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e770 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e772 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e773 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe7ec size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e7ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe7ec;
    loop {
        match pc {
        0xe7ec => {
            // block e7ec
            #[cfg(feature = "trace")] eprintln!("f_0000e7ec pc e7ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7ed mov rbp,rsp
            rbp = rsp;
            // e7f0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e7f2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e7f4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e7f5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e7f6 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // e7f9 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e7fc mov rbx,rdi
            rbx = rdi;
            // e7ff mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // e802 call 0xcb617
            { let r = { let t = m.env.code_va(0xcb617); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e807 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e809 je 0xe822
            if fl.e() { pc = 0xe822; continue; }
            pc = 0xe80b;
        }
        0xe80b => {
            // block e80b
            #[cfg(feature = "trace")] eprintln!("f_0000e7ec pc e80b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e80b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e80e movzx r15d,r15w
            r15 = ((r15 as u16 as u64)) as u32 as u64;
            // e812 mov rdi,rbx
            rdi = rbx;
            // e815 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e818 call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e820 je 0xe826
            if fl.e() { pc = 0xe826; continue; }
            pc = 0xe822;
        }
        0xe822 => {
            // block e822
            #[cfg(feature = "trace")] eprintln!("f_0000e7ec pc e822 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e822 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e824 jmp 0xe849
            pc = 0xe849; continue;
        }
        0xe826 => {
            // block e826
            #[cfg(feature = "trace")] eprintln!("f_0000e7ec pc e826 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e826 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e829 lea rcx,[rip+0xf0b90] # 0xff3c0
            rcx = m.env.data_va(80781);
            // e830 mov esi,DWORD PTR [rcx+r15*4]
            rsi = (m.r32(rcx.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // e834 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // e838 mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // e83d mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // e843 mov rdi,rbx
            rdi = rbx;
            // e846 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe849;
        }
        0xe849 => {
            // block e849
            #[cfg(feature = "trace")] eprintln!("f_0000e7ec pc e849 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e849 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e84d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e84e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e850 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e852 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e853 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42a78 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042a78<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42a78;
    loop {
        match pc {
        0x42a78 => {
            // block 42a78
            #[cfg(feature = "trace")] eprintln!("f_00042a78 pc 42a78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42a78 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42a79 mov rbp,rsp
            rbp = rsp;
            // 42a7c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42a7e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42a80 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42a81 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42a82 mov r14,rsi
            r14 = rsi;
            // 42a85 mov rbx,rdi
            rbx = rdi;
            // 42a88 mov DWORD PTR [r14],0x8
            m.w32(r14, (0x8u64) as u32);
            // 42a8f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42a92 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42a98 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42a9a je 0x42acf
            if fl.e() { pc = 0x42acf; continue; }
            pc = 0x42a9c;
        }
        0x42a9c => {
            // block 42a9c
            #[cfg(feature = "trace")] eprintln!("f_00042a78 pc 42a9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42a9c mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 42aa3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42aa6 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 42aaa xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42aad mov esi,0x900
            rsi = (0x900u64) as u32 as u64;
            // 42ab2 mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // 42ab7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42aba mov rdi,rbx
            rdi = rbx;
            // 42abd call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42ac0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42ac2 je 0x42ad2
            if fl.e() { pc = 0x42ad2; continue; }
            pc = 0x42ac4;
        }
        0x42ac4 => {
            // block 42ac4
            #[cfg(feature = "trace")] eprintln!("f_00042a78 pc 42ac4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42ac4 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 42ac7 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 42aca mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 42acd jmp 0x42ad2
            pc = 0x42ad2; continue;
        }
        0x42acf => {
            // block 42acf
            #[cfg(feature = "trace")] eprintln!("f_00042a78 pc 42acf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42acf xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x42ad2;
        }
        0x42ad2 => {
            // block 42ad2
            #[cfg(feature = "trace")] eprintln!("f_00042a78 pc 42ad2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42ad2 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 42ad5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42ad9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42ada pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42adc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42ade pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42adf ret
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
            #[cfg(feature = "trace")] eprintln!("f_000ba672 pc ba672 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000ba672 pc ba69f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba69f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ba6a1 jmp 0xba6ca
            pc = 0xba6ca; continue;
        }
        0xba6a3 => {
            // block ba6a3
            #[cfg(feature = "trace")] eprintln!("f_000ba672 pc ba6a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000ba672 pc ba6ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.env.code_va(0xe4a46); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xe4a46); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xe4a46); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xe4a46); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            #[cfg(feature = "trace")] eprintln!("f_000b6182 pc b6182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b6182 pc b61ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b6182 pc b61db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xd4318 size=106
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4318<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4318;
    loop {
        match pc {
        0xd4318 => {
            // block d4318
            #[cfg(feature = "trace")] eprintln!("f_000d4318 pc d4318 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4318 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4319 mov rbp,rsp
            rbp = rsp;
            // d431c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d431e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4320 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4321 sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d4325 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d4328 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d432b mov rbx,rdi
            rbx = rdi;
            // d432e lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d4332 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d4337 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // d433c call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4341 mov DWORD PTR [rbp-0x58],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0x1u64) as u32);
            // d4348 movzx eax,r14b
            rax = ((r14 as u8 as u64)) as u32 as u64;
            // d434c mov DWORD PTR [rbp-0x54],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((rax as u32 as u64)) as u32);
            // d434f lea rdx,[rbp-0x74]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // d4353 mov rdi,rbx
            rdi = rbx;
            // d4356 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d4359 call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d435e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d4360 je 0xd4374
            if fl.e() { pc = 0xd4374; continue; }
            pc = 0xd4362;
        }
        0xd4362 => {
            // block d4362
            #[cfg(feature = "trace")] eprintln!("f_000d4318 pc d4362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4362 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d4365 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d4369 mov rdi,rbx
            rdi = rbx;
            // d436c call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4372 jmp 0xd4376
            pc = 0xd4376; continue;
        }
        0xd4374 => {
            // block d4374
            #[cfg(feature = "trace")] eprintln!("f_000d4318 pc d4374 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4374 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd4376;
        }
        0xd4376 => {
            // block d4376
            #[cfg(feature = "trace")] eprintln!("f_000d4318 pc d4376 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4376 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d437a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d437b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d437d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d437f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4380 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3d92 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003d92<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3d92;
    loop {
        match pc {
        0x3d92 => {
            // block 3d92
            #[cfg(feature = "trace")] eprintln!("f_00003d92 pc 3d92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d92 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3d93 mov rbp,rsp
            rbp = rsp;
            // 3d96 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 3d98 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3d9a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3d9b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3d9c mov r14,rsi
            r14 = rsi;
            // 3d9f mov rbx,rdi
            rbx = rdi;
            // 3da2 mov WORD PTR [r14],0x0
            m.w16(r14, (0x0u64) as u16);
            // 3da8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3dab call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3db1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3db3 je 0x3dec
            if fl.e() { pc = 0x3dec; continue; }
            pc = 0x3db5;
        }
        0x3db5 => {
            // block 3db5
            #[cfg(feature = "trace")] eprintln!("f_00003d92 pc 3db5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3db5 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 3dbc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3dbf lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 3dc3 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 3dc6 mov esi,0x30
            rsi = (0x30u64) as u32 as u64;
            // 3dcb mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3dd0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3dd3 mov rdi,rbx
            rdi = rbx;
            // 3dd6 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3dd9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3ddb je 0x3def
            if fl.e() { pc = 0x3def; continue; }
            pc = 0x3ddd;
        }
        0x3ddd => {
            // block 3ddd
            #[cfg(feature = "trace")] eprintln!("f_00003d92 pc 3ddd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3ddd mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 3de0 movzx eax,ah
            rax = (((rax >> 8) as u8 as u64)) as u32 as u64;
            // 3de3 mov WORD PTR [r14],ax
            m.w16(r14, ((rax as u16 as u64)) as u16);
            // 3de7 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 3dea jmp 0x3def
            pc = 0x3def; continue;
        }
        0x3dec => {
            // block 3dec
            #[cfg(feature = "trace")] eprintln!("f_00003d92 pc 3dec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3dec xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x3def;
        }
        0x3def => {
            // block 3def
            #[cfg(feature = "trace")] eprintln!("f_00003d92 pc 3def rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3def mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 3df2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 3df6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3df7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3df9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3dfb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3dfc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5800 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005800<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5800;
    loop {
        match pc {
        0x5800 => {
            // block 5800
            #[cfg(feature = "trace")] eprintln!("f_00005800 pc 5800 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5800 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5801 mov rbp,rsp
            rbp = rsp;
            // 5804 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5806 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5808 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5809 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 580a mov r14,rdx
            r14 = rdx;
            // 580d mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5810 mov rbx,rdi
            rbx = rdi;
            // 5813 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 581a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 581d call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5823 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5825 jne 0x585f
            if fl.ne() { pc = 0x585f; continue; }
            pc = 0x5827;
        }
        0x5827 => {
            // block 5827
            #[cfg(feature = "trace")] eprintln!("f_00005800 pc 5827 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5827 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 582a cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 582e sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5831 movzx esi,cl
            rsi = ((rcx as u8 as u64)) as u32 as u64;
            // 5834 add esi,0xa5
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xa5u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 583a lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 583e mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // 5843 mov r8d,0x2
            r8 = (0x2u64) as u32 as u64;
            // 5849 mov rdi,rbx
            rdi = rbx;
            // 584c call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 584f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5851 je 0x585f
            if fl.e() { pc = 0x585f; continue; }
            pc = 0x5853;
        }
        0x5853 => {
            // block 5853
            #[cfg(feature = "trace")] eprintln!("f_00005800 pc 5853 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5853 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // 5857 setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            // 585b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 585d jmp 0x5861
            pc = 0x5861; continue;
        }
        0x585f => {
            // block 585f
            #[cfg(feature = "trace")] eprintln!("f_00005800 pc 585f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 585f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5861;
        }
        0x5861 => {
            // block 5861
            #[cfg(feature = "trace")] eprintln!("f_00005800 pc 5861 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5861 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5865 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5866 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5868 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 586a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 586b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd778 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d778<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd778;
    loop {
        match pc {
        0xd778 => {
            // block d778
            #[cfg(feature = "trace")] eprintln!("f_0000d778 pc d778 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d778 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d779 mov rbp,rsp
            rbp = rsp;
            // d77c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d77e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d780 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d781 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d782 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d785 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d787 mov r15,rdi
            r15 = rdi;
            // d78a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d78d call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d793 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d795 je 0xd79b
            if fl.e() { pc = 0xd79b; continue; }
            pc = 0xd797;
        }
        0xd797 => {
            // block d797
            #[cfg(feature = "trace")] eprintln!("f_0000d778 pc d797 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d797 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d799 jmp 0xd7d9
            pc = 0xd7d9; continue;
        }
        0xd79b => {
            // block d79b
            #[cfg(feature = "trace")] eprintln!("f_0000d778 pc d79b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d79b dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // d79d cmp ebx,0x6
            fl = sub::<32>((rbx as u32 as u64), 0x6u64).1;
            // d7a0 ja 0xd7bc
            if fl.a() { pc = 0xd7bc; continue; }
            pc = 0xd7a2;
        }
        0xd7a2 => {
            // block d7a2
            #[cfg(feature = "trace")] eprintln!("f_0000d778 pc d7a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7a2 movsxd rax,ebx
            rax = sx::<32>((rbx as u32 as u64)) as u64;
            // d7a5 lea rcx,[rip+0xf2294] # 0xffa40
            rcx = m.env.data_va(82445);
            // d7ac mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // d7af lea rdx,[rip+0xf22aa] # 0xffa60
            rdx = m.env.data_va(82477);
            // d7b6 mov r8d,DWORD PTR [rdx+rax*4]
            r8 = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // d7ba jmp 0xd7c7
            pc = 0xd7c7; continue;
        }
        0xd7bc => {
            // block d7bc
            #[cfg(feature = "trace")] eprintln!("f_0000d778 pc d7bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7bc mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // d7c2 mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            pc = 0xd7c7;
        }
        0xd7c7 => {
            // block d7c7
            #[cfg(feature = "trace")] eprintln!("f_0000d778 pc d7c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7c7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7ca movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // d7ce mov esi,0x100
            rsi = (0x100u64) as u32 as u64;
            // d7d3 mov rdi,r15
            rdi = r15;
            // d7d6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd7d9;
        }
        0xd7d9 => {
            // block d7d9
            #[cfg(feature = "trace")] eprintln!("f_0000d778 pc d7d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7d9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d7dd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7de pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7e0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d7e2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7e3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4497c size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004497c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4497c;
    loop {
        match pc {
        0x4497c => {
            // block 4497c
            #[cfg(feature = "trace")] eprintln!("f_0004497c pc 4497c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4497c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4497d mov rbp,rsp
            rbp = rsp;
            // 44980 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44982 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44984 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 44986 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44987 mov r14,rdx
            r14 = rdx;
            // 4498a mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 4498c mov r15,rdi
            r15 = rdi;
            // 4498f cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 44992 ja 0x449dd
            if fl.a() { pc = 0x449dd; continue; }
            pc = 0x44994;
        }
        0x44994 => {
            // block 44994
            #[cfg(feature = "trace")] eprintln!("f_0004497c pc 44994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44994 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 44998 call 0xcf820
            { let r = { let t = m.env.code_va(0xcf820); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4499d movzx r12d,ax
            r12 = ((rax as u16 as u64)) as u32 as u64;
            // 449a1 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 449a4 mov rdi,r15
            rdi = r15;
            // 449a7 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 449ad movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 449b0 add eax,r12d
            { let (r, f) = add::<32>((rax as u32 as u64), (r12 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 449b3 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // 449b5 jle 0x449dd
            if fl.le() { pc = 0x449dd; continue; }
            pc = 0x449b7;
        }
        0x449b7 => {
            // block 449b7
            #[cfg(feature = "trace")] eprintln!("f_0004497c pc 449b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 449b7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 449ba mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 449bc lea rdx,[rip+0xbbaad] # 0x100470
            rdx = m.env.data_va(85053);
            // 449c3 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 449c6 movzx edx,BYTE PTR [r14]
            rdx = (m.r8(r14) as u64) as u32 as u64;
            // 449ca mov ecx,0x80000
            rcx = (0x80000u64) as u32 as u64;
            // 449cf mov r8d,0x13
            r8 = (0x13u64) as u32 as u64;
            // 449d5 mov rdi,r15
            rdi = r15;
            // 449d8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 449db jmp 0x449df
            pc = 0x449df; continue;
        }
        0x449dd => {
            // block 449dd
            #[cfg(feature = "trace")] eprintln!("f_0004497c pc 449dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 449dd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x449df;
        }
        0x449df => {
            // block 449df
            #[cfg(feature = "trace")] eprintln!("f_0004497c pc 449df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 449df pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 449e0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 449e2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 449e4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 449e6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 449e7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd9f06 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d9f06<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd9f06;
    loop {
        match pc {
        0xd9f06 => {
            // block d9f06
            #[cfg(feature = "trace")] eprintln!("f_000d9f06 pc d9f06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9f06 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d9f07 mov rbp,rsp
            rbp = rsp;
            // d9f0a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d9f0b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d9f0c mov rbx,rdi
            rbx = rdi;
            // d9f0f mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // d9f16 call 0xd8956
            { let r = { let t = m.env.code_va(0xd8956); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9f1b mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d9f1d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d9f1f test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d9f21 je 0xd9f6a
            if fl.e() { pc = 0xd9f6a; continue; }
            pc = 0xd9f23;
        }
        0xd9f23 => {
            // block d9f23
            #[cfg(feature = "trace")] eprintln!("f_000d9f06 pc d9f23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9f23 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d9f26 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d9f2b mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // d9f30 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9f35 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9f38 mov rdi,rbx
            rdi = rbx;
            // d9f3b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9f3e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d9f41 mov rdi,rbx
            rdi = rbx;
            // d9f44 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9f4a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d9f4d lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d9f51 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d9f56 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9f5b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9f5e mov rdi,rbx
            rdi = rbx;
            // d9f61 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9f64 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // d9f67 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd9f6a;
        }
        0xd9f6a => {
            // block d9f6a
            #[cfg(feature = "trace")] eprintln!("f_000d9f06 pc d9f6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9f6a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d9f6e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d9f6f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d9f70 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2a62 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2a62<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2a62;
    loop {
        match pc {
        0xe2a62 => {
            // block e2a62
            #[cfg(feature = "trace")] eprintln!("f_000e2a62 pc e2a62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a62 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2a63 mov rbp,rsp
            rbp = rsp;
            // e2a66 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2a67 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e2a6b mov rbx,rcx
            rbx = rcx;
            // e2a6e xorps xmm0,xmm0
            x0 = 0;
            // e2a71 movaps XMMWORD PTR [rbp-0x20],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), x0);
            // e2a75 mov QWORD PTR [rbp-0x10],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u64);
            // e2a7d lea rcx,[rbp-0x20]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e2a81 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2a86 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2a88 je 0xe2a96
            if fl.e() { pc = 0xe2a96; continue; }
            pc = 0xe2a8a;
        }
        0xe2a8a => {
            // block e2a8a
            #[cfg(feature = "trace")] eprintln!("f_000e2a62 pc e2a8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a8a test BYTE PTR [rbp-0x20],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64 & 0x1u64);
            // e2a8e jne 0xe2a9a
            if fl.ne() { pc = 0xe2a9a; continue; }
            pc = 0xe2a90;
        }
        0xe2a90 => {
            // block e2a90
            #[cfg(feature = "trace")] eprintln!("f_000e2a62 pc e2a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a90 lea rdi,[rbp-0x1f]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe1);
            // e2a94 jmp 0xe2a9e
            pc = 0xe2a9e; continue;
        }
        0xe2a96 => {
            // block e2a96
            #[cfg(feature = "trace")] eprintln!("f_000e2a62 pc e2a96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a96 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2a98 jmp 0xe2aa7
            pc = 0xe2aa7; continue;
        }
        0xe2a9a => {
            // block e2a9a
            #[cfg(feature = "trace")] eprintln!("f_000e2a62 pc e2a9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a9a mov rdi,QWORD PTR [rbp-0x10]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64;
            pc = 0xe2a9e;
        }
        0xe2a9e => {
            // block e2a9e
            #[cfg(feature = "trace")] eprintln!("f_000e2a62 pc e2a9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2a9e call 0xec1a6
            { let r = { let t = m.env.stub_va(0x973); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2aa3 mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // e2aa5 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe2aa7;
        }
        0xe2aa7 => {
            // block e2aa7
            #[cfg(feature = "trace")] eprintln!("f_000e2a62 pc e2aa7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2aa7 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e2aab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2ab0 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2ab2 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e2ab6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2ab7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2ab8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe2ab9 => {
            // block e2ab9
            #[cfg(feature = "trace")] eprintln!("f_000e2a62 pc e2ab9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2ab9 mov rbx,rax
            rbx = rax;
            // e2abc lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e2ac0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2ac5 mov rdi,rbx
            rdi = rbx;
            // e2ac8 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe61e4 size=108
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e61e4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe61e4;
    loop {
        match pc {
        0xe61e4 => {
            // block e61e4
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e61e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e61e4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e61e5 mov rbp,rsp
            rbp = rsp;
            // e61e8 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // e61ea jne 0xe61f3
            if fl.ne() { pc = 0xe61f3; continue; }
            pc = 0xe61ec;
        }
        0xe61ec => {
            // block e61ec
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e61ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e61ec mov esi,DWORD PTR [rdi+0x8]
            rsi = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // e61ef test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // e61f1 je 0xe6240
            if fl.e() { pc = 0xe6240; continue; }
            pc = 0xe61f3;
        }
        0xe61f3 => {
            // block e61f3
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e61f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e61f3 lea eax,[rsi-0x1]
            rax = (rsi.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // e61f6 mov ecx,0x1e
            rcx = (0x1eu64) as u32 as u64;
            // e61fb cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e61fe ja 0xe620c
            if fl.a() { pc = 0xe620c; continue; }
            pc = 0xe6200;
        }
        0xe6200 => {
            // block e6200
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e6200 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6200 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e6202 lea rcx,[rip+0x43ad7] # 0x129ce0
            rcx = m.env.data_va(255149);
            // e6209 mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe620c;
        }
        0xe620c => {
            // block e620c
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e620c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e620c imul eax,ecx,0x3c
            { let (r, f) = imul::<32>((rcx as u32 as u64), 0x3cu64); rax = (r) as u32 as u64; fl = f; }
            pc = 0xe620f;
        }
        0xe620f => {
            // block e620f
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e620f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e620f cmp esi,0x4
            fl = sub::<32>((rsi as u32 as u64), 0x4u64).1;
            // e6212 je 0xe6219
            if fl.e() { pc = 0xe6219; continue; }
            pc = 0xe6214;
        }
        0xe6214 => {
            // block e6214
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e6214 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6214 cmp esi,0x8
            fl = sub::<32>((rsi as u32 as u64), 0x8u64).1;
            // e6217 jne 0xe6238
            if fl.ne() { pc = 0xe6238; continue; }
            pc = 0xe6219;
        }
        0xe6219 => {
            // block e6219
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e6219 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6219 cmp DWORD PTR [rdi+0x8],0x8
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x8)) as u64, 0x8u64).1;
            // e621d sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // e6220 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // e6223 lea ecx,[rcx+rcx*1+0x2]
            rcx = (rcx.wrapping_add(rcx).wrapping_add(0x2)) as u32 as u64;
            // e6227 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // e6229 sub edx,ecx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e622b lea ecx,[rdx+rdx*8]
            rcx = (rdx.wrapping_add(rdx.wrapping_mul(8))) as u32 as u64;
            // e622e add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6230 shl ecx,0x4
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e6233 lea eax,[rcx+rcx*8]
            rax = (rcx.wrapping_add(rcx.wrapping_mul(8))) as u32 as u64;
            // e6236 jmp 0xe623e
            pc = 0xe623e; continue;
        }
        0xe6238 => {
            // block e6238
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e6238 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6238 imul eax,ecx,0x15180
            { let (r, f) = imul::<32>((rcx as u32 as u64), 0x15180u64); rax = (r) as u32 as u64; fl = f; }
            pc = 0xe623e;
        }
        0xe623e => {
            // block e623e
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e623e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e623e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e623f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe6240 => {
            // block e6240
            #[cfg(feature = "trace")] eprintln!("f_000e61e4 pc e6240 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6240 mov esi,DWORD PTR [rdi+0x8]
            rsi = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // e6243 mov ecx,0x1e
            rcx = (0x1eu64) as u32 as u64;
            // e6248 mov eax,0x708
            rax = (0x708u64) as u32 as u64;
            // e624d jmp 0xe620f
            pc = 0xe620f; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x81b0 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000081b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x81b0;
    loop {
        match pc {
        0x81b0 => {
            // block 81b0
            #[cfg(feature = "trace")] eprintln!("f_000081b0 pc 81b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 81b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 81b1 mov rbp,rsp
            rbp = rsp;
            // 81b4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 81b6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 81b8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 81b9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 81ba mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 81bd mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 81c0 mov rbx,rdi
            rbx = rdi;
            // 81c3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 81c6 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 81c9 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 81cf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 81d1 je 0x81d7
            if fl.e() { pc = 0x81d7; continue; }
            pc = 0x81d3;
        }
        0x81d3 => {
            // block 81d3
            #[cfg(feature = "trace")] eprintln!("f_000081b0 pc 81d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 81d3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 81d5 jmp 0x8212
            pc = 0x8212; continue;
        }
        0x81d7 => {
            // block 81d7
            #[cfg(feature = "trace")] eprintln!("f_000081b0 pc 81d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 81d7 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 81da call 0xce76c
            { let r = { let t = m.env.code_va(0xce76c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 81df mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 81e1 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 81e3 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 81e6 jne 0x8212
            if fl.ne() { pc = 0x8212; continue; }
            pc = 0x81e8;
        }
        0x81e8 => {
            // block 81e8
            #[cfg(feature = "trace")] eprintln!("f_000081b0 pc 81e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 81e8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 81eb movzx edx,r15b
            rdx = ((r15 as u8 as u64)) as u32 as u64;
            // 81ef mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 81f2 lea rcx,[rip+0xf72d7] # 0xff4d0
            rcx = m.env.data_va(81053);
            // 81f9 mov ecx,DWORD PTR [rcx+rsi*4]
            rcx = (m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // 81fc lea rdi,[rip+0xf72ed] # 0xff4f0
            rdi = m.env.data_va(81085);
            // 8203 mov r8d,DWORD PTR [rdi+rsi*4]
            r8 = (m.r32(rdi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // 8207 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 820c mov rdi,rbx
            rdi = rbx;
            // 820f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x8212;
        }
        0x8212 => {
            // block 8212
            #[cfg(feature = "trace")] eprintln!("f_000081b0 pc 8212 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8212 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 8216 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8217 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8219 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 821b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 821c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2b48c size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002b48c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2b48c;
    loop {
        match pc {
        0x2b48c => {
            // block 2b48c
            #[cfg(feature = "trace")] eprintln!("f_0002b48c pc 2b48c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b48c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b48d mov rbp,rsp
            rbp = rsp;
            // 2b490 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2b492 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2b494 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2b496 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2b498 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2b499 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2b49a mov r14,r9
            r14 = r9;
            // 2b49d mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // 2b4a0 mov r15,rcx
            r15 = rcx;
            // 2b4a3 mov rbx,rdi
            rbx = rdi;
            // 2b4a6 mov r13,QWORD PTR [rbp+0x18]
            r13 = m.r64(rbp.wrapping_add(0x18)) as u64;
            // 2b4aa lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 2b4ae mov edx,edx
            rdx = ((rdx as u32 as u64)) as u32 as u64;
            // 2b4b0 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b4b5 lea rdi,[rbx+0x48]
            rdi = rbx.wrapping_add(0x48);
            // 2b4b9 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 2b4bc mov rsi,r15
            rsi = r15;
            // 2b4bf call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b4c4 lea rdi,[rbx+0x70]
            rdi = rbx.wrapping_add(0x70);
            // 2b4c8 mov edx,DWORD PTR [rbp+0x10]
            rdx = (m.r32(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 2b4cb mov rsi,r14
            rsi = r14;
            // 2b4ce call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b4d3 add rbx,0x98
            { let (r, f) = add::<64>(rbx, 0x98u64);
              rbx = r;
              fl = f; }
            // 2b4da mov edx,DWORD PTR [rbp+0x20]
            rdx = (m.r32(rbp.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2b4dd mov rdi,rbx
            rdi = rbx;
            // 2b4e0 mov rsi,r13
            rsi = r13;
            // 2b4e3 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b4e8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2b4ea add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2b4ee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2b4ef pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2b4f1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2b4f3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2b4f5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2b4f7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b4f8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4487c size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004487c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4487c;
    loop {
        match pc {
        0x4487c => {
            // block 4487c
            #[cfg(feature = "trace")] eprintln!("f_0004487c pc 4487c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4487c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4487d mov rbp,rsp
            rbp = rsp;
            // 44880 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44882 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44884 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 44886 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44887 mov r15,rdx
            r15 = rdx;
            // 4488a mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 4488c mov r14,rdi
            r14 = rdi;
            // 4488f cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 44892 ja 0x448df
            if fl.a() { pc = 0x448df; continue; }
            pc = 0x44894;
        }
        0x44894 => {
            // block 44894
            #[cfg(feature = "trace")] eprintln!("f_0004487c pc 44894 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44894 movzx r12d,bx
            r12 = ((rbx as u16 as u64)) as u32 as u64;
            // 44898 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // 4489c call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 448a1 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 448a4 cmp r12d,eax
            fl = sub::<32>((r12 as u32 as u64), (rax as u32 as u64)).1;
            // 448a7 jae 0x448df
            if fl.ae() { pc = 0x448df; continue; }
            pc = 0x448a9;
        }
        0x448a9 => {
            // block 448a9
            #[cfg(feature = "trace")] eprintln!("f_0004487c pc 448a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 448a9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 448ac mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 448ae lea rdx,[rip+0xbbdeb] # 0x1006a0
            rdx = m.env.data_va(85613);
            // 448b5 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 448b8 movzx edx,BYTE PTR [r15]
            rdx = (m.r8(r15) as u64) as u32 as u64;
            // 448bc xor edx,0x1
            { let r = (rdx as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 448bf and ebx,0x1
            { let r = (rbx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 448c2 mov edi,0x8000
            rdi = (0x8000u64) as u32 as u64;
            // 448c7 mov ecx,0x2000
            rcx = (0x2000u64) as u32 as u64;
            // 448cc cmovne ecx,edi
            if fl.ne() { rcx = ((rdi as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 448cf add ebx,ebx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rbx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 448d1 or ebx,0xd
            { let r = (rbx as u32 as u64) | 0xdu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 448d4 mov rdi,r14
            rdi = r14;
            // 448d7 mov r8d,ebx
            r8 = ((rbx as u32 as u64)) as u32 as u64;
            // 448da call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 448dd jmp 0x448e1
            pc = 0x448e1; continue;
        }
        0x448df => {
            // block 448df
            #[cfg(feature = "trace")] eprintln!("f_0004487c pc 448df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 448df xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x448e1;
        }
        0x448e1 => {
            // block 448e1
            #[cfg(feature = "trace")] eprintln!("f_0004487c pc 448e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 448e1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 448e2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 448e4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 448e6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 448e8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 448e9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x58332 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00058332<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x58332;
    loop {
        match pc {
        0x58332 => {
            // block 58332
            #[cfg(feature = "trace")] eprintln!("f_00058332 pc 58332 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58332 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 58333 mov rbp,rsp
            rbp = rsp;
            // 58336 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58338 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58339 lea rbx,[rip+0x16fd50] # 0x1c8090
            rbx = m.env.data_va(903261);
            // 58340 lea rsi,[rip+0xd2967] # 0x12acae
            rsi = m.env.data_va(259195);
            // 58347 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 5834c mov rdi,rbx
            rdi = rbx;
            // 5834f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58354 mov rdi,QWORD PTR [rip+0xe7e1d] # 0x140178
            rdi = m.r64(m.env.data_va(346437)) as u64;
            // 5835b lea r14,[rip+0xfffffffffffa6e0e] # 0xfffffffffffff170
            r14 = m.env.data_va(-968387);
            // 58362 mov rsi,rbx
            rsi = rbx;
            // 58365 mov rdx,r14
            rdx = r14;
            // 58368 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5836d lea rbx,[rip+0x16fd34] # 0x1c80a8
            rbx = m.env.data_va(903285);
            // 58374 mov rdi,rbx
            rdi = rbx;
            // 58377 call 0x55a36
            { let r = { let t = m.env.code_va(0x55a36); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5837c lea rdi,[rip+0xffffffffffffa17d] # 0x52500
            rdi = m.env.code_va(0x52500);
            // 58383 mov rsi,rbx
            rsi = rbx;
            // 58386 mov rdx,r14
            rdx = r14;
            // 58389 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5838e lea rdi,[rip+0x16ff03] # 0x1c8298
            rdi = m.env.data_va(903781);
            // 58395 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58396 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58398 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58399 jmp 0x57d4a
            { let r = { let t = m.env.code_va(0x57d4a); m.call(t, &[], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb568e size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b568e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb568e;
    loop {
        match pc {
        0xb568e => {
            // block b568e
            #[cfg(feature = "trace")] eprintln!("f_000b568e pc b568e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b568e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b568f mov rbp,rsp
            rbp = rsp;
            // b5692 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5694 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5696 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5698 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5699 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b569d mov r15,rsi
            r15 = rsi;
            // b56a0 mov r14,rdi
            r14 = rdi;
            // b56a3 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // b56a7 mov rdx,r15
            rdx = r15;
            // b56aa call 0xebc06
            { let r = { let t = m.env.stub_va(0x3d3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b56af mov r12,rax
            r12 = rax;
            // b56b2 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // b56b6 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b56b9 jne 0xb56e8
            if fl.ne() { pc = 0xb56e8; continue; }
            pc = 0xb56bb;
        }
        0xb56bb => {
            // block b56bb
            #[cfg(feature = "trace")] eprintln!("f_000b568e pc b56bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b56bb mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // b56c0 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b56c5 mov rbx,rax
            rbx = rax;
            // b56c8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b56cb mov QWORD PTR [rbx+0x1c],rax
            m.w64(rbx.wrapping_add(0x1c), (rax) as u64);
            // b56cf mov DWORD PTR [rbx+0x24],0x0
            m.w32(rbx.wrapping_add(0x24), (0x0u64) as u32);
            // b56d6 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b56da mov rdi,r14
            rdi = r14;
            // b56dd mov rdx,r12
            rdx = r12;
            // b56e0 mov rcx,rbx
            rcx = rbx;
            // b56e3 call 0xebc3c
            { let r = { let t = m.env.stub_va(0x409); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb56e8;
        }
        0xb56e8 => {
            // block b56e8
            #[cfg(feature = "trace")] eprintln!("f_000b568e pc b56e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b56e8 add rbx,0x24
            { let (r, f) = add::<64>(rbx, 0x24u64);
              rbx = r;
              fl = f; }
            // b56ec mov rax,rbx
            rax = rbx;
            // b56ef add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b56f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b56f4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b56f6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b56f8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b56fa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b56fb ret
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
            #[cfg(feature = "trace")] eprintln!("f_000c3ac8 pc c3ac8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c3ac8 pc c3af4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c3ac8 pc c3b21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xc3c4e size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3c4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3c4e;
    loop {
        match pc {
        0xc3c4e => {
            // block c3c4e
            #[cfg(feature = "trace")] eprintln!("f_000c3c4e pc c3c4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3c4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3c4f mov rbp,rsp
            rbp = rsp;
            // c3c52 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3c54 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3c56 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3c58 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3c59 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // c3c5d mov r14,rsi
            r14 = rsi;
            // c3c60 mov r12,rdi
            r12 = rdi;
            // c3c63 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3c67 mov rdx,r14
            rdx = r14;
            // c3c6a call 0xebd56
            { let r = { let t = m.env.stub_va(0x523); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3c6f mov r15,rax
            r15 = rax;
            // c3c72 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3c75 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3c78 jne 0xc3ca7
            if fl.ne() { pc = 0xc3ca7; continue; }
            pc = 0xc3c7a;
        }
        0xc3c7a => {
            // block c3c7a
            #[cfg(feature = "trace")] eprintln!("f_000c3c4e pc c3c7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3c7a lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // c3c7e mov rsi,r12
            rsi = r12;
            // c3c81 mov rdx,r14
            rdx = r14;
            // c3c84 call 0xebd5c
            { let r = { let t = m.env.stub_va(0x529); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3c89 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3c8d mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // c3c91 mov rdi,r12
            rdi = r12;
            // c3c94 mov rdx,r15
            rdx = r15;
            // c3c97 mov rcx,rbx
            rcx = rbx;
            // c3c9a call 0xebe7c
            { let r = { let t = m.env.stub_va(0x649); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3c9f mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            pc = 0xc3ca7;
        }
        0xc3ca7 => {
            // block c3ca7
            #[cfg(feature = "trace")] eprintln!("f_000c3c4e pc c3ca7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3ca7 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // c3cab mov rax,rbx
            rax = rbx;
            // c3cae add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // c3cb2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3cb3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3cb5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3cb7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3cb9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3cba ret
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
            #[cfg(feature = "trace")] eprintln!("f_000c3d56 pc c3d56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c3d56 pc c3d83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c3d56 pc c3daf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xc3dc4 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3dc4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3dc4;
    loop {
        match pc {
        0xc3dc4 => {
            // block c3dc4
            #[cfg(feature = "trace")] eprintln!("f_000c3dc4 pc c3dc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3dc4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3dc5 mov rbp,rsp
            rbp = rsp;
            // c3dc8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3dca push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3dcc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3dce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3dcf sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3dd3 mov r15,rsi
            r15 = rsi;
            // c3dd6 mov r14,rdi
            r14 = rdi;
            // c3dd9 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3ddd mov rdx,r15
            rdx = r15;
            // c3de0 call 0xebd26
            { let r = { let t = m.env.stub_va(0x4f3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3de5 mov r12,rax
            r12 = rax;
            // c3de8 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // c3dec test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3def jne 0xc3e1d
            if fl.ne() { pc = 0xc3e1d; continue; }
            pc = 0xc3df1;
        }
        0xc3df1 => {
            // block c3df1
            #[cfg(feature = "trace")] eprintln!("f_000c3dc4 pc c3df1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3df1 mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // c3df6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3dfb mov rbx,rax
            rbx = rax;
            // c3dfe mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // c3e01 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c3e04 mov DWORD PTR [rbx+0x20],0x0
            m.w32(rbx.wrapping_add(0x20), (0x0u64) as u32);
            // c3e0b mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3e0f mov rdi,r14
            rdi = r14;
            // c3e12 mov rdx,r12
            rdx = r12;
            // c3e15 mov rcx,rbx
            rcx = rbx;
            // c3e18 call 0xebe3a
            { let r = { let t = m.env.stub_va(0x607); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3e1d;
        }
        0xc3e1d => {
            // block c3e1d
            #[cfg(feature = "trace")] eprintln!("f_000c3dc4 pc c3e1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3e1d add rbx,0x20
            { let (r, f) = add::<64>(rbx, 0x20u64);
              rbx = r;
              fl = f; }
            // c3e21 mov rax,rbx
            rax = rbx;
            // c3e24 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3e28 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3e29 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3e2b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3e2d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3e2f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3e30 ret
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
            #[cfg(feature = "trace")] eprintln!("f_000e30c4 pc e30c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xe6250 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e6250<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe6250;
    loop {
        match pc {
        0xe6250 => {
            // block e6250
            #[cfg(feature = "trace")] eprintln!("f_000e6250 pc e6250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6250 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e6251 mov rbp,rsp
            rbp = rsp;
            // e6254 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e6256 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e6258 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e6259 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e625d mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e6260 mov rbx,rdi
            rbx = rdi;
            // e6263 mov ecx,DWORD PTR [rbx+0x5c]
            rcx = (m.r32(rbx.wrapping_add(0x5c)) as u64) as u32 as u64;
            // e6266 mov r8d,DWORD PTR [rbx+0x58]
            r8 = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // e626a mov r9d,DWORD PTR [rbx+0x54]
            r9 = (m.r32(rbx.wrapping_add(0x54)) as u64) as u32 as u64;
            // e626e mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // e6271 mov eax,DWORD PTR [rbx+0x50]
            rax = (m.r32(rbx.wrapping_add(0x50)) as u64) as u32 as u64;
            // e6274 mov DWORD PTR [rsp],eax
            m.w32(rsp, ((rax as u32 as u64)) as u32);
            // e6277 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e627b call 0xe605e
            { let r = { let t = m.env.code_va(0xe605e); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e6280 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // e6283 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e6285 mov rdi,rbx
            rdi = rbx;
            // e6288 call 0xe61e4
            { let r = { let t = m.env.code_va(0xe61e4); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e628d mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // e628f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e6291 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // e6294 div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // e6296 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e6299 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e629b cmovb r15d,ecx
            if fl.b() { r15 = ((rcx as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // e629f add r15d,eax
            { let (r, f) = add::<32>((r15 as u32 as u64), (rax as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // e62a2 mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // e62a5 mov rdi,rbx
            rdi = rbx;
            // e62a8 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e62ab call 0xe54b0
            { let r = { let t = m.env.code_va(0xe54b0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e62b0 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // e62b3 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e62b7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e62b8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e62ba pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e62bc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e62bd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeafd6 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eafd6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xeafd6;
    loop {
        match pc {
        0xeafd6 => {
            // block eafd6
            #[cfg(feature = "trace")] eprintln!("f_000eafd6 pc eafd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafd6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eafd7 mov rbp,rsp
            rbp = rsp;
            // eafda push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eafdc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eafdd sub rsp,0x290
            { let (r, f) = sub::<64>(rsp, 0x290u64);
              rsp = r;
              fl = f; }
            // eafe4 mov rbx,rdi
            rbx = rdi;
            // eafe7 lea r14,[rbp-0x298]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fd68);
            // eafee mov rdi,r14
            rdi = r14;
            // eaff1 call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaff6 mov edi,DWORD PTR [rbx+0x5c]
            rdi = (m.r32(rbx.wrapping_add(0x5c)) as u64) as u32 as u64;
            // eaff9 mov rsi,r14
            rsi = r14;
            // eaffc call 0x17202
            { let r = { let t = m.env.code_va(0x17202); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb001 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb003 je 0xeb013
            if fl.e() { pc = 0xeb013; continue; }
            pc = 0xeb005;
        }
        0xeb005 => {
            // block eb005
            #[cfg(feature = "trace")] eprintln!("f_000eafd6 pc eb005 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb005 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // eb008 mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // eb00c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eb00e mov rdi,rbx
            rdi = rbx;
            // eb011 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb013;
        }
        0xeb013 => {
            // block eb013
            #[cfg(feature = "trace")] eprintln!("f_000eafd6 pc eb013 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb013 lea rdi,[rbp-0x298]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd68);
            // eb01a call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb01f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // eb021 add rsp,0x290
            { let (r, f) = add::<64>(rsp, 0x290u64);
              rsp = r;
              fl = f; }
            // eb028 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb029 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb02b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb02c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeb02d => {
            // block eb02d
            #[cfg(feature = "trace")] eprintln!("f_000eafd6 pc eb02d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb02d mov rbx,rax
            rbx = rax;
            // eb030 lea rdi,[rbp-0x298]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd68);
            // eb037 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb03c mov rdi,rbx
            rdi = rbx;
            // eb03f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbc2c size=111
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbc2c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbbc2c;
    loop {
        match pc {
        0xbbc2c => {
            // block bbc2c
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc2c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbc2d mov rbp,rsp
            rbp = rsp;
            // bbc30 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bbc31 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bbc32 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // bbc34 lea eax,[rdi-0x2]
            rax = (rdi.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // bbc37 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // bbc3a ja 0xbbc68
            if fl.a() { pc = 0xbbc68; continue; }
            pc = 0xbbc3c;
        }
        0xbbc3c => {
            // block bbc3c
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc3c call 0x5c889
            { let r = { let t = m.env.code_va(0x5c889); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbc41 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // bbc43 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bbc48 cmp ecx,0x7
            fl = sub::<32>((rcx as u32 as u64), 0x7u64).1;
            // bbc4b ja 0xbbc94
            if fl.a() { pc = 0xbbc94; continue; }
            pc = 0xbbc4d;
        }
        0xbbc4d => {
            // block bbc4d
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc4d mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // bbc4f lea rcx,[rip+0x4707a] # 0x102cd0
            rcx = m.env.data_va(95389);
            // bbc56 lea rdx,[rip+0x47053] # 0x102cb0
            rdx = m.env.data_va(95357);
            // bbc5d test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // bbc5f cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbc63 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // bbc66 jmp 0xbbc94
            pc = 0xbbc94; continue;
        }
        0xbbc68 => {
            // block bbc68
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc68 mov eax,0x76
            rax = (0x76u64) as u32 as u64;
            // bbc6d test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // bbc6f je 0xbbc94
            if fl.e() { pc = 0xbbc94; continue; }
            pc = 0xbbc71;
        }
        0xbbc71 => {
            // block bbc71
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc71 cmp edi,0x1
            fl = sub::<32>((rdi as u32 as u64), 0x1u64).1;
            // bbc74 jne 0xbbc8f
            if fl.ne() { pc = 0xbbc8f; continue; }
            pc = 0xbbc76;
        }
        0xbbc76 => {
            // block bbc76
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc76 movzx ecx,dx
            rcx = ((rdx as u16 as u64)) as u32 as u64;
            // bbc79 mov eax,0x65
            rax = (0x65u64) as u32 as u64;
            // bbc7e cmp ecx,0x63
            fl = sub::<32>((rcx as u32 as u64), 0x63u64).1;
            // bbc81 je 0xbbc94
            if fl.e() { pc = 0xbbc94; continue; }
            pc = 0xbbc83;
        }
        0xbbc83 => {
            // block bbc83
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc83 lea rax,[rip+0x47016] # 0x102ca0
            rax = m.env.data_va(95341);
            // bbc8a mov eax,DWORD PTR [rax+rcx*4]
            rax = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // bbc8d jmp 0xbbc94
            pc = 0xbbc94; continue;
        }
        0xbbc8f => {
            // block bbc8f
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc8f mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            pc = 0xbbc94;
        }
        0xbbc94 => {
            // block bbc94
            #[cfg(feature = "trace")] eprintln!("f_000bbc2c pc bbc94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc94 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bbc98 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bbc99 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbc9a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8b6c size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008b6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8b6c;
    loop {
        match pc {
        0x8b6c => {
            // block 8b6c
            #[cfg(feature = "trace")] eprintln!("f_00008b6c pc 8b6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8b6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8b6d mov rbp,rsp
            rbp = rsp;
            // 8b70 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8b72 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8b74 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8b75 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 8b76 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 8b79 mov r14,rsi
            r14 = rsi;
            // 8b7c mov rbx,rdi
            rbx = rdi;
            // 8b7f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8b82 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 8b85 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8b8b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8b8d je 0x8b93
            if fl.e() { pc = 0x8b93; continue; }
            pc = 0x8b8f;
        }
        0x8b8f => {
            // block 8b8f
            #[cfg(feature = "trace")] eprintln!("f_00008b6c pc 8b8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8b8f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8b91 jmp 0x8bd1
            pc = 0x8bd1; continue;
        }
        0x8b93 => {
            // block 8b93
            #[cfg(feature = "trace")] eprintln!("f_00008b6c pc 8b93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8b93 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 8b9a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8b9d mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 8ba0 lea rcx,[rip+0xf65c9] # 0xff170
            rcx = m.env.data_va(80189);
            // 8ba7 mov esi,DWORD PTR [rcx+rdx*4]
            rsi = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8baa lea rcx,[rip+0xf695f] # 0xff510
            rcx = m.env.data_va(81117);
            // 8bb1 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8bb4 lea rdi,[rip+0xf6985] # 0xff540
            rdi = m.env.data_va(81165);
            // 8bbb mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8bbf lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 8bc3 mov rdi,rbx
            rdi = rbx;
            // 8bc6 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8bc9 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // 8bcd setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0x8bd1;
        }
        0x8bd1 => {
            // block 8bd1
            #[cfg(feature = "trace")] eprintln!("f_00008b6c pc 8bd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8bd1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 8bd5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8bd6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8bd8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8bda pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8bdb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8c4c size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008c4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8c4c;
    loop {
        match pc {
        0x8c4c => {
            // block 8c4c
            #[cfg(feature = "trace")] eprintln!("f_00008c4c pc 8c4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8c4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8c4d mov rbp,rsp
            rbp = rsp;
            // 8c50 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8c52 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8c54 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8c55 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 8c56 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 8c59 mov r14,rsi
            r14 = rsi;
            // 8c5c mov rbx,rdi
            rbx = rdi;
            // 8c5f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8c62 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 8c65 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8c6b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8c6d je 0x8c73
            if fl.e() { pc = 0x8c73; continue; }
            pc = 0x8c6f;
        }
        0x8c6f => {
            // block 8c6f
            #[cfg(feature = "trace")] eprintln!("f_00008c4c pc 8c6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8c6f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8c71 jmp 0x8cb1
            pc = 0x8cb1; continue;
        }
        0x8c73 => {
            // block 8c73
            #[cfg(feature = "trace")] eprintln!("f_00008c4c pc 8c73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8c73 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 8c7a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8c7d mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 8c80 lea rcx,[rip+0xf64e9] # 0xff170
            rcx = m.env.data_va(80189);
            // 8c87 mov esi,DWORD PTR [rcx+rdx*4]
            rsi = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8c8a lea rcx,[rip+0xf693f] # 0xff5d0
            rcx = m.env.data_va(81309);
            // 8c91 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8c94 lea rdi,[rip+0xf6965] # 0xff600
            rdi = m.env.data_va(81357);
            // 8c9b mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8c9f lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 8ca3 mov rdi,rbx
            rdi = rbx;
            // 8ca6 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8ca9 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // 8cad setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0x8cb1;
        }
        0x8cb1 => {
            // block 8cb1
            #[cfg(feature = "trace")] eprintln!("f_00008c4c pc 8cb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8cb1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 8cb5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8cb6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8cb8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8cba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8cbb ret
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
            #[cfg(feature = "trace")] eprintln!("f_000b5ee2 pc b5ee2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b5ee2 pc b5f0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5f0f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b5f11 jmp 0xb5f42
            pc = 0xb5f42; continue;
        }
        0xb5f13 => {
            // block b5f13
            #[cfg(feature = "trace")] eprintln!("f_000b5ee2 pc b5f13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b5ee2 pc b5f42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xd4070 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4070<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4070;
    loop {
        match pc {
        0xd4070 => {
            // block d4070
            #[cfg(feature = "trace")] eprintln!("f_000d4070 pc d4070 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4070 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4071 mov rbp,rsp
            rbp = rsp;
            // d4074 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4076 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4078 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4079 sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d407d mov rbx,rdx
            rbx = rdx;
            // d4080 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d4083 mov r14,rdi
            r14 = rdi;
            // d4086 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d4089 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // d408c movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // d408f lea esi,[rax+rax*8+0x1]
            rsi = (rax.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x1)) as u32 as u64;
            // d4093 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d4097 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // d409c call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d40a1 mov rax,rbx
            rax = rbx;
            // d40a4 shr rax,0x20
            { let (r, f) = shr::<64>(rax, 0x20u64, fl); rax = r; fl = f; }
            // d40a8 mov DWORD PTR [rbp-0x70],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), ((rax as u32 as u64)) as u32);
            // d40ab mov DWORD PTR [rbp-0x6c],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rbx as u32 as u64)) as u32);
            // d40ae lea rdx,[rbp-0x74]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // d40b2 mov rdi,r14
            rdi = r14;
            // d40b5 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d40b8 call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d40bd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d40bf je 0xd40d3
            if fl.e() { pc = 0xd40d3; continue; }
            pc = 0xd40c1;
        }
        0xd40c1 => {
            // block d40c1
            #[cfg(feature = "trace")] eprintln!("f_000d4070 pc d40c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d40c1 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d40c4 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // d40c8 mov rdi,r14
            rdi = r14;
            // d40cb call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d40d1 jmp 0xd40d5
            pc = 0xd40d5; continue;
        }
        0xd40d3 => {
            // block d40d3
            #[cfg(feature = "trace")] eprintln!("f_000d4070 pc d40d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d40d3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd40d5;
        }
        0xd40d5 => {
            // block d40d5
            #[cfg(feature = "trace")] eprintln!("f_000d4070 pc d40d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d40d5 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d40d9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d40da pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d40dc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d40de pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d40df ret
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
            #[cfg(feature = "trace")] eprintln!("f_000e2d84 pc e2d84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000e2d84 pc e2dbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000e2d84 pc e2de5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x1016e size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001016e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1016e;
    loop {
        match pc {
        0x1016e => {
            // block 1016e
            #[cfg(feature = "trace")] eprintln!("f_0001016e pc 1016e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1016e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1016f mov rbp,rsp
            rbp = rsp;
            // 10172 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10174 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10175 mov r14,rsi
            r14 = rsi;
            // 10178 mov rbx,rdi
            rbx = rdi;
            // 1017b mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1017f mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 10183 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 10185 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 10188 je 0x10194
            if fl.e() { pc = 0x10194; continue; }
            pc = 0x1018a;
        }
        0x1018a => {
            // block 1018a
            #[cfg(feature = "trace")] eprintln!("f_0001016e pc 1018a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1018a shl rcx,0x7
            { let (r, f) = shl::<64>(rcx, 0x7u64, fl); rcx = r; fl = f; }
            // 1018e dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // 10191 mov rsi,rcx
            rsi = rcx;
            pc = 0x10194;
        }
        0x10194 => {
            // block 10194
            #[cfg(feature = "trace")] eprintln!("f_0001016e pc 10194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10194 mov rdx,QWORD PTR [rbx+0x20]
            rdx = m.r64(rbx.wrapping_add(0x20)) as u64;
            // 10198 mov rcx,QWORD PTR [rbx+0x28]
            rcx = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 1019c sub rsi,rdx
            { let (r, f) = sub::<64>(rsi, rdx);
              rsi = r;
              fl = f; }
            // 1019f cmp rsi,rcx
            fl = sub::<64>(rsi, rcx).1;
            // 101a2 jne 0x101b8
            if fl.ne() { pc = 0x101b8; continue; }
            pc = 0x101a4;
        }
        0x101a4 => {
            // block 101a4
            #[cfg(feature = "trace")] eprintln!("f_0001016e pc 101a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 101a4 mov rdi,rbx
            rdi = rbx;
            // 101a7 call 0xeb876
            { let r = { let t = m.env.stub_va(0x43); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 101ac mov rcx,QWORD PTR [rbx+0x28]
            rcx = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 101b0 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 101b4 mov rdx,QWORD PTR [rbx+0x20]
            rdx = m.r64(rbx.wrapping_add(0x20)) as u64;
            pc = 0x101b8;
        }
        0x101b8 => {
            // block 101b8
            #[cfg(feature = "trace")] eprintln!("f_0001016e pc 101b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 101b8 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 101bb mov rsi,rdx
            rsi = rdx;
            // 101be shr rsi,0xa
            { let (r, f) = shr::<64>(rsi, 0xau64, fl); rsi = r; fl = f; }
            // 101c2 mov rax,QWORD PTR [rax+rsi*8]
            rax = m.r64(rax.wrapping_add(rsi.wrapping_mul(8))) as u64;
            // 101c6 and rdx,0x3ff
            { let r = rdx & 0x3ffu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 101cd mov esi,DWORD PTR [r14]
            rsi = (m.r32(r14) as u64) as u32 as u64;
            // 101d0 mov DWORD PTR [rax+rdx*4],esi
            m.w32(rax.wrapping_add(rdx.wrapping_mul(4)), ((rsi as u32 as u64)) as u32);
            // 101d3 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 101d6 mov QWORD PTR [rbx+0x28],rcx
            m.w64(rbx.wrapping_add(0x28), (rcx) as u64);
            // 101da pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 101db pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 101dd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 101de ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43d26 size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043d26<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43d26;
    loop {
        match pc {
        0x43d26 => {
            // block 43d26
            #[cfg(feature = "trace")] eprintln!("f_00043d26 pc 43d26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43d26 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43d27 mov rbp,rsp
            rbp = rsp;
            // 43d2a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43d2c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43d2e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 43d30 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43d31 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // 43d34 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 43d36 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 43d39 mov r12,rdi
            r12 = rdi;
            // 43d3c mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43d41 call 0xcc811
            { let r = { let t = m.env.code_va(0xcc811); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43d46 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 43d48 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43d4a cmp ebx,0x3f
            fl = sub::<32>((rbx as u32 as u64), 0x3fu64).1;
            // 43d4d ja 0x43d8e
            if fl.a() { pc = 0x43d8e; continue; }
            pc = 0x43d4f;
        }
        0x43d4f => {
            // block 43d4f
            #[cfg(feature = "trace")] eprintln!("f_00043d26 pc 43d4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43d4f cmp r15d,0x7
            fl = sub::<32>((r15 as u32 as u64), 0x7u64).1;
            // 43d53 ja 0x43d8e
            if fl.a() { pc = 0x43d8e; continue; }
            pc = 0x43d55;
        }
        0x43d55 => {
            // block 43d55
            #[cfg(feature = "trace")] eprintln!("f_00043d26 pc 43d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43d55 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 43d57 je 0x43d8e
            if fl.e() { pc = 0x43d8e; continue; }
            pc = 0x43d59;
        }
        0x43d59 => {
            // block 43d59
            #[cfg(feature = "trace")] eprintln!("f_00043d26 pc 43d59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43d59 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 43d5d mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 43d5f mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 43d62 lea rdx,[rdx+rdx*2]
            rdx = rdx.wrapping_add(rdx.wrapping_mul(2));
            // 43d66 shl rdx,0x5
            { let (r, f) = shl::<64>(rdx, 0x5u64, fl); rdx = r; fl = f; }
            // 43d6a lea rdi,[rip+0xbc34f] # 0x1000c0
            rdi = m.env.data_va(84109);
            // 43d71 add rdi,rdx
            { let (r, f) = add::<64>(rdi, rdx);
              rdi = r;
              fl = f; }
            // 43d74 lea rbx,[rcx+rcx*2]
            rbx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // 43d78 mov esi,DWORD PTR [rdi+rbx*4]
            rsi = (m.r32(rdi.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43d7b movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // 43d7f mov ecx,DWORD PTR [rdi+rbx*4+0x4]
            rcx = (m.r32(rdi.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x4)) as u64) as u32 as u64;
            // 43d83 mov r8d,DWORD PTR [rdi+rbx*4+0x8]
            r8 = (m.r32(rdi.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x8)) as u64) as u32 as u64;
            // 43d88 mov rdi,r12
            rdi = r12;
            // 43d8b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43d8e;
        }
        0x43d8e => {
            // block 43d8e
            #[cfg(feature = "trace")] eprintln!("f_00043d26 pc 43d8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43d8e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43d8f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 43d91 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43d93 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43d95 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43d96 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6ec8 size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6ec8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd6ec8;
    loop {
        match pc {
        0xd6ec8 => {
            // block d6ec8
            #[cfg(feature = "trace")] eprintln!("f_000d6ec8 pc d6ec8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ec8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6ec9 mov rbp,rsp
            rbp = rsp;
            // d6ecc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6ecd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d6ece mov rbx,rdi
            rbx = rdi;
            // d6ed1 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // d6ed8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6edb call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6ee1 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d6ee3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d6ee5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d6ee7 je 0xd6f32
            if fl.e() { pc = 0xd6f32; continue; }
            pc = 0xd6ee9;
        }
        0xd6ee9 => {
            // block d6ee9
            #[cfg(feature = "trace")] eprintln!("f_000d6ec8 pc d6ee9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ee9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6eec mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d6ef1 mov edx,0x9f
            rdx = (0x9fu64) as u32 as u64;
            // d6ef6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d6efb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6efe mov rdi,rbx
            rdi = rbx;
            // d6f01 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6f04 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6f07 mov rdi,rbx
            rdi = rbx;
            // d6f0a call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6f10 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6f13 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d6f17 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d6f1c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d6f21 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6f24 mov rdi,rbx
            rdi = rbx;
            // d6f27 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6f2a mov eax,0xffffff
            rax = (0xffffffu64) as u32 as u64;
            // d6f2f and eax,DWORD PTR [rbp-0xc]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd6f32;
        }
        0xd6f32 => {
            // block d6f32
            #[cfg(feature = "trace")] eprintln!("f_000d6ec8 pc d6f32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6f32 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d6f36 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6f37 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6f38 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2ace size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2ace<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2ace;
    loop {
        match pc {
        0xe2ace => {
            // block e2ace
            #[cfg(feature = "trace")] eprintln!("f_000e2ace pc e2ace rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2ace push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2acf mov rbp,rsp
            rbp = rsp;
            // e2ad2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2ad3 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e2ad7 mov rbx,rcx
            rbx = rcx;
            // e2ada xorps xmm0,xmm0
            x0 = 0;
            // e2add movaps XMMWORD PTR [rbp-0x20],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), x0);
            // e2ae1 mov QWORD PTR [rbp-0x10],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u64);
            // e2ae9 lea rcx,[rbp-0x20]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e2aed call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2af2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2af4 je 0xe2b02
            if fl.e() { pc = 0xe2b02; continue; }
            pc = 0xe2af6;
        }
        0xe2af6 => {
            // block e2af6
            #[cfg(feature = "trace")] eprintln!("f_000e2ace pc e2af6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2af6 test BYTE PTR [rbp-0x20],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64 & 0x1u64);
            // e2afa jne 0xe2b06
            if fl.ne() { pc = 0xe2b06; continue; }
            pc = 0xe2afc;
        }
        0xe2afc => {
            // block e2afc
            #[cfg(feature = "trace")] eprintln!("f_000e2ace pc e2afc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2afc lea rdi,[rbp-0x1f]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe1);
            // e2b00 jmp 0xe2b0a
            pc = 0xe2b0a; continue;
        }
        0xe2b02 => {
            // block e2b02
            #[cfg(feature = "trace")] eprintln!("f_000e2ace pc e2b02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b02 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2b04 jmp 0xe2b1a
            pc = 0xe2b1a; continue;
        }
        0xe2b06 => {
            // block e2b06
            #[cfg(feature = "trace")] eprintln!("f_000e2ace pc e2b06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b06 mov rdi,QWORD PTR [rbp-0x10]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64;
            pc = 0xe2b0a;
        }
        0xe2b0a => {
            // block e2b0a
            #[cfg(feature = "trace")] eprintln!("f_000e2ace pc e2b0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b0a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e2b0c mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // e2b11 call 0xec2de
            { let r = { let t = m.env.stub_va(0xaab); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2b16 mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // e2b18 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe2b1a;
        }
        0xe2b1a => {
            // block e2b1a
            #[cfg(feature = "trace")] eprintln!("f_000e2ace pc e2b1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b1a lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e2b1e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2b23 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2b25 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e2b29 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2b2a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2b2b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe2b2c => {
            // block e2b2c
            #[cfg(feature = "trace")] eprintln!("f_000e2ace pc e2b2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b2c mov rbx,rax
            rbx = rax;
            // e2b2f lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e2b33 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2b38 mov rdi,rbx
            rdi = rbx;
            // e2b3b call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4ce0 size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004ce0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4ce0;
    loop {
        match pc {
        0x4ce0 => {
            // block 4ce0
            #[cfg(feature = "trace")] eprintln!("f_00004ce0 pc 4ce0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4ce0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4ce1 mov rbp,rsp
            rbp = rsp;
            // 4ce4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4ce6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4ce8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4ce9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4cea mov r14,rdx
            r14 = rdx;
            // 4ced mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 4cf0 mov rbx,rdi
            rbx = rdi;
            // 4cf3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4cf6 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4cfc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4cfe je 0x4d04
            if fl.e() { pc = 0x4d04; continue; }
            pc = 0x4d00;
        }
        0x4d00 => {
            // block 4d00
            #[cfg(feature = "trace")] eprintln!("f_00004ce0 pc 4d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d00 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4d02 jmp 0x4d48
            pc = 0x4d48; continue;
        }
        0x4d04 => {
            // block 4d04
            #[cfg(feature = "trace")] eprintln!("f_00004ce0 pc 4d04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d04 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 4d0b mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 4d0e lea rcx,[rip+0xfa54b] # 0xff260
            rcx = m.env.data_va(80429);
            // 4d15 mov esi,DWORD PTR [rcx+rax*4]
            rsi = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 4d18 lea rcx,[rip+0xfa571] # 0xff290
            rcx = m.env.data_va(80477);
            // 4d1f mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 4d22 lea rdx,[rip+0xfa597] # 0xff2c0
            rdx = m.env.data_va(80525);
            // 4d29 mov r8d,DWORD PTR [rdx+rax*4]
            r8 = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 4d2d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4d30 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 4d34 mov rdi,rbx
            rdi = rbx;
            // 4d37 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d3a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4d3c mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 4d41 cmovne ecx,DWORD PTR [rbp-0x1c]
            if fl.ne() { rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 4d45 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            pc = 0x4d48;
        }
        0x4d48 => {
            // block 4d48
            #[cfg(feature = "trace")] eprintln!("f_00004ce0 pc 4d48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d48 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4d4c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4d4d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4d4f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4d51 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4d52 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11bf1 size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011bf1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x11bf1;
    loop {
        match pc {
        0x11bf1 => {
            // block 11bf1
            #[cfg(feature = "trace")] eprintln!("f_00011bf1 pc 11bf1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11bf1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11bf2 mov rbp,rsp
            rbp = rsp;
            // 11bf5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11bf7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11bf9 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11bfa push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 11bfb mov rbx,rdi
            rbx = rdi;
            // 11bfe movabs rax,0x249249249249249
            rax = 0x249_2492_4924_9249u64;
            // 11c08 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 11c0b cmovg rbx,rax
            if fl.g() { rbx = rax; }
            // 11c0f xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 11c12 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 11c15 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 11c1a jle 0x11c57
            if fl.le() { pc = 0x11c57; continue; }
            pc = 0x11c1c;
        }
        0x11c1c => {
            // block 11c1c
            #[cfg(feature = "trace")] eprintln!("f_00011bf1 pc 11c1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11c1c mov r15,QWORD PTR [rip+0x12e58d] # 0x1401b0
            r15 = m.r64(m.env.data_va(346493)) as u64;
            // 11c23 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x11c26;
        }
        0x11c26 => {
            // block 11c26
            #[cfg(feature = "trace")] eprintln!("f_00011bf1 pc 11c26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11c26 imul rdi,rbx,0x38
            { let (r, f) = imul::<64>(rbx, 0x38u64); rdi = r; fl = f; }
            // 11c2a mov rsi,r15
            rsi = r15;
            // 11c2d call 0xec14c
            { let r = { let t = m.env.stub_va(0x919); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11c32 test rax,rax
            fl = logic::<64>(rax & rax);
            // 11c35 jne 0x11c54
            if fl.ne() { pc = 0x11c54; continue; }
            pc = 0x11c37;
        }
        0x11c37 => {
            // block 11c37
            #[cfg(feature = "trace")] eprintln!("f_00011bf1 pc 11c37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11c37 mov rax,rbx
            rax = rbx;
            // 11c3a shr rax,0x3f
            { let (r, f) = shr::<64>(rax, 0x3fu64, fl); rax = r; fl = f; }
            // 11c3e add rax,rbx
            { let (r, f) = add::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // 11c41 sar rax,1
            { let (r, f) = sar::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 11c44 cmp rbx,0x1
            fl = sub::<64>(rbx, 0x1u64).1;
            // 11c48 mov rbx,rax
            rbx = rax;
            // 11c4b mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 11c50 jg 0x11c26
            if fl.g() { pc = 0x11c26; continue; }
            pc = 0x11c52;
        }
        0x11c52 => {
            // block 11c52
            #[cfg(feature = "trace")] eprintln!("f_00011bf1 pc 11c52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11c52 jmp 0x11c57
            pc = 0x11c57; continue;
        }
        0x11c54 => {
            // block 11c54
            #[cfg(feature = "trace")] eprintln!("f_00011bf1 pc 11c54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11c54 mov r14,rbx
            r14 = rbx;
            pc = 0x11c57;
        }
        0x11c57 => {
            // block 11c57
            #[cfg(feature = "trace")] eprintln!("f_00011bf1 pc 11c57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11c57 mov rdx,r14
            rdx = r14;
            // 11c5a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 11c5e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11c5f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11c61 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11c63 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11c64 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5d438 size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005d438<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5d438;
    loop {
        match pc {
        0x5d438 => {
            // block 5d438
            #[cfg(feature = "trace")] eprintln!("f_0005d438 pc 5d438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d438 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5d439 mov rbp,rsp
            rbp = rsp;
            // 5d43c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5d43d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5d43e mov rbx,rdi
            rbx = rdi;
            // 5d441 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 5d444 jne 0x5d466
            if fl.ne() { pc = 0x5d466; continue; }
            pc = 0x5d446;
        }
        0x5d446 => {
            // block 5d446
            #[cfg(feature = "trace")] eprintln!("f_0005d438 pc 5d446 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d446 lea rax,[rip+0xce923] # 0x12bd70
            rax = m.env.data_va(263485);
            // 5d44d lea rsi,[rip+0xce91f] # 0x12bd73
            rsi = m.env.data_va(263488);
            // 5d454 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5d456 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 5d45a mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // 5d45f mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // 5d464 jmp 0x5d488
            pc = 0x5d488; continue;
        }
        0x5d466 => {
            // block 5d466
            #[cfg(feature = "trace")] eprintln!("f_0005d438 pc 5d466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d466 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 5d468 jne 0x5d48e
            if fl.ne() { pc = 0x5d48e; continue; }
            pc = 0x5d46a;
        }
        0x5d46a => {
            // block 5d46a
            #[cfg(feature = "trace")] eprintln!("f_0005d438 pc 5d46a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d46a lea rax,[rip+0xce8e3] # 0x12bd54
            rax = m.env.data_va(263457);
            // 5d471 lea rsi,[rip+0xce8e0] # 0x12bd58
            rsi = m.env.data_va(263461);
            // 5d478 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5d47a cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 5d47e mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 5d483 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            pc = 0x5d488;
        }
        0x5d488 => {
            // block 5d488
            #[cfg(feature = "trace")] eprintln!("f_0005d438 pc 5d488 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d488 cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            // 5d48c jmp 0x5d49a
            pc = 0x5d49a; continue;
        }
        0x5d48e => {
            // block 5d48e
            #[cfg(feature = "trace")] eprintln!("f_0005d438 pc 5d48e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d48e lea rsi,[rip+0xce753] # 0x12bbe8
            rsi = m.env.data_va(263093);
            // 5d495 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            pc = 0x5d49a;
        }
        0x5d49a => {
            // block 5d49a
            #[cfg(feature = "trace")] eprintln!("f_0005d438 pc 5d49a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d49a mov rdi,rbx
            rdi = rbx;
            // 5d49d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d4a2 mov rax,rbx
            rax = rbx;
            // 5d4a5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5d4a9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5d4aa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5d4ab ret
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
            #[cfg(feature = "trace")] eprintln!("f_000a28c0 pc a28c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000a28c0 pc a28d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a28d1 mov QWORD PTR [rdi+0x58],rax
            m.w64(rdi.wrapping_add(0x58), (rax) as u64);
            // a28d5 mov r10,rax
            r10 = rax;
            pc = 0xa28d8;
        }
        0xa28d8 => {
            // block a28d8
            #[cfg(feature = "trace")] eprintln!("f_000a28c0 pc a28d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000a28c0 pc a28ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a28ea cmp esi,0xffffffff
            fl = sub::<32>((rsi as u32 as u64), 0xffffffffu64).1;
            // a28ed je 0xa2919
            if fl.e() { pc = 0xa2919; continue; }
            pc = 0xa28ef;
        }
        0xa28ef => {
            // block a28ef
            #[cfg(feature = "trace")] eprintln!("f_000a28c0 pc a28ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a28ef test BYTE PTR [rdi+0x60],0x10
            fl = logic::<8>(m.r8(rdi.wrapping_add(0x60)) as u64 & 0x10u64);
            // a28f3 jne 0xa2903
            if fl.ne() { pc = 0xa2903; continue; }
            pc = 0xa28f5;
        }
        0xa28f5 => {
            // block a28f5
            #[cfg(feature = "trace")] eprintln!("f_000a28c0 pc a28f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000a28c0 pc a2903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000a28c0 pc a2919 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000a28c0 pc a292a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a292a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a292b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb56fc size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b56fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb56fc;
    loop {
        match pc {
        0xb56fc => {
            // block b56fc
            #[cfg(feature = "trace")] eprintln!("f_000b56fc pc b56fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b56fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b56fd mov rbp,rsp
            rbp = rsp;
            // b5700 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5702 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5704 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5706 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5707 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b570b mov r15,rsi
            r15 = rsi;
            // b570e mov r14,rdi
            r14 = rdi;
            // b5711 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // b5715 mov rdx,r15
            rdx = r15;
            // b5718 call 0xebbf4
            { let r = { let t = m.env.stub_va(0x3c1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b571d mov r12,rax
            r12 = rax;
            // b5720 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // b5724 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5727 jne 0xb575c
            if fl.ne() { pc = 0xb575c; continue; }
            pc = 0xb5729;
        }
        0xb5729 => {
            // block b5729
            #[cfg(feature = "trace")] eprintln!("f_000b56fc pc b5729 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5729 mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // b572e call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5733 mov rbx,rax
            rbx = rax;
            // b5736 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // b5739 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // b573c mov DWORD PTR [rbx+0x20],0x0
            m.w32(rbx.wrapping_add(0x20), (0x0u64) as u32);
            // b5743 mov DWORD PTR [rbx+0x24],0x0
            m.w32(rbx.wrapping_add(0x24), (0x0u64) as u32);
            // b574a mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b574e mov rdi,r14
            rdi = r14;
            // b5751 mov rdx,r12
            rdx = r12;
            // b5754 mov rcx,rbx
            rcx = rbx;
            // b5757 call 0xebc12
            { let r = { let t = m.env.stub_va(0x3df); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb575c;
        }
        0xb575c => {
            // block b575c
            #[cfg(feature = "trace")] eprintln!("f_000b56fc pc b575c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b575c add rbx,0x20
            { let (r, f) = add::<64>(rbx, 0x20u64);
              rbx = r;
              fl = f; }
            // b5760 mov rax,rbx
            rax = rbx;
            // b5763 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b5767 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5768 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b576a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b576c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b576e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b576f ret
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
            #[cfg(feature = "trace")] eprintln!("f_000b5770 pc b5770 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b5770 pc b579d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b5770 pc b57d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xe0d04 size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0d04<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0d04;
    loop {
        match pc {
        0xe0d04 => {
            // block e0d04
            #[cfg(feature = "trace")] eprintln!("f_000e0d04 pc e0d04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d04 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0d05 mov rbp,rsp
            rbp = rsp;
            // e0d08 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0d0a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0d0b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0d0f mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e0d11 mov r14,rdi
            r14 = rdi;
            // e0d14 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0d17 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0d1b call QWORD PTR [rax+0x1ae8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0d21 mov esi,DWORD PTR [rbp-0x14]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // e0d24 and esi,0xffffffc0
            { let r = (rsi as u32 as u64) & 0xffffffc0u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0d27 mov DWORD PTR [rbp-0x14],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rsi as u32 as u64)) as u32);
            // e0d2a mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e0d2c cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // e0d2f ja 0xe0d51
            if fl.a() { pc = 0xe0d51; continue; }
            pc = 0xe0d31;
        }
        0xe0d31 => {
            // block e0d31
            #[cfg(feature = "trace")] eprintln!("f_000e0d04 pc e0d31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d31 lea rcx,[rip+0x30] # 0xe0d68
            rcx = m.env.code_va(0xe0d68);
            // e0d38 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // e0d3c add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // e0d3f jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xe0d41 => {
            // block e0d41
            #[cfg(feature = "trace")] eprintln!("f_000e0d04 pc e0d41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d41 or esi,0x5
            { let r = (rsi as u32 as u64) | 0x5u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0d44 jmp 0xe0d4e
            pc = 0xe0d4e; continue;
        }
        0xe0d46 => {
            // block e0d46
            #[cfg(feature = "trace")] eprintln!("f_000e0d04 pc e0d46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d46 or esi,0x9
            { let r = (rsi as u32 as u64) | 0x9u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0d49 jmp 0xe0d4e
            pc = 0xe0d4e; continue;
        }
        0xe0d4b => {
            // block e0d4b
            #[cfg(feature = "trace")] eprintln!("f_000e0d04 pc e0d4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d4b or esi,0x1
            { let r = (rsi as u32 as u64) | 0x1u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xe0d4e;
        }
        0xe0d4e => {
            // block e0d4e
            #[cfg(feature = "trace")] eprintln!("f_000e0d04 pc e0d4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d4e mov DWORD PTR [rbp-0x14],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rsi as u32 as u64)) as u32);
            pc = 0xe0d51;
        }
        0xe0d51 => {
            // block e0d51
            #[cfg(feature = "trace")] eprintln!("f_000e0d04 pc e0d51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d51 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0d54 mov rdi,r14
            rdi = r14;
            // e0d57 call QWORD PTR [rax+0x1ae0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe0d5d;
        }
        0xe0d5d => {
            // block e0d5d
            #[cfg(feature = "trace")] eprintln!("f_000e0d04 pc e0d5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d5d add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0d61 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0d62 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0d64 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0d65 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe7554 size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7554<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe7554;
    loop {
        match pc {
        0xe7554 => {
            // block e7554
            #[cfg(feature = "trace")] eprintln!("f_000e7554 pc e7554 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7554 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7555 mov rbp,rsp
            rbp = rsp;
            // e7558 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e7559 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e755d mov rbx,rdi
            rbx = rdi;
            // e7560 xorps xmm0,xmm0
            x0 = 0;
            // e7563 movaps XMMWORD PTR [rbp-0x20],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), x0);
            // e7567 mov QWORD PTR [rbp-0x10],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u64);
            // e756f add rsi,0x20
            { let (r, f) = add::<64>(rsi, 0x20u64);
              rsi = r;
              fl = f; }
            // e7573 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e7577 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e757c movzx edx,BYTE PTR [rbp-0x20]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // e7580 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // e7583 jne 0xe758e
            if fl.ne() { pc = 0xe758e; continue; }
            pc = 0xe7585;
        }
        0xe7585 => {
            // block e7585
            #[cfg(feature = "trace")] eprintln!("f_000e7554 pc e7585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7585 lea rsi,[rbp-0x1f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe1);
            // e7589 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e758c jmp 0xe7596
            pc = 0xe7596; continue;
        }
        0xe758e => {
            // block e758e
            #[cfg(feature = "trace")] eprintln!("f_000e7554 pc e758e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e758e mov rdx,QWORD PTR [rbp-0x18]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e7592 mov rsi,QWORD PTR [rbp-0x10]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64;
            pc = 0xe7596;
        }
        0xe7596 => {
            // block e7596
            #[cfg(feature = "trace")] eprintln!("f_000e7554 pc e7596 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7596 mov rdi,rbx
            rdi = rbx;
            // e7599 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e759e lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e75a2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e75a7 mov rax,rbx
            rax = rbx;
            // e75aa add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e75ae pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e75af pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e75b0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe75b1 => {
            // block e75b1
            #[cfg(feature = "trace")] eprintln!("f_000e7554 pc e75b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e75b1 mov rbx,rax
            rbx = rax;
            // e75b4 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e75b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e75bd mov rdi,rbx
            rdi = rbx;
            // e75c0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x58cc size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000058cc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x58cc;
    loop {
        match pc {
        0x58cc => {
            // block 58cc
            #[cfg(feature = "trace")] eprintln!("f_000058cc pc 58cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58cc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 58cd mov rbp,rsp
            rbp = rsp;
            // 58d0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 58d2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58d4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58d5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 58d6 mov r14,rdx
            r14 = rdx;
            // 58d9 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 58dc mov rbx,rdi
            rbx = rdi;
            // 58df mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 58e2 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58e8 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 58ea call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58ef movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 58f2 cmp eax,r15d
            fl = sub::<32>((rax as u32 as u64), (r15 as u32 as u64)).1;
            // 58f5 jbe 0x5934
            if fl.be() { pc = 0x5934; continue; }
            pc = 0x58f7;
        }
        0x58f7 => {
            // block 58f7
            #[cfg(feature = "trace")] eprintln!("f_000058cc pc 58f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58f7 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 58fe mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5901 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 5905 lea rdx,[rip+0xf9a74] # 0xff380
            rdx = m.env.data_va(80717);
            // 590c mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 590f lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 5913 mov ecx,0x2000
            rcx = (0x2000u64) as u32 as u64;
            // 5918 mov r8d,0xd
            r8 = (0xdu64) as u32 as u64;
            // 591e mov rdi,rbx
            rdi = rbx;
            // 5921 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5924 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5926 je 0x5934
            if fl.e() { pc = 0x5934; continue; }
            pc = 0x5928;
        }
        0x5928 => {
            // block 5928
            #[cfg(feature = "trace")] eprintln!("f_000058cc pc 5928 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5928 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // 592c setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            // 5930 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 5932 jmp 0x5936
            pc = 0x5936; continue;
        }
        0x5934 => {
            // block 5934
            #[cfg(feature = "trace")] eprintln!("f_000058cc pc 5934 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5934 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5936;
        }
        0x5936 => {
            // block 5936
            #[cfg(feature = "trace")] eprintln!("f_000058cc pc 5936 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5936 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 593a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 593b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 593d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 593f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5940 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf15e size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f15e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf15e;
    loop {
        match pc {
        0xf15e => {
            // block f15e
            #[cfg(feature = "trace")] eprintln!("f_0000f15e pc f15e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f15e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f15f mov rbp,rsp
            rbp = rsp;
            // f162 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f164 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f165 sub rsp,0x80
            { let (r, f) = sub::<64>(rsp, 0x80u64);
              rsp = r;
              fl = f; }
            // f16c mov rbx,rdi
            rbx = rdi;
            // f16f lea r14,[rbp-0x90]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // f176 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // f17b mov rdi,r14
            rdi = r14;
            // f17e call 0x2ccf6
            { let r = { let t = m.env.code_va(0x2ccf6); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f183 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f186 mov rax,QWORD PTR [rax+0xd0]
            rax = m.r64(rax.wrapping_add(0xd0)) as u64;
            // f18d mov rdi,rbx
            rdi = rbx;
            // f190 mov rsi,r14
            rsi = r14;
            // f193 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f195 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // f197 lea rdi,[rbp-0x44]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // f19b call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f1a0 lea rdi,[rbp-0x6c]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff94);
            // f1a4 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f1a9 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // f1ab add rsp,0x80
            { let (r, f) = add::<64>(rsp, 0x80u64);
              rsp = r;
              fl = f; }
            // f1b2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f1b3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f1b5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f1b6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xf1b7 => {
            // block f1b7
            #[cfg(feature = "trace")] eprintln!("f_0000f15e pc f1b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f1b7 mov rbx,rax
            rbx = rax;
            // f1ba lea rdi,[rbp-0x44]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // f1be call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f1c3 lea rdi,[rbp-0x6c]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff94);
            // f1c7 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f1cc mov rdi,rbx
            rdi = rbx;
            // f1cf call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x5a7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            #[cfg(feature = "trace")] eprintln!("f_000d36d8 pc d36d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d36d8 pc d36e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d36d8 pc d373b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d36d8 pc d3744 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3744 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3746;
        }
        0xd3746 => {
            // block d3746
            #[cfg(feature = "trace")] eprintln!("f_000d36d8 pc d3746 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xd3aac size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3aac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd3aac;
    loop {
        match pc {
        0xd3aac => {
            // block d3aac
            #[cfg(feature = "trace")] eprintln!("f_000d3aac pc d3aac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3aac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3aad mov rbp,rsp
            rbp = rsp;
            // d3ab0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3ab2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d3ab4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d3ab6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d3ab7 sub rsp,0x60
            { let (r, f) = sub::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // d3abb mov r14,rdx
            r14 = rdx;
            // d3abe mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d3ac0 mov r15,rdi
            r15 = rdi;
            // d3ac3 mov DWORD PTR [r14],ebx
            m.w32(r14, ((rbx as u32 as u64)) as u32);
            // d3ac6 lea r12,[rbp-0x80]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // d3aca mov esi,0x10
            rsi = (0x10u64) as u32 as u64;
            // d3acf mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // d3ad4 mov rdi,r12
            rdi = r12;
            // d3ad7 call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3adc xorps xmm0,xmm0
            x0 = 0;
            // d3adf movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // d3ae3 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d3ae7 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // d3aeb movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // d3aef movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // d3af3 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // d3af7 mov DWORD PTR [rbp-0x80],0x4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x4u64) as u32);
            // d3afe mov DWORD PTR [rbp-0x7c],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff84), ((rbx as u32 as u64)) as u32);
            // d3b01 mov QWORD PTR [rbp-0x40],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r14) as u64);
            // d3b05 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d3b08 mov rdi,r15
            rdi = r15;
            // d3b0b mov rsi,r12
            rsi = r12;
            // d3b0e call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3b14 add rsp,0x60
            { let (r, f) = add::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // d3b18 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3b19 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d3b1b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3b1d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3b1f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3b20 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdc460 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dc460<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdc460;
    loop {
        match pc {
        0xdc460 => {
            // block dc460
            #[cfg(feature = "trace")] eprintln!("f_000dc460 pc dc460 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc460 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc461 mov rbp,rsp
            rbp = rsp;
            // dc464 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dc466 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dc468 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dc46a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dc46c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dc46d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // dc46e mov r15,rsi
            r15 = rsi;
            // dc471 mov r14,rdi
            r14 = rdi;
            // dc474 mov rbx,QWORD PTR [r14+0x258]
            rbx = m.r64(r14.wrapping_add(0x258)) as u64;
            // dc47b mov r12,QWORD PTR [r14+0x260]
            r12 = m.r64(r14.wrapping_add(0x260)) as u64;
            // dc482 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // dc485 je 0xdc4a5
            if fl.e() { pc = 0xdc4a5; continue; }
            pc = 0xdc487;
        }
        0xdc487 => {
            // block dc487
            #[cfg(feature = "trace")] eprintln!("f_000dc460 pc dc487 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc487 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xdc48a;
        }
        0xdc48a => {
            // block dc48a
            #[cfg(feature = "trace")] eprintln!("f_000dc460 pc dc48a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc48a mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // dc48d mov rsi,r15
            rsi = r15;
            // dc490 call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc495 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // dc497 je 0xdc4b4
            if fl.e() { pc = 0xdc4b4; continue; }
            pc = 0xdc499;
        }
        0xdc499 => {
            // block dc499
            #[cfg(feature = "trace")] eprintln!("f_000dc460 pc dc499 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc499 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // dc49c add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // dc4a0 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // dc4a3 jne 0xdc48a
            if fl.ne() { pc = 0xdc48a; continue; }
            pc = 0xdc4a5;
        }
        0xdc4a5 => {
            // block dc4a5
            #[cfg(feature = "trace")] eprintln!("f_000dc460 pc dc4a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc4a5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // dc4a9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc4aa pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dc4ac pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dc4ae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc4b0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dc4b2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc4b3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdc4b4 => {
            // block dc4b4
            #[cfg(feature = "trace")] eprintln!("f_000dc460 pc dc4b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc4b4 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dc4b7 mov rax,QWORD PTR [rax+0x1970]
            rax = m.r64(rax.wrapping_add(0x1970)) as u64;
            // dc4be movzx esi,r13w
            rsi = ((r13 as u16 as u64)) as u32 as u64;
            // dc4c2 mov rdi,r14
            rdi = r14;
            // dc4c5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // dc4c9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc4ca pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dc4cc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dc4ce pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc4d0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dc4d2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc4d3 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
