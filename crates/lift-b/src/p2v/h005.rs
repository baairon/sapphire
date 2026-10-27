//! Mechanically lifted functions (scripts/lift_run.py), module h005. Each one is checked against the original
//! in the world harness (tests/world_h005.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xd3547 size=135
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3547<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd3547;
    loop {
        match pc {
        0xd3547 => {
            // block d3547
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3547 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3547 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3548 mov rbp,rsp
            rbp = rsp;
            // d354b mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // d354d cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d3550 jne 0xd3562
            if fl.ne() { pc = 0xd3562; continue; }
            pc = 0xd3552;
        }
        0xd3552 => {
            // block d3552
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3552 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3552 cmp DWORD PTR [rdi+0x4],esi
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, (rsi as u32 as u64)).1;
            // d3555 jne 0xd35ca
            if fl.ne() { pc = 0xd35ca; continue; }
            pc = 0xd3557;
        }
        0xd3557 => {
            // block d3557
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3557 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3557 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d3559 lea rcx,[rip+0xea030] # 0x1bd590
            rcx = m.env.data_va(859485);
            // d3560 jmp 0xd358b
            pc = 0xd358b; continue;
        }
        0xd3562 => {
            // block d3562
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3562 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d3565 jne 0xd357d
            if fl.ne() { pc = 0xd357d; continue; }
            pc = 0xd3567;
        }
        0xd3567 => {
            // block d3567
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3567 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3567 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // d356c cmp esi,0x14
            fl = sub::<32>((rsi as u32 as u64), 0x14u64).1;
            // d356f je 0xd35a4
            if fl.e() { pc = 0xd35a4; continue; }
            pc = 0xd3571;
        }
        0xd3571 => {
            // block d3571
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3571 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3571 cmp esi,0x15
            fl = sub::<32>((rsi as u32 as u64), 0x15u64).1;
            // d3574 jne 0xd35a2
            if fl.ne() { pc = 0xd35a2; continue; }
            pc = 0xd3576;
        }
        0xd3576 => {
            // block d3576
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3576 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3576 mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // d357b jmp 0xd35a4
            pc = 0xd35a4; continue;
        }
        0xd357d => {
            // block d357d
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d357d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d357d cmp DWORD PTR [rdi+0x4],esi
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, (rsi as u32 as u64)).1;
            // d3580 jne 0xd35ca
            if fl.ne() { pc = 0xd35ca; continue; }
            pc = 0xd3582;
        }
        0xd3582 => {
            // block d3582
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d3582 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3582 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d3584 lea rcx,[rip+0xea0c5] # 0x1bd650
            rcx = m.env.data_va(859677);
            pc = 0xd358b;
        }
        0xd358b => {
            // block d358b
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d358b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d358b mov rsi,QWORD PTR [rcx+rax*8]
            rsi = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // d358f mov edx,DWORD PTR [rdi+0x8]
            rdx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // d3592 add rdi,0x40
            { let (r, f) = add::<64>(rdi, 0x40u64);
              rdi = r;
              fl = f; }
            // d3596 call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d359b test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d359d sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d35a0 jmp 0xd35cc
            pc = 0xd35cc; continue;
        }
        0xd35a2 => {
            // block d35a2
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d35a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35a2 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            pc = 0xd35a4;
        }
        0xd35a4 => {
            // block d35a4
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d35a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35a4 cmp eax,DWORD PTR [rdi+0x4]
            fl = sub::<32>((rax as u32 as u64), m.r32(rdi.wrapping_add(0x4)) as u64).1;
            // d35a7 jne 0xd35ca
            if fl.ne() { pc = 0xd35ca; continue; }
            pc = 0xd35a9;
        }
        0xd35a9 => {
            // block d35a9
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d35a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35a9 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d35ab lea rcx,[rip+0xe9f1e] # 0x1bd4d0
            rcx = m.env.data_va(859293);
            // d35b2 mov rsi,QWORD PTR [rcx+rax*8]
            rsi = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // d35b6 mov edx,DWORD PTR [rdi+0x8]
            rdx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // d35b9 add rdi,0x40
            { let (r, f) = add::<64>(rdi, 0x40u64);
              rdi = r;
              fl = f; }
            // d35bd call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d35c2 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d35c4 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d35c6 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d35c8 je 0xd35cc
            if fl.e() { pc = 0xd35cc; continue; }
            pc = 0xd35ca;
        }
        0xd35ca => {
            // block d35ca
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d35ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35ca xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd35cc;
        }
        0xd35cc => {
            // block d35cc
            #[cfg(feature = "trace")] eprintln!("f_000d3547 pc d35cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35cc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d35cd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdc3d0 size=144
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dc3d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdc3d0;
    loop {
        match pc {
        0xdc3d0 => {
            // block dc3d0
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc3d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc3d1 mov rbp,rsp
            rbp = rsp;
            // dc3d4 cmp esi,0x8
            fl = sub::<32>((rsi as u32 as u64), 0x8u64).1;
            // dc3d7 jb 0xdc3ff
            if fl.b() { pc = 0xdc3ff; continue; }
            pc = 0xdc3d9;
        }
        0xdc3d9 => {
            // block dc3d9
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc3d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3d9 movzx eax,si
            rax = ((rsi as u16 as u64)) as u32 as u64;
            // dc3dc add eax,0xfffffff8
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffff8u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // dc3df cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // dc3e2 ja 0xdc41c
            if fl.a() { pc = 0xdc41c; continue; }
            pc = 0xdc3e4;
        }
        0xdc3e4 => {
            // block dc3e4
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc3e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3e4 lea rcx,[rip+0x61] # 0xdc44c
            rcx = m.env.code_va(0xdc44c);
            // dc3eb movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // dc3ef add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // dc3f2 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xdc3f4 => {
            // block dc3f4
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc3f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3f4 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc3f7 call QWORD PTR [rax+0x1980]
            { let r = { let t = m.r64(rax.wrapping_add(0x1980)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc3fd jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc3ff => {
            // block dc3ff
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc3ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3ff mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc402 movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // dc405 imul rcx,rcx,0x608
            { let (r, f) = imul::<64>(rcx, 0x608u64); rcx = r; fl = f; }
            // dc40c lea rsi,[rip+0xe138d] # 0x1bd7a0
            rsi = m.env.data_va(860013);
            // dc413 add rsi,rcx
            { let (r, f) = add::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // dc416 call QWORD PTR [rax+0x19f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19f8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdc41c;
        }
        0xdc41c => {
            // block dc41c
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc41c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc41c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // dc41e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc41f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdc420 => {
            // block dc420
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc420 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc423 call QWORD PTR [rax+0x1988]
            { let r = { let t = m.r64(rax.wrapping_add(0x1988)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc429 jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc42b => {
            // block dc42b
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc42b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc42b mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc42e call QWORD PTR [rax+0x1990]
            { let r = { let t = m.r64(rax.wrapping_add(0x1990)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc434 jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc436 => {
            // block dc436
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc436 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc439 call QWORD PTR [rax+0x19a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19a8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc43f jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc441 => {
            // block dc441
            #[cfg(feature = "trace")] eprintln!("f_000dc3d0 pc dc441 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc441 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc444 call QWORD PTR [rax+0x19b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x19b0)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc44a jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc7f94 size=147
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c7f94<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc7f94;
    loop {
        match pc {
        0xc7f94 => {
            // block c7f94
            #[cfg(feature = "trace")] eprintln!("f_000c7f94 pc c7f94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7f94 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c7f95 mov rbp,rsp
            rbp = rsp;
            // c7f98 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c7f9a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c7f9c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c7f9e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c7fa0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c7fa1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c7fa2 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // c7fa5 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // c7fa7 mov r14,rsi
            r14 = rsi;
            // c7faa mov r12,rdi
            r12 = rdi;
            // c7fad cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // c7fb0 jne 0xc7fd1
            if fl.ne() { pc = 0xc7fd1; continue; }
            pc = 0xc7fb2;
        }
        0xc7fb2 => {
            // block c7fb2
            #[cfg(feature = "trace")] eprintln!("f_000c7f94 pc c7fb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7fb2 movzx ecx,r15b
            rcx = ((r15 as u8 as u64)) as u32 as u64;
            // c7fb6 mov rdi,r12
            rdi = r12;
            // c7fb9 mov rsi,r14
            rsi = r14;
            // c7fbc mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // c7fbe add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c7fc2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7fc3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c7fc5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c7fc7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7fc9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c7fcb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7fcc jmp 0xc68b5
            { let r = { let t = m.env.code_va(0xc68b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xc7fd1 => {
            // block c7fd1
            #[cfg(feature = "trace")] eprintln!("f_000c7f94 pc c7fd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7fd1 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // c7fd4 jne 0xc8018
            if fl.ne() { pc = 0xc8018; continue; }
            pc = 0xc7fd6;
        }
        0xc7fd6 => {
            // block c7fd6
            #[cfg(feature = "trace")] eprintln!("f_000c7f94 pc c7fd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7fd6 lea edi,[rbx+rbx*1]
            rdi = (rbx.wrapping_add(rbx)) as u32 as u64;
            // c7fd9 add rdi,rdi
            { let (r, f) = add::<64>(rdi, rdi);
              rdi = r;
              fl = f; }
            // c7fdc call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7fe1 mov r13,rax
            r13 = rax;
            // c7fe4 mov rdi,r12
            rdi = r12;
            // c7fe7 mov rsi,r13
            rsi = r13;
            // c7fea mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // c7fec call 0xc5cac
            { let r = { let t = m.env.code_va(0xc5cac); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7ff1 movzx ecx,r15b
            rcx = ((r15 as u8 as u64)) as u32 as u64;
            // c7ff5 mov rdi,r13
            rdi = r13;
            // c7ff8 mov rsi,r14
            rsi = r14;
            // c7ffb mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // c7ffd call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c8002 mov rdi,r13
            rdi = r13;
            // c8005 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c8009 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c800a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c800c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c800e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8010 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c8012 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8013 jmp 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc8018 => {
            // block c8018
            #[cfg(feature = "trace")] eprintln!("f_000c7f94 pc c8018 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8018 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c801c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c801d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c801f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c8021 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8023 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c8025 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8026 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4244 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4244<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4244;
    loop {
        match pc {
        0xc4244 => {
            // block c4244
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c4244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4244 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4245 mov rbp,rsp
            rbp = rsp;
            // c4248 mov r8,QWORD PTR [rdi]
            r8 = m.r64(rdi) as u64;
            // c424b mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c424f cmp rdx,r8
            fl = sub::<64>(rdx, r8).1;
            // c4252 je 0xc42a2
            if fl.e() { pc = 0xc42a2; continue; }
            pc = 0xc4254;
        }
        0xc4254 => {
            // block c4254
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c4254 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4254 mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            pc = 0xc4258;
        }
        0xc4258 => {
            // block c4258
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c4258 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4258 mov rax,QWORD PTR [rdx-0x8]
            rax = m.r64(rdx.wrapping_add(0xffff_ffff_ffff_fff8)) as u64;
            // c425c mov QWORD PTR [rcx-0x8],rax
            m.w64(rcx.wrapping_add(0xffff_ffff_ffff_fff8), (rax) as u64);
            // c4260 mov r9,QWORD PTR [rdx-0x18]
            r9 = m.r64(rdx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // c4264 mov rax,QWORD PTR [rdx-0x10]
            rax = m.r64(rdx.wrapping_add(0xffff_ffff_ffff_fff0)) as u64;
            // c4268 mov QWORD PTR [rcx-0x10],rax
            m.w64(rcx.wrapping_add(0xffff_ffff_ffff_fff0), (rax) as u64);
            // c426c mov QWORD PTR [rcx-0x18],r9
            m.w64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8), (r9) as u64);
            // c4270 mov QWORD PTR [rdx-0x8],0x0
            m.w64(rdx.wrapping_add(0xffff_ffff_ffff_fff8), (0x0u64) as u64);
            // c4278 mov QWORD PTR [rdx-0x10],0x0
            m.w64(rdx.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u64);
            // c4280 mov QWORD PTR [rdx-0x18],0x0
            m.w64(rdx.wrapping_add(0xffff_ffff_ffff_ffe8), (0x0u64) as u64);
            // c4288 lea rdx,[rdx-0x18]
            rdx = rdx.wrapping_add(0xffff_ffff_ffff_ffe8);
            // c428c mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c4290 add rcx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rcx, 0xffff_ffff_ffff_ffe8u64);
              rcx = r;
              fl = f; }
            // c4294 mov QWORD PTR [rsi+0x8],rcx
            m.w64(rsi.wrapping_add(0x8), (rcx) as u64);
            // c4298 cmp r8,rdx
            fl = sub::<64>(r8, rdx).1;
            // c429b jne 0xc4258
            if fl.ne() { pc = 0xc4258; continue; }
            pc = 0xc429d;
        }
        0xc429d => {
            // block c429d
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c429d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c429d mov r8,QWORD PTR [rdi]
            r8 = m.r64(rdi) as u64;
            // c42a0 jmp 0xc42a6
            pc = 0xc42a6; continue;
        }
        0xc42a2 => {
            // block c42a2
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c42a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c42a2 mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            pc = 0xc42a6;
        }
        0xc42a6 => {
            // block c42a6
            #[cfg(feature = "trace")] eprintln!("f_000c4244 pc c42a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c42a6 mov QWORD PTR [rdi],rcx
            m.w64(rdi, (rcx) as u64);
            // c42a9 mov QWORD PTR [rsi+0x8],r8
            m.w64(rsi.wrapping_add(0x8), (r8) as u64);
            // c42ad mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c42b1 mov rcx,QWORD PTR [rsi+0x10]
            rcx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // c42b5 mov QWORD PTR [rdi+0x8],rcx
            m.w64(rdi.wrapping_add(0x8), (rcx) as u64);
            // c42b9 mov QWORD PTR [rsi+0x10],rax
            m.w64(rsi.wrapping_add(0x10), (rax) as u64);
            // c42bd mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // c42c1 mov rcx,QWORD PTR [rsi+0x18]
            rcx = m.r64(rsi.wrapping_add(0x18)) as u64;
            // c42c5 mov QWORD PTR [rdi+0x10],rcx
            m.w64(rdi.wrapping_add(0x10), (rcx) as u64);
            // c42c9 mov QWORD PTR [rsi+0x18],rax
            m.w64(rsi.wrapping_add(0x18), (rax) as u64);
            // c42cd mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c42d1 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c42d4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c42d5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba598 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba598<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba598;
    loop {
        match pc {
        0xba598 => {
            // block ba598
            #[cfg(feature = "trace")] eprintln!("f_000ba598 pc ba598 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba598 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba599 mov rbp,rsp
            rbp = rsp;
            // ba59c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba59e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba5a0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba5a1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ba5a2 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // ba5a5 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // ba5a9 lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // ba5ad mov rcx,QWORD PTR [rip+0x85dfc] # 0x1403b0
            rcx = m.r64(m.env.data_va(347005)) as u64;
            // ba5b4 lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // ba5b8 mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // ba5bc lea r14,[rdi+rax*1+0x80]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x80);
            // ba5c4 lea rdx,[rcx+0x68]
            rdx = rcx.wrapping_add(0x68);
            // ba5c8 mov QWORD PTR [rdi+rax*1+0x80],rdx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x80), (rdx) as u64);
            // ba5d0 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // ba5d4 mov QWORD PTR [rdi+rax*1+0x10],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x10), (rcx) as u64);
            // ba5d9 lea rbx,[rdi+rax*1+0x18]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x18);
            // ba5de mov rcx,QWORD PTR [rip+0x85dc3] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // ba5e5 add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // ba5e9 mov QWORD PTR [rdi+rax*1+0x18],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x18), (rcx) as u64);
            // ba5ee lea rdi,[rdi+rax*1+0x58]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x58);
            // ba5f3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba5f8 mov rdi,rbx
            rdi = rbx;
            // ba5fb call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba600 mov rsi,QWORD PTR [rip+0x85bf1] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // ba607 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ba60b mov rdi,r15
            rdi = r15;
            // ba60e call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba613 mov rdi,r14
            rdi = r14;
            // ba616 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba61b mov rdi,r15
            rdi = r15;
            // ba61e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ba622 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba623 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba625 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba627 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba628 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd38e0 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d38e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd38e0;
    loop {
        match pc {
        0xd38e0 => {
            // block d38e0
            #[cfg(feature = "trace")] eprintln!("f_000d38e0 pc d38e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d38e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d38e1 mov rbp,rsp
            rbp = rsp;
            // d38e4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d38e6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d38e8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d38e9 sub rsp,0x488
            { let (r, f) = sub::<64>(rsp, 0x488u64);
              rsp = r;
              fl = f; }
            // d38f0 mov r15,rsi
            r15 = rsi;
            // d38f3 mov rbx,rdi
            rbx = rdi;
            // d38f6 test r15,r15
            fl = logic::<64>(r15 & r15);
            // d38f9 je 0xd3966
            if fl.e() { pc = 0xd3966; continue; }
            pc = 0xd38fb;
        }
        0xd38fb => {
            // block d38fb
            #[cfg(feature = "trace")] eprintln!("f_000d38e0 pc d38fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d38fb xorps xmm0,xmm0
            x0 = 0;
            // d38fe movaps XMMWORD PTR [rbp-0x4a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb60), x0);
            // d3905 mov QWORD PTR [rbp-0x490],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb70), (0x0u64) as u64);
            // d3910 mov DWORD PTR [rbp-0x4a0],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb60), (0x5u64) as u32);
            // d391a mov DWORD PTR [rbp-0x49c],0x480
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb64), (0x480u64) as u32);
            // d3924 lea r14,[rbp-0x488]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // d392b mov edx,0x468
            rdx = (0x468u64) as u32 as u64;
            // d3930 mov rdi,r14
            rdi = r14;
            // d3933 mov rsi,r15
            rsi = r15;
            // d3936 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d393b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d393e lea rsi,[rbp-0x4a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb60);
            // d3945 mov rdi,rbx
            rdi = rbx;
            // d3948 call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d394e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d3950 je 0xd3966
            if fl.e() { pc = 0xd3966; continue; }
            pc = 0xd3952;
        }
        0xd3952 => {
            // block d3952
            #[cfg(feature = "trace")] eprintln!("f_000d38e0 pc d3952 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3952 mov edx,0x468
            rdx = (0x468u64) as u32 as u64;
            // d3957 mov rdi,r15
            rdi = r15;
            // d395a mov rsi,r14
            rsi = r14;
            // d395d call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3962 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d3964 jmp 0xd3968
            pc = 0xd3968; continue;
        }
        0xd3966 => {
            // block d3966
            #[cfg(feature = "trace")] eprintln!("f_000d38e0 pc d3966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3966 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3968;
        }
        0xd3968 => {
            // block d3968
            #[cfg(feature = "trace")] eprintln!("f_000d38e0 pc d3968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3968 add rsp,0x488
            { let (r, f) = add::<64>(rsp, 0x488u64);
              rsp = r;
              fl = f; }
            // d396f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3970 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3972 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3974 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3975 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdedb8 size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dedb8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdedb8;
    loop {
        match pc {
        0xdedb8 => {
            // block dedb8
            #[cfg(feature = "trace")] eprintln!("f_000dedb8 pc dedb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dedb8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dedb9 mov rbp,rsp
            rbp = rsp;
            // dedbc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dedbe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dedc0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dedc2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dedc3 mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // dedc6 mov r14,rdx
            r14 = rdx;
            // dedc9 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // dedcc mov rbx,rdi
            rbx = rdi;
            // dedcf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dedd2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dedd4 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dedd9 call QWORD PTR [rax+0x368]
            { let r = { let t = m.r64(rax.wrapping_add(0x368)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // deddf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dede2 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dede7 mov rdi,rbx
            rdi = rbx;
            // dedea mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // deded mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // dedf0 call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dedf6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dedf9 mov rdi,rbx
            rdi = rbx;
            // dedfc mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // dedff call QWORD PTR [rax+0x1908]
            { let r = { let t = m.r64(rax.wrapping_add(0x1908)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee05 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee08 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // dee0d mov rdi,rbx
            rdi = rbx;
            // dee10 call QWORD PTR [rax+0x1940]
            { let r = { let t = m.r64(rax.wrapping_add(0x1940)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee16 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee19 mov rdi,rbx
            rdi = rbx;
            // dee1c mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // dee1f call QWORD PTR [rax+0x18e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x18e8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee25 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee28 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dee2a mov rdi,rbx
            rdi = rbx;
            // dee2d mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // dee30 call QWORD PTR [rax+0x4b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x4b0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee36 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee39 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dee3b mov rdi,rbx
            rdi = rbx;
            // dee3e mov rsi,r14
            rsi = r14;
            // dee41 call QWORD PTR [rax+0x19c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19c8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee47 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // dee49 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dee4a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dee4c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dee4e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dee50 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dee51 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5d66 size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5d66<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5d66;
    loop {
        match pc {
        0xb5d66 => {
            // block b5d66
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5d66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d66 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5d67 mov rbp,rsp
            rbp = rsp;
            // b5d6a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5d6c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5d6e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5d70 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5d72 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5d73 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b5d74 mov r13,rsi
            r13 = rsi;
            // b5d77 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b5d7b lea r14,[rdi+0x8]
            r14 = rdi.wrapping_add(0x8);
            // b5d7f mov rcx,r14
            rcx = r14;
            // b5d82 call 0xebc9c
            { let r = { let t = m.env.stub_va(0x469); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5d87 mov r15,rax
            r15 = rax;
            // b5d8a cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // b5d8d je 0xb5df9
            if fl.e() { pc = 0xb5df9; continue; }
            pc = 0xb5d8f;
        }
        0xb5d8f => {
            // block b5d8f
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5d8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d8f lea rsi,[r15+0x20]
            rsi = r15.wrapping_add(0x20);
            // b5d93 movzx r12d,BYTE PTR [r13+0x0]
            r12 = (m.r8(r13) as u64) as u32 as u64;
            // b5d98 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // b5d9b and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5d9d jne 0xb5da4
            if fl.ne() { pc = 0xb5da4; continue; }
            pc = 0xb5d9f;
        }
        0xb5d9f => {
            // block b5d9f
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5d9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d9f shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // b5da2 jmp 0xb5da8
            pc = 0xb5da8; continue;
        }
        0xb5da4 => {
            // block b5da4
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5da4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5da4 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0xb5da8;
        }
        0xb5da8 => {
            // block b5da8
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5da8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5da8 movzx ebx,BYTE PTR [rsi]
            rbx = (m.r8(rsi) as u64) as u32 as u64;
            // b5dab mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // b5dad and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5db0 jne 0xb5db7
            if fl.ne() { pc = 0xb5db7; continue; }
            pc = 0xb5db2;
        }
        0xb5db2 => {
            // block b5db2
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5db2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5db2 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // b5db5 jmp 0xb5dbb
            pc = 0xb5dbb; continue;
        }
        0xb5db7 => {
            // block b5db7
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5db7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5db7 mov rbx,QWORD PTR [r15+0x28]
            rbx = m.r64(r15.wrapping_add(0x28)) as u64;
            pc = 0xb5dbb;
        }
        0xb5dbb => {
            // block b5dbb
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5dbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5dbb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b5dbd jne 0xb5dc4
            if fl.ne() { pc = 0xb5dc4; continue; }
            pc = 0xb5dbf;
        }
        0xb5dbf => {
            // block b5dbf
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5dbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5dbf inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // b5dc2 jmp 0xb5dc8
            pc = 0xb5dc8; continue;
        }
        0xb5dc4 => {
            // block b5dc4
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5dc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5dc4 mov r13,QWORD PTR [r13+0x10]
            r13 = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0xb5dc8;
        }
        0xb5dc8 => {
            // block b5dc8
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5dc8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5dc8 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // b5dca jne 0xb5dd1
            if fl.ne() { pc = 0xb5dd1; continue; }
            pc = 0xb5dcc;
        }
        0xb5dcc => {
            // block b5dcc
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5dcc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5dcc inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // b5dcf jmp 0xb5dd5
            pc = 0xb5dd5; continue;
        }
        0xb5dd1 => {
            // block b5dd1
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5dd1 mov rsi,QWORD PTR [r15+0x30]
            rsi = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xb5dd5;
        }
        0xb5dd5 => {
            // block b5dd5
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5dd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5dd5 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // b5dd8 mov rdx,r12
            rdx = r12;
            // b5ddb cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // b5ddf test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5de2 je 0xb5df4
            if fl.e() { pc = 0xb5df4; continue; }
            pc = 0xb5de4;
        }
        0xb5de4 => {
            // block b5de4
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5de4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5de4 mov rdi,r13
            rdi = r13;
            // b5de7 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5dec test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b5dee je 0xb5df4
            if fl.e() { pc = 0xb5df4; continue; }
            pc = 0xb5df0;
        }
        0xb5df0 => {
            // block b5df0
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5df0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5df0 jns 0xb5dfc
            if fl.ns() { pc = 0xb5dfc; continue; }
            pc = 0xb5df2;
        }
        0xb5df2 => {
            // block b5df2
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5df2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5df2 jmp 0xb5df9
            pc = 0xb5df9; continue;
        }
        0xb5df4 => {
            // block b5df4
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5df4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5df4 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // b5df7 jae 0xb5dfc
            if fl.ae() { pc = 0xb5dfc; continue; }
            pc = 0xb5df9;
        }
        0xb5df9 => {
            // block b5df9
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5df9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5df9 mov r15,r14
            r15 = r14;
            pc = 0xb5dfc;
        }
        0xb5dfc => {
            // block b5dfc
            #[cfg(feature = "trace")] eprintln!("f_000b5d66 pc b5dfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5dfc mov rax,r15
            rax = r15;
            // b5dff add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b5e03 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5e04 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b5e06 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b5e08 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5e0a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5e0c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5e0d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc486a size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c486a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc486a;
    loop {
        match pc {
        0xc486a => {
            // block c486a
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c486a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c486a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c486b mov rbp,rsp
            rbp = rsp;
            // c486e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c4870 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4872 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c4874 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c4876 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4877 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c4878 mov r13,rsi
            r13 = rsi;
            // c487b mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c487f lea r14,[rdi+0x8]
            r14 = rdi.wrapping_add(0x8);
            // c4883 mov rcx,r14
            rcx = r14;
            // c4886 call 0xebe5e
            { let r = { let t = m.env.stub_va(0x62b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c488b mov r15,rax
            r15 = rax;
            // c488e cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // c4891 je 0xc48fd
            if fl.e() { pc = 0xc48fd; continue; }
            pc = 0xc4893;
        }
        0xc4893 => {
            // block c4893
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c4893 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4893 lea rsi,[r15+0x20]
            rsi = r15.wrapping_add(0x20);
            // c4897 movzx r12d,BYTE PTR [r13+0x0]
            r12 = (m.r8(r13) as u64) as u32 as u64;
            // c489c mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // c489f and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c48a1 jne 0xc48a8
            if fl.ne() { pc = 0xc48a8; continue; }
            pc = 0xc48a3;
        }
        0xc48a3 => {
            // block c48a3
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48a3 shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // c48a6 jmp 0xc48ac
            pc = 0xc48ac; continue;
        }
        0xc48a8 => {
            // block c48a8
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48a8 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0xc48ac;
        }
        0xc48ac => {
            // block c48ac
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48ac movzx ebx,BYTE PTR [rsi]
            rbx = (m.r8(rsi) as u64) as u32 as u64;
            // c48af mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // c48b1 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c48b4 jne 0xc48bb
            if fl.ne() { pc = 0xc48bb; continue; }
            pc = 0xc48b6;
        }
        0xc48b6 => {
            // block c48b6
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48b6 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // c48b9 jmp 0xc48bf
            pc = 0xc48bf; continue;
        }
        0xc48bb => {
            // block c48bb
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48bb mov rbx,QWORD PTR [r15+0x28]
            rbx = m.r64(r15.wrapping_add(0x28)) as u64;
            pc = 0xc48bf;
        }
        0xc48bf => {
            // block c48bf
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48bf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c48c1 jne 0xc48c8
            if fl.ne() { pc = 0xc48c8; continue; }
            pc = 0xc48c3;
        }
        0xc48c3 => {
            // block c48c3
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48c3 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // c48c6 jmp 0xc48cc
            pc = 0xc48cc; continue;
        }
        0xc48c8 => {
            // block c48c8
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48c8 mov r13,QWORD PTR [r13+0x10]
            r13 = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0xc48cc;
        }
        0xc48cc => {
            // block c48cc
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48cc test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // c48ce jne 0xc48d5
            if fl.ne() { pc = 0xc48d5; continue; }
            pc = 0xc48d0;
        }
        0xc48d0 => {
            // block c48d0
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48d0 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // c48d3 jmp 0xc48d9
            pc = 0xc48d9; continue;
        }
        0xc48d5 => {
            // block c48d5
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48d5 mov rsi,QWORD PTR [r15+0x30]
            rsi = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xc48d9;
        }
        0xc48d9 => {
            // block c48d9
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48d9 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // c48dc mov rdx,r12
            rdx = r12;
            // c48df cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // c48e3 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c48e6 je 0xc48f8
            if fl.e() { pc = 0xc48f8; continue; }
            pc = 0xc48e8;
        }
        0xc48e8 => {
            // block c48e8
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48e8 mov rdi,r13
            rdi = r13;
            // c48eb call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c48f0 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c48f2 je 0xc48f8
            if fl.e() { pc = 0xc48f8; continue; }
            pc = 0xc48f4;
        }
        0xc48f4 => {
            // block c48f4
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48f4 jns 0xc4900
            if fl.ns() { pc = 0xc4900; continue; }
            pc = 0xc48f6;
        }
        0xc48f6 => {
            // block c48f6
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48f6 jmp 0xc48fd
            pc = 0xc48fd; continue;
        }
        0xc48f8 => {
            // block c48f8
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48f8 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // c48fb jae 0xc4900
            if fl.ae() { pc = 0xc4900; continue; }
            pc = 0xc48fd;
        }
        0xc48fd => {
            // block c48fd
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c48fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c48fd mov r15,r14
            r15 = r14;
            pc = 0xc4900;
        }
        0xc4900 => {
            // block c4900
            #[cfg(feature = "trace")] eprintln!("f_000c486a pc c4900 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4900 mov rax,r15
            rax = r15;
            // c4903 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c4907 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4908 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c490a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c490c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c490e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c4910 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4911 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc13c size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc13c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbc13c;
    loop {
        match pc {
        0xbc13c => {
            // block bc13c
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc13c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc13c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc13d mov rbp,rsp
            rbp = rsp;
            // bc140 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc142 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc144 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc146 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc147 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc14b mov r15,rsi
            r15 = rsi;
            // bc14e mov r14d,edi
            r14 = ((rdi as u32 as u64)) as u32 as u64;
            // bc151 mov DWORD PTR [rbp-0x24],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((r14 as u32 as u64)) as u32);
            // bc155 lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // bc159 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // bc15d mov rdi,r15
            rdi = r15;
            // bc160 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc165 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // bc16d mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // bc170 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // bc178 lea rdi,[rip+0x10c521] # 0x1c86a0
            rdi = m.env.data_va(904813);
            // bc17f lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // bc183 call 0xebdc8
            { let r = { let t = m.env.stub_va(0x595); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc188 mov rbx,rax
            rbx = rax;
            // bc18b lea r12,[rip+0x10c516] # 0x1c86a8
            r12 = m.env.data_va(904821);
            // bc192 jmp 0xbc1cb
            pc = 0xbc1cb; continue;
        }
        0xbc194 => {
            // block bc194
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc194 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc194 cmp DWORD PTR [rbx+0x1c],r14d
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x1c)) as u64, (r14 as u32 as u64)).1;
            // bc198 jne 0xbc1d0
            if fl.ne() { pc = 0xbc1d0; continue; }
            pc = 0xbc19a;
        }
        0xbc19a => {
            // block bc19a
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc19a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc19a lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // bc19e mov rdi,r15
            rdi = r15;
            // bc1a1 call 0xebd98
            { let r = { let t = m.env.stub_va(0x565); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc1a6 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc1aa test rax,rax
            fl = logic::<64>(rax & rax);
            // bc1ad je 0xbc1bc
            if fl.e() { pc = 0xbc1bc; continue; }
            pc = 0xbc1af;
        }
        0xbc1af => {
            // block bc1af
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1af mov rbx,rax
            rbx = rax;
            // bc1b2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bc1b5 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc1b8 jne 0xbc1af
            if fl.ne() { pc = 0xbc1af; continue; }
            pc = 0xbc1ba;
        }
        0xbc1ba => {
            // block bc1ba
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1ba jmp 0xbc1cb
            pc = 0xbc1cb; continue;
        }
        0xbc1bc => {
            // block bc1bc
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1bc mov rax,rbx
            rax = rbx;
            pc = 0xbc1bf;
        }
        0xbc1bf => {
            // block bc1bf
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1bf mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc1c3 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // bc1c6 mov rax,rbx
            rax = rbx;
            // bc1c9 jne 0xbc1bf
            if fl.ne() { pc = 0xbc1bf; continue; }
            pc = 0xbc1cb;
        }
        0xbc1cb => {
            // block bc1cb
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1cb cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // bc1ce jne 0xbc194
            if fl.ne() { pc = 0xbc194; continue; }
            pc = 0xbc1d0;
        }
        0xbc1d0 => {
            // block bc1d0
            #[cfg(feature = "trace")] eprintln!("f_000bc13c pc bc1d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc1d0 cmp QWORD PTR [r15+0x10],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x0u64).1;
            // bc1d5 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc1d8 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc1dc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc1dd pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc1df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc1e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc1e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc1e4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc4d4 size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc4d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbc4d4;
    loop {
        match pc {
        0xbc4d4 => {
            // block bc4d4
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc4d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc4d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc4d5 mov rbp,rsp
            rbp = rsp;
            // bc4d8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc4da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc4dc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc4de push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc4df sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc4e3 mov r15,rsi
            r15 = rsi;
            // bc4e6 mov r14d,edi
            r14 = ((rdi as u32 as u64)) as u32 as u64;
            // bc4e9 mov DWORD PTR [rbp-0x24],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((r14 as u32 as u64)) as u32);
            // bc4ed lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // bc4f1 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // bc4f5 mov rdi,r15
            rdi = r15;
            // bc4f8 call 0xebdb6
            { let r = { let t = m.env.stub_va(0x583); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc4fd mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // bc505 mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // bc508 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // bc510 lea rdi,[rip+0x10c171] # 0x1c8688
            rdi = m.env.data_va(904789);
            // bc517 lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // bc51b call 0xebde0
            { let r = { let t = m.env.stub_va(0x5ad); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc520 mov rbx,rax
            rbx = rax;
            // bc523 lea r12,[rip+0x10c166] # 0x1c8690
            r12 = m.env.data_va(904797);
            // bc52a jmp 0xbc563
            pc = 0xbc563; continue;
        }
        0xbc52c => {
            // block bc52c
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc52c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc52c cmp DWORD PTR [rbx+0x1c],r14d
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x1c)) as u64, (r14 as u32 as u64)).1;
            // bc530 jne 0xbc568
            if fl.ne() { pc = 0xbc568; continue; }
            pc = 0xbc532;
        }
        0xbc532 => {
            // block bc532
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc532 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc532 lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // bc536 mov rdi,r15
            rdi = r15;
            // bc539 call 0xebdaa
            { let r = { let t = m.env.stub_va(0x577); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc53e mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc542 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc545 je 0xbc554
            if fl.e() { pc = 0xbc554; continue; }
            pc = 0xbc547;
        }
        0xbc547 => {
            // block bc547
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc547 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc547 mov rbx,rax
            rbx = rax;
            // bc54a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bc54d test rax,rax
            fl = logic::<64>(rax & rax);
            // bc550 jne 0xbc547
            if fl.ne() { pc = 0xbc547; continue; }
            pc = 0xbc552;
        }
        0xbc552 => {
            // block bc552
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc552 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc552 jmp 0xbc563
            pc = 0xbc563; continue;
        }
        0xbc554 => {
            // block bc554
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc554 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc554 mov rax,rbx
            rax = rbx;
            pc = 0xbc557;
        }
        0xbc557 => {
            // block bc557
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc557 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc557 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc55b cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // bc55e mov rax,rbx
            rax = rbx;
            // bc561 jne 0xbc557
            if fl.ne() { pc = 0xbc557; continue; }
            pc = 0xbc563;
        }
        0xbc563 => {
            // block bc563
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc563 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // bc566 jne 0xbc52c
            if fl.ne() { pc = 0xbc52c; continue; }
            pc = 0xbc568;
        }
        0xbc568 => {
            // block bc568
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc568 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc568 cmp QWORD PTR [r15+0x10],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x0u64).1;
            // bc56d setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc570 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc574 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc575 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc577 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc579 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc57b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc57c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd35ce size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d35ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd35ce;
    loop {
        match pc {
        0xd35ce => {
            // block d35ce
            #[cfg(feature = "trace")] eprintln!("f_000d35ce pc d35ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d35cf mov rbp,rsp
            rbp = rsp;
            // d35d2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d35d4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d35d6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d35d8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d35d9 sub rsp,0x1a0
            { let (r, f) = sub::<64>(rsp, 0x1a0u64);
              rsp = r;
              fl = f; }
            // d35e0 mov rbx,rsi
            rbx = rsi;
            // d35e3 mov r15,rdi
            r15 = rdi;
            // d35e6 mov r12,QWORD PTR [rip+0x6cdeb] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // d35ed mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // d35f1 mov QWORD PTR [rbp-0x28],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (r12) as u64);
            // d35f5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d35f8 je 0xd3662
            if fl.e() { pc = 0xd3662; continue; }
            pc = 0xd35fa;
        }
        0xd35fa => {
            // block d35fa
            #[cfg(feature = "trace")] eprintln!("f_000d35ce pc d35fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d35fa xorps xmm0,xmm0
            x0 = 0;
            // d35fd movaps XMMWORD PTR [rbp-0x1c0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe40), x0);
            // d3604 mov QWORD PTR [rbp-0x1b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (0x0u64) as u64);
            // d360f movabs rax,0x19400000001
            rax = 0x0_0194_0000_0001u64;
            // d3619 mov QWORD PTR [rbp-0x1c0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe40), (rax) as u64);
            // d3620 lea r14,[rbp-0x1a8]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // d3627 mov edx,0x17c
            rdx = (0x17cu64) as u32 as u64;
            // d362c mov rdi,r14
            rdi = r14;
            // d362f mov rsi,rbx
            rsi = rbx;
            // d3632 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3637 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d363a lea rsi,[rbp-0x1c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // d3641 mov rdi,r15
            rdi = r15;
            // d3644 call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d364a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d364c je 0xd3662
            if fl.e() { pc = 0xd3662; continue; }
            pc = 0xd364e;
        }
        0xd364e => {
            // block d364e
            #[cfg(feature = "trace")] eprintln!("f_000d35ce pc d364e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d364e mov edx,0x17c
            rdx = (0x17cu64) as u32 as u64;
            // d3653 mov rdi,rbx
            rdi = rbx;
            // d3656 mov rsi,r14
            rsi = r14;
            // d3659 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d365e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d3660 jmp 0xd3664
            pc = 0xd3664; continue;
        }
        0xd3662 => {
            // block d3662
            #[cfg(feature = "trace")] eprintln!("f_000d35ce pc d3662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3662 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3664;
        }
        0xd3664 => {
            // block d3664
            #[cfg(feature = "trace")] eprintln!("f_000d35ce pc d3664 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3664 cmp r12,QWORD PTR [rbp-0x28]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64).1;
            // d3668 jne 0xd367a
            if fl.ne() { pc = 0xd367a; continue; }
            pc = 0xd366a;
        }
        0xd366a => {
            // block d366a
            #[cfg(feature = "trace")] eprintln!("f_000d35ce pc d366a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d366a add rsp,0x1a0
            { let (r, f) = add::<64>(rsp, 0x1a0u64);
              rsp = r;
              fl = f; }
            // d3671 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3672 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d3674 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3676 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3678 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3679 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd367a => {
            // block d367a
            #[cfg(feature = "trace")] eprintln!("f_000d35ce pc d367a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d367a call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe08f8 size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e08f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe08f8;
    loop {
        match pc {
        0xe08f8 => {
            // block e08f8
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e08f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e08f9 mov rbp,rsp
            rbp = rsp;
            // e08fc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e08fe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0900 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e0902 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e0904 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0905 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0906 mov r14,rsi
            r14 = rsi;
            // e0909 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e090b test r14,r14
            fl = logic::<64>(r14 & r14);
            // e090e je 0xe0997
            if fl.e() { pc = 0xe0997; continue; }
            pc = 0xe0914;
        }
        0xe0914 => {
            // block e0914
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0914 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0917 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e091a mov r15,QWORD PTR [rax]
            r15 = m.r64(rax) as u64;
            // e091d mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // e0921 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0923 mov rax,rcx
            rax = rcx;
            // e0926 sub rax,r15
            { let (r, f) = sub::<64>(rax, r15);
              rax = r;
              fl = f; }
            // e0929 je 0xe0997
            if fl.e() { pc = 0xe0997; continue; }
            pc = 0xe092b;
        }
        0xe092b => {
            // block e092b
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e092b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e092b mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // e092f sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e0933 movabs r13,0x6db6db6db6db6db7
            r13 = 0x6db6_db6d_b6db_6db7u64;
            // e093d imul r13,rax
            { let (r, f) = imul::<64>(r13, rax); r13 = r; fl = f; }
            // e0941 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e0944 mov rbx,r15
            rbx = r15;
            pc = 0xe0947;
        }
        0xe0947 => {
            // block e0947
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0947 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0947 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e094b mov rsi,r14
            rsi = r14;
            // e094e call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0953 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e0955 je 0xe096d
            if fl.e() { pc = 0xe096d; continue; }
            pc = 0xe0957;
        }
        0xe0957 => {
            // block e0957
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0957 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0957 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // e095a add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // e095e cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // e0961 jb 0xe0947
            if fl.b() { pc = 0xe0947; continue; }
            pc = 0xe0963;
        }
        0xe0963 => {
            // block e0963
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0963 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0965 cmp QWORD PTR [rbp-0x30],r15
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, r15).1;
            // e0969 jne 0xe097b
            if fl.ne() { pc = 0xe097b; continue; }
            pc = 0xe096b;
        }
        0xe096b => {
            // block e096b
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e096b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e096b jmp 0xe0997
            pc = 0xe0997; continue;
        }
        0xe096d => {
            // block e096d
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e096d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e096d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // e0970 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e0974 jne 0xe0997
            if fl.ne() { pc = 0xe0997; continue; }
            pc = 0xe0976;
        }
        0xe0976 => {
            // block e0976
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0976 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // e0979 je 0xe0997
            if fl.e() { pc = 0xe0997; continue; }
            pc = 0xe097b;
        }
        0xe097b => {
            // block e097b
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e097b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e097b lea rsi,[rip+0x4a1fe] # 0x12ab80
            rsi = m.env.data_va(258893);
            // e0982 mov rdi,r14
            rdi = r14;
            // e0985 call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e098a xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e098c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e098e cmove rcx,r15
            if fl.e() { rcx = r15; }
            // e0992 mov rax,rcx
            rax = rcx;
            // e0995 jmp 0xe099a
            pc = 0xe099a; continue;
        }
        0xe0997 => {
            // block e0997
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0997 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0997 mov rax,rbx
            rax = rbx;
            pc = 0xe099a;
        }
        0xe099a => {
            // block e099a
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e099a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e099a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e099e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e099f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e09a1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e09a3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e09a5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e09a7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e09a8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb905b size=184
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b905b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb905b;
    loop {
        match pc {
        0xb905b => {
            // block b905b
            #[cfg(feature = "trace")] eprintln!("f_000b905b pc b905b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b905b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b905c mov rbp,rsp
            rbp = rsp;
            // b905f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b9061 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b9063 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b9065 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b9067 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9068 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b906c mov r15,rsi
            r15 = rsi;
            // b906f mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // b9073 mov r14,QWORD PTR [r15]
            r14 = m.r64(r15) as u64;
            // b9076 cmp r14,QWORD PTR [r15+0x8]
            fl = sub::<64>(r14, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // b907a je 0xb90ec
            if fl.e() { pc = 0xb90ec; continue; }
            pc = 0xb907c;
        }
        0xb907c => {
            // block b907c
            #[cfg(feature = "trace")] eprintln!("f_000b905b pc b907c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b907c lea r13,[rbp-0x30]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0xb9080;
        }
        0xb9080 => {
            // block b9080
            #[cfg(feature = "trace")] eprintln!("f_000b905b pc b9080 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9080 movsd xmm0,QWORD PTR [r14]
            x0 = m.r64(r14) as u128;
            // b9085 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b9089 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b908e mov rbx,rax
            rbx = rax;
            // b9091 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b9094 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b9098 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b909b mov rdi,r13
            rdi = r13;
            // b909e call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90a3 mov rdi,r13
            rdi = r13;
            // b90a6 mov rsi,QWORD PTR [rip+0x870e3] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b90ad call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90b2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b90b5 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b90b9 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b90be mov rdi,rax
            rdi = rax;
            // b90c1 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90c3 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b90c6 mov rdi,r13
            rdi = r13;
            // b90c9 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90ce movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // b90d2 mov rdi,rbx
            rdi = rbx;
            // b90d5 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90da mov rdi,rbx
            rdi = rbx;
            // b90dd call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90e2 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // b90e6 cmp r14,QWORD PTR [r15+0x8]
            fl = sub::<64>(r14, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // b90ea jne 0xb9080
            if fl.ne() { pc = 0xb9080; continue; }
            pc = 0xb90ec;
        }
        0xb90ec => {
            // block b90ec
            #[cfg(feature = "trace")] eprintln!("f_000b905b pc b90ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b90ec mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b90f0 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b90f4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b90f5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b90f7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b90f9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b90fb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b90fd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b90fe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb90ff => {
            // block b90ff
            #[cfg(feature = "trace")] eprintln!("f_000b905b pc b90ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b90ff mov rbx,rax
            rbx = rax;
            // b9102 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b9106 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b910b mov rdi,rbx
            rdi = rbx;
            // b910e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd31f1 size=190
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d31f1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd31f1;
    loop {
        match pc {
        0xd31f1 => {
            // block d31f1
            #[cfg(feature = "trace")] eprintln!("f_000d31f1 pc d31f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31f1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d31f2 mov rbp,rsp
            rbp = rsp;
            // d31f5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d31f7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d31f9 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d31fa sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // d3201 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // d3203 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // d3206 mov r14,rsi
            r14 = rsi;
            // d3209 mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // d320b xorps xmm0,xmm0
            x0 = 0;
            // d320e movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // d3212 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d3216 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // d321a movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // d321e movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // d3222 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // d3226 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // d322d movaps XMMWORD PTR [rbp-0xa0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff60), x0);
            // d3234 movsxd rdi,ebx
            rdi = sx::<32>((rbx as u32 as u64)) as u64;
            // d3237 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d323c mov cl,dil
            rcx = (rcx & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // d323f shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // d3241 mov rcx,rdi
            rcx = rdi;
            // d3244 shr rcx,0x5
            { let (r, f) = shr::<64>(rcx, 0x5u64, fl); rcx = r; fl = f; }
            // d3248 or DWORD PTR [rbp+rcx*4-0xa0],edx
            { let r = m.r32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff60)) as u64 | (rdx as u32 as u64); let f = logic::<32>(r);
              m.w32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff60), (r) as u32);
              fl = f; }
            // d324f cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // d3251 mov QWORD PTR [rbp-0xb0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (rax) as u64);
            // d3258 mov DWORD PTR [rbp-0xa8],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u32);
            // d3262 inc edi
            { let (r, f) = inc::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // d3264 lea rsi,[rbp-0xa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d326b lea r8,[rbp-0xb0]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // d3272 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d3274 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3276 call 0xec284
            { let r = { let t = m.env.stub_va(0xa51); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d327b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d327d mov eax,0xfffffffe
            rax = (0xfffffffeu64) as u32 as u64;
            // d3282 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3284 je 0xd32a1
            if fl.e() { pc = 0xd32a1; continue; }
            pc = 0xd3286;
        }
        0xd3286 => {
            // block d3286
            #[cfg(feature = "trace")] eprintln!("f_000d31f1 pc d3286 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3286 cmp ecx,0xffffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffffu64).1;
            // d3289 jne 0xd3292
            if fl.ne() { pc = 0xd3292; continue; }
            pc = 0xd328b;
        }
        0xd328b => {
            // block d328b
            #[cfg(feature = "trace")] eprintln!("f_000d31f1 pc d328b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d328b mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // d3290 jmp 0xd32a1
            pc = 0xd32a1; continue;
        }
        0xd3292 => {
            // block d3292
            #[cfg(feature = "trace")] eprintln!("f_000d31f1 pc d3292 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3292 movsxd rdx,r15d
            rdx = sx::<32>((r15 as u32 as u64)) as u64;
            // d3295 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3297 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // d3299 mov rsi,r14
            rsi = r14;
            // d329c call 0xec278
            { let r = { let t = m.env.stub_va(0xa45); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd32a1;
        }
        0xd32a1 => {
            // block d32a1
            #[cfg(feature = "trace")] eprintln!("f_000d31f1 pc d32a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d32a1 add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // d32a8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d32a9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d32ab pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d32ad pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d32ae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd32af size=194
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d32af<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd32af;
    loop {
        match pc {
        0xd32af => {
            // block d32af
            #[cfg(feature = "trace")] eprintln!("f_000d32af pc d32af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d32af push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d32b0 mov rbp,rsp
            rbp = rsp;
            // d32b3 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d32b5 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d32b7 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d32b9 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d32ba sub rsp,0x90
            { let (r, f) = sub::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // d32c1 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // d32c3 mov r12d,edi
            r12 = ((rdi as u32 as u64)) as u32 as u64;
            // d32c6 xorps xmm0,xmm0
            x0 = 0;
            // d32c9 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // d32cd movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d32d1 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // d32d5 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // d32d9 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // d32dd movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // d32e1 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // d32e8 movaps XMMWORD PTR [rbp-0xa0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff60), x0);
            // d32ef movsxd rdi,r12d
            rdi = sx::<32>((r12 as u32 as u64)) as u64;
            // d32f2 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // d32f7 mov cl,dil
            rcx = (rcx & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // d32fa shl ebx,cl
            { let (r, f) = shl::<32>((rbx as u32 as u64), (rcx as u8 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // d32fc mov rcx,rdi
            rcx = rdi;
            // d32ff shr rcx,0x5
            { let (r, f) = shr::<64>(rcx, 0x5u64, fl); rcx = r; fl = f; }
            // d3303 or DWORD PTR [rbp+rcx*4-0xa0],ebx
            { let r = m.r32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff60)) as u64 | (rbx as u32 as u64); let f = logic::<32>(r);
              m.w32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff60), (r) as u32);
              fl = f; }
            // d330a mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // d330d mov r14,rsi
            r14 = rsi;
            // d3310 mov QWORD PTR [rbp-0xb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u64);
            // d331b mov DWORD PTR [rbp-0xa8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), ((rax as u32 as u64)) as u32);
            // d3321 inc edi
            { let (r, f) = inc::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // d3323 lea rsi,[rbp-0xa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d332a lea r8,[rbp-0xb0]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // d3331 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d3333 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3335 call 0xec284
            { let r = { let t = m.env.stub_va(0xa51); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d333a mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d333c mov eax,0xfffffffe
            rax = (0xfffffffeu64) as u32 as u64;
            // d3341 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3343 je 0xd3361
            if fl.e() { pc = 0xd3361; continue; }
            pc = 0xd3345;
        }
        0xd3345 => {
            // block d3345
            #[cfg(feature = "trace")] eprintln!("f_000d32af pc d3345 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3345 cmp ecx,0xffffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffffu64).1;
            // d3348 jne 0xd3351
            if fl.ne() { pc = 0xd3351; continue; }
            pc = 0xd334a;
        }
        0xd334a => {
            // block d334a
            #[cfg(feature = "trace")] eprintln!("f_000d32af pc d334a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d334a mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // d334f jmp 0xd3361
            pc = 0xd3361; continue;
        }
        0xd3351 => {
            // block d3351
            #[cfg(feature = "trace")] eprintln!("f_000d32af pc d3351 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3351 movsxd rdx,r15d
            rdx = sx::<32>((r15 as u32 as u64)) as u64;
            // d3354 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3356 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // d3359 mov rsi,r14
            rsi = r14;
            // d335c call 0xec278
            { let r = { let t = m.env.stub_va(0xa45); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd3361;
        }
        0xd3361 => {
            // block d3361
            #[cfg(feature = "trace")] eprintln!("f_000d32af pc d3361 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3361 add rsp,0x90
            { let (r, f) = add::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // d3368 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3369 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d336b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d336d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d336f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3370 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5c92 size=212
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5c92<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5c92;
    loop {
        match pc {
        0xb5c92 => {
            // block b5c92
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5c92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c92 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5c93 mov rbp,rsp
            rbp = rsp;
            // b5c96 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5c98 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5c9a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5c9c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5c9e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5c9f sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b5ca3 mov rbx,rcx
            rbx = rcx;
            // b5ca6 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5ca9 je 0xb5d53
            if fl.e() { pc = 0xb5d53; continue; }
            pc = 0xb5caf;
        }
        0xb5caf => {
            // block b5caf
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5caf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5caf movzx r14d,BYTE PTR [rsi]
            r14 = (m.r8(rsi) as u64) as u32 as u64;
            // b5cb3 mov r15b,r14b
            r15 = (r15 & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b5cb6 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5cba shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b5cbd mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // b5cc1 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b5cc5 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // b5cc9 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b5ccd lea rax,[rsi+0x1]
            rax = rsi.wrapping_add(0x1);
            // b5cd1 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0xb5cd5;
        }
        0xb5cd5 => {
            // block b5cd5
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5cd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5cd5 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // b5cd9 mov rbx,rdx
            rbx = rdx;
            pc = 0xb5cdc;
        }
        0xb5cdc => {
            // block b5cdc
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5cdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5cdc movzx r13d,BYTE PTR [rbx+0x20]
            r13 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b5ce1 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // b5ce4 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5ce6 jne 0xb5ced
            if fl.ne() { pc = 0xb5ced; continue; }
            pc = 0xb5ce8;
        }
        0xb5ce8 => {
            // block b5ce8
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5ce8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ce8 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // b5ceb jmp 0xb5cf1
            pc = 0xb5cf1; continue;
        }
        0xb5ced => {
            // block b5ced
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5ced rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ced mov r13,QWORD PTR [rbx+0x28]
            r13 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xb5cf1;
        }
        0xb5cf1 => {
            // block b5cf1
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5cf1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5cf1 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b5cf4 mov r12,QWORD PTR [rbp-0x30]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b5cf8 cmove r12,r14
            if fl.e() { r12 = r14; }
            // b5cfc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b5cfe jne 0xb5d09
            if fl.ne() { pc = 0xb5d09; continue; }
            pc = 0xb5d00;
        }
        0xb5d00 => {
            // block b5d00
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d00 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // b5d04 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // b5d07 jmp 0xb5d0d
            pc = 0xb5d0d; continue;
        }
        0xb5d09 => {
            // block b5d09
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d09 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xb5d0d;
        }
        0xb5d0d => {
            // block b5d0d
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d0d cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // b5d10 mov rdx,r13
            rdx = r13;
            // b5d13 cmovb rdx,r12
            if fl.b() { rdx = r12; }
            // b5d17 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5d1a je 0xb5d35
            if fl.e() { pc = 0xb5d35; continue; }
            pc = 0xb5d1c;
        }
        0xb5d1c => {
            // block b5d1c
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d1c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b5d1f mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b5d23 cmove rsi,QWORD PTR [rbp-0x40]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64; }
            // b5d28 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5d2d test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b5d2f je 0xb5d35
            if fl.e() { pc = 0xb5d35; continue; }
            pc = 0xb5d31;
        }
        0xb5d31 => {
            // block b5d31
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d31 js 0xb5d3a
            if fl.s() { pc = 0xb5d3a; continue; }
            pc = 0xb5d33;
        }
        0xb5d33 => {
            // block b5d33
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d33 jmp 0xb5d45
            pc = 0xb5d45; continue;
        }
        0xb5d35 => {
            // block b5d35
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d35 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // b5d38 jae 0xb5d45
            if fl.ae() { pc = 0xb5d45; continue; }
            pc = 0xb5d3a;
        }
        0xb5d3a => {
            // block b5d3a
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d3a mov rbx,QWORD PTR [rbx+0x8]
            rbx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5d3e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5d41 jne 0xb5cdc
            if fl.ne() { pc = 0xb5cdc; continue; }
            pc = 0xb5d43;
        }
        0xb5d43 => {
            // block b5d43
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d43 jmp 0xb5d4f
            pc = 0xb5d4f; continue;
        }
        0xb5d45 => {
            // block b5d45
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d45 mov rdx,QWORD PTR [rbx]
            rdx = m.r64(rbx) as u64;
            // b5d48 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5d4b jne 0xb5cd5
            if fl.ne() { pc = 0xb5cd5; continue; }
            pc = 0xb5d4d;
        }
        0xb5d4d => {
            // block b5d4d
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d4d jmp 0xb5d53
            pc = 0xb5d53; continue;
        }
        0xb5d4f => {
            // block b5d4f
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d4f mov rbx,QWORD PTR [rbp-0x48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xb5d53;
        }
        0xb5d53 => {
            // block b5d53
            #[cfg(feature = "trace")] eprintln!("f_000b5c92 pc b5d53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5d53 mov rax,rbx
            rax = rbx;
            // b5d56 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b5d5a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5d5b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b5d5d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b5d5f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5d61 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5d63 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5d64 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4912 size=212
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4912<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4912;
    loop {
        match pc {
        0xc4912 => {
            // block c4912
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c4912 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4912 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4913 mov rbp,rsp
            rbp = rsp;
            // c4916 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c4918 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c491a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c491c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c491e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c491f sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c4923 mov rbx,rcx
            rbx = rcx;
            // c4926 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4929 je 0xc49d3
            if fl.e() { pc = 0xc49d3; continue; }
            pc = 0xc492f;
        }
        0xc492f => {
            // block c492f
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c492f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c492f movzx r14d,BYTE PTR [rsi]
            r14 = (m.r8(rsi) as u64) as u32 as u64;
            // c4933 mov r15b,r14b
            r15 = (r15 & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // c4936 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c493a shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // c493d mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c4941 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // c4945 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // c4949 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // c494d lea rax,[rsi+0x1]
            rax = rsi.wrapping_add(0x1);
            // c4951 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0xc4955;
        }
        0xc4955 => {
            // block c4955
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c4955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4955 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // c4959 mov rbx,rdx
            rbx = rdx;
            pc = 0xc495c;
        }
        0xc495c => {
            // block c495c
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c495c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c495c movzx r13d,BYTE PTR [rbx+0x20]
            r13 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // c4961 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // c4964 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c4966 jne 0xc496d
            if fl.ne() { pc = 0xc496d; continue; }
            pc = 0xc4968;
        }
        0xc4968 => {
            // block c4968
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c4968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4968 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // c496b jmp 0xc4971
            pc = 0xc4971; continue;
        }
        0xc496d => {
            // block c496d
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c496d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c496d mov r13,QWORD PTR [rbx+0x28]
            r13 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xc4971;
        }
        0xc4971 => {
            // block c4971
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c4971 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4971 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c4974 mov r12,QWORD PTR [rbp-0x30]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // c4978 cmove r12,r14
            if fl.e() { r12 = r14; }
            // c497c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c497e jne 0xc4989
            if fl.ne() { pc = 0xc4989; continue; }
            pc = 0xc4980;
        }
        0xc4980 => {
            // block c4980
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c4980 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4980 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // c4984 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // c4987 jmp 0xc498d
            pc = 0xc498d; continue;
        }
        0xc4989 => {
            // block c4989
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c4989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4989 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xc498d;
        }
        0xc498d => {
            // block c498d
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c498d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c498d cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // c4990 mov rdx,r13
            rdx = r13;
            // c4993 cmovb rdx,r12
            if fl.b() { rdx = r12; }
            // c4997 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c499a je 0xc49b5
            if fl.e() { pc = 0xc49b5; continue; }
            pc = 0xc499c;
        }
        0xc499c => {
            // block c499c
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c499c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c499c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c499f mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c49a3 cmove rsi,QWORD PTR [rbp-0x40]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64; }
            // c49a8 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c49ad test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c49af je 0xc49b5
            if fl.e() { pc = 0xc49b5; continue; }
            pc = 0xc49b1;
        }
        0xc49b1 => {
            // block c49b1
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c49b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c49b1 js 0xc49ba
            if fl.s() { pc = 0xc49ba; continue; }
            pc = 0xc49b3;
        }
        0xc49b3 => {
            // block c49b3
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c49b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c49b3 jmp 0xc49c5
            pc = 0xc49c5; continue;
        }
        0xc49b5 => {
            // block c49b5
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c49b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c49b5 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // c49b8 jae 0xc49c5
            if fl.ae() { pc = 0xc49c5; continue; }
            pc = 0xc49ba;
        }
        0xc49ba => {
            // block c49ba
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c49ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c49ba mov rbx,QWORD PTR [rbx+0x8]
            rbx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c49be test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c49c1 jne 0xc495c
            if fl.ne() { pc = 0xc495c; continue; }
            pc = 0xc49c3;
        }
        0xc49c3 => {
            // block c49c3
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c49c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c49c3 jmp 0xc49cf
            pc = 0xc49cf; continue;
        }
        0xc49c5 => {
            // block c49c5
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c49c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c49c5 mov rdx,QWORD PTR [rbx]
            rdx = m.r64(rbx) as u64;
            // c49c8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c49cb jne 0xc4955
            if fl.ne() { pc = 0xc4955; continue; }
            pc = 0xc49cd;
        }
        0xc49cd => {
            // block c49cd
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c49cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c49cd jmp 0xc49d3
            pc = 0xc49d3; continue;
        }
        0xc49cf => {
            // block c49cf
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c49cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c49cf mov rbx,QWORD PTR [rbp-0x48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xc49d3;
        }
        0xc49d3 => {
            // block c49d3
            #[cfg(feature = "trace")] eprintln!("f_000c4912 pc c49d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c49d3 mov rax,rbx
            rax = rbx;
            // c49d6 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c49da pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c49db pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c49dd pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c49df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c49e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c49e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c49e4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8250 size=216
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d8250<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd8250;
    loop {
        match pc {
        0xd8250 => {
            // block d8250
            #[cfg(feature = "trace")] eprintln!("f_000d8250 pc d8250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8250 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8251 mov rbp,rsp
            rbp = rsp;
            // d8254 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d8255 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d8256 mov rbx,rdi
            rbx = rdi;
            // d8259 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d825c mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8261 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d8263 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8268 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d826b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d826e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8271 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8276 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d827b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8280 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8283 mov rdi,rbx
            rdi = rbx;
            // d8286 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8289 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d828c mov rdi,rbx
            rdi = rbx;
            // d828f call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8295 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8298 mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d829d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d829f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d82a4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d82a7 mov rdi,rbx
            rdi = rbx;
            // d82aa call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82ad mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82b0 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d82b5 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d82ba mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d82bf xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d82c2 mov rdi,rbx
            rdi = rbx;
            // d82c5 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82c8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82cb mov rdi,rbx
            rdi = rbx;
            // d82ce call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82d4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82d7 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d82dc mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d82e1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d82e6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d82e9 mov rdi,rbx
            rdi = rbx;
            // d82ec call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82ef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82f2 mov rdi,rbx
            rdi = rbx;
            // d82f5 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82fb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82fe mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8303 mov edx,0xc7
            rdx = (0xc7u64) as u32 as u64;
            // d8308 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d830d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8310 mov rdi,rbx
            rdi = rbx;
            // d8313 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8316 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8319 mov rdi,rbx
            rdi = rbx;
            // d831c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d8320 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d8321 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8322 jmp QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
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
            { let r = { let t = m.env.stub_va(0x559); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
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

/// parity: P2v @0xe64b6 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e64b6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut x2: u128 = fx[2] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe64b6;
    loop {
        match pc {
        0xe64b6 => {
            // block e64b6
            #[cfg(feature = "trace")] eprintln!("f_000e64b6 pc e64b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e64b6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e64b7 mov rbp,rsp
            rbp = rsp;
            // e64ba mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e64bc shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e64bf mov DWORD PTR [rdi+0x7c],eax
            m.w32(rdi.wrapping_add(0x7c), ((rax as u32 as u64)) as u32);
            // e64c2 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e64c4 shr eax,0x18
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x18u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e64c7 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e64ca mov DWORD PTR [rdi+0x78],eax
            m.w32(rdi.wrapping_add(0x78), ((rax as u32 as u64)) as u32);
            // e64cd mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e64cf shr eax,0x14
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x14u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e64d2 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e64d5 mov DWORD PTR [rdi+0x74],eax
            m.w32(rdi.wrapping_add(0x74), ((rax as u32 as u64)) as u32);
            // e64d8 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e64da shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e64dd and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e64e0 mov DWORD PTR [rdi+0x70],eax
            m.w32(rdi.wrapping_add(0x70), ((rax as u32 as u64)) as u32);
            // e64e3 mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // e64e6 shr r10d,0xc
            { let (r, f) = shr::<32>((r10 as u32 as u64), 0xcu64, fl); r10 = (r) as u32 as u64; fl = f; }
            // e64ea mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // e64ec shr edx,0x8
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x8u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e64ef mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // e64f1 shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e64f4 movd xmm0,r10d
            x0 = ((r10 as u32 as u64)) as u32 as u128;
            // e64f9 movd xmm1,ecx
            x1 = ((rcx as u32 as u64)) as u32 as u128;
            // e64fd punpckldq xmm1,xmm0
            { let a = x1; let b = x0; x1 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // e6501 movd xmm0,esi
            x0 = ((rsi as u32 as u64)) as u32 as u128;
            // e6505 movd xmm2,edx
            x2 = ((rdx as u32 as u64)) as u32 as u128;
            // e6509 punpckldq xmm0,xmm2
            { let a = x0; let b = x2; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // e650d punpckldq xmm0,xmm1
            { let a = x0; let b = x1; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // e6511 pand xmm0,XMMWORD PTR [rip+0x41e07] # 0x128320
            x0 &= m.r128(m.env.data_va(248557));
            // e6519 movdqu XMMWORD PTR [rdi+0x60],xmm0
            m.w128(rdi.wrapping_add(0x60), x0);
            // e651e mov r8d,0xf0f0f0f
            r8 = (0xf0f0f0fu64) as u32 as u64;
            // e6524 mov r9d,DWORD PTR [rdi+0x88]
            r9 = (m.r32(rdi.wrapping_add(0x88)) as u64) as u32 as u64;
            // e652b and r9d,r8d
            { let r = (r9 as u32 as u64) & (r8 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // e652e and r8d,DWORD PTR [rdi+0x84]
            { let r = (r8 as u32 as u64) & m.r32(rdi.wrapping_add(0x84)) as u64; let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e6535 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e6537 and eax,0xf0000000
            { let r = (rax as u32 as u64) & 0xf0000000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e653c and ecx,0xf00000
            { let r = (rcx as u32 as u64) & 0xf00000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6542 or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6544 and edx,0xf000
            { let r = (rdx as u32 as u64) & 0xf000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e654a or edx,ecx
            { let r = (rdx as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e654c and r10d,0xf0
            { let r = (r10 as u32 as u64) & 0xf0u64; let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // e6553 or r10d,edx
            { let r = (r10 as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // e6556 or r10d,r9d
            { let r = (r10 as u32 as u64) | (r9 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // e6559 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e655b shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e655e and eax,0xf0000000
            { let r = (rax as u32 as u64) & 0xf0000000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6563 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // e6565 shl ecx,0xc
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0xcu64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e6568 and ecx,0xf00000
            { let r = (rcx as u32 as u64) & 0xf00000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e656e or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6570 shl esi,0x8
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x8u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e6573 and esi,0xf000
            { let r = (rsi as u32 as u64) & 0xf000u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e6579 or esi,ecx
            { let r = (rsi as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e657b movd eax,xmm0
            rax = (x0 as u32 as u64) as u32 as u64;
            // e657f shl eax,0x4
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e6582 or eax,esi
            { let r = (rax as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6584 or eax,r8d
            { let r = (rax as u32 as u64) | (r8 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6587 mov DWORD PTR [rdi+0x88],r10d
            m.w32(rdi.wrapping_add(0x88), ((r10 as u32 as u64)) as u32);
            // e658e mov DWORD PTR [rdi+0x84],eax
            m.w32(rdi.wrapping_add(0x84), ((rax as u32 as u64)) as u32);
            // e6594 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e6596 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e6597 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0404 size=230
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0404<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0404;
    loop {
        match pc {
        0xe0404 => {
            // block e0404
            #[cfg(feature = "trace")] eprintln!("f_000e0404 pc e0404 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0404 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0405 mov rbp,rsp
            rbp = rsp;
            // e0408 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e040a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e040c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e040e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e040f imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // e0413 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e0415 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // e0416 idiv r8d
            { let (q, r) = idiv::<32>(rdx, rax, (r8 as u32 as u64)); rax = q; rdx = r; }
            // e0419 mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // e041c imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // e0420 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e0422 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // e0423 idiv r8d
            { let (q, r) = idiv::<32>(rdx, rax, (r8 as u32 as u64)); rax = q; rdx = r; }
            // e0426 mov r10d,eax
            r10 = ((rax as u32 as u64)) as u32 as u64;
            // e0429 cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // e042c jge 0xe04e0
            if fl.ge() { pc = 0xe04e0; continue; }
            pc = 0xe0432;
        }
        0xe0432 => {
            // block e0432
            #[cfg(feature = "trace")] eprintln!("f_000e0404 pc e0432 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0432 movzx ecx,WORD PTR [rbp+0x10]
            rcx = (m.r16(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // e0436 shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // e043a mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // e043d cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // e043e idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // e0441 sub r10d,r11d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // e0444 movsxd rdx,r11d
            rdx = sx::<32>((r11 as u32 as u64)) as u64;
            // e0447 imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            // e044b movsx r9,cx
            r9 = sx::<16>((rcx as u16 as u64)) as u64;
            // e044f imul rdx,r9
            { let (r, f) = imul::<64>(rdx, r9); rdx = r; fl = f; }
            // e0453 lea r15,[rsi+rdx*2]
            r15 = rsi.wrapping_add(rdx.wrapping_mul(2));
            // e0457 add r9,r9
            { let (r, f) = add::<64>(r9, r9);
              r9 = r;
              fl = f; }
            // e045a lea r14,[rip+0x47bbf] # 0x128020
            r14 = m.env.data_va(247789);
            // e0461 mov r8w,0x7fff
            r8 = (r8 & !0xffff) | ((0x7fffu64) as u16 as u64);
            pc = 0xe0466;
        }
        0xe0466 => {
            // block e0466
            #[cfg(feature = "trace")] eprintln!("f_000e0404 pc e0466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0466 mov ecx,r11d
            rcx = ((r11 as u32 as u64)) as u32 as u64;
            // e0469 sar ecx,0x10
            { let (r, f) = sar::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e046c mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // e046f shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0472 and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0475 movsxd r12,ecx
            r12 = sx::<32>((rcx as u32 as u64)) as u64;
            // e0478 movsx esi,WORD PTR [rdi+r12*2-0x2]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e047e mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // e0483 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0485 imul esi,DWORD PTR [r14+rdx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r14.wrapping_add(rdx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // e048a movsx edx,WORD PTR [rdi+r12*2]
            rdx = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e048f mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // e0494 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0496 imul edx,DWORD PTR [r14+rcx*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r14.wrapping_add(rcx.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // e049b add edx,esi
            { let (r, f) = add::<32>((rdx as u32 as u64), (rsi as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e049d movsx ecx,WORD PTR [rdi+r12*2+0x2]
            rcx = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e04a3 mov esi,0x60
            rsi = (0x60u64) as u32 as u64;
            // e04a8 sub esi,ebx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e04aa imul ecx,DWORD PTR [r14+rsi*4]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(r14.wrapping_add(rsi.wrapping_mul(4))) as u64); rcx = (r) as u32 as u64; fl = f; }
            // e04af add ecx,edx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rdx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e04b1 movsx edx,WORD PTR [rdi+r12*2+0x4]
            rdx = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e04b7 mov esi,0x80
            rsi = (0x80u64) as u32 as u64;
            // e04bc sub esi,ebx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e04be imul edx,DWORD PTR [r14+rsi*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r14.wrapping_add(rsi.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // e04c3 add edx,ecx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e04c5 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // e04c7 shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e04ca test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e04cc cmovs cx,r8w
            if fl.s() { rcx = (rcx & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            // e04d1 mov WORD PTR [r15],cx
            m.w16(r15, ((rcx as u16 as u64)) as u16);
            // e04d5 add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // e04d8 add r15,r9
            { let (r, f) = add::<64>(r15, r9);
              r15 = r;
              fl = f; }
            // e04db dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // e04de jne 0xe0466
            if fl.ne() { pc = 0xe0466; continue; }
            pc = 0xe04e0;
        }
        0xe04e0 => {
            // block e04e0
            #[cfg(feature = "trace")] eprintln!("f_000e0404 pc e04e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e04e0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e04e1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e04e3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e04e5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e04e7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e04e8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe86f8 size=364
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e86f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe86f8;
    loop {
        match pc {
        0xe86f8 => {
            // block e86f8
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e86f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e86f9 mov rbp,rsp
            rbp = rsp;
            // e86fc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e86fe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e8700 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e8702 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e8704 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e8705 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e8706 mov r15,rsi
            r15 = rsi;
            // e8709 mov r14,rdi
            r14 = rdi;
            // e870c lea rsi,[r15+0x2a8]
            rsi = r15.wrapping_add(0x2a8);
            // e8713 movzx r13d,BYTE PTR [r15+0x2a8]
            r13 = (m.r8(r15.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // e871b mov r11b,r13b
            r11 = (r11 & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // e871e and r11b,0x1
            { let r = (r11 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r11 = (r11 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e8722 jne 0xe8730
            if fl.ne() { pc = 0xe8730; continue; }
            pc = 0xe8724;
        }
        0xe8724 => {
            // block e8724
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8724 lea r8,[rsi+0x1]
            r8 = rsi.wrapping_add(0x1);
            // e8728 mov rdi,r13
            rdi = r13;
            // e872b shr rdi,1
            { let (r, f) = shr::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // e872e jmp 0xe873e
            pc = 0xe873e; continue;
        }
        0xe8730 => {
            // block e8730
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8730 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8730 mov rdi,QWORD PTR [r15+0x2b0]
            rdi = m.r64(r15.wrapping_add(0x2b0)) as u64;
            // e8737 mov r8,QWORD PTR [r15+0x2b8]
            r8 = m.r64(r15.wrapping_add(0x2b8)) as u64;
            pc = 0xe873e;
        }
        0xe873e => {
            // block e873e
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e873e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e873e mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e8743 cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e8747 jb 0xe87bc
            if fl.b() { pc = 0xe87bc; continue; }
            pc = 0xe8749;
        }
        0xe8749 => {
            // block e8749
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8749 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8749 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e874e cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e8752 jl 0xe87bc
            if fl.l() { pc = 0xe87bc; continue; }
            pc = 0xe8754;
        }
        0xe8754 => {
            // block e8754
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8754 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8754 mov rax,rdi
            rax = rdi;
            // e8757 add rax,0xfffffffffffffffd
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fffdu64);
              rax = r;
              fl = f; }
            // e875b mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e8760 je 0xe87bc
            if fl.e() { pc = 0xe87bc; continue; }
            pc = 0xe8762;
        }
        0xe8762 => {
            // block e8762
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8762 lea r9,[r8+rdi*1]
            r9 = r8.wrapping_add(rdi);
            // e8766 lea r10,[r8+rdi*1-0x3]
            r10 = r8.wrapping_add(rdi).wrapping_add(0xffff_ffff_ffff_fffd);
            // e876b lea r12,[rip+0x54ad7] # 0x13d249
            r12 = m.env.data_va(334358);
            // e8772 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e8777 mov rdi,r8
            rdi = r8;
            pc = 0xe877a;
        }
        0xe877a => {
            // block e877a
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e877a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e877a movzx eax,BYTE PTR [rdi]
            rax = (m.r8(rdi) as u64) as u32 as u64;
            // e877d cmp eax,0x44
            fl = sub::<32>((rax as u32 as u64), 0x44u64).1;
            // e8780 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e8785 jne 0xe879d
            if fl.ne() { pc = 0xe879d; continue; }
            pc = 0xe8787;
        }
        0xe8787 => {
            // block e8787
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8787 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8787 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // e878b je 0xe87a7
            if fl.e() { pc = 0xe87a7; continue; }
            pc = 0xe878d;
        }
        0xe878d => {
            // block e878d
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e878d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e878d movzx ebx,BYTE PTR [rax+r12*1]
            rbx = (m.r8(rax.wrapping_add(r12)) as u64) as u32 as u64;
            // e8792 movzx ecx,BYTE PTR [rdi+rax*1]
            rcx = (m.r8(rdi.wrapping_add(rax)) as u64) as u32 as u64;
            // e8796 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e8799 cmp ecx,ebx
            fl = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64)).1;
            // e879b je 0xe8787
            if fl.e() { pc = 0xe8787; continue; }
            pc = 0xe879d;
        }
        0xe879d => {
            // block e879d
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e879d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e879d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e87a0 cmp rdi,r10
            fl = sub::<64>(rdi, r10).1;
            // e87a3 jne 0xe877a
            if fl.ne() { pc = 0xe877a; continue; }
            pc = 0xe87a5;
        }
        0xe87a5 => {
            // block e87a5
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87a5 jmp 0xe87bc
            pc = 0xe87bc; continue;
        }
        0xe87a7 => {
            // block e87a7
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87a7 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e87ac cmp rdi,r9
            fl = sub::<64>(rdi, r9).1;
            // e87af je 0xe87bc
            if fl.e() { pc = 0xe87bc; continue; }
            pc = 0xe87b1;
        }
        0xe87b1 => {
            // block e87b1
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87b1 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e87b6 sub rdx,r8
            { let (r, f) = sub::<64>(rdx, r8);
              rdx = r;
              fl = f; }
            // e87b9 add rdx,rdi
            { let (r, f) = add::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            pc = 0xe87bc;
        }
        0xe87bc => {
            // block e87bc
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87bc test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // e87bf jne 0xe87ca
            if fl.ne() { pc = 0xe87ca; continue; }
            pc = 0xe87c1;
        }
        0xe87c1 => {
            // block e87c1
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87c1 lea r10,[rsi+0x1]
            r10 = rsi.wrapping_add(0x1);
            // e87c5 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // e87c8 jmp 0xe87d8
            pc = 0xe87d8; continue;
        }
        0xe87ca => {
            // block e87ca
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87ca mov r13,QWORD PTR [r15+0x2b0]
            r13 = m.r64(r15.wrapping_add(0x2b0)) as u64;
            // e87d1 mov r10,QWORD PTR [r15+0x2b8]
            r10 = m.r64(r15.wrapping_add(0x2b8)) as u64;
            pc = 0xe87d8;
        }
        0xe87d8 => {
            // block e87d8
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87d8 mov r11,0xffffffffffffffff
            r11 = 0xffff_ffff_ffff_ffffu64;
            // e87df cmp r13,0x4
            fl = sub::<64>(r13, 0x4u64).1;
            // e87e3 jb 0xe883f
            if fl.b() { pc = 0xe883f; continue; }
            pc = 0xe87e5;
        }
        0xe87e5 => {
            // block e87e5
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87e5 cmp r13,0x4
            fl = sub::<64>(r13, 0x4u64).1;
            // e87e9 jl 0xe883f
            if fl.l() { pc = 0xe883f; continue; }
            pc = 0xe87eb;
        }
        0xe87eb => {
            // block e87eb
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87eb mov rcx,r13
            rcx = r13;
            // e87ee add rcx,0xfffffffffffffffd
            { let (r, f) = add::<64>(rcx, 0xffff_ffff_ffff_fffdu64);
              rcx = r;
              fl = f; }
            // e87f2 je 0xe883f
            if fl.e() { pc = 0xe883f; continue; }
            pc = 0xe87f4;
        }
        0xe87f4 => {
            // block e87f4
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87f4 lea r8,[r10+r13*1]
            r8 = r10.wrapping_add(r13);
            // e87f8 lea r9,[r10+r13*1-0x3]
            r9 = r10.wrapping_add(r13).wrapping_add(0xffff_ffff_ffff_fffd);
            // e87fd lea r15,[rip+0x54a45] # 0x13d249
            r15 = m.env.data_va(334358);
            // e8804 mov rcx,r10
            rcx = r10;
            pc = 0xe8807;
        }
        0xe8807 => {
            // block e8807
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8807 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8807 movzx edi,BYTE PTR [rcx]
            rdi = (m.r8(rcx) as u64) as u32 as u64;
            // e880a cmp edi,0x44
            fl = sub::<32>((rdi as u32 as u64), 0x44u64).1;
            // e880d mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // e8812 jne 0xe882a
            if fl.ne() { pc = 0xe882a; continue; }
            pc = 0xe8814;
        }
        0xe8814 => {
            // block e8814
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8814 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8814 cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e8818 je 0xe8834
            if fl.e() { pc = 0xe8834; continue; }
            pc = 0xe881a;
        }
        0xe881a => {
            // block e881a
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e881a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e881a movzx eax,BYTE PTR [rdi+r15*1]
            rax = (m.r8(rdi.wrapping_add(r15)) as u64) as u32 as u64;
            // e881f movzx ebx,BYTE PTR [rcx+rdi*1]
            rbx = (m.r8(rcx.wrapping_add(rdi)) as u64) as u32 as u64;
            // e8823 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e8826 cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // e8828 je 0xe8814
            if fl.e() { pc = 0xe8814; continue; }
            pc = 0xe882a;
        }
        0xe882a => {
            // block e882a
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e882a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e882a inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // e882d cmp rcx,r9
            fl = sub::<64>(rcx, r9).1;
            // e8830 jne 0xe8807
            if fl.ne() { pc = 0xe8807; continue; }
            pc = 0xe8832;
        }
        0xe8832 => {
            // block e8832
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8832 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8832 jmp 0xe883f
            pc = 0xe883f; continue;
        }
        0xe8834 => {
            // block e8834
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8834 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8834 cmp rcx,r8
            fl = sub::<64>(rcx, r8).1;
            // e8837 je 0xe883f
            if fl.e() { pc = 0xe883f; continue; }
            pc = 0xe8839;
        }
        0xe8839 => {
            // block e8839
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8839 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8839 sub rcx,r10
            { let (r, f) = sub::<64>(rcx, r10);
              rcx = r;
              fl = f; }
            // e883c mov r11,rcx
            r11 = rcx;
            pc = 0xe883f;
        }
        0xe883f => {
            // block e883f
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e883f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e883f mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // e8844 sub rcx,r11
            { let (r, f) = sub::<64>(rcx, r11);
              rcx = r;
              fl = f; }
            // e8847 mov rdi,r14
            rdi = r14;
            // e884a mov r8,rsi
            r8 = rsi;
            // e884d call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8852 mov rax,r14
            rax = r14;
            // e8855 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e8859 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e885a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e885c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e885e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e8860 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e8862 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e8863 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc51ba size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c51ba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc51ba;
    loop {
        match pc {
        0xc51ba => {
            // block c51ba
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c51ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c51ba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c51bb mov rbp,rsp
            rbp = rsp;
            // c51be push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c51c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c51c2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c51c4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c51c6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c51c7 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c51cb mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c51cf test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c51d2 je 0xc530e
            if fl.e() { pc = 0xc530e; continue; }
            pc = 0xc51d8;
        }
        0xc51d8 => {
            // block c51d8
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c51d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c51d8 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // c51dc movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // c51df mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c51e2 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c51e6 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // c51e9 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // c51ed mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // c51f1 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // c51f5 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // c51f9 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // c51fd lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // c5201 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // c5205 jmp 0xc520a
            pc = 0xc520a; continue;
        }
        0xc5207 => {
            // block c5207
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5207 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5207 mov rbx,rax
            rbx = rax;
            pc = 0xc520a;
        }
        0xc520a => {
            // block c520a
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c520a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c520a test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c520d mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c5211 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // c5216 movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // c521a mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c521d and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c5221 jne 0xc522b
            if fl.ne() { pc = 0xc522b; continue; }
            pc = 0xc5223;
        }
        0xc5223 => {
            // block c5223
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5223 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5223 mov r14,rax
            r14 = rax;
            // c5226 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // c5229 jmp 0xc522f
            pc = 0xc522f; continue;
        }
        0xc522b => {
            // block c522b
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c522b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c522b mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xc522f;
        }
        0xc522f => {
            // block c522f
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c522f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c522f lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // c5233 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c5236 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c523a cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // c523f test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c5242 jne 0xc524e
            if fl.ne() { pc = 0xc524e; continue; }
            pc = 0xc5244;
        }
        0xc5244 => {
            // block c5244
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5244 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // c5248 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // c524c jmp 0xc5256
            pc = 0xc5256; continue;
        }
        0xc524e => {
            // block c524e
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c524e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c524e mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // c5252 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xc5256;
        }
        0xc5256 => {
            // block c5256
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5256 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5256 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // c5259 mov rdx,r13
            rdx = r13;
            // c525c cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // c5260 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c5263 je 0xc527a
            if fl.e() { pc = 0xc527a; continue; }
            pc = 0xc5265;
        }
        0xc5265 => {
            // block c5265
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5265 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // c5269 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c526e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c5270 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // c5274 je 0xc527a
            if fl.e() { pc = 0xc527a; continue; }
            pc = 0xc5276;
        }
        0xc5276 => {
            // block c5276
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5276 jns 0xc528c
            if fl.ns() { pc = 0xc528c; continue; }
            pc = 0xc5278;
        }
        0xc5278 => {
            // block c5278
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5278 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5278 jmp 0xc527f
            pc = 0xc527f; continue;
        }
        0xc527a => {
            // block c527a
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c527a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c527a cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // c527d jae 0xc528c
            if fl.ae() { pc = 0xc528c; continue; }
            pc = 0xc527f;
        }
        0xc527f => {
            // block c527f
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c527f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c527f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c5282 test rax,rax
            fl = logic::<64>(rax & rax);
            // c5285 jne 0xc5207
            if fl.ne() { pc = 0xc5207; continue; }
            pc = 0xc5287;
        }
        0xc5287 => {
            // block c5287
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5287 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5287 jmp 0xc5323
            pc = 0xc5323; continue;
        }
        0xc528c => {
            // block c528c
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c528c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c528c test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c528f jne 0xc5299
            if fl.ne() { pc = 0xc5299; continue; }
            pc = 0xc5291;
        }
        0xc5291 => {
            // block c5291
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5291 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5291 mov r14,rbx
            r14 = rbx;
            // c5294 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // c5297 jmp 0xc52a0
            pc = 0xc52a0; continue;
        }
        0xc5299 => {
            // block c5299
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5299 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5299 mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // c529d mov r14,rbx
            r14 = rbx;
            pc = 0xc52a0;
        }
        0xc52a0 => {
            // block c52a0
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52a0 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // c52a4 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c52a7 mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c52ab cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // c52b0 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c52b3 jne 0xc52ba
            if fl.ne() { pc = 0xc52ba; continue; }
            pc = 0xc52b5;
        }
        0xc52b5 => {
            // block c52b5
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52b5 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // c52b8 jmp 0xc52be
            pc = 0xc52be; continue;
        }
        0xc52ba => {
            // block c52ba
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52ba mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0xc52be;
        }
        0xc52be => {
            // block c52be
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52be cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // c52c1 mov rdx,rax
            rdx = rax;
            // c52c4 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // c52c8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c52cb je 0xc52ec
            if fl.e() { pc = 0xc52ec; continue; }
            pc = 0xc52cd;
        }
        0xc52cd => {
            // block c52cd
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52cd test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c52d0 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c52d4 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // c52d9 mov r12,rax
            r12 = rax;
            // c52dc call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c52e1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c52e3 mov rax,r12
            rax = r12;
            // c52e6 je 0xc52ec
            if fl.e() { pc = 0xc52ec; continue; }
            pc = 0xc52e8;
        }
        0xc52e8 => {
            // block c52e8
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52e8 js 0xc52f1
            if fl.s() { pc = 0xc52f1; continue; }
            pc = 0xc52ea;
        }
        0xc52ea => {
            // block c52ea
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52ea jmp 0xc531a
            pc = 0xc531a; continue;
        }
        0xc52ec => {
            // block c52ec
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52ec cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // c52ef jae 0xc531a
            if fl.ae() { pc = 0xc531a; continue; }
            pc = 0xc52f1;
        }
        0xc52f1 => {
            // block c52f1
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52f1 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // c52f5 test rax,rax
            fl = logic::<64>(rax & rax);
            // c52f8 jne 0xc5207
            if fl.ne() { pc = 0xc5207; continue; }
            pc = 0xc52fe;
        }
        0xc52fe => {
            // block c52fe
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52fe mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c5302 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // c5305 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c5309 mov rax,r14
            rax = r14;
            // c530c jmp 0xc532d
            pc = 0xc532d; continue;
        }
        0xc530e => {
            // block c530e
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c530e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c530e add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c5312 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c5315 mov rax,rdi
            rax = rdi;
            // c5318 jmp 0xc532d
            pc = 0xc532d; continue;
        }
        0xc531a => {
            // block c531a
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c531a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c531a mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c531e mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // c5321 jmp 0xc532d
            pc = 0xc532d; continue;
        }
        0xc5323 => {
            // block c5323
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5323 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5323 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c5327 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // c532a mov rax,rbx
            rax = rbx;
            pc = 0xc532d;
        }
        0xc532d => {
            // block c532d
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c532d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c532d add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c5331 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5332 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c5334 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c5336 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c5338 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c533a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c533b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9208 size=388
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9208<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9208;
    loop {
        match pc {
        0xe9208 => {
            // block e9208
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9208 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9208 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9209 mov rbp,rsp
            rbp = rsp;
            // e920c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e920e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9210 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e9212 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e9214 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9215 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e9219 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e921c mov r14,rdi
            r14 = rdi;
            // e921f xorps xmm0,xmm0
            x0 = 0;
            // e9222 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // e9226 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // e922e xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e9230 cmp QWORD PTR [r14+0x90],0x0
            fl = sub::<64>(m.r64(r14.wrapping_add(0x90)) as u64, 0x0u64).1;
            // e9238 je 0xe9353
            if fl.e() { pc = 0xe9353; continue; }
            pc = 0xe923e;
        }
        0xe923e => {
            // block e923e
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e923e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e923e lea r13,[r14+0x8]
            r13 = r14.wrapping_add(0x8);
            // e9242 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e9244 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e9246 mov rdi,r13
            rdi = r13;
            // e9249 call 0xec044
            { let r = { let t = m.env.stub_va(0x811); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e924e lea rdi,[r14+0x268]
            rdi = r14.wrapping_add(0x268);
            // e9255 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // e9258 je 0xe9269
            if fl.e() { pc = 0xe9269; continue; }
            pc = 0xe925a;
        }
        0xe925a => {
            // block e925a
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e925a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e925a lea esi,[r15+0x1]
            rsi = (r15.wrapping_add(0x1)) as u32 as u64;
            // e925e mov QWORD PTR [rbp-0x60],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rdi) as u64);
            // e9262 call 0xebf18
            { let r = { let t = m.env.stub_va(0x6e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9267 jmp 0xe927d
            pc = 0xe927d; continue;
        }
        0xe9269 => {
            // block e9269
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9269 mov r15d,0x1e8480
            r15 = (0x1e8480u64) as u32 as u64;
            // e926f mov esi,0x1e8480
            rsi = (0x1e8480u64) as u32 as u64;
            // e9274 mov QWORD PTR [rbp-0x60],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rdi) as u64);
            // e9278 call 0xebf18
            { let r = { let t = m.env.stub_va(0x6e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe927d;
        }
        0xe927d => {
            // block e927d
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e927d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e927d xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // e927f call 0xec0ec
            { let r = { let t = m.env.stub_va(0x8b9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9284 mov QWORD PTR [rbp-0x68],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r14) as u64);
            // e9288 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // e928b mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e928f mov QWORD PTR [rbp-0x70],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r15) as u64);
            // e9293 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e9296 lea rbx,[rbp-0x50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e929a xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // e929d jmp 0xe92a6
            pc = 0xe92a6; continue;
        }
        0xe929f => {
            // block e929f
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e929f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e929f inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // e92a2 add r14,0x18
            { let (r, f) = add::<64>(r14, 0x18u64);
              r14 = r;
              fl = f; }
            pc = 0xe92a6;
        }
        0xe92a6 => {
            // block e92a6
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e92a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e92a6 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // e92aa mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e92ae add rsi,r13
            { let (r, f) = add::<64>(rsi, r13);
              rsi = r;
              fl = f; }
            // e92b1 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92b5 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92ba lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92be mov rsi,QWORD PTR [rip+0x56ecb] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // e92c5 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92ca mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e92cd mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // e92d1 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // e92d6 mov rdi,rax
            rdi = rax;
            // e92d9 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92db mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e92de lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92e2 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92e7 movsx edx,r12b
            rdx = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // e92eb mov rdi,r13
            rdi = r13;
            // e92ee mov rsi,rbx
            rsi = rbx;
            // e92f1 call 0xebf00
            { let r = { let t = m.env.stub_va(0x6cd); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92f6 cmp r15,QWORD PTR [rbp-0x58]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64).1;
            // e92fa jae 0xe9328
            if fl.ae() { pc = 0xe9328; continue; }
            pc = 0xe92fc;
        }
        0xe92fc => {
            // block e92fc
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e92fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e92fc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e92ff mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e9303 mov eax,DWORD PTR [rcx+rax*1+0x20]
            rax = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x20)) as u64) as u32 as u64;
            // e9307 and eax,0x5
            { let r = (rax as u32 as u64) & 0x5u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e930a jne 0xe9328
            if fl.ne() { pc = 0xe9328; continue; }
            pc = 0xe930c;
        }
        0xe930c => {
            // block e930c
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e930c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e930c mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e9310 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // e9313 add rdi,r14
            { let (r, f) = add::<64>(rdi, r14);
              rdi = r;
              fl = f; }
            // e9316 mov rsi,rbx
            rsi = rbx;
            // e9319 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e931e jmp 0xe929f
            pc = 0xe929f; continue;
        }
        0xe9323 => {
            // block e9323
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9323 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9323 mov rbx,rax
            rbx = rax;
            // e9326 jmp 0xe9379
            pc = 0xe9379; continue;
        }
        0xe9328 => {
            // block e9328
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9328 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // e932c cmp eax,0x1e847f
            fl = sub::<32>((rax as u32 as u64), 0x1e847fu64).1;
            // e9331 mov rbx,QWORD PTR [rbp-0x68]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e9335 ja 0xe934d
            if fl.a() { pc = 0xe934d; continue; }
            pc = 0xe9337;
        }
        0xe9337 => {
            // block e9337
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9337 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9337 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e933b add r14,QWORD PTR [rax]
            { let (r, f) = add::<64>(r14, m.r64(rax) as u64);
              r14 = r;
              fl = f; }
            // e933e lea rsi,[rip+0x4fb72] # 0x138eb7
            rsi = m.env.data_va(317060);
            // e9345 mov rdi,r14
            rdi = r14;
            // e9348 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe934d;
        }
        0xe934d => {
            // block e934d
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e934d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e934d mov ebx,DWORD PTR [rbx+0x260]
            rbx = (m.r32(rbx.wrapping_add(0x260)) as u64) as u32 as u64;
            pc = 0xe9353;
        }
        0xe9353 => {
            // block e9353
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9353 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9353 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e9357 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e935c mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e935e add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e9362 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9363 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e9365 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e9367 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9369 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e936b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e936c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe936d => {
            // block e936d
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e936d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e936d mov rbx,rax
            rbx = rax;
            // e9370 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e9374 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe9379;
        }
        0xe9379 => {
            // block e9379
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9379 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e937d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9382 mov rdi,rbx
            rdi = rbx;
            // e9385 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e938a jmp 0xe9323
            pc = 0xe9323; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc62d4 size=435
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c62d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut x2: u128 = fx[2] as u128;
    let mut x3: u128 = fx[3] as u128;
    let mut x4: u128 = fx[4] as u128;
    let mut x5: u128 = fx[5] as u128;
    let mut x6: u128 = fx[6] as u128;
    let mut x7: u128 = fx[7] as u128;
    let mut x8: u128 = 0;
    let mut x9: u128 = 0;
    let mut x10: u128 = 0;
    let mut x11: u128 = 0;
    let mut x12: u128 = 0;
    let mut x13: u128 = 0;
    let mut x14: u128 = 0;
    let mut x15: u128 = 0;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc62d4;
    loop {
        match pc {
        0xc62d4 => {
            // block c62d4
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c62d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c62d5 mov rbp,rsp
            rbp = rsp;
            // c62d8 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c62da jle 0xc6485
            if fl.le() { pc = 0xc6485; continue; }
            pc = 0xc62e0;
        }
        0xc62e0 => {
            // block c62e0
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c62e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62e0 add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c62e2 and ecx,0xfffffffc
            { let r = (rcx as u32 as u64) & 0xfffffffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c62e5 movsxd rax,ecx
            rax = sx::<32>((rcx as u32 as u64)) as u64;
            // c62e8 lea r10,[rsi+rax*2]
            r10 = rsi.wrapping_add(rax.wrapping_mul(2));
            // c62ec mov r9d,edx
            r9 = ((rdx as u32 as u64)) as u32 as u64;
            // c62ef xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c62f1 movsd xmm10,QWORD PTR [rip+0x3caf6] # 0x102df0
            x10 = m.r64(m.env.data_va(95677)) as u128;
            // c62fa movapd xmm11,XMMWORD PTR [rip+0x3cb2d] # 0x102e30
            x11 = m.r128(m.env.data_va(95741));
            // c6303 movapd xmm12,XMMWORD PTR [rip+0x3cb34] # 0x102e40
            x12 = m.r128(m.env.data_va(95757));
            // c630c movapd xmm13,XMMWORD PTR [rip+0x3cb3b] # 0x102e50
            x13 = m.r128(m.env.data_va(95773));
            // c6315 movsd xmm14,QWORD PTR [rip+0x3caaa] # 0x102dc8
            x14 = m.r64(m.env.data_va(95637)) as u128;
            // c631e movsd xmm15,QWORD PTR [rip+0x3caa9] # 0x102dd0
            x15 = m.r64(m.env.data_va(95645)) as u128;
            // c6327 movsd xmm9,QWORD PTR [rip+0x3caa8] # 0x102dd8
            x9 = m.r64(m.env.data_va(95653)) as u128;
            // c6330 movapd xmm1,XMMWORD PTR [rip+0x3cac8] # 0x102e00
            x1 = m.r128(m.env.data_va(95693));
            // c6338 movapd xmm2,XMMWORD PTR [rip+0x3cad0] # 0x102e10
            x2 = m.r128(m.env.data_va(95709));
            // c6340 movapd xmm3,XMMWORD PTR [rip+0x3cad8] # 0x102e20
            x3 = m.r128(m.env.data_va(95725));
            pc = 0xc6348;
        }
        0xc6348 => {
            // block c6348
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6348 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6348 movzx ecx,WORD PTR [rdi+rdx*8+0x4]
            rcx = (m.r16(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4)) as u64) as u32 as u64;
            // c634d xorps xmm4,xmm4
            x4 = 0;
            // c6350 cvtsi2sd xmm4,ecx
            x4 = set_lo64(x4, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c6354 movzx ecx,WORD PTR [rdi+rdx*8+0x2]
            rcx = (m.r16(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x2)) as u64) as u32 as u64;
            // c6359 movzx esi,WORD PTR [rdi+rdx*8]
            rsi = (m.r16(rdi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u32 as u64;
            // c635d movapd xmm6,xmm4
            x6 = x4;
            // c6361 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // c6364 je 0xc63d0
            if fl.e() { pc = 0xc63d0; continue; }
            pc = 0xc6366;
        }
        0xc6366 => {
            // block c6366
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6366 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6366 mulsd xmm6,QWORD PTR [rip+0x3ca72] # 0x102de0
            x6 = set_lo64(x6, (f64_of(x6) * f64_of((m.r64(m.env.data_va(95661)) as u128))).to_bits());
            // c636e movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // c6371 cvtsi2sd xmm7,ecx
            x7 = set_lo64(x7, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c6375 movapd xmm5,xmm7
            x5 = x7;
            // c6379 unpcklpd xmm5,xmm4
            x5 = (x5 & u64::MAX as u128) | ((x4 & u64::MAX as u128) << 64);
            // c637d unpcklpd xmm4,xmm7
            x4 = (x4 & u64::MAX as u128) | ((x7 & u64::MAX as u128) << 64);
            // c6381 mulsd xmm7,QWORD PTR [rip+0x3ca5f] # 0x102de8
            x7 = set_lo64(x7, (f64_of(x7) * f64_of((m.r64(m.env.data_va(95669)) as u128))).to_bits());
            // c6389 addsd xmm7,xmm6
            x7 = set_lo64(x7, (f64_of(x7) + f64_of(x6)).to_bits());
            // c638d movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // c6390 xorps xmm6,xmm6
            x6 = 0;
            // c6393 cvtsi2sd xmm6,ecx
            x6 = set_lo64(x6, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c6397 movddup xmm8,xmm6
            { let v = (x6) as u64; x8 = (v as u128) | ((v as u128) << 64); }
            // c639c mulsd xmm6,xmm10
            x6 = set_lo64(x6, (f64_of(x6) * f64_of(x10)).to_bits());
            // c63a1 addsd xmm6,xmm7
            x6 = set_lo64(x6, (f64_of(x6) + f64_of(x7)).to_bits());
            // c63a5 cvttsd2si rsi,xmm6
            rsi = cvtt64(f64_of(x6)) as u64;
            // c63aa add esi,0x40
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x40u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // c63ad mulpd xmm5,xmm11
            { let a = x5; let b = x11; x5 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63b2 mulpd xmm4,xmm12
            { let a = x4; let b = x12; x4 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63b7 subpd xmm5,xmm4
            { let a = x5; let b = x4; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63bb mulpd xmm8,xmm13
            { let a = x8; let b = x13; x8 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63c0 movapd xmm4,xmm5
            x4 = x5;
            // c63c4 addpd xmm4,xmm8
            { let a = x4; let b = x8; x4 = ((f64::from_bits(a as u64) + f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) + f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63c9 subpd xmm5,xmm8
            { let a = x5; let b = x8; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63ce jmp 0xc6429
            pc = 0xc6429; continue;
        }
        0xc63d0 => {
            // block c63d0
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c63d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c63d0 mulsd xmm6,xmm14
            x6 = set_lo64(x6, (f64_of(x6) * f64_of(x14)).to_bits());
            // c63d5 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // c63d8 cvtsi2sd xmm7,ecx
            x7 = set_lo64(x7, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c63dc movapd xmm5,xmm7
            x5 = x7;
            // c63e0 unpcklpd xmm5,xmm4
            x5 = (x5 & u64::MAX as u128) | ((x4 & u64::MAX as u128) << 64);
            // c63e4 unpcklpd xmm4,xmm7
            x4 = (x4 & u64::MAX as u128) | ((x7 & u64::MAX as u128) << 64);
            // c63e8 mulsd xmm7,xmm15
            x7 = set_lo64(x7, (f64_of(x7) * f64_of(x15)).to_bits());
            // c63ed addsd xmm7,xmm6
            x7 = set_lo64(x7, (f64_of(x7) + f64_of(x6)).to_bits());
            // c63f1 movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // c63f4 xorps xmm6,xmm6
            x6 = 0;
            // c63f7 cvtsi2sd xmm6,ecx
            x6 = set_lo64(x6, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c63fb movddup xmm0,xmm6
            { let v = (x6) as u64; x0 = (v as u128) | ((v as u128) << 64); }
            // c63ff mulsd xmm6,xmm9
            x6 = set_lo64(x6, (f64_of(x6) * f64_of(x9)).to_bits());
            // c6404 addsd xmm6,xmm7
            x6 = set_lo64(x6, (f64_of(x6) + f64_of(x7)).to_bits());
            // c6408 cvttsd2si rsi,xmm6
            rsi = cvtt64(f64_of(x6)) as u64;
            // c640d mulpd xmm5,xmm1
            { let a = x5; let b = x1; x5 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6411 mulpd xmm4,xmm2
            { let a = x4; let b = x2; x4 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6415 subpd xmm5,xmm4
            { let a = x5; let b = x4; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6419 mulpd xmm0,xmm3
            { let a = x0; let b = x3; x0 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c641d movapd xmm4,xmm5
            x4 = x5;
            // c6421 addpd xmm4,xmm0
            { let a = x4; let b = x0; x4 = ((f64::from_bits(a as u64) + f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) + f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6425 subpd xmm5,xmm0
            { let a = x5; let b = x0; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            pc = 0xc6429;
        }
        0xc6429 => {
            // block c6429
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6429 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6429 movsd xmm5,xmm4
            x5 = set_lo64(x5, x4 as u64);
            // c642d and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // c6433 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // c6436 jne 0xc6471
            if fl.ne() { pc = 0xc6471; continue; }
            pc = 0xc6438;
        }
        0xc6438 => {
            // block c6438
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6438 cvttsd2si rcx,xmm5
            rcx = cvtt64(f64_of(x5)) as u64;
            // c643d add ecx,0x200
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x200u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c6443 and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c6449 shufpd xmm5,xmm5,0x1
            { let a = x5; let b = x5; x5 = (((a >> 64)) & u64::MAX as u128) | (((b) & u64::MAX as u128) << 64); }
            // c644e cvttsd2si rax,xmm5
            rax = cvtt64(f64_of(x5)) as u64;
            // c6453 add eax,0x200
            { let (r, f) = add::<32>((rax as u32 as u64), 0x200u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6458 and eax,0x3ff
            { let r = (rax as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c645d mov WORD PTR [r10],cx
            m.w16(r10, ((rcx as u16 as u64)) as u16);
            // c6461 mov WORD PTR [r10+0x2],si
            m.w16(r10.wrapping_add(0x2), ((rsi as u16 as u64)) as u16);
            // c6466 mov WORD PTR [r10+0x4],ax
            m.w16(r10.wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // c646b add r10,0x6
            { let (r, f) = add::<64>(r10, 0x6u64);
              r10 = r;
              fl = f; }
            // c646f jmp 0xc6479
            pc = 0xc6479; continue;
        }
        0xc6471 => {
            // block c6471
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6471 mov WORD PTR [r10],si
            m.w16(r10, ((rsi as u16 as u64)) as u16);
            // c6475 add r10,0x2
            { let (r, f) = add::<64>(r10, 0x2u64);
              r10 = r;
              fl = f; }
            pc = 0xc6479;
        }
        0xc6479 => {
            // block c6479
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6479 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6479 inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // c647c cmp r9d,edx
            fl = sub::<32>((r9 as u32 as u64), (rdx as u32 as u64)).1;
            // c647f jne 0xc6348
            if fl.ne() { pc = 0xc6348; continue; }
            pc = 0xc6485;
        }
        0xc6485 => {
            // block c6485
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6485 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c6486 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdfdb4 size=483
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dfdb4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdfdb4;
    loop {
        match pc {
        0xdfdb4 => {
            // block dfdb4
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dfdb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfdb4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dfdb5 mov rbp,rsp
            rbp = rsp;
            // dfdb8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dfdba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dfdbc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dfdbe push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dfdc0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dfdc1 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // dfdc3 mov DWORD PTR [rdi-0x4],eax
            m.w32(rdi.wrapping_add(0xffff_ffff_ffff_fffc), ((rax as u32 as u64)) as u32);
            // dfdc6 movsxd rbx,r8d
            rbx = sx::<32>((r8 as u32 as u64)) as u64;
            // dfdc9 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // dfdcc movsxd r10,eax
            r10 = sx::<32>((rax as u32 as u64)) as u64;
            // dfdcf lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // dfdd2 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // dfdd4 mov eax,DWORD PTR [rdi+rax*4]
            rax = (m.r32(rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // dfdd7 mov DWORD PTR [rdi+r10*4],eax
            m.w32(rdi.wrapping_add(r10.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // dfddb mov DWORD PTR [rdi+rbx*4],eax
            m.w32(rdi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // dfdde imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfde2 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // dfde4 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dfde5 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // dfde7 mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // dfdea imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // dfdee mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // dfdf0 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dfdf1 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // dfdf3 mov r10d,eax
            r10 = ((rax as u32 as u64)) as u32 as u64;
            // dfdf6 cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // dfdf9 jge 0xdff8c
            if fl.ge() { pc = 0xdff8c; continue; }
            pc = 0xdfdff;
        }
        0xdfdff => {
            // block dfdff
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dfdff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfdff shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // dfe03 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // dfe06 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dfe07 idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // dfe0a mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // dfe0d sub r10d,r11d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // dfe10 movsxd rcx,r11d
            rcx = sx::<32>((r11 as u32 as u64)) as u64;
            // dfe13 lea r13,[rsi+rcx*4]
            r13 = rsi.wrapping_add(rcx.wrapping_mul(4));
            // dfe17 imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            pc = 0xdfe1b;
        }
        0xdfe1b => {
            // block dfe1b
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dfe1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfe1b mov ecx,r11d
            rcx = ((r11 as u32 as u64)) as u32 as u64;
            // dfe1e sar ecx,0x10
            { let (r, f) = sar::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // dfe21 mov r9d,r11d
            r9 = ((r11 as u32 as u64)) as u32 as u64;
            // dfe24 shr r9d,0xb
            { let (r, f) = shr::<32>((r9 as u32 as u64), 0xbu64, fl); r9 = (r) as u32 as u64; fl = f; }
            // dfe28 and r9d,0x1f
            { let r = (r9 as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // dfe2c movsxd rsi,ecx
            rsi = sx::<32>((rcx as u32 as u64)) as u64;
            // dfe2f movzx r8d,BYTE PTR [rdi+rsi*4-0x2]
            r8 = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u32 as u64;
            // dfe35 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // dfe3a sub ecx,r9d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r9 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // dfe3d lea rax,[rip+0x481dc] # 0x128020
            rax = m.env.data_va(247789);
            // dfe44 mov r12d,DWORD PTR [rax+rcx*4]
            r12 = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe48 imul r8d,r12d
            { let (r, f) = imul::<32>((r8 as u32 as u64), (r12 as u32 as u64)); r8 = (r) as u32 as u64; fl = f; }
            // dfe4c movzx ebx,BYTE PTR [rdi+rsi*4+0x2]
            rbx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x2)) as u64) as u32 as u64;
            // dfe51 mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // dfe56 sub ecx,r9d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r9 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // dfe59 mov ecx,DWORD PTR [rax+rcx*4]
            rcx = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe5c imul ebx,ecx
            { let (r, f) = imul::<32>((rbx as u32 as u64), (rcx as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // dfe5f add ebx,r8d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r8 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // dfe62 movzx r14d,BYTE PTR [rdi+rsi*4+0x6]
            r14 = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x6)) as u64) as u32 as u64;
            // dfe68 mov edx,0x60
            rdx = (0x60u64) as u32 as u64;
            // dfe6d sub edx,r9d
            { let (r, f) = sub::<32>((rdx as u32 as u64), (r9 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfe70 mov r8d,DWORD PTR [rax+rdx*4]
            r8 = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe74 imul r14d,r8d
            { let (r, f) = imul::<32>((r14 as u32 as u64), (r8 as u32 as u64)); r14 = (r) as u32 as u64; fl = f; }
            // dfe78 add r14d,ebx
            { let (r, f) = add::<32>((r14 as u32 as u64), (rbx as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // dfe7b movzx ebx,BYTE PTR [rdi+rsi*4+0xa]
            rbx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa)) as u64) as u32 as u64;
            // dfe80 mov edx,0x80
            rdx = (0x80u64) as u32 as u64;
            // dfe85 sub edx,r9d
            { let (r, f) = sub::<32>((rdx as u32 as u64), (r9 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfe88 mov r9d,DWORD PTR [rax+rdx*4]
            r9 = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe8c imul ebx,r9d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r9 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // dfe90 add ebx,r14d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r14 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // dfe93 movzx edx,BYTE PTR [rdi+rsi*4-0x3]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffd)) as u64) as u32 as u64;
            // dfe98 imul edx,r12d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r12 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfe9c movzx eax,BYTE PTR [rdi+rsi*4+0x1]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x1)) as u64) as u32 as u64;
            // dfea1 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfea4 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dfea6 movzx edx,BYTE PTR [rdi+rsi*4+0x5]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x5)) as u64) as u32 as u64;
            // dfeab imul edx,r8d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r8 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfeaf add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfeb1 movzx eax,BYTE PTR [rdi+rsi*4+0x9]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x9)) as u64) as u32 as u64;
            // dfeb6 imul eax,r9d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r9 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfeba add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dfebc mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // dfebf shr r14d,0x8
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x8u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // dfec3 and r14d,0xff00
            { let r = (r14 as u32 as u64) & 0xff00u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // dfeca cmp eax,0xffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffu64).1;
            // dfecf mov eax,0xff00
            rax = (0xff00u64) as u32 as u64;
            // dfed4 cmova r14d,eax
            if fl.a() { r14 = ((rax as u32 as u64)) as u32 as u64; } else { r14 = r14 as u32 as u64; }
            // dfed8 movzx eax,BYTE PTR [rdi+rsi*4-0x4]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // dfedd imul eax,r12d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r12 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfee1 movzx edx,BYTE PTR [rdi+rsi*4]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // dfee5 imul edx,ecx
            { let (r, f) = imul::<32>((rdx as u32 as u64), (rcx as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfee8 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfeea movzx eax,BYTE PTR [rdi+rsi*4+0x4]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x4)) as u64) as u32 as u64;
            // dfeef imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfef3 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dfef5 movzx edx,BYTE PTR [rdi+rsi*4+0x8]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x8)) as u64) as u32 as u64;
            // dfefa imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfefe add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dff00 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // dff02 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // dff05 cmp edx,0xffffff
            fl = sub::<32>((rdx as u32 as u64), 0xffffffu64).1;
            // dff0b movzx r15d,al
            r15 = ((rax as u8 as u64)) as u32 as u64;
            // dff0f mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // dff14 cmova r15d,eax
            if fl.a() { r15 = ((rax as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // dff18 movzx eax,BYTE PTR [rdi+rsi*4-0x1]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ffff)) as u64) as u32 as u64;
            // dff1d imul eax,r12d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r12 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dff21 movzx edx,BYTE PTR [rdi+rsi*4+0x3]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x3)) as u64) as u32 as u64;
            // dff26 imul edx,ecx
            { let (r, f) = imul::<32>((rdx as u32 as u64), (rcx as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dff29 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dff2b movzx eax,BYTE PTR [rdi+rsi*4+0x7]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x7)) as u64) as u32 as u64;
            // dff30 imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dff34 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dff36 movzx ecx,BYTE PTR [rdi+rsi*4+0xb]
            rcx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xb)) as u64) as u32 as u64;
            // dff3b imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // dff3f add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff41 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // dff43 shl eax,0x8
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // dff46 and eax,0xff000000
            { let r = (rax as u32 as u64) & 0xff000000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dff4b cmp ecx,0xffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffu64).1;
            // dff51 mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // dff56 cmova eax,ecx
            if fl.a() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // dff59 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // dff5b and ecx,0xff0000
            { let r = (rcx as u32 as u64) & 0xff0000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff61 cmp ebx,0xffffff
            fl = sub::<32>((rbx as u32 as u64), 0xffffffu64).1;
            // dff67 mov edx,0xff0000
            rdx = (0xff0000u64) as u32 as u64;
            // dff6c cmova ecx,edx
            if fl.a() { rcx = ((rdx as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // dff6f or ecx,r14d
            { let r = (rcx as u32 as u64) | (r14 as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff72 or ecx,r15d
            { let r = (rcx as u32 as u64) | (r15 as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff75 or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff77 mov DWORD PTR [r13+0x0],ecx
            m.w32(r13, ((rcx as u32 as u64)) as u32);
            // dff7b add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // dff7f add r11d,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // dff83 dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // dff86 jne 0xdfe1b
            if fl.ne() { pc = 0xdfe1b; continue; }
            pc = 0xdff8c;
        }
        0xdff8c => {
            // block dff8c
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dff8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dff8c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dff8d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dff8f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dff91 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dff93 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dff95 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dff96 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3708 size=505
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3708<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3708;
    loop {
        match pc {
        0xc3708 => {
            // block c3708
            #[cfg(feature = "trace")] eprintln!("f_000c3708 pc c3708 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3708 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3709 mov rbp,rsp
            rbp = rsp;
            // c370c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c370d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c370e lea rbx,[rip+0x104f03] # 0x1c8618
            rbx = m.env.data_va(904677);
            // c3715 mov rsi,QWORD PTR [rip+0x104efc] # 0x1c8618
            rsi = m.r64(m.env.data_va(904677)) as u64;
            // c371c lea rdi,[rip+0x104eed] # 0x1c8610
            rdi = m.env.data_va(904669);
            // c3723 call 0xebe34
            { let r = { let t = m.env.stub_va(0x601); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3728 mov QWORD PTR [rip+0x104eed],0x0 # 0x1c8620
            m.w64(m.env.data_va(904685), (0x0u64) as u64);
            // c3733 mov QWORD PTR [rip+0x104ed6],rbx # 0x1c8610
            m.w64(m.env.data_va(904669), (rbx) as u64);
            // c373a mov QWORD PTR [rip+0x104ed3],0x0 # 0x1c8618
            m.w64(m.env.data_va(904677), (0x0u64) as u64);
            // c3745 lea rbx,[rip+0x104eb4] # 0x1c8600
            rbx = m.env.data_va(904653);
            // c374c mov rsi,QWORD PTR [rip+0x104ead] # 0x1c8600
            rsi = m.r64(m.env.data_va(904653)) as u64;
            // c3753 lea rdi,[rip+0x104e9e] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // c375a call 0xebe70
            { let r = { let t = m.env.stub_va(0x63d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c375f mov QWORD PTR [rip+0x104e9e],0x0 # 0x1c8608
            m.w64(m.env.data_va(904661), (0x0u64) as u64);
            // c376a mov QWORD PTR [rip+0x104e87],rbx # 0x1c85f8
            m.w64(m.env.data_va(904645), (rbx) as u64);
            // c3771 mov QWORD PTR [rip+0x104e84],0x0 # 0x1c8600
            m.w64(m.env.data_va(904653), (0x0u64) as u64);
            // c377c lea rbx,[rip+0x104ead] # 0x1c8630
            rbx = m.env.data_va(904701);
            // c3783 mov rsi,QWORD PTR [rip+0x104ea6] # 0x1c8630
            rsi = m.r64(m.env.data_va(904701)) as u64;
            // c378a lea rdi,[rip+0x104e97] # 0x1c8628
            rdi = m.env.data_va(904693);
            // c3791 call 0xebdf8
            { let r = { let t = m.env.stub_va(0x5c5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3796 mov QWORD PTR [rip+0x104e97],0x0 # 0x1c8638
            m.w64(m.env.data_va(904709), (0x0u64) as u64);
            // c37a1 mov QWORD PTR [rip+0x104e80],rbx # 0x1c8628
            m.w64(m.env.data_va(904693), (rbx) as u64);
            // c37a8 mov QWORD PTR [rip+0x104e7d],0x0 # 0x1c8630
            m.w64(m.env.data_va(904701), (0x0u64) as u64);
            // c37b3 lea rbx,[rip+0x104ea6] # 0x1c8660
            rbx = m.env.data_va(904749);
            // c37ba mov rsi,QWORD PTR [rip+0x104e9f] # 0x1c8660
            rsi = m.r64(m.env.data_va(904749)) as u64;
            // c37c1 lea rdi,[rip+0x104e90] # 0x1c8658
            rdi = m.env.data_va(904741);
            // c37c8 call 0xebe58
            { let r = { let t = m.env.stub_va(0x625); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c37cd mov QWORD PTR [rip+0x104e90],0x0 # 0x1c8668
            m.w64(m.env.data_va(904757), (0x0u64) as u64);
            // c37d8 mov QWORD PTR [rip+0x104e79],rbx # 0x1c8658
            m.w64(m.env.data_va(904741), (rbx) as u64);
            // c37df mov QWORD PTR [rip+0x104e76],0x0 # 0x1c8660
            m.w64(m.env.data_va(904749), (0x0u64) as u64);
            // c37ea lea rbx,[rip+0x104e57] # 0x1c8648
            rbx = m.env.data_va(904725);
            // c37f1 mov rsi,QWORD PTR [rip+0x104e50] # 0x1c8648
            rsi = m.r64(m.env.data_va(904725)) as u64;
            // c37f8 lea rdi,[rip+0x104e41] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c37ff call 0xebe88
            { let r = { let t = m.env.stub_va(0x655); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3804 mov QWORD PTR [rip+0x104e41],0x0 # 0x1c8650
            m.w64(m.env.data_va(904733), (0x0u64) as u64);
            // c380f mov QWORD PTR [rip+0x104e2a],rbx # 0x1c8640
            m.w64(m.env.data_va(904717), (rbx) as u64);
            // c3816 mov QWORD PTR [rip+0x104e27],0x0 # 0x1c8648
            m.w64(m.env.data_va(904725), (0x0u64) as u64);
            // c3821 lea rbx,[rip+0x104e50] # 0x1c8678
            rbx = m.env.data_va(904773);
            // c3828 mov rsi,QWORD PTR [rip+0x104e49] # 0x1c8678
            rsi = m.r64(m.env.data_va(904773)) as u64;
            // c382f lea rdi,[rip+0x104e3a] # 0x1c8670
            rdi = m.env.data_va(904765);
            // c3836 call 0xebe46
            { let r = { let t = m.env.stub_va(0x613); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c383b mov QWORD PTR [rip+0x104e3a],0x0 # 0x1c8680
            m.w64(m.env.data_va(904781), (0x0u64) as u64);
            // c3846 mov QWORD PTR [rip+0x104e23],rbx # 0x1c8670
            m.w64(m.env.data_va(904765), (rbx) as u64);
            // c384d mov QWORD PTR [rip+0x104e20],0x0 # 0x1c8678
            m.w64(m.env.data_va(904773), (0x0u64) as u64);
            // c3858 lea rbx,[rip+0x104e49] # 0x1c86a8
            rbx = m.env.data_va(904821);
            // c385f mov rsi,QWORD PTR [rip+0x104e42] # 0x1c86a8
            rsi = m.r64(m.env.data_va(904821)) as u64;
            // c3866 lea rdi,[rip+0x104e33] # 0x1c86a0
            rdi = m.env.data_va(904813);
            // c386d call 0xebdce
            { let r = { let t = m.env.stub_va(0x59b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3872 mov QWORD PTR [rip+0x104e33],0x0 # 0x1c86b0
            m.w64(m.env.data_va(904829), (0x0u64) as u64);
            // c387d mov QWORD PTR [rip+0x104e1c],rbx # 0x1c86a0
            m.w64(m.env.data_va(904813), (rbx) as u64);
            // c3884 mov QWORD PTR [rip+0x104e19],0x0 # 0x1c86a8
            m.w64(m.env.data_va(904821), (0x0u64) as u64);
            // c388f lea rbx,[rip+0x104dfa] # 0x1c8690
            rbx = m.env.data_va(904797);
            // c3896 mov rsi,QWORD PTR [rip+0x104df3] # 0x1c8690
            rsi = m.r64(m.env.data_va(904797)) as u64;
            // c389d lea rdi,[rip+0x104de4] # 0x1c8688
            rdi = m.env.data_va(904789);
            // c38a4 call 0xebde6
            { let r = { let t = m.env.stub_va(0x5b3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c38a9 mov QWORD PTR [rip+0x104de4],0x0 # 0x1c8698
            m.w64(m.env.data_va(904805), (0x0u64) as u64);
            // c38b4 mov QWORD PTR [rip+0x104dcd],rbx # 0x1c8688
            m.w64(m.env.data_va(904789), (rbx) as u64);
            // c38bb mov QWORD PTR [rip+0x104dca],0x0 # 0x1c8690
            m.w64(m.env.data_va(904797), (0x0u64) as u64);
            // c38c6 mov rax,QWORD PTR [rip+0x104d3b] # 0x1c8608
            rax = m.r64(m.env.data_va(904661)) as u64;
            // c38cd or rax,QWORD PTR [rip+0x104d4c] # 0x1c8620
            { let r = rax | m.r64(m.env.data_va(904685)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38d4 or rax,QWORD PTR [rip+0x104d5d] # 0x1c8638
            { let r = rax | m.r64(m.env.data_va(904709)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38db or rax,QWORD PTR [rip+0x104dce] # 0x1c86b0
            { let r = rax | m.r64(m.env.data_va(904829)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38e2 or rax,QWORD PTR [rip+0x104d7f] # 0x1c8668
            { let r = rax | m.r64(m.env.data_va(904757)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38e9 or rax,QWORD PTR [rip+0x104d60] # 0x1c8650
            { let r = rax | m.r64(m.env.data_va(904733)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38f0 or rax,QWORD PTR [rip+0x104d89] # 0x1c8680
            { let r = rax | m.r64(m.env.data_va(904781)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38f7 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // c38fa add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c38fe pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c38ff pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3900 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8298 size=232
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8298<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc8298;
    loop {
        match pc {
        0xc8298 => {
            // block c8298
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c8298 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8298 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8299 mov rbp,rsp
            rbp = rsp;
            // c829c mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // c829e mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // c82a1 mov rsi,QWORD PTR [rbp+0x10]
            rsi = m.r64(rbp.wrapping_add(0x10)) as u64;
            // c82a5 add edi,0xffffffe6
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xffffffe6u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c82a8 cmp edi,0x3
            fl = sub::<32>((rdi as u32 as u64), 0x3u64).1;
            // c82ab jbe 0xc82c1
            if fl.be() { pc = 0xc82c1; continue; }
            pc = 0xc82ad;
        }
        0xc82ad => {
            // block c82ad
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c82ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c82ad imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c82b1 add r9,rax
            { let (r, f) = add::<64>(r9, rax);
              r9 = r;
              fl = f; }
            // c82b4 mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            // c82b7 mov rdi,r9
            rdi = r9;
            // c82ba call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c82bf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c82c0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc82c1 => {
            // block c82c1
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c82c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c82c1 lea rdx,[rip+0xa8] # 0xc8370
            rdx = m.env.code_va(0xc8370);
            // c82c8 movsxd rdi,DWORD PTR [rdx+rdi*4]
            rdi = sx::<32>(m.r32(rdx.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // c82cc add rdi,rdx
            { let (r, f) = add::<64>(rdi, rdx);
              rdi = r;
              fl = f; }
            // c82cf jmp rdi
            pc = (rdi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc82d1 => {
            // block c82d1
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c82d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c82d1 lea edx,[rcx+0x2]
            rdx = (rcx.wrapping_add(0x2)) as u32 as u64;
            // c82d4 mov edi,0xaaaaaaab
            rdi = (0xaaaaaaabu64) as u32 as u64;
            // c82d9 imul rdi,rdx
            { let (r, f) = imul::<64>(rdi, rdx); rdi = r; fl = f; }
            // c82dd shr rdi,0x21
            { let (r, f) = shr::<64>(rdi, 0x21u64, fl); rdi = r; fl = f; }
            // c82e1 imul edi,eax
            { let (r, f) = imul::<32>((rdi as u32 as u64), (rax as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // c82e4 shl edi,0x2
            { let (r, f) = shl::<32>((rdi as u32 as u64), 0x2u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c82e7 add r9,rdi
            { let (r, f) = add::<64>(r9, rdi);
              r9 = r;
              fl = f; }
            // c82ea mov rdi,rsi
            rdi = rsi;
            // c82ed mov rsi,r9
            rsi = r9;
            // c82f0 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c82f2 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c82f4 mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c82f7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c82f8 jmp 0xc75d9
            { let r = { let t = m.env.code_va(0xc75d9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc82fd => {
            // block c82fd
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c82fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c82fd mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            // c8300 mov edi,0xaaaaaaab
            rdi = (0xaaaaaaabu64) as u32 as u64;
            // c8305 imul rdi,rdx
            { let (r, f) = imul::<64>(rdi, rdx); rdi = r; fl = f; }
            // c8309 shr rdi,0x21
            { let (r, f) = shr::<64>(rdi, 0x21u64, fl); rdi = r; fl = f; }
            // c830d imul edi,eax
            { let (r, f) = imul::<32>((rdi as u32 as u64), (rax as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // c8310 add r9,rdi
            { let (r, f) = add::<64>(r9, rdi);
              r9 = r;
              fl = f; }
            // c8313 mov rdi,rsi
            rdi = rsi;
            // c8316 mov rsi,r9
            rsi = r9;
            // c8319 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c831b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c831d mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c8320 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8321 jmp 0xc772c
            { let r = { let t = m.env.code_va(0xc772c); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc8326 => {
            // block c8326
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c8326 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8326 lea edx,[rcx+0x2]
            rdx = (rcx.wrapping_add(0x2)) as u32 as u64;
            // c8329 mov edi,0xaaaaaaab
            rdi = (0xaaaaaaabu64) as u32 as u64;
            // c832e imul rdi,rdx
            { let (r, f) = imul::<64>(rdi, rdx); rdi = r; fl = f; }
            // c8332 shr rdi,0x21
            { let (r, f) = shr::<64>(rdi, 0x21u64, fl); rdi = r; fl = f; }
            // c8336 imul edi,eax
            { let (r, f) = imul::<32>((rdi as u32 as u64), (rax as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // c8339 shl edi,0x2
            { let (r, f) = shl::<32>((rdi as u32 as u64), 0x2u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c833c add r9,rdi
            { let (r, f) = add::<64>(r9, rdi);
              r9 = r;
              fl = f; }
            // c833f mov rdi,rsi
            rdi = rsi;
            // c8342 mov rsi,r9
            rsi = r9;
            // c8345 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c8347 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c8349 mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c834c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c834d jmp 0xc77be
            { let r = { let t = m.env.code_va(0xc77be); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc8352 => {
            // block c8352
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c8352 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8352 shr r8d,0x2
            { let (r, f) = shr::<32>((r8 as u32 as u64), 0x2u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // c8356 imul r8d,eax
            { let (r, f) = imul::<32>((r8 as u32 as u64), (rax as u32 as u64)); r8 = (r) as u32 as u64; fl = f; }
            // c835a add r9,r8
            { let (r, f) = add::<64>(r9, r8);
              r9 = r;
              fl = f; }
            // c835d mov rdi,rsi
            rdi = rsi;
            // c8360 mov rsi,r9
            rsi = r9;
            // c8363 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c8365 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c8367 mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c836a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c836b jmp 0xc7978
            { let r = { let t = m.env.code_va(0xc7978); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8168 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8168<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc8168;
    loop {
        match pc {
        0xc8168 => {
            // block c8168
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8168 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8168 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8169 mov rbp,rsp
            rbp = rsp;
            // c816c mov r11,r8
            r11 = r8;
            // c816f mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // c8172 add edi,0xffffffe6
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xffffffe6u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c8175 cmp edi,0x3
            fl = sub::<32>((rdi as u32 as u64), 0x3u64).1;
            // c8178 jbe 0xc8190
            if fl.be() { pc = 0xc8190; continue; }
            pc = 0xc817a;
        }
        0xc817a => {
            // block c817a
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c817a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c817a imul r10d,ecx
            { let (r, f) = imul::<32>((r10 as u32 as u64), (rcx as u32 as u64)); r10 = (r) as u32 as u64; fl = f; }
            // c817e add r11,r10
            { let (r, f) = add::<64>(r11, r10);
              r11 = r;
              fl = f; }
            // c8181 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c8183 mov rdi,r11
            rdi = r11;
            // c8186 mov rsi,r9
            rsi = r9;
            // c8189 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc818e;
        }
        0xc818e => {
            // block c818e
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c818e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c818e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c818f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc8190 => {
            // block c8190
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8190 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8190 lea rax,[rip+0xf1] # 0xc8288
            rax = m.env.code_va(0xc8288);
            // c8197 movsxd rsi,DWORD PTR [rax+rdi*4]
            rsi = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // c819b add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // c819e jmp rsi
            pc = (rsi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc81a0 => {
            // block c81a0
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81a0 lea eax,[rdx+0x2]
            rax = (rdx.wrapping_add(0x2)) as u32 as u64;
            // c81a3 mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c81a8 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c81ac shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c81b0 imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c81b4 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c81b7 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c81ba mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c81c0 mov rdi,r9
            rdi = r9;
            // c81c3 mov rsi,r11
            rsi = r11;
            // c81c6 mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c81c9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c81ca jmp 0xc75d9
            { let r = { let t = m.env.code_va(0xc75d9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc81cf => {
            // block c81cf
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81cf test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c81d1 je 0xc818e
            if fl.e() { pc = 0xc818e; continue; }
            pc = 0xc81d3;
        }
        0xc81d3 => {
            // block c81d3
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81d3 mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c81d6 and r8d,0x1
            { let r = (r8 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c81da lea rax,[r9+0x2]
            rax = r9.wrapping_add(0x2);
            // c81de mov ecx,ecx
            rcx = ((rcx as u32 as u64)) as u32 as u64;
            // c81e0 mov esi,0xaaaaaaab
            rsi = (0xaaaaaaabu64) as u32 as u64;
            // c81e5 imul rsi,rcx
            { let (r, f) = imul::<64>(rsi, rcx); rsi = r; fl = f; }
            // c81e9 shr rsi,0x21
            { let (r, f) = shr::<64>(rsi, 0x21u64, fl); rsi = r; fl = f; }
            // c81ed imul esi,r10d
            { let (r, f) = imul::<32>((rsi as u32 as u64), (r10 as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // c81f1 add r11,rsi
            { let (r, f) = add::<64>(r11, rsi);
              r11 = r;
              fl = f; }
            // c81f4 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // c81f6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xc81f8;
        }
        0xc81f8 => {
            // block c81f8
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81f8 test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // c81fb jne 0xc8206
            if fl.ne() { pc = 0xc8206; continue; }
            pc = 0xc81fd;
        }
        0xc81fd => {
            // block c81fd
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81fd lea rsi,[rax+0x2]
            rsi = rax.wrapping_add(0x2);
            // c8201 mov rdi,rax
            rdi = rax;
            // c8204 jmp 0xc820f
            pc = 0xc820f; continue;
        }
        0xc8206 => {
            // block c8206
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8206 lea rsi,[r9+rdx*8+0x2]
            rsi = r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x2);
            // c820b lea rdi,[r9+rdx*8]
            rdi = r9.wrapping_add(rdx.wrapping_mul(8));
            pc = 0xc820f;
        }
        0xc820f => {
            // block c820f
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c820f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c820f test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // c8212 cmove rdi,rsi
            if fl.e() { rdi = rsi; }
            // c8216 mov si,WORD PTR [rdi]
            rsi = (rsi & !0xffff) | ((m.r16(rdi) as u64) as u16 as u64);
            // c8219 mov WORD PTR [r11+rdx*2],si
            m.w16(r11.wrapping_add(rdx.wrapping_mul(2)), ((rsi as u16 as u64)) as u16);
            // c821e inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // c8221 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c8225 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c8227 jne 0xc81f8
            if fl.ne() { pc = 0xc81f8; continue; }
            pc = 0xc8229;
        }
        0xc8229 => {
            // block c8229
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8229 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8229 jmp 0xc818e
            pc = 0xc818e; continue;
        }
        0xc822e => {
            // block c822e
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c822e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c822e lea eax,[rdx+0x2]
            rax = (rdx.wrapping_add(0x2)) as u32 as u64;
            // c8231 mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c8236 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c823a shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c823e imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8242 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c8245 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c8248 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c824e mov rdi,r9
            rdi = r9;
            // c8251 mov rsi,r11
            rsi = r11;
            // c8254 mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c8257 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8258 jmp 0xc77be
            { let r = { let t = m.env.code_va(0xc77be); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc825d => {
            // block c825d
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c825d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c825d mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c825f mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c8264 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c8268 shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c826c imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8270 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c8273 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c8279 mov rdi,r9
            rdi = r9;
            // c827c mov rsi,r11
            rsi = r11;
            // c827f mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c8282 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8283 jmp 0xc7978
            { let r = { let t = m.env.code_va(0xc7978); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
