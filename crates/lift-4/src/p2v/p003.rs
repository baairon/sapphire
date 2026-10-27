//! Mechanically lifted functions (scripts/lift_run.py), module p003. Each one is checked against the original
//! in the world harness (tests/world_p003.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x11e28 size=180
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011e28<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x11e28;
    loop {
        match pc {
        0x11e28 => {
            // block 11e28
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e28 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11e29 mov rbp,rsp
            rbp = rsp;
            // 11e2c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11e2e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11e30 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 11e32 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 11e34 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11e35 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 11e39 mov QWORD PTR [rbp-0x68],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rsi) as u64);
            // 11e3d mov r13,rdi
            r13 = rdi;
            // 11e40 cmp r13,rsi
            fl = sub::<64>(r13, rsi).1;
            // 11e43 je 0x11eb7
            if fl.e() { pc = 0x11eb7; continue; }
            pc = 0x11e45;
        }
        0x11e45 => {
            // block 11e45
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e45 lea rax,[r13+0x38]
            rax = r13.wrapping_add(0x38);
            // 11e49 cmp rax,QWORD PTR [rbp-0x68]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 11e4d je 0x11eb7
            if fl.e() { pc = 0x11eb7; continue; }
            pc = 0x11e4f;
        }
        0x11e4f => {
            // block 11e4f
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e4f lea rbx,[rbp-0x60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11e53 mov r12,r13
            r12 = r13;
            pc = 0x11e56;
        }
        0x11e56 => {
            // block 11e56
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e56 mov r15,rax
            r15 = rax;
            // 11e59 mov rdi,rbx
            rdi = rbx;
            // 11e5c mov rsi,r15
            rsi = r15;
            // 11e5f call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11e64 cmp r15,r13
            fl = sub::<64>(r15, r13).1;
            // 11e67 mov rbx,r15
            rbx = r15;
            // 11e6a mov r14,r15
            r14 = r15;
            // 11e6d je 0x11e90
            if fl.e() { pc = 0x11e90; continue; }
            pc = 0x11e6f;
        }
        0x11e6f => {
            // block 11e6f
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e6f mov eax,DWORD PTR [rbp-0x48]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 11e72 cmp eax,DWORD PTR [r14-0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // 11e76 jge 0x11e90
            if fl.ge() { pc = 0x11e90; continue; }
            pc = 0x11e78;
        }
        0x11e78 => {
            // block 11e78
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e78 add r14,0xffffffffffffffc8
            { let (r, f) = add::<64>(r14, 0xffff_ffff_ffff_ffc8u64);
              r14 = r;
              fl = f; }
            // 11e7c mov rdi,rbx
            rdi = rbx;
            // 11e7f mov rsi,r14
            rsi = r14;
            // 11e82 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11e87 add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            // 11e8b cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 11e8e jne 0x11e6f
            if fl.ne() { pc = 0x11e6f; continue; }
            pc = 0x11e90;
        }
        0x11e90 => {
            // block 11e90
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e90 mov rdi,rbx
            rdi = rbx;
            // 11e93 lea rbx,[rbp-0x60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11e97 mov rsi,rbx
            rsi = rbx;
            // 11e9a call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11e9f mov rdi,rbx
            rdi = rbx;
            // 11ea2 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11ea7 add r12,0x70
            { let (r, f) = add::<64>(r12, 0x70u64);
              r12 = r;
              fl = f; }
            // 11eab cmp r12,QWORD PTR [rbp-0x68]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 11eaf mov rax,r12
            rax = r12;
            // 11eb2 mov r12,r15
            r12 = r15;
            // 11eb5 jne 0x11e56
            if fl.ne() { pc = 0x11e56; continue; }
            pc = 0x11eb7;
        }
        0x11eb7 => {
            // block 11eb7
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11eb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11eb7 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 11ebb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11ebc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11ebe pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 11ec0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11ec2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11ec4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11ec5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x11ec6 => {
            // block 11ec6
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11ec6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11ec6 mov rbx,rax
            rbx = rax;
            // 11ec9 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11ecd call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11ed2 mov rdi,rbx
            rdi = rbx;
            // 11ed5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11eda jmp 0x11ec6
            pc = 0x11ec6; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3153c size=180
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0003153c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3153c;
    loop {
        match pc {
        0x3153c => {
            // block 3153c
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 3153c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3153c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3153d mov rbp,rsp
            rbp = rsp;
            // 31540 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 31542 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31544 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 31546 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 31548 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31549 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3154a mov r13,rdi
            r13 = rdi;
            // 3154d lea r14,[r13+0x18]
            r14 = r13.wrapping_add(0x18);
            // 31551 mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 31555 mov rdi,r14
            rdi = r14;
            // 31558 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3155e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31560 je 0x315db
            if fl.e() { pc = 0x315db; continue; }
            pc = 0x31562;
        }
        0x31562 => {
            // block 31562
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 31562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31562 mov rdi,r14
            rdi = r14;
            // 31565 call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3156a and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3156d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 3156f cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 31574 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // 3157a jne 0x315de
            if fl.ne() { pc = 0x315de; continue; }
            pc = 0x3157c;
        }
        0x3157c => {
            // block 3157c
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 3157c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3157c mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 31583 mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 31587 mov esi,DWORD PTR [r13+0x2a8]
            rsi = (m.r32(r13.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // 3158e lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 31592 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 31597 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3159a mov rdi,r14
            rdi = r14;
            // 3159d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 315a0 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 315a3 test BYTE PTR [rbp-0x2c],0x4
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & 0x4u64);
            // 315a7 jne 0x315de
            if fl.ne() { pc = 0x315de; continue; }
            pc = 0x315a9;
        }
        0x315a9 => {
            // block 315a9
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315a9 lea r12,[rbp-0x2c]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            pc = 0x315ad;
        }
        0x315ad => {
            // block 315ad
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315ad inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 315af cmp ebx,0x2710
            fl = sub::<32>((rbx as u32 as u64), 0x2710u64).1;
            // 315b5 ja 0x315db
            if fl.a() { pc = 0x315db; continue; }
            pc = 0x315b7;
        }
        0x315b7 => {
            // block 315b7
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315b7 mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 315bb mov esi,DWORD PTR [r13+0x2a8]
            rsi = (m.r32(r13.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // 315c2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 315c7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 315ca mov rdi,r14
            rdi = r14;
            // 315cd mov rdx,r12
            rdx = r12;
            // 315d0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 315d3 test BYTE PTR [rbp-0x2c],0x4
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & 0x4u64);
            // 315d7 je 0x315ad
            if fl.e() { pc = 0x315ad; continue; }
            pc = 0x315d9;
        }
        0x315d9 => {
            // block 315d9
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315d9 jmp 0x315de
            pc = 0x315de; continue;
        }
        0x315db => {
            // block 315db
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315db xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x315de;
        }
        0x315de => {
            // block 315de
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315de mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 315e1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 315e5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 315e6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 315e8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 315ea pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 315ec pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 315ee pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 315ef ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bbc2 size=184
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bbc2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2bbc2;
    loop {
        match pc {
        0x2bbc2 => {
            // block 2bbc2
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bbc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bbc2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bbc3 mov rbp,rsp
            rbp = rsp;
            // 2bbc6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bbc8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bbca push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2bbcc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2bbce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bbcf push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2bbd0 mov r15,rsi
            r15 = rsi;
            // 2bbd3 mov r12,rdi
            r12 = rdi;
            // 2bbd6 lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // 2bbda mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2bbde mov rdi,r15
            rdi = r15;
            // 2bbe1 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bbe6 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // 2bbee mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // 2bbf1 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 2bbf9 mov r13,QWORD PTR [r12+0x50]
            r13 = m.r64(r12.wrapping_add(0x50)) as u64;
            // 2bbfe test r13,r13
            fl = logic::<64>(r13 & r13);
            // 2bc01 je 0x2bc68
            if fl.e() { pc = 0x2bc68; continue; }
            pc = 0x2bc03;
        }
        0x2bc03 => {
            // block 2bc03
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc03 mov ecx,DWORD PTR [r12+0x58]
            rcx = (m.r32(r12.wrapping_add(0x58)) as u64) as u32 as u64;
            // 2bc08 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 2bc0a je 0x2bc68
            if fl.e() { pc = 0x2bc68; continue; }
            pc = 0x2bc0c;
        }
        0x2bc0c => {
            // block 2bc0c
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc0c mov eax,DWORD PTR [r12+0x4c]
            rax = (m.r32(r12.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 2bc11 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2bc13 je 0x2bc68
            if fl.e() { pc = 0x2bc68; continue; }
            pc = 0x2bc15;
        }
        0x2bc15 => {
            // block 2bc15
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc15 cmp eax,DWORD PTR [r12+0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r12.wrapping_add(0x20)) as u64).1;
            // 2bc1a ja 0x2bc68
            if fl.a() { pc = 0x2bc68; continue; }
            pc = 0x2bc1c;
        }
        0x2bc1c => {
            // block 2bc1c
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc1c mov rsi,QWORD PTR [r12+0x78]
            rsi = m.r64(r12.wrapping_add(0x78)) as u64;
            // 2bc21 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2bc24 je 0x2bc68
            if fl.e() { pc = 0x2bc68; continue; }
            pc = 0x2bc26;
        }
        0x2bc26 => {
            // block 2bc26
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc26 mov edx,DWORD PTR [r12+0x80]
            rdx = (m.r32(r12.wrapping_add(0x80)) as u64) as u32 as u64;
            // 2bc2e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2bc30 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 2bc32 je 0x2bc6a
            if fl.e() { pc = 0x2bc6a; continue; }
            pc = 0x2bc34;
        }
        0x2bc34 => {
            // block 2bc34
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc34 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 2bc36 jne 0x2bc6a
            if fl.ne() { pc = 0x2bc6a; continue; }
            pc = 0x2bc38;
        }
        0x2bc38 => {
            // block 2bc38
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc38 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bc3a;
        }
        0x2bc3a => {
            // block 2bc3a
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc3a mov eax,DWORD PTR [rsi+rbx*4]
            rax = (m.r32(rsi.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2bc3d mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // 2bc40 mov rdi,r15
            rdi = r15;
            // 2bc43 mov r14,rsi
            r14 = rsi;
            // 2bc46 mov rsi,r13
            rsi = r13;
            // 2bc49 call 0xeb870
            { let r = { let t = m.env.stub_va(0x3d); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bc4e mov rsi,r14
            rsi = r14;
            // 2bc51 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 2bc54 mov DWORD PTR [rax],ecx
            m.w32(rax, ((rcx as u32 as u64)) as u32);
            // 2bc56 add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // 2bc5a inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2bc5d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2bc5f cmp ebx,DWORD PTR [r12+0x4c]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r12.wrapping_add(0x4c)) as u64).1;
            // 2bc64 jb 0x2bc3a
            if fl.b() { pc = 0x2bc3a; continue; }
            pc = 0x2bc66;
        }
        0x2bc66 => {
            // block 2bc66
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc66 jmp 0x2bc6a
            pc = 0x2bc6a; continue;
        }
        0x2bc68 => {
            // block 2bc68
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc68 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bc6a;
        }
        0x2bc6a => {
            // block 2bc6a
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc6a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2bc6e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bc6f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2bc71 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2bc73 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bc75 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2bc77 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bc78 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb52d0 size=184
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b52d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb52d0;
    loop {
        match pc {
        0xb52d0 => {
            // block b52d0
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b52d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b52d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b52d1 mov rbp,rsp
            rbp = rsp;
            // b52d4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b52d5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b52d6 mov rax,rdi
            rax = rdi;
            // b52d9 cmp rsi,rax
            fl = sub::<64>(rsi, rax).1;
            // b52dc sete BYTE PTR [rsi+0x18]
            m.w8(rsi.wrapping_add(0x18), ((fl.e() as u64)) as u8);
            // b52e0 je 0xb532b
            if fl.e() { pc = 0xb532b; continue; }
            pc = 0xb52e2;
        }
        0xb52e2 => {
            // block b52e2
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b52e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b52e2 mov rbx,QWORD PTR [rsi+0x10]
            rbx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // b52e6 cmp BYTE PTR [rbx+0x18],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // b52ea jne 0xb532b
            if fl.ne() { pc = 0xb532b; continue; }
            pc = 0xb52ec;
        }
        0xb52ec => {
            // block b52ec
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b52ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b52ec mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b52f0 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // b52f3 cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // b52f6 je 0xb5305
            if fl.e() { pc = 0xb5305; continue; }
            pc = 0xb52f8;
        }
        0xb52f8 => {
            // block b52f8
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b52f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b52f8 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b52fb je 0xb5332
            if fl.e() { pc = 0xb5332; continue; }
            pc = 0xb52fd;
        }
        0xb52fd => {
            // block b52fd
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b52fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b52fd cmp BYTE PTR [rcx+0x18],0x0
            fl = sub::<8>(m.r8(rcx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // b5301 je 0xb5314
            if fl.e() { pc = 0xb5314; continue; }
            pc = 0xb5303;
        }
        0xb5303 => {
            // block b5303
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5303 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5303 jmp 0xb5332
            pc = 0xb5332; continue;
        }
        0xb5305 => {
            // block b5305
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5305 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5305 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b5309 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b530c je 0xb535a
            if fl.e() { pc = 0xb535a; continue; }
            pc = 0xb530e;
        }
        0xb530e => {
            // block b530e
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b530e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b530e cmp BYTE PTR [rcx+0x18],0x0
            fl = sub::<8>(m.r8(rcx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // b5312 jne 0xb535a
            if fl.ne() { pc = 0xb535a; continue; }
            pc = 0xb5314;
        }
        0xb5314 => {
            // block b5314
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5314 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5314 mov BYTE PTR [rbx+0x18],0x1
            m.w8(rbx.wrapping_add(0x18), (0x1u64) as u8);
            // b5318 cmp rdi,rax
            fl = sub::<64>(rdi, rax).1;
            // b531b sete BYTE PTR [rdi+0x18]
            m.w8(rdi.wrapping_add(0x18), ((fl.e() as u64)) as u8);
            // b531f mov BYTE PTR [rcx+0x18],0x1
            m.w8(rcx.wrapping_add(0x18), (0x1u64) as u8);
            // b5323 cmp rdi,rax
            fl = sub::<64>(rdi, rax).1;
            // b5326 mov rsi,rdi
            rsi = rdi;
            // b5329 jne 0xb52e2
            if fl.ne() { pc = 0xb52e2; continue; }
            pc = 0xb532b;
        }
        0xb532b => {
            // block b532b
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b532b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b532b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b532f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5330 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5331 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb5332 => {
            // block b5332
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5332 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5332 cmp QWORD PTR [rbx],rsi
            fl = sub::<64>(m.r64(rbx) as u64, rsi).1;
            // b5335 jne 0xb5347
            if fl.ne() { pc = 0xb5347; continue; }
            pc = 0xb5337;
        }
        0xb5337 => {
            // block b5337
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5337 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5337 mov rdi,rbx
            rdi = rbx;
            // b533a call 0xeb86a
            { let r = { let t = m.env.stub_va(0x37); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b533f mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b5343 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xb5347;
        }
        0xb5347 => {
            // block b5347
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5347 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5347 mov BYTE PTR [rbx+0x18],0x1
            m.w8(rbx.wrapping_add(0x18), (0x1u64) as u8);
            // b534b mov BYTE PTR [rdi+0x18],0x0
            m.w8(rdi.wrapping_add(0x18), (0x0u64) as u8);
            // b534f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b5353 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5354 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5355 jmp 0xebaaa
            { let r = { let t = m.env.stub_va(0x277); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xb535a => {
            // block b535a
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b535a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b535a cmp QWORD PTR [rbx],rsi
            fl = sub::<64>(m.r64(rbx) as u64, rsi).1;
            // b535d je 0xb536f
            if fl.e() { pc = 0xb536f; continue; }
            pc = 0xb535f;
        }
        0xb535f => {
            // block b535f
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b535f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b535f mov rdi,rbx
            rdi = rbx;
            // b5362 call 0xebaaa
            { let r = { let t = m.env.stub_va(0x277); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5367 mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b536b mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xb536f;
        }
        0xb536f => {
            // block b536f
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b536f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b536f mov BYTE PTR [rbx+0x18],0x1
            m.w8(rbx.wrapping_add(0x18), (0x1u64) as u8);
            // b5373 mov BYTE PTR [rdi+0x18],0x0
            m.w8(rdi.wrapping_add(0x18), (0x0u64) as u8);
            // b5377 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b537b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b537c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b537d jmp 0xeb86a
            { let r = { let t = m.env.stub_va(0x37); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x13f0 size=186
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000013f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x13f0;
    loop {
        match pc {
        0x13f0 => {
            // block 13f0
            #[cfg(feature = "trace")] eprintln!("f_000013f0 pc 13f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 13f1 mov rbp,rsp
            rbp = rsp;
            // 13f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 13f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 13f8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 13fa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 13fb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 13ff mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 1402 mov r14,rsi
            r14 = rsi;
            // 1405 mov rbx,rdi
            rbx = rdi;
            // 1408 mov DWORD PTR [r14],0x12
            m.w32(r14, (0x12u64) as u32);
            // 140f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1412 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1418 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 141b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 141d cmovne r12d,r15d
            if fl.ne() { r12 = ((r15 as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // 1421 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1424 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // 1427 lea rdx,[rip+0xfdbc2] # 0xfeff0
            rdx = m.env.data_va(79805);
            // 142e mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 1431 mov ecx,0x78
            rcx = (0x78u64) as u32 as u64;
            // 1436 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 143c mov rdi,rbx
            rdi = rbx;
            // 143f mov rdx,r14
            rdx = r14;
            // 1442 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1445 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1448 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 144b je 0x149a
            if fl.e() { pc = 0x149a; continue; }
            pc = 0x144d;
        }
        0x144d => {
            // block 144d
            #[cfg(feature = "trace")] eprintln!("f_000013f0 pc 144d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 144d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 1450 call 0xcb6fa
            { let r = { let t = m.env.code_va(0xcb6fa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1455 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1457 jne 0x1465
            if fl.ne() { pc = 0x1465; continue; }
            pc = 0x1459;
        }
        0x1459 => {
            // block 1459
            #[cfg(feature = "trace")] eprintln!("f_000013f0 pc 1459 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1459 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 145c call 0xcb6a1
            { let r = { let t = m.env.code_va(0xcb6a1); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1461 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1463 je 0x149a
            if fl.e() { pc = 0x149a; continue; }
            pc = 0x1465;
        }
        0x1465 => {
            // block 1465
            #[cfg(feature = "trace")] eprintln!("f_000013f0 pc 1465 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1465 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 146c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 146f lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 1473 mov rdi,rbx
            rdi = rbx;
            // 1476 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 1479 call QWORD PTR [rax+0x470]
            { let r = { let t = m.r64(rax.wrapping_add(0x470)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 147f mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1482 cmp DWORD PTR [rbp-0x24],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64, 0x0u64).1;
            // 1486 je 0x149a
            if fl.e() { pc = 0x149a; continue; }
            pc = 0x1488;
        }
        0x1488 => {
            // block 1488
            #[cfg(feature = "trace")] eprintln!("f_000013f0 pc 1488 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1488 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 148b xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 148d jne 0x149a
            if fl.ne() { pc = 0x149a; continue; }
            pc = 0x148f;
        }
        0x148f => {
            // block 148f
            #[cfg(feature = "trace")] eprintln!("f_000013f0 pc 148f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 148f mov edi,DWORD PTR [r14]
            rdi = (m.r32(r14) as u64) as u32 as u64;
            // 1492 call 0x5a8f7
            { let r = { let t = m.env.code_va(0x5a8f7); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1497 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            pc = 0x149a;
        }
        0x149a => {
            // block 149a
            #[cfg(feature = "trace")] eprintln!("f_000013f0 pc 149a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 149a mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 149d add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 14a1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14a2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 14a4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 14a6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 14a8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14a9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x136d0 size=186
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000136d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x136d0;
    loop {
        match pc {
        0x136d0 => {
            // block 136d0
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 136d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 136d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 136d1 mov rbp,rsp
            rbp = rsp;
            // 136d4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 136d6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 136d8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 136da push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 136dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 136dd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 136de mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 136e0 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // 136e4 mov r13,rdi
            r13 = rdi;
            // 136e7 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 136eb call QWORD PTR [rax+0x58]
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 136ee mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 136f1 cmp BYTE PTR [r13+0x2e8],0x0
            fl = sub::<8>(m.r8(r13.wrapping_add(0x2e8)) as u64, 0x0u64).1;
            // 136f9 mov r14d,ebx
            r14 = ((rbx as u32 as u64)) as u32 as u64;
            // 136fc mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 13701 je 0x1377a
            if fl.e() { pc = 0x1377a; continue; }
            pc = 0x13703;
        }
        0x13703 => {
            // block 13703
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13703 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13703 lea rbx,[r13+0x8]
            rbx = r13.wrapping_add(0x8);
            // 13707 movsxd rsi,DWORD PTR [r13+0x260]
            rsi = sx::<32>(m.r32(r13.wrapping_add(0x260)) as u64) as u64;
            // 1370e xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 13711 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 13713 mov rdi,rbx
            rdi = rbx;
            // 13716 call 0xec044
            { let r = { let t = m.env.stub_va(0x811); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1371b mov esi,DWORD PTR [r13+0x2ec]
            rsi = (m.r32(r13.wrapping_add(0x2ec)) as u64) as u32 as u64;
            pc = 0x13722;
        }
        0x13722 => {
            // block 13722
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13722 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13722 cmp esi,r12d
            fl = sub::<32>((rsi as u32 as u64), (r12 as u32 as u64)).1;
            // 13725 jae 0x13762
            if fl.ae() { pc = 0x13762; continue; }
            pc = 0x13727;
        }
        0x13727 => {
            // block 13727
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13727 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13727 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1372a mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 1372e test BYTE PTR [rax+rbx*1+0x20],0x2
            fl = logic::<8>(m.r8(rax.wrapping_add(rbx).wrapping_add(0x20)) as u64 & 0x2u64);
            // 13733 jne 0x13767
            if fl.ne() { pc = 0x13767; continue; }
            pc = 0x13735;
        }
        0x13735 => {
            // block 13735
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13735 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13735 mov rdi,rbx
            rdi = rbx;
            // 13738 call 0xec03e
            { let r = { let t = m.env.stub_va(0x80b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1373d mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 13741 mov BYTE PTR [rcx+r15*1],al
            m.w8(rcx.wrapping_add(r15), ((rax as u8 as u64)) as u8);
            // 13745 mov esi,DWORD PTR [r13+0x2ec]
            rsi = (m.r32(r13.wrapping_add(0x2ec)) as u64) as u32 as u64;
            // 1374c inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 1374e mov DWORD PTR [r13+0x2ec],esi
            m.w32(r13.wrapping_add(0x2ec), ((rsi as u32 as u64)) as u32);
            // 13755 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 13758 cmp r14d,r15d
            fl = sub::<32>((r14 as u32 as u64), (r15 as u32 as u64)).1;
            // 1375b jne 0x13722
            if fl.ne() { pc = 0x13722; continue; }
            pc = 0x1375d;
        }
        0x1375d => {
            // block 1375d
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 1375d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1375d mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 13760 jmp 0x1377a
            pc = 0x1377a; continue;
        }
        0x13762 => {
            // block 13762
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13762 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 13765 jmp 0x1377a
            pc = 0x1377a; continue;
        }
        0x13767 => {
            // block 13767
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13767 lea rdi,[rip+0x116994] # 0x12a102
            rdi = m.env.data_va(256207);
            // 1376e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 13770 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13775 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            pc = 0x1377a;
        }
        0x1377a => {
            // block 1377a
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 1377a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1377a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1377e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1377f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 13781 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 13783 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 13785 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 13787 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 13788 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8e30 size=188
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008e30<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8e30;
    loop {
        match pc {
        0x8e30 => {
            // block 8e30
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e30 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8e31 mov rbp,rsp
            rbp = rsp;
            // 8e34 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8e36 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8e38 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 8e3a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 8e3c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8e3d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 8e41 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // 8e44 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 8e47 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 8e4a mov r13,rdi
            r13 = rdi;
            // 8e4d mov edi,DWORD PTR [r13+0x10]
            rdi = (m.r32(r13.wrapping_add(0x10)) as u64) as u32 as u64;
            // 8e51 call 0xceac8
            { let r = { let t = m.env.code_va(0xceac8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8e56 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 8e58 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 8e5f mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 8e66 mov BYTE PTR [rbp-0x31],0x1
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), (0x1u64) as u8);
            // 8e6a lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 8e6e lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 8e72 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // 8e75 call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8e7a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8e7c je 0x8edb
            if fl.e() { pc = 0x8edb; continue; }
            pc = 0x8e7e;
        }
        0x8e7e => {
            // block 8e7e
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e7e mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 8e81 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 8e83 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8e85 cmp ecx,ebx
            fl = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64)).1;
            // 8e87 jae 0x8edd
            if fl.ae() { pc = 0x8edd; continue; }
            pc = 0x8e89;
        }
        0x8e89 => {
            // block 8e89
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e89 cmp DWORD PTR [rbp-0x30],0x3
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x3u64).1;
            // 8e8d ja 0x8edd
            if fl.a() { pc = 0x8edd; continue; }
            pc = 0x8e8f;
        }
        0x8e8f => {
            // block 8e8f
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e8f test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 8e92 je 0x8eb1
            if fl.e() { pc = 0x8eb1; continue; }
            pc = 0x8e94;
        }
        0x8e94 => {
            // block 8e94
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e94 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 8e98 lea rcx,[rbp-0x31]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            // 8e9c mov rdi,r13
            rdi = r13;
            // 8e9f mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 8ea2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 8ea5 call QWORD PTR [rax+0x10f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x10f8)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8eab cmp BYTE PTR [rbp-0x31],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf)) as u64, 0x0u64).1;
            // 8eaf je 0x8edb
            if fl.e() { pc = 0x8edb; continue; }
            pc = 0x8eb1;
        }
        0x8eb1 => {
            // block 8eb1
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8eb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8eb1 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 8eb5 mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 8eb8 mov edx,DWORD PTR [rbp-0x30]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 8ebb lea rcx,[rip+0xf67ce] # 0xff690
            rcx = m.env.data_va(81501);
            // 8ec2 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8ec5 lea rdi,[rip+0xf67d4] # 0xff6a0
            rdi = m.env.data_va(81517);
            // 8ecc mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8ed0 mov rdi,r13
            rdi = r13;
            // 8ed3 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 8ed6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8ed9 jmp 0x8edd
            pc = 0x8edd; continue;
        }
        0x8edb => {
            // block 8edb
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8edb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8edb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x8edd;
        }
        0x8edd => {
            // block 8edd
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8edd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8edd add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 8ee1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8ee2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 8ee4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 8ee6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8ee8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8eea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8eeb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5b332 size=188
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005b332<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5b332;
    loop {
        match pc {
        0x5b332 => {
            // block 5b332
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b332 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b332 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5b333 mov rbp,rsp
            rbp = rsp;
            // 5b336 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5b338 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5b33a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5b33b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5b33c mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 5b342 cmp edx,0x11
            fl = sub::<32>((rdx as u32 as u64), 0x11u64).1;
            // 5b345 ja 0x5b355
            if fl.a() { pc = 0x5b355; continue; }
            pc = 0x5b347;
        }
        0x5b347 => {
            // block 5b347
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b347 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b347 movsxd rax,edx
            rax = sx::<32>((rdx as u32 as u64)) as u64;
            // 5b34a lea rdx,[rip+0xa658f] # 0x1018e0
            rdx = m.env.data_va(90285);
            // 5b351 mov r8d,DWORD PTR [rdx+rax*4]
            r8 = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5b355;
        }
        0x5b355 => {
            // block 5b355
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b355 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b355 mov edx,0xffffff93
            rdx = (0xffffff93u64) as u32 as u64;
            // 5b35a mov r9d,0x1
            r9 = (0x1u64) as u32 as u64;
            // 5b360 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5b362 lea r10,[rip+0xa59e7] # 0x100d50
            r10 = m.env.data_va(87325);
            // 5b369 lea r11,[rip+0xa6020] # 0x101390
            r11 = m.env.data_va(88925);
            // 5b370 lea r15,[rip+0xa6209] # 0x101580
            r15 = m.env.data_va(89421);
            // 5b377 movabs r14,0xf8000001f8600f
            r14 = 0xf8_0000_01f8_600fu64;
            pc = 0x5b381;
        }
        0x5b381 => {
            // block 5b381
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b381 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b381 lea ebx,[rdx+0x6d]
            rbx = (rdx.wrapping_add(0x6d)) as u32 as u64;
            // 5b384 cmp ebx,0x1e
            fl = sub::<32>((rbx as u32 as u64), 0x1eu64).1;
            // 5b387 jb 0x5b3b6
            if fl.b() { pc = 0x5b3b6; continue; }
            pc = 0x5b389;
        }
        0x5b389 => {
            // block 5b389
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b389 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b389 cmp edx,0xc
            fl = sub::<32>((rdx as u32 as u64), 0xcu64).1;
            // 5b38c jb 0x5b3b6
            if fl.b() { pc = 0x5b3b6; continue; }
            pc = 0x5b38e;
        }
        0x5b38e => {
            // block 5b38e
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b38e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b38e cmp r9d,0x6d
            fl = sub::<32>((r9 as u32 as u64), 0x6du64).1;
            // 5b392 jg 0x5b3a5
            if fl.g() { pc = 0x5b3a5; continue; }
            pc = 0x5b394;
        }
        0x5b394 => {
            // block 5b394
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b394 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b394 lea ebx,[r9-0xd]
            rbx = (r9.wrapping_add(0xffff_ffff_ffff_fff3)) as u32 as u64;
            // 5b398 cmp ebx,0x37
            fl = sub::<32>((rbx as u32 as u64), 0x37u64).1;
            // 5b39b ja 0x5b3ae
            if fl.a() { pc = 0x5b3ae; continue; }
            pc = 0x5b39d;
        }
        0x5b39d => {
            // block 5b39d
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b39d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b39d bt r14,rbx
            fl.cf = bit::<64>(r14, rbx);
            // 5b3a1 jb 0x5b3b6
            if fl.b() { pc = 0x5b3b6; continue; }
            pc = 0x5b3a3;
        }
        0x5b3a3 => {
            // block 5b3a3
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b3a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b3a3 jmp 0x5b3ae
            pc = 0x5b3ae; continue;
        }
        0x5b3a5 => {
            // block 5b3a5
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b3a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b3a5 lea ebx,[r9-0x6e]
            rbx = (r9.wrapping_add(0xffff_ffff_ffff_ff92)) as u32 as u64;
            // 5b3a9 cmp ebx,0xc
            fl = sub::<32>((rbx as u32 as u64), 0xcu64).1;
            // 5b3ac jb 0x5b3b6
            if fl.b() { pc = 0x5b3b6; continue; }
            pc = 0x5b3ae;
        }
        0x5b3ae => {
            // block 5b3ae
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b3ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b3ae lea ebx,[rdx+0x1e]
            rbx = (rdx.wrapping_add(0x1e)) as u32 as u64;
            // 5b3b1 cmp ebx,0x1d
            fl = sub::<32>((rbx as u32 as u64), 0x1du64).1;
            // 5b3b4 ja 0x5b3c8
            if fl.a() { pc = 0x5b3c8; continue; }
            pc = 0x5b3b6;
        }
        0x5b3b6 => {
            // block 5b3b6
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b3b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b3b6 cmp DWORD PTR [rax+r11*1],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(r11)) as u64, (rcx as u32 as u64)).1;
            // 5b3ba jne 0x5b3c8
            if fl.ne() { pc = 0x5b3c8; continue; }
            pc = 0x5b3bc;
        }
        0x5b3bc => {
            // block 5b3bc
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b3bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b3bc cmp DWORD PTR [rax+r10*1],esi
            fl = sub::<32>(m.r32(rax.wrapping_add(r10)) as u64, (rsi as u32 as u64)).1;
            // 5b3c0 jne 0x5b3c8
            if fl.ne() { pc = 0x5b3c8; continue; }
            pc = 0x5b3c2;
        }
        0x5b3c2 => {
            // block 5b3c2
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b3c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b3c2 cmp r8d,DWORD PTR [rax+r15*1]
            fl = sub::<32>((r8 as u32 as u64), m.r32(rax.wrapping_add(r15)) as u64).1;
            // 5b3c6 je 0x5b3db
            if fl.e() { pc = 0x5b3db; continue; }
            pc = 0x5b3c8;
        }
        0x5b3c8 => {
            // block 5b3c8
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b3c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b3c8 inc r9
            { let (r, f) = inc::<64>(r9, fl); r9 = r; fl = f; }
            // 5b3cb inc edx
            { let (r, f) = inc::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // 5b3cd add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 5b3d1 cmp r9,0x7a
            fl = sub::<64>(r9, 0x7au64).1;
            // 5b3d5 jl 0x5b381
            if fl.l() { pc = 0x5b381; continue; }
            pc = 0x5b3d7;
        }
        0x5b3d7 => {
            // block 5b3d7
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b3d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b3d7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5b3d9 jmp 0x5b3e3
            pc = 0x5b3e3; continue;
        }
        0x5b3db => {
            // block 5b3db
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b3db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b3db mov esi,r9d
            rsi = ((r9 as u32 as u64)) as u32 as u64;
            // 5b3de call 0xc8c60
            { let r = { let t = m.env.code_va(0xc8c60); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5b3e3;
        }
        0x5b3e3 => {
            // block 5b3e3
            #[cfg(feature = "trace")] eprintln!("f_0005b332 pc 5b3e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b3e3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5b3e7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5b3e8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5b3ea pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5b3ec pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5b3ed ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x315f0 size=192
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000315f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x315f0;
    loop {
        match pc {
        0x315f0 => {
            // block 315f0
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 315f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 315f1 mov rbp,rsp
            rbp = rsp;
            // 315f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 315f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 315f8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 315fa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 315fc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 315fd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 315fe mov r13,rdi
            r13 = rdi;
            // 31601 lea r14,[r13+0x18]
            r14 = r13.wrapping_add(0x18);
            // 31605 mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 31609 mov rdi,r14
            rdi = r14;
            // 3160c call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31612 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31614 je 0x3168f
            if fl.e() { pc = 0x3168f; continue; }
            pc = 0x31616;
        }
        0x31616 => {
            // block 31616
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 31616 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31616 mov rdi,r14
            rdi = r14;
            // 31619 call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3161e and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31621 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31623 cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 31628 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // 3162e jne 0x31692
            if fl.ne() { pc = 0x31692; continue; }
            pc = 0x31630;
        }
        0x31630 => {
            // block 31630
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 31630 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31630 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 31637 mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 3163b mov esi,DWORD PTR [r13+0x2a8]
            rsi = (m.r32(r13.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // 31642 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 31646 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3164b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3164e mov rdi,r14
            rdi = r14;
            // 31651 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31654 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 31657 test BYTE PTR [rbp-0x2c],0x8
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & 0x8u64);
            // 3165b jne 0x31692
            if fl.ne() { pc = 0x31692; continue; }
            pc = 0x3165d;
        }
        0x3165d => {
            // block 3165d
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 3165d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3165d lea r12,[rbp-0x2c]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            pc = 0x31661;
        }
        0x31661 => {
            // block 31661
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 31661 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31661 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 31663 cmp ebx,0x2710
            fl = sub::<32>((rbx as u32 as u64), 0x2710u64).1;
            // 31669 ja 0x3168f
            if fl.a() { pc = 0x3168f; continue; }
            pc = 0x3166b;
        }
        0x3166b => {
            // block 3166b
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 3166b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3166b mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 3166f mov esi,DWORD PTR [r13+0x2a8]
            rsi = (m.r32(r13.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // 31676 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3167b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3167e mov rdi,r14
            rdi = r14;
            // 31681 mov rdx,r12
            rdx = r12;
            // 31684 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31687 test BYTE PTR [rbp-0x2c],0x8
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & 0x8u64);
            // 3168b je 0x31661
            if fl.e() { pc = 0x31661; continue; }
            pc = 0x3168d;
        }
        0x3168d => {
            // block 3168d
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 3168d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3168d jmp 0x31692
            pc = 0x31692; continue;
        }
        0x3168f => {
            // block 3168f
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 3168f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3168f xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x31692;
        }
        0x31692 => {
            // block 31692
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 31692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31692 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 31695 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 31699 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3169a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 3169c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 3169e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 316a0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 316a2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 316a3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44402 size=194
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044402<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44402;
    loop {
        match pc {
        0x44402 => {
            // block 44402
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44402 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44403 mov rbp,rsp
            rbp = rsp;
            // 44406 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44408 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4440a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4440b sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4440f mov r14,rcx
            r14 = rcx;
            // 44412 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 44414 cmp ebx,0x3f
            fl = sub::<32>((rbx as u32 as u64), 0x3fu64).1;
            // 44417 ja 0x44474
            if fl.a() { pc = 0x44474; continue; }
            pc = 0x44419;
        }
        0x44419 => {
            // block 44419
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44419 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44419 lea r15,[rbp-0x28]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 4441d mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 44425 mov QWORD PTR [rbp-0x28],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u64);
            // 4442d mov QWORD PTR [rbp-0x30],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r15) as u64);
            // 44431 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 44434 mov rax,QWORD PTR [rax+0x858]
            rax = m.r64(rax.wrapping_add(0x858)) as u64;
            // 4443b lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 4443f call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44441 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 44445 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44447 je 0x44478
            if fl.e() { pc = 0x44478; continue; }
            pc = 0x44449;
        }
        0x44449 => {
            // block 44449
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44449 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44449 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 4444c je 0x44489
            if fl.e() { pc = 0x44489; continue; }
            pc = 0x4444e;
        }
        0x4444e => {
            // block 4444e
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 4444e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4444e mov rcx,r15
            rcx = r15;
            // 44451 mov rdx,rsi
            rdx = rsi;
            pc = 0x44454;
        }
        0x44454 => {
            // block 44454
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44454 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44454 mov rax,rdx
            rax = rdx;
            pc = 0x44457;
        }
        0x44457 => {
            // block 44457
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44457 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44457 cmp DWORD PTR [rax+0x1c],ebx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rbx as u32 as u64)).1;
            // 4445a jge 0x44467
            if fl.ge() { pc = 0x44467; continue; }
            pc = 0x4445c;
        }
        0x4445c => {
            // block 4445c
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 4445c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4445c mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 44460 test rax,rax
            fl = logic::<64>(rax & rax);
            // 44463 jne 0x44457
            if fl.ne() { pc = 0x44457; continue; }
            pc = 0x44465;
        }
        0x44465 => {
            // block 44465
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44465 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44465 jmp 0x4447c
            pc = 0x4447c; continue;
        }
        0x44467 => {
            // block 44467
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44467 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44467 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 4446a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 4446d mov rcx,rax
            rcx = rax;
            // 44470 jne 0x44454
            if fl.ne() { pc = 0x44454; continue; }
            pc = 0x44472;
        }
        0x44472 => {
            // block 44472
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44472 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44472 jmp 0x4447f
            pc = 0x4447f; continue;
        }
        0x44474 => {
            // block 44474
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44474 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44474 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 44476 jmp 0x4449e
            pc = 0x4449e; continue;
        }
        0x44478 => {
            // block 44478
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44478 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44478 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 4447a jmp 0x44495
            pc = 0x44495; continue;
        }
        0x4447c => {
            // block 4447c
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 4447c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4447c mov rax,rcx
            rax = rcx;
            pc = 0x4447f;
        }
        0x4447f => {
            // block 4447f
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 4447f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4447f cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 44482 je 0x44489
            if fl.e() { pc = 0x44489; continue; }
            pc = 0x44484;
        }
        0x44484 => {
            // block 44484
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44484 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44484 cmp DWORD PTR [rax+0x1c],ebx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rbx as u32 as u64)).1;
            // 44487 jle 0x4448c
            if fl.le() { pc = 0x4448c; continue; }
            pc = 0x44489;
        }
        0x44489 => {
            // block 44489
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44489 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44489 mov rax,r15
            rax = r15;
            pc = 0x4448c;
        }
        0x4448c => {
            // block 4448c
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 4448c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4448c cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 4448f setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            // 44493 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x44495;
        }
        0x44495 => {
            // block 44495
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 44495 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44495 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 44499 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4449e;
        }
        0x4449e => {
            // block 4449e
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 4449e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4449e mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 444a0 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 444a4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 444a5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 444a7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 444a9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 444aa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x444ab => {
            // block 444ab
            #[cfg(feature = "trace")] eprintln!("f_00044402 pc 444ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 444ab mov rbx,rax
            rbx = rax;
            // 444ae mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 444b2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 444b6 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 444bb mov rdi,rbx
            rdi = rbx;
            // 444be call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5d4e size=196
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5d4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd5d4e;
    loop {
        match pc {
        0xd5d4e => {
            // block d5d4e
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5d4f mov rbp,rsp
            rbp = rsp;
            // d5d52 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d5d54 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d5d56 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d5d58 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5d59 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d5d5d mov r15,rdx
            r15 = rdx;
            // d5d60 mov rbx,rsi
            rbx = rsi;
            // d5d63 mov r14,rdi
            r14 = rdi;
            // d5d66 mov r12,r15
            r12 = r15;
            // d5d69 sub r12,rbx
            { let (r, f) = sub::<64>(r12, rbx);
              r12 = r;
              fl = f; }
            // d5d6c mov al,BYTE PTR [r14]
            rax = (rax & !0xff) | ((m.r8(r14) as u64) as u8 as u64);
            // d5d6f mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // d5d74 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5d76 je 0xd5d85
            if fl.e() { pc = 0xd5d85; continue; }
            pc = 0xd5d78;
        }
        0xd5d78 => {
            // block d5d78
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d78 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d5d7b mov rsi,rax
            rsi = rax;
            // d5d7e and rsi,0xfffffffffffffffe
            { let r = rsi & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // d5d82 dec rsi
            { let (r, f) = dec::<64>(rsi, fl); rsi = r; fl = f; }
            pc = 0xd5d85;
        }
        0xd5d85 => {
            // block d5d85
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d85 mov rdx,r12
            rdx = r12;
            // d5d88 sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // d5d8b jbe 0xd5db6
            if fl.be() { pc = 0xd5db6; continue; }
            pc = 0xd5d8d;
        }
        0xd5d8d => {
            // block d5d8d
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d8d test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5d8f jne 0xd5d99
            if fl.ne() { pc = 0xd5d99; continue; }
            pc = 0xd5d91;
        }
        0xd5d91 => {
            // block d5d91
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d91 movzx ecx,al
            rcx = ((rax as u8 as u64)) as u32 as u64;
            // d5d94 shr rcx,1
            { let (r, f) = shr::<64>(rcx, 0x1u64, fl); rcx = r; fl = f; }
            // d5d97 jmp 0xd5d9d
            pc = 0xd5d9d; continue;
        }
        0xd5d99 => {
            // block d5d99
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d99 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            pc = 0xd5d9d;
        }
        0xd5d9d => {
            // block d5d9d
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d9d mov QWORD PTR [rsp],0x0
            m.w64(rsp, (0x0u64) as u64);
            // d5da5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5da8 mov rdi,r14
            rdi = r14;
            // d5dab mov r9,rcx
            r9 = rcx;
            // d5dae call 0xec01a
            { let r = { let t = m.env.stub_va(0x7e7); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5db3 mov al,BYTE PTR [r14]
            rax = (rax & !0xff) | ((m.r8(r14) as u64) as u8 as u64);
            pc = 0xd5db6;
        }
        0xd5db6 => {
            // block d5db6
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5db6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5db6 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5db8 jne 0xd5dc0
            if fl.ne() { pc = 0xd5dc0; continue; }
            pc = 0xd5dba;
        }
        0xd5dba => {
            // block d5dba
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dba lea rax,[r14+0x1]
            rax = r14.wrapping_add(0x1);
            // d5dbe jmp 0xd5dc4
            pc = 0xd5dc4; continue;
        }
        0xd5dc0 => {
            // block d5dc0
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dc0 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xd5dc4;
        }
        0xd5dc4 => {
            // block d5dc4
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dc4 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // d5dc7 je 0xd5dde
            if fl.e() { pc = 0xd5dde; continue; }
            pc = 0xd5dc9;
        }
        0xd5dc9 => {
            // block d5dc9
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dc9 mov rcx,rax
            rcx = rax;
            pc = 0xd5dcc;
        }
        0xd5dcc => {
            // block d5dcc
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dcc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dcc mov dl,BYTE PTR [rbx]
            rdx = (rdx & !0xff) | ((m.r8(rbx) as u64) as u8 as u64);
            // d5dce mov BYTE PTR [rcx],dl
            m.w8(rcx, ((rdx as u8 as u64)) as u8);
            // d5dd0 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // d5dd3 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // d5dd6 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // d5dd9 jne 0xd5dcc
            if fl.ne() { pc = 0xd5dcc; continue; }
            pc = 0xd5ddb;
        }
        0xd5ddb => {
            // block d5ddb
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5ddb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ddb add rax,r12
            { let (r, f) = add::<64>(rax, r12);
              rax = r;
              fl = f; }
            pc = 0xd5dde;
        }
        0xd5dde => {
            // block d5dde
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dde mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // d5de1 test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // d5de5 jne 0xd5def
            if fl.ne() { pc = 0xd5def; continue; }
            pc = 0xd5de7;
        }
        0xd5de7 => {
            // block d5de7
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5de7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5de7 add r12,r12
            { let (r, f) = add::<64>(r12, r12);
              r12 = r;
              fl = f; }
            // d5dea mov BYTE PTR [r14],r12b
            m.w8(r14, ((r12 as u8 as u64)) as u8);
            // d5ded jmp 0xd5df3
            pc = 0xd5df3; continue;
        }
        0xd5def => {
            // block d5def
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5def rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5def mov QWORD PTR [r14+0x8],r12
            m.w64(r14.wrapping_add(0x8), (r12) as u64);
            pc = 0xd5df3;
        }
        0xd5df3 => {
            // block d5df3
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5df3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5df3 mov rax,r14
            rax = r14;
            // d5df6 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d5dfa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5dfb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d5dfd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d5dff pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d5e01 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5e02 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x272ae size=199
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000272ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x272ae;
    loop {
        match pc {
        0x272ae => {
            // block 272ae
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 272ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 272ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 272af mov rbp,rsp
            rbp = rsp;
            // 272b2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 272b4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 272b6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 272b8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 272ba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 272bb sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 272bf mov r14,rsi
            r14 = rsi;
            // 272c2 mov rbx,rdi
            rbx = rdi;
            // 272c5 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // 272c8 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 272cc cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 272cf je 0x2734f
            if fl.e() { pc = 0x2734f; continue; }
            pc = 0x272d1;
        }
        0x272d1 => {
            // block 272d1
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 272d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 272d1 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 272d5 jmp 0x272ee
            pc = 0x272ee; continue;
        }
        0x272d7 => {
            // block 272d7
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 272d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 272d7 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 272dc mov rdi,rbx
            rdi = rbx;
            // 272df lea rsi,[rip+0x1031b7] # 0x12a49d
            rsi = m.env.data_va(257130);
            // 272e6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 272eb mov r13,r12
            r13 = r12;
            pc = 0x272ee;
        }
        0x272ee => {
            // block 272ee
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 272ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 272ee mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 272f2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 272f4 mov rdi,r15
            rdi = r15;
            // 272f7 call 0x60974
            { let r = { let t = m.env.code_va(0x60974); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 272fc movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 27300 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 27303 jne 0x2730e
            if fl.ne() { pc = 0x2730e; continue; }
            pc = 0x27305;
        }
        0x27305 => {
            // block 27305
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 27305 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27305 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 27308 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 2730c jmp 0x27316
            pc = 0x27316; continue;
        }
        0x2730e => {
            // block 2730e
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 2730e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2730e mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 27312 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x27316;
        }
        0x27316 => {
            // block 27316
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 27316 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27316 mov rdi,rbx
            rdi = rbx;
            // 27319 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2731e mov rdi,r15
            rdi = r15;
            // 27321 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27326 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 2732a test rax,rax
            fl = logic::<64>(rax & rax);
            // 2732d je 0x2733d
            if fl.e() { pc = 0x2733d; continue; }
            pc = 0x2732f;
        }
        0x2732f => {
            // block 2732f
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 2732f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2732f mov r12,rax
            r12 = rax;
            // 27332 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 27336 test rax,rax
            fl = logic::<64>(rax & rax);
            // 27339 jne 0x2732f
            if fl.ne() { pc = 0x2732f; continue; }
            pc = 0x2733b;
        }
        0x2733b => {
            // block 2733b
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 2733b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2733b jmp 0x2734a
            pc = 0x2734a; continue;
        }
        0x2733d => {
            // block 2733d
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 2733d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2733d mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // 27341 cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // 27345 mov r13,r12
            r13 = r12;
            // 27348 jne 0x2733d
            if fl.ne() { pc = 0x2733d; continue; }
            pc = 0x2734a;
        }
        0x2734a => {
            // block 2734a
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 2734a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2734a cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 2734d jne 0x272d7
            if fl.ne() { pc = 0x272d7; continue; }
            pc = 0x2734f;
        }
        0x2734f => {
            // block 2734f
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 2734f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2734f mov rax,rbx
            rax = rbx;
            // 27352 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 27356 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 27357 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 27359 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2735b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2735d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2735f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 27360 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x27361 => {
            // block 27361
            #[cfg(feature = "trace")] eprintln!("f_000272ae pc 27361 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27361 mov rbx,rax
            rbx = rax;
            // 27364 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 27368 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2736d mov rdi,rbx
            rdi = rbx;
            // 27370 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0828 size=208
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0828<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0828;
    loop {
        match pc {
        0xe0828 => {
            // block e0828
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e0828 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0828 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0829 mov rbp,rsp
            rbp = rsp;
            // e082c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e082e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0830 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e0832 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e0834 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0835 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0836 mov r14,rsi
            r14 = rsi;
            // e0839 mov rbx,rdi
            rbx = rdi;
            // e083c cmp QWORD PTR [rbx+0x28],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x28)) as u64, 0x0u64).1;
            // e0841 jne 0xe08da
            if fl.ne() { pc = 0xe08da; continue; }
            pc = 0xe0847;
        }
        0xe0847 => {
            // block e0847
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e0847 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0847 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e084a mov rdi,rbx
            rdi = rbx;
            // e084d call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0850 mov r15,rax
            r15 = rax;
            // e0853 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // e0857 sub rcx,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rcx, m.r64(r15) as u64);
              rcx = r;
              fl = f; }
            // e085a sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // e085e movabs rax,0x6db6db6db6db6db7
            rax = 0x6db6_db6d_b6db_6db7u64;
            // e0868 imul rax,rcx
            { let (r, f) = imul::<64>(rax, rcx); rax = r; fl = f; }
            // e086c mov DWORD PTR [rbx+0x30],eax
            m.w32(rbx.wrapping_add(0x30), ((rax as u32 as u64)) as u32);
            // e086f test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e0871 jle 0xe08da
            if fl.le() { pc = 0xe08da; continue; }
            pc = 0xe0873;
        }
        0xe0873 => {
            // block e0873
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e0873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0873 mov r13,rax
            r13 = rax;
            // e0876 shl r13,0x20
            { let (r, f) = shl::<64>(r13, 0x20u64, fl); r13 = r; fl = f; }
            // e087a movsxd r12,eax
            r12 = sx::<32>((rax as u32 as u64)) as u64;
            // e087d mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // e0882 mov rax,r12
            rax = r12;
            // e0885 mul rcx
            { let (lo, hi, f) = mul1::<64>(rax, rcx); rax = lo; rdx = hi; fl = f; }
            // e0888 mov rdi,0xffffffffffffffff
            rdi = 0xffff_ffff_ffff_ffffu64;
            // e088f cmovno rdi,rax
            if fl.no() { rdi = rax; }
            // e0893 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0898 mov QWORD PTR [rbx+0x28],rax
            m.w64(rbx.wrapping_add(0x28), (rax) as u64);
            // e089c mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // e089f mov rcx,QWORD PTR [rcx+0x10]
            rcx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // e08a3 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // e08a6 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e08ab mov ecx,0x9
            rcx = (0x9u64) as u32 as u64;
            // e08b0 movabs rdx,0x100000001
            rdx = 0x0_0001_0000_0001u64;
            // e08ba cmp r13,rdx
            fl = sub::<64>(r13, rdx).1;
            // e08bd jl 0xe08da
            if fl.l() { pc = 0xe08da; continue; }
            pc = 0xe08bf;
        }
        0xe08bf => {
            // block e08bf
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e08bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08bf mov rdx,QWORD PTR [rbx+0x28]
            rdx = m.r64(rbx.wrapping_add(0x28)) as u64;
            // e08c3 mov rsi,QWORD PTR [r15]
            rsi = m.r64(r15) as u64;
            // e08c6 mov rsi,QWORD PTR [rsi+rcx*8]
            rsi = m.r64(rsi.wrapping_add(rcx.wrapping_mul(8))) as u64;
            // e08ca mov QWORD PTR [rdx+rax*8],rsi
            m.w64(rdx.wrapping_add(rax.wrapping_mul(8)), (rsi) as u64);
            // e08ce inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e08d1 add rcx,0x7
            { let (r, f) = add::<64>(rcx, 0x7u64);
              rcx = r;
              fl = f; }
            // e08d5 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // e08d8 jl 0xe08bf
            if fl.l() { pc = 0xe08bf; continue; }
            pc = 0xe08da;
        }
        0xe08da => {
            // block e08da
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e08da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08da test r14,r14
            fl = logic::<64>(r14 & r14);
            // e08dd je 0xe08e6
            if fl.e() { pc = 0xe08e6; continue; }
            pc = 0xe08df;
        }
        0xe08df => {
            // block e08df
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e08df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08df mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // e08e3 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            pc = 0xe08e6;
        }
        0xe08e6 => {
            // block e08e6
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e08e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08e6 mov eax,DWORD PTR [rbx+0x30]
            rax = (m.r32(rbx.wrapping_add(0x30)) as u64) as u32 as u64;
            // e08e9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e08ed pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e08ee pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e08f0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e08f2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e08f4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e08f6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e08f7 ret
            return Ret { rax, rdx, xmm0: x0 };
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
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7e5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7e87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7ea4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7ea6 je 0xb7f23
            if fl.e() { pc = 0xb7f23; continue; }
            pc = 0xb7ea8;
        }
        0xb7ea8 => {
            // block b7ea8
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ea8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7eb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7eb2 cmp eax,0x1032294f
            fl = sub::<32>((rax as u32 as u64), 0x1032294fu64).1;
            // b7eb7 jg 0xb7ee9
            if fl.g() { pc = 0xb7ee9; continue; }
            pc = 0xb7eb9;
        }
        0xb7eb9 => {
            // block b7eb9
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7eb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7eb9 cmp eax,0x10266400
            fl = sub::<32>((rax as u32 as u64), 0x10266400u64).1;
            // b7ebe jne 0xb7f09
            if fl.ne() { pc = 0xb7f09; continue; }
            pc = 0xb7ec0;
        }
        0xb7ec0 => {
            // block b7ec0
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ec0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ed2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ed2 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // b7ed7 je 0xb7f10
            if fl.e() { pc = 0xb7f10; continue; }
            pc = 0xb7ed9;
        }
        0xb7ed9 => {
            // block b7ed9
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ed9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ed9 cmp eax,0x10518400
            fl = sub::<32>((rax as u32 as u64), 0x10518400u64).1;
            // b7ede je 0xb7f10
            if fl.e() { pc = 0xb7f10; continue; }
            pc = 0xb7ee0;
        }
        0xb7ee0 => {
            // block b7ee0
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ee0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ee0 cmp eax,0x10518450
            fl = sub::<32>((rax as u32 as u64), 0x10518450u64).1;
            // b7ee5 jne 0xb7f23
            if fl.ne() { pc = 0xb7f23; continue; }
            pc = 0xb7ee7;
        }
        0xb7ee7 => {
            // block b7ee7
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ee7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ee7 jmp 0xb7f10
            pc = 0xb7f10; continue;
        }
        0xb7ee9 => {
            // block b7ee9
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ee9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ee9 cmp eax,0x10322950
            fl = sub::<32>((rax as u32 as u64), 0x10322950u64).1;
            // b7eee je 0xb7f10
            if fl.e() { pc = 0xb7f10; continue; }
            pc = 0xb7ef0;
        }
        0xb7ef0 => {
            // block b7ef0
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ef0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ef0 cmp eax,0x10352300
            fl = sub::<32>((rax as u32 as u64), 0x10352300u64).1;
            // b7ef5 jne 0xb7f23
            if fl.ne() { pc = 0xb7f23; continue; }
            pc = 0xb7ef7;
        }
        0xb7ef7 => {
            // block b7ef7
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ef7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7f09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7f09 cmp eax,0x10294700
            fl = sub::<32>((rax as u32 as u64), 0x10294700u64).1;
            // b7f0e jne 0xb7f23
            if fl.ne() { pc = 0xb7f23; continue; }
            pc = 0xb7f10;
        }
        0xb7f10 => {
            // block b7f10
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7f10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7f23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xd3371 size=213
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3371<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd3371;
    loop {
        match pc {
        0xd3371 => {
            // block d3371
            #[cfg(feature = "trace")] eprintln!("f_000d3371 pc d3371 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3371 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3372 mov rbp,rsp
            rbp = rsp;
            // d3375 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3377 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d3379 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d337b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d337d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d337e sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // d3385 mov r14,r9
            r14 = r9;
            // d3388 mov r15,r8
            r15 = r8;
            // d338b mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // d338d mov r13d,edx
            r13 = ((rdx as u32 as u64)) as u32 as u64;
            // d3390 mov r12,rsi
            r12 = rsi;
            // d3393 mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // d3395 xorps xmm0,xmm0
            x0 = 0;
            // d3398 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d339c movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // d33a0 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // d33a4 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // d33a8 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // d33ac movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // d33b3 movaps XMMWORD PTR [rbp-0xa0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff60), x0);
            // d33ba movaps XMMWORD PTR [rbp-0xb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff50), x0);
            // d33c1 movsxd rdi,ebx
            rdi = sx::<32>((rbx as u32 as u64)) as u64;
            // d33c4 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d33c9 mov cl,dil
            rcx = (rcx & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // d33cc shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // d33ce mov rcx,rdi
            rcx = rdi;
            // d33d1 shr rcx,0x5
            { let (r, f) = shr::<64>(rcx, 0x5u64, fl); rcx = r; fl = f; }
            // d33d5 or DWORD PTR [rbp+rcx*4-0xb0],edx
            { let r = m.r32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff50)) as u64 | (rdx as u32 as u64); let f = logic::<32>(r);
              m.w32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff50), (r) as u32);
              fl = f; }
            // d33dc cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // d33de mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // d33e5 mov DWORD PTR [rbp-0xb8],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (0x0u64) as u32);
            // d33ef inc edi
            { let (r, f) = inc::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // d33f1 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // d33f8 lea r8,[rbp-0xc0]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // d33ff xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d3401 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3403 call 0xec284
            { let r = { let t = m.env.stub_va(0xa51); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3408 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d340a mov eax,0xfffffffe
            rax = (0xfffffffeu64) as u32 as u64;
            // d340f test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3411 je 0xd3434
            if fl.e() { pc = 0xd3434; continue; }
            pc = 0xd3413;
        }
        0xd3413 => {
            // block d3413
            #[cfg(feature = "trace")] eprintln!("f_000d3371 pc d3413 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3413 cmp ecx,0xffffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffffu64).1;
            // d3416 jne 0xd341f
            if fl.ne() { pc = 0xd341f; continue; }
            pc = 0xd3418;
        }
        0xd3418 => {
            // block d3418
            #[cfg(feature = "trace")] eprintln!("f_000d3371 pc d3418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3418 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // d341d jmp 0xd3434
            pc = 0xd3434; continue;
        }
        0xd341f => {
            // block d341f
            #[cfg(feature = "trace")] eprintln!("f_000d3371 pc d341f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d341f movsxd rdx,r13d
            rdx = sx::<32>((r13 as u32 as u64)) as u64;
            // d3422 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3424 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // d3426 mov rsi,r12
            rsi = r12;
            // d3429 mov r8,r15
            r8 = r15;
            // d342c mov r9,r14
            r9 = r14;
            // d342f call 0xec27e
            { let r = { let t = m.env.stub_va(0xa4b); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd3434;
        }
        0xd3434 => {
            // block d3434
            #[cfg(feature = "trace")] eprintln!("f_000d3371 pc d3434 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3434 add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // d343b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d343c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d343e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d3440 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3442 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3444 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3445 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x523cc size=218
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000523cc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x523cc;
    loop {
        match pc {
        0x523cc => {
            // block 523cc
            #[cfg(feature = "trace")] eprintln!("f_000523cc pc 523cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 523cc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 523cd mov rbp,rsp
            rbp = rsp;
            // 523d0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 523d2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 523d3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 523d7 mov QWORD PTR [rip+0x175eb2],rdi # 0x1c8290
            m.w64(m.env.data_va(903773), (rdi) as u64);
            // 523de mov rdi,QWORD PTR [rip+0xedd9b] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 523e5 lea rsi,[rip+0xd8bfa] # 0x12afe6
            rsi = m.env.data_va(260019);
            // 523ec mov edx,0x2b
            rdx = (0x2bu64) as u32 as u64;
            // 523f1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 523f6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 523f9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 523fd mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 52401 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 52404 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 52407 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 5240b mov rsi,QWORD PTR [rip+0x175e7e] # 0x1c8290
            rsi = m.r64(m.env.data_va(903773)) as u64;
            // 52412 mov rdi,rax
            rdi = rax;
            // 52415 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5241a mov rbx,rax
            rbx = rax;
            // 5241d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52420 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 52424 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 52428 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5242b or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5242e mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 52432 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52435 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 52439 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 5243c lea r14,[rbp-0x18]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 52440 mov rdi,r14
            rdi = r14;
            // 52443 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52448 mov rsi,QWORD PTR [rip+0xedd41] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 5244f mov rdi,r14
            rdi = r14;
            // 52452 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52457 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5245a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 5245e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 52463 mov rdi,rax
            rdi = rax;
            // 52466 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52468 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 5246b lea rdi,[rbp-0x18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 5246f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52474 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // 52478 mov rdi,rbx
            rdi = rbx;
            // 5247b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52480 mov rdi,rbx
            rdi = rbx;
            // 52483 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52488 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5248c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5248d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5248f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 52490 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x52491 => {
            // block 52491
            #[cfg(feature = "trace")] eprintln!("f_000523cc pc 52491 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52491 mov rbx,rax
            rbx = rax;
            // 52494 lea rdi,[rbp-0x18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 52498 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5249d mov rdi,rbx
            rdi = rbx;
            // 524a0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4eb0 size=220
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4eb0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4eb0;
    loop {
        match pc {
        0xc4eb0 => {
            // block c4eb0
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4eb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4eb0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4eb1 mov rbp,rsp
            rbp = rsp;
            // c4eb4 lea r9,[rdi+0x8]
            r9 = rdi.wrapping_add(0x8);
            // c4eb8 cmp r9,rsi
            fl = sub::<64>(r9, rsi).1;
            // c4ebb je 0xc4ec8
            if fl.e() { pc = 0xc4ec8; continue; }
            pc = 0xc4ebd;
        }
        0xc4ebd => {
            // block c4ebd
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4ebd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ebd mov r8d,DWORD PTR [rcx]
            r8 = (m.r32(rcx) as u64) as u32 as u64;
            // c4ec0 mov eax,DWORD PTR [rsi+0x1c]
            rax = (m.r32(rsi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c4ec3 cmp r8d,eax
            fl = sub::<32>((r8 as u32 as u64), (rax as u32 as u64)).1;
            // c4ec6 jae 0xc4ee9
            if fl.ae() { pc = 0xc4ee9; continue; }
            pc = 0xc4ec8;
        }
        0xc4ec8 => {
            // block c4ec8
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4ec8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ec8 cmp QWORD PTR [rdi],rsi
            fl = sub::<64>(m.r64(rdi) as u64, rsi).1;
            // c4ecb mov r8,QWORD PTR [rsi]
            r8 = m.r64(rsi) as u64;
            // c4ece mov r9,rsi
            r9 = rsi;
            // c4ed1 je 0xc4f1b
            if fl.e() { pc = 0xc4f1b; continue; }
            pc = 0xc4ed3;
        }
        0xc4ed3 => {
            // block c4ed3
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4ed3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ed3 test r8,r8
            fl = logic::<64>(r8 & r8);
            // c4ed6 je 0xc4f04
            if fl.e() { pc = 0xc4f04; continue; }
            pc = 0xc4ed8;
        }
        0xc4ed8 => {
            // block c4ed8
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4ed8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ed8 mov rax,r8
            rax = r8;
            pc = 0xc4edb;
        }
        0xc4edb => {
            // block c4edb
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4edb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4edb mov r9,rax
            r9 = rax;
            // c4ede mov rax,QWORD PTR [r9+0x8]
            rax = m.r64(r9.wrapping_add(0x8)) as u64;
            // c4ee2 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4ee5 jne 0xc4edb
            if fl.ne() { pc = 0xc4edb; continue; }
            pc = 0xc4ee7;
        }
        0xc4ee7 => {
            // block c4ee7
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4ee7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ee7 jmp 0xc4f13
            pc = 0xc4f13; continue;
        }
        0xc4ee9 => {
            // block c4ee9
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4ee9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ee9 cmp eax,r8d
            fl = sub::<32>((rax as u32 as u64), (r8 as u32 as u64)).1;
            // c4eec jae 0xc4f34
            if fl.ae() { pc = 0xc4f34; continue; }
            pc = 0xc4eee;
        }
        0xc4eee => {
            // block c4eee
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4eee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4eee mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c4ef2 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4ef5 je 0xc4f39
            if fl.e() { pc = 0xc4f39; continue; }
            pc = 0xc4ef7;
        }
        0xc4ef7 => {
            // block c4ef7
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4ef7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ef7 mov r10,rax
            r10 = rax;
            // c4efa mov rax,QWORD PTR [r10]
            rax = m.r64(r10) as u64;
            // c4efd test rax,rax
            fl = logic::<64>(rax & rax);
            // c4f00 jne 0xc4ef7
            if fl.ne() { pc = 0xc4ef7; continue; }
            pc = 0xc4f02;
        }
        0xc4f02 => {
            // block c4f02
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f02 jmp 0xc4f48
            pc = 0xc4f48; continue;
        }
        0xc4f04 => {
            // block c4f04
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f04 mov rax,rsi
            rax = rsi;
            pc = 0xc4f07;
        }
        0xc4f07 => {
            // block c4f07
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f07 mov r9,QWORD PTR [rax+0x10]
            r9 = m.r64(rax.wrapping_add(0x10)) as u64;
            // c4f0b cmp QWORD PTR [r9],rax
            fl = sub::<64>(m.r64(r9) as u64, rax).1;
            // c4f0e mov rax,r9
            rax = r9;
            // c4f11 je 0xc4f07
            if fl.e() { pc = 0xc4f07; continue; }
            pc = 0xc4f13;
        }
        0xc4f13 => {
            // block c4f13
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f13 mov eax,DWORD PTR [r9+0x1c]
            rax = (m.r32(r9.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c4f17 cmp eax,DWORD PTR [rcx]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx) as u64).1;
            // c4f19 jae 0xc4f62
            if fl.ae() { pc = 0xc4f62; continue; }
            pc = 0xc4f1b;
        }
        0xc4f1b => {
            // block c4f1b
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f1b test r8,r8
            fl = logic::<64>(r8 & r8);
            // c4f1e je 0xc4f2c
            if fl.e() { pc = 0xc4f2c; continue; }
            pc = 0xc4f20;
        }
        0xc4f20 => {
            // block c4f20
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f20 mov QWORD PTR [rdx],r9
            m.w64(rdx, (r9) as u64);
            // c4f23 add r9,0x8
            { let (r, f) = add::<64>(r9, 0x8u64);
              r9 = r;
              fl = f; }
            // c4f27 mov rdx,r9
            rdx = r9;
            // c4f2a jmp 0xc4f78
            pc = 0xc4f78; continue;
        }
        0xc4f2c => {
            // block c4f2c
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f2c mov QWORD PTR [rdx],rsi
            m.w64(rdx, (rsi) as u64);
            // c4f2f mov rdx,rsi
            rdx = rsi;
            // c4f32 jmp 0xc4f78
            pc = 0xc4f78; continue;
        }
        0xc4f34 => {
            // block c4f34
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f34 mov QWORD PTR [rdx],rsi
            m.w64(rdx, (rsi) as u64);
            // c4f37 jmp 0xc4f78
            pc = 0xc4f78; continue;
        }
        0xc4f39 => {
            // block c4f39
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f39 mov rax,rsi
            rax = rsi;
            pc = 0xc4f3c;
        }
        0xc4f3c => {
            // block c4f3c
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f3c mov r10,QWORD PTR [rax+0x10]
            r10 = m.r64(rax.wrapping_add(0x10)) as u64;
            // c4f40 cmp QWORD PTR [r10],rax
            fl = sub::<64>(m.r64(r10) as u64, rax).1;
            // c4f43 mov rax,r10
            rax = r10;
            // c4f46 jne 0xc4f3c
            if fl.ne() { pc = 0xc4f3c; continue; }
            pc = 0xc4f48;
        }
        0xc4f48 => {
            // block c4f48
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f48 cmp r10,r9
            fl = sub::<64>(r10, r9).1;
            // c4f4b je 0xc4f53
            if fl.e() { pc = 0xc4f53; continue; }
            pc = 0xc4f4d;
        }
        0xc4f4d => {
            // block c4f4d
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f4d cmp r8d,DWORD PTR [r10+0x1c]
            fl = sub::<32>((r8 as u32 as u64), m.r32(r10.wrapping_add(0x1c)) as u64).1;
            // c4f51 jae 0xc4f62
            if fl.ae() { pc = 0xc4f62; continue; }
            pc = 0xc4f53;
        }
        0xc4f53 => {
            // block c4f53
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f53 cmp QWORD PTR [rsi+0x8],0x0
            fl = sub::<64>(m.r64(rsi.wrapping_add(0x8)) as u64, 0x0u64).1;
            // c4f58 je 0xc4f6e
            if fl.e() { pc = 0xc4f6e; continue; }
            pc = 0xc4f5a;
        }
        0xc4f5a => {
            // block c4f5a
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f5a mov QWORD PTR [rdx],r10
            m.w64(rdx, (r10) as u64);
            // c4f5d mov rdx,r10
            rdx = r10;
            // c4f60 jmp 0xc4f78
            pc = 0xc4f78; continue;
        }
        0xc4f62 => {
            // block c4f62
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f62 mov rsi,rdx
            rsi = rdx;
            // c4f65 mov rdx,rcx
            rdx = rcx;
            // c4f68 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4f69 jmp 0xebd8c
            { let r = { let t = m.env.stub_va(0x559); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc4f6e => {
            // block c4f6e
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f6e mov QWORD PTR [rdx],rsi
            m.w64(rdx, (rsi) as u64);
            // c4f71 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // c4f75 mov rdx,rsi
            rdx = rsi;
            pc = 0xc4f78;
        }
        0xc4f78 => {
            // block c4f78
            #[cfg(feature = "trace")] eprintln!("f_000c4eb0 pc c4f78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4f78 mov rax,rdx
            rax = rdx;
            // c4f7b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4f7c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x103ce size=222
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000103ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x103ce;
    loop {
        match pc {
        0x103ce => {
            // block 103ce
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 103ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 103ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 103cf mov rbp,rsp
            rbp = rsp;
            // 103d2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 103d3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 103d4 mov rbx,rdi
            rbx = rdi;
            // 103d7 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 103db mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 103df mov rsi,QWORD PTR [rbx+0x20]
            rsi = m.r64(rbx.wrapping_add(0x20)) as u64;
            // 103e3 mov rdx,rsi
            rdx = rsi;
            // 103e6 shr rdx,0xa
            { let (r, f) = shr::<64>(rdx, 0xau64, fl); rdx = r; fl = f; }
            // 103ea lea r10,[rax+rdx*8]
            r10 = rax.wrapping_add(rdx.wrapping_mul(8));
            // 103ee sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 103f1 je 0x10424
            if fl.e() { pc = 0x10424; continue; }
            pc = 0x103f3;
        }
        0x103f3 => {
            // block 103f3
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 103f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 103f3 mov rdx,rsi
            rdx = rsi;
            // 103f6 and rdx,0x3ff
            { let r = rdx & 0x3ffu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 103fd shl rdx,0x2
            { let (r, f) = shl::<64>(rdx, 0x2u64, fl); rdx = r; fl = f; }
            // 10401 add rdx,QWORD PTR [r10]
            { let (r, f) = add::<64>(rdx, m.r64(r10) as u64);
              rdx = r;
              fl = f; }
            // 10404 lea r8,[rbx+0x28]
            r8 = rbx.wrapping_add(0x28);
            // 10408 add rsi,QWORD PTR [rbx+0x28]
            { let (r, f) = add::<64>(rsi, m.r64(rbx.wrapping_add(0x28)) as u64);
              rsi = r;
              fl = f; }
            // 1040c mov r9,rsi
            r9 = rsi;
            // 1040f shr r9,0xa
            { let (r, f) = shr::<64>(r9, 0xau64, fl); r9 = r; fl = f; }
            // 10413 and rsi,0x3ff
            { let r = rsi & 0x3ffu64; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 1041a shl rsi,0x2
            { let (r, f) = shl::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 1041e add rsi,QWORD PTR [rax+r9*8]
            { let (r, f) = add::<64>(rsi, m.r64(rax.wrapping_add(r9.wrapping_mul(8))) as u64);
              rsi = r;
              fl = f; }
            // 10422 jmp 0x10436
            pc = 0x10436; continue;
        }
        0x10424 => {
            // block 10424
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10424 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10424 lea r8,[rbx+0x28]
            r8 = rbx.wrapping_add(0x28);
            // 10428 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1042a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 1042c jmp 0x10436
            pc = 0x10436; continue;
        }
        0x1042e => {
            // block 1042e
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 1042e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1042e mov rdx,QWORD PTR [r10+0x8]
            rdx = m.r64(r10.wrapping_add(0x8)) as u64;
            // 10432 add r10,0x8
            { let (r, f) = add::<64>(r10, 0x8u64);
              r10 = r;
              fl = f; }
            pc = 0x10436;
        }
        0x10436 => {
            // block 10436
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10436 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 10439 je 0x10450
            if fl.e() { pc = 0x10450; continue; }
            pc = 0x1043b;
        }
        0x1043b => {
            // block 1043b
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 1043b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1043b add rdx,0x4
            { let (r, f) = add::<64>(rdx, 0x4u64);
              rdx = r;
              fl = f; }
            // 1043f mov rdi,rdx
            rdi = rdx;
            // 10442 sub rdi,QWORD PTR [r10]
            { let (r, f) = sub::<64>(rdi, m.r64(r10) as u64);
              rdi = r;
              fl = f; }
            // 10445 cmp rdi,0x1000
            fl = sub::<64>(rdi, 0x1000u64).1;
            // 1044c jne 0x10436
            if fl.ne() { pc = 0x10436; continue; }
            pc = 0x1044e;
        }
        0x1044e => {
            // block 1044e
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 1044e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1044e jmp 0x1042e
            pc = 0x1042e; continue;
        }
        0x10450 => {
            // block 10450
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10450 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10450 mov QWORD PTR [r8],0x0
            m.w64(r8, (0x0u64) as u64);
            // 10457 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 1045b cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // 1045f jb 0x10486
            if fl.b() { pc = 0x10486; continue; }
            pc = 0x10461;
        }
        0x10461 => {
            // block 10461
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10461 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10461 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // 10464 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10469 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1046d add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 10471 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 10475 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 10479 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 1047c sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 10480 cmp rcx,0x2
            fl = sub::<64>(rcx, 0x2u64).1;
            // 10484 ja 0x10461
            if fl.a() { pc = 0x10461; continue; }
            pc = 0x10486;
        }
        0x10486 => {
            // block 10486
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10486 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10486 cmp rcx,0x2
            fl = sub::<64>(rcx, 0x2u64).1;
            // 1048a jne 0x10496
            if fl.ne() { pc = 0x10496; continue; }
            pc = 0x1048c;
        }
        0x1048c => {
            // block 1048c
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 1048c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1048c mov QWORD PTR [rbx+0x20],0x400
            m.w64(rbx.wrapping_add(0x20), (0x400u64) as u64);
            // 10494 jmp 0x104a4
            pc = 0x104a4; continue;
        }
        0x10496 => {
            // block 10496
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 10496 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10496 cmp rcx,0x1
            fl = sub::<64>(rcx, 0x1u64).1;
            // 1049a jne 0x104a4
            if fl.ne() { pc = 0x104a4; continue; }
            pc = 0x1049c;
        }
        0x1049c => {
            // block 1049c
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 1049c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1049c mov QWORD PTR [rbx+0x20],0x200
            m.w64(rbx.wrapping_add(0x20), (0x200u64) as u64);
            pc = 0x104a4;
        }
        0x104a4 => {
            // block 104a4
            #[cfg(feature = "trace")] eprintln!("f_000103ce pc 104a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 104a4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 104a8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 104a9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 104aa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd0b0 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d0b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd0b0;
    loop {
        match pc {
        0xd0b0 => {
            // block d0b0
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d0b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d0b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d0b1 mov rbp,rsp
            rbp = rsp;
            // d0b4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d0b6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d0b8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d0b9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d0ba mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d0bd mov r15,rdi
            r15 = rdi;
            // d0c0 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // d0c3 je 0xd134
            if fl.e() { pc = 0xd134; continue; }
            pc = 0xd0c5;
        }
        0xd0c5 => {
            // block d0c5
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d0c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d0c5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d0c8 mov rdi,r15
            rdi = r15;
            // d0cb call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d0d1 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // d0d6 je 0xd124
            if fl.e() { pc = 0xd124; continue; }
            pc = 0xd0d8;
        }
        0xd0d8 => {
            // block d0d8
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d0d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d0d8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d0db mov rdi,r15
            rdi = r15;
            // d0de call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d0e4 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // d0e9 je 0xd124
            if fl.e() { pc = 0xd124; continue; }
            pc = 0xd0eb;
        }
        0xd0eb => {
            // block d0eb
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d0eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d0eb mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d0ee mov rdi,r15
            rdi = r15;
            // d0f1 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d0f7 cmp eax,0x10710800
            fl = sub::<32>((rax as u32 as u64), 0x10710800u64).1;
            // d0fc je 0xd124
            if fl.e() { pc = 0xd124; continue; }
            pc = 0xd0fe;
        }
        0xd0fe => {
            // block d0fe
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d0fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d0fe mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d101 mov rdi,r15
            rdi = r15;
            // d104 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d10a cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // d10f je 0xd124
            if fl.e() { pc = 0xd124; continue; }
            pc = 0xd111;
        }
        0xd111 => {
            // block d111
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d111 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d111 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d114 mov rdi,r15
            rdi = r15;
            // d117 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d11d cmp eax,0x10710851
            fl = sub::<32>((rax as u32 as u64), 0x10710851u64).1;
            // d122 jne 0xd134
            if fl.ne() { pc = 0xd134; continue; }
            pc = 0xd124;
        }
        0xd124 => {
            // block d124
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d124 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d124 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d127 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d12b mov rdi,r15
            rdi = r15;
            // d12e call QWORD PTR [rax+0x328]
            { let r = { let t = m.r64(rax.wrapping_add(0x328)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd134;
        }
        0xd134 => {
            // block d134
            #[cfg(feature = "trace")] eprintln!("f_0000d0b0 pc d134 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d134 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d137 mov rdi,r15
            rdi = r15;
            // d13a call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d140 cmp eax,0x10402100
            fl = sub::<32>((rax as u32 as u64), 0x10402100u64).1;
            // d145 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d148 xor al,r14b
            { let r = (rax as u8 as u64) ^ (r14 as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d14b movzx ebx,al
            rbx = ((rax as u8 as u64)) as u32 as u64;
            // d14e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d151 mov esi,0x5f
            rsi = (0x5fu64) as u32 as u64;
            // d156 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // d15b mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // d161 mov rdi,r15
            rdi = r15;
            // d164 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d166 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d169 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d16c xor ebx,0x1
            { let r = (rbx as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d16f mov esi,0x5f
            rsi = (0x5fu64) as u32 as u64;
            // d174 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // d179 mov r8d,0x5
            r8 = (0x5u64) as u32 as u64;
            // d17f mov rdi,r15
            rdi = r15;
            // d182 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d184 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d187 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d18b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d18c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d18e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d190 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d191 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd192 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d192<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd192;
    loop {
        match pc {
        0xd192 => {
            // block d192
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d192 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d192 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d193 mov rbp,rsp
            rbp = rsp;
            // d196 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d198 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d19a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d19b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d19c mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d19f mov r15,rdi
            r15 = rdi;
            // d1a2 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // d1a5 je 0xd216
            if fl.e() { pc = 0xd216; continue; }
            pc = 0xd1a7;
        }
        0xd1a7 => {
            // block d1a7
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1a7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1aa mov rdi,r15
            rdi = r15;
            // d1ad call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1b3 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // d1b8 je 0xd206
            if fl.e() { pc = 0xd206; continue; }
            pc = 0xd1ba;
        }
        0xd1ba => {
            // block d1ba
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ba mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1bd mov rdi,r15
            rdi = r15;
            // d1c0 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1c6 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // d1cb je 0xd206
            if fl.e() { pc = 0xd206; continue; }
            pc = 0xd1cd;
        }
        0xd1cd => {
            // block d1cd
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cd mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1d0 mov rdi,r15
            rdi = r15;
            // d1d3 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1d9 cmp eax,0x10710800
            fl = sub::<32>((rax as u32 as u64), 0x10710800u64).1;
            // d1de je 0xd206
            if fl.e() { pc = 0xd206; continue; }
            pc = 0xd1e0;
        }
        0xd1e0 => {
            // block d1e0
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1e3 mov rdi,r15
            rdi = r15;
            // d1e6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1ec cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // d1f1 je 0xd206
            if fl.e() { pc = 0xd206; continue; }
            pc = 0xd1f3;
        }
        0xd1f3 => {
            // block d1f3
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1f3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1f6 mov rdi,r15
            rdi = r15;
            // d1f9 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1ff cmp eax,0x10710851
            fl = sub::<32>((rax as u32 as u64), 0x10710851u64).1;
            // d204 jne 0xd216
            if fl.ne() { pc = 0xd216; continue; }
            pc = 0xd206;
        }
        0xd206 => {
            // block d206
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d206 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d209 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d20d mov rdi,r15
            rdi = r15;
            // d210 call QWORD PTR [rax+0x328]
            { let r = { let t = m.r64(rax.wrapping_add(0x328)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd216;
        }
        0xd216 => {
            // block d216
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d216 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d216 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d219 mov rdi,r15
            rdi = r15;
            // d21c call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d222 cmp eax,0x10402100
            fl = sub::<32>((rax as u32 as u64), 0x10402100u64).1;
            // d227 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d22a xor al,r14b
            { let r = (rax as u8 as u64) ^ (r14 as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d22d movzx ebx,al
            rbx = ((rax as u8 as u64)) as u32 as u64;
            // d230 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d233 mov esi,0x5f
            rsi = (0x5fu64) as u32 as u64;
            // d238 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // d23d mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // d243 mov rdi,r15
            rdi = r15;
            // d246 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d248 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d24b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d24e xor ebx,0x1
            { let r = (rbx as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d251 mov esi,0x5f
            rsi = (0x5fu64) as u32 as u64;
            // d256 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // d25b mov r8d,0x5
            r8 = (0x5u64) as u32 as u64;
            // d261 mov rdi,r15
            rdi = r15;
            // d264 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d266 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d269 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d26d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d26e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d270 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d272 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d273 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64a17 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064a17<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x64a17;
    loop {
        match pc {
        0x64a17 => {
            // block 64a17
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64a17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a17 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64a18 mov rbp,rsp
            rbp = rsp;
            // 64a1b push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 64a1d push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 64a1f push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 64a21 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 64a22 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 64a26 mov r14,rsi
            r14 = rsi;
            // 64a29 mov r12,rdi
            r12 = rdi;
            // 64a2c mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 64a35 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 64a3e mov QWORD PTR [r12],0x0
            m.w64(r12, (0x0u64) as u64);
            // 64a46 mov rbx,QWORD PTR [r14]
            rbx = m.r64(r14) as u64;
            // 64a49 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 64a4d cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 64a50 je 0x64ad6
            if fl.e() { pc = 0x64ad6; continue; }
            pc = 0x64a56;
        }
        0x64a56 => {
            // block 64a56
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64a56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a56 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 64a58 lea r15,[rbp-0x30]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 64a5c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 64a5e jmp 0x64a6d
            pc = 0x64a6d; continue;
        }
        0x64a60 => {
            // block 64a60
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64a60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a60 mov rax,QWORD PTR [r12+0x8]
            rax = m.r64(r12.wrapping_add(0x8)) as u64;
            // 64a65 mov rdx,QWORD PTR [r12+0x10]
            rdx = m.r64(r12.wrapping_add(0x10)) as u64;
            // 64a6a mov rbx,rcx
            rbx = rcx;
            pc = 0x64a6d;
        }
        0x64a6d => {
            // block 64a6d
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64a6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a6d mov ecx,DWORD PTR [rbx+0x1c]
            rcx = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 64a70 mov DWORD PTR [rbp-0x30],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rcx as u32 as u64)) as u32);
            // 64a73 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 64a7a mov DWORD PTR [rbp-0x28],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0xffffffffu64) as u32);
            // 64a81 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 64a88 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 64a8b jae 0x64aa4
            if fl.ae() { pc = 0x64aa4; continue; }
            pc = 0x64a8d;
        }
        0x64a8d => {
            // block 64a8d
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64a8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a8d mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 64a91 mov rdx,QWORD PTR [rbp-0x28]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 64a95 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 64a99 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 64a9c add QWORD PTR [r12+0x8],0x10
            { let (r, f) = add::<64>(m.r64(r12.wrapping_add(0x8)) as u64, 0x10u64);
              m.w64(r12.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 64aa2 jmp 0x64aaf
            pc = 0x64aaf; continue;
        }
        0x64aa4 => {
            // block 64aa4
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64aa4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64aa4 mov rdi,r12
            rdi = r12;
            // 64aa7 mov rsi,r15
            rsi = r15;
            // 64aaa call 0xebbbe
            { let r = { let t = m.env.stub_va(0x38b); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x64aaf;
        }
        0x64aaf => {
            // block 64aaf
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64aaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64aaf mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 64ab3 test rax,rax
            fl = logic::<64>(rax & rax);
            // 64ab6 je 0x64ac5
            if fl.e() { pc = 0x64ac5; continue; }
            pc = 0x64ab8;
        }
        0x64ab8 => {
            // block 64ab8
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ab8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ab8 mov rcx,rax
            rcx = rax;
            // 64abb mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 64abe test rax,rax
            fl = logic::<64>(rax & rax);
            // 64ac1 jne 0x64ab8
            if fl.ne() { pc = 0x64ab8; continue; }
            pc = 0x64ac3;
        }
        0x64ac3 => {
            // block 64ac3
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ac3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ac3 jmp 0x64ad1
            pc = 0x64ad1; continue;
        }
        0x64ac5 => {
            // block 64ac5
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ac5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ac5 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 64ac9 cmp QWORD PTR [rcx],rbx
            fl = sub::<64>(m.r64(rcx) as u64, rbx).1;
            // 64acc mov rbx,rcx
            rbx = rcx;
            // 64acf jne 0x64ac5
            if fl.ne() { pc = 0x64ac5; continue; }
            pc = 0x64ad1;
        }
        0x64ad1 => {
            // block 64ad1
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ad1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ad1 cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // 64ad4 jne 0x64a60
            if fl.ne() { pc = 0x64a60; continue; }
            pc = 0x64ad6;
        }
        0x64ad6 => {
            // block 64ad6
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ad6 mov rax,r12
            rax = r12;
            // 64ad9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 64add pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64ade pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 64ae0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 64ae2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 64ae4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64ae5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x64ae6 => {
            // block 64ae6
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ae6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ae6 mov r14,rax
            r14 = rax;
            // 64ae9 mov rdi,r12
            rdi = r12;
            // 64aec call 0xebb7c
            { let r = { let t = m.env.stub_va(0x349); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64af1 mov rdi,r14
            rdi = r14;
            // 64af4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 69246 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.env.code_va(0x6911e); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69285 mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 69289 lea r13,[rbp-0x38]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 6928d jmp 0x692d6
            pc = 0x692d6; continue;
        }
        0x6928f => {
            // block 6928f
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 6928f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6928f lea rbx,[r14+0x20]
            rbx = r14.wrapping_add(0x20);
            // 69293 mov rdi,r12
            rdi = r12;
            // 69296 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 69299 mov rdx,rbx
            rdx = rbx;
            // 6929c call 0x68fea
            { let r = { let t = m.env.code_va(0x68fea); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 692a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 692a3 je 0x692b1
            if fl.e() { pc = 0x692b1; continue; }
            pc = 0x692a5;
        }
        0x692a5 => {
            // block 692a5
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 692a5 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 692a9 mov rsi,rbx
            rsi = rbx;
            // 692ac call 0xebca8
            { let r = { let t = m.env.stub_va(0x475); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x692b1;
        }
        0x692b1 => {
            // block 692b1
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 692c5 jmp 0x692d6
            pc = 0x692d6; continue;
        }
        0x692c7 => {
            // block 692c7
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 692c7 mov rax,r14
            rax = r14;
            pc = 0x692ca;
        }
        0x692ca => {
            // block 692ca
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 692d6 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 692d9 jne 0x6928f
            if fl.ne() { pc = 0x6928f; continue; }
            pc = 0x692db;
        }
        0x692db => {
            // block 692db
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 692db mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 692df lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 692e3 call 0xebcc0
            { let r = { let t = m.env.stub_va(0x48d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 692fb mov rbx,rax
            rbx = rax;
            // 692fe mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 69302 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 69306 call 0xebcc0
            { let r = { let t = m.env.stub_va(0x48d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6930b;
        }
        0x6930b => {
            // block 6930b
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 6930b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6930b mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 6930f mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 69312 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 69316 call 0xebcc0
            { let r = { let t = m.env.stub_va(0x48d); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6931b mov rdi,rbx
            rdi = rbx;
            // 6931e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69323 mov rbx,rax
            rbx = rax;
            // 69326 jmp 0x6930b
            pc = 0x6930b; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbaa18 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000baa18<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbaa18;
    loop {
        match pc {
        0xbaa18 => {
            // block baa18
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baa18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baa18 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // baa19 mov rbp,rsp
            rbp = rsp;
            // baa1c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // baa1e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // baa20 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // baa22 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // baa24 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // baa25 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // baa29 mov rbx,rdx
            rbx = rdx;
            // baa2c mov r14,rsi
            r14 = rsi;
            // baa2f mov r15,rdi
            r15 = rdi;
            // baa32 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // baa35 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baa38 mov r12,QWORD PTR [r14]
            r12 = m.r64(r14) as u64;
            // baa3b add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // baa3f cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // baa42 je 0xbaae8
            if fl.e() { pc = 0xbaae8; continue; }
            pc = 0xbaa48;
        }
        0xbaa48 => {
            // block baa48
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baa48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baa48 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // baa4c lea r13,[rbp-0x30]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0xbaa50;
        }
        0xbaa50 => {
            // block baa50
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baa50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baa50 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // baa57 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // baa5e mov edi,DWORD PTR [r12+0x1c]
            rdi = (m.r32(r12.wrapping_add(0x1c)) as u64) as u32 as u64;
            // baa63 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // baa67 mov rdx,r13
            rdx = r13;
            // baa6a call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baa6f mov edi,DWORD PTR [rbp-0x2c]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // baa72 mov rsi,rbx
            rsi = rbx;
            // baa75 call 0x2ca4c
            { let r = { let t = m.env.code_va(0x2ca4c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baa7a cmp rax,QWORD PTR [rbx+0x8]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x8)) as u64).1;
            // baa7e je 0xbaab7
            if fl.e() { pc = 0xbaab7; continue; }
            pc = 0xbaa80;
        }
        0xbaa80 => {
            // block baa80
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baa80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baa80 mov ecx,DWORD PTR [rbp-0x30]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // baa83 lea rdx,[rip+0x47f76] # 0x102a00
            rdx = m.env.data_va(94669);
            // baa8a mov edx,DWORD PTR [rdx+rcx*4]
            rdx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // baa8d and edx,DWORD PTR [rax+0x4]
            { let r = (rdx as u32 as u64) & m.r32(rax.wrapping_add(0x4)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // baa90 lea rax,[rip+0x47f79] # 0x102a10
            rax = m.env.data_va(94685);
            // baa97 mov cl,BYTE PTR [rax+rcx*4]
            rcx = (rcx & !0xff) | ((m.r8(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u8 as u64);
            // baa9a shr edx,cl
            { let (r, f) = shr::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // baa9c test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // baa9e je 0xbaab7
            if fl.e() { pc = 0xbaab7; continue; }
            pc = 0xbaaa0;
        }
        0xbaaa0 => {
            // block baaa0
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baaa0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baaa0 mov eax,DWORD PTR [r12+0x1c]
            rax = (m.r32(r12.wrapping_add(0x1c)) as u64) as u32 as u64;
            // baaa5 mov DWORD PTR [rbp-0x38],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rax as u32 as u64)) as u32);
            // baaa8 mov DWORD PTR [rbp-0x34],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rdx as u32 as u64)) as u32);
            // baaab mov rdi,r15
            rdi = r15;
            // baaae lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // baab2 call 0xebe04
            { let r = { let t = m.env.stub_va(0x5d1); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbaab7;
        }
        0xbaab7 => {
            // block baab7
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baab7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baab7 mov rax,QWORD PTR [r12+0x8]
            rax = m.r64(r12.wrapping_add(0x8)) as u64;
            // baabc test rax,rax
            fl = logic::<64>(rax & rax);
            // baabf je 0xbaacf
            if fl.e() { pc = 0xbaacf; continue; }
            pc = 0xbaac1;
        }
        0xbaac1 => {
            // block baac1
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baac1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baac1 mov r12,rax
            r12 = rax;
            // baac4 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // baac8 test rax,rax
            fl = logic::<64>(rax & rax);
            // baacb jne 0xbaac1
            if fl.ne() { pc = 0xbaac1; continue; }
            pc = 0xbaacd;
        }
        0xbaacd => {
            // block baacd
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baacd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baacd jmp 0xbaadf
            pc = 0xbaadf; continue;
        }
        0xbaacf => {
            // block baacf
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baacf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baacf mov rax,r12
            rax = r12;
            pc = 0xbaad2;
        }
        0xbaad2 => {
            // block baad2
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baad2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baad2 mov r12,QWORD PTR [rax+0x10]
            r12 = m.r64(rax.wrapping_add(0x10)) as u64;
            // baad6 cmp QWORD PTR [r12],rax
            fl = sub::<64>(m.r64(r12) as u64, rax).1;
            // baada mov rax,r12
            rax = r12;
            // baadd jne 0xbaad2
            if fl.ne() { pc = 0xbaad2; continue; }
            pc = 0xbaadf;
        }
        0xbaadf => {
            // block baadf
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baadf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baadf cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // baae2 jne 0xbaa50
            if fl.ne() { pc = 0xbaa50; continue; }
            pc = 0xbaae8;
        }
        0xbaae8 => {
            // block baae8
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baae8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baae8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // baaea add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // baaee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // baaef pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // baaf1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // baaf3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // baaf5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // baaf7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // baaf8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x616c size=228
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000616c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x616c;
    loop {
        match pc {
        0x616c => {
            // block 616c
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 616c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 616c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 616d mov rbp,rsp
            rbp = rsp;
            // 6170 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6172 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6174 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6175 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6176 mov r14,rdx
            r14 = rdx;
            // 6179 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 617b mov r15,rdi
            r15 = rdi;
            // 617e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6181 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6187 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6189 jne 0x61b6
            if fl.ne() { pc = 0x61b6; continue; }
            pc = 0x618b;
        }
        0x618b => {
            // block 618b
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 618b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 618b mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 618f call 0xce76c
            { let r = { let t = m.env.code_va(0xce76c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6194 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 6197 jne 0x61af
            if fl.ne() { pc = 0x61af; continue; }
            pc = 0x6199;
        }
        0x6199 => {
            // block 6199
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 6199 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6199 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 619c mov rdi,r15
            rdi = r15;
            // 619f mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 61a1 mov rdx,r14
            rdx = r14;
            // 61a4 call QWORD PTR [rax+0xf48]
            { let r = { let t = m.r64(rax.wrapping_add(0xf48)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 61aa jmp 0x622f
            pc = 0x622f; continue;
        }
        0x61af => {
            // block 61af
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61af mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 61b1 cmp ebx,0x4
            fl = sub::<32>((rbx as u32 as u64), 0x4u64).1;
            // 61b4 jbe 0x61ba
            if fl.be() { pc = 0x61ba; continue; }
            pc = 0x61b6;
        }
        0x61b6 => {
            // block 61b6
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61b6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 61b8 jmp 0x622f
            pc = 0x622f; continue;
        }
        0x61ba => {
            // block 61ba
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61ba lea rcx,[rip+0x7b] # 0x623c
            rcx = m.env.code_va(0x623c);
            // 61c1 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 61c5 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 61c8 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x61ca => {
            // block 61ca
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61ca mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 61cd mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 61d2 jmp 0x61dc
            pc = 0x61dc; continue;
        }
        0x61d4 => {
            // block 61d4
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61d4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 61d7 mov esi,0x45
            rsi = (0x45u64) as u32 as u64;
            pc = 0x61dc;
        }
        0x61dc => {
            // block 61dc
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61dc mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 61e1 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 61e7 jmp 0x6226
            pc = 0x6226; continue;
        }
        0x61e9 => {
            // block 61e9
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61e9 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 61ec mov esi,0x45
            rsi = (0x45u64) as u32 as u64;
            // 61f1 mov ecx,0x40000000
            rcx = (0x40000000u64) as u32 as u64;
            // 61f6 mov r8d,0x1e
            r8 = (0x1eu64) as u32 as u64;
            // 61fc jmp 0x6226
            pc = 0x6226; continue;
        }
        0x61fe => {
            // block 61fe
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 61fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61fe mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6201 mov esi,0x45
            rsi = (0x45u64) as u32 as u64;
            // 6206 mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 620b mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 6211 jmp 0x6226
            pc = 0x6226; continue;
        }
        0x6213 => {
            // block 6213
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 6213 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6213 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6216 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 621b mov ecx,0x200000
            rcx = (0x200000u64) as u32 as u64;
            // 6220 mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            pc = 0x6226;
        }
        0x6226 => {
            // block 6226
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 6226 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6226 mov rdi,r15
            rdi = r15;
            // 6229 mov rdx,r14
            rdx = r14;
            // 622c call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x622f;
        }
        0x622f => {
            // block 622f
            #[cfg(feature = "trace")] eprintln!("f_0000616c pc 622f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 622f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6233 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6234 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6236 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6238 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6239 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8a33 size=231
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8a33<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc8a33;
    loop {
        match pc {
        0xc8a33 => {
            // block c8a33
            #[cfg(feature = "trace")] eprintln!("f_000c8a33 pc c8a33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8a33 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8a34 mov rbp,rsp
            rbp = rsp;
            // c8a37 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c8a39 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c8a3b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c8a3d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c8a3e imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // c8a42 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // c8a44 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c8a45 idiv r8d
            { let (q, r) = idiv::<32>(rdx, rax, (r8 as u32 as u64)); rax = q; rdx = r; }
            // c8a48 mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // c8a4b imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8a4f mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c8a51 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c8a52 idiv r8d
            { let (q, r) = idiv::<32>(rdx, rax, (r8 as u32 as u64)); rax = q; rdx = r; }
            // c8a55 mov r10d,eax
            r10 = ((rax as u32 as u64)) as u32 as u64;
            // c8a58 cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // c8a5b jge 0xc8b11
            if fl.ge() { pc = 0xc8b11; continue; }
            pc = 0xc8a61;
        }
        0xc8a61 => {
            // block c8a61
            #[cfg(feature = "trace")] eprintln!("f_000c8a33 pc c8a61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8a61 movzx ecx,WORD PTR [rbp+0x10]
            rcx = (m.r16(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // c8a65 shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // c8a69 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // c8a6c cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c8a6d idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // c8a70 sub r10d,r11d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // c8a73 movsxd rdx,r11d
            rdx = sx::<32>((r11 as u32 as u64)) as u64;
            // c8a76 imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            // c8a7a movsx r9,cx
            r9 = sx::<16>((rcx as u16 as u64)) as u64;
            // c8a7e imul rdx,r9
            { let (r, f) = imul::<64>(rdx, r9); rdx = r; fl = f; }
            // c8a82 lea r15,[rsi+rdx*2]
            r15 = rsi.wrapping_add(rdx.wrapping_mul(2));
            // c8a86 add r9,r9
            { let (r, f) = add::<64>(r9, r9);
              r9 = r;
              fl = f; }
            // c8a89 lea r14,[rip+0x80930] # 0x1493c0
            r14 = m.env.data_va(383885);
            // c8a90 mov r8w,0x8001
            r8 = (r8 & !0xffff) | ((0x8001u64) as u16 as u64);
            pc = 0xc8a95;
        }
        0xc8a95 => {
            // block c8a95
            #[cfg(feature = "trace")] eprintln!("f_000c8a33 pc c8a95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8a95 mov ecx,r11d
            rcx = ((r11 as u32 as u64)) as u32 as u64;
            // c8a98 sar ecx,0x10
            { let (r, f) = sar::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c8a9b mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // c8a9e shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c8aa1 and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c8aa4 movsxd r12,ecx
            r12 = sx::<32>((rcx as u32 as u64)) as u64;
            // c8aa7 movsx esi,WORD PTR [rdi+r12*2-0x2]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // c8aad mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // c8ab2 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8ab4 imul esi,DWORD PTR [r14+rdx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r14.wrapping_add(rdx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // c8ab9 movsx edx,WORD PTR [rdi+r12*2]
            rdx = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // c8abe mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // c8ac3 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8ac5 imul edx,DWORD PTR [r14+rcx*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r14.wrapping_add(rcx.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // c8aca add edx,esi
            { let (r, f) = add::<32>((rdx as u32 as u64), (rsi as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8acc movsx ecx,WORD PTR [rdi+r12*2+0x2]
            rcx = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // c8ad2 mov esi,0x60
            rsi = (0x60u64) as u32 as u64;
            // c8ad7 sub esi,ebx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // c8ad9 imul ecx,DWORD PTR [r14+rsi*4]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(r14.wrapping_add(rsi.wrapping_mul(4))) as u64); rcx = (r) as u32 as u64; fl = f; }
            // c8ade add ecx,edx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rdx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8ae0 movsx edx,WORD PTR [rdi+r12*2+0x4]
            rdx = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // c8ae6 mov esi,0x80
            rsi = (0x80u64) as u32 as u64;
            // c8aeb sub esi,ebx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // c8aed imul edx,DWORD PTR [r14+rsi*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r14.wrapping_add(rsi.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // c8af2 add edx,ecx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8af4 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // c8af7 cmp edx,0xffff8001
            fl = sub::<32>((rdx as u32 as u64), 0xffff8001u64).1;
            // c8afd cmovl dx,r8w
            if fl.l() { rdx = (rdx & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            // c8b02 mov WORD PTR [r15],dx
            m.w16(r15, ((rdx as u16 as u64)) as u16);
            // c8b06 add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c8b09 add r15,r9
            { let (r, f) = add::<64>(r15, r9);
              r15 = r;
              fl = f; }
            // c8b0c dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // c8b0f jne 0xc8a95
            if fl.ne() { pc = 0xc8a95; continue; }
            pc = 0xc8b11;
        }
        0xc8b11 => {
            // block c8b11
            #[cfg(feature = "trace")] eprintln!("f_000c8a33 pc c8b11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8b11 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c8b12 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c8b14 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8b16 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c8b18 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8b19 ret
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
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d31 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d33 jne 0x5ddf
            if fl.ne() { pc = 0x5ddf; continue; }
            pc = 0x5d39;
        }
        0x5d39 => {
            // block 5d39
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d39 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d3c lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 5d40 mov rdi,rbx
            rdi = rbx;
            // 5d43 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 5d46 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d4c mov r12d,DWORD PTR [rbp-0x24]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 5d50 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d53 mov rdi,rbx
            rdi = rbx;
            // 5d56 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d5c cmp r12d,eax
            fl = sub::<32>((r12 as u32 as u64), (rax as u32 as u64)).1;
            // 5d5f jbe 0x5d68
            if fl.be() { pc = 0x5d68; continue; }
            pc = 0x5d61;
        }
        0x5d61 => {
            // block 5d61
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d61 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            pc = 0x5d68;
        }
        0x5d68 => {
            // block 5d68
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d68 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d6b mov rdi,rbx
            rdi = rbx;
            // 5d6e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d74 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5d76 call 0xcdd26
            { let r = { let t = m.env.code_va(0xcdd26); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d7b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d7d je 0x5db7
            if fl.e() { pc = 0x5db7; continue; }
            pc = 0x5d7f;
        }
        0x5d7f => {
            // block 5d7f
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d8b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d8e mov rdi,rbx
            rdi = rbx;
            // 5d91 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d97 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d99 je 0x5ddf
            if fl.e() { pc = 0x5ddf; continue; }
            pc = 0x5d9b;
        }
        0x5d9b => {
            // block 5d9b
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d9b mov rax,QWORD PTR [rbx+0x210]
            rax = m.r64(rbx.wrapping_add(0x210)) as u64;
            pc = 0x5da2;
        }
        0x5da2 => {
            // block 5da2
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5da2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5db7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5db7 cmp QWORD PTR [rbx+0x218],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x218)) as u64, 0x0u64).1;
            // 5dbf jne 0x5dd1
            if fl.ne() { pc = 0x5dd1; continue; }
            pc = 0x5dc1;
        }
        0x5dc1 => {
            // block 5dc1
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dc1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5dc4 mov rdi,rbx
            rdi = rbx;
            // 5dc7 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5dcd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5dcf je 0x5ddf
            if fl.e() { pc = 0x5ddf; continue; }
            pc = 0x5dd1;
        }
        0x5dd1 => {
            // block 5dd1
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dd1 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 5dd4 je 0x5de3
            if fl.e() { pc = 0x5de3; continue; }
            pc = 0x5dd6;
        }
        0x5dd6 => {
            // block 5dd6
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dd6 mov rax,QWORD PTR [rbx+0x220]
            rax = m.r64(rbx.wrapping_add(0x220)) as u64;
            // 5ddd jmp 0x5dea
            pc = 0x5dea; continue;
        }
        0x5ddf => {
            // block 5ddf
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5ddf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ddf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5de1 jmp 0x5def
            pc = 0x5def; continue;
        }
        0x5de3 => {
            // block 5de3
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5de3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5de3 mov rax,QWORD PTR [rbx+0x218]
            rax = m.r64(rbx.wrapping_add(0x218)) as u64;
            pc = 0x5dea;
        }
        0x5dea => {
            // block 5dea
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dea mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            pc = 0x5ded;
        }
        0x5ded => {
            // block 5ded
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5ded rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ded mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x5def;
        }
        0x5def => {
            // block 5def
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5def rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xb799c size=236
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000b799c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb799c;
    loop {
        match pc {
        0xb799c => {
            // block b799c
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b799c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b799c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b799d mov rbp,rsp
            rbp = rsp;
            // b79a0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b79a2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b79a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b79a5 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b79a9 mov rax,rsi
            rax = rsi;
            // b79ac mov r14,rdi
            r14 = rdi;
            // b79af mov r15,QWORD PTR [rip+0x88a22] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // b79b6 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // b79b9 mov QWORD PTR [rbp-0x20],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (r15) as u64);
            // b79bd mov rcx,rax
            rcx = rax;
            // b79c0 shr rcx,0x20
            { let (r, f) = shr::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // b79c4 mov BYTE PTR [rbp-0x29],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd7), ((rax as u8 as u64)) as u8);
            // b79c7 mov BYTE PTR [rbp-0x28],ah
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (((rax >> 8) as u8 as u64)) as u8);
            // b79ca mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // b79cc shr edx,0x10
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b79cf mov BYTE PTR [rbp-0x27],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd9), ((rdx as u8 as u64)) as u8);
            // b79d2 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // b79d4 shr edx,0x18
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x18u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b79d7 mov BYTE PTR [rbp-0x26],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffda), ((rdx as u8 as u64)) as u8);
            // b79da mov BYTE PTR [rbp-0x25],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), ((rcx as u8 as u64)) as u8);
            // b79dd mov rcx,rax
            rcx = rax;
            // b79e0 shr rcx,0x28
            { let (r, f) = shr::<64>(rcx, 0x28u64, fl); rcx = r; fl = f; }
            // b79e4 mov BYTE PTR [rbp-0x24],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rcx as u8 as u64)) as u8);
            // b79e7 mov rcx,rax
            rcx = rax;
            // b79ea shr rcx,0x30
            { let (r, f) = shr::<64>(rcx, 0x30u64, fl); rcx = r; fl = f; }
            // b79ee mov BYTE PTR [rbp-0x23],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdd), ((rcx as u8 as u64)) as u8);
            // b79f1 mov rcx,rax
            rcx = rax;
            // b79f4 shr rcx,0x38
            { let (r, f) = shr::<64>(rcx, 0x38u64, fl); rcx = r; fl = f; }
            // b79f8 mov BYTE PTR [rbp-0x22],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffde), ((rcx as u8 as u64)) as u8);
            // b79fb mov BYTE PTR [rbp-0x21],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf), (0x0u64) as u8);
            // b79ff test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7a01 je 0xb7a5d
            if fl.e() { pc = 0xb7a5d; continue; }
            pc = 0xb7a03;
        }
        0xb7a03 => {
            // block b7a03
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a03 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xb7a05;
        }
        0xb7a05 => {
            // block b7a05
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a05 mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b7a07 and dl,0xdf
            { let r = (rdx as u8 as u64) & 0xdfu64; let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7a0a add dl,0xbf
            { let (r, f) = add::<8>((rdx as u8 as u64), 0xbfu64);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7a0d movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // b7a10 cmp edx,0x1a
            fl = sub::<32>((rdx as u32 as u64), 0x1au64).1;
            // b7a13 jb 0xb7a2f
            if fl.b() { pc = 0xb7a2f; continue; }
            pc = 0xb7a15;
        }
        0xb7a15 => {
            // block b7a15
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a15 mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b7a17 add dl,0xd0
            { let (r, f) = add::<8>((rdx as u8 as u64), 0xd0u64);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7a1a movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // b7a1d cmp edx,0xa
            fl = sub::<32>((rdx as u32 as u64), 0xau64).1;
            // b7a20 jb 0xb7a2f
            if fl.b() { pc = 0xb7a2f; continue; }
            pc = 0xb7a22;
        }
        0xb7a22 => {
            // block b7a22
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a22 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b7a25 cmp eax,0x20
            fl = sub::<32>((rax as u32 as u64), 0x20u64).1;
            // b7a28 je 0xb7a2f
            if fl.e() { pc = 0xb7a2f; continue; }
            pc = 0xb7a2a;
        }
        0xb7a2a => {
            // block b7a2a
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a2a cmp eax,0x2d
            fl = sub::<32>((rax as u32 as u64), 0x2du64).1;
            // b7a2d jne 0xb7a5d
            if fl.ne() { pc = 0xb7a5d; continue; }
            pc = 0xb7a2f;
        }
        0xb7a2f => {
            // block b7a2f
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a2f lea eax,[rcx+0x1]
            rax = (rcx.wrapping_add(0x1)) as u32 as u64;
            // b7a32 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // b7a35 ja 0xb7a46
            if fl.a() { pc = 0xb7a46; continue; }
            pc = 0xb7a37;
        }
        0xb7a37 => {
            // block b7a37
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a37 mov al,BYTE PTR [rbp+rcx*1-0x28]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(rcx).wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u8 as u64);
            // b7a3b inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // b7a3e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7a40 jne 0xb7a05
            if fl.ne() { pc = 0xb7a05; continue; }
            pc = 0xb7a42;
        }
        0xb7a42 => {
            // block b7a42
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a42 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // b7a44 je 0xb7a5d
            if fl.e() { pc = 0xb7a5d; continue; }
            pc = 0xb7a46;
        }
        0xb7a46 => {
            // block b7a46
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a46 lea rbx,[rbp-0x29]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // b7a4a mov rdi,rbx
            rdi = rbx;
            // b7a4d call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7a52 mov rdi,r14
            rdi = r14;
            // b7a55 mov rsi,rbx
            rsi = rbx;
            // b7a58 mov rdx,rax
            rdx = rax;
            // b7a5b jmp 0xb7a69
            pc = 0xb7a69; continue;
        }
        0xb7a5d => {
            // block b7a5d
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a5d lea rsi,[rip+0x81264] # 0x138cc8
            rsi = m.env.data_va(316565);
            // b7a64 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b7a66 mov rdi,r14
            rdi = r14;
            pc = 0xb7a69;
        }
        0xb7a69 => {
            // block b7a69
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a69 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7a6e cmp r15,QWORD PTR [rbp-0x20]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // b7a72 jne 0xb7a82
            if fl.ne() { pc = 0xb7a82; continue; }
            pc = 0xb7a74;
        }
        0xb7a74 => {
            // block b7a74
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a74 mov rax,r14
            rax = r14;
            // b7a77 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b7a7b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7a7c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7a7e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b7a80 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7a81 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb7a82 => {
            // block b7a82
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a82 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x63e43 size=238
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00063e43<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x63e43;
    loop {
        match pc {
        0x63e43 => {
            // block 63e43
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63e43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e43 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 63e44 mov rbp,rsp
            rbp = rsp;
            // 63e47 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 63e49 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 63e4b push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 63e4d push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 63e4f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 63e50 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 63e54 mov r12,rsi
            r12 = rsi;
            // 63e57 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 63e5b mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 63e5f lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // 63e64 cmp rcx,r13
            fl = sub::<64>(rcx, r13).1;
            // 63e67 je 0x63f0a
            if fl.e() { pc = 0x63f0a; continue; }
            pc = 0x63e6d;
        }
        0x63e6d => {
            // block 63e6d
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63e6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e6d lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 63e71 mov r14,rcx
            r14 = rcx;
            // 63e74 jmp 0x63e7d
            pc = 0x63e7d; continue;
        }
        0x63e76 => {
            // block 63e76
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63e76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e76 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 63e7a mov r14,rax
            r14 = rax;
            pc = 0x63e7d;
        }
        0x63e7d => {
            // block 63e7d
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63e7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e7d cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // 63e80 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 63e83 lea rsi,[rip+0xd4e3e] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 63e8a lea rcx,[rip+0xc660c] # 0x12a49d
            rcx = m.env.data_va(257130);
            // 63e91 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // 63e95 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 63e98 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 63e9b mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 63e9f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63ea4 mov rbx,rax
            rbx = rax;
            // 63ea7 mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 63eab xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 63ead mov rdi,r15
            rdi = r15;
            // 63eb0 call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63eb5 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 63eb9 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 63ebc jne 0x63ec7
            if fl.ne() { pc = 0x63ec7; continue; }
            pc = 0x63ebe;
        }
        0x63ebe => {
            // block 63ebe
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ebe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ebe shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 63ec1 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 63ec5 jmp 0x63ecf
            pc = 0x63ecf; continue;
        }
        0x63ec7 => {
            // block 63ec7
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ec7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ec7 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 63ecb mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x63ecf;
        }
        0x63ecf => {
            // block 63ecf
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ecf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ecf mov rdi,rbx
            rdi = rbx;
            // 63ed2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63ed7 mov rdi,r15
            rdi = r15;
            // 63eda call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63edf mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 63ee3 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 63ee6 je 0x63ef5
            if fl.e() { pc = 0x63ef5; continue; }
            pc = 0x63ee8;
        }
        0x63ee8 => {
            // block 63ee8
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ee8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ee8 mov rax,rcx
            rax = rcx;
            // 63eeb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 63eee test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 63ef1 jne 0x63ee8
            if fl.ne() { pc = 0x63ee8; continue; }
            pc = 0x63ef3;
        }
        0x63ef3 => {
            // block 63ef3
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ef3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ef3 jmp 0x63f01
            pc = 0x63f01; continue;
        }
        0x63ef5 => {
            // block 63ef5
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63ef5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ef5 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 63ef9 cmp QWORD PTR [rax],r14
            fl = sub::<64>(m.r64(rax) as u64, r14).1;
            // 63efc mov r14,rax
            r14 = rax;
            // 63eff jne 0x63ef5
            if fl.ne() { pc = 0x63ef5; continue; }
            pc = 0x63f01;
        }
        0x63f01 => {
            // block 63f01
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63f01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63f01 cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // 63f04 jne 0x63e76
            if fl.ne() { pc = 0x63e76; continue; }
            pc = 0x63f0a;
        }
        0x63f0a => {
            // block 63f0a
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63f0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63f0a mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 63f0e add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 63f12 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 63f13 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 63f15 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 63f17 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 63f19 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 63f1b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 63f1c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x63f1d => {
            // block 63f1d
            #[cfg(feature = "trace")] eprintln!("f_00063e43 pc 63f1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63f1d mov rbx,rax
            rbx = rax;
            // 63f20 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 63f24 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63f29 mov rdi,rbx
            rdi = rbx;
            // 63f2c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5556 size=240
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5556<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5556;
    loop {
        match pc {
        0xb5556 => {
            // block b5556
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5556 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5557 mov rbp,rsp
            rbp = rsp;
            // b555a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b555c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b555e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5560 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5562 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5563 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b5567 mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b556b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b556e je 0xb5621
            if fl.e() { pc = 0xb5621; continue; }
            pc = 0xb5574;
        }
        0xb5574 => {
            // block b5574
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5574 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5574 mov QWORD PTR [rbp-0x48],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rsi) as u64);
            // b5578 movzx r15d,BYTE PTR [rdx]
            r15 = (m.r8(rdx) as u64) as u32 as u64;
            // b557c mov r12b,r15b
            r12 = (r12 & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b557f and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5583 shr r15,1
            { let (r, f) = shr::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // b5586 mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // b558a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b558e mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // b5592 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b5596 lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // b559a mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // b559e jmp 0xb55a3
            pc = 0xb55a3; continue;
        }
        0xb55a0 => {
            // block b55a0
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55a0 mov rbx,rax
            rbx = rax;
            pc = 0xb55a3;
        }
        0xb55a3 => {
            // block b55a3
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55a3 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b55a6 mov r13,QWORD PTR [rbp-0x30]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b55aa cmove r13,r15
            if fl.e() { r13 = r15; }
            // b55ae movzx r14d,BYTE PTR [rbx+0x20]
            r14 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b55b3 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b55b6 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b55b8 jne 0xb55bf
            if fl.ne() { pc = 0xb55bf; continue; }
            pc = 0xb55ba;
        }
        0xb55ba => {
            // block b55ba
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55ba shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b55bd jmp 0xb55c3
            pc = 0xb55c3; continue;
        }
        0xb55bf => {
            // block b55bf
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55bf mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xb55c3;
        }
        0xb55c3 => {
            // block b55c3
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55c3 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b55c6 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b55ca cmove rdi,QWORD PTR [rbp-0x40]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64; }
            // b55cf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b55d1 jne 0xb55dc
            if fl.ne() { pc = 0xb55dc; continue; }
            pc = 0xb55d3;
        }
        0xb55d3 => {
            // block b55d3
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55d3 lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // b55d7 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // b55da jmp 0xb55e0
            pc = 0xb55e0; continue;
        }
        0xb55dc => {
            // block b55dc
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55dc mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xb55e0;
        }
        0xb55e0 => {
            // block b55e0
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55e0 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // b55e3 mov rdx,r13
            rdx = r13;
            // b55e6 cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // b55ea test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b55ed je 0xb55fc
            if fl.e() { pc = 0xb55fc; continue; }
            pc = 0xb55ef;
        }
        0xb55ef => {
            // block b55ef
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55ef call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b55f4 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b55f6 je 0xb55fc
            if fl.e() { pc = 0xb55fc; continue; }
            pc = 0xb55f8;
        }
        0xb55f8 => {
            // block b55f8
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55f8 jns 0xb560b
            if fl.ns() { pc = 0xb560b; continue; }
            pc = 0xb55fa;
        }
        0xb55fa => {
            // block b55fa
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55fa jmp 0xb5601
            pc = 0xb5601; continue;
        }
        0xb55fc => {
            // block b55fc
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b55fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b55fc cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // b55ff jae 0xb560b
            if fl.ae() { pc = 0xb560b; continue; }
            pc = 0xb5601;
        }
        0xb5601 => {
            // block b5601
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5601 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b5604 test rax,rax
            fl = logic::<64>(rax & rax);
            // b5607 jne 0xb55a0
            if fl.ne() { pc = 0xb55a0; continue; }
            pc = 0xb5609;
        }
        0xb5609 => {
            // block b5609
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5609 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5609 jmp 0xb562d
            pc = 0xb562d; continue;
        }
        0xb560b => {
            // block b560b
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b560b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b560b mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b560f test rax,rax
            fl = logic::<64>(rax & rax);
            // b5612 jne 0xb55a0
            if fl.ne() { pc = 0xb55a0; continue; }
            pc = 0xb5614;
        }
        0xb5614 => {
            // block b5614
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5614 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5614 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b5618 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // b561b add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // b561f jmp 0xb5634
            pc = 0xb5634; continue;
        }
        0xb5621 => {
            // block b5621
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5621 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5621 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b5625 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // b5628 mov rbx,rdi
            rbx = rdi;
            // b562b jmp 0xb5634
            pc = 0xb5634; continue;
        }
        0xb562d => {
            // block b562d
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b562d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b562d mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b5631 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            pc = 0xb5634;
        }
        0xb5634 => {
            // block b5634
            #[cfg(feature = "trace")] eprintln!("f_000b5556 pc b5634 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5634 mov rax,rbx
            rax = rbx;
            // b5637 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b563b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b563c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b563e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b5640 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5642 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5644 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5645 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8f6a size=241
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8f6a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb8f6a;
    loop {
        match pc {
        0xb8f6a => {
            // block b8f6a
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8f6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f6a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8f6b mov rbp,rsp
            rbp = rsp;
            // b8f6e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b8f70 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8f72 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b8f74 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b8f76 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8f77 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b8f7b mov r12,rsi
            r12 = rsi;
            // b8f7e mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // b8f82 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8f86 lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // b8f8b cmp rcx,r13
            fl = sub::<64>(rcx, r13).1;
            // b8f8e je 0xb9034
            if fl.e() { pc = 0xb9034; continue; }
            pc = 0xb8f94;
        }
        0xb8f94 => {
            // block b8f94
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8f94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f94 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8f98 mov r14,rcx
            r14 = rcx;
            // b8f9b jmp 0xb8fa4
            pc = 0xb8fa4; continue;
        }
        0xb8f9d => {
            // block b8f9d
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8f9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f9d mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8fa1 mov r14,rax
            r14 = rax;
            pc = 0xb8fa4;
        }
        0xb8fa4 => {
            // block b8fa4
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8fa4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8fa4 cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // b8fa7 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // b8faa lea rsi,[rip+0x7fd17] # 0x138cc8
            rsi = m.env.data_va(316565);
            // b8fb1 lea rcx,[rip+0x714e5] # 0x12a49d
            rcx = m.env.data_va(257130);
            // b8fb8 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b8fbc movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // b8fbf add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // b8fc2 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b8fc6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8fcb mov rbx,rax
            rbx = rax;
            // b8fce mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b8fd2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b8fd7 mov rdi,r15
            rdi = r15;
            // b8fda call 0x61146
            { let r = { let t = m.env.code_va(0x61146); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8fdf movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // b8fe3 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b8fe6 jne 0xb8ff1
            if fl.ne() { pc = 0xb8ff1; continue; }
            pc = 0xb8fe8;
        }
        0xb8fe8 => {
            // block b8fe8
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8fe8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8fe8 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b8feb lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // b8fef jmp 0xb8ff9
            pc = 0xb8ff9; continue;
        }
        0xb8ff1 => {
            // block b8ff1
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8ff1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8ff1 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b8ff5 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xb8ff9;
        }
        0xb8ff9 => {
            // block b8ff9
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8ff9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8ff9 mov rdi,rbx
            rdi = rbx;
            // b8ffc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9001 mov rdi,r15
            rdi = r15;
            // b9004 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9009 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // b900d test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b9010 je 0xb901f
            if fl.e() { pc = 0xb901f; continue; }
            pc = 0xb9012;
        }
        0xb9012 => {
            // block b9012
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b9012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9012 mov rax,rcx
            rax = rcx;
            // b9015 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b9018 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b901b jne 0xb9012
            if fl.ne() { pc = 0xb9012; continue; }
            pc = 0xb901d;
        }
        0xb901d => {
            // block b901d
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b901d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b901d jmp 0xb902b
            pc = 0xb902b; continue;
        }
        0xb901f => {
            // block b901f
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b901f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b901f mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // b9023 cmp QWORD PTR [rax],r14
            fl = sub::<64>(m.r64(rax) as u64, r14).1;
            // b9026 mov r14,rax
            r14 = rax;
            // b9029 jne 0xb901f
            if fl.ne() { pc = 0xb901f; continue; }
            pc = 0xb902b;
        }
        0xb902b => {
            // block b902b
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b902b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b902b cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // b902e jne 0xb8f9d
            if fl.ne() { pc = 0xb8f9d; continue; }
            pc = 0xb9034;
        }
        0xb9034 => {
            // block b9034
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b9034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9034 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b9038 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b903c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b903d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b903f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b9041 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9043 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9045 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9046 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb9047 => {
            // block b9047
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b9047 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9047 mov rbx,rax
            rbx = rax;
            // b904a lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b904e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9053 mov rdi,rbx
            rdi = rbx;
            // b9056 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
