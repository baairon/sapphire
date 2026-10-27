//! Mechanically lifted functions (scripts/lift_run.py), module h006. Each one is checked against the original
//! in the world harness (tests/world_h006.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x44a74 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044a74<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44a74;
    loop {
        match pc {
        0x44a74 => {
            // block 44a74
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44a74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44a74 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44a75 mov rbp,rsp
            rbp = rsp;
            // 44a78 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 44a7c add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 44a80 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 44a83 je 0x44ab7
            if fl.e() { pc = 0x44ab7; continue; }
            pc = 0x44a85;
        }
        0x44a85 => {
            // block 44a85
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44a85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44a85 mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // 44a87 mov rsi,rdi
            rsi = rdi;
            pc = 0x44a8a;
        }
        0x44a8a => {
            // block 44a8a
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44a8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44a8a mov rax,rdx
            rax = rdx;
            pc = 0x44a8d;
        }
        0x44a8d => {
            // block 44a8d
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44a8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44a8d cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // 44a90 jge 0x44a9d
            if fl.ge() { pc = 0x44a9d; continue; }
            pc = 0x44a92;
        }
        0x44a92 => {
            // block 44a92
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44a92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44a92 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 44a96 test rax,rax
            fl = logic::<64>(rax & rax);
            // 44a99 jne 0x44a8d
            if fl.ne() { pc = 0x44a8d; continue; }
            pc = 0x44a9b;
        }
        0x44a9b => {
            // block 44a9b
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44a9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44a9b jmp 0x44aaa
            pc = 0x44aaa; continue;
        }
        0x44a9d => {
            // block 44a9d
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44a9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44a9d mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 44aa0 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 44aa3 mov rsi,rax
            rsi = rax;
            // 44aa6 jne 0x44a8a
            if fl.ne() { pc = 0x44a8a; continue; }
            pc = 0x44aa8;
        }
        0x44aa8 => {
            // block 44aa8
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44aa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44aa8 jmp 0x44aad
            pc = 0x44aad; continue;
        }
        0x44aaa => {
            // block 44aaa
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44aaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44aaa mov rax,rsi
            rax = rsi;
            pc = 0x44aad;
        }
        0x44aad => {
            // block 44aad
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44aad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44aad cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 44ab0 je 0x44ab7
            if fl.e() { pc = 0x44ab7; continue; }
            pc = 0x44ab2;
        }
        0x44ab2 => {
            // block 44ab2
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44ab2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44ab2 cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // 44ab5 jge 0x44aba
            if fl.ge() { pc = 0x44aba; continue; }
            pc = 0x44ab7;
        }
        0x44ab7 => {
            // block 44ab7
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44ab7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44ab7 mov rax,rdi
            rax = rdi;
            pc = 0x44aba;
        }
        0x44aba => {
            // block 44aba
            #[cfg(feature = "trace")] eprintln!("f_00044a74 pc 44aba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44aba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44abb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6556e size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006556e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6556e;
    loop {
        match pc {
        0x6556e => {
            // block 6556e
            #[cfg(feature = "trace")] eprintln!("f_0006556e pc 6556e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6556e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6556f mov rbp,rsp
            rbp = rsp;
            // 65572 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 65574 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 65576 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65578 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65579 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 6557d mov r15,rsi
            r15 = rsi;
            // 65580 mov r14,rdi
            r14 = rdi;
            // 65583 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 65587 mov rdx,r15
            rdx = r15;
            // 6558a call 0xebba6
            { let r = { let t = m.env.stub_va(0x373); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6558f mov r12,rax
            r12 = rax;
            // 65592 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 65596 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 65599 je 0x6559f
            if fl.e() { pc = 0x6559f; continue; }
            pc = 0x6559b;
        }
        0x6559b => {
            // block 6559b
            #[cfg(feature = "trace")] eprintln!("f_0006556e pc 6559b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6559b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6559d jmp 0x655c6
            pc = 0x655c6; continue;
        }
        0x6559f => {
            // block 6559f
            #[cfg(feature = "trace")] eprintln!("f_0006556e pc 6559f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6559f mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 655a4 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 655a9 mov rbx,rax
            rbx = rax;
            // 655ac mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 655af mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 655b2 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 655b6 mov rdi,r14
            rdi = r14;
            // 655b9 mov rdx,r12
            rdx = r12;
            // 655bc mov rcx,rbx
            rcx = rbx;
            // 655bf call 0xebbb2
            { let r = { let t = m.env.stub_va(0x37f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 655c4 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x655c6;
        }
        0x655c6 => {
            // block 655c6
            #[cfg(feature = "trace")] eprintln!("f_0006556e pc 655c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 655c6 mov rax,rbx
            rax = rbx;
            // 655c9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 655cd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 655ce pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 655d0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 655d2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 655d4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 655d5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d410 size=384
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d410<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2d410;
    loop {
        match pc {
        0x2d410 => {
            // block 2d410
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d410 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d411 mov rbp,rsp
            rbp = rsp;
            // 2d414 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2d416 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d418 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2d41a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2d41c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d41d sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 2d421 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 2d424 mov rbx,rdi
            rbx = rdi;
            // 2d427 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 2d42a cmp r14d,0xffffffff
            fl = sub::<32>((r14 as u32 as u64), 0xffffffffu64).1;
            // 2d42e je 0x2d546
            if fl.e() { pc = 0x2d546; continue; }
            pc = 0x2d434;
        }
        0x2d434 => {
            // block 2d434
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d434 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d434 mov r15,QWORD PTR [rbx+0x18]
            r15 = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2d438 mov r13,QWORD PTR [rbx+0x30]
            r13 = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 2d43c sub r15,QWORD PTR [rbx+0x10]
            { let (r, f) = sub::<64>(r15, m.r64(rbx.wrapping_add(0x10)) as u64);
              r15 = r;
              fl = f; }
            // 2d440 mov rax,QWORD PTR [rbx+0x38]
            rax = m.r64(rbx.wrapping_add(0x38)) as u64;
            // 2d444 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 2d447 je 0x2d45a
            if fl.e() { pc = 0x2d45a; continue; }
            pc = 0x2d449;
        }
        0x2d449 => {
            // block 2d449
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d449 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d449 lea rdx,[rbx+0x58]
            rdx = rbx.wrapping_add(0x58);
            // 2d44d mov rdi,QWORD PTR [rbx+0x58]
            rdi = m.r64(rbx.wrapping_add(0x58)) as u64;
            // 2d451 lea rcx,[rbx+0x60]
            rcx = rbx.wrapping_add(0x60);
            // 2d455 jmp 0x2d4f5
            pc = 0x2d4f5; continue;
        }
        0x2d45a => {
            // block 2d45a
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d45a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d45a mov r12d,0xffffffff
            r12 = (0xffffffffu64) as u32 as u64;
            // 2d460 test BYTE PTR [rbx+0x60],0x10
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x60)) as u64 & 0x10u64);
            // 2d464 je 0x2d546
            if fl.e() { pc = 0x2d546; continue; }
            pc = 0x2d46a;
        }
        0x2d46a => {
            // block 2d46a
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d46a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d46a mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 2d46e mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 2d472 mov rax,QWORD PTR [rbx+0x58]
            rax = m.r64(rbx.wrapping_add(0x58)) as u64;
            // 2d476 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 2d47a lea rdi,[rbx+0x40]
            rdi = rbx.wrapping_add(0x40);
            // 2d47e mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 2d482 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2d484 call 0xec020
            { let r = { let t = m.env.stub_va(0x7ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d489 mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // 2d48e mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 2d492 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // 2d495 je 0x2d4a1
            if fl.e() { pc = 0x2d4a1; continue; }
            pc = 0x2d497;
        }
        0x2d497 => {
            // block 2d497
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d497 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d497 mov rsi,QWORD PTR [rdi]
            rsi = m.r64(rdi) as u64;
            // 2d49a and rsi,0xfffffffffffffffe
            { let r = rsi & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 2d49e dec rsi
            { let (r, f) = dec::<64>(rsi, fl); rsi = r; fl = f; }
            pc = 0x2d4a1;
        }
        0x2d4a1 => {
            // block 2d4a1
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d4a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d4a1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2d4a3 call 0xec008
            { let r = { let t = m.env.stub_va(0x7d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d4a8 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2d4ac sub r13,rax
            { let (r, f) = sub::<64>(r13, rax);
              r13 = r;
              fl = f; }
            // 2d4af mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2d4b3 sub rdi,rax
            { let (r, f) = sub::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 2d4b6 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 2d4ba movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 2d4bd test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 2d4bf jne 0x2d4c9
            if fl.ne() { pc = 0x2d4c9; continue; }
            pc = 0x2d4c1;
        }
        0x2d4c1 => {
            // block 2d4c1
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d4c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d4c1 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 2d4c4 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 2d4c7 jmp 0x2d4d1
            pc = 0x2d4d1; continue;
        }
        0x2d4c9 => {
            // block 2d4c9
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d4c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d4c9 mov rax,QWORD PTR [rbx+0x48]
            rax = m.r64(rbx.wrapping_add(0x48)) as u64;
            // 2d4cd mov rsi,QWORD PTR [rbx+0x50]
            rsi = m.r64(rbx.wrapping_add(0x50)) as u64;
            pc = 0x2d4d1;
        }
        0x2d4d1 => {
            // block 2d4d1
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d4d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d4d1 lea rcx,[rbx+0x60]
            rcx = rbx.wrapping_add(0x60);
            // 2d4d5 lea rdx,[rbx+0x58]
            rdx = rbx.wrapping_add(0x58);
            // 2d4d9 add rax,rsi
            { let (r, f) = add::<64>(rax, rsi);
              rax = r;
              fl = f; }
            // 2d4dc mov QWORD PTR [rbx+0x28],rsi
            m.w64(rbx.wrapping_add(0x28), (rsi) as u64);
            // 2d4e0 mov QWORD PTR [rbx+0x38],rax
            m.w64(rbx.wrapping_add(0x38), (rax) as u64);
            // 2d4e4 movsxd r13,r13d
            r13 = sx::<32>((r13 as u32 as u64)) as u64;
            // 2d4e7 add r13,rsi
            { let (r, f) = add::<64>(r13, rsi);
              r13 = r;
              fl = f; }
            // 2d4ea mov QWORD PTR [rbx+0x30],r13
            m.w64(rbx.wrapping_add(0x30), (r13) as u64);
            // 2d4ee add rdi,rsi
            { let (r, f) = add::<64>(rdi, rsi);
              rdi = r;
              fl = f; }
            // 2d4f1 mov QWORD PTR [rbx+0x58],rdi
            m.w64(rbx.wrapping_add(0x58), (rdi) as u64);
            pc = 0x2d4f5;
        }
        0x2d4f5 => {
            // block 2d4f5
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d4f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d4f5 lea rsi,[r13+0x1]
            rsi = r13.wrapping_add(0x1);
            // 2d4f9 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // 2d4fd cmp rsi,rdi
            fl = sub::<64>(rsi, rdi).1;
            // 2d500 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2d504 cmovb rdi,rdx
            if fl.b() { rdi = rdx; }
            // 2d508 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 2d50b mov QWORD PTR [rdx],rdi
            m.w64(rdx, (rdi) as u64);
            // 2d50e test BYTE PTR [rcx],0x8
            fl = logic::<8>(m.r8(rcx) as u64 & 0x8u64);
            // 2d511 je 0x2d535
            if fl.e() { pc = 0x2d535; continue; }
            pc = 0x2d513;
        }
        0x2d513 => {
            // block 2d513
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d513 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d513 test BYTE PTR [rbx+0x40],0x1
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x40)) as u64 & 0x1u64);
            // 2d517 jne 0x2d522
            if fl.ne() { pc = 0x2d522; continue; }
            pc = 0x2d519;
        }
        0x2d519 => {
            // block 2d519
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d519 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d519 lea rcx,[rbx+0x40]
            rcx = rbx.wrapping_add(0x40);
            // 2d51d inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 2d520 jmp 0x2d526
            pc = 0x2d526; continue;
        }
        0x2d522 => {
            // block 2d522
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d522 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d522 mov rcx,QWORD PTR [rbx+0x50]
            rcx = m.r64(rbx.wrapping_add(0x50)) as u64;
            pc = 0x2d526;
        }
        0x2d526 => {
            // block 2d526
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d526 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d526 add r15,rcx
            { let (r, f) = add::<64>(r15, rcx);
              r15 = r;
              fl = f; }
            // 2d529 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // 2d52d mov QWORD PTR [rbx+0x18],r15
            m.w64(rbx.wrapping_add(0x18), (r15) as u64);
            // 2d531 mov QWORD PTR [rbx+0x20],rdi
            m.w64(rbx.wrapping_add(0x20), (rdi) as u64);
            pc = 0x2d535;
        }
        0x2d535 => {
            // block 2d535
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d535 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d535 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 2d538 je 0x2d558
            if fl.e() { pc = 0x2d558; continue; }
            pc = 0x2d53a;
        }
        0x2d53a => {
            // block 2d53a
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d53a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d53a mov QWORD PTR [rbx+0x30],rsi
            m.w64(rbx.wrapping_add(0x30), (rsi) as u64);
            // 2d53e mov BYTE PTR [r13+0x0],r14b
            m.w8(r13, ((r14 as u8 as u64)) as u8);
            // 2d542 movzx r12d,r14b
            r12 = ((r14 as u8 as u64)) as u32 as u64;
            pc = 0x2d546;
        }
        0x2d546 => {
            // block 2d546
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d546 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d546 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 2d549 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 2d54d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d54e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2d550 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2d552 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d554 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d556 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d557 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2d558 => {
            // block 2d558
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d558 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d558 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2d55b mov rax,QWORD PTR [rax+0x68]
            rax = m.r64(rax.wrapping_add(0x68)) as u64;
            // 2d55f movzx esi,r14b
            rsi = ((r14 as u8 as u64)) as u32 as u64;
            // 2d563 mov rdi,rbx
            rdi = rbx;
            // 2d566 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 2d56a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d56b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2d56d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2d56f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d571 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d573 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d574 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0x2d576 => {
            // block 2d576
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d576 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d576 mov rdi,rax
            rdi = rax;
            // 2d579 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d57e call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d583 jmp 0x2d546
            pc = 0x2d546; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ed46 size=292
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0002ed46<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2ed46;
    loop {
        match pc {
        0x2ed46 => {
            // block 2ed46
            #[cfg(feature = "trace")] eprintln!("f_0002ed46 pc 2ed46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ed46 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ed47 mov rbp,rsp
            rbp = rsp;
            // 2ed4a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2ed4c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2ed4e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2ed50 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2ed52 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2ed53 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 2ed57 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 2ed5b mov r12,rdi
            r12 = rdi;
            // 2ed5e mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 2ed62 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2ed67 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 2ed6a sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // 2ed6e inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2ed71 mov rax,rbx
            rax = rbx;
            // 2ed74 shr rax,0x3d
            { let (r, f) = shr::<64>(rax, 0x3du64, fl); rax = r; fl = f; }
            // 2ed78 je 0x2ed86
            if fl.e() { pc = 0x2ed86; continue; }
            pc = 0x2ed7a;
        }
        0x2ed7a => {
            // block 2ed7a
            #[cfg(feature = "trace")] eprintln!("f_0002ed46 pc 2ed7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ed7a mov rdi,r12
            rdi = r12;
            // 2ed7d call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x2ed86 => {
            // block 2ed86
            #[cfg(feature = "trace")] eprintln!("f_0002ed46 pc 2ed86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ed86 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 2ed8b mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2ed90 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 2ed94 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 2ed97 mov rcx,r15
            rcx = r15;
            // 2ed9a sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 2ed9e movabs rdx,0xffffffffffffffe
            rdx = 0xfff_ffff_ffff_fffeu64;
            // 2eda8 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 2edab ja 0x2edba
            if fl.a() { pc = 0x2edba; continue; }
            pc = 0x2edad;
        }
        0x2edad => {
            // block 2edad
            #[cfg(feature = "trace")] eprintln!("f_0002ed46 pc 2edad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2edad sar r15,0x2
            { let (r, f) = sar::<64>(r15, 0x2u64, fl); r15 = r; fl = f; }
            // 2edb1 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 2edb4 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 2edb8 jmp 0x2edc4
            pc = 0x2edc4; continue;
        }
        0x2edba => {
            // block 2edba
            #[cfg(feature = "trace")] eprintln!("f_0002ed46 pc 2edba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2edba movabs r15,0x1fffffffffffffff
            r15 = 0x1fff_ffff_ffff_ffffu64;
            pc = 0x2edc4;
        }
        0x2edc4 => {
            // block 2edc4
            #[cfg(feature = "trace")] eprintln!("f_0002ed46 pc 2edc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2edc4 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2edc9 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 2edcd sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 2edd0 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 2edd4 mov r13,r14
            r13 = r14;
            // 2edd7 sar r13,0x3
            { let (r, f) = sar::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // 2eddb mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 2ede3 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 2ede7 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 2ede9 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2edec je 0x2edfe
            if fl.e() { pc = 0x2edfe; continue; }
            pc = 0x2edee;
        }
        0x2edee => {
            // block 2edee
            #[cfg(feature = "trace")] eprintln!("f_0002ed46 pc 2edee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2edee lea rdi,[r15*8+0x0]
            rdi = r15.wrapping_mul(8);
            // 2edf6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2edfb mov rbx,rax
            rbx = rax;
            pc = 0x2edfe;
        }
        0x2edfe => {
            // block 2edfe
            #[cfg(feature = "trace")] eprintln!("f_0002ed46 pc 2edfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2edfe lea rax,[rbx+r15*8]
            rax = rbx.wrapping_add(r15.wrapping_mul(8));
            // 2ee02 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 2ee06 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 2ee0a mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 2ee0d mov QWORD PTR [rbx+r13*8],rax
            m.w64(rbx.wrapping_add(r13.wrapping_mul(8)), (rax) as u64);
            // 2ee11 lea r13,[rbx+r13*8+0x8]
            r13 = rbx.wrapping_add(r13.wrapping_mul(8)).wrapping_add(0x8);
            // 2ee16 mov rdi,rbx
            rdi = rbx;
            // 2ee19 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 2ee1d mov rsi,r15
            rsi = r15;
            // 2ee20 mov rdx,r14
            rdx = r14;
            // 2ee23 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ee28 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 2ee2c mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 2ee30 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 2ee35 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 2ee39 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 2ee3d mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 2ee41 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 2ee46 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 2ee4a mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 2ee4e mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 2ee52 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2ee56 call 0xebafe
            { let r = { let t = m.env.stub_va(0x2cb); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ee5b add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 2ee5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2ee60 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2ee62 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2ee64 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2ee66 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2ee68 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ee69 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6250 size=284
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006250<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6250;
    loop {
        match pc {
        0x6250 => {
            // block 6250
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 6250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6250 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6251 mov rbp,rsp
            rbp = rsp;
            // 6254 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6256 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6258 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6259 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 625a mov r14,rdx
            r14 = rdx;
            // 625d mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 625f mov r15,rdi
            r15 = rdi;
            // 6262 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6265 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 626b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 626d jne 0x6276
            if fl.ne() { pc = 0x6276; continue; }
            pc = 0x626f;
        }
        0x626f => {
            // block 626f
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 626f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 626f mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 6271 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 6274 jbe 0x627d
            if fl.be() { pc = 0x627d; continue; }
            pc = 0x6276;
        }
        0x6276 => {
            // block 6276
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 6276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6276 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6278 jmp 0x633f
            pc = 0x633f; continue;
        }
        0x627d => {
            // block 627d
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 627d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 627d lea rcx,[rip+0xc8] # 0x634c
            rcx = m.env.code_va(0x634c);
            // 6284 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 6288 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 628b jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x628d => {
            // block 628d
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 628d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 628d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6290 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 6295 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 629a mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 62a0 jmp 0x6336
            pc = 0x6336; continue;
        }
        0x62a5 => {
            // block 62a5
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 62a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62a5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 62a8 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 62ad mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // 62b2 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 62b8 jmp 0x6336
            pc = 0x6336; continue;
        }
        0x62ba => {
            // block 62ba
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 62ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62ba mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 62bd mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 62c2 mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 62c7 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 62cd jmp 0x6336
            pc = 0x6336; continue;
        }
        0x62cf => {
            // block 62cf
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 62cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62cf mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 62d2 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 62d7 mov ecx,0x80000
            rcx = (0x80000u64) as u32 as u64;
            // 62dc mov r8d,0x13
            r8 = (0x13u64) as u32 as u64;
            // 62e2 jmp 0x6336
            pc = 0x6336; continue;
        }
        0x62e4 => {
            // block 62e4
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 62e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62e4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 62e7 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 62ec mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 62f1 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 62f7 jmp 0x6336
            pc = 0x6336; continue;
        }
        0x62f9 => {
            // block 62f9
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 62f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 62f9 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 62fc mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 6301 mov ecx,0x200000
            rcx = (0x200000u64) as u32 as u64;
            // 6306 mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            // 630c jmp 0x6336
            pc = 0x6336; continue;
        }
        0x630e => {
            // block 630e
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 630e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 630e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6311 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 6316 mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 631b mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 6321 jmp 0x6336
            pc = 0x6336; continue;
        }
        0x6323 => {
            // block 6323
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 6323 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6323 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6326 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 632b mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 6330 mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            pc = 0x6336;
        }
        0x6336 => {
            // block 6336
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 6336 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6336 mov rdi,r15
            rdi = r15;
            // 6339 mov rdx,r14
            rdx = r14;
            // 633c call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x633f;
        }
        0x633f => {
            // block 633f
            #[cfg(feature = "trace")] eprintln!("f_00006250 pc 633f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 633f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6343 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6344 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6346 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6348 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6349 ret
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
            #[cfg(feature = "trace")] eprintln!("f_00002f10 pc 2f10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x25d62 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025d62<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x25d62;
    loop {
        match pc {
        0x25d62 => {
            // block 25d62
            #[cfg(feature = "trace")] eprintln!("f_00025d62 pc 25d62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d62 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25d63 mov rbp,rsp
            rbp = rsp;
            // 25d66 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 25d68 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 25d69 mov rbx,rsi
            rbx = rsi;
            // 25d6c mov r14,rdi
            r14 = rdi;
            // 25d6f test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 25d72 je 0x25d97
            if fl.e() { pc = 0x25d97; continue; }
            pc = 0x25d74;
        }
        0x25d74 => {
            // block 25d74
            #[cfg(feature = "trace")] eprintln!("f_00025d62 pc 25d74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d74 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 25d77 mov rdi,r14
            rdi = r14;
            // 25d7a call 0x25d62
            { let r = { let t = m.env.code_va(0x25d62); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25d7f mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 25d83 mov rdi,r14
            rdi = r14;
            // 25d86 call 0x25d62
            { let r = { let t = m.env.code_va(0x25d62); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25d8b mov rdi,rbx
            rdi = rbx;
            // 25d8e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25d8f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25d91 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25d92 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x25d97 => {
            // block 25d97
            #[cfg(feature = "trace")] eprintln!("f_00025d62 pc 25d97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d97 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25d98 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25d9a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25d9b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x645eb size=351
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000645eb<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut x1: u128 = fx[1] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x645eb;
    loop {
        match pc {
        0x645eb => {
            // block 645eb
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 645eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 645eb push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 645ec mov rbp,rsp
            rbp = rsp;
            // 645ef push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 645f0 movzx ebx,WORD PTR [rdi]
            rbx = (m.r16(rdi) as u64) as u32 as u64;
            // 645f3 cmp ebx,0xc350
            fl = sub::<32>((rbx as u32 as u64), 0xc350u64).1;
            // 645f9 ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x645fb;
        }
        0x645fb => {
            // block 645fb
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 645fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 645fb movzx eax,WORD PTR [rdi+0x2]
            rax = (m.r16(rdi.wrapping_add(0x2)) as u64) as u32 as u64;
            // 645ff cmp eax,0xc350
            fl = sub::<32>((rax as u32 as u64), 0xc350u64).1;
            // 64604 ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x64606;
        }
        0x64606 => {
            // block 64606
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64606 movzx edx,WORD PTR [rdi+0x4]
            rdx = (m.r16(rdi.wrapping_add(0x4)) as u64) as u32 as u64;
            // 6460a cmp edx,0xc350
            fl = sub::<32>((rdx as u32 as u64), 0xc350u64).1;
            // 64610 ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x64612;
        }
        0x64612 => {
            // block 64612
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64612 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64612 movzx ecx,WORD PTR [rdi+0x6]
            rcx = (m.r16(rdi.wrapping_add(0x6)) as u64) as u32 as u64;
            // 64616 cmp ecx,0xc350
            fl = sub::<32>((rcx as u32 as u64), 0xc350u64).1;
            // 6461c ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x6461e;
        }
        0x6461e => {
            // block 6461e
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6461e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6461e movzx r11d,WORD PTR [rdi+0x8]
            r11 = (m.r16(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 64623 cmp r11d,0xc350
            fl = sub::<32>((r11 as u32 as u64), 0xc350u64).1;
            // 6462a ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x6462c;
        }
        0x6462c => {
            // block 6462c
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6462c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6462c movzx r10d,WORD PTR [rdi+0xa]
            r10 = (m.r16(rdi.wrapping_add(0xa)) as u64) as u32 as u64;
            // 64631 cmp r10d,0xc350
            fl = sub::<32>((r10 as u32 as u64), 0xc350u64).1;
            // 64638 ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x6463a;
        }
        0x6463a => {
            // block 6463a
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6463a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6463a movzx r9d,WORD PTR [rdi+0xc]
            r9 = (m.r16(rdi.wrapping_add(0xc)) as u64) as u32 as u64;
            // 6463f cmp r9d,0xc350
            fl = sub::<32>((r9 as u32 as u64), 0xc350u64).1;
            // 64646 ja 0x64656
            if fl.a() { pc = 0x64656; continue; }
            pc = 0x64648;
        }
        0x64648 => {
            // block 64648
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64648 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64648 movzx r8d,WORD PTR [rdi+0xe]
            r8 = (m.r16(rdi.wrapping_add(0xe)) as u64) as u32 as u64;
            // 6464d cmp r8d,0xc350
            fl = sub::<32>((r8 as u32 as u64), 0xc350u64).1;
            // 64654 jbe 0x6465b
            if fl.be() { pc = 0x6465b; continue; }
            pc = 0x64656;
        }
        0x64656 => {
            // block 64656
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64656 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64656 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x64658;
        }
        0x64658 => {
            // block 64658
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64658 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64658 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64659 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6465a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6465b => {
            // block 6465b
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6465b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6465b cvtsi2sd xmm1,ebx
            x1 = set_lo64(x1, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // 6465f movsd xmm0,QWORD PTR [rip+0x9c139] # 0x1007a0
            x0 = m.r64(m.env.data_va(85869)) as u128;
            // 64667 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 6466b cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 6466f movss DWORD PTR [rsi],xmm1
            m.w32(rsi, x1 as u32);
            // 64673 xorps xmm1,xmm1
            x1 = 0;
            // 64676 cvtsi2sd xmm1,eax
            x1 = set_lo64(x1, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 6467a mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 6467e cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 64682 movss DWORD PTR [rsi+0x4],xmm1
            m.w32(rsi.wrapping_add(0x4), x1 as u32);
            // 64687 xorps xmm1,xmm1
            x1 = 0;
            // 6468a cvtsi2sd xmm1,edx
            x1 = set_lo64(x1, (sx::<32>((rdx as u32 as u64)) as f64).to_bits());
            // 6468e mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 64692 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 64696 movss DWORD PTR [rsi+0x8],xmm1
            m.w32(rsi.wrapping_add(0x8), x1 as u32);
            // 6469b xorps xmm1,xmm1
            x1 = 0;
            // 6469e cvtsi2sd xmm1,ecx
            x1 = set_lo64(x1, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // 646a2 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646a6 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646aa movss DWORD PTR [rsi+0xc],xmm1
            m.w32(rsi.wrapping_add(0xc), x1 as u32);
            // 646af xorps xmm1,xmm1
            x1 = 0;
            // 646b2 cvtsi2sd xmm1,r11d
            x1 = set_lo64(x1, (sx::<32>((r11 as u32 as u64)) as f64).to_bits());
            // 646b7 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646bb cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646bf movss DWORD PTR [rsi+0x10],xmm1
            m.w32(rsi.wrapping_add(0x10), x1 as u32);
            // 646c4 xorps xmm1,xmm1
            x1 = 0;
            // 646c7 cvtsi2sd xmm1,r10d
            x1 = set_lo64(x1, (sx::<32>((r10 as u32 as u64)) as f64).to_bits());
            // 646cc mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646d0 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646d4 movss DWORD PTR [rsi+0x14],xmm1
            m.w32(rsi.wrapping_add(0x14), x1 as u32);
            // 646d9 xorps xmm1,xmm1
            x1 = 0;
            // 646dc cvtsi2sd xmm1,r9d
            x1 = set_lo64(x1, (sx::<32>((r9 as u32 as u64)) as f64).to_bits());
            // 646e1 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646e5 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646e9 movss DWORD PTR [rsi+0x18],xmm1
            m.w32(rsi.wrapping_add(0x18), x1 as u32);
            // 646ee xorps xmm1,xmm1
            x1 = 0;
            // 646f1 cvtsi2sd xmm1,r8d
            x1 = set_lo64(x1, (sx::<32>((r8 as u32 as u64)) as f64).to_bits());
            // 646f6 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646fa cvtsd2ss xmm0,xmm1
            x0 = set_lo32(x0, (f64_of(x1) as f32).to_bits());
            // 646fe movss DWORD PTR [rsi+0x1c],xmm0
            m.w32(rsi.wrapping_add(0x1c), x0 as u32);
            // 64703 movzx eax,WORD PTR [rdi+0x12]
            rax = (m.r16(rdi.wrapping_add(0x12)) as u64) as u32 as u64;
            // 64707 xorps xmm0,xmm0
            x0 = 0;
            // 6470a cvtsi2sd xmm0,eax
            x0 = set_lo64(x0, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 6470e mulsd xmm0,QWORD PTR [rip+0x9c092] # 0x1007a8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85877)) as u128))).to_bits());
            // 64716 cvtsd2ss xmm0,xmm0
            x0 = set_lo32(x0, (f64_of(x0) as f32).to_bits());
            // 6471a movss DWORD PTR [rsi+0x24],xmm0
            m.w32(rsi.wrapping_add(0x24), x0 as u32);
            // 6471f mov ax,WORD PTR [rdi+0x10]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x10)) as u64) as u16 as u64);
            // 64723 mov WORD PTR [rsi+0x20],ax
            m.w16(rsi.wrapping_add(0x20), ((rax as u16 as u64)) as u16);
            // 64727 mov ax,WORD PTR [rdi+0x14]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x14)) as u64) as u16 as u64);
            // 6472b mov WORD PTR [rsi+0x28],ax
            m.w16(rsi.wrapping_add(0x28), ((rax as u16 as u64)) as u16);
            // 6472f mov ax,WORD PTR [rdi+0x16]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x16)) as u64) as u16 as u64);
            // 64733 mov WORD PTR [rsi+0x2a],ax
            m.w16(rsi.wrapping_add(0x2a), ((rax as u16 as u64)) as u16);
            // 64737 mov al,BYTE PTR [rdi+0x18]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x18)) as u64) as u8 as u64);
            // 6473a mov BYTE PTR [rsi+0x2c],al
            m.w8(rsi.wrapping_add(0x2c), ((rax as u8 as u64)) as u8);
            // 6473d mov al,BYTE PTR [rdi+0x19]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x19)) as u64) as u8 as u64);
            // 64740 mov BYTE PTR [rsi+0x2d],al
            m.w8(rsi.wrapping_add(0x2d), ((rax as u8 as u64)) as u8);
            // 64743 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 64745 jmp 0x64658
            pc = 0x64658; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5142 size=906
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00005142<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5142;
    loop {
        match pc {
        0x5142 => {
            // block 5142
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5142 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5142 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5143 mov rbp,rsp
            rbp = rsp;
            // 5146 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5148 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 514a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 514c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 514e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 514f sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 5153 mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // 5156 mov DWORD PTR [rbp-0x44],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((r14 as u32 as u64)) as u32);
            // 515a mov DWORD PTR [rbp-0x48],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((rcx as u32 as u64)) as u32);
            // 515d mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 5160 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5163 mov r13,rdi
            r13 = rdi;
            // 5166 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 516a call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5170 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 5172 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5174 mov esi,0x0
            rsi = (0x0u64) as u32 as u64;
            // 5179 cmovne esi,r14d
            if fl.ne() { rsi = ((r14 as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // 517d mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 5181 mov rdi,r13
            rdi = r13;
            // 5184 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 518a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 518c jne 0x519f
            if fl.ne() { pc = 0x519f; continue; }
            pc = 0x518e;
        }
        0x518e => {
            // block 518e
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 518e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 518e cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 5192 ja 0x519d
            if fl.a() { pc = 0x519d; continue; }
            pc = 0x5194;
        }
        0x5194 => {
            // block 5194
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5194 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 5197 cmp r12d,0xb
            fl = sub::<32>((r12 as u32 as u64), 0xbu64).1;
            // 519b jbe 0x51b0
            if fl.be() { pc = 0x51b0; continue; }
            pc = 0x519d;
        }
        0x519d => {
            // block 519d
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 519d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 519d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x519f;
        }
        0x519f => {
            // block 519f
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 519f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 519f mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 51a1 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 51a5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 51a6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 51a8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 51aa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 51ac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 51ae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51af ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x51b0 => {
            // block 51b0
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 51b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51b0 lea rcx,[rip+0x2e5] # 0x549c
            rcx = m.env.code_va(0x549c);
            // 51b7 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 51bb add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 51be jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x51c0 => {
            // block 51c0
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 51c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51c0 lea eax,[r15-0x1]
            rax = (r15.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 51c4 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 51c7 ja 0x523a
            if fl.a() { pc = 0x523a; continue; }
            pc = 0x51c9;
        }
        0x51c9 => {
            // block 51c9
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 51c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51c9 mov rdi,QWORD PTR [rip+0x13afb0] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 51d0 lea rsi,[rip+0x124d47] # 0x129f1e
            rsi = m.env.data_va(255723);
            // 51d7 mov edx,0x84
            rdx = (0x84u64) as u32 as u64;
            // 51dc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51e1 mov rbx,rax
            rbx = rax;
            // 51e4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 51e7 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 51eb add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 51ee lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 51f2 mov rdi,r12
            rdi = r12;
            // 51f5 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51fa mov rsi,QWORD PTR [rip+0x13af8f] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 5201 mov rdi,r12
            rdi = r12;
            // 5204 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5209 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 520c mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 5210 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 5215 mov rdi,rax
            rdi = rax;
            // 5218 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 521a mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 521d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 5221 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5226 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 522a mov rdi,rbx
            rdi = rbx;
            // 522d call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5232 mov rdi,rbx
            rdi = rbx;
            // 5235 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x523a;
        }
        0x523a => {
            // block 523a
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 523a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 523a mov esi,DWORD PTR [rbp-0x48]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 523d jmp 0x5427
            pc = 0x5427; continue;
        }
        0x5242 => {
            // block 5242
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5242 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5242 mov esi,DWORD PTR [rbp-0x48]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 5245 cmp esi,0x8
            fl = sub::<32>((rsi as u32 as u64), 0x8u64).1;
            // 5248 ja 0x5275
            if fl.a() { pc = 0x5275; continue; }
            pc = 0x524a;
        }
        0x524a => {
            // block 524a
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 524a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 524a mov eax,0x111
            rax = (0x111u64) as u32 as u64;
            // 524f bt eax,esi
            fl.cf = bit::<32>((rax as u32 as u64), (rsi as u32 as u64));
            // 5252 jae 0x5275
            if fl.ae() { pc = 0x5275; continue; }
            pc = 0x5254;
        }
        0x5254 => {
            // block 5254
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5254 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5254 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 5259 cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 525d je 0x5427
            if fl.e() { pc = 0x5427; continue; }
            pc = 0x5263;
        }
        0x5263 => {
            // block 5263
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5263 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5263 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 5267 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 526a movzx esi,al
            rsi = ((rax as u8 as u64)) as u32 as u64;
            // 526d shl esi,0x3
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x3u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 5270 jmp 0x5427
            pc = 0x5427; continue;
        }
        0x5275 => {
            // block 5275
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5275 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5275 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 5277 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 527a cmp eax,0x10
            fl = sub::<32>((rax as u32 as u64), 0x10u64).1;
            // 527d je 0x5427
            if fl.e() { pc = 0x5427; continue; }
            pc = 0x5283;
        }
        0x5283 => {
            // block 5283
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5283 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5283 cmp esi,0xd
            fl = sub::<32>((rsi as u32 as u64), 0xdu64).1;
            // 5286 ja 0x5427
            if fl.a() { pc = 0x5427; continue; }
            pc = 0x528c;
        }
        0x528c => {
            // block 528c
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 528c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 528c mov eax,0x2420
            rax = (0x2420u64) as u32 as u64;
            // 5291 bt eax,esi
            fl.cf = bit::<32>((rax as u32 as u64), (rsi as u32 as u64));
            // 5294 jae 0x5427
            if fl.ae() { pc = 0x5427; continue; }
            pc = 0x529a;
        }
        0x529a => {
            // block 529a
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 529a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 529a mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 529f cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 52a3 je 0x5427
            if fl.e() { pc = 0x5427; continue; }
            pc = 0x52a9;
        }
        0x52a9 => {
            // block 52a9
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 52a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52a9 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 52ad sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 52b0 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 52b3 lea esi,[rax+rax*2+0xa]
            rsi = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xa)) as u32 as u64;
            // 52b7 jmp 0x5427
            pc = 0x5427; continue;
        }
        0x52bc => {
            // block 52bc
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 52bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52bc mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 52c1 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 52c4 jle 0x5427
            if fl.le() { pc = 0x5427; continue; }
            pc = 0x52ca;
        }
        0x52ca => {
            // block 52ca
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 52ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52ca cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 52ce setl al
            rax = (rax & !0xff) | (((fl.l() as u64)) as u8 as u64);
            // 52d1 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 52d4 lea esi,[rax*8+0x1]
            rsi = (rax.wrapping_mul(8).wrapping_add(0x1)) as u32 as u64;
            // 52db cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 52df jne 0x5427
            if fl.ne() { pc = 0x5427; continue; }
            pc = 0x52e5;
        }
        0x52e5 => {
            // block 52e5
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 52e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52e5 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 52e8 mov rdi,QWORD PTR [rip+0x13ae91] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 52ef lea rsi,[rip+0x124b34] # 0x129e2a
            rsi = m.env.data_va(255479);
            // 52f6 mov edx,0x7a
            rdx = (0x7au64) as u32 as u64;
            // 52fb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5300 mov rbx,rax
            rbx = rax;
            // 5303 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5306 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 530a add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 530d lea r12,[rbp-0x40]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 5311 mov rdi,r12
            rdi = r12;
            // 5314 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5319 mov rsi,QWORD PTR [rip+0x13ae70] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 5320 mov rdi,r12
            rdi = r12;
            // 5323 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5328 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 532b mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 532f mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 5334 mov rdi,rax
            rdi = rax;
            // 5337 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5339 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 533c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 5340 jmp 0x540b
            pc = 0x540b; continue;
        }
        0x5345 => {
            // block 5345
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5345 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5345 mov esi,0xe
            rsi = (0xeu64) as u32 as u64;
            // 534a cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 534e je 0x5427
            if fl.e() { pc = 0x5427; continue; }
            pc = 0x5354;
        }
        0x5354 => {
            // block 5354
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5354 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5354 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 5358 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 535b movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 535e lea esi,[rax*4+0x2]
            rsi = (rax.wrapping_mul(4).wrapping_add(0x2)) as u32 as u64;
            // 5365 jmp 0x5427
            pc = 0x5427; continue;
        }
        0x536a => {
            // block 536a
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 536a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 536a mov esi,0xf
            rsi = (0xfu64) as u32 as u64;
            // 536f cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 5373 je 0x5427
            if fl.e() { pc = 0x5427; continue; }
            pc = 0x5379;
        }
        0x5379 => {
            // block 5379
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5379 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 537d sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5380 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 5383 lea esi,[rax*4+0x3]
            rsi = (rax.wrapping_mul(4).wrapping_add(0x3)) as u32 as u64;
            // 538a jmp 0x5427
            pc = 0x5427; continue;
        }
        0x538f => {
            // block 538f
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 538f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 538f mov esi,0xb
            rsi = (0xbu64) as u32 as u64;
            // 5394 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 5397 jle 0x5427
            if fl.le() { pc = 0x5427; continue; }
            pc = 0x539d;
        }
        0x539d => {
            // block 539d
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 539d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 539d cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 53a1 setl al
            rax = (rax & !0xff) | (((fl.l() as u64)) as u8 as u64);
            // 53a4 movzx esi,al
            rsi = ((rax as u8 as u64)) as u32 as u64;
            // 53a7 add esi,0xb
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xbu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 53aa cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 53ae jne 0x5427
            if fl.ne() { pc = 0x5427; continue; }
            pc = 0x53b0;
        }
        0x53b0 => {
            // block 53b0
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 53b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 53b0 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 53b3 mov rdi,QWORD PTR [rip+0x13adc6] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 53ba lea rsi,[rip+0x124ae4] # 0x129ea5
            rsi = m.env.data_va(255602);
            // 53c1 mov edx,0x78
            rdx = (0x78u64) as u32 as u64;
            // 53c6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 53cb mov rbx,rax
            rbx = rax;
            // 53ce mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 53d1 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 53d5 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 53d8 lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 53dc mov rdi,r12
            rdi = r12;
            // 53df call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 53e4 mov rsi,QWORD PTR [rip+0x13ada5] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 53eb mov rdi,r12
            rdi = r12;
            // 53ee call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 53f3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 53f6 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 53fa mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 53ff mov rdi,rax
            rdi = rax;
            // 5402 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5404 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 5407 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0x540b;
        }
        0x540b => {
            // block 540b
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 540b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 540b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5410 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 5414 mov rdi,rbx
            rdi = rbx;
            // 5417 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 541c mov rdi,rbx
            rdi = rbx;
            // 541f call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5424 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            pc = 0x5427;
        }
        0x5427 => {
            // block 5427
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5427 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5427 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 542b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 542d mov rdi,r13
            rdi = r13;
            // 5430 mov ecx,DWORD PTR [rbp-0x44]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64) as u32 as u64;
            // 5433 call QWORD PTR [rax+0x310]
            { let r = { let t = m.r64(rax.wrapping_add(0x310)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5439 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 543d mov rdi,r13
            rdi = r13;
            // 5440 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5446 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5448 call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 544d mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // 544f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5451 jne 0x519f
            if fl.ne() { pc = 0x519f; continue; }
            pc = 0x5457;
        }
        0x5457 => {
            // block 5457
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5457 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5457 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 545b dec r15d
            { let (r, f) = dec::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 545e cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 5462 sbb esi,esi
            { let (r, f) = sbb::<32>((rsi as u32 as u64), (rsi as u32 as u64), fl.cf);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5464 and esi,0x1
            { let r = (rsi as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5467 mov rdi,r13
            rdi = r13;
            // 546a call QWORD PTR [rax+0x5a0]
            { let r = { let t = m.r64(rax.wrapping_add(0x5a0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5470 jmp 0x519f
            pc = 0x519f; continue;
        }
        0x5475 => {
            // block 5475
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5475 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5475 mov rbx,rax
            rbx = rax;
            // 5478 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0x547c;
        }
        0x547c => {
            // block 547c
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 547c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 547c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5481 mov rdi,rbx
            rdi = rbx;
            // 5484 call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
        0x5492 => {
            // block 5492
            #[cfg(feature = "trace")] eprintln!("f_00005142 pc 5492 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5492 mov rbx,rax
            rbx = rax;
            // 5495 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 5499 jmp 0x547c
            pc = 0x547c; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcf3c size=372
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cf3c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xcf3c;
    loop {
        match pc {
        0xcf3c => {
            // block cf3c
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf3c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cf3d mov rbp,rsp
            rbp = rsp;
            // cf40 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // cf42 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // cf43 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // cf47 mov r14,rsi
            r14 = rsi;
            // cf4a mov rbx,rdi
            rbx = rdi;
            // cf4d mov DWORD PTR [rbp-0x18],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0xcu64) as u32);
            // cf54 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // cf57 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // cf5b call QWORD PTR [rax+0x14c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x14c8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // cf61 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // cf63 je 0xcf75
            if fl.e() { pc = 0xcf75; continue; }
            pc = 0xcf65;
        }
        0xcf65 => {
            // block cf65
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf65 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // cf68 lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // cf6c mov rdi,rbx
            rdi = rbx;
            // cf6f call QWORD PTR [rax+0x14b8]
            { let r = { let t = m.r64(rax.wrapping_add(0x14b8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xcf75;
        }
        0xcf75 => {
            // block cf75
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf75 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // cf77 je 0xd033
            if fl.e() { pc = 0xd033; continue; }
            pc = 0xcf7d;
        }
        0xcf7d => {
            // block cf7d
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf7d mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // cf80 cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // cf84 ja 0xd033
            if fl.a() { pc = 0xd033; continue; }
            pc = 0xcf8a;
        }
        0xcf8a => {
            // block cf8a
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf8a lea rdx,[rip+0xdf] # 0xd070
            rdx = m.env.code_va(0xd070);
            // cf91 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // cf95 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // cf98 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xcf9a => {
            // block cf9a
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cf9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cf9a mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cf9d cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // cfa0 jne 0xd000
            if fl.ne() { pc = 0xd000; continue; }
            pc = 0xcfa2;
        }
        0xcfa2 => {
            // block cfa2
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfa2 mov DWORD PTR [r14],0xc
            m.w32(r14, (0xcu64) as u32);
            // cfa9 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xcfae => {
            // block cfae
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfae mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cfb1 cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // cfb4 jne 0xd00e
            if fl.ne() { pc = 0xd00e; continue; }
            pc = 0xcfb6;
        }
        0xcfb6 => {
            // block cfb6
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfb6 mov DWORD PTR [r14],0xe
            m.w32(r14, (0xeu64) as u32);
            // cfbd jmp 0xd062
            pc = 0xd062; continue;
        }
        0xcfc2 => {
            // block cfc2
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfc2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // cfc4 mov edx,DWORD PTR [rbp-0x14]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cfc7 dec edx
            { let (r, f) = dec::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // cfc9 cmp edx,0xb
            fl = sub::<32>((rdx as u32 as u64), 0xbu64).1;
            // cfcc ja 0xd064
            if fl.a() { pc = 0xd064; continue; }
            pc = 0xcfd2;
        }
        0xcfd2 => {
            // block cfd2
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfd2 lea rsi,[rip+0xa7] # 0xd080
            rsi = m.env.code_va(0xd080);
            // cfd9 movsxd rdx,DWORD PTR [rsi+rdx*4]
            rdx = sx::<32>(m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // cfdd add rdx,rsi
            { let (r, f) = add::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // cfe0 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xcfe2 => {
            // block cfe2
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfe2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfe2 mov DWORD PTR [r14],0x4
            m.w32(r14, (0x4u64) as u32);
            // cfe9 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xcfeb => {
            // block cfeb
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cfeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cfeb mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cfee test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // cff0 je 0xd01c
            if fl.e() { pc = 0xd01c; continue; }
            pc = 0xcff2;
        }
        0xcff2 => {
            // block cff2
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cff2 cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // cff5 jne 0xd025
            if fl.ne() { pc = 0xd025; continue; }
            pc = 0xcff7;
        }
        0xcff7 => {
            // block cff7
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc cff7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cff7 mov DWORD PTR [r14],0x7
            m.w32(r14, (0x7u64) as u32);
            // cffe jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd000 => {
            // block d000
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d000 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d000 cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // d003 jne 0xd033
            if fl.ne() { pc = 0xd033; continue; }
            pc = 0xd005;
        }
        0xd005 => {
            // block d005
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d005 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d005 mov DWORD PTR [r14],0x9
            m.w32(r14, (0x9u64) as u32);
            // d00c jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd00e => {
            // block d00e
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d00e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d00e cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // d011 jne 0xd033
            if fl.ne() { pc = 0xd033; continue; }
            pc = 0xd013;
        }
        0xd013 => {
            // block d013
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d013 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d013 mov DWORD PTR [r14],0xb
            m.w32(r14, (0xbu64) as u32);
            // d01a jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd01c => {
            // block d01c
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d01c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d01c mov DWORD PTR [r14],0x8
            m.w32(r14, (0x8u64) as u32);
            // d023 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd025 => {
            // block d025
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d025 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d025 cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // d028 jne 0xd033
            if fl.ne() { pc = 0xd033; continue; }
            pc = 0xd02a;
        }
        0xd02a => {
            // block d02a
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d02a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d02a mov DWORD PTR [r14],0x6
            m.w32(r14, (0x6u64) as u32);
            // d031 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd033 => {
            // block d033
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d033 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d033 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d035 jmp 0xd064
            pc = 0xd064; continue;
        }
        0xd037 => {
            // block d037
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d037 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d037 mov DWORD PTR [r14],0x5
            m.w32(r14, (0x5u64) as u32);
            // d03e jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd040 => {
            // block d040
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d040 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d040 mov DWORD PTR [r14],0x1
            m.w32(r14, (0x1u64) as u32);
            // d047 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd049 => {
            // block d049
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d049 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d049 mov DWORD PTR [r14],0x2
            m.w32(r14, (0x2u64) as u32);
            // d050 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd052 => {
            // block d052
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d052 mov DWORD PTR [r14],0x3
            m.w32(r14, (0x3u64) as u32);
            // d059 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd05b => {
            // block d05b
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d05b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d05b mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            pc = 0xd062;
        }
        0xd062 => {
            // block d062
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d062 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d062 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xd064;
        }
        0xd064 => {
            // block d064
            #[cfg(feature = "trace")] eprintln!("f_0000cf3c pc d064 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d064 mov al,cl
            rax = (rax & !0xff) | (((rcx as u8 as u64)) as u8 as u64);
            // d066 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d06a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d06b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d06d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d06e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe09aa size=12
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e09aa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe09aa;
    loop {
        match pc {
        0xe09aa => {
            // block e09aa
            #[cfg(feature = "trace")] eprintln!("f_000e09aa pc e09aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e09aa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e09ab mov rbp,rsp
            rbp = rsp;
            // e09ae mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e09b1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e09b2 jmp QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2156 size=254
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002156<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2156;
    loop {
        match pc {
        0x2156 => {
            // block 2156
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2156 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2156 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2157 mov rbp,rsp
            rbp = rsp;
            // 215a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 215c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 215e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2160 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2161 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2165 mov r14,rdx
            r14 = rdx;
            // 2168 mov r15,rsi
            r15 = rsi;
            // 216b mov r12,rdi
            r12 = rdi;
            // 216e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2172 call QWORD PTR [rax+0x418]
            { let r = { let t = m.r64(rax.wrapping_add(0x418)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2178 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 217a je 0x21c6
            if fl.e() { pc = 0x21c6; continue; }
            pc = 0x217c;
        }
        0x217c => {
            // block 217c
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 217c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 217c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2180 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 2184 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 2186 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 218b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2190 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2193 mov rdi,r12
            rdi = r12;
            // 2196 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2199 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 219b je 0x2244
            if fl.e() { pc = 0x2244; continue; }
            pc = 0x21a1;
        }
        0x21a1 => {
            // block 21a1
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21a1 mov eax,0x300000
            rax = (0x300000u64) as u32 as u64;
            // 21a6 and eax,DWORD PTR [rbp-0x24]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 21a9 mov DWORD PTR [rbp-0x24],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rax as u32 as u64)) as u32);
            // 21ac cmp eax,0x300000
            fl = sub::<32>((rax as u32 as u64), 0x300000u64).1;
            // 21b1 je 0x21ca
            if fl.e() { pc = 0x21ca; continue; }
            pc = 0x21b3;
        }
        0x21b3 => {
            // block 21b3
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21b3 cmp eax,0x200000
            fl = sub::<32>((rax as u32 as u64), 0x200000u64).1;
            // 21b8 jne 0x21d3
            if fl.ne() { pc = 0x21d3; continue; }
            pc = 0x21ba;
        }
        0x21ba => {
            // block 21ba
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21ba mov eax,0x800000
            rax = (0x800000u64) as u32 as u64;
            // 21bf mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 21c4 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21c6 => {
            // block 21c6
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21c6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 21c8 jmp 0x2244
            pc = 0x2244; continue;
        }
        0x21ca => {
            // block 21ca
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21ca mov eax,0x1000000
            rax = (0x1000000u64) as u32 as u64;
            // 21cf xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 21d1 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21d3 => {
            // block 21d3
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21d3 cmp eax,0x100000
            fl = sub::<32>((rax as u32 as u64), 0x100000u64).1;
            // 21d8 jne 0x21e6
            if fl.ne() { pc = 0x21e6; continue; }
            pc = 0x21da;
        }
        0x21da => {
            // block 21da
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21da mov eax,0x400000
            rax = (0x400000u64) as u32 as u64;
            // 21df mov ebx,0x2
            rbx = (0x2u64) as u32 as u64;
            // 21e4 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21e6 => {
            // block 21e6
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21e6 mov eax,0x200000
            rax = (0x200000u64) as u32 as u64;
            // 21eb mov ebx,0x4
            rbx = (0x4u64) as u32 as u64;
            pc = 0x21f0;
        }
        0x21f0 => {
            // block 21f0
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21f0 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 21f3 je 0x21f8
            if fl.e() { pc = 0x21f8; continue; }
            pc = 0x21f5;
        }
        0x21f5 => {
            // block 21f5
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21f5 mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            pc = 0x21f8;
        }
        0x21f8 => {
            // block 21f8
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21f8 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 21fb je 0x2215
            if fl.e() { pc = 0x2215; continue; }
            pc = 0x21fd;
        }
        0x21fd => {
            // block 21fd
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21fd mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 2202 call 0xd19e0
            { let r = { let t = m.env.code_va(0xd19e0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2207 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 2209 je 0x2210
            if fl.e() { pc = 0x2210; continue; }
            pc = 0x220b;
        }
        0x220b => {
            // block 220b
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 220b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 220b imul eax,ebx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rbx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 220e jmp 0x2212
            pc = 0x2212; continue;
        }
        0x2210 => {
            // block 2210
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2210 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2210 shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0x2212;
        }
        0x2212 => {
            // block 2212
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2212 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2212 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            pc = 0x2215;
        }
        0x2215 => {
            // block 2215
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2215 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2215 mov DWORD PTR [rbp-0x28],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x12u64) as u32);
            // 221c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2220 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2224 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2226 mov rdi,r12
            rdi = r12;
            // 2229 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 222f mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 2232 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2235 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // 2237 cmp eax,0x10
            fl = sub::<32>((rax as u32 as u64), 0x10u64).1;
            // 223a jne 0x2244
            if fl.ne() { pc = 0x2244; continue; }
            pc = 0x223c;
        }
        0x223c => {
            // block 223c
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 223c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 223c shl DWORD PTR [r15],0x2
            { let (r, f) = shl::<32>(m.r32(r15) as u64, 0x2u64, fl); m.w32(r15, (r) as u32); fl = f; }
            // 2240 shr DWORD PTR [r14],0x2
            { let (r, f) = shr::<32>(m.r32(r14) as u64, 0x2u64, fl); m.w32(r14, (r) as u32); fl = f; }
            pc = 0x2244;
        }
        0x2244 => {
            // block 2244
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2244 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 2246 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 224a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 224b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 224d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 224f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2251 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2252 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15c78 size=408
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00015c78<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x15c78;
    loop {
        match pc {
        0x15c78 => {
            // block 15c78
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15c78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15c78 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15c79 mov rbp,rsp
            rbp = rsp;
            // 15c7c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15c7e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15c7f mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 15c82 mov rbx,rdi
            rbx = rdi;
            // 15c85 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 15c87 cmp QWORD PTR [rbx+0x78],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x78)) as u64, 0x0u64).1;
            // 15c8c jne 0x15d94
            if fl.ne() { pc = 0x15d94; continue; }
            pc = 0x15c92;
        }
        0x15c92 => {
            // block 15c92
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15c92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15c92 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 15c95 and edx,0xfffffffd
            { let r = (rdx as u32 as u64) & 0xfffffffdu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 15c98 lea rcx,[rip+0x1145c5] # 0x12a264
            rcx = m.env.data_va(256561);
            // 15c9f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 15ca1 cmp edx,0x2f
            fl = sub::<32>((rdx as u32 as u64), 0x2fu64).1;
            // 15ca4 jg 0x15cca
            if fl.g() { pc = 0x15cca; continue; }
            pc = 0x15ca6;
        }
        0x15ca6 => {
            // block 15ca6
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15ca6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ca6 dec edx
            { let (r, f) = dec::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // 15ca8 cmp edx,0x1c
            fl = sub::<32>((rdx as u32 as u64), 0x1cu64).1;
            // 15cab ja 0x15d94
            if fl.a() { pc = 0x15d94; continue; }
            pc = 0x15cb1;
        }
        0x15cb1 => {
            // block 15cb1
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cb1 lea rdi,[rip+0xe4] # 0x15d9c
            rdi = m.env.code_va(0x15d9c);
            // 15cb8 movsxd rdx,DWORD PTR [rdi+rdx*4]
            rdx = sx::<32>(m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // 15cbc add rdx,rdi
            { let (r, f) = add::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // 15cbf jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x15cc1 => {
            // block 15cc1
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cc1 lea rcx,[rip+0x11459e] # 0x12a266
            rcx = m.env.data_va(256563);
            // 15cc8 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15cca => {
            // block 15cca
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cca cmp edx,0x37
            fl = sub::<32>((rdx as u32 as u64), 0x37u64).1;
            // 15ccd jg 0x15ce6
            if fl.g() { pc = 0x15ce6; continue; }
            pc = 0x15ccf;
        }
        0x15ccf => {
            // block 15ccf
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15ccf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ccf cmp edx,0x30
            fl = sub::<32>((rdx as u32 as u64), 0x30u64).1;
            // 15cd2 je 0x15d43
            if fl.e() { pc = 0x15d43; continue; }
            pc = 0x15cd4;
        }
        0x15cd4 => {
            // block 15cd4
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cd4 cmp edx,0x34
            fl = sub::<32>((rdx as u32 as u64), 0x34u64).1;
            // 15cd7 jne 0x15d94
            if fl.ne() { pc = 0x15d94; continue; }
            pc = 0x15cdd;
        }
        0x15cdd => {
            // block 15cdd
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cdd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cdd lea rcx,[rip+0x11458f] # 0x12a273
            rcx = m.env.data_va(256576);
            // 15ce4 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15ce6 => {
            // block 15ce6
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15ce6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ce6 cmp edx,0x38
            fl = sub::<32>((rdx as u32 as u64), 0x38u64).1;
            // 15ce9 jne 0x15cf4
            if fl.ne() { pc = 0x15cf4; continue; }
            pc = 0x15ceb;
        }
        0x15ceb => {
            // block 15ceb
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15ceb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ceb lea rcx,[rip+0x11457b] # 0x12a26d
            rcx = m.env.data_va(256570);
            // 15cf2 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15cf4 => {
            // block 15cf4
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cf4 cmp edx,0x3c
            fl = sub::<32>((rdx as u32 as u64), 0x3cu64).1;
            // 15cf7 jne 0x15d94
            if fl.ne() { pc = 0x15d94; continue; }
            pc = 0x15cfd;
        }
        0x15cfd => {
            // block 15cfd
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cfd lea rcx,[rip+0x11457c] # 0x12a280
            rcx = m.env.data_va(256589);
            // 15d04 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d06 => {
            // block 15d06
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d06 lea rcx,[rip+0x114569] # 0x12a276
            rcx = m.env.data_va(256579);
            // 15d0d jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d0f => {
            // block 15d0f
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d0f lea rcx,[rip+0x11455a] # 0x12a270
            rcx = m.env.data_va(256573);
            // 15d16 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d18 => {
            // block 15d18
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d18 lea rcx,[rip+0x114565] # 0x12a284
            rcx = m.env.data_va(256593);
            // 15d1f jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d21 => {
            // block 15d21
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d21 lea rcx,[rip+0x114540] # 0x12a268
            rcx = m.env.data_va(256565);
            // 15d28 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d2a => {
            // block 15d2a
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d2a lea rcx,[rip+0x114548] # 0x12a279
            rcx = m.env.data_va(256582);
            // 15d31 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d33 => {
            // block 15d33
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d33 lea rcx,[rip+0x114530] # 0x12a26a
            rcx = m.env.data_va(256567);
            // 15d3a jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d3c => {
            // block 15d3c
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d3c lea rcx,[rip+0x114539] # 0x12a27c
            rcx = m.env.data_va(256585);
            pc = 0x15d43;
        }
        0x15d43 => {
            // block 15d43
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d43 mov rdi,rsi
            rdi = rsi;
            // 15d46 mov rsi,rcx
            rsi = rcx;
            // 15d49 call 0xec1c4
            { let r = { let t = m.env.stub_va(0x991); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15d4e mov rcx,rax
            rcx = rax;
            // 15d51 mov QWORD PTR [rbx+0x78],rcx
            m.w64(rbx.wrapping_add(0x78), (rcx) as u64);
            // 15d55 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 15d57 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 15d5a je 0x15d94
            if fl.e() { pc = 0x15d94; continue; }
            pc = 0x15d5c;
        }
        0x15d5c => {
            // block 15d5c
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d5c mov DWORD PTR [rbx+0x188],r14d
            m.w32(rbx.wrapping_add(0x188), ((r14 as u32 as u64)) as u32);
            // 15d63 test r14b,0x2
            fl = logic::<8>((r14 as u8 as u64) & 0x2u64);
            // 15d67 je 0x15d91
            if fl.e() { pc = 0x15d91; continue; }
            pc = 0x15d69;
        }
        0x15d69 => {
            // block 15d69
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d69 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 15d6b mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 15d70 mov rdi,rcx
            rdi = rcx;
            // 15d73 call 0xec1d6
            { let r = { let t = m.env.stub_va(0x9a3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15d78 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 15d7a je 0x15d91
            if fl.e() { pc = 0x15d91; continue; }
            pc = 0x15d7c;
        }
        0x15d7c => {
            // block 15d7c
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d7c mov rdi,QWORD PTR [rbx+0x78]
            rdi = m.r64(rbx.wrapping_add(0x78)) as u64;
            // 15d80 call 0xec1b8
            { let r = { let t = m.env.stub_va(0x985); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15d85 mov QWORD PTR [rbx+0x78],0x0
            m.w64(rbx.wrapping_add(0x78), (0x0u64) as u64);
            // 15d8d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 15d8f jmp 0x15d94
            pc = 0x15d94; continue;
        }
        0x15d91 => {
            // block 15d91
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d91 mov rax,rbx
            rax = rbx;
            pc = 0x15d94;
        }
        0x15d94 => {
            // block 15d94
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d94 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15d95 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15d97 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15d98 ret
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
            #[cfg(feature = "trace")] eprintln!("f_000430e6 pc 430e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000430e6 pc 43104 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000430e6 pc 4312c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000430e6 pc 43138 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43138 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4313a;
        }
        0x4313a => {
            // block 4313a
            #[cfg(feature = "trace")] eprintln!("f_000430e6 pc 4313a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00008bdc pc 8bdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00008bdc pc 8bff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8bff xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8c01 jmp 0x8c41
            pc = 0x8c41; continue;
        }
        0x8c03 => {
            // block 8c03
            #[cfg(feature = "trace")] eprintln!("f_00008bdc pc 8c03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00008bdc pc 8c41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000681f4 pc 681f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000681f4 pc 68208 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68208 test dx,dx
            fl = logic::<16>((rdx as u16 as u64) & (rdx as u16 as u64));
            // 6820b je 0x68246
            if fl.e() { pc = 0x68246; continue; }
            pc = 0x6820d;
        }
        0x6820d => {
            // block 6820d
            #[cfg(feature = "trace")] eprintln!("f_000681f4 pc 6820d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000681f4 pc 68221 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68221 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 68224 mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // 68229 mov rdi,r15
            rdi = r15;
            // 6822c mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 6822f call QWORD PTR [rax+0xc0]
            { let r = { let t = m.r64(rax.wrapping_add(0xc0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68235 test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // 68238 je 0x68246
            if fl.e() { pc = 0x68246; continue; }
            pc = 0x6823a;
        }
        0x6823a => {
            // block 6823a
            #[cfg(feature = "trace")] eprintln!("f_000681f4 pc 6823a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000681f4 pc 68246 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x65c08 size=1980
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00065c08<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x65c08;
    loop {
        match pc {
        0x65c08 => {
            // block 65c08
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65c08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65c08 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 65c09 mov rbp,rsp
            rbp = rsp;
            // 65c0c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 65c0e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 65c10 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 65c12 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65c14 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65c15 mov DWORD PTR [rbp-0x2c],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r8 as u32 as u64)) as u32);
            // 65c19 mov DWORD PTR [rbp-0x30],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rdx as u32 as u64)) as u32);
            // 65c1c test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 65c1e je 0x663b9
            if fl.e() { pc = 0x663b9; continue; }
            pc = 0x65c24;
        }
        0x65c24 => {
            // block 65c24
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65c24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65c24 mov r15b,BYTE PTR [rdi+0x2]
            r15 = (r15 & !0xff) | ((m.r8(rdi.wrapping_add(0x2)) as u64) as u8 as u64);
            // 65c28 mov r10b,BYTE PTR [rdi]
            r10 = (r10 & !0xff) | ((m.r8(rdi) as u64) as u8 as u64);
            // 65c2b mov r8b,BYTE PTR [rdi+0x1]
            r8 = (r8 & !0xff) | ((m.r8(rdi.wrapping_add(0x1)) as u64) as u8 as u64);
            // 65c2f xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            pc = 0x65c32;
        }
        0x65c32 => {
            // block 65c32
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65c32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65c32 mov r12d,r11d
            r12 = ((r11 as u32 as u64)) as u32 as u64;
            // 65c35 movzx eax,r8b
            rax = ((r8 as u8 as u64)) as u32 as u64;
            // 65c39 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 65c3b je 0x65c76
            if fl.e() { pc = 0x65c76; continue; }
            pc = 0x65c3d;
        }
        0x65c3d => {
            // block 65c3d
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65c3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65c3d mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 65c40 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 65c42 je 0x65caf
            if fl.e() { pc = 0x65caf; continue; }
            pc = 0x65c44;
        }
        0x65c44 => {
            // block 65c44
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65c44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65c44 imul r14d,eax,0xff40
            { let (r, f) = imul::<32>((rax as u32 as u64), 0xff40u64); r14 = (r) as u32 as u64; fl = f; }
            // 65c4b movzx r9d,r15b
            r9 = ((r15 as u8 as u64)) as u32 as u64;
            // 65c4f imul edx,r9d,0x15ddf
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x15ddfu64); rdx = (r) as u32 as u64; fl = f; }
            // 65c56 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65c59 cmp edx,0xbee37f
            fl = sub::<32>((rdx as u32 as u64), 0xbee37fu64).1;
            // 65c5f ja 0x65d13
            if fl.a() { pc = 0x65d13; continue; }
            pc = 0x65c65;
        }
        0x65c65 => {
            // block 65c65
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65c65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65c65 mov eax,0xbf6380
            rax = (0xbf6380u64) as u32 as u64;
            // 65c6a sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65c6c sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65c6f neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 65c71 jmp 0x65d1e
            pc = 0x65d1e; continue;
        }
        0x65c76 => {
            // block 65c76
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65c76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65c76 mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 65c79 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 65c7b je 0x65ce1
            if fl.e() { pc = 0x65ce1; continue; }
            pc = 0x65c7d;
        }
        0x65c7d => {
            // block 65c7d
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65c7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65c7d imul r14d,eax,0xffdf
            { let (r, f) = imul::<32>((rax as u32 as u64), 0xffdfu64); r14 = (r) as u32 as u64; fl = f; }
            // 65c84 movzx r9d,r15b
            r9 = ((r15 as u8 as u64)) as u32 as u64;
            // 65c88 imul edx,r9d,0x19a8c
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x19a8cu64); rdx = (r) as u32 as u64; fl = f; }
            // 65c8f add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65c92 cmp edx,0xdd43ef
            fl = sub::<32>((rdx as u32 as u64), 0xdd43efu64).1;
            // 65c98 ja 0x65d38
            if fl.a() { pc = 0x65d38; continue; }
            pc = 0x65c9e;
        }
        0x65c9e => {
            // block 65c9e
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65c9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65c9e mov eax,0xddc3f0
            rax = (0xddc3f0u64) as u32 as u64;
            // 65ca3 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65ca5 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65ca8 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 65caa jmp 0x65d43
            pc = 0x65d43; continue;
        }
        0x65caf => {
            // block 65caf
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65caf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65caf imul r14d,eax,0x12a15
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12a15u64); r14 = (r) as u32 as u64; fl = f; }
            // 65cb6 movzx r9d,r15b
            r9 = ((r15 as u8 as u64)) as u32 as u64;
            // 65cba imul edx,r9d,0x19895
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x19895u64); rdx = (r) as u32 as u64; fl = f; }
            // 65cc1 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65cc4 cmp edx,0xdeebcf
            fl = sub::<32>((rdx as u32 as u64), 0xdeebcfu64).1;
            // 65cca ja 0x65d64
            if fl.a() { pc = 0x65d64; continue; }
            pc = 0x65cd0;
        }
        0x65cd0 => {
            // block 65cd0
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65cd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65cd0 mov eax,0xdf6bd0
            rax = (0xdf6bd0u64) as u32 as u64;
            // 65cd5 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65cd7 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65cda neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 65cdc jmp 0x65d6f
            pc = 0x65d6f; continue;
        }
        0x65ce1 => {
            // block 65ce1
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65ce1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65ce1 imul r14d,eax,0x12acf
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12acfu64); r14 = (r) as u32 as u64; fl = f; }
            // 65ce8 movzx r9d,r15b
            r9 = ((r15 as u8 as u64)) as u32 as u64;
            // 65cec imul edx,r9d,0x1df71
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x1df71u64); rdx = (r) as u32 as u64; fl = f; }
            // 65cf3 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65cf6 cmp edx,0x102656f
            fl = sub::<32>((rdx as u32 as u64), 0x102656fu64).1;
            // 65cfc ja 0x65d8b
            if fl.a() { pc = 0x65d8b; continue; }
            pc = 0x65d02;
        }
        0x65d02 => {
            // block 65d02
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d02 mov eax,0x102e570
            rax = (0x102e570u64) as u32 as u64;
            // 65d07 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65d09 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65d0c neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 65d0e jmp 0x65d96
            pc = 0x65d96; continue;
        }
        0x65d13 => {
            // block 65d13
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d13 add edx,0xff419c80
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff419c80u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65d19 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65d1c mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x65d1e;
        }
        0x65d1e => {
            // block 65d1e
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d1e lea edx,[rax+0x10]
            rdx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 65d21 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65d23 cmp edx,0xff
            fl = sub::<32>((rdx as u32 as u64), 0xffu64).1;
            // 65d29 jg 0x65db4
            if fl.g() { pc = 0x65db4; continue; }
            pc = 0x65d2f;
        }
        0x65d2f => {
            // block 65d2f
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d2f cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 65d32 jl 0x65db2
            if fl.l() { pc = 0x65db2; continue; }
            pc = 0x65d34;
        }
        0x65d34 => {
            // block 65d34
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d34 mov bl,dl
            rbx = (rbx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 65d36 jmp 0x65db4
            pc = 0x65db4; continue;
        }
        0x65d38 => {
            // block 65d38
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d38 add edx,0xff233c10
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff233c10u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65d3e sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65d41 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x65d43;
        }
        0x65d43 => {
            // block 65d43
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d43 lea edx,[rax+0x10]
            rdx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 65d46 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65d48 cmp edx,0xff
            fl = sub::<32>((rdx as u32 as u64), 0xffu64).1;
            // 65d4e jg 0x65e0b
            if fl.g() { pc = 0x65e0b; continue; }
            pc = 0x65d54;
        }
        0x65d54 => {
            // block 65d54
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d54 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 65d57 jl 0x65e09
            if fl.l() { pc = 0x65e09; continue; }
            pc = 0x65d5d;
        }
        0x65d5d => {
            // block 65d5d
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d5d mov bl,dl
            rbx = (rbx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 65d5f jmp 0x65e0b
            pc = 0x65e0b; continue;
        }
        0x65d64 => {
            // block 65d64
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d64 add edx,0xff219430
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff219430u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65d6a sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65d6d mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x65d6f;
        }
        0x65d6f => {
            // block 65d6f
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d6f mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65d71 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 65d76 jg 0x65e62
            if fl.g() { pc = 0x65e62; continue; }
            pc = 0x65d7c;
        }
        0x65d7c => {
            // block 65d7c
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d7c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65d7e js 0x65e60
            if fl.s() { pc = 0x65e60; continue; }
            pc = 0x65d84;
        }
        0x65d84 => {
            // block 65d84
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d84 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 65d86 jmp 0x65e62
            pc = 0x65e62; continue;
        }
        0x65d8b => {
            // block 65d8b
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d8b add edx,0xfefe1a90
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfefe1a90u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65d91 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65d94 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x65d96;
        }
        0x65d96 => {
            // block 65d96
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65d96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65d96 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65d98 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 65d9d jg 0x65eb4
            if fl.g() { pc = 0x65eb4; continue; }
            pc = 0x65da3;
        }
        0x65da3 => {
            // block 65da3
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65da3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65da3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65da5 js 0x65eb2
            if fl.s() { pc = 0x65eb2; continue; }
            pc = 0x65dab;
        }
        0x65dab => {
            // block 65dab
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65dab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65dab mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 65dad jmp 0x65eb4
            pc = 0x65eb4; continue;
        }
        0x65db2 => {
            // block 65db2
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65db2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65db2 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65db4;
        }
        0x65db4 => {
            // block 65db4
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65db4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65db4 mov BYTE PTR [rsi+r12*4+0x2],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rbx as u8 as u64)) as u8);
            // 65db9 movzx r13d,r10b
            r13 = ((r10 as u8 as u64)) as u32 as u64;
            // 65dbd imul edx,r13d,0x1ba34
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x1ba34u64); rdx = (r) as u32 as u64; fl = f; }
            // 65dc4 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65dc7 add edx,0xff12f200
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff12f200u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65dcd js 0x65ddc
            if fl.s() { pc = 0x65ddc; continue; }
            pc = 0x65dcf;
        }
        0x65dcf => {
            // block 65dcf
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65dcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65dcf add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65dd5 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65dd8 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65dda jmp 0x65de8
            pc = 0x65de8; continue;
        }
        0x65ddc => {
            // block 65ddc
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65ddc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65ddc mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65de1 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65de3 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65de6 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65de8;
        }
        0x65de8 => {
            // block 65de8
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65de8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65de8 lea edx,[rax+0x10]
            rdx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 65deb mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65ded cmp edx,0xff
            fl = sub::<32>((rdx as u32 as u64), 0xffu64).1;
            // 65df3 jg 0x65f06
            if fl.g() { pc = 0x65f06; continue; }
            pc = 0x65df9;
        }
        0x65df9 => {
            // block 65df9
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65df9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65df9 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 65dfc jl 0x65f04
            if fl.l() { pc = 0x65f04; continue; }
            pc = 0x65e02;
        }
        0x65e02 => {
            // block 65e02
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e02 mov bl,dl
            rbx = (rbx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 65e04 jmp 0x65f06
            pc = 0x65f06; continue;
        }
        0x65e09 => {
            // block 65e09
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e09 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65e0b;
        }
        0x65e0b => {
            // block 65e0b
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e0b mov BYTE PTR [rsi+r12*4+0x2],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rbx as u8 as u64)) as u8);
            // 65e10 movzx r13d,r10b
            r13 = ((r10 as u8 as u64)) as u32 as u64;
            // 65e14 imul edx,r13d,0x1dad7
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x1dad7u64); rdx = (r) as u32 as u64; fl = f; }
            // 65e1b add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e1e add edx,0xff029690
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff029690u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e24 js 0x65e33
            if fl.s() { pc = 0x65e33; continue; }
            pc = 0x65e26;
        }
        0x65e26 => {
            // block 65e26
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e26 add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e2c sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65e2f mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65e31 jmp 0x65e3f
            pc = 0x65e3f; continue;
        }
        0x65e33 => {
            // block 65e33
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e33 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65e38 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65e3a sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65e3d neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65e3f;
        }
        0x65e3f => {
            // block 65e3f
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e3f lea edx,[rax+0x10]
            rdx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 65e42 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65e44 cmp edx,0xff
            fl = sub::<32>((rdx as u32 as u64), 0xffu64).1;
            // 65e4a jg 0x65f2a
            if fl.g() { pc = 0x65f2a; continue; }
            pc = 0x65e50;
        }
        0x65e50 => {
            // block 65e50
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e50 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 65e53 jl 0x65f28
            if fl.l() { pc = 0x65f28; continue; }
            pc = 0x65e59;
        }
        0x65e59 => {
            // block 65e59
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e59 mov bl,dl
            rbx = (rbx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 65e5b jmp 0x65f2a
            pc = 0x65f2a; continue;
        }
        0x65e60 => {
            // block 65e60
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e60 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65e62;
        }
        0x65e62 => {
            // block 65e62
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e62 mov BYTE PTR [rsi+r12*4+0x2],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rbx as u8 as u64)) as u8);
            // 65e67 movzx r13d,r10b
            r13 = ((r10 as u8 as u64)) as u32 as u64;
            // 65e6b imul edx,r13d,0x20469
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x20469u64); rdx = (r) as u32 as u64; fl = f; }
            // 65e72 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e75 add edx,0xfeeb2a30
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfeeb2a30u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e7b js 0x65e8a
            if fl.s() { pc = 0x65e8a; continue; }
            pc = 0x65e7d;
        }
        0x65e7d => {
            // block 65e7d
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e7d add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e83 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65e86 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65e88 jmp 0x65e96
            pc = 0x65e96; continue;
        }
        0x65e8a => {
            // block 65e8a
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e8a mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65e8f sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65e91 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65e94 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65e96;
        }
        0x65e96 => {
            // block 65e96
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65e96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65e96 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65e98 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 65e9d jg 0x65f7b
            if fl.g() { pc = 0x65f7b; continue; }
            pc = 0x65ea3;
        }
        0x65ea3 => {
            // block 65ea3
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65ea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65ea3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65ea5 js 0x65f79
            if fl.s() { pc = 0x65f79; continue; }
            pc = 0x65eab;
        }
        0x65eab => {
            // block 65eab
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65eab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65eab mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 65ead jmp 0x65f7b
            pc = 0x65f7b; continue;
        }
        0x65eb2 => {
            // block 65eb2
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65eb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65eb2 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65eb4;
        }
        0x65eb4 => {
            // block 65eb4
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65eb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65eb4 mov BYTE PTR [rsi+r12*4+0x2],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rbx as u8 as u64)) as u8);
            // 65eb9 movzx r13d,r10b
            r13 = ((r10 as u8 as u64)) as u32 as u64;
            // 65ebd imul edx,r13d,0x22a86
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x22a86u64); rdx = (r) as u32 as u64; fl = f; }
            // 65ec4 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65ec7 add edx,0xfed81010
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfed81010u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65ecd js 0x65edc
            if fl.s() { pc = 0x65edc; continue; }
            pc = 0x65ecf;
        }
        0x65ecf => {
            // block 65ecf
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65ecf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65ecf add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65ed5 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65ed8 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65eda jmp 0x65ee8
            pc = 0x65ee8; continue;
        }
        0x65edc => {
            // block 65edc
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65edc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65edc mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65ee1 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65ee3 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65ee6 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65ee8;
        }
        0x65ee8 => {
            // block 65ee8
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65ee8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65ee8 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65eea cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 65eef jg 0x65f9f
            if fl.g() { pc = 0x65f9f; continue; }
            pc = 0x65ef5;
        }
        0x65ef5 => {
            // block 65ef5
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65ef5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65ef5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65ef7 js 0x65f9d
            if fl.s() { pc = 0x65f9d; continue; }
            pc = 0x65efd;
        }
        0x65efd => {
            // block 65efd
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65efd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65efd mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 65eff jmp 0x65f9f
            pc = 0x65f9f; continue;
        }
        0x65f04 => {
            // block 65f04
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f04 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65f06;
        }
        0x65f06 => {
            // block 65f06
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f06 mov BYTE PTR [rsi+r12*4],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rbx as u8 as u64)) as u8);
            // 65f0a imul eax,r13d,0xffffaa1f
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffaa1fu64); rax = (r) as u32 as u64; fl = f; }
            // 65f11 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 65f14 imul edx,r9d,0xffff4dc9
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff4dc9u64); rdx = (r) as u32 as u64; fl = f; }
            // 65f1b add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f1e add edx,0x741800
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x741800u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f24 jns 0x65f4a
            if fl.ns() { pc = 0x65f4a; continue; }
            pc = 0x65f26;
        }
        0x65f26 => {
            // block 65f26
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f26 jmp 0x65f57
            pc = 0x65f57; continue;
        }
        0x65f28 => {
            // block 65f28
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f28 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65f2a;
        }
        0x65f2a => {
            // block 65f2a
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f2a mov BYTE PTR [rsi+r12*4],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rbx as u8 as u64)) as u8);
            // 65f2e imul eax,r13d,0xffffd007
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffd007u64); rax = (r) as u32 as u64; fl = f; }
            // 65f35 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 65f38 imul edx,r9d,0xffff87f3
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff87f3u64); rdx = (r) as u32 as u64; fl = f; }
            // 65f3f add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f42 add edx,0x440510
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x440510u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f48 js 0x65f57
            if fl.s() { pc = 0x65f57; continue; }
            pc = 0x65f4a;
        }
        0x65f4a => {
            // block 65f4a
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f4a add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f50 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65f53 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65f55 jmp 0x65f63
            pc = 0x65f63; continue;
        }
        0x65f57 => {
            // block 65f57
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f57 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65f5c sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65f5e sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65f61 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65f63;
        }
        0x65f63 => {
            // block 65f63
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f63 lea edx,[rax+0x10]
            rdx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 65f66 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65f68 cmp edx,0xff
            fl = sub::<32>((rdx as u32 as u64), 0xffu64).1;
            // 65f6e jg 0x65feb
            if fl.g() { pc = 0x65feb; continue; }
            pc = 0x65f70;
        }
        0x65f70 => {
            // block 65f70
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f70 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 65f73 jl 0x65fe9
            if fl.l() { pc = 0x65fe9; continue; }
            pc = 0x65f75;
        }
        0x65f75 => {
            // block 65f75
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f75 mov bl,dl
            rbx = (rbx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 65f77 jmp 0x65feb
            pc = 0x65feb; continue;
        }
        0x65f79 => {
            // block 65f79
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f79 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65f7b;
        }
        0x65f7b => {
            // block 65f7b
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f7b mov BYTE PTR [rsi+r12*4],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rbx as u8 as u64)) as u8);
            // 65f7f imul eax,r13d,0xffff9bb6
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffff9bb6u64); rax = (r) as u32 as u64; fl = f; }
            // 65f86 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 65f89 imul edx,r9d,0xffff2fe1
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff2fe1u64); rdx = (r) as u32 as u64; fl = f; }
            // 65f90 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f93 add edx,0x879330
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x879330u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f99 jns 0x65fbf
            if fl.ns() { pc = 0x65fbf; continue; }
            pc = 0x65f9b;
        }
        0x65f9b => {
            // block 65f9b
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f9b jmp 0x65fcc
            pc = 0x65fcc; continue;
        }
        0x65f9d => {
            // block 65f9d
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f9d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65f9f;
        }
        0x65f9f => {
            // block 65f9f
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65f9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65f9f mov BYTE PTR [rsi+r12*4],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rbx as u8 as u64)) as u8);
            // 65fa3 imul eax,r13d,0xffffc7fa
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffc7fau64); rax = (r) as u32 as u64; fl = f; }
            // 65faa add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 65fad imul edx,r9d,0xffff73ce
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff73ceu64); rdx = (r) as u32 as u64; fl = f; }
            // 65fb4 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65fb7 add edx,0x4f6f10
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x4f6f10u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65fbd js 0x65fcc
            if fl.s() { pc = 0x65fcc; continue; }
            pc = 0x65fbf;
        }
        0x65fbf => {
            // block 65fbf
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65fbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65fbf add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65fc5 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65fc8 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65fca jmp 0x65fd8
            pc = 0x65fd8; continue;
        }
        0x65fcc => {
            // block 65fcc
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65fcc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65fcc mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65fd1 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65fd3 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65fd6 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65fd8;
        }
        0x65fd8 => {
            // block 65fd8
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65fd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65fd8 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65fda cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 65fdf jg 0x65feb
            if fl.g() { pc = 0x65feb; continue; }
            pc = 0x65fe1;
        }
        0x65fe1 => {
            // block 65fe1
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65fe1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65fe1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65fe3 js 0x65fe9
            if fl.s() { pc = 0x65fe9; continue; }
            pc = 0x65fe5;
        }
        0x65fe5 => {
            // block 65fe5
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65fe5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65fe5 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 65fe7 jmp 0x65feb
            pc = 0x65feb; continue;
        }
        0x65fe9 => {
            // block 65fe9
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65fe9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65fe9 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65feb;
        }
        0x65feb => {
            // block 65feb
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 65feb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65feb mov BYTE PTR [rsi+r12*4+0x1],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x1), ((rbx as u8 as u64)) as u8);
            // 65ff0 movzx r10d,BYTE PTR [rdi+r11*2+0x4]
            r10 = (m.r8(rdi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u32 as u64;
            // 65ff6 mov r8b,BYTE PTR [rdi+r11*2+0x5]
            r8 = (r8 & !0xff) | ((m.r8(rdi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x5)) as u64) as u8 as u64);
            // 65ffb movzx r15d,BYTE PTR [rdi+r11*2+0x6]
            r15 = (m.r8(rdi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x6)) as u64) as u32 as u64;
            // 66001 add r13d,r10d
            { let (r, f) = add::<32>((r13 as u32 as u64), (r10 as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // 66004 shr r13d,1
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x1u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // 66007 add r9d,r15d
            { let (r, f) = add::<32>((r9 as u32 as u64), (r15 as u32 as u64));
              r9 = (r) as u32 as u64;
              fl = f; }
            // 6600a shr r9d,1
            { let (r, f) = shr::<32>((r9 as u32 as u64), 0x1u64, fl); r9 = (r) as u32 as u64; fl = f; }
            // 6600d lea r12d,[r11+0x1]
            r12 = (r11.wrapping_add(0x1)) as u32 as u64;
            // 66011 movzx eax,BYTE PTR [rdi+r11*2+0x3]
            rax = (m.r8(rdi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x3)) as u64) as u32 as u64;
            // 66017 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 66019 je 0x66050
            if fl.e() { pc = 0x66050; continue; }
            pc = 0x6601b;
        }
        0x6601b => {
            // block 6601b
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6601b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6601b mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 6601e test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 66020 je 0x66085
            if fl.e() { pc = 0x66085; continue; }
            pc = 0x66022;
        }
        0x66022 => {
            // block 66022
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66022 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66022 imul r14d,eax,0xff40
            { let (r, f) = imul::<32>((rax as u32 as u64), 0xff40u64); r14 = (r) as u32 as u64; fl = f; }
            // 66029 imul edx,r9d,0x15ddf
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x15ddfu64); rdx = (r) as u32 as u64; fl = f; }
            // 66030 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66033 cmp edx,0xbee37f
            fl = sub::<32>((rdx as u32 as u64), 0xbee37fu64).1;
            // 66039 ja 0x660dd
            if fl.a() { pc = 0x660dd; continue; }
            pc = 0x6603f;
        }
        0x6603f => {
            // block 6603f
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6603f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6603f mov eax,0xbf6380
            rax = (0xbf6380u64) as u32 as u64;
            // 66044 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 66046 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 66049 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 6604b jmp 0x660e8
            pc = 0x660e8; continue;
        }
        0x66050 => {
            // block 66050
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66050 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66050 mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 66053 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 66055 je 0x660b1
            if fl.e() { pc = 0x660b1; continue; }
            pc = 0x66057;
        }
        0x66057 => {
            // block 66057
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66057 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66057 imul r14d,eax,0xffdf
            { let (r, f) = imul::<32>((rax as u32 as u64), 0xffdfu64); r14 = (r) as u32 as u64; fl = f; }
            // 6605e imul edx,r9d,0x19a8c
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x19a8cu64); rdx = (r) as u32 as u64; fl = f; }
            // 66065 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66068 cmp edx,0xdd43ef
            fl = sub::<32>((rdx as u32 as u64), 0xdd43efu64).1;
            // 6606e ja 0x66106
            if fl.a() { pc = 0x66106; continue; }
            pc = 0x66074;
        }
        0x66074 => {
            // block 66074
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66074 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66074 mov eax,0xddc3f0
            rax = (0xddc3f0u64) as u32 as u64;
            // 66079 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 6607b sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 6607e neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 66080 jmp 0x66111
            pc = 0x66111; continue;
        }
        0x66085 => {
            // block 66085
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66085 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66085 imul ebx,eax,0x12a15
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12a15u64); rbx = (r) as u32 as u64; fl = f; }
            // 6608b imul edx,r9d,0x19895
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x19895u64); rdx = (r) as u32 as u64; fl = f; }
            // 66092 add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66094 cmp edx,0xdeebcf
            fl = sub::<32>((rdx as u32 as u64), 0xdeebcfu64).1;
            // 6609a ja 0x66132
            if fl.a() { pc = 0x66132; continue; }
            pc = 0x660a0;
        }
        0x660a0 => {
            // block 660a0
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 660a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 660a0 mov eax,0xdf6bd0
            rax = (0xdf6bd0u64) as u32 as u64;
            // 660a5 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 660a7 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 660aa neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 660ac jmp 0x6613d
            pc = 0x6613d; continue;
        }
        0x660b1 => {
            // block 660b1
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 660b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 660b1 imul ebx,eax,0x12acf
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12acfu64); rbx = (r) as u32 as u64; fl = f; }
            // 660b7 imul edx,r9d,0x1df71
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x1df71u64); rdx = (r) as u32 as u64; fl = f; }
            // 660be add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 660c0 cmp edx,0x102656f
            fl = sub::<32>((rdx as u32 as u64), 0x102656fu64).1;
            // 660c6 ja 0x66159
            if fl.a() { pc = 0x66159; continue; }
            pc = 0x660cc;
        }
        0x660cc => {
            // block 660cc
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 660cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 660cc mov eax,0x102e570
            rax = (0x102e570u64) as u32 as u64;
            // 660d1 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 660d3 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 660d6 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 660d8 jmp 0x66164
            pc = 0x66164; continue;
        }
        0x660dd => {
            // block 660dd
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 660dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 660dd add edx,0xff419c80
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff419c80u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 660e3 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 660e6 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x660e8;
        }
        0x660e8 => {
            // block 660e8
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 660e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 660e8 lea ebx,[rax+0x10]
            rbx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 660eb mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 660ed cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // 660f3 jg 0x66184
            if fl.g() { pc = 0x66184; continue; }
            pc = 0x660f9;
        }
        0x660f9 => {
            // block 660f9
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 660f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 660f9 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 660fc jl 0x66182
            if fl.l() { pc = 0x66182; continue; }
            pc = 0x66102;
        }
        0x66102 => {
            // block 66102
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66102 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66102 mov dl,bl
            rdx = (rdx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 66104 jmp 0x66184
            pc = 0x66184; continue;
        }
        0x66106 => {
            // block 66106
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66106 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66106 add edx,0xff233c10
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff233c10u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6610c sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 6610f mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x66111;
        }
        0x66111 => {
            // block 66111
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66111 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66111 lea ebx,[rax+0x10]
            rbx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 66114 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 66116 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // 6611c jg 0x661d7
            if fl.g() { pc = 0x661d7; continue; }
            pc = 0x66122;
        }
        0x66122 => {
            // block 66122
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66122 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66122 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 66125 jl 0x661d5
            if fl.l() { pc = 0x661d5; continue; }
            pc = 0x6612b;
        }
        0x6612b => {
            // block 6612b
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6612b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6612b mov dl,bl
            rdx = (rdx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 6612d jmp 0x661d7
            pc = 0x661d7; continue;
        }
        0x66132 => {
            // block 66132
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66132 add edx,0xff219430
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff219430u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66138 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 6613b mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x6613d;
        }
        0x6613d => {
            // block 6613d
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6613d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6613d mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 6613f cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 66144 jg 0x6622a
            if fl.g() { pc = 0x6622a; continue; }
            pc = 0x6614a;
        }
        0x6614a => {
            // block 6614a
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6614a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6614a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 6614c js 0x66228
            if fl.s() { pc = 0x66228; continue; }
            pc = 0x66152;
        }
        0x66152 => {
            // block 66152
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66152 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66152 mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 66154 jmp 0x6622a
            pc = 0x6622a; continue;
        }
        0x66159 => {
            // block 66159
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66159 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66159 add edx,0xfefe1a90
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfefe1a90u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6615f sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 66162 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x66164;
        }
        0x66164 => {
            // block 66164
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66164 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66164 mov r14b,0xff
            r14 = (r14 & !0xff) | ((0xffu64) as u8 as u64);
            // 66167 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 6616c jg 0x66278
            if fl.g() { pc = 0x66278; continue; }
            pc = 0x66172;
        }
        0x66172 => {
            // block 66172
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66172 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66172 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 66174 js 0x66275
            if fl.s() { pc = 0x66275; continue; }
            pc = 0x6617a;
        }
        0x6617a => {
            // block 6617a
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6617a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6617a mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 6617d jmp 0x66278
            pc = 0x66278; continue;
        }
        0x66182 => {
            // block 66182
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66182 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x66184;
        }
        0x66184 => {
            // block 66184
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66184 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66184 mov BYTE PTR [rsi+r12*4+0x2],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rdx as u8 as u64)) as u8);
            // 66189 imul edx,r13d,0x1ba34
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x1ba34u64); rdx = (r) as u32 as u64; fl = f; }
            // 66190 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66193 add edx,0xff12f200
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff12f200u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66199 js 0x661a8
            if fl.s() { pc = 0x661a8; continue; }
            pc = 0x6619b;
        }
        0x6619b => {
            // block 6619b
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6619b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6619b add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 661a1 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 661a4 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 661a6 jmp 0x661b4
            pc = 0x661b4; continue;
        }
        0x661a8 => {
            // block 661a8
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 661a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 661a8 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 661ad sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 661af sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 661b2 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x661b4;
        }
        0x661b4 => {
            // block 661b4
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 661b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 661b4 lea ebx,[rax+0x10]
            rbx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 661b7 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 661b9 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // 661bf jg 0x662c5
            if fl.g() { pc = 0x662c5; continue; }
            pc = 0x661c5;
        }
        0x661c5 => {
            // block 661c5
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 661c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 661c5 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 661c8 jl 0x662c3
            if fl.l() { pc = 0x662c3; continue; }
            pc = 0x661ce;
        }
        0x661ce => {
            // block 661ce
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 661ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 661ce mov dl,bl
            rdx = (rdx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 661d0 jmp 0x662c5
            pc = 0x662c5; continue;
        }
        0x661d5 => {
            // block 661d5
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 661d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 661d5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x661d7;
        }
        0x661d7 => {
            // block 661d7
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 661d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 661d7 mov BYTE PTR [rsi+r12*4+0x2],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rdx as u8 as u64)) as u8);
            // 661dc imul edx,r13d,0x1dad7
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x1dad7u64); rdx = (r) as u32 as u64; fl = f; }
            // 661e3 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 661e6 add edx,0xff029690
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff029690u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 661ec js 0x661fb
            if fl.s() { pc = 0x661fb; continue; }
            pc = 0x661ee;
        }
        0x661ee => {
            // block 661ee
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 661ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 661ee add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 661f4 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 661f7 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 661f9 jmp 0x66207
            pc = 0x66207; continue;
        }
        0x661fb => {
            // block 661fb
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 661fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 661fb mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 66200 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 66202 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 66205 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x66207;
        }
        0x66207 => {
            // block 66207
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66207 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66207 lea ebx,[rax+0x10]
            rbx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 6620a mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 6620c cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // 66212 jg 0x662e9
            if fl.g() { pc = 0x662e9; continue; }
            pc = 0x66218;
        }
        0x66218 => {
            // block 66218
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66218 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66218 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 6621b jl 0x662e7
            if fl.l() { pc = 0x662e7; continue; }
            pc = 0x66221;
        }
        0x66221 => {
            // block 66221
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66221 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66221 mov dl,bl
            rdx = (rdx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 66223 jmp 0x662e9
            pc = 0x662e9; continue;
        }
        0x66228 => {
            // block 66228
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66228 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66228 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x6622a;
        }
        0x6622a => {
            // block 6622a
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6622a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6622a mov BYTE PTR [rsi+r12*4+0x2],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rdx as u8 as u64)) as u8);
            // 6622f imul edx,r13d,0x20469
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x20469u64); rdx = (r) as u32 as u64; fl = f; }
            // 66236 add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66238 add edx,0xfeeb2a30
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfeeb2a30u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6623e js 0x6624d
            if fl.s() { pc = 0x6624d; continue; }
            pc = 0x66240;
        }
        0x66240 => {
            // block 66240
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66240 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66240 add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66246 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 66249 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 6624b jmp 0x66259
            pc = 0x66259; continue;
        }
        0x6624d => {
            // block 6624d
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6624d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6624d mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 66252 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 66254 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 66257 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x66259;
        }
        0x66259 => {
            // block 66259
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66259 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66259 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 6625b cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 66260 jg 0x6633a
            if fl.g() { pc = 0x6633a; continue; }
            pc = 0x66266;
        }
        0x66266 => {
            // block 66266
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66266 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66266 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 66268 js 0x66338
            if fl.s() { pc = 0x66338; continue; }
            pc = 0x6626e;
        }
        0x6626e => {
            // block 6626e
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6626e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6626e mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 66270 jmp 0x6633a
            pc = 0x6633a; continue;
        }
        0x66275 => {
            // block 66275
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66275 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66275 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x66278;
        }
        0x66278 => {
            // block 66278
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66278 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66278 mov BYTE PTR [rsi+r12*4+0x2],r14b
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((r14 as u8 as u64)) as u8);
            // 6627d imul edx,r13d,0x22a86
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x22a86u64); rdx = (r) as u32 as u64; fl = f; }
            // 66284 add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66286 add edx,0xfed81010
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfed81010u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6628c js 0x6629b
            if fl.s() { pc = 0x6629b; continue; }
            pc = 0x6628e;
        }
        0x6628e => {
            // block 6628e
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6628e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6628e add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66294 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 66297 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 66299 jmp 0x662a7
            pc = 0x662a7; continue;
        }
        0x6629b => {
            // block 6629b
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6629b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6629b mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 662a0 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 662a2 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 662a5 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x662a7;
        }
        0x662a7 => {
            // block 662a7
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 662a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 662a7 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 662a9 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 662ae jg 0x6635c
            if fl.g() { pc = 0x6635c; continue; }
            pc = 0x662b4;
        }
        0x662b4 => {
            // block 662b4
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 662b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 662b4 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 662b6 js 0x6635a
            if fl.s() { pc = 0x6635a; continue; }
            pc = 0x662bc;
        }
        0x662bc => {
            // block 662bc
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 662bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 662bc mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 662be jmp 0x6635c
            pc = 0x6635c; continue;
        }
        0x662c3 => {
            // block 662c3
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 662c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 662c3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x662c5;
        }
        0x662c5 => {
            // block 662c5
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 662c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 662c5 mov BYTE PTR [rsi+r12*4],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdx as u8 as u64)) as u8);
            // 662c9 imul eax,r13d,0xffffaa1f
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffaa1fu64); rax = (r) as u32 as u64; fl = f; }
            // 662d0 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 662d3 imul edx,r9d,0xffff4dc9
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff4dc9u64); rdx = (r) as u32 as u64; fl = f; }
            // 662da add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 662dd add edx,0x741800
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x741800u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 662e3 jns 0x66309
            if fl.ns() { pc = 0x66309; continue; }
            pc = 0x662e5;
        }
        0x662e5 => {
            // block 662e5
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 662e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 662e5 jmp 0x66316
            pc = 0x66316; continue;
        }
        0x662e7 => {
            // block 662e7
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 662e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 662e7 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x662e9;
        }
        0x662e9 => {
            // block 662e9
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 662e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 662e9 mov BYTE PTR [rsi+r12*4],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdx as u8 as u64)) as u8);
            // 662ed imul eax,r13d,0xffffd007
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffd007u64); rax = (r) as u32 as u64; fl = f; }
            // 662f4 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 662f7 imul edx,r9d,0xffff87f3
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff87f3u64); rdx = (r) as u32 as u64; fl = f; }
            // 662fe add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66301 add edx,0x440510
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x440510u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66307 js 0x66316
            if fl.s() { pc = 0x66316; continue; }
            pc = 0x66309;
        }
        0x66309 => {
            // block 66309
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66309 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66309 add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6630f sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 66312 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 66314 jmp 0x66322
            pc = 0x66322; continue;
        }
        0x66316 => {
            // block 66316
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66316 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66316 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 6631b sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 6631d sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 66320 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x66322;
        }
        0x66322 => {
            // block 66322
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66322 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66322 lea ebx,[rax+0x10]
            rbx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 66325 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 66327 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // 6632d jg 0x663a6
            if fl.g() { pc = 0x663a6; continue; }
            pc = 0x6632f;
        }
        0x6632f => {
            // block 6632f
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6632f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6632f cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 66332 jl 0x663a4
            if fl.l() { pc = 0x663a4; continue; }
            pc = 0x66334;
        }
        0x66334 => {
            // block 66334
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66334 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66334 mov dl,bl
            rdx = (rdx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 66336 jmp 0x663a6
            pc = 0x663a6; continue;
        }
        0x66338 => {
            // block 66338
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66338 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66338 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x6633a;
        }
        0x6633a => {
            // block 6633a
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6633a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6633a mov BYTE PTR [rsi+r12*4],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdx as u8 as u64)) as u8);
            // 6633e imul eax,r13d,0xffff9bb6
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffff9bb6u64); rax = (r) as u32 as u64; fl = f; }
            // 66345 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 66347 imul edx,r9d,0xffff2fe1
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff2fe1u64); rdx = (r) as u32 as u64; fl = f; }
            // 6634e add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66350 add edx,0x879330
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x879330u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66356 jns 0x6637a
            if fl.ns() { pc = 0x6637a; continue; }
            pc = 0x66358;
        }
        0x66358 => {
            // block 66358
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66358 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66358 jmp 0x66387
            pc = 0x66387; continue;
        }
        0x6635a => {
            // block 6635a
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6635a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6635a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x6635c;
        }
        0x6635c => {
            // block 6635c
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6635c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6635c mov BYTE PTR [rsi+r12*4],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdx as u8 as u64)) as u8);
            // 66360 imul eax,r13d,0xffffc7fa
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffc7fau64); rax = (r) as u32 as u64; fl = f; }
            // 66367 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 66369 imul edx,r9d,0xffff73ce
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff73ceu64); rdx = (r) as u32 as u64; fl = f; }
            // 66370 add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66372 add edx,0x4f6f10
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x4f6f10u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66378 js 0x66387
            if fl.s() { pc = 0x66387; continue; }
            pc = 0x6637a;
        }
        0x6637a => {
            // block 6637a
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6637a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6637a add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66380 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 66383 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 66385 jmp 0x66393
            pc = 0x66393; continue;
        }
        0x66387 => {
            // block 66387
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66387 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66387 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 6638c sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 6638e sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 66391 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x66393;
        }
        0x66393 => {
            // block 66393
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 66393 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 66393 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 66395 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 6639a jg 0x663a6
            if fl.g() { pc = 0x663a6; continue; }
            pc = 0x6639c;
        }
        0x6639c => {
            // block 6639c
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 6639c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6639c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 6639e js 0x663a4
            if fl.s() { pc = 0x663a4; continue; }
            pc = 0x663a0;
        }
        0x663a0 => {
            // block 663a0
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 663a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 663a0 mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 663a2 jmp 0x663a6
            pc = 0x663a6; continue;
        }
        0x663a4 => {
            // block 663a4
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 663a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 663a4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x663a6;
        }
        0x663a6 => {
            // block 663a6
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 663a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 663a6 mov BYTE PTR [rsi+r12*4+0x1],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x1), ((rdx as u8 as u64)) as u8);
            // 663ab add r11,0x2
            { let (r, f) = add::<64>(r11, 0x2u64);
              r11 = r;
              fl = f; }
            // 663af cmp r11d,DWORD PTR [rbp-0x30]
            fl = sub::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 663b3 jb 0x65c32
            if fl.b() { pc = 0x65c32; continue; }
            pc = 0x663b9;
        }
        0x663b9 => {
            // block 663b9
            #[cfg(feature = "trace")] eprintln!("f_00065c08 pc 663b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 663b9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 663ba pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 663bc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 663be pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 663c0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 663c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 663c3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe171e size=592
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_000e171e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe171e;
    loop {
        match pc {
        0xe171e => {
            // block e171e
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e171e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e171e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e171f mov rbp,rsp
            rbp = rsp;
            // e1722 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e1724 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1726 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e1728 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e172a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e172b sub rsp,0x138
            { let (r, f) = sub::<64>(rsp, 0x138u64);
              rsp = r;
              fl = f; }
            // e1732 mov rbx,rdx
            rbx = rdx;
            // e1735 mov r13,rsi
            r13 = rsi;
            // e1738 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e173f lea r14,[rbp-0x128]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // e1746 mov rax,QWORD PTR [rip+0x5ea7b] # 0x1401c8
            rax = m.r64(m.env.data_va(346517)) as u64;
            // e174d lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // e1751 mov QWORD PTR [rbp-0x138],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rcx) as u64);
            // e1758 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // e175c mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // e1763 mov QWORD PTR [rbp-0x130],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (0x0u64) as u64);
            // e176e mov rsi,r14
            rsi = r14;
            // e1771 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1776 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // e177e mov DWORD PTR [rbp-0x30],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0xffffffffu64) as u32);
            // e1785 mov r12,QWORD PTR [rip+0x5ec2c] # 0x1403b8
            r12 = m.r64(m.env.data_va(347013)) as u64;
            // e178c lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // e1791 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // e1798 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // e179f add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // e17a3 mov QWORD PTR [rbp-0xc0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (r12) as u64);
            // e17aa mov rdi,r14
            rdi = r14;
            // e17ad call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e17b2 mov r15,QWORD PTR [rip+0x5ebef] # 0x1403a8
            r15 = m.r64(m.env.data_va(346997)) as u64;
            // e17b9 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // e17bd mov QWORD PTR [rbp-0x128],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (r15) as u64);
            // e17c4 mov QWORD PTR [rbp-0xd0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (0x0u64) as u64);
            // e17cf mov QWORD PTR [rbp-0xd8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (0x0u64) as u64);
            // e17da mov QWORD PTR [rbp-0xe0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x0u64) as u64);
            // e17e5 mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // e17f0 mov DWORD PTR [rbp-0xc8],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (0x8u64) as u32);
            // e17fa mov rdi,r14
            rdi = r14;
            // e17fd mov rsi,r13
            rsi = r13;
            // e1800 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1805 xorps xmm0,xmm0
            x0 = 0;
            // e1808 movaps XMMWORD PTR [rbp-0x150],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_feb0), x0);
            // e180f mov QWORD PTR [rbp-0x140],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (0x0u64) as u64);
            // e181a lea r13,[rbp-0x138]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // e1821 lea r14,[rbp-0x150]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // e1828 jmp 0xe182f
            pc = 0xe182f; continue;
        }
        0xe182a => {
            // block e182a
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e182a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e182a add QWORD PTR [rbx+0x8],0x18
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, 0x18u64);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            pc = 0xe182f;
        }
        0xe182f => {
            // block e182f
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e182f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e182f mov edx,0x2c
            rdx = (0x2cu64) as u32 as u64;
            // e1834 mov rdi,r13
            rdi = r13;
            // e1837 mov rsi,r14
            rsi = r14;
            // e183a call 0xebf00
            { let r = { let t = m.env.stub_va(0x6cd); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e183f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e1842 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e1846 test BYTE PTR [rcx+rax*1+0x20],0x5
            fl = logic::<8>(m.r8(rcx.wrapping_add(rax).wrapping_add(0x20)) as u64 & 0x5u64);
            // e184b jne 0xe18e0
            if fl.ne() { pc = 0xe18e0; continue; }
            pc = 0xe1851;
        }
        0xe1851 => {
            // block e1851
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e1851 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1851 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e1855 cmp rdi,QWORD PTR [rbx+0x10]
            fl = sub::<64>(rdi, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // e1859 je 0xe1865
            if fl.e() { pc = 0xe1865; continue; }
            pc = 0xe185b;
        }
        0xe185b => {
            // block e185b
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e185b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e185b mov rsi,r14
            rsi = r14;
            // e185e call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1863 jmp 0xe182a
            pc = 0xe182a; continue;
        }
        0xe1865 => {
            // block e1865
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e1865 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1865 mov rdi,rbx
            rdi = rbx;
            // e1868 mov rsi,r14
            rsi = r14;
            // e186b call 0xebefa
            { let r = { let t = m.env.stub_va(0x6c7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1870 jmp 0xe182f
            pc = 0xe182f; continue;
        }
        0xe1872 => {
            // block e1872
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e1872 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1872 mov rbx,rax
            rbx = rax;
            // e1875 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // e187c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1881 mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // e1888 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // e188f mov QWORD PTR [rbp-0xc0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (r12) as u64);
            // e1896 mov QWORD PTR [rbp-0x128],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (r15) as u64);
            pc = 0xe189d;
        }
        0xe189d => {
            // block e189d
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e189d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e189d lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // e18a4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e18a9 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // e18b0 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe18b5;
        }
        0xe18b5 => {
            // block e18b5
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e18b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e18b5 mov rsi,QWORD PTR [rip+0x5e944] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // e18bc add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e18c0 lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // e18c7 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe18cc;
        }
        0xe18cc => {
            // block e18cc
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e18cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e18cc lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e18d3 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e18d8 mov rdi,rbx
            rdi = rbx;
            // e18db call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
        0xe18e0 => {
            // block e18e0
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e18e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e18e0 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // e18e7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e18ec mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // e18f3 mov QWORD PTR [rbp-0x138],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rax) as u64);
            // e18fa mov QWORD PTR [rbp-0xc0],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (r12) as u64);
            // e1901 mov QWORD PTR [rbp-0x128],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (r15) as u64);
            // e1908 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // e190f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1914 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // e191b call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1920 mov rsi,QWORD PTR [rip+0x5e8d9] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // e1927 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e192b lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // e1932 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1937 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e193e call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1943 add rsp,0x138
            { let (r, f) = add::<64>(rsp, 0x138u64);
              rsp = r;
              fl = f; }
            // e194a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e194b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e194d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e194f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e1951 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e1953 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1954 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe1955 => {
            // block e1955
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e1955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1955 mov rbx,rax
            rbx = rax;
            // e1958 jmp 0xe18cc
            pc = 0xe18cc; continue;
        }
        0xe195d => {
            // block e195d
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e195d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e195d mov rbx,rax
            rbx = rax;
            // e1960 jmp 0xe18b5
            pc = 0xe18b5; continue;
        }
        0xe1965 => {
            // block e1965
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e1965 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1965 mov rbx,rax
            rbx = rax;
            // e1968 jmp 0xe189d
            pc = 0xe189d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x65678 size=302
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00065678<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x65678;
    loop {
        match pc {
        0x65678 => {
            // block 65678
            #[cfg(feature = "trace")] eprintln!("f_00065678 pc 65678 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65678 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 65679 mov rbp,rsp
            rbp = rsp;
            // 6567c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6567e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 65680 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 65682 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65684 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65685 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 65689 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 6568d mov r13,rdi
            r13 = rdi;
            // 65690 mov rsi,QWORD PTR [r13+0x0]
            rsi = m.r64(r13) as u64;
            // 65694 mov rbx,QWORD PTR [r13+0x8]
            rbx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 65698 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 6569b sar rbx,0x4
            { let (r, f) = sar::<64>(rbx, 0x4u64, fl); rbx = r; fl = f; }
            // 6569f inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 656a2 mov rax,rbx
            rax = rbx;
            // 656a5 shr rax,0x3c
            { let (r, f) = shr::<64>(rax, 0x3cu64, fl); rax = r; fl = f; }
            // 656a9 je 0x656b7
            if fl.e() { pc = 0x656b7; continue; }
            pc = 0x656ab;
        }
        0x656ab => {
            // block 656ab
            #[cfg(feature = "trace")] eprintln!("f_00065678 pc 656ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 656ab mov rdi,r13
            rdi = r13;
            // 656ae call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x656b7 => {
            // block 656b7
            #[cfg(feature = "trace")] eprintln!("f_00065678 pc 656b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 656b7 lea rax,[r13+0x10]
            rax = r13.wrapping_add(0x10);
            // 656bb mov r15,QWORD PTR [r13+0x10]
            r15 = m.r64(r13.wrapping_add(0x10)) as u64;
            // 656bf mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 656c3 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 656c6 mov rcx,r15
            rcx = r15;
            // 656c9 sar rcx,0x4
            { let (r, f) = sar::<64>(rcx, 0x4u64, fl); rcx = r; fl = f; }
            // 656cd movabs rdx,0x7fffffffffffffe
            rdx = 0x7ff_ffff_ffff_fffeu64;
            // 656d7 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 656da ja 0x656e9
            if fl.a() { pc = 0x656e9; continue; }
            pc = 0x656dc;
        }
        0x656dc => {
            // block 656dc
            #[cfg(feature = "trace")] eprintln!("f_00065678 pc 656dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 656dc sar r15,0x3
            { let (r, f) = sar::<64>(r15, 0x3u64, fl); r15 = r; fl = f; }
            // 656e0 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 656e3 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 656e7 jmp 0x656f3
            pc = 0x656f3; continue;
        }
        0x656e9 => {
            // block 656e9
            #[cfg(feature = "trace")] eprintln!("f_00065678 pc 656e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 656e9 movabs r15,0xfffffffffffffff
            r15 = 0xfff_ffff_ffff_ffffu64;
            pc = 0x656f3;
        }
        0x656f3 => {
            // block 656f3
            #[cfg(feature = "trace")] eprintln!("f_00065678 pc 656f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 656f3 mov r14,QWORD PTR [r13+0x8]
            r14 = m.r64(r13.wrapping_add(0x8)) as u64;
            // 656f7 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 656fb sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 656fe mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 65702 mov r12,r14
            r12 = r14;
            // 65705 sar r12,0x4
            { let (r, f) = sar::<64>(r12, 0x4u64, fl); r12 = r; fl = f; }
            // 65709 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 65711 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 65715 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 65717 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 6571a je 0x6572b
            if fl.e() { pc = 0x6572b; continue; }
            pc = 0x6571c;
        }
        0x6571c => {
            // block 6571c
            #[cfg(feature = "trace")] eprintln!("f_00065678 pc 6571c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6571c mov rdi,r15
            rdi = r15;
            // 6571f shl rdi,0x4
            { let (r, f) = shl::<64>(rdi, 0x4u64, fl); rdi = r; fl = f; }
            // 65723 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65728 mov rbx,rax
            rbx = rax;
            pc = 0x6572b;
        }
        0x6572b => {
            // block 6572b
            #[cfg(feature = "trace")] eprintln!("f_00065678 pc 6572b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6572b shl r12,0x4
            { let (r, f) = shl::<64>(r12, 0x4u64, fl); r12 = r; fl = f; }
            // 6572f shl r15,0x4
            { let (r, f) = shl::<64>(r15, 0x4u64, fl); r15 = r; fl = f; }
            // 65733 add r15,rbx
            { let (r, f) = add::<64>(r15, rbx);
              r15 = r;
              fl = f; }
            // 65736 mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 6573a mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 6573d mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 65741 mov QWORD PTR [rbx+r12*1+0x8],rcx
            m.w64(rbx.wrapping_add(r12).wrapping_add(0x8), (rcx) as u64);
            // 65746 mov QWORD PTR [rbx+r12*1],rax
            m.w64(rbx.wrapping_add(r12), (rax) as u64);
            // 6574a lea rax,[rbx+r12*1+0x10]
            rax = rbx.wrapping_add(r12).wrapping_add(0x10);
            // 6574f mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 65753 mov rdi,rbx
            rdi = rbx;
            // 65756 mov r12,QWORD PTR [rbp-0x58]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 6575a mov rsi,r12
            rsi = r12;
            // 6575d mov rdx,r14
            rdx = r14;
            // 65760 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65765 mov QWORD PTR [r13+0x0],rbx
            m.w64(r13, (rbx) as u64);
            // 65769 mov QWORD PTR [rbp-0x48],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r12) as u64);
            // 6576d mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 65771 mov QWORD PTR [r13+0x8],rax
            m.w64(r13.wrapping_add(0x8), (rax) as u64);
            // 65775 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 65779 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 6577d mov QWORD PTR [r13+0x10],r15
            m.w64(r13.wrapping_add(0x10), (r15) as u64);
            // 65781 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 65785 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 65789 mov QWORD PTR [rbp-0x50],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r12) as u64);
            // 6578d lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 65791 call 0xebd08
            { let r = { let t = m.env.stub_va(0x4d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 65796 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 6579a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6579b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6579d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 6579f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 657a1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 657a3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 657a4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdf88 size=266
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000df88<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdf88;
    loop {
        match pc {
        0xdf88 => {
            // block df88
            #[cfg(feature = "trace")] eprintln!("f_0000df88 pc df88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df88 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df89 mov rbp,rsp
            rbp = rsp;
            // df8c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // df8e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // df90 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // df92 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // df94 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df95 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // df99 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // df9d mov rbx,rdi
            rbx = rdi;
            // dfa0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfa3 lea rsi,[rbp-0x48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // dfa7 call QWORD PTR [rax+0x16d8]
            { let r = { let t = m.r64(rax.wrapping_add(0x16d8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfad mov BYTE PTR [rbp-0x49],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffb7), ((rax as u8 as u64)) as u8);
            // dfb0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfb3 lea rsi,[rbp-0x44]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // dfb7 mov rdi,rbx
            rdi = rbx;
            // dfba call QWORD PTR [rax+0x16e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x16e8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfc0 mov BYTE PTR [rbp-0x4a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffb6), ((rax as u8 as u64)) as u8);
            // dfc3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfc6 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // dfca mov rdi,rbx
            rdi = rbx;
            // dfcd call QWORD PTR [rax+0x16c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x16c8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfd3 mov BYTE PTR [rbp-0x4b],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffb5), ((rax as u8 as u64)) as u8);
            // dfd6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfd9 lea rsi,[rbp-0x3c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // dfdd mov rdi,rbx
            rdi = rbx;
            // dfe0 call QWORD PTR [rax+0x16d0]
            { let r = { let t = m.r64(rax.wrapping_add(0x16d0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfe6 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // dfe9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfec lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // dff0 mov rdi,rbx
            rdi = rbx;
            // dff3 call QWORD PTR [rax+0x16c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x16c0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dff9 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // dffc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfff lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // e003 mov rdi,rbx
            rdi = rbx;
            // e006 call QWORD PTR [rax+0x16f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x16f8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e00c mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e00f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e012 lea rsi,[rbp-0x33]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcd);
            // e016 mov rdi,rbx
            rdi = rbx;
            // e019 call QWORD PTR [rax+0x1708]
            { let r = { let t = m.r64(rax.wrapping_add(0x1708)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e01f mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e022 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e025 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e029 mov rdi,rbx
            rdi = rbx;
            // e02c call QWORD PTR [rax+0x1718]
            { let r = { let t = m.r64(rax.wrapping_add(0x1718)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e032 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e034 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e036 cmp BYTE PTR [rbp-0x49],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffb7)) as u64, 0x0u64).1;
            // e03a je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe03c;
        }
        0xe03c => {
            // block e03c
            #[cfg(feature = "trace")] eprintln!("f_0000df88 pc e03c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e03c cmp BYTE PTR [rbp-0x4a],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffb6)) as u64, 0x0u64).1;
            // e040 je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe042;
        }
        0xe042 => {
            // block e042
            #[cfg(feature = "trace")] eprintln!("f_0000df88 pc e042 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e042 cmp BYTE PTR [rbp-0x4b],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffb5)) as u64, 0x0u64).1;
            // e046 je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe048;
        }
        0xe048 => {
            // block e048
            #[cfg(feature = "trace")] eprintln!("f_0000df88 pc e048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e048 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // e04b je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe04d;
        }
        0xe04d => {
            // block e04d
            #[cfg(feature = "trace")] eprintln!("f_0000df88 pc e04d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e04d test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // e050 je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe052;
        }
        0xe052 => {
            // block e052
            #[cfg(feature = "trace")] eprintln!("f_0000df88 pc e052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e052 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // e055 je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe057;
        }
        0xe057 => {
            // block e057
            #[cfg(feature = "trace")] eprintln!("f_0000df88 pc e057 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e057 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // e05a je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe05c;
        }
        0xe05c => {
            // block e05c
            #[cfg(feature = "trace")] eprintln!("f_0000df88 pc e05c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e05c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e05e je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe060;
        }
        0xe060 => {
            // block e060
            #[cfg(feature = "trace")] eprintln!("f_0000df88 pc e060 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e060 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e063 mov rdx,QWORD PTR [rbp-0x58]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e067 mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // e06a mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e06e mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // e072 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e076 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // e07a mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // e07e mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // e081 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe083;
        }
        0xe083 => {
            // block e083
            #[cfg(feature = "trace")] eprintln!("f_0000df88 pc e083 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e083 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // e087 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e088 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e08a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e08c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e08e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e090 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e091 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
