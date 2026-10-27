//! Mechanically lifted functions (scripts/lift_run.py), module r002. Each one is checked against the original
//! in the world harness (tests/world_r002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x636c size=530
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000636c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x636c;
    loop {
        match pc {
        0x636c => {
            // block 636c
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 636c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 636d mov rbp,rsp
            rbp = rsp;
            // 6370 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6372 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6374 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6375 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6376 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 6379 mov r15,rdi
            r15 = rdi;
            // 637c mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 6380 call 0xce76c
            { let r = { let t = m.env.code_va(0xce76c); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6385 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 6388 jne 0x63a0
            if fl.ne() { pc = 0x63a0; continue; }
            pc = 0x638a;
        }
        0x638a => {
            // block 638a
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 638a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 638d mov rdi,r15
            rdi = r15;
            // 6390 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 6393 call QWORD PTR [rax+0xf28]
            { let r = { let t = m.r64(rax.wrapping_add(0xf28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6399;
        }
        0x6399 => {
            // block 6399
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6399 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6399 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 639b jmp 0x6571
            pc = 0x6571; continue;
        }
        0x63a0 => {
            // block 63a0
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a0 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 63a2 cmp r14d,0x9
            fl = sub::<32>((r14 as u32 as u64), 0x9u64).1;
            // 63a6 ja 0x6571
            if fl.a() { pc = 0x6571; continue; }
            pc = 0x63ac;
        }
        0x63ac => {
            // block 63ac
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ac mov eax,0xf
            rax = (0xfu64) as u32 as u64;
            // 63b1 bt eax,r14d
            fl.cf = bit::<32>((rax as u32 as u64), (r14 as u32 as u64));
            // 63b5 jb 0x6451
            if fl.b() { pc = 0x6451; continue; }
            pc = 0x63bb;
        }
        0x63bb => {
            // block 63bb
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bb mov eax,0xf0
            rax = (0xf0u64) as u32 as u64;
            // 63c0 bt eax,r14d
            fl.cf = bit::<32>((rax as u32 as u64), (r14 as u32 as u64));
            // 63c4 jae 0x64e7
            if fl.ae() { pc = 0x64e7; continue; }
            pc = 0x63ca;
        }
        0x63ca => {
            // block 63ca
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ca mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 63cd mov rdi,r15
            rdi = r15;
            // 63d0 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d6 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 63d8 call 0xd0489
            { let r = { let t = m.env.code_va(0xd0489); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63dd movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 63e0 cmp eax,0x5
            fl = sub::<32>((rax as u32 as u64), 0x5u64).1;
            // 63e3 je 0x63f8
            if fl.e() { pc = 0x63f8; continue; }
            pc = 0x63e5;
        }
        0x63e5 => {
            // block 63e5
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63e5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 63e8 mov rdi,r15
            rdi = r15;
            // 63eb call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63f1 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // 63f6 jne 0x641d
            if fl.ne() { pc = 0x641d; continue; }
            pc = 0x63f8;
        }
        0x63f8 => {
            // block 63f8
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 63f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63f8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 63fb xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 63fd mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 6402 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6404 mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            // 6409 mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // 640f mov rdi,r15
            rdi = r15;
            // 6412 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6415 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6417 je 0x6571
            if fl.e() { pc = 0x6571; continue; }
            pc = 0x641d;
        }
        0x641d => {
            // block 641d
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 641d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 641d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6420 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 6425 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 642a mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 642f mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 6435 mov rdi,r15
            rdi = r15;
            // 6438 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 643b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 643d je 0x656f
            if fl.e() { pc = 0x656f; continue; }
            pc = 0x6443;
        }
        0x6443 => {
            // block 6443
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6443 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6443 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6446 add r14d,0xfffffffc
            { let (r, f) = add::<32>((r14 as u32 as u64), 0xfffffffcu64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 644a mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 644f jmp 0x64ce
            pc = 0x64ce; continue;
        }
        0x6451 => {
            // block 6451
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6451 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6451 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6454 mov rdi,r15
            rdi = r15;
            // 6457 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 645d mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 645f call 0xd0489
            { let r = { let t = m.env.code_va(0xd0489); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6464 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 6467 cmp eax,0x5
            fl = sub::<32>((rax as u32 as u64), 0x5u64).1;
            // 646a je 0x647f
            if fl.e() { pc = 0x647f; continue; }
            pc = 0x646c;
        }
        0x646c => {
            // block 646c
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 646c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 646c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 646f mov rdi,r15
            rdi = r15;
            // 6472 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6478 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // 647d jne 0x64a4
            if fl.ne() { pc = 0x64a4; continue; }
            pc = 0x647f;
        }
        0x647f => {
            // block 647f
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 647f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 647f mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6482 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 6484 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 6489 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 648b mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            // 6490 mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // 6496 mov rdi,r15
            rdi = r15;
            // 6499 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 649c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 649e je 0x6571
            if fl.e() { pc = 0x6571; continue; }
            pc = 0x64a4;
        }
        0x64a4 => {
            // block 64a4
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 64a7 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 64a9 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 64ae xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 64b0 mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 64b5 mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 64bb mov rdi,r15
            rdi = r15;
            // 64be call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64c1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 64c3 je 0x6571
            if fl.e() { pc = 0x6571; continue; }
            pc = 0x64c9;
        }
        0x64c9 => {
            // block 64c9
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64c9 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 64cc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x64ce;
        }
        0x64ce => {
            // block 64ce
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ce mov ecx,0xc0000000
            rcx = (0xc0000000u64) as u32 as u64;
            // 64d3 mov r8d,0x1e
            r8 = (0x1eu64) as u32 as u64;
            pc = 0x64d9;
        }
        0x64d9 => {
            // block 64d9
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64d9 mov rdi,r15
            rdi = r15;
            // 64dc mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 64df call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64e2 jmp 0x6399
            pc = 0x6399; continue;
        }
        0x64e7 => {
            // block 64e7
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64e7 mov eax,0x300
            rax = (0x300u64) as u32 as u64;
            // 64ec bt eax,r14d
            fl.cf = bit::<32>((rax as u32 as u64), (r14 as u32 as u64));
            // 64f0 jae 0x6571
            if fl.ae() { pc = 0x6571; continue; }
            pc = 0x64f2;
        }
        0x64f2 => {
            // block 64f2
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 64f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64f2 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 64f5 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 64f7 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 64fc xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 64fe mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 6503 mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 6509 mov rdi,r15
            rdi = r15;
            // 650c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 650f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6511 je 0x6571
            if fl.e() { pc = 0x6571; continue; }
            pc = 0x6513;
        }
        0x6513 => {
            // block 6513
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6513 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6513 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6516 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 6518 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 651a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 651c mov ecx,0xc0000000
            rcx = (0xc0000000u64) as u32 as u64;
            // 6521 mov r8d,0x1e
            r8 = (0x1eu64) as u32 as u64;
            // 6527 mov rdi,r15
            rdi = r15;
            // 652a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 652d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 652f je 0x6571
            if fl.e() { pc = 0x6571; continue; }
            pc = 0x6531;
        }
        0x6531 => {
            // block 6531
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6531 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6531 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6534 mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 6539 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 653e mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            // 6543 mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // 6549 mov rdi,r15
            rdi = r15;
            // 654c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 654f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6551 je 0x656f
            if fl.e() { pc = 0x656f; continue; }
            pc = 0x6553;
        }
        0x6553 => {
            // block 6553
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6553 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6553 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6556 add r14d,0xfffffff8
            { let (r, f) = add::<32>((r14 as u32 as u64), 0xfffffff8u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 655a mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            // 655f mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 6564 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 656a jmp 0x64d9
            pc = 0x64d9; continue;
        }
        0x656f => {
            // block 656f
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 656f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 656f xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x6571;
        }
        0x6571 => {
            // block 6571
            #[cfg(feature = "trace")] eprintln!("f_0000636c pc 6571 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6571 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 6573 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6577 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6578 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 657a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 657c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 657d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7d3c size=644
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007d3c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x7d3c;
    loop {
        match pc {
        0x7d3c => {
            // block 7d3c
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d3c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7d3d mov rbp,rsp
            rbp = rsp;
            // 7d40 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7d42 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7d44 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7d46 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7d48 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7d49 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7d4d mov r12,rsi
            r12 = rsi;
            // 7d50 mov r13,rdi
            r13 = rdi;
            // 7d53 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7d57 mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 7d5c sub rsi,rax
            { let (r, f) = sub::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 7d5f sar rsi,0x3
            { let (r, f) = sar::<64>(rsi, 0x3u64, fl); rsi = r; fl = f; }
            // 7d63 cmp rsi,0x400
            fl = sub::<64>(rsi, 0x400u64).1;
            // 7d6a jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7d70;
        }
        0x7d70 => {
            // block 7d70
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d70 mov rsi,QWORD PTR [rdx+0x8]
            rsi = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 7d74 sub rsi,QWORD PTR [rdx]
            { let (r, f) = sub::<64>(rsi, m.r64(rdx) as u64);
              rsi = r;
              fl = f; }
            // 7d77 sar rsi,0x3
            { let (r, f) = sar::<64>(rsi, 0x3u64, fl); rsi = r; fl = f; }
            // 7d7b cmp rsi,0x400
            fl = sub::<64>(rsi, 0x400u64).1;
            // 7d82 jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7d88;
        }
        0x7d88 => {
            // block 7d88
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d88 mov rsi,rcx
            rsi = rcx;
            // 7d8b mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 7d8f sub rcx,QWORD PTR [rsi]
            { let (r, f) = sub::<64>(rcx, m.r64(rsi) as u64);
              rcx = r;
              fl = f; }
            // 7d92 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 7d96 cmp rcx,0x400
            fl = sub::<64>(rcx, 0x400u64).1;
            // 7d9d jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7da3;
        }
        0x7da3 => {
            // block 7da3
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7da3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7da3 mov QWORD PTR [rbp-0x30],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdx) as u64);
            // 7da7 mov QWORD PTR [rbp-0x38],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rsi) as u64);
            // 7dab mov ebx,0x8
            rbx = (0x8u64) as u32 as u64;
            // 7db0 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 7db6 movsd xmm1,QWORD PTR [rip+0xf6b1a] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7dbe mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            // 7dc4 jmp 0x7dd9
            pc = 0x7dd9; continue;
        }
        0x7dc6 => {
            // block 7dc6
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc6 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7dca add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 7dce inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 7dd1 movsd xmm1,QWORD PTR [rip+0xf6aff] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            pc = 0x7dd9;
        }
        0x7dd9 => {
            // block 7dd9
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd9 movsd xmm0,QWORD PTR [rax+rbx*1-0x8]
            x0 = m.r64(rax.wrapping_add(rbx).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7ddf addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7de3 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7de8 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7dea mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7def js 0x7dfd
            if fl.s() { pc = 0x7dfd; continue; }
            pc = 0x7df1;
        }
        0x7df1 => {
            // block 7df1
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7df1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7df1 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7df7 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7dfb mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7dfd;
        }
        0x7dfd => {
            // block 7dfd
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dfd lea rsi,[r14+0x1ff]
            rsi = r14.wrapping_add(0x1ff);
            // 7e04 movsd xmm0,QWORD PTR [rax+rbx*1]
            x0 = m.r64(rax.wrapping_add(rbx)) as u128;
            // 7e09 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7e0d cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7e12 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7e14 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7e16 js 0x7e24
            if fl.s() { pc = 0x7e24; continue; }
            pc = 0x7e18;
        }
        0x7e18 => {
            // block 7e18
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e18 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7e1e cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7e22 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7e24;
        }
        0x7e24 => {
            // block 7e24
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e24 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7e27 shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7e2a add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e2c mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7e30 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7e35 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7e38 mov rdi,r13
            rdi = r13;
            // 7e3b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7e3e cmp r14,0x1ff
            fl = sub::<64>(r14, 0x1ffu64).1;
            // 7e45 ja 0x7e4f
            if fl.a() { pc = 0x7e4f; continue; }
            pc = 0x7e47;
        }
        0x7e47 => {
            // block 7e47
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e47 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7e49 jne 0x7dc6
            if fl.ne() { pc = 0x7dc6; continue; }
            pc = 0x7e4f;
        }
        0x7e4f => {
            // block 7e4f
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e4f mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // 7e55 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 7e5a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7e5c je 0x7efc
            if fl.e() { pc = 0x7efc; continue; }
            pc = 0x7e62;
        }
        0x7e62 => {
            // block 7e62
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e62 mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            // 7e68 mov r12,QWORD PTR [rbp-0x38]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7e6c;
        }
        0x7e6c => {
            // block 7e6c
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e6c mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7e70 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 7e73 movsd xmm0,QWORD PTR [rax+r14*1-0x8]
            x0 = m.r64(rax.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7e7a movsd xmm1,QWORD PTR [rip+0xf6a56] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7e82 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7e86 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7e8b test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7e8d mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7e92 js 0x7ea0
            if fl.s() { pc = 0x7ea0; continue; }
            pc = 0x7e94;
        }
        0x7e94 => {
            // block 7e94
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e94 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7e9a cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7e9e mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7ea0;
        }
        0x7ea0 => {
            // block 7ea0
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ea0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ea0 lea rsi,[rbx+0x3ff]
            rsi = rbx.wrapping_add(0x3ff);
            // 7ea7 movsd xmm0,QWORD PTR [rax+r14*1]
            x0 = m.r64(rax.wrapping_add(r14)) as u128;
            // 7ead addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7eb1 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7eb6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7eb8 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7eba js 0x7ec8
            if fl.s() { pc = 0x7ec8; continue; }
            pc = 0x7ebc;
        }
        0x7ebc => {
            // block 7ebc
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ebc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ebc cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7ec2 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7ec6 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7ec8;
        }
        0x7ec8 => {
            // block 7ec8
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ec8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ec8 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7ecb shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7ece add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ed0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7ed4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7ed9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7edc mov rdi,r13
            rdi = r13;
            // 7edf call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7ee2 cmp rbx,0x1ff
            fl = sub::<64>(rbx, 0x1ffu64).1;
            // 7ee9 ja 0x7f00
            if fl.a() { pc = 0x7f00; continue; }
            pc = 0x7eeb;
        }
        0x7eeb => {
            // block 7eeb
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7eeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7eeb add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // 7eef inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7ef2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7ef4 jne 0x7e6c
            if fl.ne() { pc = 0x7e6c; continue; }
            pc = 0x7efa;
        }
        0x7efa => {
            // block 7efa
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7efa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7efa jmp 0x7f00
            pc = 0x7f00; continue;
        }
        0x7efc => {
            // block 7efc
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7efc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7efc mov r12,QWORD PTR [rbp-0x38]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7f00;
        }
        0x7f00 => {
            // block 7f00
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f00 movsd xmm1,QWORD PTR [rip+0xf69d0] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7f08 mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // 7f0e mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 7f13 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7f15 je 0x7fae
            if fl.e() { pc = 0x7fae; continue; }
            pc = 0x7f1b;
        }
        0x7f1b => {
            // block 7f1b
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f1b mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            pc = 0x7f21;
        }
        0x7f21 => {
            // block 7f21
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f21 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7f25 movsd xmm0,QWORD PTR [rax+r14*1-0x8]
            x0 = m.r64(rax.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7f2c addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7f30 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7f35 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7f37 mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7f3c js 0x7f4a
            if fl.s() { pc = 0x7f4a; continue; }
            pc = 0x7f3e;
        }
        0x7f3e => {
            // block 7f3e
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f3e cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7f44 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7f48 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7f4a;
        }
        0x7f4a => {
            // block 7f4a
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f4a lea rsi,[rbx+0x5ff]
            rsi = rbx.wrapping_add(0x5ff);
            // 7f51 movsd xmm0,QWORD PTR [rax+r14*1]
            x0 = m.r64(rax.wrapping_add(r14)) as u128;
            // 7f57 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7f5b cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7f60 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7f62 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7f64 js 0x7f72
            if fl.s() { pc = 0x7f72; continue; }
            pc = 0x7f66;
        }
        0x7f66 => {
            // block 7f66
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f66 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7f6c cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7f70 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7f72;
        }
        0x7f72 => {
            // block 7f72
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f72 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7f75 shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7f78 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f7a mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7f7e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7f83 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7f86 mov rdi,r13
            rdi = r13;
            // 7f89 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7f8c cmp rbx,0x1ff
            fl = sub::<64>(rbx, 0x1ffu64).1;
            // 7f93 ja 0x7fb0
            if fl.a() { pc = 0x7fb0; continue; }
            pc = 0x7f95;
        }
        0x7f95 => {
            // block 7f95
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f95 add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // 7f99 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7f9c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7f9e movsd xmm1,QWORD PTR [rip+0xf6932] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7fa6 jne 0x7f21
            if fl.ne() { pc = 0x7f21; continue; }
            pc = 0x7fac;
        }
        0x7fac => {
            // block 7fac
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fac jmp 0x7fb0
            pc = 0x7fb0; continue;
        }
        0x7fae => {
            // block 7fae
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fae xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x7fb0;
        }
        0x7fb0 => {
            // block 7fb0
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fb0 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7fb4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7fb5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7fb7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7fb9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7fbb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7fbd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7fbe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7fc0 size=496
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007fc0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x7fc0;
    loop {
        match pc {
        0x7fc0 => {
            // block 7fc0
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 7fc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fc0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7fc1 mov rbp,rsp
            rbp = rsp;
            // 7fc4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7fc6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7fc8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7fca push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7fcc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7fcd sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 7fd1 mov r14,rcx
            r14 = rcx;
            // 7fd4 mov r15,rdx
            r15 = rdx;
            // 7fd7 mov rbx,rsi
            rbx = rsi;
            // 7fda mov r13,rdi
            r13 = rdi;
            // 7fdd mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 7fe4 mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 7fe7 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 7feb cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 7fee je 0x8005
            if fl.e() { pc = 0x8005; continue; }
            pc = 0x7ff0;
        }
        0x7ff0 => {
            // block 7ff0
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 7ff0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ff0 lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 7ff4 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 7ff7 not rdx
            { let v = !rdx; rdx = v; }
            // 7ffa and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 7ffe add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 8001 mov QWORD PTR [rbx+0x8],rdx
            m.w64(rbx.wrapping_add(0x8), (rdx) as u64);
            pc = 0x8005;
        }
        0x8005 => {
            // block 8005
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8005 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8005 mov esi,0x400
            rsi = (0x400u64) as u32 as u64;
            // 800a mov rdi,rbx
            rdi = rbx;
            // 800d mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 8011 call 0xeb89a
            { let r = { let t = m.env.stub_va(0x67); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8016 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 8019 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 801d cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 8020 je 0x8037
            if fl.e() { pc = 0x8037; continue; }
            pc = 0x8022;
        }
        0x8022 => {
            // block 8022
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8022 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8022 lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 8026 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 8029 not rdx
            { let v = !rdx; rdx = v; }
            // 802c and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 8030 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 8033 mov QWORD PTR [r15+0x8],rdx
            m.w64(r15.wrapping_add(0x8), (rdx) as u64);
            pc = 0x8037;
        }
        0x8037 => {
            // block 8037
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8037 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8037 mov esi,0x400
            rsi = (0x400u64) as u32 as u64;
            // 803c mov rdi,r15
            rdi = r15;
            // 803f mov QWORD PTR [rbp-0x40],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r15) as u64);
            // 8043 call 0xeb89a
            { let r = { let t = m.env.stub_va(0x67); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8048 mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 804b mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 804f cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 8052 je 0x8069
            if fl.e() { pc = 0x8069; continue; }
            pc = 0x8054;
        }
        0x8054 => {
            // block 8054
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8054 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8054 lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 8058 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 805b not rdx
            { let v = !rdx; rdx = v; }
            // 805e and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 8062 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 8065 mov QWORD PTR [r14+0x8],rdx
            m.w64(r14.wrapping_add(0x8), (rdx) as u64);
            pc = 0x8069;
        }
        0x8069 => {
            // block 8069
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8069 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8069 mov esi,0x400
            rsi = (0x400u64) as u32 as u64;
            // 806e mov rdi,r14
            rdi = r14;
            // 8071 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            // 8075 call 0xeb89a
            { let r = { let t = m.env.stub_va(0x67); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 807a mov ebx,0x200
            rbx = (0x200u64) as u32 as u64;
            // 807f mov r15d,0x8
            r15 = (0x8u64) as u32 as u64;
            // 8085 mov r12d,0x8
            r12 = (0x8u64) as u32 as u64;
            // 808b mov r14,QWORD PTR [rbp-0x48]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0x808f;
        }
        0x808f => {
            // block 808f
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 808f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 808f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 8093 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8098 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 809b mov rdi,r13
            rdi = r13;
            // 809e mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 80a0 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 80a4 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 80a7 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 80aa mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 80ac shr ecx,0x6
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x6u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 80af and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 80b5 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // 80b9 mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 80bc movsd QWORD PTR [rcx+r12*1-0x8],xmm0
            m.w64(rcx.wrapping_add(r12).wrapping_add(0xffff_ffff_ffff_fff8), x0 as u64);
            // 80c3 shr eax,0x16
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 80c6 xorps xmm0,xmm0
            x0 = 0;
            // 80c9 cvtsi2sd xmm0,eax
            x0 = set_lo64(x0, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 80cd movsd QWORD PTR [rcx+r12*1],xmm0
            m.w64(rcx.wrapping_add(r12), x0 as u64);
            // 80d3 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 80d5 add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // 80d9 cmp ebx,0x400
            fl = sub::<32>((rbx as u32 as u64), 0x400u64).1;
            // 80df jne 0x808f
            if fl.ne() { pc = 0x808f; continue; }
            pc = 0x80e1;
        }
        0x80e1 => {
            // block 80e1
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 80e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80e1 mov ebx,0x400
            rbx = (0x400u64) as u32 as u64;
            // 80e6 mov r12d,0x8
            r12 = (0x8u64) as u32 as u64;
            // 80ec mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            pc = 0x80f0;
        }
        0x80f0 => {
            // block 80f0
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 80f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 80f0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 80f4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 80f9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 80fc mov rdi,r13
            rdi = r13;
            // 80ff mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 8101 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 8105 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8108 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 810b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 810d shr ecx,0x6
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x6u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 8110 and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 8116 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // 811a mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 811d movsd QWORD PTR [rcx+r15*1-0x8],xmm0
            m.w64(rcx.wrapping_add(r15).wrapping_add(0xffff_ffff_ffff_fff8), x0 as u64);
            // 8124 shr eax,0x16
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8127 xorps xmm0,xmm0
            x0 = 0;
            // 812a cvtsi2sd xmm0,eax
            x0 = set_lo64(x0, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 812e movsd QWORD PTR [rcx+r15*1],xmm0
            m.w64(rcx.wrapping_add(r15), x0 as u64);
            // 8134 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 8136 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // 813a cmp ebx,0x600
            fl = sub::<32>((rbx as u32 as u64), 0x600u64).1;
            // 8140 jne 0x80f0
            if fl.ne() { pc = 0x80f0; continue; }
            pc = 0x8142;
        }
        0x8142 => {
            // block 8142
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 8142 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8142 mov ebx,0x600
            rbx = (0x600u64) as u32 as u64;
            // 8147 lea r14,[rbp-0x2c]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 814b mov r15,QWORD PTR [rbp-0x38]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x814f;
        }
        0x814f => {
            // block 814f
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 814f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 814f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 8153 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8158 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 815b mov rdi,r13
            rdi = r13;
            // 815e mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 8160 mov rdx,r14
            rdx = r14;
            // 8163 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8166 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 8169 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 816b shr ecx,0x6
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x6u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 816e and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 8174 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // 8178 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 817b movsd QWORD PTR [rcx+r12*1-0x8],xmm0
            m.w64(rcx.wrapping_add(r12).wrapping_add(0xffff_ffff_ffff_fff8), x0 as u64);
            // 8182 shr eax,0x16
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8185 xorps xmm0,xmm0
            x0 = 0;
            // 8188 cvtsi2sd xmm0,eax
            x0 = set_lo64(x0, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 818c movsd QWORD PTR [rcx+r12*1],xmm0
            m.w64(rcx.wrapping_add(r12), x0 as u64);
            // 8192 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 8194 add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // 8198 cmp ebx,0x800
            fl = sub::<32>((rbx as u32 as u64), 0x800u64).1;
            // 819e jne 0x814f
            if fl.ne() { pc = 0x814f; continue; }
            pc = 0x81a0;
        }
        0x81a0 => {
            // block 81a0
            #[cfg(feature = "trace")] eprintln!("f_00007fc0 pc 81a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 81a0 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 81a4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 81a5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 81a7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 81a9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 81ab pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 81ad pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 81ae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11344 size=502
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011344<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x11344;
    loop {
        match pc {
        0x11344 => {
            // block 11344
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11344 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11344 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11345 mov rbp,rsp
            rbp = rsp;
            // 11348 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1134a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1134c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1134e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 11350 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11351 sub rsp,0x78
            { let (r, f) = sub::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // 11355 mov rbx,rdi
            rbx = rdi;
            // 11358 mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 1135c xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 1135f test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 11362 je 0x114ff
            if fl.e() { pc = 0x114ff; continue; }
            pc = 0x11368;
        }
        0x11368 => {
            // block 11368
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11368 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11368 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 11370 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 11378 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1137b lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 1137f call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11382 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 11384 jne 0x114ff
            if fl.ne() { pc = 0x114ff; continue; }
            pc = 0x1138a;
        }
        0x1138a => {
            // block 1138a
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 1138a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1138a mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 1138e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 11391 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 11395 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11398 mov r14d,0xffffffff
            r14 = (0xffffffffu64) as u32 as u64;
            // 1139e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 113a0 jne 0x114e8
            if fl.ne() { pc = 0x114e8; continue; }
            pc = 0x113a6;
        }
        0x113a6 => {
            // block 113a6
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 113a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 113a6 lea rax,[rbx+0x8]
            rax = rbx.wrapping_add(0x8);
            // 113aa mov QWORD PTR [rbp-0xa0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (rax) as u64);
            // 113b1 lea r14,[rbp-0x90]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            pc = 0x113b8;
        }
        0x113b8 => {
            // block 113b8
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 113b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 113b8 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 113bc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 113bf lea rsi,[rbp-0x48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 113c3 lea rdx,[rbp-0x50]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 113c7 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 113ca cvtsi2sd xmm1,QWORD PTR [rbp-0x50]
            x1 = set_lo64(x1, (sx::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64) as f64).to_bits());
            // 113d0 cvtsi2sd xmm0,QWORD PTR [rbp-0x48]
            x0 = set_lo64(x0, (sx::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as f64).to_bits());
            // 113d6 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 113da movsd QWORD PTR [rbp-0x98],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), x1 as u64);
            // 113e2 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 113e6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 113e9 lea rsi,[rbp-0x58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 113ed call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 113f0 mov rdi,QWORD PTR [rbp-0x58]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 113f4 call 0x1153a
            { let r = { let t = m.env.code_va(0x1153a); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 113f9 mov r12,rax
            r12 = rax;
            // 113fc mov r13,QWORD PTR [rbp-0x40]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 11400 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 11404 mov rdi,r13
            rdi = r13;
            // 11407 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1140a mov r15,rax
            r15 = rax;
            // 1140d mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 11411 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 11414 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11417 mov rdi,r14
            rdi = r14;
            // 1141a mov rsi,rbx
            rsi = rbx;
            // 1141d mov rdx,r12
            rdx = r12;
            // 11420 mov rcx,r13
            rcx = r13;
            // 11423 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // 11426 mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // 11429 movsd xmm0,QWORD PTR [rip+0xee757] # 0xffb88
            x0 = m.r64(m.env.data_va(82773)) as u128;
            // 11431 movsd xmm1,QWORD PTR [rbp-0x98]
            x1 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u128;
            // 11439 call 0xe0562
            { let r = { let t = m.env.code_va(0xe0562); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1143e mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 11442 cmp rdi,QWORD PTR [rbx+0x18]
            fl = sub::<64>(rdi, m.r64(rbx.wrapping_add(0x18)) as u64).1;
            // 11446 je 0x11457
            if fl.e() { pc = 0x11457; continue; }
            pc = 0x11448;
        }
        0x11448 => {
            // block 11448
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11448 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11448 mov rsi,r14
            rsi = r14;
            // 1144b call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11450 add QWORD PTR [rbx+0x10],0x38
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, 0x38u64);
              m.w64(rbx.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 11455 jmp 0x11466
            pc = 0x11466; continue;
        }
        0x11457 => {
            // block 11457
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11457 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11457 mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 1145e mov rsi,r14
            rsi = r14;
            // 11461 call 0xeb8ee
            { let r = { let t = m.env.stub_va(0xbb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x11466;
        }
        0x11466 => {
            // block 11466
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11466 mov rdi,r14
            rdi = r14;
            // 11469 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1146e mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 11472 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 11475 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 11479 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1147c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 1147e je 0x113b8
            if fl.e() { pc = 0x113b8; continue; }
            pc = 0x11484;
        }
        0x11484 => {
            // block 11484
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11484 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11484 mov r14,QWORD PTR [rbx+0x8]
            r14 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 11488 mov r15,QWORD PTR [rbx+0x10]
            r15 = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 1148c mov rax,r15
            rax = r15;
            // 1148f sub rax,r14
            { let (r, f) = sub::<64>(rax, r14);
              rax = r;
              fl = f; }
            // 11492 mov rcx,rax
            rcx = rax;
            // 11495 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 11499 movabs rbx,0x6db6db6db6db6db7
            rbx = 0x6db6_db6d_b6db_6db7u64;
            // 114a3 imul rbx,rcx
            { let (r, f) = imul::<64>(rbx, rcx); rbx = r; fl = f; }
            // 114a7 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // 114aa test rax,rax
            fl = logic::<64>(rax & rax);
            // 114ad mov r12d,0x0
            r12 = (0x0u64) as u32 as u64;
            // 114b3 jle 0x114c3
            if fl.le() { pc = 0x114c3; continue; }
            pc = 0x114b5;
        }
        0x114b5 => {
            // block 114b5
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114b5 mov rdi,rbx
            rdi = rbx;
            // 114b8 call 0xeb8d6
            { let r = { let t = m.env.stub_va(0xa3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 114bd mov r12,rax
            r12 = rax;
            // 114c0 mov r9,rdx
            r9 = rdx;
            pc = 0x114c3;
        }
        0x114c3 => {
            // block 114c3
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114c3 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 114c7 mov rdi,r14
            rdi = r14;
            // 114ca mov rsi,r15
            rsi = r15;
            // 114cd mov rcx,rbx
            rcx = rbx;
            // 114d0 mov r8,r12
            r8 = r12;
            // 114d3 call 0xeb8a6
            { let r = { let t = m.env.stub_va(0x73); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 114d8 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 114db je 0x114e5
            if fl.e() { pc = 0x114e5; continue; }
            pc = 0x114dd;
        }
        0x114dd => {
            // block 114dd
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114dd mov rdi,r12
            rdi = r12;
            // 114e0 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x114e5;
        }
        0x114e5 => {
            // block 114e5
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114e5 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x114e8;
        }
        0x114e8 => {
            // block 114e8
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114e8 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 114ec test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 114ef je 0x114ff
            if fl.e() { pc = 0x114ff; continue; }
            pc = 0x114f1;
        }
        0x114f1 => {
            // block 114f1
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114f1 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 114f4 call QWORD PTR [rax+0x10]
            { let r = { let t = m.r64(rax.wrapping_add(0x10)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 114f7 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            pc = 0x114ff;
        }
        0x114ff => {
            // block 114ff
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 114ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114ff mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 11502 add rsp,0x78
            { let (r, f) = add::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // 11506 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11507 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11509 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1150b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1150d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1150f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11510 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x11511 => {
            // block 11511
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11511 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11511 mov rbx,rax
            rbx = rax;
            // 11514 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 1151b call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x11520;
        }
        0x11520 => {
            // block 11520
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11520 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11520 mov rdi,rbx
            rdi = rbx;
            // 11523 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11528 mov rbx,rax
            rbx = rax;
            // 1152b test r12,r12
            fl = logic::<64>(r12 & r12);
            // 1152e je 0x11520
            if fl.e() { pc = 0x11520; continue; }
            pc = 0x11530;
        }
        0x11530 => {
            // block 11530
            #[cfg(feature = "trace")] eprintln!("f_00011344 pc 11530 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11530 mov rdi,r12
            rdi = r12;
            // 11533 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 11538 jmp 0x11520
            pc = 0x11520; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x12c80 size=520
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_00012c80<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x12c80;
    loop {
        match pc {
        0x12c80 => {
            // block 12c80
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12c80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12c80 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12c81 mov rbp,rsp
            rbp = rsp;
            // 12c84 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12c86 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12c88 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12c8a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12c8c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12c8d sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 12c94 mov r15,rsi
            r15 = rsi;
            // 12c97 mov rbx,rdi
            rbx = rdi;
            // 12c9a mov r12,QWORD PTR [rip+0x12d737] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // 12ca1 mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // 12ca5 mov QWORD PTR [rbp-0x30],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r12) as u64);
            // 12ca9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 12cac call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12caf test BYTE PTR [r15],0x1
            fl = logic::<8>(m.r8(r15) as u64 & 0x1u64);
            // 12cb3 jne 0x12cbb
            if fl.ne() { pc = 0x12cbb; continue; }
            pc = 0x12cb5;
        }
        0x12cb5 => {
            // block 12cb5
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12cb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12cb5 lea rdi,[r15+0x1]
            rdi = r15.wrapping_add(0x1);
            // 12cb9 jmp 0x12cbf
            pc = 0x12cbf; continue;
        }
        0x12cbb => {
            // block 12cbb
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12cbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12cbb mov rdi,QWORD PTR [r15+0x10]
            rdi = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x12cbf;
        }
        0x12cbf => {
            // block 12cbf
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12cbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12cbf lea rsi,[rbp-0x148]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 12cc6 call 0xec2a2
            { let r = { let t = m.env.stub_va(0xa6f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12ccb mov eax,DWORD PTR [rbp-0xe8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff18)) as u64) as u32 as u64;
            // 12cd1 mov DWORD PTR [rbx+0x2e4],eax
            m.w32(rbx.wrapping_add(0x2e4), ((rax as u32 as u64)) as u32);
            // 12cd7 lea r14,[rbx+0x8]
            r14 = rbx.wrapping_add(0x8);
            // 12cdb test BYTE PTR [r15],0x1
            fl = logic::<8>(m.r8(r15) as u64 & 0x1u64);
            // 12cdf jne 0x12ce6
            if fl.ne() { pc = 0x12ce6; continue; }
            pc = 0x12ce1;
        }
        0x12ce1 => {
            // block 12ce1
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12ce1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ce1 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 12ce4 jmp 0x12cea
            pc = 0x12cea; continue;
        }
        0x12ce6 => {
            // block 12ce6
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12ce6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ce6 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x12cea;
        }
        0x12cea => {
            // block 12cea
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12cea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12cea mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 12cef mov rdi,r14
            rdi = r14;
            // 12cf2 mov rsi,r15
            rsi = r15;
            // 12cf5 call 0xeb918
            { let r = { let t = m.env.stub_va(0xe5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12cfa mov DWORD PTR [rbp-0x14c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feb4), (0x0u64) as u32);
            // 12d04 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 12d07 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 12d0b test BYTE PTR [rax+r14*1+0x20],0x5
            fl = logic::<8>(m.r8(rax.wrapping_add(r14).wrapping_add(0x20)) as u64 & 0x5u64);
            // 12d11 je 0x12d2b
            if fl.e() { pc = 0x12d2b; continue; }
            pc = 0x12d13;
        }
        0x12d13 => {
            // block 12d13
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d13 lea rdi,[rbx+0x2c8]
            rdi = rbx.wrapping_add(0x2c8);
            // 12d1a lea rsi,[rip+0x1172d3] # 0x129ff4
            rsi = m.env.data_va(255937);
            // 12d21 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12d26 jmp 0x12e5c
            pc = 0x12e5c; continue;
        }
        0x12d2b => {
            // block 12d2b
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d2b lea r15,[rbx+0x248]
            r15 = rbx.wrapping_add(0x248);
            // 12d32 mov r13d,0xb7
            r13 = (0xb7u64) as u32 as u64;
            // 12d38 lea r12,[rbp-0x14d]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_feb3);
            pc = 0x12d3f;
        }
        0x12d3f => {
            // block 12d3f
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d3f mov rdi,r14
            rdi = r14;
            // 12d42 call 0xec03e
            { let r = { let t = m.env.stub_va(0x80b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12d47 mov BYTE PTR [rbp-0x14d],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_feb3), ((rax as u8 as u64)) as u8);
            // 12d4d mov rcx,QWORD PTR [rbx+0x250]
            rcx = m.r64(rbx.wrapping_add(0x250)) as u64;
            // 12d54 cmp rcx,QWORD PTR [rbx+0x258]
            fl = sub::<64>(rcx, m.r64(rbx.wrapping_add(0x258)) as u64).1;
            // 12d5b jae 0x12d68
            if fl.ae() { pc = 0x12d68; continue; }
            pc = 0x12d5d;
        }
        0x12d5d => {
            // block 12d5d
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d5d mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // 12d5f inc QWORD PTR [rbx+0x250]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x250)) as u64, fl); m.w64(rbx.wrapping_add(0x250), (r) as u64); fl = f; }
            // 12d66 jmp 0x12d73
            pc = 0x12d73; continue;
        }
        0x12d68 => {
            // block 12d68
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d68 mov rdi,r15
            rdi = r15;
            // 12d6b mov rsi,r12
            rsi = r12;
            // 12d6e call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x12d73;
        }
        0x12d73 => {
            // block 12d73
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d73 dec r13d
            { let (r, f) = dec::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // 12d76 jne 0x12d3f
            if fl.ne() { pc = 0x12d3f; continue; }
            pc = 0x12d78;
        }
        0x12d78 => {
            // block 12d78
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12d78 lea r15,[rbx+0x2c8]
            r15 = rbx.wrapping_add(0x2c8);
            // 12d7f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 12d82 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 12d89 lea rdx,[rbp-0x14c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_feb4);
            // 12d90 mov rsi,rbx
            rsi = rbx;
            // 12d93 call QWORD PTR [rax+0x90]
            { let r = { let t = m.r64(rax.wrapping_add(0x90)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12d99 test BYTE PTR [rbx+0x2c8],0x1
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x2c8)) as u64 & 0x1u64);
            // 12da0 mov r12,QWORD PTR [rip+0x12d631] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // 12da7 mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // 12dab jne 0x12db5
            if fl.ne() { pc = 0x12db5; continue; }
            pc = 0x12dad;
        }
        0x12dad => {
            // block 12dad
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12dad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12dad mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 12db3 jmp 0x12dca
            pc = 0x12dca; continue;
        }
        0x12db5 => {
            // block 12db5
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12db5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12db5 mov rax,QWORD PTR [rbx+0x2d8]
            rax = m.r64(rbx.wrapping_add(0x2d8)) as u64;
            // 12dbc mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 12dbf mov QWORD PTR [rbx+0x2d0],0x0
            m.w64(rbx.wrapping_add(0x2d0), (0x0u64) as u64);
            pc = 0x12dca;
        }
        0x12dca => {
            // block 12dca
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12dca xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 12dcc mov rdi,r15
            rdi = r15;
            // 12dcf call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12dd4 mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 12ddb mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 12ddf mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 12de6 mov rcx,QWORD PTR [rbp-0x168]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 12ded mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 12df1 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 12df4 xorps xmm0,xmm0
            x0 = 0;
            // 12df7 movaps XMMWORD PTR [rbp-0x170],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe90), x0);
            // 12dfe mov QWORD PTR [rbp-0x160],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (0x0u64) as u64);
            // 12e09 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 12e10 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12e15 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 12e19 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 12e1b jne 0x12e22
            if fl.ne() { pc = 0x12e22; continue; }
            pc = 0x12e1d;
        }
        0x12e1d => {
            // block 12e1d
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e1d shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 12e20 jmp 0x12e29
            pc = 0x12e29; continue;
        }
        0x12e22 => {
            // block 12e22
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e22 mov rax,QWORD PTR [rbx+0x2d0]
            rax = m.r64(rbx.wrapping_add(0x2d0)) as u64;
            pc = 0x12e29;
        }
        0x12e29 => {
            // block 12e29
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e29 test rax,rax
            fl = logic::<64>(rax & rax);
            // 12e2c jne 0x12e5c
            if fl.ne() { pc = 0x12e5c; continue; }
            pc = 0x12e2e;
        }
        0x12e2e => {
            // block 12e2e
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e2e cmp BYTE PTR [rbx+0x2e9],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x2e9)) as u64, 0x0u64).1;
            // 12e35 jne 0x12e55
            if fl.ne() { pc = 0x12e55; continue; }
            pc = 0x12e37;
        }
        0x12e37 => {
            // block 12e37
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e37 lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 12e3e mov rsi,r14
            rsi = r14;
            // 12e41 call 0xec04a
            { let r = { let t = m.env.stub_va(0x817); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12e46 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 12e49 sub eax,DWORD PTR [rbp-0x14c]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_feb4)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 12e4f mov DWORD PTR [rbx+0x260],eax
            m.w32(rbx.wrapping_add(0x260), ((rax as u32 as u64)) as u32);
            pc = 0x12e55;
        }
        0x12e55 => {
            // block 12e55
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e55 mov BYTE PTR [rbx+0x2e8],0x1
            m.w8(rbx.wrapping_add(0x2e8), (0x1u64) as u8);
            pc = 0x12e5c;
        }
        0x12e5c => {
            // block 12e5c
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e5c mov al,BYTE PTR [rbx+0x2e8]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x2e8)) as u64) as u8 as u64);
            // 12e62 cmp r12,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 12e66 jne 0x12e7a
            if fl.ne() { pc = 0x12e7a; continue; }
            pc = 0x12e68;
        }
        0x12e68 => {
            // block 12e68
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e68 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 12e6f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 12e70 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 12e72 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 12e74 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 12e76 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12e78 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12e79 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x12e7a => {
            // block 12e7a
            #[cfg(feature = "trace")] eprintln!("f_00012c80 pc 12e7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e7a call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x154a4 size=588
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000154a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x154a4;
    loop {
        match pc {
        0x154a4 => {
            // block 154a4
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 154a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 154a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 154a5 mov rbp,rsp
            rbp = rsp;
            // 154a8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 154aa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 154ac push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 154ae push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 154b0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 154b1 sub rsp,0xd8
            { let (r, f) = sub::<64>(rsp, 0xd8u64);
              rsp = r;
              fl = f; }
            // 154b8 mov rax,QWORD PTR [rip+0x12ad61] # 0x140220
            rax = m.r64(m.env.data_va(346605)) as u64;
            // 154bf add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 154c3 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 154c6 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 154ca call 0x63dd3
            { let r = { let t = m.env.code_va(0x63dd3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 154cf mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 154d3 lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 154d7 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 154da je 0x1556c
            if fl.e() { pc = 0x1556c; continue; }
            pc = 0x154e0;
        }
        0x154e0 => {
            // block 154e0
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 154e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 154e0 lea r15,[rbp-0x80]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 154e4 lea r14,[rip+0x1ac515] # 0x1c1a00
            r14 = m.env.data_va(877005);
            // 154eb lea r13,[rbp-0x60]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            pc = 0x154ef;
        }
        0x154ef => {
            // block 154ef
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 154ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 154ef mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 154f2 mov rdi,r15
            rdi = r15;
            // 154f5 call 0x147d7
            { let r = { let t = m.env.code_va(0x147d7); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 154fa mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 154fe mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 15502 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 15506 mov rcx,QWORD PTR [rbp-0x78]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1550a mov QWORD PTR [rbp-0x58],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rcx) as u64);
            // 1550e mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 15512 xorps xmm0,xmm0
            x0 = 0;
            // 15515 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // 15519 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // 15521 mov eax,DWORD PTR [rbx+0x1c]
            rax = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 15524 mov DWORD PTR [rbp-0x48],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((rax as u32 as u64)) as u32);
            // 15527 mov rdi,r14
            rdi = r14;
            // 1552a mov rsi,r13
            rsi = r13;
            // 1552d call 0xeb92a
            { let r = { let t = m.env.stub_va(0xf7); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15532 mov rdi,r13
            rdi = r13;
            // 15535 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1553a mov rdi,r15
            rdi = r15;
            // 1553d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15542 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 15546 test rax,rax
            fl = logic::<64>(rax & rax);
            // 15549 je 0x15558
            if fl.e() { pc = 0x15558; continue; }
            pc = 0x1554b;
        }
        0x1554b => {
            // block 1554b
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 1554b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1554b mov rbx,rax
            rbx = rax;
            // 1554e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 15551 test rax,rax
            fl = logic::<64>(rax & rax);
            // 15554 jne 0x1554b
            if fl.ne() { pc = 0x1554b; continue; }
            pc = 0x15556;
        }
        0x15556 => {
            // block 15556
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 15556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15556 jmp 0x15567
            pc = 0x15567; continue;
        }
        0x15558 => {
            // block 15558
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 15558 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15558 mov rax,rbx
            rax = rbx;
            pc = 0x1555b;
        }
        0x1555b => {
            // block 1555b
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 1555b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1555b mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 1555f cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 15562 mov rax,rbx
            rax = rbx;
            // 15565 jne 0x1555b
            if fl.ne() { pc = 0x1555b; continue; }
            pc = 0x15567;
        }
        0x15567 => {
            // block 15567
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 15567 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15567 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 1556a jne 0x154ef
            if fl.ne() { pc = 0x154ef; continue; }
            pc = 0x1556c;
        }
        0x1556c => {
            // block 1556c
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 1556c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1556c lea rsi,[rip+0x114bd0] # 0x12a143
            rsi = m.env.data_va(256272);
            // 15573 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 1557a mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 1557f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15584 mov DWORD PTR [rbp-0x88],0x10322950
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0x10322950u64) as u32);
            // 1558e lea rdi,[rip+0x1ac46b] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 15595 lea rsi,[rbp-0xa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 1559c call 0xeb92a
            { let r = { let t = m.env.stub_va(0xf7); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 155a1 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 155a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 155ad lea rsi,[rip+0x114b9c] # 0x12a150
            rsi = m.env.data_va(256285);
            // 155b4 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 155bb mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 155c0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 155c5 mov DWORD PTR [rbp-0xa8],0x10294700
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x10294700u64) as u32);
            // 155cf lea rdi,[rip+0x1ac42a] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 155d6 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 155dd call 0xeb92a
            { let r = { let t = m.env.stub_va(0xf7); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 155e2 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 155e9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 155ee lea rsi,[rip+0x114b63] # 0x12a158
            rsi = m.env.data_va(256293);
            // 155f5 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 155fc mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 15601 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15606 mov DWORD PTR [rbp-0xc8],0x10538200
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (0x10538200u64) as u32);
            // 15610 lea rdi,[rip+0x1ac3e9] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 15617 lea rsi,[rbp-0xe0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 1561e call 0xeb92a
            { let r = { let t = m.env.stub_va(0xf7); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15623 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 1562a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1562f lea rsi,[rip+0x114b38] # 0x12a16e
            rsi = m.env.data_va(256315);
            // 15636 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 1563d mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 15642 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15647 mov DWORD PTR [rbp-0xe8],0x10668200
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x10668200u64) as u32);
            // 15651 lea rdi,[rip+0x1ac3a8] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 15658 lea rsi,[rbp-0x100]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 1565f call 0xeb92a
            { let r = { let t = m.env.stub_va(0xf7); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15664 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 1566b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15670 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 15674 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 15678 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1567d add rsp,0xd8
            { let (r, f) = add::<64>(rsp, 0xd8u64);
              rsp = r;
              fl = f; }
            // 15684 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15685 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15687 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15689 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1568b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1568d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1568e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1568f => {
            // block 1568f
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 1568f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1568f jmp 0x156a8
            pc = 0x156a8; continue;
        }
        0x15691 => {
            // block 15691
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 15691 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15691 mov rbx,rax
            rbx = rax;
            // 15694 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 15698 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1569d lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            pc = 0x156a1;
        }
        0x156a1 => {
            // block 156a1
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156a1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 156a6 jmp 0x156ab
            pc = 0x156ab; continue;
        }
        0x156a8 => {
            // block 156a8
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156a8 mov rbx,rax
            rbx = rax;
            pc = 0x156ab;
        }
        0x156ab => {
            // block 156ab
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156ab mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 156af lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 156b3 call 0xebb76
            { let r = { let t = m.env.stub_va(0x343); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 156b8 mov rdi,rbx
            rdi = rbx;
            // 156bb call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 156c0 mov rbx,rax
            rbx = rax;
            // 156c3 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 156ca jmp 0x156a1
            pc = 0x156a1; continue;
        }
        0x156cc => {
            // block 156cc
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156cc mov rbx,rax
            rbx = rax;
            // 156cf lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 156d6 jmp 0x156a1
            pc = 0x156a1; continue;
        }
        0x156d8 => {
            // block 156d8
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156d8 mov rbx,rax
            rbx = rax;
            // 156db lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 156e2 jmp 0x156a1
            pc = 0x156a1; continue;
        }
        0x156e4 => {
            // block 156e4
            #[cfg(feature = "trace")] eprintln!("f_000154a4 pc 156e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156e4 mov rbx,rax
            rbx = rax;
            // 156e7 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 156ee jmp 0x156a1
            pc = 0x156a1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bfcc size=620
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bfcc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2bfcc;
    loop {
        match pc {
        0x2bfcc => {
            // block 2bfcc
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2bfcc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bfcc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bfcd mov rbp,rsp
            rbp = rsp;
            // 2bfd0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bfd2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bfd4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2bfd6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2bfd8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bfd9 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2bfdd mov QWORD PTR [rbp-0x48],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rsi) as u64);
            // 2bfe1 mov rbx,rdi
            rbx = rdi;
            // 2bfe4 call 0x26172
            { let r = { let t = m.env.code_va(0x26172); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bfe9 lea rsi,[rip+0xfe914] # 0x12a904
            rsi = m.env.data_va(258257);
            // 2bff0 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 2bff5 mov rdi,rax
            rdi = rax;
            // 2bff8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bffd mov esi,DWORD PTR [rbx+0x20]
            rsi = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2c000 mov rdi,rax
            rdi = rax;
            // 2c003 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c008 lea rsi,[rip+0xfe938] # 0x12a947
            rsi = m.env.data_va(258324);
            // 2c00f mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 2c014 mov rdi,rax
            rdi = rax;
            // 2c017 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c01c lea rdi,[rbx+0x24]
            rdi = rbx.wrapping_add(0x24);
            // 2c020 mov rsi,rax
            rsi = rax;
            // 2c023 call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c028 lea rsi,[rip+0xfe926] # 0x12a955
            rsi = m.env.data_va(258338);
            // 2c02f mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 2c034 mov rdi,rax
            rdi = rax;
            // 2c037 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c03c mov esi,DWORD PTR [rbx+0x4c]
            rsi = (m.r32(rbx.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 2c03f mov rdi,rax
            rdi = rax;
            // 2c042 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c047 lea rsi,[rip+0xfe919] # 0x12a967
            rsi = m.env.data_va(258356);
            // 2c04e mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 2c053 mov rdi,rax
            rdi = rax;
            // 2c056 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c05b lea rdi,[rbx+0x50]
            rdi = rbx.wrapping_add(0x50);
            // 2c05f mov rsi,rax
            rsi = rax;
            // 2c062 call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c067 lea rsi,[rip+0xfe42f] # 0x12a49d
            rsi = m.env.data_va(257130);
            // 2c06e mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 2c073 mov rdi,rax
            rdi = rax;
            // 2c076 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c07b lea rsi,[rbx+0x78]
            rsi = rbx.wrapping_add(0x78);
            // 2c07f mov rdi,rax
            rdi = rax;
            // 2c082 call 0x26804
            { let r = { let t = m.env.code_va(0x26804); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c087 mov rax,QWORD PTR [rbx+0x24]
            rax = m.r64(rbx.wrapping_add(0x24)) as u64;
            // 2c08b mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 2c08f test rax,rax
            fl = logic::<64>(rax & rax);
            // 2c092 je 0x2c208
            if fl.e() { pc = 0x2c208; continue; }
            pc = 0x2c098;
        }
        0x2c098 => {
            // block 2c098
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c098 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c098 mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // 2c09c mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 2c0a0 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2c0a3 je 0x2c208
            if fl.e() { pc = 0x2c208; continue; }
            pc = 0x2c0a9;
        }
        0x2c0a9 => {
            // block 2c0a9
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c0a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c0a9 mov r14d,DWORD PTR [rbx+0x58]
            r14 = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // 2c0ad shr r14d,1
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x1u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 2c0b0 test r14w,r14w
            fl = logic::<16>((r14 as u16 as u64) & (r14 as u16 as u64));
            // 2c0b4 je 0x2c208
            if fl.e() { pc = 0x2c208; continue; }
            pc = 0x2c0ba;
        }
        0x2c0ba => {
            // block 2c0ba
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c0ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c0ba mov r12d,DWORD PTR [rbx+0x2c]
            r12 = (m.r32(rbx.wrapping_add(0x2c)) as u64) as u32 as u64;
            // 2c0be shr r12d,0x4
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x4u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // 2c0c2 test r12w,r12w
            fl = logic::<16>((r12 as u16 as u64) & (r12 as u16 as u64));
            // 2c0c6 je 0x2c208
            if fl.e() { pc = 0x2c208; continue; }
            pc = 0x2c0cc;
        }
        0x2c0cc => {
            // block 2c0cc
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c0cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c0cc mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2c0d0 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 2c0d3 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c0d7 add rsi,rcx
            { let (r, f) = add::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // 2c0da mov rbx,rcx
            rbx = rcx;
            // 2c0dd lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2c0e1 mov rdi,r15
            rdi = r15;
            // 2c0e4 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c0e9 mov rsi,QWORD PTR [rip+0x1140a0] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2c0f0 mov rdi,r15
            rdi = r15;
            // 2c0f3 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c0f8 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c0fb mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2c0ff mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2c104 mov rdi,rax
            rdi = rax;
            // 2c107 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c109 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2c10c lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2c110 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c115 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // 2c119 mov rdi,rbx
            rdi = rbx;
            // 2c11c call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c121 mov rdi,rbx
            rdi = rbx;
            // 2c124 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c129 mov r13d,r14d
            r13 = ((r14 as u32 as u64)) as u32 as u64;
            // 2c12c and r13d,0xffff
            { let r = (r13 as u32 as u64) & 0xffffu64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 2c133 je 0x2c208
            if fl.e() { pc = 0x2c208; continue; }
            pc = 0x2c139;
        }
        0x2c139 => {
            // block 2c139
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c139 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c139 movzx ecx,r12w
            rcx = ((r12 as u16 as u64)) as u32 as u64;
            // 2c13d mov DWORD PTR [rbp-0x54],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((rcx as u32 as u64)) as u32);
            // 2c140 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 2c143 lea r14,[rbp-0x30]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0x2c147;
        }
        0x2c147 => {
            // block 2c147
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c147 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c147 movzx r15d,r12w
            r15 = ((r12 as u16 as u64)) as u32 as u64;
            // 2c14b mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2c14f movzx ebx,WORD PTR [rax+r15*2]
            rbx = (m.r16(rax.wrapping_add(r15.wrapping_mul(2))) as u64) as u32 as u64;
            // 2c154 cmp ebx,ecx
            fl = sub::<32>((rbx as u32 as u64), (rcx as u32 as u64)).1;
            // 2c156 jae 0x2c1f8
            if fl.ae() { pc = 0x2c1f8; continue; }
            pc = 0x2c15c;
        }
        0x2c15c => {
            // block 2c15c
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c15c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c15c shl rbx,0x4
            { let (r, f) = shl::<64>(rbx, 0x4u64, fl); rbx = r; fl = f; }
            // 2c160 add rbx,QWORD PTR [rbp-0x50]
            { let (r, f) = add::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64);
              rbx = r;
              fl = f; }
            // 2c164 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 2c169 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2c16d lea rsi,[rip+0xfe807] # 0x12a97b
            rsi = m.env.data_va(258376);
            // 2c174 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c179 mov rdi,rax
            rdi = rax;
            // 2c17c mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2c17f call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c184 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 2c189 mov rdi,rax
            rdi = rax;
            // 2c18c lea rsi,[rip+0xfe7ed] # 0x12a980
            rsi = m.env.data_va(258381);
            // 2c193 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c198 mov rdi,rax
            rdi = rax;
            // 2c19b mov rsi,rbx
            rsi = rbx;
            // 2c19e call 0x2c238
            { let r = { let t = m.env.code_va(0x2c238); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1a3 mov r15,rax
            r15 = rax;
            // 2c1a6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2c1a9 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c1ad add rsi,r15
            { let (r, f) = add::<64>(rsi, r15);
              rsi = r;
              fl = f; }
            // 2c1b0 mov rdi,r14
            rdi = r14;
            // 2c1b3 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1b8 mov rdi,r14
            rdi = r14;
            // 2c1bb mov rsi,QWORD PTR [rip+0x113fce] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2c1c2 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1c7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c1ca mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2c1ce mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2c1d3 mov rdi,rax
            rdi = rax;
            // 2c1d6 call rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1d8 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2c1da mov rdi,r14
            rdi = r14;
            // 2c1dd call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1e2 movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // 2c1e5 mov rdi,r15
            rdi = r15;
            // 2c1e8 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1ed mov rdi,r15
            rdi = r15;
            // 2c1f0 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c1f5 mov ecx,DWORD PTR [rbp-0x54]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64) as u32 as u64;
            pc = 0x2c1f8;
        }
        0x2c1f8 => {
            // block 2c1f8
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c1f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c1f8 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // 2c1fb movzx eax,r12w
            rax = ((r12 as u16 as u64)) as u32 as u64;
            // 2c1ff cmp eax,r13d
            fl = sub::<32>((rax as u32 as u64), (r13 as u32 as u64)).1;
            // 2c202 jb 0x2c147
            if fl.b() { pc = 0x2c147; continue; }
            pc = 0x2c208;
        }
        0x2c208 => {
            // block 2c208
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c208 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c208 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2c20c add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2c210 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2c211 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2c213 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2c215 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2c217 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2c219 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2c21a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2c21b => {
            // block 2c21b
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c21b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c21b mov rbx,rax
            rbx = rax;
            // 2c21e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0x2c222;
        }
        0x2c222 => {
            // block 2c222
            #[cfg(feature = "trace")] eprintln!("f_0002bfcc pc 2c222 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c222 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c227 mov rdi,rbx
            rdi = rbx;
            // 2c22a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c22f mov rbx,rax
            rbx = rax;
            // 2c232 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2c236 jmp 0x2c222
            pc = 0x2c222; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2fdd8 size=616
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002fdd8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2fdd8;
    loop {
        match pc {
        0x2fdd8 => {
            // block 2fdd8
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fdd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fdd8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2fdd9 mov rbp,rsp
            rbp = rsp;
            // 2fddc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2fdde push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2fde0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2fde2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2fde4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2fde5 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2fde9 mov DWORD PTR [rbp-0x48],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((r8 as u32 as u64)) as u32);
            // 2fded mov rbx,rdx
            rbx = rdx;
            // 2fdf0 mov r15,rsi
            r15 = rsi;
            // 2fdf3 mov r14,rdi
            r14 = rdi;
            // 2fdf6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2fdf9 cmp rax,QWORD PTR [r15+0x8]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 2fdfd je 0x2ff81
            if fl.e() { pc = 0x2ff81; continue; }
            pc = 0x2fe03;
        }
        0x2fe03 => {
            // block 2fe03
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fe03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fe03 mov QWORD PTR [rbp-0x50],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rcx) as u64);
            // 2fe07 mov rdi,r14
            rdi = r14;
            // 2fe0a call 0x31164
            { let r = { let t = m.env.code_va(0x31164); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fe0f lea rdi,[r14+0x18]
            rdi = r14.wrapping_add(0x18);
            // 2fe13 mov QWORD PTR [rbp-0x40],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdi) as u64);
            // 2fe17 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2fe1b mov esi,DWORD PTR [r14+0x2b4]
            rsi = (m.r32(r14.wrapping_add(0x2b4)) as u64) as u32 as u64;
            // 2fe22 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2fe24 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2fe29 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2fe2c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fe2f mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 2fe36 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2fe39 cmp QWORD PTR [r15+0x8],rax
            fl = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax).1;
            // 2fe3d je 0x2febe
            if fl.e() { pc = 0x2febe; continue; }
            pc = 0x2fe3f;
        }
        0x2fe3f => {
            // block 2fe3f
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fe3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fe3f mov QWORD PTR [rbp-0x58],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rbx) as u64);
            // 2fe43 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 2fe49 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2fe4b lea r13,[rbp-0x2c]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            pc = 0x2fe4f;
        }
        0x2fe4f => {
            // block 2fe4f
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fe4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fe4f mov rbx,QWORD PTR [r14+0x18]
            rbx = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2fe53 mov esi,DWORD PTR [r14+0x2ac]
            rsi = (m.r32(r14.wrapping_add(0x2ac)) as u64) as u32 as u64;
            // 2fe5a movzx edx,BYTE PTR [rax+rdx*1]
            rdx = (m.r8(rax.wrapping_add(rdx)) as u64) as u32 as u64;
            // 2fe5e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2fe63 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2fe66 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2fe6a call QWORD PTR [rbx+0x30]
            { let r = { let t = m.r64(rbx.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fe6d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2fe70 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2fe74 mov rdx,rcx
            rdx = rcx;
            // 2fe77 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 2fe7a cmp rdx,0x2
            fl = sub::<64>(rdx, 0x2u64).1;
            // 2fe7e jb 0x2fea4
            if fl.b() { pc = 0x2fea4; continue; }
            pc = 0x2fe80;
        }
        0x2fe80 => {
            // block 2fe80
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fe80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fe80 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2fe84 mov esi,DWORD PTR [r14+0x2b0]
            rsi = (m.r32(r14.wrapping_add(0x2b0)) as u64) as u32 as u64;
            // 2fe8b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2fe90 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2fe93 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2fe97 mov rdx,r13
            rdx = r13;
            // 2fe9a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fe9d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2fea0 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x2fea4;
        }
        0x2fea4 => {
            // block 2fea4
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fea4 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 2fea7 mov rsi,rcx
            rsi = rcx;
            // 2feaa sub rsi,rax
            { let (r, f) = sub::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 2fead inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // 2feb0 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 2feb3 jb 0x2fe4f
            if fl.b() { pc = 0x2fe4f; continue; }
            pc = 0x2feb5;
        }
        0x2feb5 => {
            // block 2feb5
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2feb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2feb5 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 2feb8 mov rbx,QWORD PTR [rbp-0x58]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 2febc jne 0x2fec9
            if fl.ne() { pc = 0x2fec9; continue; }
            pc = 0x2febe;
        }
        0x2febe => {
            // block 2febe
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2febe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2febe mov rdi,r15
            rdi = r15;
            // 2fec1 call 0xebfba
            { let r = { let t = m.env.stub_va(0x787); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fec6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            pc = 0x2fec9;
        }
        0x2fec9 => {
            // block 2fec9
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fec9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fec9 movzx ecx,BYTE PTR [rax]
            rcx = (m.r8(rax) as u64) as u32 as u64;
            // 2fecc cmp ecx,0x21
            fl = sub::<32>((rcx as u32 as u64), 0x21u64).1;
            // 2fecf je 0x30010
            if fl.e() { pc = 0x30010; continue; }
            pc = 0x2fed5;
        }
        0x2fed5 => {
            // block 2fed5
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fed5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fed5 cmp QWORD PTR [r15+0x8],rax
            fl = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax).1;
            // 2fed9 jne 0x2fee6
            if fl.ne() { pc = 0x2fee6; continue; }
            pc = 0x2fedb;
        }
        0x2fedb => {
            // block 2fedb
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fedb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fedb mov rdi,r15
            rdi = r15;
            // 2fede call 0xebfba
            { let r = { let t = m.env.stub_va(0x787); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fee3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            pc = 0x2fee6;
        }
        0x2fee6 => {
            // block 2fee6
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fee6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fee6 movzx eax,BYTE PTR [rax]
            rax = (m.r8(rax) as u64) as u32 as u64;
            // 2fee9 cmp eax,0xdc
            fl = sub::<32>((rax as u32 as u64), 0xdcu64).1;
            // 2feee je 0x30010
            if fl.e() { pc = 0x30010; continue; }
            pc = 0x2fef4;
        }
        0x2fef4 => {
            // block 2fef4
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fef4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fef4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2fef7 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2fefb cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 2fefe je 0x2ff88
            if fl.e() { pc = 0x2ff88; continue; }
            pc = 0x2ff04;
        }
        0x2ff04 => {
            // block 2ff04
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff04 mov esi,DWORD PTR [rbp-0x48]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 2ff07 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 2ff09 mov rdi,rcx
            rdi = rcx;
            // 2ff0c sub rdi,rax
            { let (r, f) = sub::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 2ff0f cmp rdx,rdi
            fl = sub::<64>(rdx, rdi).1;
            // 2ff12 cmovbe edi,esi
            if fl.be() { rdi = ((rsi as u32 as u64)) as u32 as u64; } else { rdi = rdi as u32 as u64; }
            // 2ff15 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 2ff19 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // 2ff1b je 0x30010
            if fl.e() { pc = 0x30010; continue; }
            pc = 0x2ff21;
        }
        0x2ff21 => {
            // block 2ff21
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff21 mov rdx,QWORD PTR [rbp-0x48]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2ff25 dec edx
            { let (r, f) = dec::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // 2ff27 mov QWORD PTR [rbp-0x48],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdx) as u64);
            // 2ff2b xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 2ff2e jmp 0x2ff3a
            pc = 0x2ff3a; continue;
        }
        0x2ff30 => {
            // block 2ff30
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff30 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 2ff33 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2ff36 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0x2ff3a;
        }
        0x2ff3a => {
            // block 2ff3a
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff3a mov rdx,QWORD PTR [r14+0x18]
            rdx = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2ff3e mov r13,QWORD PTR [rdx+0x30]
            r13 = m.r64(rdx.wrapping_add(0x30)) as u64;
            // 2ff42 mov r15d,DWORD PTR [r14+0x2ac]
            r15 = (m.r32(r14.wrapping_add(0x2ac)) as u64) as u32 as u64;
            // 2ff49 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 2ff4c cmp rcx,r12
            fl = sub::<64>(rcx, r12).1;
            // 2ff4f ja 0x2ff5c
            if fl.a() { pc = 0x2ff5c; continue; }
            pc = 0x2ff51;
        }
        0x2ff51 => {
            // block 2ff51
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff51 mov rdi,rbx
            rdi = rbx;
            // 2ff54 call 0xebfba
            { let r = { let t = m.env.stub_va(0x787); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ff59 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            pc = 0x2ff5c;
        }
        0x2ff5c => {
            // block 2ff5c
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff5c movzx edx,BYTE PTR [rax+r12*1]
            rdx = (m.r8(rax.wrapping_add(r12)) as u64) as u32 as u64;
            // 2ff61 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2ff66 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2ff69 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2ff6d mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2ff70 call r13
            { let r = { let t = r13; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ff73 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2ff77 cmp eax,r12d
            fl = sub::<32>((rax as u32 as u64), (r12 as u32 as u64)).1;
            // 2ff7a jne 0x2ff30
            if fl.ne() { pc = 0x2ff30; continue; }
            pc = 0x2ff7c;
        }
        0x2ff7c => {
            // block 2ff7c
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff7c jmp 0x30010
            pc = 0x30010; continue;
        }
        0x2ff81 => {
            // block 2ff81
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff81 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2ff83 jmp 0x30031
            pc = 0x30031; continue;
        }
        0x2ff88 => {
            // block 2ff88
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff88 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 2ff8f cmp DWORD PTR [rbp-0x48],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, 0xffffffffu64).1;
            // 2ff93 je 0x30010
            if fl.e() { pc = 0x30010; continue; }
            pc = 0x2ff95;
        }
        0x2ff95 => {
            // block 2ff95
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ff95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ff95 not DWORD PTR [rbp-0x48]
            { let v = !m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64; m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (v) as u32); }
            // 2ff98 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 2ff9b lea r15,[rbp-0x30]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2ff9f lea r12,[rbp-0x31]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            pc = 0x2ffa3;
        }
        0x2ffa3 => {
            // block 2ffa3
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ffa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ffa3 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2ffa7 mov esi,DWORD PTR [r14+0x2ac]
            rsi = (m.r32(r14.wrapping_add(0x2ac)) as u64) as u32 as u64;
            // 2ffae xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2ffb0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2ffb5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2ffb8 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2ffbc mov rdi,rbx
            rdi = rbx;
            // 2ffbf call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ffc2 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2ffc6 mov esi,DWORD PTR [r14+0x2b0]
            rsi = (m.r32(r14.wrapping_add(0x2b0)) as u64) as u32 as u64;
            // 2ffcd mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2ffd2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2ffd5 mov rdi,rbx
            rdi = rbx;
            // 2ffd8 mov rdx,r15
            rdx = r15;
            // 2ffdb call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ffde test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 2ffe1 je 0x30007
            if fl.e() { pc = 0x30007; continue; }
            pc = 0x2ffe3;
        }
        0x2ffe3 => {
            // block 2ffe3
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ffe3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ffe3 mov al,BYTE PTR [rbp-0x30]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u8 as u64);
            // 2ffe6 mov BYTE PTR [rbp-0x31],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), ((rax as u8 as u64)) as u8);
            // 2ffe9 mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 2ffed mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2fff1 cmp rcx,QWORD PTR [rdi+0x10]
            fl = sub::<64>(rcx, m.r64(rdi.wrapping_add(0x10)) as u64).1;
            // 2fff5 jae 0x2ffff
            if fl.ae() { pc = 0x2ffff; continue; }
            pc = 0x2fff7;
        }
        0x2fff7 => {
            // block 2fff7
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2fff7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fff7 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // 2fff9 inc QWORD PTR [rdi+0x8]
            { let (r, f) = inc::<64>(m.r64(rdi.wrapping_add(0x8)) as u64, fl); m.w64(rdi.wrapping_add(0x8), (r) as u64); fl = f; }
            // 2fffd jmp 0x30007
            pc = 0x30007; continue;
        }
        0x2ffff => {
            // block 2ffff
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 2ffff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ffff mov rsi,r12
            rsi = r12;
            // 30002 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x30007;
        }
        0x30007 => {
            // block 30007
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 30007 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30007 dec r13d
            { let (r, f) = dec::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // 3000a cmp DWORD PTR [rbp-0x48],r13d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, (r13 as u32 as u64)).1;
            // 3000e jne 0x2ffa3
            if fl.ne() { pc = 0x2ffa3; continue; }
            pc = 0x30010;
        }
        0x30010 => {
            // block 30010
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 30010 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30010 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 30014 mov esi,DWORD PTR [r14+0x2b4]
            rsi = (m.r32(r14.wrapping_add(0x2b4)) as u64) as u32 as u64;
            // 3001b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 30020 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 30025 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 30028 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 3002c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3002f mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x30031;
        }
        0x30031 => {
            // block 30031
            #[cfg(feature = "trace")] eprintln!("f_0002fdd8 pc 30031 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 30031 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 30035 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 30036 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 30038 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 3003a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3003c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3003e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3003f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x32792 size=506
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00032792<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x32792;
    loop {
        match pc {
        0x32792 => {
            // block 32792
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32792 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32792 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 32793 mov rbp,rsp
            rbp = rsp;
            // 32796 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 32798 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3279a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 3279c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3279d mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // 327a0 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 327a3 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // 327a6 mov rbx,rdi
            rbx = rdi;
            // 327a9 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 327b1 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 327b8 mov DWORD PTR [rbx+0x10],0xc
            m.w32(rbx.wrapping_add(0x10), (0xcu64) as u32);
            // 327bf mov DWORD PTR [rbx+0x14],0x0
            m.w32(rbx.wrapping_add(0x14), (0x0u64) as u32);
            // 327c6 mov DWORD PTR [rbx+0x18],0x20
            m.w32(rbx.wrapping_add(0x18), (0x20u64) as u32);
            // 327cd mov DWORD PTR [rbx+0x1c],0x3
            m.w32(rbx.wrapping_add(0x1c), (0x3u64) as u32);
            // 327d4 mov BYTE PTR [rbx+0x20],0x0
            m.w8(rbx.wrapping_add(0x20), (0x0u64) as u8);
            // 327d8 mov WORD PTR [rbx+0x34],0x0
            m.w16(rbx.wrapping_add(0x34), (0x0u64) as u16);
            // 327de mov QWORD PTR [rbx+0x2c],0x0
            m.w64(rbx.wrapping_add(0x2c), (0x0u64) as u64);
            // 327e6 mov QWORD PTR [rbx+0x24],0x0
            m.w64(rbx.wrapping_add(0x24), (0x0u64) as u64);
            // 327ee mov DWORD PTR [rbx+0x38],0x12
            m.w32(rbx.wrapping_add(0x38), (0x12u64) as u32);
            // 327f5 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 327f8 call 0x5a914
            { let r = { let t = m.env.code_va(0x5a914); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 327fd cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 32801 ja 0x32960
            if fl.a() { pc = 0x32960; continue; }
            pc = 0x32807;
        }
        0x32807 => {
            // block 32807
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32807 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32807 cmp r14d,0x1f
            fl = sub::<32>((r14 as u32 as u64), 0x1fu64).1;
            // 3280b ja 0x32960
            if fl.a() { pc = 0x32960; continue; }
            pc = 0x32811;
        }
        0x32811 => {
            // block 32811
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32811 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32811 cmp eax,0xb
            fl = sub::<32>((rax as u32 as u64), 0xbu64).1;
            // 32814 ja 0x32960
            if fl.a() { pc = 0x32960; continue; }
            pc = 0x3281a;
        }
        0x3281a => {
            // block 3281a
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3281a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3281a cmp r14d,0x17
            fl = sub::<32>((r14 as u32 as u64), 0x17u64).1;
            // 3281e sete dl
            rdx = (rdx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32821 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 32824 and ecx,0xffffffef
            { let r = (rcx as u32 as u64) & 0xffffffefu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 32827 cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 3282a sete dil
            rdi = (rdi & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 3282e lea ecx,[r14-0x1b]
            rcx = (r14.wrapping_add(0xffff_ffff_ffff_ffe5)) as u32 as u64;
            // 32832 cmp ecx,0x5
            fl = sub::<32>((rcx as u32 as u64), 0x5u64).1;
            // 32835 setb cl
            rcx = (rcx & !0xff) | (((fl.b() as u64)) as u8 as u64);
            // 32838 lea esi,[r15-0x1]
            rsi = (r15.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 3283c cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 3283f ja 0x3284c
            if fl.a() { pc = 0x3284c; continue; }
            pc = 0x32841;
        }
        0x32841 => {
            // block 32841
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32841 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32841 or dl,dil
            { let r = (rdx as u8 as u64) | (rdi as u8 as u64); let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32844 or cl,dl
            { let r = (rcx as u8 as u64) | (rdx as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32846 jne 0x32960
            if fl.ne() { pc = 0x32960; continue; }
            pc = 0x3284c;
        }
        0x3284c => {
            // block 3284c
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3284c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3284c mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 3284f mov r8d,eax
            r8 = ((rax as u32 as u64)) as u32 as u64;
            // 32852 imul rdi,r8,0x780
            { let (r, f) = imul::<64>(r8, 0x780u64); rdi = r; fl = f; }
            // 32859 lea rdx,[rip+0x18fe10] # 0x1c2670
            rdx = m.env.data_va(880189);
            // 32860 add rdx,rdi
            { let (r, f) = add::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // 32863 imul rcx,rcx,0x3c
            { let (r, f) = imul::<64>(rcx, 0x3cu64); rcx = r; fl = f; }
            // 32867 mov edi,DWORD PTR [rcx+rdx*1+0x38]
            rdi = (m.r32(rcx.wrapping_add(rdx).wrapping_add(0x38)) as u64) as u32 as u64;
            // 3286b mov DWORD PTR [rbx+0x38],edi
            m.w32(rbx.wrapping_add(0x38), ((rdi as u32 as u64)) as u32);
            // 3286e mov rdi,QWORD PTR [rcx+rdx*1+0x30]
            rdi = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x30)) as u64;
            // 32873 mov QWORD PTR [rbx+0x30],rdi
            m.w64(rbx.wrapping_add(0x30), (rdi) as u64);
            // 32877 mov rdi,QWORD PTR [rcx+rdx*1+0x28]
            rdi = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x28)) as u64;
            // 3287c mov QWORD PTR [rbx+0x28],rdi
            m.w64(rbx.wrapping_add(0x28), (rdi) as u64);
            // 32880 mov rdi,QWORD PTR [rcx+rdx*1+0x20]
            rdi = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x20)) as u64;
            // 32885 mov QWORD PTR [rbx+0x20],rdi
            m.w64(rbx.wrapping_add(0x20), (rdi) as u64);
            // 32889 mov rdi,QWORD PTR [rcx+rdx*1+0x18]
            rdi = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x18)) as u64;
            // 3288e mov QWORD PTR [rbx+0x18],rdi
            m.w64(rbx.wrapping_add(0x18), (rdi) as u64);
            // 32892 mov rdi,QWORD PTR [rcx+rdx*1+0x10]
            rdi = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x10)) as u64;
            // 32897 mov QWORD PTR [rbx+0x10],rdi
            m.w64(rbx.wrapping_add(0x10), (rdi) as u64);
            // 3289b mov rdi,QWORD PTR [rcx+rdx*1]
            rdi = m.r64(rcx.wrapping_add(rdx)) as u64;
            // 3289f mov rcx,QWORD PTR [rcx+rdx*1+0x8]
            rcx = m.r64(rcx.wrapping_add(rdx).wrapping_add(0x8)) as u64;
            // 328a4 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 328a8 mov QWORD PTR [rbx],rdi
            m.w64(rbx, (rdi) as u64);
            // 328ab mov DWORD PTR [rbx+0x14],r12d
            m.w32(rbx.wrapping_add(0x14), ((r12 as u32 as u64)) as u32);
            // 328af mov DWORD PTR [rbx+0x10],eax
            m.w32(rbx.wrapping_add(0x10), ((rax as u32 as u64)) as u32);
            // 328b2 mov DWORD PTR [rbx+0x18],r14d
            m.w32(rbx.wrapping_add(0x18), ((r14 as u32 as u64)) as u32);
            // 328b6 mov DWORD PTR [rbx+0x1c],r15d
            m.w32(rbx.wrapping_add(0x1c), ((r15 as u32 as u64)) as u32);
            // 328ba mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 328bc and ecx,0xfffffffe
            { let r = (rcx as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 328bf cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 328c2 sete BYTE PTR [rbx+0x20]
            m.w8(rbx.wrapping_add(0x20), ((fl.e() as u64)) as u8);
            // 328c6 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 328c9 ja 0x3293f
            if fl.a() { pc = 0x3293f; continue; }
            pc = 0x328cb;
        }
        0x328cb => {
            // block 328cb
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 328cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 328cb mov ecx,DWORD PTR [rbx]
            rcx = (m.r32(rbx) as u64) as u32 as u64;
            // 328cd cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // 328d0 jbe 0x328d8
            if fl.be() { pc = 0x328d8; continue; }
            pc = 0x328d2;
        }
        0x328d2 => {
            // block 328d2
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 328d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 328d2 mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            // 328d4 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 328d6 jmp 0x3293a
            pc = 0x3293a; continue;
        }
        0x328d8 => {
            // block 328d8
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 328d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 328d8 lea rax,[rip+0x8d] # 0x3296c
            rax = m.env.code_va(0x3296c);
            // 328df movsxd rdx,DWORD PTR [rax+r8*4]
            rdx = sx::<32>(m.r32(rax.wrapping_add(r8.wrapping_mul(4))) as u64) as u64;
            // 328e3 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 328e6 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x328e8 => {
            // block 328e8
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 328e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 328e8 cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 328ec sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 328ef movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 328f2 lea eax,[rax+rax*1+0x458]
            rax = (rax.wrapping_add(rax).wrapping_add(0x458)) as u32 as u64;
            // 328f9 mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // 328fb jmp 0x3293a
            pc = 0x3293a; continue;
        }
        0x328fd => {
            // block 328fd
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 328fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 328fd mov DWORD PTR [rbx],0x2e4
            m.w32(rbx, (0x2e4u64) as u32);
            // 32903 mov eax,0x2e4
            rax = (0x2e4u64) as u32 as u64;
            // 32908 jmp 0x3293a
            pc = 0x3293a; continue;
        }
        0x3290a => {
            // block 3290a
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3290a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3290a cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 3290e mov edx,0x202
            rdx = (0x202u64) as u32 as u64;
            // 32913 mov eax,0x1fc
            rax = (0x1fcu64) as u32 as u64;
            // 32918 jmp 0x32928
            pc = 0x32928; continue;
        }
        0x3291a => {
            // block 3291a
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3291a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3291a cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 3291e mov edx,0x264
            rdx = (0x264u64) as u32 as u64;
            // 32923 mov eax,0x256
            rax = (0x256u64) as u32 as u64;
            pc = 0x32928;
        }
        0x32928 => {
            // block 32928
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32928 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32928 cmove eax,edx
            if fl.e() { rax = ((rdx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 3292b mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // 3292d jmp 0x3293a
            pc = 0x3293a; continue;
        }
        0x3292f => {
            // block 3292f
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3292f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3292f mov DWORD PTR [rbx],0x634
            m.w32(rbx, (0x634u64) as u32);
            // 32935 mov eax,0x634
            rax = (0x634u64) as u32 as u64;
            pc = 0x3293a;
        }
        0x3293a => {
            // block 3293a
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3293a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3293a sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 3293c mov DWORD PTR [rbx+0xc],eax
            m.w32(rbx.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            pc = 0x3293f;
        }
        0x3293f => {
            // block 3293f
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 3293f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3293f cmp r14d,0x1f
            fl = sub::<32>((r14 as u32 as u64), 0x1fu64).1;
            // 32943 ja 0x32960
            if fl.a() { pc = 0x32960; continue; }
            pc = 0x32945;
        }
        0x32945 => {
            // block 32945
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32945 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32945 mov eax,0xfc800400
            rax = (0xfc800400u64) as u32 as u64;
            // 3294a bt eax,r14d
            fl.cf = bit::<32>((rax as u32 as u64), (r14 as u32 as u64));
            // 3294e jae 0x32960
            if fl.ae() { pc = 0x32960; continue; }
            pc = 0x32950;
        }
        0x32950 => {
            // block 32950
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32950 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32950 mov rdi,rbx
            rdi = rbx;
            // 32953 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32954 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 32956 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 32958 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3295a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3295b jmp 0x325ca
            { let r = { let t = m.env.code_va(0x325ca); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x32960 => {
            // block 32960
            #[cfg(feature = "trace")] eprintln!("f_00032792 pc 32960 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32960 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32961 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 32963 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 32965 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 32967 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32968 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51f58 size=698
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051f58<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51f58;
    loop {
        match pc {
        0x51f58 => {
            // block 51f58
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 51f58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51f58 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51f59 mov rbp,rsp
            rbp = rsp;
            // 51f5c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 51f5e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 51f60 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 51f62 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 51f64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 51f65 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 51f69 mov r14,rdx
            r14 = rdx;
            // 51f6c mov rbx,rdi
            rbx = rdi;
            // 51f6f mov DWORD PTR [rbp-0x3c],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0xffffffffu64) as u32);
            // 51f76 mov DWORD PTR [rbp-0x40],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u32);
            // 51f7d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 51f80 lea rdx,[rbp-0x3c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // 51f84 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // 51f89 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51f8e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51f91 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51f94 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 51f97 lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 51f9b mov esi,0x27b5
            rsi = (0x27b5u64) as u32 as u64;
            // 51fa0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51fa5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51fa8 mov rdi,rbx
            rdi = rbx;
            // 51fab call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51fae mov eax,DWORD PTR [rbp-0x40]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 51fb1 mov DWORD PTR [rbp-0x54],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((rax as u32 as u64)) as u32);
            // 51fb4 cmp eax,0xfe
            fl = sub::<32>((rax as u32 as u64), 0xfeu64).1;
            // 51fb9 jne 0x5200c
            if fl.ne() { pc = 0x5200c; continue; }
            pc = 0x51fbb;
        }
        0x51fbb => {
            // block 51fbb
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 51fbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51fbb mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 51fc2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 51fc5 lea rdx,[rbp-0x44]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // 51fc9 mov esi,0x15
            rsi = (0x15u64) as u32 as u64;
            // 51fce mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // 51fd3 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 51fd9 mov rdi,rbx
            rdi = rbx;
            // 51fdc call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 51fdf cmp DWORD PTR [rbp-0x44],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64, 0x0u64).1;
            // 51fe3 mov eax,0xfe020000
            rax = (0xfe020000u64) as u32 as u64;
            // 51fe8 mov ecx,0xfe000000
            rcx = (0xfe000000u64) as u32 as u64;
            // 51fed cmovne ecx,eax
            if fl.ne() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 51ff0 lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 51ff4 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 51ff8 mov DWORD PTR [rbp-0x38],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rcx as u32 as u64)) as u32);
            // 51ffb mov r15d,0x71
            r15 = (0x71u64) as u32 as u64;
            // 52001 mov r13d,0x70
            r13 = (0x70u64) as u32 as u64;
            // 52007 jmp 0x5213d
            pc = 0x5213d; continue;
        }
        0x5200c => {
            // block 5200c
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 5200c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5200c mov DWORD PTR [rbp-0x48],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u32);
            // 52013 mov edi,DWORD PTR [rbp-0x3c]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64) as u32 as u64;
            // 52016 call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5201b mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 5201e movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 52021 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 52024 jb 0x52048
            if fl.b() { pc = 0x52048; continue; }
            pc = 0x52026;
        }
        0x52026 => {
            // block 52026
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52026 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52026 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52029 lea rdx,[rbp-0x48]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 5202d mov esi,0x2724
            rsi = (0x2724u64) as u32 as u64;
            // 52032 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52037 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5203a mov rdi,rbx
            rdi = rbx;
            // 5203d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52040 cmp DWORD PTR [rbp-0x48],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, 0x0u64).1;
            // 52044 sete r15b
            r15 = (r15 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x52048;
        }
        0x52048 => {
            // block 52048
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52048 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 5204b mov eax,0x1d
            rax = (0x1du64) as u32 as u64;
            // 52050 mov r12d,0x40
            r12 = (0x40u64) as u32 as u64;
            // 52056 cmovne r12d,eax
            if fl.ne() { r12 = ((rax as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // 5205a mov DWORD PTR [rbp-0x4c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), (0x0u64) as u32);
            // 52061 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52064 lea rdx,[rbp-0x4c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffb4);
            // 52068 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 5206d mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 52073 mov rdi,rbx
            rdi = rbx;
            // 52076 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 52079 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5207c cmp DWORD PTR [rbp-0x4c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64, 0x0u64).1;
            // 52080 je 0x520f9
            if fl.e() { pc = 0x520f9; continue; }
            pc = 0x52082;
        }
        0x52082 => {
            // block 52082
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52082 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52082 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52085 lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 52089 mov QWORD PTR [rbp-0x60],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rdx) as u64);
            // 5208d mov ecx,0xff
            rcx = (0xffu64) as u32 as u64;
            // 52092 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 52095 mov rdi,rbx
            rdi = rbx;
            // 52098 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 5209b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5209e cmp DWORD PTR [rbp-0x38],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, 0x0u64).1;
            // 520a2 mov eax,DWORD PTR [r14]
            rax = (m.r32(r14) as u64) as u32 as u64;
            // 520a5 je 0x520f5
            if fl.e() { pc = 0x520f5; continue; }
            pc = 0x520a7;
        }
        0x520a7 => {
            // block 520a7
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 520a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 520a7 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 520a9 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 520ac jbe 0x52100
            if fl.be() { pc = 0x52100; continue; }
            pc = 0x520ae;
        }
        0x520ae => {
            // block 520ae
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 520ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 520ae mov DWORD PTR [rbp-0x50],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u32);
            // 520b5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 520b8 lea rdx,[rbp-0x50]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 520bc mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 520c1 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 520c7 mov rdi,rbx
            rdi = rbx;
            // 520ca mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 520cd call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 520d0 cmp DWORD PTR [rbp-0x50],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64, 0x1u64).1;
            // 520d4 jne 0x520f9
            if fl.ne() { pc = 0x520f9; continue; }
            pc = 0x520d6;
        }
        0x520d6 => {
            // block 520d6
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 520d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 520d6 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 520d9 mov eax,0x6e
            rax = (0x6eu64) as u32 as u64;
            // 520de mov r13d,0xfc
            r13 = (0xfcu64) as u32 as u64;
            // 520e4 cmovne r13d,eax
            if fl.ne() { r13 = ((rax as u32 as u64)) as u32 as u64; } else { r13 = r13 as u32 as u64; }
            // 520e8 mov eax,0x6f
            rax = (0x6fu64) as u32 as u64;
            // 520ed mov r15d,0xfd
            r15 = (0xfdu64) as u32 as u64;
            // 520f3 jmp 0x5211d
            pc = 0x5211d; continue;
        }
        0x520f5 => {
            // block 520f5
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 520f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 520f5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 520f7 je 0x52100
            if fl.e() { pc = 0x52100; continue; }
            pc = 0x520f9;
        }
        0x520f9 => {
            // block 520f9
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 520f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 520f9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 520fb jmp 0x52203
            pc = 0x52203; continue;
        }
        0x52100 => {
            // block 52100
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52100 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52100 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 52103 mov eax,0x1e
            rax = (0x1eu64) as u32 as u64;
            // 52108 mov r13d,0x41
            r13 = (0x41u64) as u32 as u64;
            // 5210e cmovne r13d,eax
            if fl.ne() { r13 = ((rax as u32 as u64)) as u32 as u64; } else { r13 = r13 as u32 as u64; }
            // 52112 mov eax,0x1f
            rax = (0x1fu64) as u32 as u64;
            // 52117 mov r15d,0x42
            r15 = (0x42u64) as u32 as u64;
            pc = 0x5211d;
        }
        0x5211d => {
            // block 5211d
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 5211d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5211d cmovne r15d,eax
            if fl.ne() { r15 = ((rax as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // 52121 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52124 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52129 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5212c mov rdi,rbx
            rdi = rbx;
            // 5212f mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 52132 mov DWORD PTR [rbp-0x64],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), ((r12 as u32 as u64)) as u32);
            // 52136 lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 5213a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5213d;
        }
        0x5213d => {
            // block 5213d
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 5213d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5213d mov DWORD PTR [rbp-0x58],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((r13 as u32 as u64)) as u32);
            // 52141 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52144 lea rdx,[rbp-0x34]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 52148 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 5214b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52150 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 52153 mov rdi,rbx
            rdi = rbx;
            // 52156 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 52159 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5215c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5215f lea r13,[rbp-0x30]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 52163 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52168 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5216b mov rdi,rbx
            rdi = rbx;
            // 5216e mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 52171 mov rdx,r13
            rdx = r13;
            // 52174 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x52177;
        }
        0x52177 => {
            // block 52177
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52177 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52177 cmp DWORD PTR [rbp-0x54],0xfe
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64, 0xfeu64).1;
            // 5217e mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 52181 mov DWORD PTR [r14+0x8],eax
            m.w32(r14.wrapping_add(0x8), ((rax as u32 as u64)) as u32);
            // 52185 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 52189 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 5218c je 0x521a6
            if fl.e() { pc = 0x521a6; continue; }
            pc = 0x5218e;
        }
        0x5218e => {
            // block 5218e
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 5218e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5218e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52191 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52196 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 52199 mov rdi,rbx
            rdi = rbx;
            // 5219c mov esi,DWORD PTR [rbp-0x64]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 5219f mov rdx,QWORD PTR [rbp-0x60]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 521a3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x521a6;
        }
        0x521a6 => {
            // block 521a6
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521a6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 521a9 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 521ae xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 521b1 mov rdi,rbx
            rdi = rbx;
            // 521b4 mov esi,DWORD PTR [rbp-0x58]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // 521b7 lea rdx,[rbp-0x34]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 521bb call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 521be mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 521c1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 521c6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 521c9 mov rdi,rbx
            rdi = rbx;
            // 521cc mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 521cf mov rdx,r13
            rdx = r13;
            // 521d2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 521d5 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 521d8 cmp eax,DWORD PTR [r14]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14) as u64).1;
            // 521db jne 0x521f7
            if fl.ne() { pc = 0x521f7; continue; }
            pc = 0x521dd;
        }
        0x521dd => {
            // block 521dd
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521dd mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // 521e0 cmp eax,DWORD PTR [r14+0x4]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x4)) as u64).1;
            // 521e4 jne 0x521f7
            if fl.ne() { pc = 0x521f7; continue; }
            pc = 0x521e6;
        }
        0x521e6 => {
            // block 521e6
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521e6 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 521e9 cmp eax,DWORD PTR [r14+0x8]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x8)) as u64).1;
            // 521ed mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 521ef je 0x521f4
            if fl.e() { pc = 0x521f4; continue; }
            pc = 0x521f1;
        }
        0x521f1 => {
            // block 521f1
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521f1 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            pc = 0x521f4;
        }
        0x521f4 => {
            // block 521f4
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521f4 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x521f7;
        }
        0x521f7 => {
            // block 521f7
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 521f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 521f7 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 521f9 test r12b,0x1
            fl = logic::<8>((r12 as u8 as u64) & 0x1u64);
            // 521fd je 0x52177
            if fl.e() { pc = 0x52177; continue; }
            pc = 0x52203;
        }
        0x52203 => {
            // block 52203
            #[cfg(feature = "trace")] eprintln!("f_00051f58 pc 52203 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52203 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 52207 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 52208 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5220a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5220c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5220e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 52210 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 52211 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5987b size=525
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005987b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5987b;
    loop {
        match pc {
        0x5987b => {
            // block 5987b
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 5987b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5987b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5987c mov rbp,rsp
            rbp = rsp;
            // 5987f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 59881 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 59883 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 59885 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 59887 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 59888 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 5988c mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // 5988f mov rbx,rsi
            rbx = rsi;
            // 59892 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 59895 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 59898 je 0x59a5a
            if fl.e() { pc = 0x59a5a; continue; }
            pc = 0x5989e;
        }
        0x5989e => {
            // block 5989e
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 5989e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5989e test dx,dx
            fl = logic::<16>((rdx as u16 as u64) & (rdx as u16 as u64));
            // 598a1 je 0x59a5a
            if fl.e() { pc = 0x59a5a; continue; }
            pc = 0x598a7;
        }
        0x598a7 => {
            // block 598a7
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598a7 test r14w,r14w
            fl = logic::<16>((r14 as u16 as u64) & (r14 as u16 as u64));
            // 598ab je 0x59a5a
            if fl.e() { pc = 0x59a5a; continue; }
            pc = 0x598b1;
        }
        0x598b1 => {
            // block 598b1
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598b1 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 598b4 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 598b6 cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // 598b9 ja 0x59a5a
            if fl.a() { pc = 0x59a5a; continue; }
            pc = 0x598bf;
        }
        0x598bf => {
            // block 598bf
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598bf lea rcx,[rip+0x1a6] # 0x59a6c
            rcx = m.env.code_va(0x59a6c);
            // 598c6 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 598ca add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 598cd jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x598cf => {
            // block 598cf
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598cf mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 598d1 and ecx,0xfffc
            { let r = (rcx as u32 as u64) & 0xfffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 598d7 shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 598da xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 598dc test cx,cx
            fl = logic::<16>((rcx as u16 as u64) & (rcx as u16 as u64));
            // 598df je 0x59905
            if fl.e() { pc = 0x59905; continue; }
            pc = 0x598e1;
        }
        0x598e1 => {
            // block 598e1
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598e1 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            pc = 0x598e4;
        }
        0x598e4 => {
            // block 598e4
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598e4 cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // 598e7 je 0x598f7
            if fl.e() { pc = 0x598f7; continue; }
            pc = 0x598e9;
        }
        0x598e9 => {
            // block 598e9
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598e9 cmp edi,0x3
            fl = sub::<32>((rdi as u32 as u64), 0x3u64).1;
            // 598ec je 0x598f7
            if fl.e() { pc = 0x598f7; continue; }
            pc = 0x598ee;
        }
        0x598ee => {
            // block 598ee
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598ee cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // 598f1 jne 0x59a57
            if fl.ne() { pc = 0x59a57; continue; }
            pc = 0x598f7;
        }
        0x598f7 => {
            // block 598f7
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598f7 mov BYTE PTR [rbx+rax*4],0x0
            m.w8(rbx.wrapping_add(rax.wrapping_mul(4)), (0x0u64) as u8);
            // 598fb inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 598fe movzx esi,ax
            rsi = ((rax as u16 as u64)) as u32 as u64;
            // 59901 cmp esi,ecx
            fl = sub::<32>((rsi as u32 as u64), (rcx as u32 as u64)).1;
            // 59903 jb 0x598e4
            if fl.b() { pc = 0x598e4; continue; }
            pc = 0x59905;
        }
        0x59905 => {
            // block 59905
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59905 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59905 movzx r12d,dx
            r12 = ((rdx as u16 as u64)) as u32 as u64;
            // 59909 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 5990c movzx eax,r14w
            rax = ((r14 as u16 as u64)) as u32 as u64;
            // 59910 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 59913 jb 0x59a5a
            if fl.b() { pc = 0x59a5a; continue; }
            pc = 0x59919;
        }
        0x59919 => {
            // block 59919
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59919 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59919 lea r13,[rbx+r12*1]
            r13 = rbx.wrapping_add(r12);
            // 5991d dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            pc = 0x59920;
        }
        0x59920 => {
            // block 59920
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59920 mov rdi,r13
            rdi = r13;
            // 59923 mov rsi,rbx
            rsi = rbx;
            // 59926 mov rdx,r12
            rdx = r12;
            // 59929 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5992e add r13,r12
            { let (r, f) = add::<64>(r13, r12);
              r13 = r;
              fl = f; }
            // 59931 dec r14w
            { let (r, f) = dec::<16>((r14 as u16 as u64), fl); r14 = (r14 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // 59935 jne 0x59920
            if fl.ne() { pc = 0x59920; continue; }
            pc = 0x59937;
        }
        0x59937 => {
            // block 59937
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59937 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59937 jmp 0x59a5a
            pc = 0x59a5a; continue;
        }
        0x5993c => {
            // block 5993c
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 5993c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5993c mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 5993e and eax,0xfff0
            { let r = (rax as u32 as u64) & 0xfff0u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 59943 shr eax,0x4
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 59946 mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // 59949 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 5994b lea r12d,[rax+rax*2]
            r12 = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // 5994f movzx eax,r12w
            rax = ((r12 as u16 as u64)) as u32 as u64;
            // 59953 mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 59956 movzx eax,dx
            rax = ((rdx as u16 as u64)) as u32 as u64;
            // 59959 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 5995d and r12,0xfffe
            { let r = r12 & 0xfffeu64; let f = logic::<64>(r);
              r12 = r;
              fl = f; }
            // 59964 add r12,r12
            { let (r, f) = add::<64>(r12, r12);
              r12 = r;
              fl = f; }
            // 59967 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x5996a;
        }
        0x5996a => {
            // block 5996a
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 5996a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5996a mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 5996d test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 59970 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 59975 je 0x5998d
            if fl.e() { pc = 0x5998d; continue; }
            pc = 0x59977;
        }
        0x59977 => {
            // block 59977
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59977 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59977 mov WORD PTR [rbx+rax*2],0x200
            m.w16(rbx.wrapping_add(rax.wrapping_mul(2)), (0x200u64) as u16);
            // 5997d mov WORD PTR [rbx+rax*2+0x2],0x40
            m.w16(rbx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x2), (0x40u64) as u16);
            // 59984 add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 59988 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 5998b jl 0x59977
            if fl.l() { pc = 0x59977; continue; }
            pc = 0x5998d;
        }
        0x5998d => {
            // block 5998d
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 5998d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5998d mov rdi,rbx
            rdi = rbx;
            // 59990 mov rsi,rbx
            rsi = rbx;
            // 59993 mov edx,DWORD PTR [rbp-0x30]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 59996 call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5999b add rbx,QWORD PTR [rbp-0x38]
            { let (r, f) = add::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64);
              rbx = r;
              fl = f; }
            // 5999f inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // 599a2 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // 599a6 movzx ecx,r14w
            rcx = ((r14 as u16 as u64)) as u32 as u64;
            // 599aa mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 599ad cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // 599af jne 0x5996a
            if fl.ne() { pc = 0x5996a; continue; }
            pc = 0x599b1;
        }
        0x599b1 => {
            // block 599b1
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599b1 jmp 0x59a5a
            pc = 0x59a5a; continue;
        }
        0x599b6 => {
            // block 599b6
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599b6 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 599b8 and eax,0xfffe
            { let r = (rax as u32 as u64) & 0xfffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 599bd shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 599bf movzx r8d,dx
            r8 = ((rdx as u16 as u64)) as u32 as u64;
            // 599c3 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 599c5 add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // 599c7 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 599c9 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 599cb movzx edi,r14w
            rdi = ((r14 as u16 as u64)) as u32 as u64;
            pc = 0x599cf;
        }
        0x599cf => {
            // block 599cf
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599cf test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 599d2 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 599d7 je 0x599eb
            if fl.e() { pc = 0x599eb; continue; }
            pc = 0x599d9;
        }
        0x599d9 => {
            // block 599d9
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599d9 mov BYTE PTR [rbx+rcx*1],0x80
            m.w8(rbx.wrapping_add(rcx), (0x80u64) as u8);
            // 599dd mov BYTE PTR [rbx+rcx*1+0x1],0x10
            m.w8(rbx.wrapping_add(rcx).wrapping_add(0x1), (0x10u64) as u8);
            // 599e2 add rcx,0x2
            { let (r, f) = add::<64>(rcx, 0x2u64);
              rcx = r;
              fl = f; }
            // 599e6 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 599e9 jl 0x599d9
            if fl.l() { pc = 0x599d9; continue; }
            pc = 0x599eb;
        }
        0x599eb => {
            // block 599eb
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599eb add rbx,r8
            { let (r, f) = add::<64>(rbx, r8);
              rbx = r;
              fl = f; }
            // 599ee inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 599f0 movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // 599f3 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 599f6 cmp ecx,edi
            fl = sub::<32>((rcx as u32 as u64), (rdi as u32 as u64)).1;
            // 599f8 jne 0x599cf
            if fl.ne() { pc = 0x599cf; continue; }
            pc = 0x599fa;
        }
        0x599fa => {
            // block 599fa
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599fa jmp 0x59a5a
            pc = 0x59a5a; continue;
        }
        0x599fc => {
            // block 599fc
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599fc mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 599fe and eax,0xfffc
            { let r = (rax as u32 as u64) & 0xfffcu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 59a03 movzx r12d,dx
            r12 = ((rdx as u16 as u64)) as u32 as u64;
            // 59a07 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 59a0a test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 59a0d je 0x59a2b
            if fl.e() { pc = 0x59a2b; continue; }
            pc = 0x59a0f;
        }
        0x59a0f => {
            // block 59a0f
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a0f dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 59a11 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 59a14 lea rdx,[rax*4+0x4]
            rdx = rax.wrapping_mul(4).wrapping_add(0x4);
            // 59a1c lea rsi,[rip+0xa824d] # 0x101c70
            rsi = m.env.data_va(91197);
            // 59a23 mov rdi,rbx
            rdi = rbx;
            // 59a26 call 0xec230
            { let r = { let t = m.env.stub_va(0x9fd); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x59a2b;
        }
        0x59a2b => {
            // block 59a2b
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a2b mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 59a2e movzx eax,r14w
            rax = ((r14 as u16 as u64)) as u32 as u64;
            // 59a32 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 59a35 jb 0x59a5a
            if fl.b() { pc = 0x59a5a; continue; }
            pc = 0x59a37;
        }
        0x59a37 => {
            // block 59a37
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a37 lea r13,[rbx+r12*1]
            r13 = rbx.wrapping_add(r12);
            // 59a3b dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            pc = 0x59a3e;
        }
        0x59a3e => {
            // block 59a3e
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a3e mov rdi,r13
            rdi = r13;
            // 59a41 mov rsi,rbx
            rsi = rbx;
            // 59a44 mov rdx,r12
            rdx = r12;
            // 59a47 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59a4c add r13,r12
            { let (r, f) = add::<64>(r13, r12);
              r13 = r;
              fl = f; }
            // 59a4f dec r14w
            { let (r, f) = dec::<16>((r14 as u16 as u64), fl); r14 = (r14 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // 59a53 jne 0x59a3e
            if fl.ne() { pc = 0x59a3e; continue; }
            pc = 0x59a55;
        }
        0x59a55 => {
            // block 59a55
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a55 jmp 0x59a5a
            pc = 0x59a5a; continue;
        }
        0x59a57 => {
            // block 59a57
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a57 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x59a5a;
        }
        0x59a5a => {
            // block 59a5a
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a5a mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 59a5d add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 59a61 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 59a62 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 59a64 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 59a66 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 59a68 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 59a6a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 59a6b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a96d size=491
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a96d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5a96d;
    loop {
        match pc {
        0x5a96d => {
            // block 5a96d
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a96d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a96d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a96e mov rbp,rsp
            rbp = rsp;
            // 5a971 sub rsp,0x40
            { let (r, f) = sub::<64>(rsp, 0x40u64);
              rsp = r;
              fl = f; }
            // 5a975 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 5a977 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 5a979 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a97b test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // 5a97d je 0x5ab3f
            if fl.e() { pc = 0x5ab3f; continue; }
            pc = 0x5a983;
        }
        0x5a983 => {
            // block 5a983
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a983 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a983 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 5a986 ja 0x5ab3f
            if fl.a() { pc = 0x5ab3f; continue; }
            pc = 0x5a98c;
        }
        0x5a98c => {
            // block 5a98c
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a98c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a98c lea r8d,[rdi-0x1]
            r8 = (rdi.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 5a990 cmp r8d,0x1e
            fl = sub::<32>((r8 as u32 as u64), 0x1eu64).1;
            // 5a994 jb 0x5a9c4
            if fl.b() { pc = 0x5a9c4; continue; }
            pc = 0x5a996;
        }
        0x5a996 => {
            // block 5a996
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a996 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a996 lea esi,[rdi-0x6e]
            rsi = (rdi.wrapping_add(0xffff_ffff_ffff_ff92)) as u32 as u64;
            // 5a999 cmp esi,0xc
            fl = sub::<32>((rsi as u32 as u64), 0xcu64).1;
            // 5a99c jb 0x5a9c4
            if fl.b() { pc = 0x5a9c4; continue; }
            pc = 0x5a99e;
        }
        0x5a99e => {
            // block 5a99e
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a99e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a99e cmp edi,0x6d
            fl = sub::<32>((rdi as u32 as u64), 0x6du64).1;
            // 5a9a1 jg 0x5a9bf
            if fl.g() { pc = 0x5a9bf; continue; }
            pc = 0x5a9a3;
        }
        0x5a9a3 => {
            // block 5a9a3
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9a3 lea r9d,[rdi-0xd]
            r9 = (rdi.wrapping_add(0xffff_ffff_ffff_fff3)) as u32 as u64;
            // 5a9a7 cmp r9d,0x37
            fl = sub::<32>((r9 as u32 as u64), 0x37u64).1;
            // 5a9ab ja 0x5a9ce
            if fl.a() { pc = 0x5a9ce; continue; }
            pc = 0x5a9ad;
        }
        0x5a9ad => {
            // block 5a9ad
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9ad movabs rsi,0xf8000001f8600f
            rsi = 0xf8_0000_01f8_600fu64;
            // 5a9b7 bt rsi,r9
            fl.cf = bit::<64>(rsi, r9);
            // 5a9bb jb 0x5a9c4
            if fl.b() { pc = 0x5a9c4; continue; }
            pc = 0x5a9bd;
        }
        0x5a9bd => {
            // block 5a9bd
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9bd jmp 0x5a9ce
            pc = 0x5a9ce; continue;
        }
        0x5a9bf => {
            // block 5a9bf
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9bf cmp esi,0xc
            fl = sub::<32>((rsi as u32 as u64), 0xcu64).1;
            // 5a9c2 jae 0x5a9ce
            if fl.ae() { pc = 0x5a9ce; continue; }
            pc = 0x5a9c4;
        }
        0x5a9c4 => {
            // block 5a9c4
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9c4 cmp edx,0x1f
            fl = sub::<32>((rdx as u32 as u64), 0x1fu64).1;
            // 5a9c7 jbe 0x5a9e3
            if fl.be() { pc = 0x5a9e3; continue; }
            pc = 0x5a9c9;
        }
        0x5a9c9 => {
            // block 5a9c9
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9c9 jmp 0x5ab3f
            pc = 0x5ab3f; continue;
        }
        0x5a9ce => {
            // block 5a9ce
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9ce lea esi,[rdi-0x50]
            rsi = (rdi.wrapping_add(0xffff_ffff_ffff_ffb0)) as u32 as u64;
            // 5a9d1 cmp esi,0x1d
            fl = sub::<32>((rsi as u32 as u64), 0x1du64).1;
            // 5a9d4 ja 0x5ab3f
            if fl.a() { pc = 0x5ab3f; continue; }
            pc = 0x5a9da;
        }
        0x5a9da => {
            // block 5a9da
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9da cmp edx,0x20
            fl = sub::<32>((rdx as u32 as u64), 0x20u64).1;
            // 5a9dd jae 0x5ab3f
            if fl.ae() { pc = 0x5ab3f; continue; }
            pc = 0x5a9e3;
        }
        0x5a9e3 => {
            // block 5a9e3
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9e3 lea esi,[rdx-0x1c]
            rsi = (rdx.wrapping_add(0xffff_ffff_ffff_ffe4)) as u32 as u64;
            // 5a9e6 cmp esi,0x3
            fl = sub::<32>((rsi as u32 as u64), 0x3u64).1;
            // 5a9e9 jbe 0x5aa4e
            if fl.be() { pc = 0x5aa4e; continue; }
            pc = 0x5a9eb;
        }
        0x5a9eb => {
            // block 5a9eb
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9eb cmp r8d,0x78
            fl = sub::<32>((r8 as u32 as u64), 0x78u64).1;
            // 5a9ef ja 0x5ab3f
            if fl.a() { pc = 0x5ab3f; continue; }
            pc = 0x5a9f5;
        }
        0x5a9f5 => {
            // block 5a9f5
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5a9f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a9f5 movsxd rsi,r8d
            rsi = sx::<32>((r8 as u32 as u64)) as u64;
            // 5a9f8 lea rdi,[rip+0xa6991] # 0x101390
            rdi = m.env.data_va(88925);
            // 5a9ff mov esi,DWORD PTR [rdi+rsi*4]
            rsi = (m.r32(rdi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // 5aa02 cmp esi,0xb
            fl = sub::<32>((rsi as u32 as u64), 0xbu64).1;
            // 5aa05 ja 0x5ab3f
            if fl.a() { pc = 0x5ab3f; continue; }
            pc = 0x5aa0b;
        }
        0x5aa0b => {
            // block 5aa0b
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa0b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 5aa0f call 0x32788
            { let r = { let t = m.env.code_va(0x32788); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5aa14 mov ecx,DWORD PTR [rbp-0x40]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 5aa17 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5aa19 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 5aa1b je 0x5ab3f
            if fl.e() { pc = 0x5ab3f; continue; }
            pc = 0x5aa21;
        }
        0x5aa21 => {
            // block 5aa21
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa21 cmp DWORD PTR [rbp-0x3c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64, 0x0u64).1;
            // 5aa25 je 0x5ab3f
            if fl.e() { pc = 0x5ab3f; continue; }
            pc = 0x5aa2b;
        }
        0x5aa2b => {
            // block 5aa2b
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa2b cmp WORD PTR [rbp-0xc],0x0
            fl = sub::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 5aa30 je 0x5ab3f
            if fl.e() { pc = 0x5ab3f; continue; }
            pc = 0x5aa36;
        }
        0x5aa36 => {
            // block 5aa36
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa36 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // 5aa3a je 0x5ab3f
            if fl.e() { pc = 0x5ab3f; continue; }
            pc = 0x5aa40;
        }
        0x5aa40 => {
            // block 5aa40
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa40 imul ecx,DWORD PTR [rbp-0x38]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // 5aa44 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 5aa47 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 5aa49 jmp 0x5ab3f
            pc = 0x5ab3f; continue;
        }
        0x5aa4e => {
            // block 5aa4e
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa4e lea rax,[rip+0xf3] # 0x5ab48
            rax = m.env.code_va(0x5ab48);
            // 5aa55 movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // 5aa59 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 5aa5c jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x5aa5e => {
            // block 5aa5e
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa5e cmp r8d,0x78
            fl = sub::<32>((r8 as u32 as u64), 0x78u64).1;
            // 5aa62 ja 0x5aab2
            if fl.a() { pc = 0x5aab2; continue; }
            pc = 0x5aa64;
        }
        0x5aa64 => {
            // block 5aa64
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa64 movsxd rax,r8d
            rax = sx::<32>((r8 as u32 as u64)) as u64;
            // 5aa67 lea rcx,[rip+0xa66c2] # 0x101130
            rcx = m.env.data_va(88317);
            // 5aa6e mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5aa71 jmp 0x5aabe
            pc = 0x5aabe; continue;
        }
        0x5aa73 => {
            // block 5aa73
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa73 cmp r8d,0x78
            fl = sub::<32>((r8 as u32 as u64), 0x78u64).1;
            // 5aa77 ja 0x5aad7
            if fl.a() { pc = 0x5aad7; continue; }
            pc = 0x5aa79;
        }
        0x5aa79 => {
            // block 5aa79
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa79 movsxd rax,r8d
            rax = sx::<32>((r8 as u32 as u64)) as u64;
            // 5aa7c lea rcx,[rip+0xa66ad] # 0x101130
            rcx = m.env.data_va(88317);
            // 5aa83 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5aa86 jmp 0x5aae3
            pc = 0x5aae3; continue;
        }
        0x5aa88 => {
            // block 5aa88
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa88 cmp r8d,0x78
            fl = sub::<32>((r8 as u32 as u64), 0x78u64).1;
            // 5aa8c ja 0x5aafe
            if fl.a() { pc = 0x5aafe; continue; }
            pc = 0x5aa8e;
        }
        0x5aa8e => {
            // block 5aa8e
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa8e movsxd rax,r8d
            rax = sx::<32>((r8 as u32 as u64)) as u64;
            // 5aa91 lea rcx,[rip+0xa6698] # 0x101130
            rcx = m.env.data_va(88317);
            // 5aa98 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5aa9b jmp 0x5ab0a
            pc = 0x5ab0a; continue;
        }
        0x5aa9d => {
            // block 5aa9d
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aa9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aa9d cmp r8d,0x78
            fl = sub::<32>((r8 as u32 as u64), 0x78u64).1;
            // 5aaa1 ja 0x5ab21
            if fl.a() { pc = 0x5ab21; continue; }
            pc = 0x5aaa3;
        }
        0x5aaa3 => {
            // block 5aaa3
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aaa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aaa3 movsxd rax,r8d
            rax = sx::<32>((r8 as u32 as u64)) as u64;
            // 5aaa6 lea rcx,[rip+0xa6683] # 0x101130
            rcx = m.env.data_va(88317);
            // 5aaad mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5aab0 jmp 0x5ab2d
            pc = 0x5ab2d; continue;
        }
        0x5aab2 => {
            // block 5aab2
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aab2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aab2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5aab4 cmp edi,0x7a
            fl = sub::<32>((rdi as u32 as u64), 0x7au64).1;
            // 5aab7 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 5aabc ja 0x5aacb
            if fl.a() { pc = 0x5aacb; continue; }
            pc = 0x5aabe;
        }
        0x5aabe => {
            // block 5aabe
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aabe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aabe movsxd rcx,edi
            rcx = sx::<32>((rdi as u32 as u64)) as u64;
            // 5aac1 lea rdx,[rip+0xa6478] # 0x100f40
            rdx = m.env.data_va(87821);
            // 5aac8 mov ecx,DWORD PTR [rdx+rcx*4]
            rcx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5aacb;
        }
        0x5aacb => {
            // block 5aacb
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aacb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aacb imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 5aace lea eax,[rax+rax*2]
            rax = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // 5aad1 shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 5aad3 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 5aad5 jmp 0x5aaf6
            pc = 0x5aaf6; continue;
        }
        0x5aad7 => {
            // block 5aad7
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aad7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aad7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5aad9 cmp edi,0x7a
            fl = sub::<32>((rdi as u32 as u64), 0x7au64).1;
            // 5aadc mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 5aae1 ja 0x5aaf0
            if fl.a() { pc = 0x5aaf0; continue; }
            pc = 0x5aae3;
        }
        0x5aae3 => {
            // block 5aae3
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aae3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aae3 movsxd rcx,edi
            rcx = sx::<32>((rdi as u32 as u64)) as u64;
            // 5aae6 lea rdx,[rip+0xa6453] # 0x100f40
            rdx = m.env.data_va(87821);
            // 5aaed mov ecx,DWORD PTR [rdx+rcx*4]
            rcx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5aaf0;
        }
        0x5aaf0 => {
            // block 5aaf0
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aaf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aaf0 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 5aaf3 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0x5aaf6;
        }
        0x5aaf6 => {
            // block 5aaf6
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aaf6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aaf6 lea eax,[rax+rax*4]
            rax = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // 5aaf9 shr eax,0x3
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x3u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 5aafc jmp 0x5ab3f
            pc = 0x5ab3f; continue;
        }
        0x5aafe => {
            // block 5aafe
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5aafe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5aafe xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5ab00 cmp edi,0x7a
            fl = sub::<32>((rdi as u32 as u64), 0x7au64).1;
            // 5ab03 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 5ab08 ja 0x5ab17
            if fl.a() { pc = 0x5ab17; continue; }
            pc = 0x5ab0a;
        }
        0x5ab0a => {
            // block 5ab0a
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab0a movsxd rcx,edi
            rcx = sx::<32>((rdi as u32 as u64)) as u64;
            // 5ab0d lea rdx,[rip+0xa642c] # 0x100f40
            rdx = m.env.data_va(87821);
            // 5ab14 mov ecx,DWORD PTR [rdx+rcx*4]
            rcx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5ab17;
        }
        0x5ab17 => {
            // block 5ab17
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab17 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 5ab1a lea eax,[rax+rax*2]
            rax = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // 5ab1d shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 5ab1f jmp 0x5ab3f
            pc = 0x5ab3f; continue;
        }
        0x5ab21 => {
            // block 5ab21
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab21 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5ab23 cmp edi,0x7a
            fl = sub::<32>((rdi as u32 as u64), 0x7au64).1;
            // 5ab26 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 5ab2b ja 0x5ab3a
            if fl.a() { pc = 0x5ab3a; continue; }
            pc = 0x5ab2d;
        }
        0x5ab2d => {
            // block 5ab2d
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab2d movsxd rcx,edi
            rcx = sx::<32>((rdi as u32 as u64)) as u64;
            // 5ab30 lea rdx,[rip+0xa6409] # 0x100f40
            rdx = m.env.data_va(87821);
            // 5ab37 mov ecx,DWORD PTR [rdx+rcx*4]
            rcx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5ab3a;
        }
        0x5ab3a => {
            // block 5ab3a
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab3a imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 5ab3d add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5ab3f;
        }
        0x5ab3f => {
            // block 5ab3f
            #[cfg(feature = "trace")] eprintln!("f_0005a96d pc 5ab3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab3f add rsp,0x40
            { let (r, f) = add::<64>(rsp, 0x40u64);
              rsp = r;
              fl = f; }
            // 5ab43 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5ab44 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5b092 size=641
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005b092<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5b092;
    loop {
        match pc {
        0x5b092 => {
            // block 5b092
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b092 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5b093 mov rbp,rsp
            rbp = rsp;
            // 5b096 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 5b098 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 5b09b jne 0x5b0a4
            if fl.ne() { pc = 0x5b0a4; continue; }
            pc = 0x5b09d;
        }
        0x5b09d => {
            // block 5b09d
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b09d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b09d mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 5b09f jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b0a4 => {
            // block 5b0a4
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0a4 cmp rsi,0x64
            fl = sub::<64>(rsi, 0x64u64).1;
            // 5b0a8 jne 0x5b0e0
            if fl.ne() { pc = 0x5b0e0; continue; }
            pc = 0x5b0aa;
        }
        0x5b0aa => {
            // block 5b0aa
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0aa mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5b0af cmp rdi,0x12ba
            fl = sub::<64>(rdi, 0x12bau64).1;
            // 5b0b6 jg 0x5b19b
            if fl.g() { pc = 0x5b19b; continue; }
            pc = 0x5b0bc;
        }
        0x5b0bc => {
            // block 5b0bc
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0bc cmp rdi,0x95d
            fl = sub::<64>(rdi, 0x95du64).1;
            // 5b0c3 jg 0x5b1c8
            if fl.g() { pc = 0x5b1c8; continue; }
            pc = 0x5b0c9;
        }
        0x5b0c9 => {
            // block 5b0c9
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0c9 cmp rdi,0x5da
            fl = sub::<64>(rdi, 0x5dau64).1;
            // 5b0d0 jne 0x5b27f
            if fl.ne() { pc = 0x5b27f; continue; }
            pc = 0x5b0d6;
        }
        0x5b0d6 => {
            // block 5b0d6
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0d6 mov eax,0xe
            rax = (0xeu64) as u32 as u64;
            // 5b0db jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b0e0 => {
            // block 5b0e0
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0e0 mov rax,rdi
            rax = rdi;
            // 5b0e3 cqo
            rdx = if (rax as i64) < 0 { u64::MAX } else { 0 };
            // 5b0e5 idiv rsi
            { let (q, r) = idiv::<64>(rdx, rax, rsi); rax = q; rdx = r; }
            // 5b0e8 mov rdx,rax
            rdx = rax;
            // 5b0eb mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5b0f0 cmp ecx,0xe
            fl = sub::<32>((rcx as u32 as u64), 0xeu64).1;
            // 5b0f3 ja 0x5b311
            if fl.a() { pc = 0x5b311; continue; }
            pc = 0x5b0f9;
        }
        0x5b0f9 => {
            // block 5b0f9
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b0f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b0f9 cvtsi2ss xmm0,rdx
            x0 = set_lo32(x0, (sx::<64>(rdx) as f32).to_bits());
            // 5b0fe mulss xmm0,DWORD PTR [rip+0xa56aa] # 0x1007b0
            x0 = set_lo32(x0, (f32_of(x0) * f32_of((m.r32(m.env.data_va(85885)) as u128))).to_bits());
            // 5b106 cvttss2si rdx,xmm0
            rdx = cvtt64(f32_of(x0) as f64) as u64;
            // 5b10b mov esi,0x2b6a
            rsi = (0x2b6au64) as u32 as u64;
            // 5b110 bt esi,ecx
            fl.cf = bit::<32>((rsi as u32 as u64), (rcx as u32 as u64));
            // 5b113 jae 0x5b210
            if fl.ae() { pc = 0x5b210; continue; }
            pc = 0x5b119;
        }
        0x5b119 => {
            // block 5b119
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b119 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b119 mov eax,0xd
            rax = (0xdu64) as u32 as u64;
            // 5b11e cmp rdx,0x641
            fl = sub::<64>(rdx, 0x641u64).1;
            // 5b125 jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b12b;
        }
        0x5b12b => {
            // block 5b12b
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b12b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b12b mov eax,0x6
            rax = (0x6u64) as u32 as u64;
            // 5b130 cmp rdx,0x993
            fl = sub::<64>(rdx, 0x993u64).1;
            // 5b137 jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b13d;
        }
        0x5b13d => {
            // block 5b13d
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b13d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b13d mov eax,0x5
            rax = (0x5u64) as u32 as u64;
            // 5b142 cmp rdx,0xa29
            fl = sub::<64>(rdx, 0xa29u64).1;
            // 5b149 jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b14f;
        }
        0x5b14f => {
            // block 5b14f
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b14f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b14f mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 5b154 cmp rdx,0xc1d
            fl = sub::<64>(rdx, 0xc1du64).1;
            // 5b15b jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b161;
        }
        0x5b161 => {
            // block 5b161
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b161 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b161 mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // 5b166 cmp rdx,0x1325
            fl = sub::<64>(rdx, 0x1325u64).1;
            // 5b16d jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b173;
        }
        0x5b173 => {
            // block 5b173
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b173 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b173 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // 5b178 cmp rdx,0x13ed
            fl = sub::<64>(rdx, 0x13edu64).1;
            // 5b17f jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b185;
        }
        0x5b185 => {
            // block 5b185
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b185 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b185 cmp rdx,0x17d5
            fl = sub::<64>(rdx, 0x17d5u64).1;
            // 5b18c mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 5b191 mov eax,0xb
            rax = (0xbu64) as u32 as u64;
            // 5b196 jmp 0x5b277
            pc = 0x5b277; continue;
        }
        0x5b19b => {
            // block 5b19b
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b19b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b19b cmp rdi,0x2ed3
            fl = sub::<64>(rdi, 0x2ed3u64).1;
            // 5b1a2 jg 0x5b1f9
            if fl.g() { pc = 0x5b1f9; continue; }
            pc = 0x5b1a4;
        }
        0x5b1a4 => {
            // block 5b1a4
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1a4 cmp rdi,0x1387
            fl = sub::<64>(rdi, 0x1387u64).1;
            // 5b1ab jg 0x5b293
            if fl.g() { pc = 0x5b293; continue; }
            pc = 0x5b1b1;
        }
        0x5b1b1 => {
            // block 5b1b1
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1b1 cmp rdi,0x12bb
            fl = sub::<64>(rdi, 0x12bbu64).1;
            // 5b1b8 jne 0x5b2c3
            if fl.ne() { pc = 0x5b2c3; continue; }
            pc = 0x5b1be;
        }
        0x5b1be => {
            // block 5b1be
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1be mov eax,0xa
            rax = (0xau64) as u32 as u64;
            // 5b1c3 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b1c8 => {
            // block 5b1c8
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1c8 cmp rdi,0xbb7
            fl = sub::<64>(rdi, 0xbb7u64).1;
            // 5b1cf jg 0x5b2a3
            if fl.g() { pc = 0x5b2a3; continue; }
            pc = 0x5b1d5;
        }
        0x5b1d5 => {
            // block 5b1d5
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1d5 cmp rdi,0x9c3
            fl = sub::<64>(rdi, 0x9c3u64).1;
            // 5b1dc jg 0x5b2d3
            if fl.g() { pc = 0x5b2d3; continue; }
            pc = 0x5b1e2;
        }
        0x5b1e2 => {
            // block 5b1e2
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1e2 cmp rdi,0x95e
            fl = sub::<64>(rdi, 0x95eu64).1;
            // 5b1e9 jne 0x5b2f3
            if fl.ne() { pc = 0x5b2f3; continue; }
            pc = 0x5b1ef;
        }
        0x5b1ef => {
            // block 5b1ef
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1ef mov eax,0x7
            rax = (0x7u64) as u32 as u64;
            // 5b1f4 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b1f9 => {
            // block 5b1f9
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b1f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b1f9 cmp rdi,0x2ed4
            fl = sub::<64>(rdi, 0x2ed4u64).1;
            // 5b200 jne 0x5b2b3
            if fl.ne() { pc = 0x5b2b3; continue; }
            pc = 0x5b206;
        }
        0x5b206 => {
            // block 5b206
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b206 mov eax,0xc
            rax = (0xcu64) as u32 as u64;
            // 5b20b jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b210 => {
            // block 5b210
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b210 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b210 mov esi,0x5494
            rsi = (0x5494u64) as u32 as u64;
            // 5b215 bt esi,ecx
            fl.cf = bit::<32>((rsi as u32 as u64), (rcx as u32 as u64));
            // 5b218 jae 0x5b311
            if fl.ae() { pc = 0x5b311; continue; }
            pc = 0x5b21e;
        }
        0x5b21e => {
            // block 5b21e
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b21e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b21e mov eax,0xe
            rax = (0xeu64) as u32 as u64;
            // 5b223 cmp rdx,0x63f
            fl = sub::<64>(rdx, 0x63fu64).1;
            // 5b22a jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b230;
        }
        0x5b230 => {
            // block 5b230
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b230 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b230 mov eax,0x7
            rax = (0x7u64) as u32 as u64;
            // 5b235 cmp rdx,0x9c3
            fl = sub::<64>(rdx, 0x9c3u64).1;
            // 5b23c jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b242;
        }
        0x5b242 => {
            // block 5b242
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b242 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b242 mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            // 5b247 cmp rdx,0xc1a
            fl = sub::<64>(rdx, 0xc1au64).1;
            // 5b24e jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b254;
        }
        0x5b254 => {
            // block 5b254
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b254 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b254 mov eax,0xa
            rax = (0xau64) as u32 as u64;
            // 5b259 cmp rdx,0x1320
            fl = sub::<64>(rdx, 0x1320u64).1;
            // 5b260 jl 0x5b311
            if fl.l() { pc = 0x5b311; continue; }
            pc = 0x5b266;
        }
        0x5b266 => {
            // block 5b266
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b266 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b266 cmp rdx,0x17cf
            fl = sub::<64>(rdx, 0x17cfu64).1;
            // 5b26d mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 5b272 mov eax,0xc
            rax = (0xcu64) as u32 as u64;
            pc = 0x5b277;
        }
        0x5b277 => {
            // block 5b277
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b277 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b277 cmovl eax,ecx
            if fl.l() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 5b27a jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b27f => {
            // block 5b27f
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b27f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b27f cmp rdi,0x5dc
            fl = sub::<64>(rdi, 0x5dcu64).1;
            // 5b286 jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b28c;
        }
        0x5b28c => {
            // block 5b28c
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b28c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b28c mov eax,0xd
            rax = (0xdu64) as u32 as u64;
            // 5b291 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b293 => {
            // block 5b293
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b293 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b293 cmp rdi,0x1388
            fl = sub::<64>(rdi, 0x1388u64).1;
            // 5b29a jne 0x5b2e3
            if fl.ne() { pc = 0x5b2e3; continue; }
            pc = 0x5b29c;
        }
        0x5b29c => {
            // block 5b29c
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b29c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b29c mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // 5b2a1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2a3 => {
            // block 5b2a3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2a3 cmp rdi,0xbb8
            fl = sub::<64>(rdi, 0xbb8u64).1;
            // 5b2aa jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b2ac;
        }
        0x5b2ac => {
            // block 5b2ac
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2ac mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 5b2b1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2b3 => {
            // block 5b2b3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2b3 cmp rdi,0x2ee0
            fl = sub::<64>(rdi, 0x2ee0u64).1;
            // 5b2ba jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b2bc;
        }
        0x5b2bc => {
            // block 5b2bc
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2bc mov eax,0xb
            rax = (0xbu64) as u32 as u64;
            // 5b2c1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2c3 => {
            // block 5b2c3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2c3 cmp rdi,0x12c0
            fl = sub::<64>(rdi, 0x12c0u64).1;
            // 5b2ca jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b2cc;
        }
        0x5b2cc => {
            // block 5b2cc
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2cc mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // 5b2d1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2d3 => {
            // block 5b2d3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2d3 cmp rdi,0x9c4
            fl = sub::<64>(rdi, 0x9c4u64).1;
            // 5b2da jne 0x5b303
            if fl.ne() { pc = 0x5b303; continue; }
            pc = 0x5b2dc;
        }
        0x5b2dc => {
            // block 5b2dc
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2dc mov eax,0x5
            rax = (0x5u64) as u32 as u64;
            // 5b2e1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2e3 => {
            // block 5b2e3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2e3 cmp rdi,0x176a
            fl = sub::<64>(rdi, 0x176au64).1;
            // 5b2ea jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b2ec;
        }
        0x5b2ec => {
            // block 5b2ec
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2ec mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // 5b2f1 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b2f3 => {
            // block 5b2f3
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2f3 cmp rdi,0x960
            fl = sub::<64>(rdi, 0x960u64).1;
            // 5b2fa jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b2fc;
        }
        0x5b2fc => {
            // block 5b2fc
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b2fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b2fc mov eax,0x6
            rax = (0x6u64) as u32 as u64;
            // 5b301 jmp 0x5b311
            pc = 0x5b311; continue;
        }
        0x5b303 => {
            // block 5b303
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b303 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b303 cmp rdi,0xbb5
            fl = sub::<64>(rdi, 0xbb5u64).1;
            // 5b30a jne 0x5b311
            if fl.ne() { pc = 0x5b311; continue; }
            pc = 0x5b30c;
        }
        0x5b30c => {
            // block 5b30c
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b30c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b30c mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            pc = 0x5b311;
        }
        0x5b311 => {
            // block 5b311
            #[cfg(feature = "trace")] eprintln!("f_0005b092 pc 5b311 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5b311 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5b312 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5be20 size=537
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0005be20<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x5be20;
    loop {
        match pc {
        0x5be20 => {
            // block 5be20
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5be20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5be20 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5be21 mov rbp,rsp
            rbp = rsp;
            // 5be24 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5be26 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5be28 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 5be2a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5be2c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5be2d sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 5be34 mov QWORD PTR [rbp-0x160],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rcx) as u64);
            // 5be3b mov DWORD PTR [rbp-0x158],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea8), ((rdx as u32 as u64)) as u32);
            // 5be41 mov QWORD PTR [rbp-0x168],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rsi) as u64);
            // 5be48 mov r13,rdi
            r13 = rdi;
            // 5be4b mov edx,DWORD PTR [rbp+0x18]
            rdx = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 5be4e mov rax,QWORD PTR [rip+0xe4583] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // 5be55 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 5be58 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 5be5c lea ecx,[r9-0x1]
            rcx = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 5be60 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5be65 shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5be67 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 5be69 je 0x5bead
            if fl.e() { pc = 0x5bead; continue; }
            pc = 0x5be6b;
        }
        0x5be6b => {
            // block 5be6b
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5be6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5be6b mov rcx,QWORD PTR [rbp-0x168]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 5be72 mov ecx,DWORD PTR [rcx]
            rcx = (m.r32(rcx) as u64) as u32 as u64;
            // 5be74 cvtsi2sd xmm1,rcx
            x1 = set_lo64(x1, (sx::<64>(rcx) as f64).to_bits());
            // 5be79 lea rcx,[rbp-0x130]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 5be80 lea rdx,[rbp-0xb0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 5be87 mov rsi,r8
            rsi = r8;
            // 5be8a mov edi,DWORD PTR [rbp+0x18]
            rdi = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            pc = 0x5be8d;
        }
        0x5be8d => {
            // block 5be8d
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5be8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5be8d movapd xmm2,xmm0
            x2 = x0;
            // 5be91 divsd xmm2,QWORD PTR [rsi]
            x2 = set_lo64(x2, (f64_of(x2) / f64_of((m.r64(rsi) as u128))).to_bits());
            // 5be95 movsd QWORD PTR [rcx],xmm2
            m.w64(rcx, x2 as u64);
            // 5be99 movsd QWORD PTR [rdx],xmm1
            m.w64(rdx, x1 as u64);
            // 5be9d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 5bea1 add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 5bea5 add rdx,0x8
            { let (r, f) = add::<64>(rdx, 0x8u64);
              rdx = r;
              fl = f; }
            // 5bea9 dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 5beab jne 0x5be8d
            if fl.ne() { pc = 0x5be8d; continue; }
            pc = 0x5bead;
        }
        0x5bead => {
            // block 5bead
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bead rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bead test r13,r13
            fl = logic::<64>(r13 & r13);
            // 5beb0 sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5beb3 cmp QWORD PTR [rbp-0x160],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64, 0x0u64).1;
            // 5bebb sete dl
            rdx = (rdx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5bebe test r8,r8
            fl = logic::<64>(r8 & r8);
            // 5bec1 sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5bec4 cmp DWORD PTR [rbp-0x158],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64, 0x0u64).1;
            // 5becb je 0x5c003
            if fl.e() { pc = 0x5c003; continue; }
            pc = 0x5bed1;
        }
        0x5bed1 => {
            // block 5bed1
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bed1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bed1 or cl,dl
            { let r = (rcx as u8 as u64) | (rdx as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 5bed3 or cl,bl
            { let r = (rcx as u8 as u64) | (rbx as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 5bed5 jne 0x5c003
            if fl.ne() { pc = 0x5c003; continue; }
            pc = 0x5bedb;
        }
        0x5bedb => {
            // block 5bedb
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bedb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bedb cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 5bee0 addsd xmm0,QWORD PTR [rip+0xa48a0] # 0x100788
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(85845)) as u128))).to_bits());
            // 5bee8 movsd QWORD PTR [rbp-0x148],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), x0 as u64);
            // 5bef0 mov eax,DWORD PTR [rbp+0x18]
            rax = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 5bef3 lea eax,[rax-0x1]
            rax = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 5bef6 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 5bef9 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 5bf00 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0x5bf02;
        }
        0x5bf02 => {
            // block 5bf02
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bf02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bf02 mov DWORD PTR [rbp-0x154],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feac), ((rcx as u32 as u64)) as u32);
            // 5bf08 mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            // 5bf0f mov eax,DWORD PTR [rbp+0x18]
            rax = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 5bf12 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 5bf14 mov r14,QWORD PTR [rbp-0x160]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 5bf1b lea r12,[rbp-0x130]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fed0);
            // 5bf22 lea rbx,[rbp-0xb0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 5bf29 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // 5bf2c je 0x5bfdf
            if fl.e() { pc = 0x5bfdf; continue; }
            pc = 0x5bf32;
        }
        0x5bf32 => {
            // block 5bf32
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bf32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bf32 movsd xmm0,QWORD PTR [rbx]
            x0 = m.r64(rbx) as u128;
            // 5bf36 movsd QWORD PTR [rbp-0x138],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), x0 as u64);
            // 5bf3e movsd xmm1,QWORD PTR [r12]
            x1 = m.r64(r12) as u128;
            // 5bf44 movsd QWORD PTR [rbp-0x140],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), x1 as u64);
            // 5bf4c divsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x1)).to_bits());
            // 5bf50 mulsd xmm0,QWORD PTR [rip+0xa4838] # 0x100790
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85853)) as u128))).to_bits());
            // 5bf58 call 0xec290
            { let r = { let t = m.env.stub_va(0xa5d); m.call_r(t, &[], &[x0 as u64], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 5bf5d mulsd xmm0,QWORD PTR [r14]
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(r14) as u128))).to_bits());
            // 5bf62 cvtsd2ss xmm0,xmm0
            x0 = set_lo32(x0, (f64_of(x0) as f32).to_bits());
            // 5bf66 cvtss2sd xmm0,xmm0
            x0 = set_lo64(x0, (f32_of(x0) as f64).to_bits());
            // 5bf6a mulsd xmm0,QWORD PTR [rbp-0x148]
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u128))).to_bits());
            // 5bf72 addsd xmm0,QWORD PTR [rip+0xa295e] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 5bf7a cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 5bf7f cmp BYTE PTR [rbp+0x10],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0x10)) as u64, 0x0u64).1;
            // 5bf83 je 0x5bf87
            if fl.e() { pc = 0x5bf87; continue; }
            pc = 0x5bf85;
        }
        0x5bf85 => {
            // block 5bf85
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bf85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bf85 bswap eax
            rax = (rax as u32).swap_bytes() as u64;
            pc = 0x5bf87;
        }
        0x5bf87 => {
            // block 5bf87
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bf87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bf87 mov DWORD PTR [r13+0x0],eax
            m.w32(r13, ((rax as u32 as u64)) as u32);
            // 5bf8b movsd xmm0,QWORD PTR [rbp-0x138]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u128;
            // 5bf93 addsd xmm0,QWORD PTR [rip+0xa3bed] # 0xffb88
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(82773)) as u128))).to_bits());
            // 5bf9b movsd xmm1,QWORD PTR [rbp-0x140]
            x1 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u128;
            // 5bfa3 ucomisd xmm0,xmm1
            fl = ucomi(f64_of(x0), f64_of(x1));
            // 5bfa7 jbe 0x5bfad
            if fl.be() { pc = 0x5bfad; continue; }
            pc = 0x5bfa9;
        }
        0x5bfa9 => {
            // block 5bfa9
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bfa9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfa9 subsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) - f64_of(x1)).to_bits());
            pc = 0x5bfad;
        }
        0x5bfad => {
            // block 5bfad
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bfad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfad add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // 5bfb1 movsd QWORD PTR [rbx],xmm0
            m.w64(rbx, x0 as u64);
            // 5bfb5 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 5bfb9 add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // 5bfbd add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 5bfc1 dec r15d
            { let (r, f) = dec::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 5bfc4 jne 0x5bf32
            if fl.ne() { pc = 0x5bf32; continue; }
            pc = 0x5bfca;
        }
        0x5bfca => {
            // block 5bfca
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bfca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfca mov r13,QWORD PTR [rbp-0x150]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 5bfd1 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 5bfd8 lea r13,[r13+rax*4+0x0]
            r13 = r13.wrapping_add(rax.wrapping_mul(4));
            // 5bfdd jmp 0x5bfe6
            pc = 0x5bfe6; continue;
        }
        0x5bfdf => {
            // block 5bfdf
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bfdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfdf mov r13,QWORD PTR [rbp-0x150]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            pc = 0x5bfe6;
        }
        0x5bfe6 => {
            // block 5bfe6
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5bfe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5bfe6 mov ecx,DWORD PTR [rbp-0x154]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_feac)) as u64) as u32 as u64;
            // 5bfec mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 5bff3 inc DWORD PTR [rax]
            { let (r, f) = inc::<32>(m.r32(rax) as u64, fl); m.w32(rax, (r) as u32); fl = f; }
            // 5bff5 inc ecx
            { let (r, f) = inc::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5bff7 cmp ecx,DWORD PTR [rbp-0x158]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64).1;
            // 5bffd jne 0x5bf02
            if fl.ne() { pc = 0x5bf02; continue; }
            pc = 0x5c003;
        }
        0x5c003 => {
            // block 5c003
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5c003 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c003 mov rax,QWORD PTR [rip+0xe43ce] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // 5c00a mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 5c00d cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 5c011 jne 0x5c034
            if fl.ne() { pc = 0x5c034; continue; }
            pc = 0x5c013;
        }
        0x5c013 => {
            // block 5c013
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5c013 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c013 mov eax,DWORD PTR [rbp-0x158]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64) as u32 as u64;
            // 5c019 mov ecx,DWORD PTR [rbp+0x18]
            rcx = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 5c01c imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 5c01f shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 5c022 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 5c029 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5c02a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5c02c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5c02e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5c030 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5c032 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c033 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5c034 => {
            // block 5c034
            #[cfg(feature = "trace")] eprintln!("f_0005be20 pc 5c034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c034 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x697d6 size=558
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000697d6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x697d6;
    loop {
        match pc {
        0x697d6 => {
            // block 697d6
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 697d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 697d6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 697d7 mov rbp,rsp
            rbp = rsp;
            // 697da push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 697dc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 697de push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 697e0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 697e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 697e3 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 697e7 mov ebx,ecx
            rbx = ((rcx as u32 as u64)) as u32 as u64;
            // 697e9 mov r13,rsi
            r13 = rsi;
            // 697ec mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 697f0 lea rax,[rdi+0x8]
            rax = rdi.wrapping_add(0x8);
            // 697f4 mov QWORD PTR [rdi+0x10],0x0
            m.w64(rdi.wrapping_add(0x10), (0x0u64) as u64);
            // 697fc mov QWORD PTR [rdi+0x8],0x0
            m.w64(rdi.wrapping_add(0x8), (0x0u64) as u64);
            // 69804 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 69807 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 6980b mov rsi,rdx
            rsi = rdx;
            // 6980e call 0xb5094
            { let r = { let t = m.env.code_va(0xb5094); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69813 movzx eax,BYTE PTR [rbp-0x40]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 69817 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 69819 jne 0x69820
            if fl.ne() { pc = 0x69820; continue; }
            pc = 0x6981b;
        }
        0x6981b => {
            // block 6981b
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6981b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6981b shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 6981e jmp 0x69824
            pc = 0x69824; continue;
        }
        0x69820 => {
            // block 69820
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69820 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69820 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x69824;
        }
        0x69824 => {
            // block 69824
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69824 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69824 cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 69827 jne 0x69863
            if fl.ne() { pc = 0x69863; continue; }
            pc = 0x69829;
        }
        0x69829 => {
            // block 69829
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69829 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69829 lea rdi,[r13+0x1b0]
            rdi = r13.wrapping_add(0x1b0);
            // 69830 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 69834 call 0xebbd6
            { let r = { let t = m.env.stub_va(0x3a3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69839 add r13,0x1b8
            { let (r, f) = add::<64>(r13, 0x1b8u64);
              r13 = r;
              fl = f; }
            // 69840 cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // 69843 je 0x699c1
            if fl.e() { pc = 0x699c1; continue; }
            pc = 0x69849;
        }
        0x69849 => {
            // block 69849
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69849 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69849 add rax,0x38
            { let (r, f) = add::<64>(rax, 0x38u64);
              rax = r;
              fl = f; }
            // 6984d mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 69851 mov rsi,rax
            rsi = rax;
            // 69854 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69859 jmp 0x699c1
            pc = 0x699c1; continue;
        }
        0x6985e => {
            // block 6985e
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6985e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6985e jmp 0x699dd
            pc = 0x699dd; continue;
        }
        0x69863 => {
            // block 69863
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69863 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69863 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 69867 mov DWORD PTR [rbp-0x4c],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((rbx as u32 as u64)) as u32);
            // 6986a mov rbx,QWORD PTR [r13+0x1b0]
            rbx = m.r64(r13.wrapping_add(0x1b0)) as u64;
            // 69871 add r13,0x1b8
            { let (r, f) = add::<64>(r13, 0x1b8u64);
              r13 = r;
              fl = f; }
            // 69878 jmp 0x699b8
            pc = 0x699b8; continue;
        }
        0x6987d => {
            // block 6987d
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6987d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6987d movzx r8d,BYTE PTR [rbx+0x20]
            r8 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 69882 mov r14b,r8b
            r14 = (r14 & !0xff) | (((r8 as u8 as u64)) as u8 as u64);
            // 69885 and r14b,0x1
            { let r = (r14 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r14 = (r14 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 69889 jne 0x6989a
            if fl.ne() { pc = 0x6989a; continue; }
            pc = 0x6988b;
        }
        0x6988b => {
            // block 6988b
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6988b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6988b lea r9,[rbx+0x20]
            r9 = rbx.wrapping_add(0x20);
            // 6988f inc r9
            { let (r, f) = inc::<64>(r9, fl); r9 = r; fl = f; }
            // 69892 mov rsi,r8
            rsi = r8;
            // 69895 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 69898 jmp 0x698a2
            pc = 0x698a2; continue;
        }
        0x6989a => {
            // block 6989a
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6989a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6989a mov rsi,QWORD PTR [rbx+0x28]
            rsi = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 6989e mov r9,QWORD PTR [rbx+0x30]
            r9 = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x698a2;
        }
        0x698a2 => {
            // block 698a2
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698a2 movzx edi,BYTE PTR [rbp-0x40]
            rdi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 698a6 test dil,0x1
            fl = logic::<8>((rdi as u8 as u64) & 0x1u64);
            // 698aa jne 0x698b5
            if fl.ne() { pc = 0x698b5; continue; }
            pc = 0x698ac;
        }
        0x698ac => {
            // block 698ac
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698ac shr rdi,1
            { let (r, f) = shr::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // 698af lea rax,[rbp-0x3f]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 698b3 jmp 0x698bd
            pc = 0x698bd; continue;
        }
        0x698b5 => {
            // block 698b5
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698b5 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 698b9 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x698bd;
        }
        0x698bd => {
            // block 698bd
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698bd cmp rsi,rdi
            fl = sub::<64>(rsi, rdi).1;
            // 698c0 jb 0x69993
            if fl.b() { pc = 0x69993; continue; }
            pc = 0x698c6;
        }
        0x698c6 => {
            // block 698c6
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698c6 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 698c9 mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 698ce je 0x69938
            if fl.e() { pc = 0x69938; continue; }
            pc = 0x698d0;
        }
        0x698d0 => {
            // block 698d0
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698d0 cmp rsi,rdi
            fl = sub::<64>(rsi, rdi).1;
            // 698d3 jl 0x69993
            if fl.l() { pc = 0x69993; continue; }
            pc = 0x698d9;
        }
        0x698d9 => {
            // block 698d9
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698d9 mov r10d,0x1
            r10 = (0x1u64) as u32 as u64;
            // 698df sub r10,rdi
            { let (r, f) = sub::<64>(r10, rdi);
              r10 = r;
              fl = f; }
            // 698e2 add r10,rsi
            { let (r, f) = add::<64>(r10, rsi);
              r10 = r;
              fl = f; }
            // 698e5 je 0x69993
            if fl.e() { pc = 0x69993; continue; }
            pc = 0x698eb;
        }
        0x698eb => {
            // block 698eb
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698eb add rsi,r9
            { let (r, f) = add::<64>(rsi, r9);
              rsi = r;
              fl = f; }
            // 698ee add r10,r9
            { let (r, f) = add::<64>(r10, r9);
              r10 = r;
              fl = f; }
            // 698f1 mov r11b,BYTE PTR [rax]
            r11 = (r11 & !0xff) | ((m.r8(rax) as u64) as u8 as u64);
            // 698f4 mov rdx,r9
            rdx = r9;
            pc = 0x698f7;
        }
        0x698f7 => {
            // block 698f7
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 698f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698f7 movzx r12d,r11b
            r12 = ((r11 as u8 as u64)) as u32 as u64;
            // 698fb movzx ecx,BYTE PTR [rdx]
            rcx = (m.r8(rdx) as u64) as u32 as u64;
            // 698fe cmp ecx,r12d
            fl = sub::<32>((rcx as u32 as u64), (r12 as u32 as u64)).1;
            // 69901 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 69907 jne 0x69920
            if fl.ne() { pc = 0x69920; continue; }
            pc = 0x69909;
        }
        0x69909 => {
            // block 69909
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69909 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69909 cmp rdi,r12
            fl = sub::<64>(rdi, r12).1;
            // 6990c je 0x6992a
            if fl.e() { pc = 0x6992a; continue; }
            pc = 0x6990e;
        }
        0x6990e => {
            // block 6990e
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6990e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6990e movzx ecx,BYTE PTR [rax+r12*1]
            rcx = (m.r8(rax.wrapping_add(r12)) as u64) as u32 as u64;
            // 69913 movzx r15d,BYTE PTR [rdx+r12*1]
            r15 = (m.r8(rdx.wrapping_add(r12)) as u64) as u32 as u64;
            // 69918 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 6991b cmp r15d,ecx
            fl = sub::<32>((r15 as u32 as u64), (rcx as u32 as u64)).1;
            // 6991e je 0x69909
            if fl.e() { pc = 0x69909; continue; }
            pc = 0x69920;
        }
        0x69920 => {
            // block 69920
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69920 inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // 69923 cmp rdx,r10
            fl = sub::<64>(rdx, r10).1;
            // 69926 jne 0x698f7
            if fl.ne() { pc = 0x698f7; continue; }
            pc = 0x69928;
        }
        0x69928 => {
            // block 69928
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69928 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69928 jmp 0x69993
            pc = 0x69993; continue;
        }
        0x6992a => {
            // block 6992a
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6992a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6992a cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 6992d je 0x69993
            if fl.e() { pc = 0x69993; continue; }
            pc = 0x6992f;
        }
        0x6992f => {
            // block 6992f
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6992f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6992f sub rdx,r9
            { let (r, f) = sub::<64>(rdx, r9);
              rdx = r;
              fl = f; }
            // 69932 cmp rdx,0xffffffffffffffff
            fl = sub::<64>(rdx, 0xffff_ffff_ffff_ffffu64).1;
            // 69936 je 0x69993
            if fl.e() { pc = 0x69993; continue; }
            pc = 0x69938;
        }
        0x69938 => {
            // block 69938
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69938 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69938 mov eax,DWORD PTR [rbp-0x4c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u32 as u64;
            // 6993b cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 6993e je 0x69959
            if fl.e() { pc = 0x69959; continue; }
            pc = 0x69940;
        }
        0x69940 => {
            // block 69940
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69940 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69940 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 69943 jne 0x6996a
            if fl.ne() { pc = 0x6996a; continue; }
            pc = 0x69945;
        }
        0x69945 => {
            // block 69945
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69945 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69945 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 69948 jne 0x69993
            if fl.ne() { pc = 0x69993; continue; }
            pc = 0x6994a;
        }
        0x6994a => {
            // block 6994a
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6994a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6994a lea rsi,[rbx+0x38]
            rsi = rbx.wrapping_add(0x38);
            // 6994e mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 69952 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69957 jmp 0x69993
            pc = 0x69993; continue;
        }
        0x69959 => {
            // block 69959
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69959 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69959 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 6995d add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 69960 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 69963 jne 0x6997d
            if fl.ne() { pc = 0x6997d; continue; }
            pc = 0x69965;
        }
        0x69965 => {
            // block 69965
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69965 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69965 shr r8,1
            { let (r, f) = shr::<64>(r8, 0x1u64, fl); r8 = r; fl = f; }
            // 69968 jmp 0x69981
            pc = 0x69981; continue;
        }
        0x6996a => {
            // block 6996a
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6996a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6996a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 6996c jne 0x69993
            if fl.ne() { pc = 0x69993; continue; }
            pc = 0x6996e;
        }
        0x6996e => {
            // block 6996e
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6996e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6996e lea rsi,[rbx+0x38]
            rsi = rbx.wrapping_add(0x38);
            // 69972 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 69976 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6997b jmp 0x69993
            pc = 0x69993; continue;
        }
        0x6997d => {
            // block 6997d
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6997d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6997d mov r8,QWORD PTR [rbx+0x28]
            r8 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x69981;
        }
        0x69981 => {
            // block 69981
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69981 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69981 cmp rdx,r8
            fl = sub::<64>(rdx, r8).1;
            // 69984 jne 0x69993
            if fl.ne() { pc = 0x69993; continue; }
            pc = 0x69986;
        }
        0x69986 => {
            // block 69986
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69986 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69986 lea rsi,[rbx+0x38]
            rsi = rbx.wrapping_add(0x38);
            // 6998a mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 6998e call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x69993;
        }
        0x69993 => {
            // block 69993
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 69993 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69993 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 69997 test rax,rax
            fl = logic::<64>(rax & rax);
            // 6999a je 0x699a9
            if fl.e() { pc = 0x699a9; continue; }
            pc = 0x6999c;
        }
        0x6999c => {
            // block 6999c
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 6999c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6999c mov rbx,rax
            rbx = rax;
            // 6999f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 699a2 test rax,rax
            fl = logic::<64>(rax & rax);
            // 699a5 jne 0x6999c
            if fl.ne() { pc = 0x6999c; continue; }
            pc = 0x699a7;
        }
        0x699a7 => {
            // block 699a7
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699a7 jmp 0x699b8
            pc = 0x699b8; continue;
        }
        0x699a9 => {
            // block 699a9
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699a9 mov rax,rbx
            rax = rbx;
            pc = 0x699ac;
        }
        0x699ac => {
            // block 699ac
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699ac mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 699b0 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 699b3 mov rax,rbx
            rax = rbx;
            // 699b6 jne 0x699ac
            if fl.ne() { pc = 0x699ac; continue; }
            pc = 0x699b8;
        }
        0x699b8 => {
            // block 699b8
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699b8 cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // 699bb jne 0x6987d
            if fl.ne() { pc = 0x6987d; continue; }
            pc = 0x699c1;
        }
        0x699c1 => {
            // block 699c1
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699c1 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 699c5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 699ca mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 699ce add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 699d2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 699d3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 699d5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 699d7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 699d9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 699db pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 699dc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x699dd => {
            // block 699dd
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699dd mov rbx,rax
            rbx = rax;
            // 699e0 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 699e4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 699e9 jmp 0x699ee
            pc = 0x699ee; continue;
        }
        0x699eb => {
            // block 699eb
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699eb mov rbx,rax
            rbx = rax;
            pc = 0x699ee;
        }
        0x699ee => {
            // block 699ee
            #[cfg(feature = "trace")] eprintln!("f_000697d6 pc 699ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 699ee mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 699f2 mov rsi,QWORD PTR [rdi+0x8]
            rsi = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 699f6 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 699fb mov rdi,rbx
            rdi = rbx;
            // 699fe call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4555 size=591
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4555<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4555;
    loop {
        match pc {
        0xc4555 => {
            // block c4555
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4555 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4555 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4556 mov rbp,rsp
            rbp = rsp;
            // c4559 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c455b push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c455c mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // c455f test rax,rax
            fl = logic::<64>(rax & rax);
            // c4562 mov rcx,rsi
            rcx = rsi;
            // c4565 je 0xc457b
            if fl.e() { pc = 0xc457b; continue; }
            pc = 0xc4567;
        }
        0xc4567 => {
            // block c4567
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4567 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4567 mov rdx,QWORD PTR [rsi+0x8]
            rdx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c456b test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c456e je 0xc458f
            if fl.e() { pc = 0xc458f; continue; }
            pc = 0xc4570;
        }
        0xc4570 => {
            // block c4570
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4570 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4570 mov rcx,rdx
            rcx = rdx;
            // c4573 mov rdx,QWORD PTR [rcx]
            rdx = m.r64(rcx) as u64;
            // c4576 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4579 jne 0xc4570
            if fl.ne() { pc = 0xc4570; continue; }
            pc = 0xc457b;
        }
        0xc457b => {
            // block c457b
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c457b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c457b mov rax,QWORD PTR [rcx+0x8]
            rax = m.r64(rcx.wrapping_add(0x8)) as u64;
            // c457f test rax,rax
            fl = logic::<64>(rax & rax);
            // c4582 jne 0xc4592
            if fl.ne() { pc = 0xc4592; continue; }
            pc = 0xc4584;
        }
        0xc4584 => {
            // block c4584
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4584 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4584 lea r9,[rcx+0x10]
            r9 = rcx.wrapping_add(0x10);
            // c4588 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c458b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c458d jmp 0xc45a1
            pc = 0xc45a1; continue;
        }
        0xc458f => {
            // block c458f
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c458f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c458f mov rcx,rsi
            rcx = rsi;
            pc = 0xc4592;
        }
        0xc4592 => {
            // block c4592
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4592 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4592 mov rdx,QWORD PTR [rcx+0x10]
            rdx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // c4596 mov QWORD PTR [rax+0x10],rdx
            m.w64(rax.wrapping_add(0x10), (rdx) as u64);
            // c459a mov r8b,0x1
            r8 = (r8 & !0xff) | ((0x1u64) as u8 as u64);
            // c459d lea r9,[rcx+0x10]
            r9 = rcx.wrapping_add(0x10);
            pc = 0xc45a1;
        }
        0xc45a1 => {
            // block c45a1
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45a1 mov rdx,QWORD PTR [r9]
            rdx = m.r64(r9) as u64;
            // c45a4 mov rbx,QWORD PTR [rdx]
            rbx = m.r64(rdx) as u64;
            // c45a7 cmp rbx,rcx
            fl = sub::<64>(rbx, rcx).1;
            // c45aa je 0xc45b2
            if fl.e() { pc = 0xc45b2; continue; }
            pc = 0xc45ac;
        }
        0xc45ac => {
            // block c45ac
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45ac mov QWORD PTR [rdx+0x8],rax
            m.w64(rdx.wrapping_add(0x8), (rax) as u64);
            // c45b0 jmp 0xc45c3
            pc = 0xc45c3; continue;
        }
        0xc45b2 => {
            // block c45b2
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45b2 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // c45b5 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c45b7 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // c45ba mov r14,rax
            r14 = rax;
            // c45bd je 0xc45c6
            if fl.e() { pc = 0xc45c6; continue; }
            pc = 0xc45bf;
        }
        0xc45bf => {
            // block c45bf
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45bf mov rbx,QWORD PTR [rdx+0x8]
            rbx = m.r64(rdx.wrapping_add(0x8)) as u64;
            pc = 0xc45c3;
        }
        0xc45c3 => {
            // block c45c3
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45c3 mov r14,rdi
            r14 = rdi;
            pc = 0xc45c6;
        }
        0xc45c6 => {
            // block c45c6
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45c6 mov r10b,BYTE PTR [rcx+0x18]
            r10 = (r10 & !0xff) | ((m.r8(rcx.wrapping_add(0x18)) as u64) as u8 as u64);
            // c45ca cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // c45cd je 0xc4610
            if fl.e() { pc = 0xc4610; continue; }
            pc = 0xc45cf;
        }
        0xc45cf => {
            // block c45cf
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45cf mov rdx,QWORD PTR [rsi+0x10]
            rdx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // c45d3 mov QWORD PTR [r9],rdx
            m.w64(r9, (rdx) as u64);
            // c45d6 mov rdi,QWORD PTR [rsi+0x10]
            rdi = m.r64(rsi.wrapping_add(0x10)) as u64;
            // c45da cmp QWORD PTR [rdi],rsi
            fl = sub::<64>(m.r64(rdi) as u64, rsi).1;
            // c45dd je 0xc45e5
            if fl.e() { pc = 0xc45e5; continue; }
            pc = 0xc45df;
        }
        0xc45df => {
            // block c45df
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45df mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // c45e3 jmp 0xc45e8
            pc = 0xc45e8; continue;
        }
        0xc45e5 => {
            // block c45e5
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45e5 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            pc = 0xc45e8;
        }
        0xc45e8 => {
            // block c45e8
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45e8 mov rdx,QWORD PTR [rsi]
            rdx = m.r64(rsi) as u64;
            // c45eb mov QWORD PTR [rcx],rdx
            m.w64(rcx, (rdx) as u64);
            // c45ee mov QWORD PTR [rdx+0x10],rcx
            m.w64(rdx.wrapping_add(0x10), (rcx) as u64);
            // c45f2 mov rdx,QWORD PTR [rsi+0x8]
            rdx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c45f6 mov QWORD PTR [rcx+0x8],rdx
            m.w64(rcx.wrapping_add(0x8), (rdx) as u64);
            // c45fa test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c45fd je 0xc4603
            if fl.e() { pc = 0xc4603; continue; }
            pc = 0xc45ff;
        }
        0xc45ff => {
            // block c45ff
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c45ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c45ff mov QWORD PTR [rdx+0x10],rcx
            m.w64(rdx.wrapping_add(0x10), (rcx) as u64);
            pc = 0xc4603;
        }
        0xc4603 => {
            // block c4603
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4603 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4603 mov dl,BYTE PTR [rsi+0x18]
            rdx = (rdx & !0xff) | ((m.r8(rsi.wrapping_add(0x18)) as u64) as u8 as u64);
            // c4606 mov BYTE PTR [rcx+0x18],dl
            m.w8(rcx.wrapping_add(0x18), ((rdx as u8 as u64)) as u8);
            // c4609 cmp r14,rsi
            fl = sub::<64>(r14, rsi).1;
            // c460c cmove r14,rcx
            if fl.e() { r14 = rcx; }
            pc = 0xc4610;
        }
        0xc4610 => {
            // block c4610
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4610 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4610 test r10b,r10b
            fl = logic::<8>((r10 as u8 as u64) & (r10 as u8 as u64));
            // c4613 je 0xc470d
            if fl.e() { pc = 0xc470d; continue; }
            pc = 0xc4619;
        }
        0xc4619 => {
            // block c4619
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4619 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4619 test r14,r14
            fl = logic::<64>(r14 & r14);
            // c461c je 0xc470d
            if fl.e() { pc = 0xc470d; continue; }
            pc = 0xc4622;
        }
        0xc4622 => {
            // block c4622
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4622 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4622 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // c4625 je 0xc4642
            if fl.e() { pc = 0xc4642; continue; }
            pc = 0xc4627;
        }
        0xc4627 => {
            // block c4627
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4627 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4627 mov BYTE PTR [rax+0x18],0x1
            m.w8(rax.wrapping_add(0x18), (0x1u64) as u8);
            // c462b jmp 0xc470d
            pc = 0xc470d; continue;
        }
        0xc4630 => {
            // block c4630
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4630 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4630 mov rdx,QWORD PTR [rcx+0x10]
            rdx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // c4634 lea rax,[rdx+0x8]
            rax = rdx.wrapping_add(0x8);
            // c4638 cmp QWORD PTR [rdx],rcx
            fl = sub::<64>(m.r64(rdx) as u64, rcx).1;
            // c463b cmovne rax,rdx
            if fl.ne() { rax = rdx; }
            // c463f mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            pc = 0xc4642;
        }
        0xc4642 => {
            // block c4642
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4642 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c4646 cmp QWORD PTR [rdi],rbx
            fl = sub::<64>(m.r64(rdi) as u64, rbx).1;
            // c4649 mov al,BYTE PTR [rbx+0x18]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x18)) as u64) as u8 as u64);
            // c464c je 0xc46a7
            if fl.e() { pc = 0xc46a7; continue; }
            pc = 0xc464e;
        }
        0xc464e => {
            // block c464e
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c464e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c464e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c4650 jne 0xc466d
            if fl.ne() { pc = 0xc466d; continue; }
            pc = 0xc4652;
        }
        0xc4652 => {
            // block c4652
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4652 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4652 mov BYTE PTR [rbx+0x18],0x1
            m.w8(rbx.wrapping_add(0x18), (0x1u64) as u8);
            // c4656 mov BYTE PTR [rdi+0x18],0x0
            m.w8(rdi.wrapping_add(0x18), (0x0u64) as u8);
            // c465a call 0xebaaa
            { let r = { let t = m.env.stub_va(0x277); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c465f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c4662 cmp r14,rax
            fl = sub::<64>(r14, rax).1;
            // c4665 cmove r14,rbx
            if fl.e() { r14 = rbx; }
            // c4669 mov rbx,QWORD PTR [rax+0x8]
            rbx = m.r64(rax.wrapping_add(0x8)) as u64;
            pc = 0xc466d;
        }
        0xc466d => {
            // block c466d
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c466d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c466d mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // c4670 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c4673 je 0xc467f
            if fl.e() { pc = 0xc467f; continue; }
            pc = 0xc4675;
        }
        0xc4675 => {
            // block c4675
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4675 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4675 cmp BYTE PTR [rcx+0x18],0x0
            fl = sub::<8>(m.r8(rcx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c4679 je 0xc4712
            if fl.e() { pc = 0xc4712; continue; }
            pc = 0xc467f;
        }
        0xc467f => {
            // block c467f
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c467f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c467f mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4683 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4686 je 0xc4692
            if fl.e() { pc = 0xc4692; continue; }
            pc = 0xc4688;
        }
        0xc4688 => {
            // block c4688
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4688 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4688 cmp BYTE PTR [rax+0x18],0x0
            fl = sub::<8>(m.r8(rax.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c468c je 0xc471b
            if fl.e() { pc = 0xc471b; continue; }
            pc = 0xc4692;
        }
        0xc4692 => {
            // block c4692
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4692 mov BYTE PTR [rbx+0x18],0x0
            m.w8(rbx.wrapping_add(0x18), (0x0u64) as u8);
            // c4696 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c469a cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // c469d je 0xc4708
            if fl.e() { pc = 0xc4708; continue; }
            pc = 0xc469f;
        }
        0xc469f => {
            // block c469f
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c469f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c469f cmp BYTE PTR [rcx+0x18],0x0
            fl = sub::<8>(m.r8(rcx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c46a3 jne 0xc4630
            if fl.ne() { pc = 0xc4630; continue; }
            pc = 0xc46a5;
        }
        0xc46a5 => {
            // block c46a5
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46a5 jmp 0xc4705
            pc = 0xc4705; continue;
        }
        0xc46a7 => {
            // block c46a7
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46a7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c46a9 jne 0xc46c6
            if fl.ne() { pc = 0xc46c6; continue; }
            pc = 0xc46ab;
        }
        0xc46ab => {
            // block c46ab
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46ab mov BYTE PTR [rbx+0x18],0x1
            m.w8(rbx.wrapping_add(0x18), (0x1u64) as u8);
            // c46af mov BYTE PTR [rdi+0x18],0x0
            m.w8(rdi.wrapping_add(0x18), (0x0u64) as u8);
            // c46b3 call 0xeb86a
            { let r = { let t = m.env.stub_va(0x37); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c46b8 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c46bc cmp r14,rax
            fl = sub::<64>(r14, rax).1;
            // c46bf cmove r14,rbx
            if fl.e() { r14 = rbx; }
            // c46c3 mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            pc = 0xc46c6;
        }
        0xc46c6 => {
            // block c46c6
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46c6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c46c9 test rax,rax
            fl = logic::<64>(rax & rax);
            // c46cc je 0xc46d8
            if fl.e() { pc = 0xc46d8; continue; }
            pc = 0xc46ce;
        }
        0xc46ce => {
            // block c46ce
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46ce cmp BYTE PTR [rax+0x18],0x0
            fl = sub::<8>(m.r8(rax.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c46d2 je 0xc4759
            if fl.e() { pc = 0xc4759; continue; }
            pc = 0xc46d8;
        }
        0xc46d8 => {
            // block c46d8
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46d8 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c46dc test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c46df je 0xc46e7
            if fl.e() { pc = 0xc46e7; continue; }
            pc = 0xc46e1;
        }
        0xc46e1 => {
            // block c46e1
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46e1 cmp BYTE PTR [rcx+0x18],0x0
            fl = sub::<8>(m.r8(rcx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c46e5 je 0xc4754
            if fl.e() { pc = 0xc4754; continue; }
            pc = 0xc46e7;
        }
        0xc46e7 => {
            // block c46e7
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46e7 mov BYTE PTR [rbx+0x18],0x0
            m.w8(rbx.wrapping_add(0x18), (0x0u64) as u8);
            // c46eb mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c46ef cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // c46f2 je 0xc46ff
            if fl.e() { pc = 0xc46ff; continue; }
            pc = 0xc46f4;
        }
        0xc46f4 => {
            // block c46f4
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46f4 mov al,BYTE PTR [rcx+0x18]
            rax = (rax & !0xff) | ((m.r8(rcx.wrapping_add(0x18)) as u64) as u8 as u64);
            // c46f7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c46f9 jne 0xc4630
            if fl.ne() { pc = 0xc4630; continue; }
            pc = 0xc46ff;
        }
        0xc46ff => {
            // block c46ff
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c46ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c46ff mov BYTE PTR [rcx+0x18],0x1
            m.w8(rcx.wrapping_add(0x18), (0x1u64) as u8);
            // c4703 jmp 0xc470d
            pc = 0xc470d; continue;
        }
        0xc4705 => {
            // block c4705
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4705 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4705 mov r14,rcx
            r14 = rcx;
            pc = 0xc4708;
        }
        0xc4708 => {
            // block c4708
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4708 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4708 mov BYTE PTR [r14+0x18],0x1
            m.w8(r14.wrapping_add(0x18), (0x1u64) as u8);
            pc = 0xc470d;
        }
        0xc470d => {
            // block c470d
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c470d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c470d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c470e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4710 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4711 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc4712 => {
            // block c4712
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4712 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4712 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4716 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4719 je 0xc4721
            if fl.e() { pc = 0xc4721; continue; }
            pc = 0xc471b;
        }
        0xc471b => {
            // block c471b
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c471b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c471b cmp BYTE PTR [rax+0x18],0x0
            fl = sub::<8>(m.r8(rax.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c471f je 0xc4739
            if fl.e() { pc = 0xc4739; continue; }
            pc = 0xc4721;
        }
        0xc4721 => {
            // block c4721
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4721 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4721 mov BYTE PTR [rcx+0x18],0x1
            m.w8(rcx.wrapping_add(0x18), (0x1u64) as u8);
            // c4725 mov BYTE PTR [rbx+0x18],0x0
            m.w8(rbx.wrapping_add(0x18), (0x0u64) as u8);
            // c4729 mov rdi,rbx
            rdi = rbx;
            // c472c call 0xeb86a
            { let r = { let t = m.env.stub_va(0x37); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4731 mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c4735 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0xc4739;
        }
        0xc4739 => {
            // block c4739
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4739 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4739 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c473d mov cl,BYTE PTR [rdi+0x18]
            rcx = (rcx & !0xff) | ((m.r8(rdi.wrapping_add(0x18)) as u64) as u8 as u64);
            // c4740 mov BYTE PTR [rbx+0x18],cl
            m.w8(rbx.wrapping_add(0x18), ((rcx as u8 as u64)) as u8);
            // c4743 mov BYTE PTR [rdi+0x18],0x1
            m.w8(rdi.wrapping_add(0x18), (0x1u64) as u8);
            // c4747 mov BYTE PTR [rax+0x18],0x1
            m.w8(rax.wrapping_add(0x18), (0x1u64) as u8);
            // c474b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c474c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c474e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c474f jmp 0xebaaa
            { let r = { let t = m.env.stub_va(0x277); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xc4754 => {
            // block c4754
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4754 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4754 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4757 je 0xc475f
            if fl.e() { pc = 0xc475f; continue; }
            pc = 0xc4759;
        }
        0xc4759 => {
            // block c4759
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c4759 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4759 cmp BYTE PTR [rax+0x18],0x0
            fl = sub::<8>(m.r8(rax.wrapping_add(0x18)) as u64, 0x0u64).1;
            // c475d je 0xc477a
            if fl.e() { pc = 0xc477a; continue; }
            pc = 0xc475f;
        }
        0xc475f => {
            // block c475f
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c475f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c475f mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4763 mov BYTE PTR [rax+0x18],0x1
            m.w8(rax.wrapping_add(0x18), (0x1u64) as u8);
            // c4767 mov BYTE PTR [rbx+0x18],0x0
            m.w8(rbx.wrapping_add(0x18), (0x0u64) as u8);
            // c476b mov rdi,rbx
            rdi = rbx;
            // c476e call 0xebaaa
            { let r = { let t = m.env.stub_va(0x277); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4773 mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c4777 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            pc = 0xc477a;
        }
        0xc477a => {
            // block c477a
            #[cfg(feature = "trace")] eprintln!("f_000c4555 pc c477a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c477a mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c477e mov cl,BYTE PTR [rdi+0x18]
            rcx = (rcx & !0xff) | ((m.r8(rdi.wrapping_add(0x18)) as u64) as u8 as u64);
            // c4781 mov BYTE PTR [rbx+0x18],cl
            m.w8(rbx.wrapping_add(0x18), ((rcx as u8 as u64)) as u8);
            // c4784 mov BYTE PTR [rdi+0x18],0x1
            m.w8(rdi.wrapping_add(0x18), (0x1u64) as u8);
            // c4788 mov BYTE PTR [rax+0x18],0x1
            m.w8(rax.wrapping_add(0x18), (0x1u64) as u8);
            // c478c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c478d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c478f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4790 jmp 0xeb86a
            { let r = { let t = m.env.stub_va(0x37); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8380 size=536
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8380<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc8380;
    loop {
        match pc {
        0xc8380 => {
            // block c8380
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c8380 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8380 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8381 mov rbp,rsp
            rbp = rsp;
            // c8384 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c8386 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c8388 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c838a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c838c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c838d mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // c838f mov DWORD PTR [rdi-0x4],eax
            m.w32(rdi.wrapping_add(0xffff_ffff_ffff_fffc), ((rax as u32 as u64)) as u32);
            // c8392 movsxd rbx,r8d
            rbx = sx::<32>((r8 as u32 as u64)) as u64;
            // c8395 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // c8398 movsxd r10,eax
            r10 = sx::<32>((rax as u32 as u64)) as u64;
            // c839b lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // c839e cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // c83a0 mov eax,DWORD PTR [rdi+rax*4]
            rax = (m.r32(rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // c83a3 mov DWORD PTR [rdi+r10*4],eax
            m.w32(rdi.wrapping_add(r10.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // c83a7 mov DWORD PTR [rdi+rbx*4],eax
            m.w32(rdi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // c83aa imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // c83ae mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // c83b0 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c83b1 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // c83b3 mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // c83b6 imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c83ba mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c83bc cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c83bd idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // c83bf mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c83c1 cmp r11d,ecx
            fl = sub::<32>((r11 as u32 as u64), (rcx as u32 as u64)).1;
            // c83c4 jge 0xc858d
            if fl.ge() { pc = 0xc858d; continue; }
            pc = 0xc83ca;
        }
        0xc83ca => {
            // block c83ca
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c83ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c83ca shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // c83ce mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // c83d1 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // c83d2 idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // c83d5 mov DWORD PTR [rbp-0x3c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((rax as u32 as u64)) as u32);
            // c83d8 sub ecx,r11d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r11 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c83db movsxd rdx,r11d
            rdx = sx::<32>((r11 as u32 as u64)) as u64;
            // c83de lea rdx,[rsi+rdx*4]
            rdx = rsi.wrapping_add(rdx.wrapping_mul(4));
            // c83e2 imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            pc = 0xc83e6;
        }
        0xc83e6 => {
            // block c83e6
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c83e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c83e6 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // c83ea mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // c83ed mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // c83f0 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c83f3 mov ecx,r11d
            rcx = ((r11 as u32 as u64)) as u32 as u64;
            // c83f6 shr ecx,0xb
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0xbu64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c83f9 and ecx,0x1f
            { let r = (rcx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c83fc movsxd r9,eax
            r9 = sx::<32>((rax as u32 as u64)) as u64;
            // c83ff movzx eax,BYTE PTR [rdi+r9*4-0x2]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u32 as u64;
            // c8405 mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // c840a sub edx,ecx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c840c lea rbx,[rip+0x80fad] # 0x1493c0
            rbx = m.env.data_va(383885);
            // c8413 mov rsi,rbx
            rsi = rbx;
            // c8416 mov r10d,DWORD PTR [rsi+rdx*4]
            r10 = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // c841a imul eax,r10d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r10 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c841e movzx ebx,BYTE PTR [rdi+r9*4+0x2]
            rbx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x2)) as u64) as u32 as u64;
            // c8424 mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // c8429 sub edx,ecx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c842b mov r13d,DWORD PTR [rsi+rdx*4]
            r13 = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // c842f imul ebx,r13d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r13 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // c8433 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c8435 movzx edx,BYTE PTR [rdi+r9*4+0x6]
            rdx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x6)) as u64) as u32 as u64;
            // c843b mov eax,0x60
            rax = (0x60u64) as u32 as u64;
            // c8440 sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c8442 mov r8d,DWORD PTR [rsi+rax*4]
            r8 = (m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // c8446 imul edx,r8d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r8 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // c844a add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c844c movzx eax,BYTE PTR [rdi+r9*4+0xa]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xa)) as u64) as u32 as u64;
            // c8452 mov ebx,0x80
            rbx = (0x80u64) as u32 as u64;
            // c8457 sub ebx,ecx
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c8459 mov r14d,DWORD PTR [rsi+rbx*4]
            r14 = (m.r32(rsi.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // c845d imul eax,r14d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r14 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c8461 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c8463 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8465 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c8467 sar ecx,0x10
            { let (r, f) = sar::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c846a mov esi,0x0
            rsi = (0x0u64) as u32 as u64;
            // c846f js 0xc8486
            if fl.s() { pc = 0xc8486; continue; }
            pc = 0xc8471;
        }
        0xc8471 => {
            // block c8471
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c8471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8471 and eax,0xff0000
            { let r = (rax as u32 as u64) & 0xff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c8476 cmp ecx,0xff
            fl = sub::<32>((rcx as u32 as u64), 0xffu64).1;
            // c847c mov ecx,0xff0000
            rcx = (0xff0000u64) as u32 as u64;
            // c8481 cmovg eax,ecx
            if fl.g() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // c8484 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xc8486;
        }
        0xc8486 => {
            // block c8486
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c8486 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8486 movzx eax,BYTE PTR [rdi+r9*4-0x3]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffd)) as u64) as u32 as u64;
            // c848c imul eax,r10d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r10 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c8490 movzx ecx,BYTE PTR [rdi+r9*4+0x1]
            rcx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x1)) as u64) as u32 as u64;
            // c8496 imul ecx,r13d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r13 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c849a add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c849c movzx ebx,BYTE PTR [rdi+r9*4+0x5]
            rbx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x5)) as u64) as u32 as u64;
            // c84a2 imul ebx,r8d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r8 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // c84a6 add ebx,ecx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c84a8 movzx eax,BYTE PTR [rdi+r9*4+0x9]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x9)) as u64) as u32 as u64;
            // c84ae imul eax,r14d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r14 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c84b2 add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c84b4 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c84b7 js 0xc84ce
            if fl.s() { pc = 0xc84ce; continue; }
            pc = 0xc84b9;
        }
        0xc84b9 => {
            // block c84b9
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c84b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c84b9 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c84bb shl ecx,0x8
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c84be cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // c84c3 movzx edx,cx
            rdx = ((rcx as u16 as u64)) as u32 as u64;
            // c84c6 mov eax,0xff00
            rax = (0xff00u64) as u32 as u64;
            // c84cb cmovg edx,eax
            if fl.g() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            pc = 0xc84ce;
        }
        0xc84ce => {
            // block c84ce
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c84ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c84ce movzx eax,BYTE PTR [rdi+r9*4-0x4]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // c84d4 imul eax,r10d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r10 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c84d8 movzx ecx,BYTE PTR [rdi+r9*4]
            rcx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4))) as u64) as u32 as u64;
            // c84dd imul ecx,r13d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r13 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c84e1 add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c84e3 movzx ebx,BYTE PTR [rdi+r9*4+0x4]
            rbx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x4)) as u64) as u32 as u64;
            // c84e9 imul ebx,r8d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r8 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // c84ed add ebx,ecx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c84ef movzx eax,BYTE PTR [rdi+r9*4+0x8]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x8)) as u64) as u32 as u64;
            // c84f5 imul eax,r14d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r14 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c84f9 add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c84fb xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c84fe sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c8501 mov r12d,0x0
            r12 = (0x0u64) as u32 as u64;
            // c8507 js 0xc851b
            if fl.s() { pc = 0xc851b; continue; }
            pc = 0xc8509;
        }
        0xc8509 => {
            // block c8509
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c8509 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8509 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // c850e movzx r12d,al
            r12 = ((rax as u8 as u64)) as u32 as u64;
            // c8512 mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // c8517 cmovg r12d,eax
            if fl.g() { r12 = ((rax as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            pc = 0xc851b;
        }
        0xc851b => {
            // block c851b
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c851b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c851b movzx eax,BYTE PTR [rdi+r9*4-0x1]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ffff)) as u64) as u32 as u64;
            // c8521 imul eax,r10d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r10 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c8525 movzx ecx,BYTE PTR [rdi+r9*4+0x3]
            rcx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x3)) as u64) as u32 as u64;
            // c852b imul ecx,r13d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r13 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c852f add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c8531 movzx ebx,BYTE PTR [rdi+r9*4+0x7]
            rbx = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x7)) as u64) as u32 as u64;
            // c8537 imul ebx,r8d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r8 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // c853b add ebx,ecx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c853d movzx eax,BYTE PTR [rdi+r9*4+0xb]
            rax = (m.r8(rdi.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0xb)) as u64) as u32 as u64;
            // c8543 imul eax,r14d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r14 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c8547 add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c8549 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c854c js 0xc8563
            if fl.s() { pc = 0xc8563; continue; }
            pc = 0xc854e;
        }
        0xc854e => {
            // block c854e
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c854e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c854e mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // c8551 shl r15d,0x18
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x18u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // c8555 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // c855a mov eax,0xff000000
            rax = (0xff000000u64) as u32 as u64;
            // c855f cmovg r15d,eax
            if fl.g() { r15 = ((rax as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            pc = 0xc8563;
        }
        0xc8563 => {
            // block c8563
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c8563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8563 or r15d,esi
            { let r = (r15 as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c8566 and r15d,0xffff0000
            { let r = (r15 as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c856d or r12d,edx
            { let r = (r12 as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // c8570 or r12d,r15d
            { let r = (r12 as u32 as u64) | (r15 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // c8573 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c8577 mov DWORD PTR [rdx],r12d
            m.w32(rdx, ((r12 as u32 as u64)) as u32);
            // c857a add rdx,0x4
            { let (r, f) = add::<64>(rdx, 0x4u64);
              rdx = r;
              fl = f; }
            // c857e add r11d,DWORD PTR [rbp-0x3c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // c8582 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // c8585 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // c8587 jne 0xc83e6
            if fl.ne() { pc = 0xc83e6; continue; }
            pc = 0xc858d;
        }
        0xc858d => {
            // block c858d
            #[cfg(feature = "trace")] eprintln!("f_000c8380 pc c858d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c858d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c858e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c8590 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c8592 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8594 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c8596 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8597 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4690 size=630
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4690<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4690;
    loop {
        match pc {
        0xd4690 => {
            // block d4690
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4690 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4691 mov rbp,rsp
            rbp = rsp;
            // d4694 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4696 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4698 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d469a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d469b sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // d469f mov r14,rdx
            r14 = rdx;
            // d46a2 mov rbx,rdi
            rbx = rdi;
            // d46a5 cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // d46a9 je 0xd4715
            if fl.e() { pc = 0xd4715; continue; }
            pc = 0xd46ab;
        }
        0xd46ab => {
            // block d46ab
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d46ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46ab mov DWORD PTR [rbp-0x24],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x12u64) as u32);
            // d46b2 mov DWORD PTR [rbp-0x28],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x2u64) as u32);
            // d46b9 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // d46bd mov rdi,rbx
            rdi = rbx;
            // d46c0 call 0xd40e0
            { let r = { let t = m.env.code_va(0xd40e0); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d46c5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d46c7 je 0xd4715
            if fl.e() { pc = 0xd4715; continue; }
            pc = 0xd46c9;
        }
        0xd46c9 => {
            // block d46c9
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d46c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46c9 mov r12d,DWORD PTR [rbp-0x24]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // d46cd xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d46d0 cmp r12d,0x11
            fl = sub::<32>((r12 as u32 as u64), 0x11u64).1;
            // d46d4 ja 0xd4718
            if fl.a() { pc = 0xd4718; continue; }
            pc = 0xd46d6;
        }
        0xd46d6 => {
            // block d46d6
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d46d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46d6 mov eax,0x3ffcf
            rax = (0x3ffcfu64) as u32 as u64;
            // d46db bt eax,r12d
            fl.cf = bit::<32>((rax as u32 as u64), (r12 as u32 as u64));
            // d46df jae 0xd4718
            if fl.ae() { pc = 0xd4718; continue; }
            pc = 0xd46e1;
        }
        0xd46e1 => {
            // block d46e1
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d46e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46e1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d46e4 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // d46e8 mov rdi,rbx
            rdi = rbx;
            // d46eb call QWORD PTR [rax+0x338]
            { let r = { let t = m.r64(rax.wrapping_add(0x338)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d46f1 cmp r12d,0x11
            fl = sub::<32>((r12 as u32 as u64), 0x11u64).1;
            // d46f5 ja 0xd4728
            if fl.a() { pc = 0xd4728; continue; }
            pc = 0xd46f7;
        }
        0xd46f7 => {
            // block d46f7
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d46f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46f7 mov eax,0x3c30c
            rax = (0x3c30cu64) as u32 as u64;
            // d46fc bt eax,r12d
            fl.cf = bit::<32>((rax as u32 as u64), (r12 as u32 as u64));
            // d4700 jae 0xd4728
            if fl.ae() { pc = 0xd4728; continue; }
            pc = 0xd4702;
        }
        0xd4702 => {
            // block d4702
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4702 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4702 lea rdi,[r14+0x16c]
            rdi = r14.wrapping_add(0x16c);
            // d4709 mov esi,0xff
            rsi = (0xffu64) as u32 as u64;
            // d470e call 0x28d56
            { let r = { let t = m.env.code_va(0x28d56); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4713 jmp 0xd478b
            pc = 0xd478b; continue;
        }
        0xd4715 => {
            // block d4715
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4715 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4715 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd4718;
        }
        0xd4718 => {
            // block d4718
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4718 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4718 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // d471b add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // d471f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4720 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d4722 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4724 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d4726 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4727 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd4728 => {
            // block d4728
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4728 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4728 cmp r12d,0xd
            fl = sub::<32>((r12 as u32 as u64), 0xdu64).1;
            // d472c ja 0xd478b
            if fl.a() { pc = 0xd478b; continue; }
            pc = 0xd472e;
        }
        0xd472e => {
            // block d472e
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d472e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d472e mov eax,0x3cc3
            rax = (0x3cc3u64) as u32 as u64;
            // d4733 bt eax,r12d
            fl.cf = bit::<32>((rax as u32 as u64), (r12 as u32 as u64));
            // d4737 jae 0xd478b
            if fl.ae() { pc = 0xd478b; continue; }
            pc = 0xd4739;
        }
        0xd4739 => {
            // block d4739
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4739 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4739 cmp DWORD PTR [r14+0x2a4],0xffffffff
            fl = sub::<32>(m.r32(r14.wrapping_add(0x2a4)) as u64, 0xffffffffu64).1;
            // d4741 jne 0xd4757
            if fl.ne() { pc = 0xd4757; continue; }
            pc = 0xd4743;
        }
        0xd4743 => {
            // block d4743
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4743 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4743 cmp DWORD PTR [r14+0x2a8],0xffffffff
            fl = sub::<32>(m.r32(r14.wrapping_add(0x2a8)) as u64, 0xffffffffu64).1;
            // d474b jne 0xd4757
            if fl.ne() { pc = 0xd4757; continue; }
            pc = 0xd474d;
        }
        0xd474d => {
            // block d474d
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d474d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d474d cmp DWORD PTR [r14+0x2ac],0xffffffff
            fl = sub::<32>(m.r32(r14.wrapping_add(0x2ac)) as u64, 0xffffffffu64).1;
            // d4755 je 0xd4766
            if fl.e() { pc = 0xd4766; continue; }
            pc = 0xd4757;
        }
        0xd4757 => {
            // block d4757
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4757 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4757 lea rsi,[r14+0x2a4]
            rsi = r14.wrapping_add(0x2a4);
            // d475e mov rdi,r14
            rdi = r14;
            // d4761 call 0x2b652
            { let r = { let t = m.env.code_va(0x2b652); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd4766;
        }
        0xd4766 => {
            // block d4766
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4766 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4766 mov rsi,QWORD PTR [r14+0xc0]
            rsi = m.r64(r14.wrapping_add(0xc0)) as u64;
            // d476d test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // d4770 je 0xd478b
            if fl.e() { pc = 0xd478b; continue; }
            pc = 0xd4772;
        }
        0xd4772 => {
            // block d4772
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4772 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4772 cmp DWORD PTR [rsi],0xffffffff
            fl = sub::<32>(m.r32(rsi) as u64, 0xffffffffu64).1;
            // d4775 jne 0xd4783
            if fl.ne() { pc = 0xd4783; continue; }
            pc = 0xd4777;
        }
        0xd4777 => {
            // block d4777
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4777 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4777 cmp DWORD PTR [rsi+0x4],0xffffffff
            fl = sub::<32>(m.r32(rsi.wrapping_add(0x4)) as u64, 0xffffffffu64).1;
            // d477b jne 0xd4783
            if fl.ne() { pc = 0xd4783; continue; }
            pc = 0xd477d;
        }
        0xd477d => {
            // block d477d
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d477d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d477d cmp DWORD PTR [rsi+0x8],0xffffffff
            fl = sub::<32>(m.r32(rsi.wrapping_add(0x8)) as u64, 0xffffffffu64).1;
            // d4781 je 0xd478b
            if fl.e() { pc = 0xd478b; continue; }
            pc = 0xd4783;
        }
        0xd4783 => {
            // block d4783
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4783 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4783 mov rdi,r14
            rdi = r14;
            // d4786 call 0x2b652
            { let r = { let t = m.env.code_va(0x2b652); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd478b;
        }
        0xd478b => {
            // block d478b
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d478b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d478b mov DWORD PTR [r14+0x2b0],r12d
            m.w32(r14.wrapping_add(0x2b0), ((r12 as u32 as u64)) as u32);
            // d4792 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d4795 mov rdi,rbx
            rdi = rbx;
            // d4798 mov rsi,r14
            rsi = r14;
            // d479b call QWORD PTR [rax+0xd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xd0)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d47a1 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d47a4 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // d47a7 je 0xd4718
            if fl.e() { pc = 0xd4718; continue; }
            pc = 0xd47ad;
        }
        0xd47ad => {
            // block d47ad
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d47ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d47ad cmp DWORD PTR [rbp-0x28],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, 0x1u64).1;
            // d47b1 jne 0xd4718
            if fl.ne() { pc = 0xd4718; continue; }
            pc = 0xd47b7;
        }
        0xd47b7 => {
            // block d47b7
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d47b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d47b7 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // d47ba cmp r12d,0x11
            fl = sub::<32>((r12 as u32 as u64), 0x11u64).1;
            // d47be ja 0xd4718
            if fl.a() { pc = 0xd4718; continue; }
            pc = 0xd47c4;
        }
        0xd47c4 => {
            // block d47c4
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d47c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d47c4 mov eax,0x3c30c
            rax = (0x3c30cu64) as u32 as u64;
            // d47c9 bt eax,r12d
            fl.cf = bit::<32>((rax as u32 as u64), (r12 as u32 as u64));
            // d47cd jae 0xd4718
            if fl.ae() { pc = 0xd4718; continue; }
            pc = 0xd47d3;
        }
        0xd47d3 => {
            // block d47d3
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d47d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d47d3 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // d47da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d47dd lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // d47e1 mov esi,0x2724
            rsi = (0x2724u64) as u32 as u64;
            // d47e6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d47eb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d47ee mov rdi,rbx
            rdi = rbx;
            // d47f1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d47f4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d47f7 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d47fb cmp DWORD PTR [rbp-0x2c],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64, 0x1u64).1;
            // d47ff jne 0xd482d
            if fl.ne() { pc = 0xd482d; continue; }
            pc = 0xd4801;
        }
        0xd4801 => {
            // block d4801
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4801 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4801 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d4805 mov esi,0x27b5
            rsi = (0x27b5u64) as u32 as u64;
            // d480a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d480f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d4812 mov rdi,rbx
            rdi = rbx;
            // d4815 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4817 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // d481a cmp eax,0xfe
            fl = sub::<32>((rax as u32 as u64), 0xfeu64).1;
            // d481f je 0xd4859
            if fl.e() { pc = 0xd4859; continue; }
            pc = 0xd4821;
        }
        0xd4821 => {
            // block d4821
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4821 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4821 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d4824 jne 0xd4860
            if fl.ne() { pc = 0xd4860; continue; }
            pc = 0xd4826;
        }
        0xd4826 => {
            // block d4826
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4826 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4826 mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // d482b jmp 0xd486f
            pc = 0xd486f; continue;
        }
        0xd482d => {
            // block d482d
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d482d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d482d lea rdx,[rbp-0x44]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // d4831 mov esi,0x27b5
            rsi = (0x27b5u64) as u32 as u64;
            // d4836 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d483b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d483e mov rdi,rbx
            rdi = rbx;
            // d4841 call rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4843 mov eax,DWORD PTR [rbp-0x44]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64) as u32 as u64;
            // d4846 cmp eax,0xfe
            fl = sub::<32>((rax as u32 as u64), 0xfeu64).1;
            // d484b je 0xd48ac
            if fl.e() { pc = 0xd48ac; continue; }
            pc = 0xd484d;
        }
        0xd484d => {
            // block d484d
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d484d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d484d cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d4850 jne 0xd48b3
            if fl.ne() { pc = 0xd48b3; continue; }
            pc = 0xd4852;
        }
        0xd4852 => {
            // block d4852
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4852 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4852 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // d4857 jmp 0xd48c2
            pc = 0xd48c2; continue;
        }
        0xd4859 => {
            // block d4859
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4859 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4859 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // d485e jmp 0xd486f
            pc = 0xd486f; continue;
        }
        0xd4860 => {
            // block d4860
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d4860 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4860 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d4865 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d4868 jne 0xd486f
            if fl.ne() { pc = 0xd486f; continue; }
            pc = 0xd486a;
        }
        0xd486a => {
            // block d486a
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d486a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d486a mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            pc = 0xd486f;
        }
        0xd486f => {
            // block d486f
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d486f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d486f mov DWORD PTR [rbp-0x40],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0xffffffffu64) as u32);
            // d4876 mov DWORD PTR [rbp-0x3c],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0xffffffffu64) as u32);
            // d487d mov DWORD PTR [rbp-0x38],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0xffffffffu64) as u32);
            // d4884 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d4888 mov rdi,r14
            rdi = r14;
            // d488b call 0x2b77c
            { let r = { let t = m.env.code_va(0x2b77c); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4890 mov rax,QWORD PTR [r14+0x16c]
            rax = m.r64(r14.wrapping_add(0x16c)) as u64;
            // d4897 test rax,rax
            fl = logic::<64>(rax & rax);
            // d489a je 0xd4718
            if fl.e() { pc = 0xd4718; continue; }
            pc = 0xd48a0;
        }
        0xd48a0 => {
            // block d48a0
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48a0 mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // d48a3 mov DWORD PTR [rax+0x8],ecx
            m.w32(rax.wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // d48a6 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d48aa jmp 0xd48fd
            pc = 0xd48fd; continue;
        }
        0xd48ac => {
            // block d48ac
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48ac mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // d48b1 jmp 0xd48c2
            pc = 0xd48c2; continue;
        }
        0xd48b3 => {
            // block d48b3
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48b3 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // d48b8 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d48bb jne 0xd48c2
            if fl.ne() { pc = 0xd48c2; continue; }
            pc = 0xd48bd;
        }
        0xd48bd => {
            // block d48bd
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48bd mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            pc = 0xd48c2;
        }
        0xd48c2 => {
            // block d48c2
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48c2 mov DWORD PTR [rbp-0x50],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0xffffffffu64) as u32);
            // d48c9 mov DWORD PTR [rbp-0x4c],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), (0xffffffffu64) as u32);
            // d48d0 mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // d48d7 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // d48db mov rdi,r14
            rdi = r14;
            // d48de call 0x2b77c
            { let r = { let t = m.env.code_va(0x2b77c); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d48e3 mov rax,QWORD PTR [r14+0x16c]
            rax = m.r64(r14.wrapping_add(0x16c)) as u64;
            // d48ea test rax,rax
            fl = logic::<64>(rax & rax);
            // d48ed je 0xd4718
            if fl.e() { pc = 0xd4718; continue; }
            pc = 0xd48f3;
        }
        0xd48f3 => {
            // block d48f3
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48f3 mov ecx,DWORD PTR [rbp-0x48]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // d48f6 mov DWORD PTR [rax+0x8],ecx
            m.w32(rax.wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // d48f9 mov rcx,QWORD PTR [rbp-0x50]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0xd48fd;
        }
        0xd48fd => {
            // block d48fd
            #[cfg(feature = "trace")] eprintln!("f_000d4690 pc d48fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d48fd mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // d4900 jmp 0xd4718
            pc = 0xd4718; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd69d6 size=672
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d69d6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd69d6;
    loop {
        match pc {
        0xd69d6 => {
            // block d69d6
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d69d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d69d6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d69d7 mov rbp,rsp
            rbp = rsp;
            // d69da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d69dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d69dd mov rbx,rdi
            rbx = rdi;
            // d69e0 call 0xd6ec8
            { let r = { let t = m.env.code_va(0xd6ec8); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d69e5 mov DWORD PTR [rbx+0x330],eax
            m.w32(rbx.wrapping_add(0x330), ((rax as u32 as u64)) as u32);
            // d69eb mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d69ee call 0xcdaff
            { let r = { let t = m.env.code_va(0xcdaff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d69f3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d69f5 je 0xd6a3d
            if fl.e() { pc = 0xd6a3d; continue; }
            pc = 0xd69f7;
        }
        0xd69f7 => {
            // block d69f7
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d69f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d69f7 movabs rax,0x100000001000000
            rax = 0x100_0000_0100_0000u64;
            // d6a01 mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6a08 mov DWORD PTR [rbx+0x35c],0x80
            m.w32(rbx.wrapping_add(0x35c), (0x80u64) as u32);
            // d6a12 mov DWORD PTR [rbx+0x368],0x7f
            m.w32(rbx.wrapping_add(0x368), (0x7fu64) as u32);
            // d6a1c mov QWORD PTR [rbx+0x33c],0x10000
            m.w64(rbx.wrapping_add(0x33c), (0x10000u64) as u64);
            // d6a27 movabs rax,0xfe000000800000
            rax = 0xfe_0000_0080_0000u64;
            pc = 0xd6a31;
        }
        0xd6a31 => {
            // block d6a31
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a31 mov QWORD PTR [rbx+0x344],rax
            m.w64(rbx.wrapping_add(0x344), (rax) as u64);
            // d6a38 jmp 0xd6bad
            pc = 0xd6bad; continue;
        }
        0xd6a3d => {
            // block d6a3d
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a3d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6a40 call 0xcdb57
            { let r = { let t = m.env.code_va(0xcdb57); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6a45 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6a47 je 0xd6a80
            if fl.e() { pc = 0xd6a80; continue; }
            pc = 0xd6a49;
        }
        0xd6a49 => {
            // block d6a49
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a49 cmp DWORD PTR [rbx+0x330],0x10220
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x330)) as u64, 0x10220u64).1;
            // d6a53 jne 0xd6b0f
            if fl.ne() { pc = 0xd6b0f; continue; }
            pc = 0xd6a59;
        }
        0xd6a59 => {
            // block d6a59
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a59 movabs rax,0x100000004000000
            rax = 0x100_0000_0400_0000u64;
            // d6a63 mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6a6a mov DWORD PTR [rbx+0x35c],0x40
            m.w32(rbx.wrapping_add(0x35c), (0x40u64) as u32);
            // d6a74 mov DWORD PTR [rbx+0x368],0x3d
            m.w32(rbx.wrapping_add(0x368), (0x3du64) as u32);
            // d6a7e jmp 0xd6ab5
            pc = 0xd6ab5; continue;
        }
        0xd6a80 => {
            // block d6a80
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a80 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6a83 call 0xcdbbf
            { let r = { let t = m.env.code_va(0xcdbbf); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6a88 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6a8a je 0xd6b63
            if fl.e() { pc = 0xd6b63; continue; }
            pc = 0xd6a90;
        }
        0xd6a90 => {
            // block d6a90
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a90 movabs rax,0x100000004000000
            rax = 0x100_0000_0400_0000u64;
            // d6a9a mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6aa1 mov DWORD PTR [rbx+0x35c],0x40
            m.w32(rbx.wrapping_add(0x35c), (0x40u64) as u32);
            // d6aab mov DWORD PTR [rbx+0x368],0x3c
            m.w32(rbx.wrapping_add(0x368), (0x3cu64) as u32);
            pc = 0xd6ab5;
        }
        0xd6ab5 => {
            // block d6ab5
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6ab5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ab5 mov DWORD PTR [rbx+0x360],0x40
            m.w32(rbx.wrapping_add(0x360), (0x40u64) as u32);
            // d6abf mov DWORD PTR [rbx+0x364],0x40
            m.w32(rbx.wrapping_add(0x364), (0x40u64) as u32);
            // d6ac9 mov QWORD PTR [rbx+0x33c],0x40000
            m.w64(rbx.wrapping_add(0x33c), (0x40000u64) as u64);
            // d6ad4 mov DWORD PTR [rbx+0x354],0x0
            m.w32(rbx.wrapping_add(0x354), (0x0u64) as u32);
            // d6ade mov DWORD PTR [rbx+0x358],0x0
            m.w32(rbx.wrapping_add(0x358), (0x0u64) as u32);
            // d6ae8 movabs rax,0xf8000000000000
            rax = 0xf8_0000_0000_0000u64;
            // d6af2 mov QWORD PTR [rbx+0x344],rax
            m.w64(rbx.wrapping_add(0x344), (rax) as u64);
            // d6af9 movabs rax,0xf0000000f40000
            rax = 0xf0_0000_00f4_0000u64;
            // d6b03 mov QWORD PTR [rbx+0x34c],rax
            m.w64(rbx.wrapping_add(0x34c), (rax) as u64);
            // d6b0a jmp 0xd6bad
            pc = 0xd6bad; continue;
        }
        0xd6b0f => {
            // block d6b0f
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6b0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b0f movabs rax,0x100000002000000
            rax = 0x100_0000_0200_0000u64;
            // d6b19 mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6b20 mov DWORD PTR [rbx+0x35c],0x100
            m.w32(rbx.wrapping_add(0x35c), (0x100u64) as u32);
            // d6b2a mov DWORD PTR [rbx+0x368],0xff
            m.w32(rbx.wrapping_add(0x368), (0xffu64) as u32);
            // d6b34 mov QWORD PTR [rbx+0x33c],0x10000
            m.w64(rbx.wrapping_add(0x33c), (0x10000u64) as u64);
            // d6b3f movabs rax,0xfe000000000000
            rax = 0xfe_0000_0000_0000u64;
            // d6b49 mov QWORD PTR [rbx+0x344],rax
            m.w64(rbx.wrapping_add(0x344), (rax) as u64);
            // d6b50 movabs rax,0xfc000000fd0000
            rax = 0xfc_0000_00fd_0000u64;
            // d6b5a mov QWORD PTR [rbx+0x34c],rax
            m.w64(rbx.wrapping_add(0x34c), (rax) as u64);
            // d6b61 jmp 0xd6bad
            pc = 0xd6bad; continue;
        }
        0xd6b63 => {
            // block d6b63
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6b63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b63 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6b66 call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6b6b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6b6d je 0xd6c23
            if fl.e() { pc = 0xd6c23; continue; }
            pc = 0xd6b73;
        }
        0xd6b73 => {
            // block d6b73
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6b73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b73 movabs rax,0x100000004000000
            rax = 0x100_0000_0400_0000u64;
            // d6b7d mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6b84 mov DWORD PTR [rbx+0x35c],0x80
            m.w32(rbx.wrapping_add(0x35c), (0x80u64) as u32);
            // d6b8e mov DWORD PTR [rbx+0x368],0x7f
            m.w32(rbx.wrapping_add(0x368), (0x7fu64) as u32);
            // d6b98 mov QWORD PTR [rbx+0x33c],0x40000
            m.w64(rbx.wrapping_add(0x33c), (0x40000u64) as u64);
            // d6ba3 mov DWORD PTR [rbx+0x344],0x0
            m.w32(rbx.wrapping_add(0x344), (0x0u64) as u32);
            pc = 0xd6bad;
        }
        0xd6bad => {
            // block d6bad
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6bad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6bad mov rdi,QWORD PTR [rbx+0x6b8]
            rdi = m.r64(rbx.wrapping_add(0x6b8)) as u64;
            // d6bb4 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6bb7 je 0xd6bca
            if fl.e() { pc = 0xd6bca; continue; }
            pc = 0xd6bb9;
        }
        0xd6bb9 => {
            // block d6bb9
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6bb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6bb9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d6bbc call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6bbf mov QWORD PTR [rbx+0x6b8],0x0
            m.w64(rbx.wrapping_add(0x6b8), (0x0u64) as u64);
            pc = 0xd6bca;
        }
        0xd6bca => {
            // block d6bca
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6bca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6bca mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6bcd mov rdi,rbx
            rdi = rbx;
            // d6bd0 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6bd6 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // d6bd8 call 0x2f83a
            { let r = { let t = m.env.code_va(0x2f83a); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6bdd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6bdf je 0xd6c1c
            if fl.e() { pc = 0xd6c1c; continue; }
            pc = 0xd6be1;
        }
        0xd6be1 => {
            // block d6be1
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6be1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6be1 mov edi,0x2c0
            rdi = (0x2c0u64) as u32 as u64;
            // d6be6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6beb mov r14,rax
            r14 = rax;
            // d6bee mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6bf1 mov rax,QWORD PTR [rax+0x1a0]
            rax = m.r64(rax.wrapping_add(0x1a0)) as u64;
            // d6bf8 mov rdi,rbx
            rdi = rbx;
            // d6bfb call rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6bfd movzx esi,ax
            rsi = ((rax as u16 as u64)) as u32 as u64;
            // d6c00 cmp BYTE PTR [rbx+0x378],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x378)) as u64, 0x0u64).1;
            // d6c07 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d6c0a movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // d6c0d mov rdi,r14
            rdi = r14;
            // d6c10 call 0x2f7d2
            { let r = { let t = m.env.code_va(0x2f7d2); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6c15 mov QWORD PTR [rbx+0x6b8],r14
            m.w64(rbx.wrapping_add(0x6b8), (r14) as u64);
            pc = 0xd6c1c;
        }
        0xd6c1c => {
            // block d6c1c
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6c1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c1c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d6c1e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6c1f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6c21 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6c22 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd6c23 => {
            // block d6c23
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6c23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c23 movabs rax,0x100000001000000
            rax = 0x100_0000_0100_0000u64;
            // d6c2d mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6c34 mov DWORD PTR [rbx+0x35c],0x40
            m.w32(rbx.wrapping_add(0x35c), (0x40u64) as u32);
            // d6c3e mov DWORD PTR [rbx+0x368],0x3f
            m.w32(rbx.wrapping_add(0x368), (0x3fu64) as u32);
            // d6c48 mov QWORD PTR [rbx+0x33c],0x10000
            m.w64(rbx.wrapping_add(0x33c), (0x10000u64) as u64);
            // d6c53 movabs rax,0xfe000000c00000
            rax = 0xfe_0000_00c0_0000u64;
            // d6c5d jmp 0xd6a31
            pc = 0xd6a31; continue;
        }
        0xd6c62 => {
            // block d6c62
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6c62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c62 mov rbx,rax
            rbx = rax;
            // d6c65 mov rdi,r14
            rdi = r14;
            // d6c68 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6c6d mov rdi,rbx
            rdi = rbx;
            // d6c70 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7110 size=600
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7110<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd7110;
    loop {
        match pc {
        0xd7110 => {
            // block d7110
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7110 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7110 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7111 mov rbp,rsp
            rbp = rsp;
            // d7114 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d7116 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d7118 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d711a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d711c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d711d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d711e mov r12,rsi
            r12 = rsi;
            // d7121 mov r14,rdi
            r14 = rdi;
            // d7124 lea rsi,[rip+0xe6655] # 0x1bd780
            rsi = m.env.data_va(859981);
            // d712b mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // d7130 mov rdi,r12
            rdi = r12;
            // d7133 call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7138 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d713a jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd7140;
        }
        0xd7140 => {
            // block d7140
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7140 movzx eax,BYTE PTR [r12+0xd]
            rax = (m.r8(r12.wrapping_add(0xd)) as u64) as u32 as u64;
            // d7146 cmp eax,0x61
            fl = sub::<32>((rax as u32 as u64), 0x61u64).1;
            // d7149 jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd714f;
        }
        0xd714f => {
            // block d714f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d714f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d714f mov bx,WORD PTR [r12+0xe]
            rbx = (rbx & !0xffff) | ((m.r16(r12.wrapping_add(0xe)) as u64) as u16 as u64);
            // d7155 rol bx,0x8
            { let (r, f) = rol::<16>((rbx as u16 as u64), 0x8u64, fl); rbx = (rbx & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d7159 lea rsi,[r12+0x10]
            rsi = r12.wrapping_add(0x10);
            // d715e lea rdi,[r14+0x2e8]
            rdi = r14.wrapping_add(0x2e8);
            // d7165 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d716a movzx ebx,bx
            rbx = ((rbx as u16 as u64)) as u32 as u64;
            // d716d movzx eax,BYTE PTR [rbx+r12*1+0x10]
            rax = (m.r8(rbx.wrapping_add(r12).wrapping_add(0x10)) as u64) as u32 as u64;
            // d7173 cmp eax,0x62
            fl = sub::<32>((rax as u32 as u64), 0x62u64).1;
            // d7176 jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd717c;
        }
        0xd717c => {
            // block d717c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d717c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d717c mov r13w,WORD PTR [rbx+r12*1+0x11]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x11)) as u64) as u16 as u64);
            // d7182 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d7187 lea rsi,[r12+rbx*1+0x13]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x13);
            // d718c lea r15,[r14+0x300]
            r15 = r14.wrapping_add(0x300);
            // d7193 mov rdi,r15
            rdi = r15;
            // d7196 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d719b movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d719f lea rbx,[rax+rbx*1+0x13]
            rbx = rax.wrapping_add(rbx).wrapping_add(0x13);
            // d71a4 movzx eax,BYTE PTR [r12+rbx*1]
            rax = (m.r8(r12.wrapping_add(rbx)) as u64) as u32 as u64;
            // d71a9 cmp eax,0x63
            fl = sub::<32>((rax as u32 as u64), 0x63u64).1;
            // d71ac jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd71ae;
        }
        0xd71ae => {
            // block d71ae
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d71ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d71ae mov r13w,WORD PTR [rbx+r12*1+0x1]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x1)) as u64) as u16 as u64);
            // d71b4 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d71b9 lea rsi,[r12+rbx*1+0x3]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x3);
            // d71be lea rdi,[r14+0x2b8]
            rdi = r14.wrapping_add(0x2b8);
            // d71c5 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d71ca movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d71ce lea rbx,[rax+rbx*1+0x3]
            rbx = rax.wrapping_add(rbx).wrapping_add(0x3);
            // d71d3 movzx eax,BYTE PTR [r12+rbx*1]
            rax = (m.r8(r12.wrapping_add(rbx)) as u64) as u32 as u64;
            // d71d8 cmp eax,0x64
            fl = sub::<32>((rax as u32 as u64), 0x64u64).1;
            // d71db jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd71dd;
        }
        0xd71dd => {
            // block d71dd
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d71dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d71dd mov r13w,WORD PTR [rbx+r12*1+0x1]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x1)) as u64) as u16 as u64);
            // d71e3 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d71e8 lea rsi,[r12+rbx*1+0x3]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x3);
            // d71ed lea rdi,[r14+0x2d0]
            rdi = r14.wrapping_add(0x2d0);
            // d71f4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d71f9 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d71fd lea rax,[rax+rbx*1+0x3]
            rax = rax.wrapping_add(rbx).wrapping_add(0x3);
            // d7202 movzx ecx,BYTE PTR [r12+rax*1]
            rcx = (m.r8(r12.wrapping_add(rax)) as u64) as u32 as u64;
            // d7207 cmp ecx,0x65
            fl = sub::<32>((rcx as u32 as u64), 0x65u64).1;
            // d720a jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd720c;
        }
        0xd720c => {
            // block d720c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d720c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d720c mov ecx,DWORD PTR [rax+r12*1+0x1]
            rcx = (m.r32(rax.wrapping_add(r12).wrapping_add(0x1)) as u64) as u32 as u64;
            // d7211 bswap ecx
            rcx = (rcx as u32).swap_bytes() as u64;
            // d7213 mov DWORD PTR [r14+0x36c],ecx
            m.w32(r14.wrapping_add(0x36c), ((rcx as u32 as u64)) as u32);
            // d721a mov cl,BYTE PTR [r14+0x300]
            rcx = (rcx & !0xff) | ((m.r8(r14.wrapping_add(0x300)) as u64) as u8 as u64);
            // d7221 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d7224 jne 0xd723d
            if fl.ne() { pc = 0xd723d; continue; }
            pc = 0xd7226;
        }
        0xd7226 => {
            // block d7226
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7226 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7226 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d722a jmp 0xd7244
            pc = 0xd7244; continue;
        }
        0xd722c => {
            // block d722c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d722c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d722c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd722e;
        }
        0xd722e => {
            // block d722e
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d722e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d722e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d7232 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7233 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d7235 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d7237 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7239 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d723b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d723c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd723d => {
            // block d723d
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d723d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d723d mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7244;
        }
        0xd7244 => {
            // block d7244
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7244 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7247 cmp edx,0x35
            fl = sub::<32>((rdx as u32 as u64), 0x35u64).1;
            // d724a je 0xd729b
            if fl.e() { pc = 0xd729b; continue; }
            pc = 0xd724c;
        }
        0xd724c => {
            // block d724c
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d724c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d724c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d724e jne 0xd7256
            if fl.ne() { pc = 0xd7256; continue; }
            pc = 0xd7250;
        }
        0xd7250 => {
            // block d7250
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7250 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7254 jmp 0xd725d
            pc = 0xd725d; continue;
        }
        0xd7256 => {
            // block d7256
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7256 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7256 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd725d;
        }
        0xd725d => {
            // block d725d
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d725d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d725d movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7260 cmp edx,0x36
            fl = sub::<32>((rdx as u32 as u64), 0x36u64).1;
            // d7263 je 0xd729b
            if fl.e() { pc = 0xd729b; continue; }
            pc = 0xd7265;
        }
        0xd7265 => {
            // block d7265
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7265 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7267 jne 0xd726f
            if fl.ne() { pc = 0xd726f; continue; }
            pc = 0xd7269;
        }
        0xd7269 => {
            // block d7269
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7269 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d726d jmp 0xd7276
            pc = 0xd7276; continue;
        }
        0xd726f => {
            // block d726f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d726f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d726f mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7276;
        }
        0xd7276 => {
            // block d7276
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7276 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7279 cmp edx,0x37
            fl = sub::<32>((rdx as u32 as u64), 0x37u64).1;
            // d727c je 0xd729b
            if fl.e() { pc = 0xd729b; continue; }
            pc = 0xd727e;
        }
        0xd727e => {
            // block d727e
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d727e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d727e test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7280 jne 0xd7288
            if fl.ne() { pc = 0xd7288; continue; }
            pc = 0xd7282;
        }
        0xd7282 => {
            // block d7282
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7282 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7286 jmp 0xd728f
            pc = 0xd728f; continue;
        }
        0xd7288 => {
            // block d7288
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7288 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7288 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd728f;
        }
        0xd728f => {
            // block d728f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d728f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d728f movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7292 cmp edx,0x78
            fl = sub::<32>((rdx as u32 as u64), 0x78u64).1;
            // d7295 jne 0xd7361
            if fl.ne() { pc = 0xd7361; continue; }
            pc = 0xd729b;
        }
        0xd729b => {
            // block d729b
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d729b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d729b test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d729d jne 0xd72a5
            if fl.ne() { pc = 0xd72a5; continue; }
            pc = 0xd729f;
        }
        0xd729f => {
            // block d729f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d729f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d729f lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72a3 jmp 0xd72ac
            pc = 0xd72ac; continue;
        }
        0xd72a5 => {
            // block d72a5
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72a5 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72ac;
        }
        0xd72ac => {
            // block d72ac
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72ac movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72af cmp edx,0x35
            fl = sub::<32>((rdx as u32 as u64), 0x35u64).1;
            // d72b2 je 0xd731a
            if fl.e() { pc = 0xd731a; continue; }
            pc = 0xd72b4;
        }
        0xd72b4 => {
            // block d72b4
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72b4 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72b6 jne 0xd72be
            if fl.ne() { pc = 0xd72be; continue; }
            pc = 0xd72b8;
        }
        0xd72b8 => {
            // block d72b8
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72b8 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72bc jmp 0xd72c5
            pc = 0xd72c5; continue;
        }
        0xd72be => {
            // block d72be
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72be mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72c5;
        }
        0xd72c5 => {
            // block d72c5
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72c5 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72c8 cmp edx,0x36
            fl = sub::<32>((rdx as u32 as u64), 0x36u64).1;
            // d72cb jne 0xd72e7
            if fl.ne() { pc = 0xd72e7; continue; }
            pc = 0xd72cd;
        }
        0xd72cd => {
            // block d72cd
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72cd test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72cf jne 0xd72d7
            if fl.ne() { pc = 0xd72d7; continue; }
            pc = 0xd72d1;
        }
        0xd72d1 => {
            // block d72d1
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72d1 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72d5 jmp 0xd72de
            pc = 0xd72de; continue;
        }
        0xd72d7 => {
            // block d72d7
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72d7 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72de;
        }
        0xd72de => {
            // block d72de
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72de movzx edx,BYTE PTR [rdx+0x1]
            rdx = (m.r8(rdx.wrapping_add(0x1)) as u64) as u32 as u64;
            // d72e2 cmp edx,0x76
            fl = sub::<32>((rdx as u32 as u64), 0x76u64).1;
            // d72e5 je 0xd731a
            if fl.e() { pc = 0xd731a; continue; }
            pc = 0xd72e7;
        }
        0xd72e7 => {
            // block d72e7
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72e7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72e9 jne 0xd72f1
            if fl.ne() { pc = 0xd72f1; continue; }
            pc = 0xd72eb;
        }
        0xd72eb => {
            // block d72eb
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72eb lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72ef jmp 0xd72f8
            pc = 0xd72f8; continue;
        }
        0xd72f1 => {
            // block d72f1
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72f1 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72f8;
        }
        0xd72f8 => {
            // block d72f8
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d72f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72f8 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72fb cmp edx,0x37
            fl = sub::<32>((rdx as u32 as u64), 0x37u64).1;
            // d72fe jne 0xd733a
            if fl.ne() { pc = 0xd733a; continue; }
            pc = 0xd7300;
        }
        0xd7300 => {
            // block d7300
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7300 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7300 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7302 jne 0xd730a
            if fl.ne() { pc = 0xd730a; continue; }
            pc = 0xd7304;
        }
        0xd7304 => {
            // block d7304
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7304 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7304 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7308 jmp 0xd7311
            pc = 0xd7311; continue;
        }
        0xd730a => {
            // block d730a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d730a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d730a mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7311;
        }
        0xd7311 => {
            // block d7311
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7311 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7311 movzx edx,BYTE PTR [rdx+0x1]
            rdx = (m.r8(rdx.wrapping_add(0x1)) as u64) as u32 as u64;
            // d7315 cmp edx,0x6b
            fl = sub::<32>((rdx as u32 as u64), 0x6bu64).1;
            // d7318 jne 0xd733a
            if fl.ne() { pc = 0xd733a; continue; }
            pc = 0xd731a;
        }
        0xd731a => {
            // block d731a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d731a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d731a lea rdi,[rax+r12*1+0x31]
            rdi = rax.wrapping_add(r12).wrapping_add(0x31);
            pc = 0xd731f;
        }
        0xd731f => {
            // block d731f
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d731f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d731f lea rsi,[rip+0xe6467] # 0x1bd78d
            rsi = m.env.data_va(859994);
            // d7326 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // d732b call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7330 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d7332 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d7335 jmp 0xd722e
            pc = 0xd722e; continue;
        }
        0xd733a => {
            // block d733a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d733a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d733a test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d733c jne 0xd7343
            if fl.ne() { pc = 0xd7343; continue; }
            pc = 0xd733e;
        }
        0xd733e => {
            // block d733e
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d733e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d733e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // d7341 jmp 0xd734a
            pc = 0xd734a; continue;
        }
        0xd7343 => {
            // block d7343
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7343 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7343 mov r15,QWORD PTR [r14+0x310]
            r15 = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd734a;
        }
        0xd734a => {
            // block d734a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d734a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d734a movzx ecx,BYTE PTR [r15]
            rcx = (m.r8(r15) as u64) as u32 as u64;
            // d734e cmp ecx,0x78
            fl = sub::<32>((rcx as u32 as u64), 0x78u64).1;
            // d7351 jne 0xd735a
            if fl.ne() { pc = 0xd735a; continue; }
            pc = 0xd7353;
        }
        0xd7353 => {
            // block d7353
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7353 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7353 lea rdi,[rax+r12*1+0x51]
            rdi = rax.wrapping_add(r12).wrapping_add(0x51);
            // d7358 jmp 0xd731f
            pc = 0xd731f; continue;
        }
        0xd735a => {
            // block d735a
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d735a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d735a lea rdi,[rax+r12*1+0x11]
            rdi = rax.wrapping_add(r12).wrapping_add(0x11);
            // d735f jmp 0xd731f
            pc = 0xd731f; continue;
        }
        0xd7361 => {
            // block d7361
            #[cfg(feature = "trace")] eprintln!("f_000d7110 pc d7361 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7361 lea rdi,[rax+r12*1+0x5]
            rdi = rax.wrapping_add(r12).wrapping_add(0x5);
            // d7366 jmp 0xd731f
            pc = 0xd731f; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7eca size=630
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7eca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd7eca;
    loop {
        match pc {
        0xd7eca => {
            // block d7eca
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7eca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7eca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7ecb mov rbp,rsp
            rbp = rsp;
            // d7ece push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d7ed0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d7ed2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d7ed4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d7ed6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d7ed7 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // d7edb mov r15,rdi
            r15 = rdi;
            // d7ede dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // d7ee0 cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // d7ee3 ja 0xd7efe
            if fl.a() { pc = 0xd7efe; continue; }
            pc = 0xd7ee5;
        }
        0xd7ee5 => {
            // block d7ee5
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7ee5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7ee5 lea rax,[rip+0x238] # 0xd8124
            rax = m.env.code_va(0xd8124);
            // d7eec movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // d7ef0 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d7ef3 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd7ef5 => {
            // block d7ef5
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7ef5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7ef5 lea rax,[r15+0x344]
            rax = r15.wrapping_add(0x344);
            // d7efc jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
        0xd7efe => {
            // block d7efe
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7efe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7efe lea rax,[r15+0x340]
            rax = r15.wrapping_add(0x340);
            pc = 0xd7f05;
        }
        0xd7f05 => {
            // block d7f05
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f05 mov ebx,DWORD PTR [rax]
            rbx = (m.r32(rax) as u64) as u32 as u64;
            // d7f07 mov rax,QWORD PTR [r15+0x288]
            rax = m.r64(r15.wrapping_add(0x288)) as u64;
            // d7f0e mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // d7f12 mov eax,DWORD PTR [r15+0x298]
            rax = (m.r32(r15.wrapping_add(0x298)) as u64) as u32 as u64;
            // d7f19 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // d7f1c shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d7f1f cmp DWORD PTR [r15+0x370],0x1
            fl = sub::<32>(m.r32(r15.wrapping_add(0x370)) as u64, 0x1u64).1;
            // d7f27 jne 0xd7f30
            if fl.ne() { pc = 0xd7f30; continue; }
            pc = 0xd7f29;
        }
        0xd7f29 => {
            // block d7f29
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f29 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // d7f2e jmp 0xd7f32
            pc = 0xd7f32; continue;
        }
        0xd7f30 => {
            // block d7f30
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f30 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7f32;
        }
        0xd7f32 => {
            // block d7f32
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f32 mov rdi,r15
            rdi = r15;
            // d7f35 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // d7f38 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7f3d mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // d7f40 lea r14,[r15+0x378]
            r14 = r15.wrapping_add(0x378);
            // d7f47 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d7f49 je 0xd808a
            if fl.e() { pc = 0xd808a; continue; }
            pc = 0xd7f4f;
        }
        0xd7f4f => {
            // block d7f4f
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f4f mov QWORD PTR [rbp-0x50],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r14) as u64);
            // d7f53 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d7f56 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d7f59 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d7f5b mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d7f5f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d7f61 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0xd7f65;
        }
        0xd7f65 => {
            // block d7f65
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f65 mov DWORD PTR [rbp-0x30],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rcx as u32 as u64)) as u32);
            // d7f68 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // d7f6c call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7f71 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7f73 je 0xd7f9e
            if fl.e() { pc = 0xd7f9e; continue; }
            pc = 0xd7f75;
        }
        0xd7f75 => {
            // block d7f75
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f75 cmp ebx,DWORD PTR [r15+0x338]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r15.wrapping_add(0x338)) as u64).1;
            // d7f7c jne 0xd7f9e
            if fl.ne() { pc = 0xd7f9e; continue; }
            pc = 0xd7f7e;
        }
        0xd7f7e => {
            // block d7f7e
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f7e cmp DWORD PTR [r15+0x370],0x1
            fl = sub::<32>(m.r32(r15.wrapping_add(0x370)) as u64, 0x1u64).1;
            // d7f86 jne 0xd7f8f
            if fl.ne() { pc = 0xd7f8f; continue; }
            pc = 0xd7f88;
        }
        0xd7f88 => {
            // block d7f88
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f88 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d7f8d jmp 0xd7f94
            pc = 0xd7f94; continue;
        }
        0xd7f8f => {
            // block d7f8f
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f8f mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            pc = 0xd7f94;
        }
        0xd7f94 => {
            // block d7f94
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f94 mov rdi,r15
            rdi = r15;
            // d7f97 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7f9c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7f9e;
        }
        0xd7f9e => {
            // block d7f9e
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d7f9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7f9e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7fa1 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d7fa6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7fab xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7fae mov rdi,r15
            rdi = r15;
            // d7fb1 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d7fb3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7fb6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7fb9 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d7fbe mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d7fc3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7fc8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7fcb mov rdi,r15
            rdi = r15;
            // d7fce call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7fd1 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7fd4 mov rdi,r15
            rdi = r15;
            // d7fd7 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7fdd mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7fe0 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d7fe5 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7fea xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7fed mov rdi,r15
            rdi = r15;
            // d7ff0 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // d7ff4 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ff7 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // d7ffb mov edx,DWORD PTR [rax+r12*4]
            rdx = (m.r32(rax.wrapping_add(r12.wrapping_mul(4))) as u64) as u32 as u64;
            // d7fff mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // d8002 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // d8004 jne 0xd800b
            if fl.ne() { pc = 0xd800b; continue; }
            pc = 0xd8006;
        }
        0xd8006 => {
            // block d8006
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d8006 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8006 mov ecx,DWORD PTR [rbp-0x30]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // d8009 jmp 0xd802e
            pc = 0xd802e; continue;
        }
        0xd800b => {
            // block d800b
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d800b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d800b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d800d lea rdi,[rip+0x60dcf] # 0x138de3
            rdi = m.env.data_va(316848);
            // d8014 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // d8017 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d801c mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d8020 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d8022 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // d8026 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d8029 mov ecx,DWORD PTR [rbp-0x30]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // d802c ja 0xd8084
            if fl.a() { pc = 0xd8084; continue; }
            pc = 0xd802e;
        }
        0xd802e => {
            // block d802e
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d802e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d802e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d8030 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // d8033 div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // d8035 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d8039 cmp BYTE PTR [r14],0x0
            fl = sub::<8>(m.r8(r14) as u64, 0x0u64).1;
            // d803d jne 0xd806a
            if fl.ne() { pc = 0xd806a; continue; }
            pc = 0xd803f;
        }
        0xd803f => {
            // block d803f
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d803f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d803f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8041 lea rdi,[rip+0x60db5] # 0x138dfd
            rdi = m.env.data_va(316874);
            // d8048 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d804c mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // d804f call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8054 mov rax,QWORD PTR [rip+0x68385] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // d805b mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // d805e call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8063 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // d8066 mov r14,QWORD PTR [rbp-0x50]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0xd806a;
        }
        0xd806a => {
            // block d806a
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d806a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d806a add ebx,0x100
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x100u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d8070 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // d8074 add r13d,0x1900
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x1900u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d807b cmp r12d,ecx
            fl = sub::<32>((r12 as u32 as u64), (rcx as u32 as u64)).1;
            // d807e jb 0xd7f65
            if fl.b() { pc = 0xd7f65; continue; }
            pc = 0xd8084;
        }
        0xd8084 => {
            // block d8084
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d8084 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8084 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d8088 jmp 0xd8092
            pc = 0xd8092; continue;
        }
        0xd808a => {
            // block d808a
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d808a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d808a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d808c mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d8090 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd8092;
        }
        0xd8092 => {
            // block d8092
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d8092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8092 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d8094 mov rdi,r15
            rdi = r15;
            // d8097 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d809c test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // d809e mov al,BYTE PTR [r14]
            rax = (rax & !0xff) | ((m.r8(r14) as u64) as u8 as u64);
            // d80a1 je 0xd80ab
            if fl.e() { pc = 0xd80ab; continue; }
            pc = 0xd80a3;
        }
        0xd80a3 => {
            // block d80a3
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80a3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d80a5 je 0xd80c1
            if fl.e() { pc = 0xd80c1; continue; }
            pc = 0xd80a7;
        }
        0xd80a7 => {
            // block d80a7
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80a7 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d80a9 jmp 0xd80d5
            pc = 0xd80d5; continue;
        }
        0xd80ab => {
            // block d80ab
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80ab mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // d80ad test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d80af jne 0xd80d5
            if fl.ne() { pc = 0xd80d5; continue; }
            pc = 0xd80b1;
        }
        0xd80b1 => {
            // block d80b1
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80b1 lea rdi,[rip+0x60d78] # 0x138e30
            rdi = m.env.data_va(316925);
            // d80b8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d80ba call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d80bf jmp 0xd80d5
            pc = 0xd80d5; continue;
        }
        0xd80c1 => {
            // block d80c1
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80c1 lea rdi,[rip+0x60d4b] # 0x138e13
            rdi = m.env.data_va(316896);
            // d80c8 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d80ca xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d80cc mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d80d0 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd80d5;
        }
        0xd80d5 => {
            // block d80d5
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80d5 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d80d7 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // d80db pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d80dc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d80de pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d80e0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d80e2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d80e4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d80e5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd80e6 => {
            // block d80e6
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80e6 lea rax,[r15+0x354]
            rax = r15.wrapping_add(0x354);
            // d80ed jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
        0xd80f2 => {
            // block d80f2
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80f2 lea rax,[r15+0x358]
            rax = r15.wrapping_add(0x358);
            // d80f9 jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
        0xd80fe => {
            // block d80fe
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d80fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d80fe lea rax,[r15+0x348]
            rax = r15.wrapping_add(0x348);
            // d8105 jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
        0xd810a => {
            // block d810a
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d810a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d810a lea rax,[r15+0x34c]
            rax = r15.wrapping_add(0x34c);
            // d8111 jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
        0xd8116 => {
            // block d8116
            #[cfg(feature = "trace")] eprintln!("f_000d7eca pc d8116 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8116 lea rax,[r15+0x350]
            rax = r15.wrapping_add(0x350);
            // d811d jmp 0xd7f05
            pc = 0xd7f05; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8382 size=510
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d8382<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd8382;
    loop {
        match pc {
        0xd8382 => {
            // block d8382
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d8382 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8382 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8383 mov rbp,rsp
            rbp = rsp;
            // d8386 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d8388 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d838a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d838c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d838e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d838f sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d8393 mov r14,rdi
            r14 = rdi;
            // d8396 lea eax,[rsi-0x1]
            rax = (rsi.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // d8399 cmp eax,0x6
            fl = sub::<32>((rax as u32 as u64), 0x6u64).1;
            // d839c ja 0xd83be
            if fl.a() { pc = 0xd83be; continue; }
            pc = 0xd839e;
        }
        0xd839e => {
            // block d839e
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d839e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d839e lea rcx,[rip+0x1bf] # 0xd8564
            rcx = m.env.code_va(0xd8564);
            // d83a5 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // d83a9 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // d83ac jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd83ae => {
            // block d83ae
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d83ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d83ae lea rax,[r14+0x344]
            rax = r14.wrapping_add(0x344);
            // d83b5 mov ecx,DWORD PTR [r14+0x368]
            rcx = (m.r32(r14.wrapping_add(0x368)) as u64) as u32 as u64;
            // d83bc jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
        0xd83be => {
            // block d83be
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d83be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d83be lea rax,[r14+0x340]
            rax = r14.wrapping_add(0x340);
            // d83c5 mov ecx,DWORD PTR [r14+0x35c]
            rcx = (m.r32(r14.wrapping_add(0x35c)) as u64) as u32 as u64;
            pc = 0xd83cc;
        }
        0xd83cc => {
            // block d83cc
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d83cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d83cc mov ebx,DWORD PTR [rax]
            rbx = (m.r32(rax) as u64) as u32 as u64;
            // d83ce imul ecx,DWORD PTR [r14+0x33c]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(r14.wrapping_add(0x33c)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // d83d6 shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // d83d9 mov rdi,r14
            rdi = r14;
            // d83dc mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // d83df call 0xd74b0
            { let r = { let t = m.env.code_va(0xd74b0); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d83e4 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // d83e7 lea rax,[r14+0x378]
            rax = r14.wrapping_add(0x378);
            // d83ee test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d83f0 je 0xd84c4
            if fl.e() { pc = 0xd84c4; continue; }
            pc = 0xd83f6;
        }
        0xd83f6 => {
            // block d83f6
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d83f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d83f6 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d83fa mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // d83fd xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd8400;
        }
        0xd8400 => {
            // block d8400
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d8400 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8400 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // d8403 mov BYTE PTR [rbp-0x2d],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3), ((rax as u8 as u64)) as u8);
            // d8406 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d8409 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d840c mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d8411 mov r13d,ecx
            r13 = ((rcx as u32 as u64)) as u32 as u64;
            // d8414 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8419 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d841c mov rdi,r14
            rdi = r14;
            // d841f mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d8421 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8424 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d8427 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d842c mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d8431 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8436 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8439 mov rdi,r14
            rdi = r14;
            // d843c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d843f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d8442 mov rdi,r14
            rdi = r14;
            // d8445 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d844b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d844e mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d8453 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8458 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d845b mov rdi,r14
            rdi = r14;
            // d845e lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // d8462 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8465 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // d8468 cmp DWORD PTR [rbp-0x2c],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64, 0xffffffffu64).1;
            // d846c mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // d846e jne 0xd84b2
            if fl.ne() { pc = 0xd84b2; continue; }
            pc = 0xd8470;
        }
        0xd8470 => {
            // block d8470
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d8470 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8470 mov r13b,BYTE PTR [rbp-0x2d]
            r13 = (r13 & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3)) as u64) as u8 as u64);
            // d8474 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d8478 cmp BYTE PTR [rax],0x0
            fl = sub::<8>(m.r8(rax) as u64, 0x0u64).1;
            // d847b jne 0xd84ac
            if fl.ne() { pc = 0xd84ac; continue; }
            pc = 0xd847d;
        }
        0xd847d => {
            // block d847d
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d847d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d847d imul eax,r15d,0x64
            { let (r, f) = imul::<32>((r15 as u32 as u64), 0x64u64); rax = (r) as u32 as u64; fl = f; }
            // d8481 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d8483 div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // d8485 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // d8488 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d848a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d848c lea rdi,[rip+0x609c8] # 0x138e5b
            rdi = m.env.data_va(316968);
            // d8493 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // d8495 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d849a mov rax,QWORD PTR [rip+0x67f3f] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // d84a1 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // d84a4 call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d84a9 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            pc = 0xd84ac;
        }
        0xd84ac => {
            // block d84ac
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84ac mov r12b,r13b
            r12 = (r12 & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // d84af mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            pc = 0xd84b2;
        }
        0xd84b2 => {
            // block d84b2
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84b2 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d84b4 add ebx,0x4
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x4u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d84b7 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // d84b9 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // d84bc jb 0xd8400
            if fl.b() { pc = 0xd8400; continue; }
            pc = 0xd84c2;
        }
        0xd84c2 => {
            // block d84c2
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84c2 jmp 0xd84cb
            pc = 0xd84cb; continue;
        }
        0xd84c4 => {
            // block d84c4
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84c4 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d84c8 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd84cb;
        }
        0xd84cb => {
            // block d84cb
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84cb and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d84cf je 0xd84e9
            if fl.e() { pc = 0xd84e9; continue; }
            pc = 0xd84d1;
        }
        0xd84d1 => {
            // block d84d1
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84d1 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d84d5 mov al,BYTE PTR [rax]
            rax = (rax & !0xff) | ((m.r8(rax) as u64) as u8 as u64);
            // d84d7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d84d9 jne 0xd84e9
            if fl.ne() { pc = 0xd84e9; continue; }
            pc = 0xd84db;
        }
        0xd84db => {
            // block d84db
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84db lea rdi,[rip+0x6098d] # 0x138e6f
            rdi = m.env.data_va(316988);
            // d84e2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d84e4 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd84e9;
        }
        0xd84e9 => {
            // block d84e9
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d84e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d84e9 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d84eb mov rdi,r14
            rdi = r14;
            // d84ee call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d84f3 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // d84f6 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // d84f9 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d84fd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d84fe pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d8500 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d8502 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d8504 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d8506 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8507 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd8508 => {
            // block d8508
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d8508 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8508 lea rax,[r14+0x354]
            rax = r14.wrapping_add(0x354);
            // d850f mov ecx,DWORD PTR [r14+0x360]
            rcx = (m.r32(r14.wrapping_add(0x360)) as u64) as u32 as u64;
            // d8516 jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
        0xd851b => {
            // block d851b
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d851b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d851b lea rax,[r14+0x358]
            rax = r14.wrapping_add(0x358);
            // d8522 mov ecx,DWORD PTR [r14+0x364]
            rcx = (m.r32(r14.wrapping_add(0x364)) as u64) as u32 as u64;
            // d8529 jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
        0xd852e => {
            // block d852e
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d852e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d852e lea rax,[r14+0x348]
            rax = r14.wrapping_add(0x348);
            // d8535 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // d853a jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
        0xd853f => {
            // block d853f
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d853f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d853f lea rax,[r14+0x34c]
            rax = r14.wrapping_add(0x34c);
            // d8546 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // d854b jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
        0xd8550 => {
            // block d8550
            #[cfg(feature = "trace")] eprintln!("f_000d8382 pc d8550 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8550 lea rax,[r14+0x350]
            rax = r14.wrapping_add(0x350);
            // d8557 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // d855c jmp 0xd83cc
            pc = 0xd83cc; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdb4bc size=506
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000db4bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdb4bc;
    loop {
        match pc {
        0xdb4bc => {
            // block db4bc
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db4bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db4bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // db4bd mov rbp,rsp
            rbp = rsp;
            // db4c0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // db4c2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // db4c4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // db4c6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // db4c8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // db4c9 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // db4cd mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // db4d0 mov rbx,rdi
            rbx = rdi;
            // db4d3 call 0xd74b0
            { let r = { let t = m.env.code_va(0xd74b0); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db4d8 mov r15d,DWORD PTR [rbx+0x338]
            r15 = (m.r32(rbx.wrapping_add(0x338)) as u64) as u32 as u64;
            // db4df shr r15d,0x2
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x2u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // db4e3 mov DWORD PTR [rbp-0x3c],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((r15 as u32 as u64)) as u32);
            // db4e7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db4ea mov esi,0x2880
            rsi = (0x2880u64) as u32 as u64;
            // db4ef mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db4f4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db4f7 mov rdi,rbx
            rdi = rbx;
            // db4fa mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // db4fd call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db500 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // db503 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // db505 je 0xdb659
            if fl.e() { pc = 0xdb659; continue; }
            pc = 0xdb50b;
        }
        0xdb50b => {
            // block db50b
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db50b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db50b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // db50e xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // db511 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // db514 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db516 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            pc = 0xdb51a;
        }
        0xdb51a => {
            // block db51a
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db51a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db51a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db51d mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // db522 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db527 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db52a mov rdi,rbx
            rdi = rbx;
            // db52d mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // db530 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db533 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db536 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // db53b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db540 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db543 mov rdi,rbx
            rdi = rbx;
            // db546 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // db549 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db54c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db54f mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // db554 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // db559 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db55e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db561 mov rdi,rbx
            rdi = rbx;
            // db564 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db567 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db56a mov rdi,rbx
            rdi = rbx;
            // db56d call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db573 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // db576 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // db57b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // db580 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // db583 mov rdi,rbx
            rdi = rbx;
            // db586 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // db58a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db58d mov rax,QWORD PTR [rbx+0x6a0]
            rax = m.r64(rbx.wrapping_add(0x6a0)) as u64;
            // db594 movzx ecx,BYTE PTR [rax+r12*4]
            rcx = (m.r8(rax.wrapping_add(r12.wrapping_mul(4))) as u64) as u32 as u64;
            // db599 movzx edx,BYTE PTR [rax+r12*4+0x1]
            rdx = (m.r8(rax.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x1)) as u64) as u32 as u64;
            // db59f movzx esi,BYTE PTR [rax+r12*4+0x2]
            rsi = (m.r8(rax.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2)) as u64) as u32 as u64;
            // db5a5 movzx eax,BYTE PTR [rax+r12*4+0x3]
            rax = (m.r8(rax.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x3)) as u64) as u32 as u64;
            // db5ab shl eax,0x18
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x18u64, fl); rax = (r) as u32 as u64; fl = f; }
            // db5ae shl esi,0x10
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x10u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // db5b1 shl edx,0x8
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x8u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // db5b4 or edx,ecx
            { let r = (rdx as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db5b6 or edx,esi
            { let r = (rdx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db5b8 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db5ba mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // db5bd cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // db5bf jne 0xdb5c6
            if fl.ne() { pc = 0xdb5c6; continue; }
            pc = 0xdb5c1;
        }
        0xdb5c1 => {
            // block db5c1
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db5c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db5c1 mov ecx,DWORD PTR [rbp-0x3c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64) as u32 as u64;
            // db5c4 jmp 0xdb5e9
            pc = 0xdb5e9; continue;
        }
        0xdb5c6 => {
            // block db5c6
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db5c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db5c6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db5c8 lea rdi,[rip+0x5d814] # 0x138de3
            rdi = m.env.data_va(316848);
            // db5cf mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // db5d2 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db5d7 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // db5db inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // db5dd mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // db5e1 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // db5e4 mov ecx,DWORD PTR [rbp-0x3c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64) as u32 as u64;
            // db5e7 jae 0xdb667
            if fl.ae() { pc = 0xdb667; continue; }
            pc = 0xdb5e9;
        }
        0xdb5e9 => {
            // block db5e9
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db5e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db5e9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // db5eb mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // db5ee div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // db5f0 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // db5f3 cmp BYTE PTR [rbx+0x378],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x378)) as u64, 0x0u64).1;
            // db5fa jne 0xdb62a
            if fl.ne() { pc = 0xdb62a; continue; }
            pc = 0xdb5fc;
        }
        0xdb5fc => {
            // block db5fc
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db5fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db5fc xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db5fe lea rdi,[rip+0x5d7f8] # 0x138dfd
            rdi = m.env.data_va(316874);
            // db605 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // db608 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // db60c mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // db60f call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db614 mov rax,QWORD PTR [rip+0x64dc5] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // db61b mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // db61e call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db623 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // db626 mov r15,QWORD PTR [rbp-0x48]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xdb62a;
        }
        0xdb62a => {
            // block db62a
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db62a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db62a add r14d,0x190
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x190u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // db631 add r12,0x64
            { let (r, f) = add::<64>(r12, 0x64u64);
              r12 = r;
              fl = f; }
            // db635 add r13d,0x2710
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x2710u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // db63c cmp r12d,ecx
            fl = sub::<32>((r12 as u32 as u64), (rcx as u32 as u64)).1;
            // db63f jb 0xdb51a
            if fl.b() { pc = 0xdb51a; continue; }
            pc = 0xdb645;
        }
        0xdb645 => {
            // block db645
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db645 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // db649 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // db64b mov al,BYTE PTR [rbx+0x378]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0x378)) as u64) as u8 as u64);
            // db651 je 0xdb68b
            if fl.e() { pc = 0xdb68b; continue; }
            pc = 0xdb653;
        }
        0xdb653 => {
            // block db653
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db653 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db653 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // db655 jne 0xdb670
            if fl.ne() { pc = 0xdb670; continue; }
            pc = 0xdb657;
        }
        0xdb657 => {
            // block db657
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db657 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db657 jmp 0xdb675
            pc = 0xdb675; continue;
        }
        0xdb659 => {
            // block db659
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db659 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db659 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // db65c cmp BYTE PTR [rbx+0x378],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x378)) as u64, 0x0u64).1;
            // db663 jne 0xdb6a3
            if fl.ne() { pc = 0xdb6a3; continue; }
            pc = 0xdb665;
        }
        0xdb665 => {
            // block db665
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db665 jmp 0xdb692
            pc = 0xdb692; continue;
        }
        0xdb667 => {
            // block db667
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db667 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db667 cmp BYTE PTR [rbx+0x378],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x378)) as u64, 0x0u64).1;
            // db66e je 0xdb675
            if fl.e() { pc = 0xdb675; continue; }
            pc = 0xdb670;
        }
        0xdb670 => {
            // block db670
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db670 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db670 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // db673 jmp 0xdb6a3
            pc = 0xdb6a3; continue;
        }
        0xdb675 => {
            // block db675
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db675 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db675 lea rdi,[rip+0x5d797] # 0x138e13
            rdi = m.env.data_va(316896);
            // db67c xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // db67f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db681 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // db684 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db689 jmp 0xdb6a3
            pc = 0xdb6a3; continue;
        }
        0xdb68b => {
            // block db68b
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db68b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db68b mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // db68e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // db690 jne 0xdb6a3
            if fl.ne() { pc = 0xdb6a3; continue; }
            pc = 0xdb692;
        }
        0xdb692 => {
            // block db692
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db692 lea rdi,[rip+0x5d797] # 0x138e30
            rdi = m.env.data_va(316925);
            // db699 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // db69b call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db6a0 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xdb6a3;
        }
        0xdb6a3 => {
            // block db6a3
            #[cfg(feature = "trace")] eprintln!("f_000db4bc pc db6a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db6a3 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // db6a6 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // db6aa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // db6ab pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // db6ad pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // db6af pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // db6b1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // db6b3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // db6b4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe171e size=592
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
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
            { let r = { let t = m.env.stub_va(0x8c5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x88f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x271); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x6cd); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x6c7); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e18a9 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // e18b0 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x823); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe18cc;
        }
        0xe18cc => {
            // block e18cc
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e18cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e18cc lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e18d3 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e18d8 mov rdi,rbx
            rdi = rbx;
            // e18db call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe18e0;
        }
        0xe18e0 => {
            // block e18e0
            #[cfg(feature = "trace")] eprintln!("f_000e171e pc e18e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e18e0 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // e18e7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1914 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // e191b call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1920 mov rsi,QWORD PTR [rip+0x5e8d9] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // e1927 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e192b lea rdi,[rbp-0x138]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec8);
            // e1932 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1937 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e193e call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0xe2466 size=642
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2466<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2466;
    loop {
        match pc {
        0xe2466 => {
            // block e2466
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2466 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2466 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2467 mov rbp,rsp
            rbp = rsp;
            // e246a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e246c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e246e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2470 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2471 sub rsp,0xa0
            { let (r, f) = sub::<64>(rsp, 0xa0u64);
              rsp = r;
              fl = f; }
            // e2478 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // e247b mov rbx,rsi
            rbx = rsi;
            // e247e mov r14,rdi
            r14 = rdi;
            // e2481 call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2486 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // e2489 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e248b je 0xe2648
            if fl.e() { pc = 0xe2648; continue; }
            pc = 0xe2491;
        }
        0xe2491 => {
            // block e2491
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2491 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2491 lea rdi,[r14+0x34]
            rdi = r14.wrapping_add(0x34);
            // e2495 test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // e2498 jne 0xe249f
            if fl.ne() { pc = 0xe249f; continue; }
            pc = 0xe249a;
        }
        0xe249a => {
            // block e249a
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e249a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e249a inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e249d jmp 0xe24a3
            pc = 0xe24a3; continue;
        }
        0xe249f => {
            // block e249f
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e249f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e249f mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xe24a3;
        }
        0xe24a3 => {
            // block e24a3
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e24a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e24a3 lea rsi,[rip+0x56fe0] # 0x13948a
            rsi = m.env.data_va(318551);
            // e24aa xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e24ad mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e24b2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e24b4 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // e24b7 mov rcx,rbx
            rcx = rbx;
            // e24ba call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e24bf mov esi,0xfa
            rsi = (0xfau64) as u32 as u64;
            // e24c4 mov rdi,r14
            rdi = r14;
            // e24c7 call 0xe435c
            { let r = { let t = m.env.code_va(0xe435c); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e24cc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e24ce je 0xe2648
            if fl.e() { pc = 0xe2648; continue; }
            pc = 0xe24d4;
        }
        0xe24d4 => {
            // block e24d4
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e24d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e24d4 xorps xmm0,xmm0
            x0 = 0;
            // e24d7 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // e24db mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // e24e3 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e24e7 mov rdi,r14
            rdi = r14;
            // e24ea call 0xe4a2e
            { let r = { let t = m.env.code_va(0xe4a2e); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e24ef xorps xmm0,xmm0
            x0 = 0;
            // e24f2 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // e24f6 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // e24fe lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e2502 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2506 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e250b lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e250f lea rdx,[rbp-0x60]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e2513 call 0xe171e
            { let r = { let t = m.env.code_va(0xe171e); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2518 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e251c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2521 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e2525 cmp QWORD PTR [rbp-0x58],rbx
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64, rbx).1;
            // e2529 je 0xe262b
            if fl.e() { pc = 0xe262b; continue; }
            pc = 0xe252f;
        }
        0xe252f => {
            // block e252f
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e252f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e252f xorps xmm0,xmm0
            x0 = 0;
            // e2532 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // e2539 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // e2541 lea rsi,[rip+0x56ecb] # 0x139413
            rsi = m.env.data_va(318432);
            // e2548 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e254f mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // e2554 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2559 lea rdx,[rbp-0xa8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e2560 lea rcx,[rbp-0x90]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2567 mov rsi,rbx
            rsi = rbx;
            // e256a call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e256f mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e2571 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e2578 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e257d test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e257f je 0xe261f
            if fl.e() { pc = 0xe261f; continue; }
            pc = 0xe2585;
        }
        0xe2585 => {
            // block e2585
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2585 lea rsi,[rip+0x56e8e] # 0x13941a
            rsi = m.env.data_va(318439);
            // e258c lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2593 call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2598 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e259a je 0xe265b
            if fl.e() { pc = 0xe265b; continue; }
            pc = 0xe25a0;
        }
        0xe25a0 => {
            // block e25a0
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e25a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e25a0 lea rsi,[rip+0x56e76] # 0x13941d
            rsi = m.env.data_va(318442);
            // e25a7 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e25ae call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e25b3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e25b5 jne 0xe261f
            if fl.ne() { pc = 0xe261f; continue; }
            pc = 0xe25b7;
        }
        0xe25b7 => {
            // block e25b7
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e25b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e25b7 mov rbx,QWORD PTR [rbp-0x60]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e25bb mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e25bf sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // e25c2 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e25c6 movabs rcx,0xaaaaaaaaaaaaaaab
            rcx = 0xaaaa_aaaa_aaaa_aaabu64;
            // e25d0 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e25d4 cmp rcx,0x4
            fl = sub::<64>(rcx, 0x4u64).1;
            // e25d8 jb 0xe261f
            if fl.b() { pc = 0xe261f; continue; }
            pc = 0xe25da;
        }
        0xe25da => {
            // block e25da
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e25da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e25da lea rsi,[rip+0x56e41] # 0x139422
            rsi = m.env.data_va(318447);
            // e25e1 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e25e8 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e25ed call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e25f2 add rbx,0x30
            { let (r, f) = add::<64>(rbx, 0x30u64);
              rbx = r;
              fl = f; }
            // e25f6 lea rcx,[r14+0x10]
            rcx = r14.wrapping_add(0x10);
            // e25fa lea rdx,[rbp-0xc0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e2601 mov rsi,rbx
            rsi = rbx;
            // e2604 call 0xe196e
            { let r = { let t = m.env.code_va(0xe196e); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2609 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e2610 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2615 mov DWORD PTR [r14+0x8],0x1a
            m.w32(r14.wrapping_add(0x8), (0x1au64) as u32);
            // e261d jmp 0xe2674
            pc = 0xe2674; continue;
        }
        0xe261f => {
            // block e261f
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e261f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e261f lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e2626 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe262b;
        }
        0xe262b => {
            // block e262b
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e262b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e262b mov DWORD PTR [r14+0x8],0x17
            m.w32(r14.wrapping_add(0x8), (0x17u64) as u32);
            // e2633 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe2636;
        }
        0xe2636 => {
            // block e2636
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2636 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2636 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e263a call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e263f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2643 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2648;
        }
        0xe2648 => {
            // block e2648
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2648 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2648 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e264b add rsp,0xa0
            { let (r, f) = add::<64>(rsp, 0xa0u64);
              rsp = r;
              fl = f; }
            // e2652 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2653 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2655 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2657 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2659 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e265a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe265b => {
            // block e265b
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e265b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e265b mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e265f sub rax,QWORD PTR [rbp-0x60]
            { let (r, f) = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64);
              rax = r;
              fl = f; }
            // e2663 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // e2666 cmp rax,0x30
            fl = sub::<64>(rax, 0x30u64).1;
            // e266a je 0xe2677
            if fl.e() { pc = 0xe2677; continue; }
            pc = 0xe266c;
        }
        0xe266c => {
            // block e266c
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e266c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e266c mov DWORD PTR [r14+0x8],0x18
            m.w32(r14.wrapping_add(0x8), (0x18u64) as u32);
            pc = 0xe2674;
        }
        0xe2674 => {
            // block e2674
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2674 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2674 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe2677;
        }
        0xe2677 => {
            // block e2677
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2677 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2677 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e267e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2683 jmp 0xe2636
            pc = 0xe2636; continue;
        }
        0xe2685 => {
            // block e2685
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e2685 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2685 mov rbx,rax
            rbx = rax;
            // e2688 jmp 0xe26c6
            pc = 0xe26c6; continue;
        }
        0xe268a => {
            // block e268a
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e268a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e268a mov rbx,rax
            rbx = rax;
            // e268d jmp 0xe26bd
            pc = 0xe26bd; continue;
        }
        0xe268f => {
            // block e268f
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e268f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e268f mov rbx,rax
            rbx = rax;
            // e2692 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e2696 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e269b jmp 0xe26bd
            pc = 0xe26bd; continue;
        }
        0xe269d => {
            // block e269d
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e269d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e269d mov rbx,rax
            rbx = rax;
            // e26a0 jmp 0xe26b1
            pc = 0xe26b1; continue;
        }
        0xe26a2 => {
            // block e26a2
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e26a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e26a2 mov rbx,rax
            rbx = rax;
            // e26a5 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e26ac call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe26b1;
        }
        0xe26b1 => {
            // block e26b1
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e26b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e26b1 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e26b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe26bd;
        }
        0xe26bd => {
            // block e26bd
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e26bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e26bd lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e26c1 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe26c6;
        }
        0xe26c6 => {
            // block e26c6
            #[cfg(feature = "trace")] eprintln!("f_000e2466 pc e26c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e26c6 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e26ca call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e26cf mov rdi,rbx
            rdi = rbx;
            // e26d2 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e26d7 mov rbx,rax
            rbx = rax;
            // e26da lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e26e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e26e6 jmp 0xe26b1
            pc = 0xe26b1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4cd0 size=592
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000e4cd0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4cd0;
    loop {
        match pc {
        0xe4cd0 => {
            // block e4cd0
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4cd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4cd0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4cd1 mov rbp,rsp
            rbp = rsp;
            // e4cd4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4cd6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4cd8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4cd9 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // e4cdd mov r11d,edx
            r11 = ((rdx as u32 as u64)) as u32 as u64;
            // e4ce0 mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // e4ce3 mov rbx,rdi
            rbx = rdi;
            // e4ce6 mov r14,QWORD PTR [rip+0x5b6eb] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // e4ced mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // e4cf0 mov QWORD PTR [rbp-0x20],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (r14) as u64);
            // e4cf4 test r9d,r9d
            fl = logic::<32>((r9 as u32 as u64) & (r9 as u32 as u64));
            // e4cf7 je 0xe4cff
            if fl.e() { pc = 0xe4cff; continue; }
            pc = 0xe4cf9;
        }
        0xe4cf9 => {
            // block e4cf9
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4cf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4cf9 mov DWORD PTR [rbx+0x8],r9d
            m.w32(rbx.wrapping_add(0x8), ((r9 as u32 as u64)) as u32);
            // e4cfd jmp 0xe4d03
            pc = 0xe4d03; continue;
        }
        0xe4cff => {
            // block e4cff
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4cff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4cff mov r9d,DWORD PTR [rbx+0x8]
            r9 = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe4d03;
        }
        0xe4d03 => {
            // block e4d03
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d03 cmp r9d,0x4
            fl = sub::<32>((r9 as u32 as u64), 0x4u64).1;
            // e4d07 je 0xe4d0f
            if fl.e() { pc = 0xe4d0f; continue; }
            pc = 0xe4d09;
        }
        0xe4d09 => {
            // block e4d09
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d09 cmp r9d,0x8
            fl = sub::<32>((r9 as u32 as u64), 0x8u64).1;
            // e4d0d jne 0xe4d14
            if fl.ne() { pc = 0xe4d14; continue; }
            pc = 0xe4d0f;
        }
        0xe4d0f => {
            // block e4d0f
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d0f mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // e4d12 jmp 0xe4d17
            pc = 0xe4d17; continue;
        }
        0xe4d14 => {
            // block e4d14
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d14 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4d17;
        }
        0xe4d17 => {
            // block e4d17
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d17 lea eax,[r9-0x1]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // e4d1b mov esi,0x1e
            rsi = (0x1eu64) as u32 as u64;
            // e4d20 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e4d23 ja 0xe4d31
            if fl.a() { pc = 0xe4d31; continue; }
            pc = 0xe4d25;
        }
        0xe4d25 => {
            // block e4d25
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d25 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e4d27 lea rdx,[rip+0x44fb2] # 0x129ce0
            rdx = m.env.data_va(255149);
            // e4d2e mov esi,DWORD PTR [rdx+rax*4]
            rsi = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe4d31;
        }
        0xe4d31 => {
            // block e4d31
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d31 cmp esi,r10d
            fl = sub::<32>((rsi as u32 as u64), (r10 as u32 as u64)).1;
            // e4d34 ja 0xe4d43
            if fl.a() { pc = 0xe4d43; continue; }
            pc = 0xe4d36;
        }
        0xe4d36 => {
            // block e4d36
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d36 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e4d38 mov eax,r10d
            rax = ((r10 as u32 as u64)) as u32 as u64;
            // e4d3b div esi
            { let (q, r) = div::<32>(rdx, rax, (rsi as u32 as u64)); rax = q; rdx = r; }
            // e4d3d add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // e4d40 mov r10d,edx
            r10 = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0xe4d43;
        }
        0xe4d43 => {
            // block e4d43
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d43 cmp r11d,0x3c
            fl = sub::<32>((r11 as u32 as u64), 0x3cu64).1;
            // e4d47 jb 0xe4d61
            if fl.b() { pc = 0xe4d61; continue; }
            pc = 0xe4d49;
        }
        0xe4d49 => {
            // block e4d49
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d49 mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // e4d4c mov edx,0x88888889
            rdx = (0x88888889u64) as u32 as u64;
            // e4d51 imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e4d55 shr rdx,0x25
            { let (r, f) = shr::<64>(rdx, 0x25u64, fl); rdx = r; fl = f; }
            // e4d59 add ecx,edx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rdx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4d5b imul eax,edx,0x3c
            { let (r, f) = imul::<32>((rdx as u32 as u64), 0x3cu64); rax = (r) as u32 as u64; fl = f; }
            // e4d5e sub r11d,eax
            { let (r, f) = sub::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4d61;
        }
        0xe4d61 => {
            // block e4d61
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d61 cmp ecx,0x3c
            fl = sub::<32>((rcx as u32 as u64), 0x3cu64).1;
            // e4d64 jb 0xe4d7d
            if fl.b() { pc = 0xe4d7d; continue; }
            pc = 0xe4d66;
        }
        0xe4d66 => {
            // block e4d66
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d66 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e4d68 mov edx,0x88888889
            rdx = (0x88888889u64) as u32 as u64;
            // e4d6d imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e4d71 shr rdx,0x25
            { let (r, f) = shr::<64>(rdx, 0x25u64, fl); rdx = r; fl = f; }
            // e4d75 add r8d,edx
            { let (r, f) = add::<32>((r8 as u32 as u64), (rdx as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4d78 imul eax,edx,0x3c
            { let (r, f) = imul::<32>((rdx as u32 as u64), 0x3cu64); rax = (r) as u32 as u64; fl = f; }
            // e4d7b sub ecx,eax
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4d7d;
        }
        0xe4d7d => {
            // block e4d7d
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d7d cmp r8d,0x18
            fl = sub::<32>((r8 as u32 as u64), 0x18u64).1;
            // e4d81 jb 0xe4d9c
            if fl.b() { pc = 0xe4d9c; continue; }
            pc = 0xe4d83;
        }
        0xe4d83 => {
            // block e4d83
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d83 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // e4d86 mov edx,0xaaaaaaab
            rdx = (0xaaaaaaabu64) as u32 as u64;
            // e4d8b imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e4d8f shr rdx,0x24
            { let (r, f) = shr::<64>(rdx, 0x24u64, fl); rdx = r; fl = f; }
            // e4d93 shl edx,0x3
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x3u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e4d96 lea eax,[rdx+rdx*2]
            rax = (rdx.wrapping_add(rdx.wrapping_mul(2))) as u32 as u64;
            // e4d99 sub r8d,eax
            { let (r, f) = sub::<32>((r8 as u32 as u64), (rax as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4d9c;
        }
        0xe4d9c => {
            // block e4d9c
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4d9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d9c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // e4d9f je 0xe4dfc
            if fl.e() { pc = 0xe4dfc; continue; }
            pc = 0xe4da1;
        }
        0xe4da1 => {
            // block e4da1
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4da1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4da1 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e4da3 mov edx,0xcccccccd
            rdx = (0xcccccccdu64) as u32 as u64;
            // e4da8 imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // e4dac shr rdx,0x23
            { let (r, f) = shr::<64>(rdx, 0x23u64, fl); rdx = r; fl = f; }
            // e4db0 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e4db2 cmp r9d,0x4
            fl = sub::<32>((r9 as u32 as u64), 0x4u64).1;
            // e4db6 jne 0xe4dd7
            if fl.ne() { pc = 0xe4dd7; continue; }
            pc = 0xe4db8;
        }
        0xe4db8 => {
            // block e4db8
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4db8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4db8 lea eax,[rdx+rdx*4]
            rax = (rdx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // e4dbb cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // e4dbd mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // e4dc2 cmove eax,r10d
            if fl.e() { rax = ((r10 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e4dc6 test r11d,r11d
            fl = logic::<32>((r11 as u32 as u64) & (r11 as u32 as u64));
            // e4dc9 cmovne eax,r10d
            if fl.ne() { rax = ((r10 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e4dcd cmp r10d,0x1
            fl = sub::<32>((r10 as u32 as u64), 0x1u64).1;
            // e4dd1 cmovbe r10d,eax
            if fl.be() { r10 = ((rax as u32 as u64)) as u32 as u64; } else { r10 = r10 as u32 as u64; }
            // e4dd5 jmp 0xe4dfc
            pc = 0xe4dfc; continue;
        }
        0xe4dd7 => {
            // block e4dd7
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4dd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4dd7 cmp r9d,0x8
            fl = sub::<32>((r9 as u32 as u64), 0x8u64).1;
            // e4ddb mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            // e4de0 cmovne eax,r10d
            if fl.ne() { rax = ((r10 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e4de4 test r11d,r11d
            fl = logic::<32>((r11 as u32 as u64) & (r11 as u32 as u64));
            // e4de7 cmovne eax,r10d
            if fl.ne() { rax = ((r10 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e4deb cmp r10d,0x3
            fl = sub::<32>((r10 as u32 as u64), 0x3u64).1;
            // e4def lea edx,[rdx+rdx*4]
            rdx = (rdx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // e4df2 cmova eax,r10d
            if fl.a() { rax = ((r10 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e4df6 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // e4df8 cmovne r10d,eax
            if fl.ne() { r10 = ((rax as u32 as u64)) as u32 as u64; } else { r10 = r10 as u32 as u64; }
            pc = 0xe4dfc;
        }
        0xe4dfc => {
            // block e4dfc
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4dfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4dfc mov DWORD PTR [rbx+0x50],r10d
            m.w32(rbx.wrapping_add(0x50), ((r10 as u32 as u64)) as u32);
            // e4e00 mov DWORD PTR [rbx+0x54],r11d
            m.w32(rbx.wrapping_add(0x54), ((r11 as u32 as u64)) as u32);
            // e4e04 mov DWORD PTR [rbx+0x58],ecx
            m.w32(rbx.wrapping_add(0x58), ((rcx as u32 as u64)) as u32);
            // e4e07 mov DWORD PTR [rbx+0x5c],r8d
            m.w32(rbx.wrapping_add(0x5c), ((r8 as u32 as u64)) as u32);
            // e4e0b test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // e4e0e je 0xe4e19
            if fl.e() { pc = 0xe4e19; continue; }
            pc = 0xe4e10;
        }
        0xe4e10 => {
            // block e4e10
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4e10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4e10 lea rsi,[rip+0x546c0] # 0x1394d7
            rsi = m.env.data_va(318628);
            // e4e17 jmp 0xe4e20
            pc = 0xe4e20; continue;
        }
        0xe4e19 => {
            // block e4e19
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4e19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4e19 lea rsi,[rip+0x546cb] # 0x1394eb
            rsi = m.env.data_va(318648);
            pc = 0xe4e20;
        }
        0xe4e20 => {
            // block e4e20
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4e20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4e20 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // e4e24 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4e26 mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            // e4e29 mov r8d,r11d
            r8 = ((r11 as u32 as u64)) as u32 as u64;
            // e4e2c mov r9d,r10d
            r9 = ((r10 as u32 as u64)) as u32 as u64;
            // e4e2f call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4e34 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // e4e38 lea rsi,[rbp-0x70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // e4e3c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4e41 mov rdi,rbx
            rdi = rbx;
            // e4e44 call 0xe5ab6
            { let r = { let t = m.env.code_va(0xe5ab6); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4e49 mov WORD PTR [rbx+0xc],0x1
            m.w16(rbx.wrapping_add(0xc), (0x1u64) as u16);
            // e4e4f mov BYTE PTR [rbx+0xe],0x0
            m.w8(rbx.wrapping_add(0xe), (0x0u64) as u8);
            // e4e53 mov eax,0xfffffbcf
            rax = (0xfffffbcfu64) as u32 as u64;
            // e4e58 and eax,DWORD PTR [rbx+0x84]
            { let r = (rax as u32 as u64) & m.r32(rbx.wrapping_add(0x84)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4e5e mov BYTE PTR [rbx+0xf],0x0
            m.w8(rbx.wrapping_add(0xf), (0x0u64) as u8);
            // e4e62 mov BYTE PTR [rbx+0x14],r15b
            m.w8(rbx.wrapping_add(0x14), ((r15 as u8 as u64)) as u8);
            // e4e66 lea edx,[rax+0x400]
            rdx = (rax.wrapping_add(0x400)) as u32 as u64;
            // e4e6c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // e4e6f cmove edx,eax
            if fl.e() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // e4e72 mov BYTE PTR [rbx+0x15],0x0
            m.w8(rbx.wrapping_add(0x15), (0x0u64) as u8);
            // e4e76 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e4e78 and eax,0xfffff7cf
            { let r = (rax as u32 as u64) & 0xfffff7cfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4e7d mov DWORD PTR [rbx+0x84],eax
            m.w32(rbx.wrapping_add(0x84), ((rax as u32 as u64)) as u32);
            // e4e83 mov eax,DWORD PTR [rbx+0x8]
            rax = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // e4e86 lea esi,[rax-0x1]
            rsi = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // e4e89 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4e8b cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // e4e8e ja 0xe4eb1
            if fl.a() { pc = 0xe4eb1; continue; }
            pc = 0xe4e90;
        }
        0xe4e90 => {
            // block e4e90
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4e90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4e90 movsxd rsi,esi
            rsi = sx::<32>((rsi as u32 as u64)) as u64;
            // e4e93 lea rdi,[rip+0x44ea6] # 0x129d40
            rdi = m.env.data_va(255245);
            // e4e9a mov esi,DWORD PTR [rdi+rsi*4]
            rsi = (m.r32(rdi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // e4e9d dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e4e9f cmp esi,0xd
            fl = sub::<32>((rsi as u32 as u64), 0xdu64).1;
            // e4ea2 ja 0xe4eb1
            if fl.a() { pc = 0xe4eb1; continue; }
            pc = 0xe4ea4;
        }
        0xe4ea4 => {
            // block e4ea4
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4ea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4ea4 movsxd rcx,esi
            rcx = sx::<32>((rsi as u32 as u64)) as u64;
            // e4ea7 lea rsi,[rip+0x44e52] # 0x129d00
            rsi = m.env.data_va(255181);
            // e4eae mov ecx,DWORD PTR [rsi+rcx*4]
            rcx = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe4eb1;
        }
        0xe4eb1 => {
            // block e4eb1
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4eb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4eb1 and edx,0xf0ff7cf
            { let r = (rdx as u32 as u64) & 0xf0ff7cfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e4eb7 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // e4eb9 and esi,0xd
            { let r = (rsi as u32 as u64) & 0xdu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e4ebc shl esi,0x14
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x14u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e4ebf or esi,edx
            { let r = (rsi as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e4ec1 and ecx,0x70
            { let r = (rcx as u32 as u64) & 0x70u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4ec4 shl ecx,0x18
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x18u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e4ec7 or ecx,esi
            { let r = (rcx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4ec9 mov DWORD PTR [rbx+0x84],ecx
            m.w32(rbx.wrapping_add(0x84), ((rcx as u32 as u64)) as u32);
            // e4ecf lea edx,[rax-0x5]
            rdx = (rax.wrapping_add(0xffff_ffff_ffff_fffb)) as u32 as u64;
            // e4ed2 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // e4ed5 jb 0xe4efb
            if fl.b() { pc = 0xe4efb; continue; }
            pc = 0xe4ed7;
        }
        0xe4ed7 => {
            // block e4ed7
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4ed7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4ed7 mov DWORD PTR [rbx+0x10],0x0
            m.w32(rbx.wrapping_add(0x10), (0x0u64) as u32);
            // e4ede and eax,0xfffffffb
            { let r = (rax as u32 as u64) & 0xfffffffbu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4ee1 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // e4ee4 jne 0xe4eef
            if fl.ne() { pc = 0xe4eef; continue; }
            pc = 0xe4ee6;
        }
        0xe4ee6 => {
            // block e4ee6
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4ee6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4ee6 and BYTE PTR [rbx+0x8b],0xf7
            { let r = m.r8(rbx.wrapping_add(0x8b)) as u64 & 0xf7u64; let f = logic::<8>(r);
              m.w8(rbx.wrapping_add(0x8b), (r) as u8);
              fl = f; }
            // e4eed jmp 0xe4efb
            pc = 0xe4efb; continue;
        }
        0xe4eef => {
            // block e4eef
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4eef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4eef and ecx,0x77dff7cf
            { let r = (rcx as u32 as u64) & 0x77dff7cfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4ef5 mov DWORD PTR [rbx+0x84],ecx
            m.w32(rbx.wrapping_add(0x84), ((rcx as u32 as u64)) as u32);
            pc = 0xe4efb;
        }
        0xe4efb => {
            // block e4efb
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4efb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4efb xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e4efd xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e4eff xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4f01 mov rdi,rbx
            rdi = rbx;
            // e4f04 call 0xe5efc
            { let r = { let t = m.env.code_va(0xe5efc); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4f09 cmp r14,QWORD PTR [rbp-0x20]
            fl = sub::<64>(r14, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // e4f0d jne 0xe4f1a
            if fl.ne() { pc = 0xe4f1a; continue; }
            pc = 0xe4f0f;
        }
        0xe4f0f => {
            // block e4f0f
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4f0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4f0f add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // e4f13 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4f14 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4f16 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4f18 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4f19 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe4f1a => {
            // block e4f1a
            #[cfg(feature = "trace")] eprintln!("f_000e4cd0 pc e4f1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4f1a call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe850c size=492
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e850c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe850c;
    loop {
        match pc {
        0xe850c => {
            // block e850c
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e850c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e850c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e850d mov rbp,rsp
            rbp = rsp;
            // e8510 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e8512 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e8514 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e8516 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e8518 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e8519 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e851a mov r12,rsi
            r12 = rsi;
            // e851d mov r14,rdi
            r14 = rdi;
            // e8520 lea rsi,[r12+0x2a8]
            rsi = r12.wrapping_add(0x2a8);
            // e8528 movzx r15d,BYTE PTR [r12+0x2a8]
            r15 = (m.r8(r12.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // e8531 mov r9b,r15b
            r9 = (r9 & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // e8534 and r9b,0x1
            { let r = (r9 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r9 = (r9 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e8538 jne 0xe8546
            if fl.ne() { pc = 0xe8546; continue; }
            pc = 0xe853a;
        }
        0xe853a => {
            // block e853a
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e853a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e853a lea r11,[rsi+0x1]
            r11 = rsi.wrapping_add(0x1);
            // e853e mov rcx,r15
            rcx = r15;
            // e8541 shr rcx,1
            { let (r, f) = shr::<64>(rcx, 0x1u64, fl); rcx = r; fl = f; }
            // e8544 jmp 0xe8556
            pc = 0xe8556; continue;
        }
        0xe8546 => {
            // block e8546
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8546 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8546 mov rcx,QWORD PTR [r12+0x2b0]
            rcx = m.r64(r12.wrapping_add(0x2b0)) as u64;
            // e854e mov r11,QWORD PTR [r12+0x2b8]
            r11 = m.r64(r12.wrapping_add(0x2b8)) as u64;
            pc = 0xe8556;
        }
        0xe8556 => {
            // block e8556
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8556 mov rdx,0xffffffffffffffff
            rdx = 0xffff_ffff_ffff_ffffu64;
            // e855d cmp rcx,0xe
            fl = sub::<64>(rcx, 0xeu64).1;
            // e8561 jb 0xe85bd
            if fl.b() { pc = 0xe85bd; continue; }
            pc = 0xe8563;
        }
        0xe8563 => {
            // block e8563
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8563 cmp rcx,0xe
            fl = sub::<64>(rcx, 0xeu64).1;
            // e8567 jl 0xe85bd
            if fl.l() { pc = 0xe85bd; continue; }
            pc = 0xe8569;
        }
        0xe8569 => {
            // block e8569
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8569 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8569 mov rax,rcx
            rax = rcx;
            // e856c add rax,0xfffffffffffffff3
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fff3u64);
              rax = r;
              fl = f; }
            // e8570 je 0xe85bd
            if fl.e() { pc = 0xe85bd; continue; }
            pc = 0xe8572;
        }
        0xe8572 => {
            // block e8572
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8572 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8572 lea r8,[r11+rcx*1]
            r8 = r11.wrapping_add(rcx);
            // e8576 lea r10,[r11+rcx*1-0xd]
            r10 = r11.wrapping_add(rcx).wrapping_add(0xffff_ffff_ffff_fff3);
            // e857b lea r13,[rip+0x54cb8] # 0x13d23a
            r13 = m.env.data_va(334343);
            // e8582 mov rcx,r11
            rcx = r11;
            pc = 0xe8585;
        }
        0xe8585 => {
            // block e8585
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8585 movzx eax,BYTE PTR [rcx]
            rax = (m.r8(rcx) as u64) as u32 as u64;
            // e8588 cmp eax,0x50
            fl = sub::<32>((rax as u32 as u64), 0x50u64).1;
            // e858b mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // e8590 jne 0xe85a8
            if fl.ne() { pc = 0xe85a8; continue; }
            pc = 0xe8592;
        }
        0xe8592 => {
            // block e8592
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8592 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8592 cmp rdi,0xe
            fl = sub::<64>(rdi, 0xeu64).1;
            // e8596 je 0xe85b2
            if fl.e() { pc = 0xe85b2; continue; }
            pc = 0xe8598;
        }
        0xe8598 => {
            // block e8598
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8598 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8598 movzx eax,BYTE PTR [rdi+r13*1]
            rax = (m.r8(rdi.wrapping_add(r13)) as u64) as u32 as u64;
            // e859d movzx ebx,BYTE PTR [rcx+rdi*1]
            rbx = (m.r8(rcx.wrapping_add(rdi)) as u64) as u32 as u64;
            // e85a1 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e85a4 cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // e85a6 je 0xe8592
            if fl.e() { pc = 0xe8592; continue; }
            pc = 0xe85a8;
        }
        0xe85a8 => {
            // block e85a8
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85a8 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // e85ab cmp rcx,r10
            fl = sub::<64>(rcx, r10).1;
            // e85ae jne 0xe8585
            if fl.ne() { pc = 0xe8585; continue; }
            pc = 0xe85b0;
        }
        0xe85b0 => {
            // block e85b0
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85b0 jmp 0xe85bd
            pc = 0xe85bd; continue;
        }
        0xe85b2 => {
            // block e85b2
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85b2 cmp rcx,r8
            fl = sub::<64>(rcx, r8).1;
            // e85b5 je 0xe85bd
            if fl.e() { pc = 0xe85bd; continue; }
            pc = 0xe85b7;
        }
        0xe85b7 => {
            // block e85b7
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85b7 sub rcx,r11
            { let (r, f) = sub::<64>(rcx, r11);
              rcx = r;
              fl = f; }
            // e85ba mov rdx,rcx
            rdx = rcx;
            pc = 0xe85bd;
        }
        0xe85bd => {
            // block e85bd
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85bd test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // e85c0 jne 0xe85ce
            if fl.ne() { pc = 0xe85ce; continue; }
            pc = 0xe85c2;
        }
        0xe85c2 => {
            // block e85c2
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85c2 lea r8,[rsi+0x1]
            r8 = rsi.wrapping_add(0x1);
            // e85c6 mov rdi,r15
            rdi = r15;
            // e85c9 shr rdi,1
            { let (r, f) = shr::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // e85cc jmp 0xe85de
            pc = 0xe85de; continue;
        }
        0xe85ce => {
            // block e85ce
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85ce mov rdi,QWORD PTR [r12+0x2b0]
            rdi = m.r64(r12.wrapping_add(0x2b0)) as u64;
            // e85d6 mov r8,QWORD PTR [r12+0x2b8]
            r8 = m.r64(r12.wrapping_add(0x2b8)) as u64;
            pc = 0xe85de;
        }
        0xe85de => {
            // block e85de
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85de mov rcx,0xfffffffffffffffe
            rcx = 0xffff_ffff_ffff_fffeu64;
            // e85e5 cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e85e9 jb 0xe864f
            if fl.b() { pc = 0xe864f; continue; }
            pc = 0xe85eb;
        }
        0xe85eb => {
            // block e85eb
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85eb cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e85ef jl 0xe864f
            if fl.l() { pc = 0xe864f; continue; }
            pc = 0xe85f1;
        }
        0xe85f1 => {
            // block e85f1
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85f1 mov rax,rdi
            rax = rdi;
            // e85f4 add rax,0xfffffffffffffffd
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fffdu64);
              rax = r;
              fl = f; }
            // e85f8 je 0xe864f
            if fl.e() { pc = 0xe864f; continue; }
            pc = 0xe85fa;
        }
        0xe85fa => {
            // block e85fa
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e85fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e85fa lea rax,[r8+rdi*1]
            rax = r8.wrapping_add(rdi);
            // e85fe mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // e8602 lea r11,[r8+rdi*1-0x3]
            r11 = r8.wrapping_add(rdi).wrapping_add(0xffff_ffff_ffff_fffd);
            // e8607 lea r13,[rip+0x54c3b] # 0x13d249
            r13 = m.env.data_va(334358);
            // e860e mov rbx,r8
            rbx = r8;
            pc = 0xe8611;
        }
        0xe8611 => {
            // block e8611
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8611 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8611 movzx eax,BYTE PTR [rbx]
            rax = (m.r8(rbx) as u64) as u32 as u64;
            // e8614 cmp eax,0x44
            fl = sub::<32>((rax as u32 as u64), 0x44u64).1;
            // e8617 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e861c jne 0xe8636
            if fl.ne() { pc = 0xe8636; continue; }
            pc = 0xe861e;
        }
        0xe861e => {
            // block e861e
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e861e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e861e cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // e8622 je 0xe8640
            if fl.e() { pc = 0xe8640; continue; }
            pc = 0xe8624;
        }
        0xe8624 => {
            // block e8624
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8624 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8624 movzx edi,BYTE PTR [rax+r13*1]
            rdi = (m.r8(rax.wrapping_add(r13)) as u64) as u32 as u64;
            // e8629 movzx r10d,BYTE PTR [rbx+rax*1]
            r10 = (m.r8(rbx.wrapping_add(rax)) as u64) as u32 as u64;
            // e862e inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e8631 cmp r10d,edi
            fl = sub::<32>((r10 as u32 as u64), (rdi as u32 as u64)).1;
            // e8634 je 0xe861e
            if fl.e() { pc = 0xe861e; continue; }
            pc = 0xe8636;
        }
        0xe8636 => {
            // block e8636
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8636 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8636 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e8639 cmp rbx,r11
            fl = sub::<64>(rbx, r11).1;
            // e863c jne 0xe8611
            if fl.ne() { pc = 0xe8611; continue; }
            pc = 0xe863e;
        }
        0xe863e => {
            // block e863e
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e863e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e863e jmp 0xe864f
            pc = 0xe864f; continue;
        }
        0xe8640 => {
            // block e8640
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8640 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8640 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // e8644 je 0xe864f
            if fl.e() { pc = 0xe864f; continue; }
            pc = 0xe8646;
        }
        0xe8646 => {
            // block e8646
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8646 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8646 sub rbx,r8
            { let (r, f) = sub::<64>(rbx, r8);
              rbx = r;
              fl = f; }
            // e8649 dec rbx
            { let (r, f) = dec::<64>(rbx, fl); rbx = r; fl = f; }
            // e864c mov rcx,rbx
            rcx = rbx;
            pc = 0xe864f;
        }
        0xe864f => {
            // block e864f
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e864f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e864f test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // e8652 jne 0xe865d
            if fl.ne() { pc = 0xe865d; continue; }
            pc = 0xe8654;
        }
        0xe8654 => {
            // block e8654
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8654 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8654 lea r10,[rsi+0x1]
            r10 = rsi.wrapping_add(0x1);
            // e8658 shr r15,1
            { let (r, f) = shr::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // e865b jmp 0xe866d
            pc = 0xe866d; continue;
        }
        0xe865d => {
            // block e865d
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e865d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e865d mov r15,QWORD PTR [r12+0x2b0]
            r15 = m.r64(r12.wrapping_add(0x2b0)) as u64;
            // e8665 mov r10,QWORD PTR [r12+0x2b8]
            r10 = m.r64(r12.wrapping_add(0x2b8)) as u64;
            pc = 0xe866d;
        }
        0xe866d => {
            // block e866d
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e866d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e866d mov r11,0xffffffffffffffff
            r11 = 0xffff_ffff_ffff_ffffu64;
            // e8674 cmp r15,0xe
            fl = sub::<64>(r15, 0xeu64).1;
            // e8678 jb 0xe86d7
            if fl.b() { pc = 0xe86d7; continue; }
            pc = 0xe867a;
        }
        0xe867a => {
            // block e867a
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e867a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e867a cmp r15,0xe
            fl = sub::<64>(r15, 0xeu64).1;
            // e867e jl 0xe86d7
            if fl.l() { pc = 0xe86d7; continue; }
            pc = 0xe8680;
        }
        0xe8680 => {
            // block e8680
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8680 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8680 mov rax,r15
            rax = r15;
            // e8683 add rax,0xfffffffffffffff3
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fff3u64);
              rax = r;
              fl = f; }
            // e8687 je 0xe86d7
            if fl.e() { pc = 0xe86d7; continue; }
            pc = 0xe8689;
        }
        0xe8689 => {
            // block e8689
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e8689 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8689 lea r8,[r10+r15*1]
            r8 = r10.wrapping_add(r15);
            // e868d lea r9,[r10+r15*1-0xd]
            r9 = r10.wrapping_add(r15).wrapping_add(0xffff_ffff_ffff_fff3);
            // e8692 lea r15,[rip+0x54ba1] # 0x13d23a
            r15 = m.env.data_va(334343);
            // e8699 mov r12,r10
            r12 = r10;
            pc = 0xe869c;
        }
        0xe869c => {
            // block e869c
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e869c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e869c movzx eax,BYTE PTR [r12]
            rax = (m.r8(r12) as u64) as u32 as u64;
            // e86a1 cmp eax,0x50
            fl = sub::<32>((rax as u32 as u64), 0x50u64).1;
            // e86a4 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e86a9 jne 0xe86c2
            if fl.ne() { pc = 0xe86c2; continue; }
            pc = 0xe86ab;
        }
        0xe86ab => {
            // block e86ab
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86ab cmp rax,0xe
            fl = sub::<64>(rax, 0xeu64).1;
            // e86af je 0xe86cc
            if fl.e() { pc = 0xe86cc; continue; }
            pc = 0xe86b1;
        }
        0xe86b1 => {
            // block e86b1
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86b1 movzx ebx,BYTE PTR [rax+r15*1]
            rbx = (m.r8(rax.wrapping_add(r15)) as u64) as u32 as u64;
            // e86b6 movzx edi,BYTE PTR [r12+rax*1]
            rdi = (m.r8(r12.wrapping_add(rax)) as u64) as u32 as u64;
            // e86bb inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e86be cmp edi,ebx
            fl = sub::<32>((rdi as u32 as u64), (rbx as u32 as u64)).1;
            // e86c0 je 0xe86ab
            if fl.e() { pc = 0xe86ab; continue; }
            pc = 0xe86c2;
        }
        0xe86c2 => {
            // block e86c2
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86c2 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // e86c5 cmp r12,r9
            fl = sub::<64>(r12, r9).1;
            // e86c8 jne 0xe869c
            if fl.ne() { pc = 0xe869c; continue; }
            pc = 0xe86ca;
        }
        0xe86ca => {
            // block e86ca
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86ca jmp 0xe86d7
            pc = 0xe86d7; continue;
        }
        0xe86cc => {
            // block e86cc
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86cc cmp r12,r8
            fl = sub::<64>(r12, r8).1;
            // e86cf je 0xe86d7
            if fl.e() { pc = 0xe86d7; continue; }
            pc = 0xe86d1;
        }
        0xe86d1 => {
            // block e86d1
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86d1 sub r12,r10
            { let (r, f) = sub::<64>(r12, r10);
              r12 = r;
              fl = f; }
            // e86d4 mov r11,r12
            r11 = r12;
            pc = 0xe86d7;
        }
        0xe86d7 => {
            // block e86d7
            #[cfg(feature = "trace")] eprintln!("f_000e850c pc e86d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86d7 sub rcx,r11
            { let (r, f) = sub::<64>(rcx, r11);
              rcx = r;
              fl = f; }
            // e86da mov rdi,r14
            rdi = r14;
            // e86dd mov r8,rsi
            r8 = rsi;
            // e86e0 call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e86e5 mov rax,r14
            rax = r14;
            // e86e8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e86ec pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e86ed pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e86ef pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e86f1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e86f3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e86f5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e86f6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9b24 size=604
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949, stub:0x961
pub fn f_000e9b24<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9b24;
    loop {
        match pc {
        0xe9b24 => {
            // block e9b24
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9b24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b24 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9b25 mov rbp,rsp
            rbp = rsp;
            // e9b28 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9b2a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9b2c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e9b2e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e9b30 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9b31 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9b38 mov r15,rdi
            r15 = rdi;
            // e9b3b mov rbx,QWORD PTR [rip+0x56896] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e9b42 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e9b45 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // e9b49 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e9b4c cmp QWORD PTR [r15+0x78],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x78)) as u64, 0x0u64).1;
            // e9b51 je 0xe9d35
            if fl.e() { pc = 0xe9d35; continue; }
            pc = 0xe9b57;
        }
        0xe9b57 => {
            // block e9b57
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9b57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b57 mov rax,QWORD PTR [r15+0x80]
            rax = m.r64(r15.wrapping_add(0x80)) as u64;
            // e9b5e test rax,rax
            fl = logic::<64>(rax & rax);
            // e9b61 je 0xe9d55
            if fl.e() { pc = 0xe9d55; continue; }
            pc = 0xe9b67;
        }
        0xe9b67 => {
            // block e9b67
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9b67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b67 mov ecx,DWORD PTR [r15+0x18c]
            rcx = (m.r32(r15.wrapping_add(0x18c)) as u64) as u32 as u64;
            // e9b6e test cl,0x10
            fl = logic::<8>((rcx as u8 as u64) & 0x10u64);
            // e9b71 jne 0xe9bad
            if fl.ne() { pc = 0xe9bad; continue; }
            pc = 0xe9b73;
        }
        0xe9b73 => {
            // block e9b73
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9b73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b73 test cl,0x8
            fl = logic::<8>((rcx as u8 as u64) & 0x8u64);
            // e9b76 je 0xe9d32
            if fl.e() { pc = 0xe9d32; continue; }
            pc = 0xe9b7c;
        }
        0xe9b7c => {
            // block e9b7c
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9b7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9b7c lea rsi,[r15+0x108]
            rsi = r15.wrapping_add(0x108);
            // e9b83 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e9b8a mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // e9b8f rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            // e9b92 cmp BYTE PTR [r15+0x192],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x192)) as u64, 0x0u64).1;
            // e9b9a je 0xe9c42
            if fl.e() { pc = 0xe9c42; continue; }
            pc = 0xe9ba0;
        }
        0xe9ba0 => {
            // block e9ba0
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9ba0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ba0 mov r14,QWORD PTR [r15+0x20]
            r14 = m.r64(r15.wrapping_add(0x20)) as u64;
            // e9ba4 sub r14,QWORD PTR [r15+0x18]
            { let (r, f) = sub::<64>(r14, m.r64(r15.wrapping_add(0x18)) as u64);
              r14 = r;
              fl = f; }
            // e9ba8 jmp 0xe9cc5
            pc = 0xe9cc5; continue;
        }
        0xe9bad => {
            // block e9bad
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9bad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9bad mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // e9bb1 cmp rax,QWORD PTR [r15+0x28]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x28)) as u64).1;
            // e9bb5 je 0xe9bd4
            if fl.e() { pc = 0xe9bd4; continue; }
            pc = 0xe9bb7;
        }
        0xe9bb7 => {
            // block e9bb7
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9bb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9bb7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e9bba mov r13d,0xffffffff
            r13 = (0xffffffffu64) as u32 as u64;
            // e9bc0 mov esi,0xffffffff
            rsi = (0xffffffffu64) as u32 as u64;
            // e9bc5 mov rdi,r15
            rdi = r15;
            // e9bc8 call QWORD PTR [rax+0x68]
            { let r = { let t = m.r64(rax.wrapping_add(0x68)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9bcb cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // e9bce je 0xe9d35
            if fl.e() { pc = 0xe9d35; continue; }
            pc = 0xe9bd4;
        }
        0xe9bd4 => {
            // block e9bd4
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9bd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9bd4 lea r14,[r15+0x88]
            r14 = r15.wrapping_add(0x88);
            // e9bdb mov r13d,0xffffffff
            r13 = (0xffffffffu64) as u32 as u64;
            pc = 0xe9be1;
        }
        0xe9be1 => {
            // block e9be1
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9be1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9be1 mov rdx,QWORD PTR [r15+0x40]
            rdx = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9be5 mov rdi,QWORD PTR [r15+0x80]
            rdi = m.r64(r15.wrapping_add(0x80)) as u64;
            // e9bec mov rcx,QWORD PTR [r15+0x60]
            rcx = m.r64(r15.wrapping_add(0x60)) as u64;
            // e9bf0 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // e9bf3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e9bf6 mov rsi,r14
            rsi = r14;
            // e9bf9 lea r8,[rbp-0xb8]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // e9c00 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c03 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e9c05 mov r12,QWORD PTR [rbp-0xb8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // e9c0c mov rdi,QWORD PTR [r15+0x40]
            rdi = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9c10 mov rcx,QWORD PTR [r15+0x78]
            rcx = m.r64(r15.wrapping_add(0x78)) as u64;
            // e9c14 sub r12,rdi
            { let (r, f) = sub::<64>(r12, rdi);
              r12 = r;
              fl = f; }
            // e9c17 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e9c1c mov rdx,r12
            rdx = r12;
            // e9c1f call 0xec1e8
            { let r = { let t = m.env.stub_va(0x9b5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c24 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // e9c27 jne 0xe9c33
            if fl.ne() { pc = 0xe9c33; continue; }
            pc = 0xe9c29;
        }
        0xe9c29 => {
            // block e9c29
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c29 cmp ebx,0x1
            fl = sub::<32>((rbx as u32 as u64), 0x1u64).1;
            // e9c2c je 0xe9be1
            if fl.e() { pc = 0xe9be1; continue; }
            pc = 0xe9c2e;
        }
        0xe9c2e => {
            // block e9c2e
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c2e cmp ebx,0x2
            fl = sub::<32>((rbx as u32 as u64), 0x2u64).1;
            // e9c31 jne 0xe9c6d
            if fl.ne() { pc = 0xe9c6d; continue; }
            pc = 0xe9c33;
        }
        0xe9c33 => {
            // block e9c33
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c33 mov rbx,QWORD PTR [rip+0x5679e] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e9c3a mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e9c3d jmp 0xe9d35
            pc = 0xe9d35; continue;
        }
        0xe9c42 => {
            // block e9c42
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c42 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e9c45 mov rdi,rax
            rdi = rax;
            // e9c48 call QWORD PTR [rcx+0x30]
            { let r = { let t = m.r64(rcx.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c4b mov rcx,QWORD PTR [r15+0x48]
            rcx = m.r64(r15.wrapping_add(0x48)) as u64;
            // e9c4f mov r14,QWORD PTR [r15+0x50]
            r14 = m.r64(r15.wrapping_add(0x50)) as u64;
            // e9c53 sub r14,rcx
            { let (r, f) = sub::<64>(r14, rcx);
              r14 = r;
              fl = f; }
            // e9c56 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9c58 jle 0xe9c8d
            if fl.le() { pc = 0xe9c8d; continue; }
            pc = 0xe9c5a;
        }
        0xe9c5a => {
            // block e9c5a
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c5a cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e9c5c mov rcx,QWORD PTR [r15+0x20]
            rcx = m.r64(r15.wrapping_add(0x20)) as u64;
            // e9c60 sub rcx,QWORD PTR [r15+0x18]
            { let (r, f) = sub::<64>(rcx, m.r64(r15.wrapping_add(0x18)) as u64);
              rcx = r;
              fl = f; }
            // e9c64 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e9c68 add r14,rcx
            { let (r, f) = add::<64>(r14, rcx);
              r14 = r;
              fl = f; }
            // e9c6b jmp 0xe9cc5
            pc = 0xe9cc5; continue;
        }
        0xe9c6d => {
            // block e9c6d
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c6d mov rdi,QWORD PTR [r15+0x78]
            rdi = m.r64(r15.wrapping_add(0x78)) as u64;
            // e9c71 call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c76 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9c78 mov rbx,QWORD PTR [rip+0x56759] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e9c7f mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e9c82 jne 0xe9d35
            if fl.ne() { pc = 0xe9d35; continue; }
            pc = 0xe9c88;
        }
        0xe9c88 => {
            // block e9c88
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c88 jmp 0xe9d32
            pc = 0xe9d32; continue;
        }
        0xe9c8d => {
            // block e9c8d
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c8d mov r8,QWORD PTR [r15+0x18]
            r8 = m.r64(r15.wrapping_add(0x18)) as u64;
            // e9c91 cmp r8,QWORD PTR [r15+0x20]
            fl = sub::<64>(r8, m.r64(r15.wrapping_add(0x20)) as u64).1;
            // e9c95 je 0xe9cc5
            if fl.e() { pc = 0xe9cc5; continue; }
            pc = 0xe9c97;
        }
        0xe9c97 => {
            // block e9c97
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9c97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9c97 mov rdx,QWORD PTR [r15+0x40]
            rdx = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9c9b mov rdi,QWORD PTR [r15+0x80]
            rdi = m.r64(r15.wrapping_add(0x80)) as u64;
            // e9ca2 sub r8,QWORD PTR [r15+0x10]
            { let (r, f) = sub::<64>(r8, m.r64(r15.wrapping_add(0x10)) as u64);
              r8 = r;
              fl = f; }
            // e9ca6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e9ca9 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e9cb0 call QWORD PTR [rax+0x40]
            { let r = { let t = m.r64(rax.wrapping_add(0x40)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9cb3 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e9cb5 sub r14,rax
            { let (r, f) = sub::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // e9cb8 add r14,QWORD PTR [r15+0x48]
            { let (r, f) = add::<64>(r14, m.r64(r15.wrapping_add(0x48)) as u64);
              r14 = r;
              fl = f; }
            // e9cbc sub r14,QWORD PTR [r15+0x40]
            { let (r, f) = sub::<64>(r14, m.r64(r15.wrapping_add(0x40)) as u64);
              r14 = r;
              fl = f; }
            // e9cc0 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // e9cc3 jmp 0xe9cc8
            pc = 0xe9cc8; continue;
        }
        0xe9cc5 => {
            // block e9cc5
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9cc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9cc5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe9cc8;
        }
        0xe9cc8 => {
            // block e9cc8
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9cc8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9cc8 mov rdi,QWORD PTR [r15+0x78]
            rdi = m.r64(r15.wrapping_add(0x78)) as u64;
            // e9ccc neg r14
            { let (r, f) = neg::<64>(r14); r14 = r; fl = f; }
            // e9ccf mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // e9cd4 mov rsi,r14
            rsi = r14;
            // e9cd7 call 0xec1dc
            { let r = { let t = m.env.stub_va(0x9a9); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9cdc test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9cde je 0xe9ce8
            if fl.e() { pc = 0xe9ce8; continue; }
            pc = 0xe9ce0;
        }
        0xe9ce0 => {
            // block e9ce0
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9ce0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ce0 mov r13d,0xffffffff
            r13 = (0xffffffffu64) as u32 as u64;
            // e9ce6 jmp 0xe9d35
            pc = 0xe9d35; continue;
        }
        0xe9ce8 => {
            // block e9ce8
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9ce8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ce8 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // e9ceb je 0xe9d03
            if fl.e() { pc = 0xe9d03; continue; }
            pc = 0xe9ced;
        }
        0xe9ced => {
            // block e9ced
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9ced rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ced lea rdi,[r15+0x88]
            rdi = r15.wrapping_add(0x88);
            // e9cf4 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e9cfb mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // e9d00 rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            pc = 0xe9d03;
        }
        0xe9d03 => {
            // block e9d03
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d03 mov rax,QWORD PTR [r15+0x40]
            rax = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9d07 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // e9d0b mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // e9d0f mov DWORD PTR [r15+0x18c],0x0
            m.w32(r15.wrapping_add(0x18c), (0x0u64) as u32);
            // e9d1a mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            // e9d22 mov QWORD PTR [r15+0x18],0x0
            m.w64(r15.wrapping_add(0x18), (0x0u64) as u64);
            // e9d2a mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            pc = 0xe9d32;
        }
        0xe9d32 => {
            // block e9d32
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d32 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe9d35;
        }
        0xe9d35 => {
            // block e9d35
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d35 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // e9d39 jne 0xe9d50
            if fl.ne() { pc = 0xe9d50; continue; }
            pc = 0xe9d3b;
        }
        0xe9d3b => {
            // block e9d3b
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d3b mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // e9d3e add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9d45 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9d46 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e9d48 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e9d4a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9d4c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e9d4e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9d4f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe9d50 => {
            // block e9d50
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d50 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0xe9d55 => {
            // block e9d55
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d55 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // e9d5a call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9d5f mov rbx,rax
            rbx = rax;
            // e9d62 mov rdi,rbx
            rdi = rbx;
            // e9d65 call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9d6a mov rsi,QWORD PTR [rip+0x5646f] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // e9d71 mov rdx,QWORD PTR [rip+0x56430] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // e9d78 mov rdi,rbx
            rdi = rbx;
            // e9d7b call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9d80 size=600
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_000e9d80<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9d80;
    loop {
        match pc {
        0xe9d80 => {
            // block e9d80
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9d80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d80 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9d81 mov rbp,rsp
            rbp = rsp;
            // e9d84 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9d86 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9d88 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e9d8a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e9d8c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9d8d sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // e9d91 mov rbx,rdi
            rbx = rdi;
            // e9d94 mov r15d,0xffffffff
            r15 = (0xffffffffu64) as u32 as u64;
            // e9d9a cmp QWORD PTR [rbx+0x78],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x78)) as u64, 0x0u64).1;
            // e9d9f je 0xe9f9b
            if fl.e() { pc = 0xe9f9b; continue; }
            pc = 0xe9da5;
        }
        0xe9da5 => {
            // block e9da5
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9da5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9da5 mov rdi,rbx
            rdi = rbx;
            // e9da8 call 0xeb900
            { let r = { let t = m.env.stub_va(0xcd); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9dad mov rsi,QWORD PTR [rbx+0x18]
            rsi = m.r64(rbx.wrapping_add(0x18)) as u64;
            // e9db1 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // e9db4 jne 0xe9dca
            if fl.ne() { pc = 0xe9dca; continue; }
            pc = 0xe9db6;
        }
        0xe9db6 => {
            // block e9db6
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9db6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9db6 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // e9dba lea rcx,[rbp-0x29]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // e9dbe mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // e9dc2 mov QWORD PTR [rbx+0x18],rsi
            m.w64(rbx.wrapping_add(0x18), (rsi) as u64);
            // e9dc6 mov QWORD PTR [rbx+0x20],rsi
            m.w64(rbx.wrapping_add(0x20), (rsi) as u64);
            pc = 0xe9dca;
        }
        0xe9dca => {
            // block e9dca
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9dca mov rcx,QWORD PTR [rbx+0x20]
            rcx = m.r64(rbx.wrapping_add(0x20)) as u64;
            // e9dce xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e9dd1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e9dd3 jne 0xe9df7
            if fl.ne() { pc = 0xe9df7; continue; }
            pc = 0xe9dd5;
        }
        0xe9dd5 => {
            // block e9dd5
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9dd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9dd5 mov rax,rcx
            rax = rcx;
            // e9dd8 sub rax,QWORD PTR [rbx+0x10]
            { let (r, f) = sub::<64>(rax, m.r64(rbx.wrapping_add(0x10)) as u64);
              rax = r;
              fl = f; }
            // e9ddc mov rdx,rax
            rdx = rax;
            // e9ddf shr rdx,0x3f
            { let (r, f) = shr::<64>(rdx, 0x3fu64, fl); rdx = r; fl = f; }
            // e9de3 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // e9de6 sar rdx,1
            { let (r, f) = sar::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e9de9 cmp rdx,0x4
            fl = sub::<64>(rdx, 0x4u64).1;
            // e9ded mov r14d,0x4
            r14 = (0x4u64) as u32 as u64;
            // e9df3 cmovbe r14,rdx
            if fl.be() { r14 = rdx; }
            pc = 0xe9df7;
        }
        0xe9df7 => {
            // block e9df7
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9df7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9df7 cmp rsi,rcx
            fl = sub::<64>(rsi, rcx).1;
            // e9dfa je 0xe9e09
            if fl.e() { pc = 0xe9e09; continue; }
            pc = 0xe9dfc;
        }
        0xe9dfc => {
            // block e9dfc
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9dfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9dfc movzx r15d,BYTE PTR [rsi]
            r15 = (m.r8(rsi) as u64) as u32 as u64;
            // e9e00 lea r13,[rbx+0x10]
            r13 = rbx.wrapping_add(0x10);
            // e9e04 jmp 0xe9f82
            pc = 0xe9f82; continue;
        }
        0xe9e09 => {
            // block e9e09
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9e09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9e09 lea r13,[rbx+0x10]
            r13 = rbx.wrapping_add(0x10);
            // e9e0d mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e9e11 sub rsi,r14
            { let (r, f) = sub::<64>(rsi, r14);
              rsi = r;
              fl = f; }
            // e9e14 mov rdx,r14
            rdx = r14;
            // e9e17 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9e1c cmp BYTE PTR [rbx+0x192],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x192)) as u64, 0x0u64).1;
            // e9e23 je 0xe9e73
            if fl.e() { pc = 0xe9e73; continue; }
            pc = 0xe9e25;
        }
        0xe9e25 => {
            // block e9e25
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9e25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9e25 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e9e29 mov rdx,QWORD PTR [rbx+0x20]
            rdx = m.r64(rbx.wrapping_add(0x20)) as u64;
            // e9e2d sub rdx,r14
            { let (r, f) = sub::<64>(rdx, r14);
              rdx = r;
              fl = f; }
            // e9e30 sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // e9e33 add rdi,r14
            { let (r, f) = add::<64>(rdi, r14);
              rdi = r;
              fl = f; }
            // e9e36 mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // e9e3a mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e9e3f call 0xec1ca
            { let r = { let t = m.env.stub_va(0x997); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9e44 mov r15d,0xffffffff
            r15 = (0xffffffffu64) as u32 as u64;
            // e9e4a test rax,rax
            fl = logic::<64>(rax & rax);
            // e9e4d je 0xe9f82
            if fl.e() { pc = 0xe9f82; continue; }
            pc = 0xe9e53;
        }
        0xe9e53 => {
            // block e9e53
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9e53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9e53 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e9e57 lea rdx,[rcx+r14*1]
            rdx = rcx.wrapping_add(r14);
            // e9e5b add rax,r14
            { let (r, f) = add::<64>(rax, r14);
              rax = r;
              fl = f; }
            // e9e5e add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // e9e61 mov QWORD PTR [rbx+0x18],rdx
            m.w64(rbx.wrapping_add(0x18), (rdx) as u64);
            // e9e65 mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // e9e69 movzx r15d,BYTE PTR [rcx+r14*1]
            r15 = (m.r8(rcx.wrapping_add(r14)) as u64) as u32 as u64;
            // e9e6e jmp 0xe9f82
            pc = 0xe9f82; continue;
        }
        0xe9e73 => {
            // block e9e73
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9e73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9e73 mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // e9e77 mov rsi,QWORD PTR [rbx+0x48]
            rsi = m.r64(rbx.wrapping_add(0x48)) as u64;
            // e9e7b mov rdx,QWORD PTR [rbx+0x50]
            rdx = m.r64(rbx.wrapping_add(0x50)) as u64;
            // e9e7f sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // e9e82 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9e87 mov rdx,QWORD PTR [rbx+0x40]
            rdx = m.r64(rbx.wrapping_add(0x40)) as u64;
            // e9e8b mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // e9e8f sub rax,QWORD PTR [rbx+0x48]
            { let (r, f) = sub::<64>(rax, m.r64(rbx.wrapping_add(0x48)) as u64);
              rax = r;
              fl = f; }
            // e9e93 add rax,rdx
            { let (r, f) = add::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // e9e96 mov QWORD PTR [rbx+0x48],rax
            m.w64(rbx.wrapping_add(0x48), (rax) as u64);
            // e9e9a lea rsi,[rbx+0x58]
            rsi = rbx.wrapping_add(0x58);
            // e9e9e mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // e9ea3 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // e9ea6 je 0xe9eac
            if fl.e() { pc = 0xe9eac; continue; }
            pc = 0xe9ea8;
        }
        0xe9ea8 => {
            // block e9ea8
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9ea8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ea8 mov rcx,QWORD PTR [rbx+0x60]
            rcx = m.r64(rbx.wrapping_add(0x60)) as u64;
            pc = 0xe9eac;
        }
        0xe9eac => {
            // block e9eac
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9eac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9eac add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // e9eaf mov QWORD PTR [rbx+0x50],rdx
            m.w64(rbx.wrapping_add(0x50), (rdx) as u64);
            // e9eb3 mov rcx,QWORD PTR [rbx+0x70]
            rcx = m.r64(rbx.wrapping_add(0x70)) as u64;
            // e9eb7 sub rcx,r14
            { let (r, f) = sub::<64>(rcx, r14);
              rcx = r;
              fl = f; }
            // e9eba sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // e9ebd cmp rdx,rcx
            fl = sub::<64>(rdx, rcx).1;
            // e9ec0 cmovae rdx,rcx
            if fl.ae() { rdx = rcx; }
            // e9ec4 lea rdi,[rbx+0x108]
            rdi = rbx.wrapping_add(0x108);
            // e9ecb lea r12,[rbx+0x88]
            r12 = rbx.wrapping_add(0x88);
            // e9ed2 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // e9ed7 mov rsi,r12
            rsi = r12;
            // e9eda rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            // e9edd mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // e9ee1 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e9ee6 mov rdi,rax
            rdi = rax;
            // e9ee9 call 0xec1ca
            { let r = { let t = m.env.stub_va(0x997); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9eee mov r15d,0xffffffff
            r15 = (0xffffffffu64) as u32 as u64;
            // e9ef4 test rax,rax
            fl = logic::<64>(rax & rax);
            // e9ef7 je 0xe9f82
            if fl.e() { pc = 0xe9f82; continue; }
            pc = 0xe9efd;
        }
        0xe9efd => {
            // block e9efd
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9efd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9efd mov rdi,QWORD PTR [rbx+0x80]
            rdi = m.r64(rbx.wrapping_add(0x80)) as u64;
            // e9f04 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e9f07 je 0xe9fad
            if fl.e() { pc = 0xe9fad; continue; }
            pc = 0xe9f0d;
        }
        0xe9f0d => {
            // block e9f0d
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f0d lea r8,[rbx+0x48]
            r8 = rbx.wrapping_add(0x48);
            // e9f11 add rax,QWORD PTR [rbx+0x48]
            { let (r, f) = add::<64>(rax, m.r64(rbx.wrapping_add(0x48)) as u64);
              rax = r;
              fl = f; }
            // e9f15 mov QWORD PTR [rbx+0x50],rax
            m.w64(rbx.wrapping_add(0x50), (rax) as u64);
            // e9f19 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e9f1d mov rdx,QWORD PTR [rbx+0x40]
            rdx = m.r64(rbx.wrapping_add(0x40)) as u64;
            // e9f21 lea r9,[rcx+r14*1]
            r9 = rcx.wrapping_add(r14);
            // e9f25 add rcx,QWORD PTR [rbx+0x70]
            { let (r, f) = add::<64>(rcx, m.r64(rbx.wrapping_add(0x70)) as u64);
              rcx = r;
              fl = f; }
            // e9f29 mov r10,QWORD PTR [rdi]
            r10 = m.r64(rdi) as u64;
            // e9f2c lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // e9f30 mov QWORD PTR [rsp+0x8],rsi
            m.w64(rsp.wrapping_add(0x8), (rsi) as u64);
            // e9f35 mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // e9f39 mov rsi,r12
            rsi = r12;
            // e9f3c mov rcx,rax
            rcx = rax;
            // e9f3f call QWORD PTR [r10+0x20]
            { let r = { let t = m.r64(r10.wrapping_add(0x20)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9f43 cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e9f46 jne 0xe9f62
            if fl.ne() { pc = 0xe9f62; continue; }
            pc = 0xe9f48;
        }
        0xe9f48 => {
            // block e9f48
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f48 mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // e9f4c mov rcx,QWORD PTR [rbx+0x50]
            rcx = m.r64(rbx.wrapping_add(0x50)) as u64;
            // e9f50 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e9f54 mov QWORD PTR [rbx+0x18],rax
            m.w64(rbx.wrapping_add(0x18), (rax) as u64);
            // e9f58 mov QWORD PTR [rbx+0x20],rcx
            m.w64(rbx.wrapping_add(0x20), (rcx) as u64);
            // e9f5c movzx r15d,BYTE PTR [rax]
            r15 = (m.r8(rax) as u64) as u32 as u64;
            // e9f60 jmp 0xe9f82
            pc = 0xe9f82; continue;
        }
        0xe9f62 => {
            // block e9f62
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f62 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e9f66 mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // e9f6a add r14,rcx
            { let (r, f) = add::<64>(r14, rcx);
              r14 = r;
              fl = f; }
            // e9f6d cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // e9f70 je 0xe9f82
            if fl.e() { pc = 0xe9f82; continue; }
            pc = 0xe9f72;
        }
        0xe9f72 => {
            // block e9f72
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f72 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // e9f76 mov QWORD PTR [rbx+0x18],r14
            m.w64(rbx.wrapping_add(0x18), (r14) as u64);
            // e9f7a mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // e9f7e movzx r15d,BYTE PTR [r14]
            r15 = (m.r8(r14) as u64) as u32 as u64;
            pc = 0xe9f82;
        }
        0xe9f82 => {
            // block e9f82
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f82 lea rax,[rbp-0x29]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // e9f86 cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // e9f8a jne 0xe9f9b
            if fl.ne() { pc = 0xe9f9b; continue; }
            pc = 0xe9f8c;
        }
        0xe9f8c => {
            // block e9f8c
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f8c mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // e9f94 xorps xmm0,xmm0
            x0 = 0;
            // e9f97 movups XMMWORD PTR [rbx+0x18],xmm0
            m.w128(rbx.wrapping_add(0x18), x0);
            pc = 0xe9f9b;
        }
        0xe9f9b => {
            // block e9f9b
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9f9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9f9b mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // e9f9e add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // e9fa2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9fa3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e9fa5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e9fa7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9fa9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e9fab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9fac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe9fad => {
            // block e9fad
            #[cfg(feature = "trace")] eprintln!("f_000e9d80 pc e9fad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9fad mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // e9fb2 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9fb7 mov rbx,rax
            rbx = rax;
            // e9fba mov rdi,rbx
            rdi = rbx;
            // e9fbd call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9fc2 mov rsi,QWORD PTR [rip+0x56217] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // e9fc9 mov rdx,QWORD PTR [rip+0x561d8] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // e9fd0 mov rdi,rbx
            rdi = rbx;
            // e9fd3 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea028 size=488
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_000ea028<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xea028;
    loop {
        match pc {
        0xea028 => {
            // block ea028
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea028 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea028 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea029 mov rbp,rsp
            rbp = rsp;
            // ea02c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ea02e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea030 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ea032 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ea034 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea035 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // ea039 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // ea03c mov rbx,rdi
            rbx = rdi;
            // ea03f mov r15d,0xffffffff
            r15 = (0xffffffffu64) as u32 as u64;
            // ea045 cmp QWORD PTR [rbx+0x78],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x78)) as u64, 0x0u64).1;
            // ea04a je 0xea1e7
            if fl.e() { pc = 0xea1e7; continue; }
            pc = 0xea050;
        }
        0xea050 => {
            // block ea050
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea050 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea050 mov rdi,rbx
            rdi = rbx;
            // ea053 call 0xebf0c
            { let r = { let t = m.env.stub_va(0x6d9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea058 mov rsi,QWORD PTR [rbx+0x28]
            rsi = m.r64(rbx.wrapping_add(0x28)) as u64;
            // ea05c mov rcx,QWORD PTR [rbx+0x30]
            rcx = m.r64(rbx.wrapping_add(0x30)) as u64;
            // ea060 mov rdi,QWORD PTR [rbx+0x38]
            rdi = m.r64(rbx.wrapping_add(0x38)) as u64;
            // ea064 cmp r14d,0xffffffff
            fl = sub::<32>((r14 as u32 as u64), 0xffffffffu64).1;
            // ea068 je 0xea097
            if fl.e() { pc = 0xea097; continue; }
            pc = 0xea06a;
        }
        0xea06a => {
            // block ea06a
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea06a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea06a test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // ea06d jne 0xea083
            if fl.ne() { pc = 0xea083; continue; }
            pc = 0xea06f;
        }
        0xea06f => {
            // block ea06f
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea06f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea06f lea rax,[rbp-0x28]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // ea073 lea rcx,[rbp-0x29]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // ea077 mov QWORD PTR [rbx+0x30],rcx
            m.w64(rbx.wrapping_add(0x30), (rcx) as u64);
            // ea07b mov QWORD PTR [rbx+0x28],rcx
            m.w64(rbx.wrapping_add(0x28), (rcx) as u64);
            // ea07f mov QWORD PTR [rbx+0x38],rax
            m.w64(rbx.wrapping_add(0x38), (rax) as u64);
            pc = 0xea083;
        }
        0xea083 => {
            // block ea083
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea083 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea083 mov BYTE PTR [rcx],r14b
            m.w8(rcx, ((r14 as u8 as u64)) as u8);
            // ea086 mov rcx,QWORD PTR [rbx+0x30]
            rcx = m.r64(rbx.wrapping_add(0x30)) as u64;
            // ea08a inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // ea08d mov QWORD PTR [rbx+0x30],rcx
            m.w64(rbx.wrapping_add(0x30), (rcx) as u64);
            // ea091 mov rdx,QWORD PTR [rbx+0x28]
            rdx = m.r64(rbx.wrapping_add(0x28)) as u64;
            // ea095 jmp 0xea09a
            pc = 0xea09a; continue;
        }
        0xea097 => {
            // block ea097
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea097 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea097 mov rdx,rsi
            rdx = rsi;
            pc = 0xea09a;
        }
        0xea09a => {
            // block ea09a
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea09a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea09a mov r12,rcx
            r12 = rcx;
            // ea09d sub r12,rdx
            { let (r, f) = sub::<64>(r12, rdx);
              r12 = r;
              fl = f; }
            // ea0a0 je 0xea1dc
            if fl.e() { pc = 0xea1dc; continue; }
            pc = 0xea0a6;
        }
        0xea0a6 => {
            // block ea0a6
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0a6 mov QWORD PTR [rbp-0x58],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rdi) as u64);
            // ea0aa mov QWORD PTR [rbp-0x50],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rsi) as u64);
            // ea0ae cmp BYTE PTR [rbx+0x192],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x192)) as u64, 0x0u64).1;
            // ea0b5 je 0xea0d9
            if fl.e() { pc = 0xea0d9; continue; }
            pc = 0xea0b7;
        }
        0xea0b7 => {
            // block ea0b7
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0b7 mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // ea0bb mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // ea0c0 mov rdi,rdx
            rdi = rdx;
            pc = 0xea0c3;
        }
        0xea0c3 => {
            // block ea0c3
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0c3 mov rdx,r12
            rdx = r12;
            // ea0c6 call 0xec1e8
            { let r = { let t = m.env.stub_va(0x9b5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea0cb cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // ea0ce jne 0xea1e7
            if fl.ne() { pc = 0xea1e7; continue; }
            pc = 0xea0d4;
        }
        0xea0d4 => {
            // block ea0d4
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0d4 jmp 0xea1c8
            pc = 0xea1c8; continue;
        }
        0xea0d9 => {
            // block ea0d9
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0d9 mov r9,QWORD PTR [rbx+0x40]
            r9 = m.r64(rbx.wrapping_add(0x40)) as u64;
            // ea0dd mov QWORD PTR [rbp-0x38],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r9) as u64);
            // ea0e1 mov rdi,QWORD PTR [rbx+0x80]
            rdi = m.r64(rbx.wrapping_add(0x80)) as u64;
            // ea0e8 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // ea0eb je 0xea19d
            if fl.e() { pc = 0xea19d; continue; }
            pc = 0xea0f1;
        }
        0xea0f1 => {
            // block ea0f1
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0f1 lea rax,[rbx+0x88]
            rax = rbx.wrapping_add(0x88);
            // ea0f8 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // ea0fc jmp 0xea102
            pc = 0xea102; continue;
        }
        0xea0fe => {
            // block ea0fe
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0fe mov r9,QWORD PTR [rbx+0x40]
            r9 = m.r64(rbx.wrapping_add(0x40)) as u64;
            pc = 0xea102;
        }
        0xea102 => {
            // block ea102
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea102 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea102 mov rax,QWORD PTR [rbx+0x60]
            rax = m.r64(rbx.wrapping_add(0x60)) as u64;
            // ea106 add rax,r9
            { let (r, f) = add::<64>(rax, r9);
              rax = r;
              fl = f; }
            // ea109 mov r10,QWORD PTR [rdi]
            r10 = m.r64(rdi) as u64;
            // ea10c lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // ea110 mov QWORD PTR [rsp+0x8],rsi
            m.w64(rsp.wrapping_add(0x8), (rsi) as u64);
            // ea115 mov QWORD PTR [rsp],rax
            m.w64(rsp, (rax) as u64);
            // ea119 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // ea11d lea r8,[rbp-0x40]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ea121 call QWORD PTR [r10+0x18]
            { let r = { let t = m.r64(r10.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea125 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // ea128 mov rdi,QWORD PTR [rbx+0x28]
            rdi = m.r64(rbx.wrapping_add(0x28)) as u64;
            // ea12c cmp QWORD PTR [rbp-0x40],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64, rdi).1;
            // ea130 je 0xea1e7
            if fl.e() { pc = 0xea1e7; continue; }
            pc = 0xea136;
        }
        0xea136 => {
            // block ea136
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea136 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea136 cmp r13d,0x3
            fl = sub::<32>((r13 as u32 as u64), 0x3u64).1;
            // ea13a je 0xea1f9
            if fl.e() { pc = 0xea1f9; continue; }
            pc = 0xea140;
        }
        0xea140 => {
            // block ea140
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea140 cmp r13d,0x1
            fl = sub::<32>((r13 as u32 as u64), 0x1u64).1;
            // ea144 ja 0xea1e7
            if fl.a() { pc = 0xea1e7; continue; }
            pc = 0xea14a;
        }
        0xea14a => {
            // block ea14a
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea14a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea14a mov r12,QWORD PTR [rbp-0x38]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // ea14e mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // ea152 mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // ea156 sub r12,rdi
            { let (r, f) = sub::<64>(r12, rdi);
              r12 = r;
              fl = f; }
            // ea159 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // ea15e mov rdx,r12
            rdx = r12;
            // ea161 call 0xec1e8
            { let r = { let t = m.env.stub_va(0x9b5); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea166 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // ea169 jne 0xea1e7
            if fl.ne() { pc = 0xea1e7; continue; }
            pc = 0xea16b;
        }
        0xea16b => {
            // block ea16b
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea16b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea16b cmp r13d,0x1
            fl = sub::<32>((r13 as u32 as u64), 0x1u64).1;
            // ea16f jne 0xea1c8
            if fl.ne() { pc = 0xea1c8; continue; }
            pc = 0xea171;
        }
        0xea171 => {
            // block ea171
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea171 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea171 mov rdx,QWORD PTR [rbp-0x40]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // ea175 mov rax,QWORD PTR [rbx+0x30]
            rax = m.r64(rbx.wrapping_add(0x30)) as u64;
            // ea179 mov QWORD PTR [rbx+0x28],rdx
            m.w64(rbx.wrapping_add(0x28), (rdx) as u64);
            // ea17d mov QWORD PTR [rbx+0x38],rax
            m.w64(rbx.wrapping_add(0x38), (rax) as u64);
            // ea181 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // ea183 movsxd rcx,eax
            rcx = sx::<32>((rax as u32 as u64)) as u64;
            // ea186 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // ea189 mov QWORD PTR [rbx+0x30],rcx
            m.w64(rbx.wrapping_add(0x30), (rcx) as u64);
            // ea18d mov rdi,QWORD PTR [rbx+0x80]
            rdi = m.r64(rbx.wrapping_add(0x80)) as u64;
            // ea194 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // ea197 jne 0xea0fe
            if fl.ne() { pc = 0xea0fe; continue; }
            pc = 0xea19d;
        }
        0xea19d => {
            // block ea19d
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea19d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea19d mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // ea1a2 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea1a7 mov rbx,rax
            rbx = rax;
            // ea1aa mov rdi,rbx
            rdi = rbx;
            // ea1ad call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea1b2 mov rsi,QWORD PTR [rip+0x56027] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // ea1b9 mov rdx,QWORD PTR [rip+0x55fe8] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // ea1c0 mov rdi,rbx
            rdi = rbx;
            // ea1c3 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0xea1c8 => {
            // block ea1c8
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea1c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea1c8 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // ea1cc mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // ea1d0 mov QWORD PTR [rbx+0x28],rax
            m.w64(rbx.wrapping_add(0x28), (rax) as u64);
            // ea1d4 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // ea1d8 mov QWORD PTR [rbx+0x38],rax
            m.w64(rbx.wrapping_add(0x38), (rax) as u64);
            pc = 0xea1dc;
        }
        0xea1dc => {
            // block ea1dc
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea1dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea1dc xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // ea1df cmp r14d,0xffffffff
            fl = sub::<32>((r14 as u32 as u64), 0xffffffffu64).1;
            // ea1e3 cmovne r15d,r14d
            if fl.ne() { r15 = ((r14 as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            pc = 0xea1e7;
        }
        0xea1e7 => {
            // block ea1e7
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea1e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea1e7 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // ea1ea add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // ea1ee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ea1ef pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ea1f1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ea1f3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ea1f5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ea1f7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea1f8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xea1f9 => {
            // block ea1f9
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea1f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea1f9 mov r12,QWORD PTR [rbx+0x30]
            r12 = m.r64(rbx.wrapping_add(0x30)) as u64;
            // ea1fd mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // ea201 sub r12,rdi
            { let (r, f) = sub::<64>(r12, rdi);
              r12 = r;
              fl = f; }
            // ea204 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // ea209 jmp 0xea0c3
            pc = 0xea0c3; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
