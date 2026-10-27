//! Mechanically lifted functions (scripts/lift_run.py), module k2_001. Each one is checked against the original
//! in the world harness (tests/world_k2_001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

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
            { m.outregs(0x3155e, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3155e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31560 je 0x315db
            if crate::cov::br(fl.e()) { pc = 0x315db; continue; }
            pc = 0x31562;
        }
        0x31562 => {
            // block 31562
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 31562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31562 mov rdi,r14
            rdi = r14;
            // 31565 call 0xb6914
            { m.outregs(0x3156a, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xb6914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if crate::cov::br(fl.ne()) { pc = 0x315de; continue; }
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
            { m.outregs(0x315a0, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 315a0 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 315a3 test BYTE PTR [rbp-0x2c],0x4
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & 0x4u64);
            // 315a7 jne 0x315de
            if crate::cov::br(fl.ne()) { pc = 0x315de; continue; }
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
            if crate::cov::br(fl.a()) { pc = 0x315db; continue; }
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
            { m.outregs(0x315d3, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 315d3 test BYTE PTR [rbp-0x2c],0x4
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & 0x4u64);
            // 315d7 je 0x315ad
            if crate::cov::br(fl.e()) { pc = 0x315ad; continue; }
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
            { m.outregs(0x136ee, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 136ee mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 136f1 cmp BYTE PTR [r13+0x2e8],0x0
            fl = sub::<8>(m.r8(r13.wrapping_add(0x2e8)) as u64, 0x0u64).1;
            // 136f9 mov r14d,ebx
            r14 = ((rbx as u32 as u64)) as u32 as u64;
            // 136fc mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 13701 je 0x1377a
            if crate::cov::br(fl.e()) { pc = 0x1377a; continue; }
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
            { m.outregs(0x1371b, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x811); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if crate::cov::br(fl.ae()) { pc = 0x13762; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0x13767; continue; }
            pc = 0x13735;
        }
        0x13735 => {
            // block 13735
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13735 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13735 mov rdi,rbx
            rdi = rbx;
            // 13738 call 0xec03e
            { m.outregs(0x1373d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x80b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if crate::cov::br(fl.ne()) { pc = 0x13722; continue; }
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
            { m.outregs(0x13775, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0x8e56, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xceac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0x8e7a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x6858c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8e7a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8e7c je 0x8edb
            if crate::cov::br(fl.e()) { pc = 0x8edb; continue; }
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
            if crate::cov::br(fl.ae()) { pc = 0x8edd; continue; }
            pc = 0x8e89;
        }
        0x8e89 => {
            // block 8e89
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e89 cmp DWORD PTR [rbp-0x30],0x3
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x3u64).1;
            // 8e8d ja 0x8edd
            if crate::cov::br(fl.a()) { pc = 0x8edd; continue; }
            pc = 0x8e8f;
        }
        0x8e8f => {
            // block 8e8f
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e8f test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 8e92 je 0x8eb1
            if crate::cov::br(fl.e()) { pc = 0x8eb1; continue; }
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
            { m.outregs(0x8eab, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x10f8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8eab cmp BYTE PTR [rbp-0x31],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf)) as u64, 0x0u64).1;
            // 8eaf je 0x8edb
            if crate::cov::br(fl.e()) { pc = 0x8edb; continue; }
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
            { m.outregs(0x8ed9, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if crate::cov::br(fl.e()) { pc = 0xd5d85; continue; }
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
            if crate::cov::br(fl.be()) { pc = 0xd5db6; continue; }
            pc = 0xd5d8d;
        }
        0xd5d8d => {
            // block d5d8d
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d8d test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5d8f jne 0xd5d99
            if crate::cov::br(fl.ne()) { pc = 0xd5d99; continue; }
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
            { m.outregs(0xd5db3, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7e7); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if crate::cov::br(fl.ne()) { pc = 0xd5dc0; continue; }
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
            if crate::cov::br(fl.e()) { pc = 0xd5dde; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0xd5dcc; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0xd5def; continue; }
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
            { m.outregs(0xd3408, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa51); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3408 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d340a mov eax,0xfffffffe
            rax = (0xfffffffeu64) as u32 as u64;
            // d340f test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3411 je 0xd3434
            if crate::cov::br(fl.e()) { pc = 0xd3434; continue; }
            pc = 0xd3413;
        }
        0xd3413 => {
            // block d3413
            #[cfg(feature = "trace")] eprintln!("f_000d3371 pc d3413 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3413 cmp ecx,0xffffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffffu64).1;
            // d3416 jne 0xd341f
            if crate::cov::br(fl.ne()) { pc = 0xd341f; continue; }
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
            { m.outregs(0xd3434, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa4b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0x523f6, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0x5241a, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0x52448, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52448 mov rsi,QWORD PTR [rip+0xedd41] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 5244f mov rdi,r14
            rdi = r14;
            // 52452 call 0xebfcc
            { m.outregs(0x52457, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52457 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5245a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 5245e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 52463 mov rdi,rax
            rdi = rax;
            // 52466 call rcx
            { m.outregs(0x52468, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52468 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 5246b lea rdi,[rbp-0x18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 5246f call 0xec0e0
            { m.outregs(0x52474, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52474 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // 52478 mov rdi,rbx
            rdi = rbx;
            // 5247b call 0xec068
            { m.outregs(0x52480, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 52480 mov rdi,rbx
            rdi = rbx;
            // 52483 call 0xec06e
            { m.outregs(0x52488, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5249d mov rdi,rbx
            rdi = rbx;
            // 524a0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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
            if crate::cov::br(fl.e()) { pc = 0xd216; continue; }
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
            { m.outregs(0xd1b3, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1b3 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // d1b8 je 0xd206
            if crate::cov::br(fl.e()) { pc = 0xd206; continue; }
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
            { m.outregs(0xd1c6, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1c6 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // d1cb je 0xd206
            if crate::cov::br(fl.e()) { pc = 0xd206; continue; }
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
            { m.outregs(0xd1d9, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1d9 cmp eax,0x10710800
            fl = sub::<32>((rax as u32 as u64), 0x10710800u64).1;
            // d1de je 0xd206
            if crate::cov::br(fl.e()) { pc = 0xd206; continue; }
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
            { m.outregs(0xd1ec, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1ec cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // d1f1 je 0xd206
            if crate::cov::br(fl.e()) { pc = 0xd206; continue; }
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
            { m.outregs(0xd1ff, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1ff cmp eax,0x10710851
            fl = sub::<32>((rax as u32 as u64), 0x10710851u64).1;
            // d204 jne 0xd216
            if crate::cov::br(fl.ne()) { pc = 0xd216; continue; }
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
            { m.outregs(0xd216, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x328)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0xd222, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0xd24b, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0xd269, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if crate::cov::br(fl.e()) { pc = 0x64ad6; continue; }
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
            if crate::cov::br(fl.ae()) { pc = 0x64aa4; continue; }
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
            { m.outregs(0x64aaf, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x38b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if crate::cov::br(fl.e()) { pc = 0x64ac5; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0x64ab8; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0x64ac5; continue; }
            pc = 0x64ad1;
        }
        0x64ad1 => {
            // block 64ad1
            #[cfg(feature = "trace")] eprintln!("f_00064a17 pc 64ad1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64ad1 cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // 64ad4 jne 0x64a60
            if crate::cov::br(fl.ne()) { pc = 0x64a60; continue; }
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
            { let r = { let t = m.env.stub_va(0x349); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64af1 mov rdi,r14
            rdi = r14;
            // 64af4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0x69285, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x6911e); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0x692a1, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x68fea); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 692a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 692a3 je 0x692b1
            if crate::cov::br(fl.e()) { pc = 0x692b1; continue; }
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
            { m.outregs(0x692b1, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x475); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if crate::cov::br(fl.e()) { pc = 0x692c7; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0x692ba; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0x692ca; continue; }
            pc = 0x692d6;
        }
        0x692d6 => {
            // block 692d6
            #[cfg(feature = "trace")] eprintln!("f_00069246 pc 692d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 692d6 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 692d9 jne 0x6928f
            if crate::cov::br(fl.ne()) { pc = 0x6928f; continue; }
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
            { m.outregs(0x692e8, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x48d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x48d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if crate::cov::br(fl.ge()) { pc = 0xc8b11; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0xc8a95; continue; }
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
            { m.outregs(0x5d31, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d31 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d33 jne 0x5ddf
            if crate::cov::br(fl.ne()) { pc = 0x5ddf; continue; }
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
            { m.outregs(0x5d4c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d4c mov r12d,DWORD PTR [rbp-0x24]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 5d50 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d53 mov rdi,rbx
            rdi = rbx;
            // 5d56 call QWORD PTR [rax+0x128]
            { m.outregs(0x5d5c, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d5c cmp r12d,eax
            fl = sub::<32>((r12 as u32 as u64), (rax as u32 as u64)).1;
            // 5d5f jbe 0x5d68
            if crate::cov::br(fl.be()) { pc = 0x5d68; continue; }
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
            { m.outregs(0x5d74, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d74 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5d76 call 0xcdd26
            { m.outregs(0x5d7b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcdd26); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d7b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d7d je 0x5db7
            if crate::cov::br(fl.e()) { pc = 0x5db7; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0x5da2; continue; }
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
            { m.outregs(0x5d97, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d97 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d99 je 0x5ddf
            if crate::cov::br(fl.e()) { pc = 0x5ddf; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0x5dd1; continue; }
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
            { m.outregs(0x5dcd, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5dcd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5dcf je 0x5ddf
            if crate::cov::br(fl.e()) { pc = 0x5ddf; continue; }
            pc = 0x5dd1;
        }
        0x5dd1 => {
            // block 5dd1
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dd1 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 5dd4 je 0x5de3
            if crate::cov::br(fl.e()) { pc = 0x5de3; continue; }
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
            if crate::cov::br(fl.e()) { pc = 0xb7a5d; continue; }
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
            if crate::cov::br(fl.b()) { pc = 0xb7a2f; continue; }
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
            if crate::cov::br(fl.b()) { pc = 0xb7a2f; continue; }
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
            if crate::cov::br(fl.e()) { pc = 0xb7a2f; continue; }
            pc = 0xb7a2a;
        }
        0xb7a2a => {
            // block b7a2a
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a2a cmp eax,0x2d
            fl = sub::<32>((rax as u32 as u64), 0x2du64).1;
            // b7a2d jne 0xb7a5d
            if crate::cov::br(fl.ne()) { pc = 0xb7a5d; continue; }
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
            if crate::cov::br(fl.a()) { pc = 0xb7a46; continue; }
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
            if crate::cov::br(fl.ne()) { pc = 0xb7a05; continue; }
            pc = 0xb7a42;
        }
        0xb7a42 => {
            // block b7a42
            #[cfg(feature = "trace")] eprintln!("f_000b799c pc b7a42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a42 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // b7a44 je 0xb7a5d
            if crate::cov::br(fl.e()) { pc = 0xb7a5d; continue; }
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
            { m.outregs(0xb7a52, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { m.outregs(0xb7a6e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7a6e cmp r15,QWORD PTR [rbp-0x20]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // b7a72 jne 0xb7a82
            if crate::cov::br(fl.ne()) { pc = 0xb7a82; continue; }
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
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb944a size=242
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b944a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb944a;
    loop {
        match pc {
        0xb944a => {
            // block b944a
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b944a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b944a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b944b mov rbp,rsp
            rbp = rsp;
            // b944e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b9450 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b9452 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b9454 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b9456 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9457 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b9458 mov r13,rdx
            r13 = rdx;
            // b945b mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // b945e mov r14,rdi
            r14 = rdi;
            // b9461 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9465 call 0xcbc54
            { m.outregs(0xb946a, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcbc54); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b946a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b946c je 0xb952a
            if crate::cov::br(fl.e()) { pc = 0xb952a; continue; }
            pc = 0xb9472;
        }
        0xb9472 => {
            // block b9472
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9472 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9472 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9476 call 0xcbe98
            { m.outregs(0xb947b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b947b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b947d je 0xb952a
            if crate::cov::br(fl.e()) { pc = 0xb952a; continue; }
            pc = 0xb9483;
        }
        0xb9483 => {
            // block b9483
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9483 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9487 call 0xd0ac8
            { m.outregs(0xb948c, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b948c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // b948f movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // b9493 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b9496 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b9498 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // b949d jbe 0xb952c
            if crate::cov::br(fl.be()) { pc = 0xb952c; continue; }
            pc = 0xb94a3;
        }
        0xb94a3 => {
            // block b94a3
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94a3 lea rax,[rip+0x49506] # 0x1029b0
            rax = m.env.data_va(94589);
            // b94aa mov r15d,DWORD PTR [rax+rcx*4]
            r15 = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b94ae mov rbx,QWORD PTR [r13+0x0]
            rbx = m.r64(r13) as u64;
            // b94b2 add r13,0x8
            { let (r, f) = add::<64>(r13, 0x8u64);
              r13 = r;
              fl = f; }
            pc = 0xb94b6;
        }
        0xb94b6 => {
            // block b94b6
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94b6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b94b8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xb94ba;
        }
        0xb94ba => {
            // block b94ba
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94ba cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // b94bd je 0xb94d5
            if crate::cov::br(fl.e()) { pc = 0xb94d5; continue; }
            pc = 0xb94bf;
        }
        0xb94bf => {
            // block b94bf
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94bf mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b94c3 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b94c6 je 0xb94dc
            if crate::cov::br(fl.e()) { pc = 0xb94dc; continue; }
            pc = 0xb94c8;
        }
        0xb94c8 => {
            // block b94c8
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94c8 mov rcx,rsi
            rcx = rsi;
            // b94cb mov rsi,QWORD PTR [rcx]
            rsi = m.r64(rcx) as u64;
            // b94ce test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b94d1 jne 0xb94c8
            if crate::cov::br(fl.ne()) { pc = 0xb94c8; continue; }
            pc = 0xb94d3;
        }
        0xb94d3 => {
            // block b94d3
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94d3 jmp 0xb94eb
            pc = 0xb94eb; continue;
        }
        0xb94d5 => {
            // block b94d5
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94d5 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b94d7 mov rbx,r13
            rbx = r13;
            // b94da jmp 0xb94f2
            pc = 0xb94f2; continue;
        }
        0xb94dc => {
            // block b94dc
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94dc mov rsi,rbx
            rsi = rbx;
            pc = 0xb94df;
        }
        0xb94df => {
            // block b94df
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94df mov rcx,QWORD PTR [rsi+0x10]
            rcx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // b94e3 cmp QWORD PTR [rcx],rsi
            fl = sub::<64>(m.r64(rcx) as u64, rsi).1;
            // b94e6 mov rsi,rcx
            rsi = rcx;
            // b94e9 jne 0xb94df
            if crate::cov::br(fl.ne()) { pc = 0xb94df; continue; }
            pc = 0xb94eb;
        }
        0xb94eb => {
            // block b94eb
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94eb mov sil,BYTE PTR [rbx+0x19]
            rsi = (rsi & !0xff) | ((m.r8(rbx.wrapping_add(0x19)) as u64) as u8 as u64);
            // b94ef mov rbx,rcx
            rbx = rcx;
            pc = 0xb94f2;
        }
        0xb94f2 => {
            // block b94f2
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94f2 movzx esi,sil
            rsi = ((rsi as u8 as u64)) as u32 as u64;
            // b94f6 lea ecx,[rax*8+0x0]
            rcx = (rax.wrapping_mul(8)) as u32 as u64;
            // b94fd shl esi,cl
            { let (r, f) = shl::<32>((rsi as u32 as u64), (rcx as u8 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // b94ff or edx,esi
            { let r = (rdx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b9501 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // b9503 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // b9506 jne 0xb94ba
            if crate::cov::br(fl.ne()) { pc = 0xb94ba; continue; }
            pc = 0xb9508;
        }
        0xb9508 => {
            // block b9508
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9508 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9508 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b950b lea esi,[r12+r15*1]
            rsi = (r12.wrapping_add(r15)) as u32 as u64;
            // b950f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b9514 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9517 mov rdi,r14
            rdi = r14;
            // b951a call QWORD PTR [rax+0x30]
            { m.outregs(0xb951d, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b951d inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // b9520 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // b9522 cmp r12d,0x5
            fl = sub::<32>((r12 as u32 as u64), 0x5u64).1;
            // b9526 jne 0xb94b6
            if crate::cov::br(fl.ne()) { pc = 0xb94b6; continue; }
            pc = 0xb9528;
        }
        0xb9528 => {
            // block b9528
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9528 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9528 jmp 0xb952c
            pc = 0xb952c; continue;
        }
        0xb952a => {
            // block b952a
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b952a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b952a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb952c;
        }
        0xb952c => {
            // block b952c
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b952c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b952c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b9530 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9531 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b9533 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b9535 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9537 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9539 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b953a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x33a14 size=244
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00033a14<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x33a14;
    loop {
        match pc {
        0x33a14 => {
            // block 33a14
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a14 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 33a15 mov rbp,rsp
            rbp = rsp;
            // 33a18 mov r8,rdx
            r8 = rdx;
            // 33a1b mov BYTE PTR [rcx],0x0
            m.w8(rcx, (0x0u64) as u8);
            // 33a1e mov DWORD PTR [r8],0x0
            m.w32(r8, (0x0u64) as u32);
            // 33a25 mov edx,DWORD PTR [rdi]
            rdx = (m.r32(rdi) as u64) as u32 as u64;
            // 33a27 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 33a29 je 0x33a7b
            if crate::cov::br(fl.e()) { pc = 0x33a7b; continue; }
            pc = 0x33a2b;
        }
        0x33a2b => {
            // block 33a2b
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a2b cmp DWORD PTR [rdi+0x4],0x0
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 33a2f je 0x33a7b
            if crate::cov::br(fl.e()) { pc = 0x33a7b; continue; }
            pc = 0x33a31;
        }
        0x33a31 => {
            // block 33a31
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a31 cmp WORD PTR [rdi+0x34],0x0
            fl = sub::<16>(m.r16(rdi.wrapping_add(0x34)) as u64, 0x0u64).1;
            // 33a36 je 0x33a7b
            if crate::cov::br(fl.e()) { pc = 0x33a7b; continue; }
            pc = 0x33a38;
        }
        0x33a38 => {
            // block 33a38
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a38 cmp DWORD PTR [rdi+0x24],0x0
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 33a3c je 0x33a7b
            if crate::cov::br(fl.e()) { pc = 0x33a7b; continue; }
            pc = 0x33a3e;
        }
        0x33a3e => {
            // block 33a3e
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a3e mov r9d,DWORD PTR [rdi+0x10]
            r9 = (m.r32(rdi.wrapping_add(0x10)) as u64) as u32 as u64;
            // 33a42 cmp r9,0xb
            fl = sub::<64>(r9, 0xbu64).1;
            // 33a46 ja 0x33a7b
            if crate::cov::br(fl.a()) { pc = 0x33a7b; continue; }
            pc = 0x33a48;
        }
        0x33a48 => {
            // block 33a48
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a48 mov edi,DWORD PTR [rdi+0x1c]
            rdi = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 33a4b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33a4d cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // 33a50 ja 0x33a7d
            if crate::cov::br(fl.a()) { pc = 0x33a7d; continue; }
            pc = 0x33a52;
        }
        0x33a52 => {
            // block 33a52
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a52 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // 33a54 jbe 0x33a7d
            if crate::cov::br(fl.be()) { pc = 0x33a7d; continue; }
            pc = 0x33a56;
        }
        0x33a56 => {
            // block 33a56
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a56 lea eax,[r9-0x1]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33a5a cmp eax,0xb
            fl = sub::<32>((rax as u32 as u64), 0xbu64).1;
            // 33a5d jae 0x33a69
            if crate::cov::br(fl.ae()) { pc = 0x33a69; continue; }
            pc = 0x33a5f;
        }
        0x33a5f => {
            // block 33a5f
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a5f mov edx,0x7a9
            rdx = (0x7a9u64) as u32 as u64;
            // 33a64 bt edx,eax
            fl.cf = bit::<32>((rdx as u32 as u64), (rax as u32 as u64));
            // 33a67 jb 0x33a7f
            if crate::cov::br(fl.b()) { pc = 0x33a7f; continue; }
            pc = 0x33a69;
        }
        0x33a69 => {
            // block 33a69
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a69 cmp r9d,0x2
            fl = sub::<32>((r9 as u32 as u64), 0x2u64).1;
            // 33a6d sete dl
            rdx = (rdx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 33a70 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 33a72 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33a75 xor al,dl
            { let r = (rax as u8 as u64) ^ (rdx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 33a77 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // 33a79 jmp 0x33a81
            pc = 0x33a81; continue;
        }
        0x33a7b => {
            // block 33a7b
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a7b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x33a7d;
        }
        0x33a7d => {
            // block 33a7d
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a7d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 33a7e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x33a7f => {
            // block 33a7f
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a7f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x33a81;
        }
        0x33a81 => {
            // block 33a81
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a81 lea ecx,[r9-0x1]
            rcx = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33a85 cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 33a88 jae 0x33a9a
            if crate::cov::br(fl.ae()) { pc = 0x33a9a; continue; }
            pc = 0x33a8a;
        }
        0x33a8a => {
            // block 33a8a
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a8a mov r10d,0x1
            r10 = (0x1u64) as u32 as u64;
            // 33a90 mov edx,0x3a9
            rdx = (0x3a9u64) as u32 as u64;
            // 33a95 bt edx,ecx
            fl.cf = bit::<32>((rdx as u32 as u64), (rcx as u32 as u64));
            // 33a98 jb 0x33aa8
            if crate::cov::br(fl.b()) { pc = 0x33aa8; continue; }
            pc = 0x33a9a;
        }
        0x33a9a => {
            // block 33a9a
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a9a cmp r9d,0xb
            fl = sub::<32>((r9 as u32 as u64), 0xbu64).1;
            // 33a9e setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 33aa1 movzx r10d,cl
            r10 = ((rcx as u8 as u64)) as u32 as u64;
            // 33aa5 inc r10d
            { let (r, f) = inc::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            pc = 0x33aa8;
        }
        0x33aa8 => {
            // block 33aa8
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33aa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33aa8 cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // 33aab je 0x33ac4
            if crate::cov::br(fl.e()) { pc = 0x33ac4; continue; }
            pc = 0x33aad;
        }
        0x33aad => {
            // block 33aad
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33aad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33aad cmp edi,0x1
            fl = sub::<32>((rdi as u32 as u64), 0x1u64).1;
            // 33ab0 jne 0x33ad6
            if crate::cov::br(fl.ne()) { pc = 0x33ad6; continue; }
            pc = 0x33ab2;
        }
        0x33ab2 => {
            // block 33ab2
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33ab2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ab2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 33ab4 lea rax,[rip+0xcc495] # 0xfff50
            rax = m.env.data_va(83741);
            // 33abb lea rcx,[rip+0xcc44e] # 0xfff10
            rcx = m.env.data_va(83677);
            // 33ac2 jmp 0x33aec
            pc = 0x33aec; continue;
        }
        0x33ac4 => {
            // block 33ac4
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33ac4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ac4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 33ac6 lea rax,[rip+0xcc503] # 0xfffd0
            rax = m.env.data_va(83869);
            // 33acd lea rcx,[rip+0xcc4bc] # 0xfff90
            rcx = m.env.data_va(83805);
            // 33ad4 jmp 0x33aec
            pc = 0x33aec; continue;
        }
        0x33ad6 => {
            // block 33ad6
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33ad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ad6 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 33ad8 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // 33ada jne 0x33af4
            if crate::cov::br(fl.ne()) { pc = 0x33af4; continue; }
            pc = 0x33adc;
        }
        0x33adc => {
            // block 33adc
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33adc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33adc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 33ade lea rax,[rip+0xcc3eb] # 0xffed0
            rax = m.env.data_va(83613);
            // 33ae5 lea rcx,[rip+0xcc3a4] # 0xffe90
            rcx = m.env.data_va(83549);
            pc = 0x33aec;
        }
        0x33aec => {
            // block 33aec
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33aec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33aec cmove rcx,rax
            if fl.e() { rcx = rax; }
            // 33af0 mov ecx,DWORD PTR [rcx+r9*4]
            rcx = (m.r32(rcx.wrapping_add(r9.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x33af4;
        }
        0x33af4 => {
            // block 33af4
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33af4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33af4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33af6 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 33af8 div r10d
            { let (q, r) = div::<32>(rdx, rax, (r10 as u32 as u64)); rax = q; rdx = r; }
            // 33afb add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 33afd mov DWORD PTR [r8],eax
            m.w32(r8, ((rax as u32 as u64)) as u32);
            // 33b00 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 33b02 jmp 0x33a7d
            pc = 0x33a7d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9356 size=244
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9356<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb9356;
    loop {
        match pc {
        0xb9356 => {
            // block b9356
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9356 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9356 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9357 mov rbp,rsp
            rbp = rsp;
            // b935a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b935c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b935e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b9360 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b9362 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9363 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b9367 mov r13,rdx
            r13 = rdx;
            // b936a mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // b936d mov r14,rdi
            r14 = rdi;
            // b9370 lea rbx,[r13+0x8]
            rbx = r13.wrapping_add(0x8);
            // b9374 mov rsi,QWORD PTR [r13+0x8]
            rsi = m.r64(r13.wrapping_add(0x8)) as u64;
            // b9378 mov rdi,r13
            rdi = r13;
            // b937b call 0xebcf0
            { m.outregs(0xb9380, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x4bd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9380 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // b9388 mov QWORD PTR [r13+0x0],rbx
            m.w64(r13, (rbx) as u64);
            // b938c mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // b9394 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9398 call 0xcbc54
            { m.outregs(0xb939d, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcbc54); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b939d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b939f je 0xb9438
            if crate::cov::br(fl.e()) { pc = 0xb9438; continue; }
            pc = 0xb93a5;
        }
        0xb93a5 => {
            // block b93a5
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93a5 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b93a9 call 0xcbe98
            { m.outregs(0xb93ae, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b93ae test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b93b0 je 0xb9438
            if crate::cov::br(fl.e()) { pc = 0xb9438; continue; }
            pc = 0xb93b6;
        }
        0xb93b6 => {
            // block b93b6
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93b6 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b93ba call 0xd0ac8
            { m.outregs(0xb93bf, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b93bf xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // b93c2 movzx ecx,r12w
            rcx = ((r12 as u16 as u64)) as u32 as u64;
            // b93c6 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b93c9 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b93cb mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // b93d0 jbe 0xb943a
            if crate::cov::br(fl.be()) { pc = 0xb943a; continue; }
            pc = 0xb93d2;
        }
        0xb93d2 => {
            // block b93d2
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93d2 lea rax,[rip+0x495d7] # 0x1029b0
            rax = m.env.data_va(94589);
            // b93d9 mov eax,DWORD PTR [rax+rcx*4]
            rax = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b93dc mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b93e0 lea r12,[rbp-0x2d]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd3);
            pc = 0xb93e4;
        }
        0xb93e4 => {
            // block b93e4
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93e4 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // b93eb mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b93ee mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b93f2 lea esi,[r15+rcx*1]
            rsi = (r15.wrapping_add(rcx)) as u32 as u64;
            // b93f6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b93f8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b93fd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9400 mov rdi,r14
            rdi = r14;
            // b9403 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // b9407 call QWORD PTR [rax+0x38]
            { m.outregs(0xb940a, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb940a;
        }
        0xb940a => {
            // block b940a
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b940a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b940a mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // b940d mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // b940f shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // b9411 mov BYTE PTR [rbp-0x2d],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3), ((rax as u8 as u64)) as u8);
            // b9414 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b9416 je 0xb9423
            if crate::cov::br(fl.e()) { pc = 0xb9423; continue; }
            pc = 0xb9418;
        }
        0xb9418 => {
            // block b9418
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9418 mov rdi,r13
            rdi = r13;
            // b941b mov rsi,r12
            rsi = r12;
            // b941e call 0xebce4
            { m.outregs(0xb9423, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x4b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb9423;
        }
        0xb9423 => {
            // block b9423
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9423 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9423 add ebx,0x8
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x8u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b9426 cmp ebx,0x20
            fl = sub::<32>((rbx as u32 as u64), 0x20u64).1;
            // b9429 jne 0xb940a
            if crate::cov::br(fl.ne()) { pc = 0xb940a; continue; }
            pc = 0xb942b;
        }
        0xb942b => {
            // block b942b
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b942b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b942b inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // b942e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // b9430 cmp r15d,0x5
            fl = sub::<32>((r15 as u32 as u64), 0x5u64).1;
            // b9434 jne 0xb93e4
            if crate::cov::br(fl.ne()) { pc = 0xb93e4; continue; }
            pc = 0xb9436;
        }
        0xb9436 => {
            // block b9436
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9436 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9436 jmp 0xb943a
            pc = 0xb943a; continue;
        }
        0xb9438 => {
            // block b9438
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b9438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9438 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb943a;
        }
        0xb943a => {
            // block b943a
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b943a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b943a add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b943e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b943f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b9441 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b9443 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9445 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9447 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9448 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x9046 size=260
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00009046<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x9046;
    loop {
        match pc {
        0x9046 => {
            // block 9046
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 9046 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9046 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 9047 mov rbp,rsp
            rbp = rsp;
            // 904a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 904c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 904e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 9050 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 9052 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9053 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 9057 mov r14,rdi
            r14 = rdi;
            // 905a lea rsi,[rip+0x120f42] # 0x129fa3
            rsi = m.env.data_va(255856);
            // 9061 lea rbx,[rbp-0x58]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9065 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 906a mov rdi,rbx
            rdi = rbx;
            // 906d call 0xebfe4
            { m.outregs(0x9072, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9072 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9076 mov rsi,rbx
            rsi = rbx;
            // 9079 call 0x68488
            { m.outregs(0x907e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x68488); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 907e lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9082 call 0xec032
            { m.outregs(0x9087, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9087 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // 908b call 0xceac8
            { m.outregs(0x9090, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xceac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9090 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 9093 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 9097 lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 909b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 909e cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 90a1 je 0x90f7
            if crate::cov::br(fl.e()) { pc = 0x90f7; continue; }
            pc = 0x90a3;
        }
        0x90a3 => {
            // block 90a3
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90a3 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x90a6;
        }
        0x90a6 => {
            // block 90a6
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90a6 mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 90a9 cmp esi,r12d
            fl = sub::<32>((rsi as u32 as u64), (r12 as u32 as u64)).1;
            // 90ac ja 0x90cd
            if crate::cov::br(fl.a()) { pc = 0x90cd; continue; }
            pc = 0x90ae;
        }
        0x90ae => {
            // block 90ae
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90ae mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 90b1 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 90b5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 90b7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 90bc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 90bf mov rdi,r14
            rdi = r14;
            // 90c2 call rax
            { m.outregs(0x90c4, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 90c4 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 90c7 xor eax,0x1
            { let r = (rax as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 90ca add r13d,eax
            { let (r, f) = add::<32>((r13 as u32 as u64), (rax as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x90cd;
        }
        0x90cd => {
            // block 90cd
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90cd mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 90d1 test rax,rax
            fl = logic::<64>(rax & rax);
            // 90d4 je 0x90e3
            if crate::cov::br(fl.e()) { pc = 0x90e3; continue; }
            pc = 0x90d6;
        }
        0x90d6 => {
            // block 90d6
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90d6 mov rbx,rax
            rbx = rax;
            // 90d9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 90dc test rax,rax
            fl = logic::<64>(rax & rax);
            // 90df jne 0x90d6
            if crate::cov::br(fl.ne()) { pc = 0x90d6; continue; }
            pc = 0x90e1;
        }
        0x90e1 => {
            // block 90e1
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90e1 jmp 0x90f2
            pc = 0x90f2; continue;
        }
        0x90e3 => {
            // block 90e3
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90e3 mov rax,rbx
            rax = rbx;
            pc = 0x90e6;
        }
        0x90e6 => {
            // block 90e6
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90e6 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 90ea cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 90ed mov rax,rbx
            rax = rbx;
            // 90f0 jne 0x90e6
            if crate::cov::br(fl.ne()) { pc = 0x90e6; continue; }
            pc = 0x90f2;
        }
        0x90f2 => {
            // block 90f2
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90f2 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 90f5 jne 0x90a6
            if crate::cov::br(fl.ne()) { pc = 0x90a6; continue; }
            pc = 0x90f7;
        }
        0x90f7 => {
            // block 90f7
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 90f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 90f7 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 90fa sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 90fd mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 9101 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9105 call 0xebcd2
            { m.outregs(0x910a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 910a mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 910c add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 9110 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9111 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 9113 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 9115 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9117 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 9119 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 911a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x911b => {
            // block 911b
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 911b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 911b mov rbx,rax
            rbx = rax;
            pc = 0x911e;
        }
        0x911e => {
            // block 911e
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 911e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 911e mov rsi,QWORD PTR [r15]
            rsi = m.r64(r15) as u64;
            // 9121 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9125 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x912a;
        }
        0x912a => {
            // block 912a
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 912a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 912a mov rdi,rbx
            rdi = rbx;
            // 912d call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9132 mov rbx,rax
            rbx = rax;
            // 9135 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9139 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 913e jmp 0x912a
            pc = 0x912a; continue;
        }
        0x9140 => {
            // block 9140
            #[cfg(feature = "trace")] eprintln!("f_00009046 pc 9140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9140 mov rbx,rax
            rbx = rax;
            // 9143 lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 9147 jmp 0x911e
            pc = 0x911e; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6051c size=263
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006051c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6051c;
    loop {
        match pc {
        0x6051c => {
            // block 6051c
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 6051c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6051c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6051d mov rbp,rsp
            rbp = rsp;
            // 60520 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 60522 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 60524 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 60526 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 60528 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 60529 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 6052d mov r14,rsi
            r14 = rsi;
            // 60530 mov rbx,rdi
            rbx = rdi;
            // 60533 lea rsi,[rip+0xc9f1c] # 0x12a456
            rsi = m.env.data_va(257059);
            // 6053a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 6053f call 0xebb88
            { m.outregs(0x60544, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60544 mov rsi,QWORD PTR [r14+0x10]
            rsi = m.r64(r14.wrapping_add(0x10)) as u64;
            // 60548 mov rdi,rax
            rdi = rax;
            // 6054b call 0xec0a4
            { m.outregs(0x60550, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60550 lea rsi,[rip+0xceb2f] # 0x12f086
            rsi = m.env.data_va(276563);
            // 60557 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 6055c mov rdi,rax
            rdi = rax;
            // 6055f call 0xebb88
            { m.outregs(0x60564, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60564 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // 60567 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 6056b cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 6056e je 0x605ec
            if crate::cov::br(fl.e()) { pc = 0x605ec; continue; }
            pc = 0x60570;
        }
        0x60570 => {
            // block 60570
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 60570 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60570 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 60574 jmp 0x6058d
            pc = 0x6058d; continue;
        }
        0x60576 => {
            // block 60576
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 60576 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 60576 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 6057b mov rdi,rbx
            rdi = rbx;
            // 6057e lea rsi,[rip+0xc9f18] # 0x12a49d
            rsi = m.env.data_va(257130);
            // 60585 call 0xebb88
            { m.outregs(0x6058a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6058a mov r13,r12
            r13 = r12;
            pc = 0x6058d;
        }
        0x6058d => {
            // block 6058d
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 6058d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6058d mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 60591 mov rdi,r15
            rdi = r15;
            // 60594 call 0x60623
            { m.outregs(0x60599, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x60623); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60599 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 6059d test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 605a0 jne 0x605ab
            if crate::cov::br(fl.ne()) { pc = 0x605ab; continue; }
            pc = 0x605a2;
        }
        0x605a2 => {
            // block 605a2
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605a2 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 605a5 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 605a9 jmp 0x605b3
            pc = 0x605b3; continue;
        }
        0x605ab => {
            // block 605ab
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605ab mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 605af mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x605b3;
        }
        0x605b3 => {
            // block 605b3
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605b3 mov rdi,rbx
            rdi = rbx;
            // 605b6 call 0xebb88
            { m.outregs(0x605bb, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 605bb mov rdi,r15
            rdi = r15;
            // 605be call 0xec032
            { m.outregs(0x605c3, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 605c3 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 605c7 test rax,rax
            fl = logic::<64>(rax & rax);
            // 605ca je 0x605da
            if crate::cov::br(fl.e()) { pc = 0x605da; continue; }
            pc = 0x605cc;
        }
        0x605cc => {
            // block 605cc
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605cc mov r12,rax
            r12 = rax;
            // 605cf mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 605d3 test rax,rax
            fl = logic::<64>(rax & rax);
            // 605d6 jne 0x605cc
            if crate::cov::br(fl.ne()) { pc = 0x605cc; continue; }
            pc = 0x605d8;
        }
        0x605d8 => {
            // block 605d8
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605d8 jmp 0x605e7
            pc = 0x605e7; continue;
        }
        0x605da => {
            // block 605da
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605da mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // 605de cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // 605e2 mov r13,r12
            r13 = r12;
            // 605e5 jne 0x605da
            if crate::cov::br(fl.ne()) { pc = 0x605da; continue; }
            pc = 0x605e7;
        }
        0x605e7 => {
            // block 605e7
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605e7 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 605ea jne 0x60576
            if crate::cov::br(fl.ne()) { pc = 0x60576; continue; }
            pc = 0x605ec;
        }
        0x605ec => {
            // block 605ec
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 605ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 605ec lea rsi,[rip+0xc9e61] # 0x12a454
            rsi = m.env.data_va(257057);
            // 605f3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 605f8 mov rdi,rbx
            rdi = rbx;
            // 605fb call 0xebb88
            { m.outregs(0x60600, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 60600 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 60604 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 60605 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 60607 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 60609 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6060b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6060d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6060e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6060f => {
            // block 6060f
            #[cfg(feature = "trace")] eprintln!("f_0006051c pc 6060f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6060f mov rbx,rax
            rbx = rax;
            // 60612 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 60616 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6061b mov rdi,rbx
            rdi = rbx;
            // 6061e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x593a3 size=264
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000593a3<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x593a3;
    loop {
        match pc {
        0x593a3 => {
            // block 593a3
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 593a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 593a3 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 593a4 mov rbp,rsp
            rbp = rsp;
            // 593a7 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 593a9 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 593ab push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 593ad push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 593ae sub rsp,0x2050
            { let (r, f) = sub::<64>(rsp, 0x2050u64);
              rsp = r;
              fl = f; }
            // 593b5 mov r14,rcx
            r14 = rcx;
            // 593b8 mov r12,rdi
            r12 = rdi;
            // 593bb mov al,BYTE PTR [rbp+0x10]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0x10)) as u64) as u8 as u64);
            // 593be mov r15,QWORD PTR [rip+0xe7013] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 593c5 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 593c8 mov QWORD PTR [rbp-0x28],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (r15) as u64);
            // 593cc movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 593cf movzx ecx,r8b
            rcx = ((r8 as u8 as u64)) as u32 as u64;
            // 593d3 movzx r8d,r9b
            r8 = ((r9 as u8 as u64)) as u32 as u64;
            // 593d7 lea rdi,[rbp-0x2070]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df90);
            // 593de mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // 593e1 call 0x32c5c
            { m.outregs(0x593e6, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x32c5c); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 593e6 mov edx,DWORD PTR [rbp-0x206c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df94)) as u64) as u32 as u64;
            // 593ec test dx,dx
            fl = logic::<16>((rdx as u16 as u64) & (rdx as u16 as u64));
            // 593ef je 0x5943b
            if crate::cov::br(fl.e()) { pc = 0x5943b; continue; }
            pc = 0x593f1;
        }
        0x593f1 => {
            // block 593f1
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 593f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 593f1 movabs rax,0xffffffffffff
            rax = 0x0_ffff_ffff_ffffu64;
            // 593fb and rax,r14
            { let r = rax & r14; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 593fe mov rcx,rax
            rcx = rax;
            // 59401 shr rcx,0x10
            { let (r, f) = shr::<64>(rcx, 0x10u64, fl); rcx = r; fl = f; }
            // 59405 shr rax,0x20
            { let (r, f) = shr::<64>(rax, 0x20u64, fl); rax = r; fl = f; }
            // 59409 movzx esi,dx
            rsi = ((rdx as u16 as u64)) as u32 as u64;
            // 5940c add rsi,rsi
            { let (r, f) = add::<64>(rsi, rsi);
              rsi = r;
              fl = f; }
            // 5940f xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0x59411;
        }
        0x59411 => {
            // block 59411
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59411 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59411 mov WORD PTR [rbp+rdi*2-0x2030],r14w
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd0), ((r14 as u16 as u64)) as u16);
            // 5941a mov WORD PTR [rbp+rdi*2-0x202e],cx
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd2), ((rcx as u16 as u64)) as u16);
            // 59422 mov WORD PTR [rbp+rdi*2-0x202c],ax
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd4), ((rax as u16 as u64)) as u16);
            // 5942a mov WORD PTR [rbp+rdi*2-0x202a],cx
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd6), ((rcx as u16 as u64)) as u16);
            // 59432 add rdi,0x4
            { let (r, f) = add::<64>(rdi, 0x4u64);
              rdi = r;
              fl = f; }
            // 59436 cmp rdi,rsi
            fl = sub::<64>(rdi, rsi).1;
            // 59439 jl 0x59411
            if crate::cov::br(fl.l()) { pc = 0x59411; continue; }
            pc = 0x5943b;
        }
        0x5943b => {
            // block 5943b
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 5943b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5943b cmp DWORD PTR [rbp-0x2070],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df90)) as u64, 0x0u64).1;
            // 59442 je 0x59490
            if crate::cov::br(fl.e()) { pc = 0x59490; continue; }
            pc = 0x59444;
        }
        0x59444 => {
            // block 59444
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59444 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59444 lea rdi,[rbp-0x2030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            // 5944b mov rsi,r12
            rsi = r12;
            // 5944e call 0x59033
            { m.outregs(0x59453, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x59033); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59453 cmp DWORD PTR [rbp-0x2070],0x2
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df90)) as u64, 0x2u64).1;
            // 5945a jb 0x59490
            if crate::cov::br(fl.b()) { pc = 0x59490; continue; }
            pc = 0x5945c;
        }
        0x5945c => {
            // block 5945c
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 5945c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5945c mov bx,0x2
            rbx = (rbx & !0xffff) | ((0x2u64) as u16 as u64);
            // 59460 lea r14,[rbp-0x2030]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            pc = 0x59467;
        }
        0x59467 => {
            // block 59467
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59467 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59467 mov edx,DWORD PTR [rbp-0x206c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df94)) as u64) as u32 as u64;
            // 5946d mov eax,DWORD PTR [rbp-0x2068]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df98)) as u64) as u32 as u64;
            // 59473 lea r12,[r12+rax*4]
            r12 = r12.wrapping_add(rax.wrapping_mul(4));
            // 59477 mov rdi,r14
            rdi = r14;
            // 5947a mov rsi,r12
            rsi = r12;
            // 5947d call 0x59033
            { m.outregs(0x59482, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x59033); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59482 movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // 59485 lea ebx,[rbx+0x1]
            rbx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 59488 cmp eax,DWORD PTR [rbp-0x2070]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df90)) as u64).1;
            // 5948e jb 0x59467
            if crate::cov::br(fl.b()) { pc = 0x59467; continue; }
            pc = 0x59490;
        }
        0x59490 => {
            // block 59490
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59490 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59490 cmp r15,QWORD PTR [rbp-0x28]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64).1;
            // 59494 jne 0x594a6
            if crate::cov::br(fl.ne()) { pc = 0x594a6; continue; }
            pc = 0x59496;
        }
        0x59496 => {
            // block 59496
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59496 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59496 add rsp,0x2050
            { let (r, f) = add::<64>(rsp, 0x2050u64);
              rsp = r;
              fl = f; }
            // 5949d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5949e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 594a0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 594a2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 594a4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 594a5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x594a6 => {
            // block 594a6
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 594a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 594a6 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8d6b size=270
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8d6b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb8d6b;
    loop {
        match pc {
        0xb8d6b => {
            // block b8d6b
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8d6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8d6b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8d6c mov rbp,rsp
            rbp = rsp;
            // b8d6f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b8d71 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8d73 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b8d75 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b8d77 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8d78 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b8d7c mov r12,rsi
            r12 = rsi;
            // b8d7f mov r15,rdi
            r15 = rdi;
            // b8d82 cmp QWORD PTR [r12+0x10],0x0
            fl = sub::<64>(m.r64(r12.wrapping_add(0x10)) as u64, 0x0u64).1;
            // b8d88 je 0xb8e3f
            if crate::cov::br(fl.e()) { pc = 0xb8e3f; continue; }
            pc = 0xb8d8e;
        }
        0xb8d8e => {
            // block b8d8e
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8d8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8d8e mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8d92 lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // b8d97 cmp rcx,r13
            fl = sub::<64>(rcx, r13).1;
            // b8d9a je 0xb8e53
            if crate::cov::br(fl.e()) { pc = 0xb8e53; continue; }
            pc = 0xb8da0;
        }
        0xb8da0 => {
            // block b8da0
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8da0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8da0 mov r14,rcx
            r14 = rcx;
            // b8da3 jmp 0xb8dac
            pc = 0xb8dac; continue;
        }
        0xb8da5 => {
            // block b8da5
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8da5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8da5 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8da9 mov r14,rax
            r14 = rax;
            pc = 0xb8dac;
        }
        0xb8dac => {
            // block b8dac
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8dac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8dac cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // b8daf setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // b8db2 lea rsi,[rip+0x7ff0f] # 0x138cc8
            rsi = m.env.data_va(316565);
            // b8db9 lea rcx,[rip+0x716dd] # 0x12a49d
            rcx = m.env.data_va(257130);
            // b8dc0 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b8dc4 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // b8dc7 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // b8dca mov rdi,r15
            rdi = r15;
            // b8dcd call 0xebb88
            { m.outregs(0xb8dd2, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8dd2 mov rbx,rax
            rbx = rax;
            // b8dd5 mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b8dd9 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b8dde lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8de2 call 0x60c44
            { m.outregs(0xb8de7, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x60c44); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8de7 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // b8deb test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b8dee jne 0xb8df9
            if crate::cov::br(fl.ne()) { pc = 0xb8df9; continue; }
            pc = 0xb8df0;
        }
        0xb8df0 => {
            // block b8df0
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8df0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8df0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b8df3 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // b8df7 jmp 0xb8e01
            pc = 0xb8e01; continue;
        }
        0xb8df9 => {
            // block b8df9
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8df9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8df9 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b8dfd mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xb8e01;
        }
        0xb8e01 => {
            // block b8e01
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e01 mov rdi,rbx
            rdi = rbx;
            // b8e04 call 0xebb88
            { m.outregs(0xb8e09, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8e09 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8e0d call 0xec032
            { m.outregs(0xb8e12, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8e12 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // b8e16 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b8e19 je 0xb8e28
            if crate::cov::br(fl.e()) { pc = 0xb8e28; continue; }
            pc = 0xb8e1b;
        }
        0xb8e1b => {
            // block b8e1b
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e1b mov rax,rcx
            rax = rcx;
            // b8e1e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b8e21 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b8e24 jne 0xb8e1b
            if crate::cov::br(fl.ne()) { pc = 0xb8e1b; continue; }
            pc = 0xb8e26;
        }
        0xb8e26 => {
            // block b8e26
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e26 jmp 0xb8e34
            pc = 0xb8e34; continue;
        }
        0xb8e28 => {
            // block b8e28
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e28 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // b8e2c cmp QWORD PTR [rax],r14
            fl = sub::<64>(m.r64(rax) as u64, r14).1;
            // b8e2f mov r14,rax
            r14 = rax;
            // b8e32 jne 0xb8e28
            if crate::cov::br(fl.ne()) { pc = 0xb8e28; continue; }
            pc = 0xb8e34;
        }
        0xb8e34 => {
            // block b8e34
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e34 cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // b8e37 jne 0xb8da5
            if crate::cov::br(fl.ne()) { pc = 0xb8da5; continue; }
            pc = 0xb8e3d;
        }
        0xb8e3d => {
            // block b8e3d
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e3d jmp 0xb8e53
            pc = 0xb8e53; continue;
        }
        0xb8e3f => {
            // block b8e3f
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e3f lea rsi,[rip+0x7efb7] # 0x137dfd
            rsi = m.env.data_va(312778);
            // b8e46 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // b8e4b mov rdi,r15
            rdi = r15;
            // b8e4e call 0xebb88
            { m.outregs(0xb8e53, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb8e53;
        }
        0xb8e53 => {
            // block b8e53
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e53 mov rax,r15
            rax = r15;
            // b8e56 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b8e5a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8e5b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b8e5d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b8e5f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8e61 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b8e63 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8e64 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb8e65 => {
            // block b8e65
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e65 mov rbx,rax
            rbx = rax;
            // b8e68 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8e6c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8e71 mov rdi,rbx
            rdi = rbx;
            // b8e74 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1250d size=271
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001250d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1250d;
    loop {
        match pc {
        0x1250d => {
            // block 1250d
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1250d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1250d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1250e mov rbp,rsp
            rbp = rsp;
            // 12511 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12513 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12515 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12517 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12519 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1251a sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 1251e mov rbx,r8
            rbx = r8;
            // 12521 mov r13,rcx
            r13 = rcx;
            // 12524 mov QWORD PTR [rbp-0x38],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r13) as u64);
            // 12528 mov r12,rdx
            r12 = rdx;
            // 1252b mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // 1252f mov rcx,rdi
            rcx = rdi;
            // 12532 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 12535 mov r14,rbx
            r14 = rbx;
            // 12538 cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // 1253b je 0x12589
            if crate::cov::br(fl.e()) { pc = 0x12589; continue; }
            pc = 0x1253d;
        }
        0x1253d => {
            // block 1253d
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1253d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1253d cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 12540 je 0x125ab
            if crate::cov::br(fl.e()) { pc = 0x125ab; continue; }
            pc = 0x12542;
        }
        0x12542 => {
            // block 12542
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12542 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12542 mov eax,DWORD PTR [r12+0x18]
            rax = (m.r32(r12.wrapping_add(0x18)) as u64) as u32 as u64;
            // 12547 cmp eax,DWORD PTR [rcx+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx.wrapping_add(0x18)) as u64).1;
            // 1254a jge 0x12563
            if crate::cov::br(fl.ge()) { pc = 0x12563; continue; }
            pc = 0x1254c;
        }
        0x1254c => {
            // block 1254c
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1254c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1254c mov r13,rcx
            r13 = rcx;
            // 1254f mov rdi,r14
            rdi = r14;
            // 12552 mov rsi,r12
            rsi = r12;
            // 12555 call 0xe06fe
            { m.outregs(0x1255a, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1255a add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 1255e mov rcx,r13
            rcx = r13;
            // 12561 jmp 0x12578
            pc = 0x12578; continue;
        }
        0x12563 => {
            // block 12563
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12563 mov rdi,r14
            rdi = r14;
            // 12566 mov rsi,rcx
            rsi = rcx;
            // 12569 mov r13,rcx
            r13 = rcx;
            // 1256c call 0xe06fe
            { m.outregs(0x12571, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12571 mov rcx,r13
            rcx = r13;
            // 12574 add rcx,0x38
            { let (r, f) = add::<64>(rcx, 0x38u64);
              rcx = r;
              fl = f; }
            pc = 0x12578;
        }
        0x12578 => {
            // block 12578
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12578 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12578 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 1257b add r14,0x38
            { let (r, f) = add::<64>(r14, 0x38u64);
              r14 = r;
              fl = f; }
            // 1257f cmp rcx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 12583 mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 12587 jne 0x1253d
            if crate::cov::br(fl.ne()) { pc = 0x1253d; continue; }
            pc = 0x12589;
        }
        0x12589 => {
            // block 12589
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 12589 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12589 cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 1258c je 0x125dd
            if crate::cov::br(fl.e()) { pc = 0x125dd; continue; }
            pc = 0x1258e;
        }
        0x1258e => {
            // block 1258e
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1258e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1258e mov rdi,r14
            rdi = r14;
            // 12591 mov rsi,r12
            rsi = r12;
            // 12594 call 0xe06fe
            { m.outregs(0x12599, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12599 add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 1259d add r14,0x38
            { let (r, f) = add::<64>(r14, 0x38u64);
              r14 = r;
              fl = f; }
            // 125a1 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 125a4 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 125a7 jne 0x1258e
            if crate::cov::br(fl.ne()) { pc = 0x1258e; continue; }
            pc = 0x125a9;
        }
        0x125a9 => {
            // block 125a9
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125a9 jmp 0x125dd
            pc = 0x125dd; continue;
        }
        0x125ab => {
            // block 125ab
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125ab cmp rcx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 125af je 0x125dd
            if crate::cov::br(fl.e()) { pc = 0x125dd; continue; }
            pc = 0x125b1;
        }
        0x125b1 => {
            // block 125b1
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125b1 sub QWORD PTR [rbp-0x30],rcx
            { let (r, f) = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, rcx);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r) as u64);
              fl = f; }
            // 125b5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 125b8 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x125bb;
        }
        0x125bb => {
            // block 125bb
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125bb mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 125bf lea rdi,[r14+r13*1]
            rdi = r14.wrapping_add(r13);
            // 125c3 lea rsi,[rcx+r13*1]
            rsi = rcx.wrapping_add(r13);
            // 125c7 call 0xe06fe
            { m.outregs(0x125cc, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 125cc add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 125d0 dec r12
            { let (r, f) = dec::<64>(r12, fl); r12 = r; fl = f; }
            // 125d3 cmp QWORD PTR [rbp-0x30],r13
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, r13).1;
            // 125d7 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 125db jne 0x125bb
            if crate::cov::br(fl.ne()) { pc = 0x125bb; continue; }
            pc = 0x125dd;
        }
        0x125dd => {
            // block 125dd
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125dd add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 125e1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 125e2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 125e4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 125e6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 125e8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 125ea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 125eb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x125ec => {
            // block 125ec
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125ec jmp 0x125ee
            pc = 0x125ee; continue;
        }
        0x125ee => {
            // block 125ee
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125ee mov r14,rax
            r14 = rax;
            pc = 0x125f1;
        }
        0x125f1 => {
            // block 125f1
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125f1 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 125f4 je 0x1260c
            if crate::cov::br(fl.e()) { pc = 0x1260c; continue; }
            pc = 0x125f6;
        }
        0x125f6 => {
            // block 125f6
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125f6 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 125f9 je 0x1260c
            if crate::cov::br(fl.e()) { pc = 0x1260c; continue; }
            pc = 0x125fb;
        }
        0x125fb => {
            // block 125fb
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 125fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 125fb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 125fe mov rdi,rbx
            rdi = rbx;
            // 12601 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12603 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12607 dec r15
            { let (r, f) = dec::<64>(r15, fl); r15 = r; fl = f; }
            // 1260a jne 0x125fb
            if crate::cov::br(fl.ne()) { pc = 0x125fb; continue; }
            pc = 0x1260c;
        }
        0x1260c => {
            // block 1260c
            #[cfg(feature = "trace")] eprintln!("f_0001250d pc 1260c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1260c mov rdi,r14
            rdi = r14;
            // 1260f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12614 mov r14,rax
            r14 = rax;
            // 12617 sub r15,r12
            { let (r, f) = sub::<64>(r15, r12);
              r15 = r;
              fl = f; }
            // 1261a jmp 0x125f1
            pc = 0x125f1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x12824 size=272
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012824<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x12824;
    loop {
        match pc {
        0x12824 => {
            // block 12824
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12824 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12824 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12825 mov rbp,rsp
            rbp = rsp;
            // 12828 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1282a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1282c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1282e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12830 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12831 sub rsp,0x88
            { let (r, f) = sub::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // 12838 mov r14,rdx
            r14 = rdx;
            // 1283b mov r15,rsi
            r15 = rsi;
            // 1283e mov r13,rdi
            r13 = rdi;
            // 12841 lea r12,[rbp-0x60]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 12845 mov rbx,r15
            rbx = r15;
            // 12848 jmp 0x12851
            pc = 0x12851; continue;
        }
        0x1284a => {
            // block 1284a
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 1284a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1284a cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 1284d cmove r15,rbx
            if fl.e() { r15 = rbx; }
            pc = 0x12851;
        }
        0x12851 => {
            // block 12851
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12851 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12851 mov rdi,r12
            rdi = r12;
            // 12854 mov rsi,r13
            rsi = r13;
            // 12857 call 0xe06fe
            { m.outregs(0x1285c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1285c mov rdi,r13
            rdi = r13;
            // 1285f mov rsi,rbx
            rsi = rbx;
            // 12862 call 0xe064a
            { m.outregs(0x12867, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12867 mov rdi,rbx
            rdi = rbx;
            // 1286a mov rsi,r12
            rsi = r12;
            // 1286d call 0xe064a
            { m.outregs(0x12872, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12872 mov rdi,r12
            rdi = r12;
            // 12875 call 0xe05fa
            { m.outregs(0x1287a, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1287a add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 1287e add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12882 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 12885 jne 0x1284a
            if crate::cov::br(fl.ne()) { pc = 0x1284a; continue; }
            pc = 0x12887;
        }
        0x12887 => {
            // block 12887
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12887 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12887 mov QWORD PTR [rbp-0xa0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (r14) as u64);
            // 1288e cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 12891 je 0x128fe
            if crate::cov::br(fl.e()) { pc = 0x128fe; continue; }
            pc = 0x12893;
        }
        0x12893 => {
            // block 12893
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12893 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12893 lea r14,[rbp-0x98]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 1289a mov rbx,r13
            rbx = r13;
            // 1289d mov r12,r15
            r12 = r15;
            // 128a0 jmp 0x128b1
            pc = 0x128b1; continue;
        }
        0x128a2 => {
            // block 128a2
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128a2 cmp QWORD PTR [rbp-0xa8],rbx
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, rbx).1;
            // 128a9 cmove r15,r12
            if fl.e() { r15 = r12; }
            // 128ad add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            pc = 0x128b1;
        }
        0x128b1 => {
            // block 128b1
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128b1 lea rax,[r15-0x38]
            rax = r15.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 128b5 mov QWORD PTR [rbp-0xa8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rax) as u64);
            pc = 0x128bc;
        }
        0x128bc => {
            // block 128bc
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128bc mov rdi,r14
            rdi = r14;
            // 128bf mov rsi,rbx
            rsi = rbx;
            // 128c2 call 0xe06fe
            { m.outregs(0x128c7, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 128c7 mov rdi,rbx
            rdi = rbx;
            // 128ca mov rsi,r12
            rsi = r12;
            // 128cd call 0xe064a
            { m.outregs(0x128d2, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 128d2 mov rdi,r12
            rdi = r12;
            // 128d5 mov rsi,r14
            rsi = r14;
            // 128d8 call 0xe064a
            { m.outregs(0x128dd, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 128dd mov rdi,r14
            rdi = r14;
            // 128e0 call 0xe05fa
            { m.outregs(0x128e5, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 128e5 add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 128e9 cmp r12,QWORD PTR [rbp-0xa0]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64).1;
            // 128f0 jne 0x128a2
            if crate::cov::br(fl.ne()) { pc = 0x128a2; continue; }
            pc = 0x128f2;
        }
        0x128f2 => {
            // block 128f2
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128f2 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 128f6 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 128f9 mov r12,r15
            r12 = r15;
            // 128fc jne 0x128bc
            if crate::cov::br(fl.ne()) { pc = 0x128bc; continue; }
            pc = 0x128fe;
        }
        0x128fe => {
            // block 128fe
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128fe mov rax,r13
            rax = r13;
            // 12901 add rsp,0x88
            { let (r, f) = add::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // 12908 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 12909 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1290b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1290d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1290f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12911 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12912 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x12913 => {
            // block 12913
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12913 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12913 mov rbx,rax
            rbx = rax;
            // 12916 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1291a jmp 0x12926
            pc = 0x12926; continue;
        }
        0x1291c => {
            // block 1291c
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 1291c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1291c mov rbx,rax
            rbx = rax;
            // 1291f lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            pc = 0x12926;
        }
        0x12926 => {
            // block 12926
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12926 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12926 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1292b mov rdi,rbx
            rdi = rbx;
            // 1292e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x287ca size=276
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000287ca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x287ca;
    loop {
        match pc {
        0x287ca => {
            // block 287ca
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 287ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 287ca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 287cb mov rbp,rsp
            rbp = rsp;
            // 287ce push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 287d0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 287d2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 287d4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 287d6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 287d7 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 287db mov r12,rsi
            r12 = rsi;
            // 287de mov r14,rdi
            r14 = rdi;
            // 287e1 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 287e5 mov rsi,QWORD PTR [r12+0x10]
            rsi = m.r64(r12.wrapping_add(0x10)) as u64;
            // 287ea call 0xec0a4
            { m.outregs(0x287ef, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 287ef cmp QWORD PTR [r12+0x10],0x1
            fl = sub::<64>(m.r64(r12.wrapping_add(0x10)) as u64, 0x1u64).1;
            // 287f5 lea rcx,[rip+0x101ff6] # 0x12a7f2
            rcx = m.env.data_va(257983);
            // 287fc lea rsi,[rip+0x102000] # 0x12a803
            rsi = m.env.data_va(258000);
            // 28803 cmove rsi,rcx
            if fl.e() { rsi = rcx; }
            // 28807 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 2880a movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // 2880d or rdx,0x10
            { let r = rdx | 0x10u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 28811 mov rdi,rax
            rdi = rax;
            // 28814 call 0xebb88
            { m.outregs(0x28819, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28819 add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // 2881d cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28820 je 0x288b8
            if crate::cov::br(fl.e()) { pc = 0x288b8; continue; }
            pc = 0x28826;
        }
        0x28826 => {
            // block 28826
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 28826 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28826 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2882a lea r13,[rip+0x101c6c] # 0x12a49d
            r13 = m.env.data_va(257130);
            pc = 0x28831;
        }
        0x28831 => {
            // block 28831
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 28831 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28831 mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 28834 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28836 mov rdi,r15
            rdi = r15;
            // 28839 call 0x61aed
            { m.outregs(0x2883e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x61aed); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2883e movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 28842 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 28845 jne 0x28850
            if crate::cov::br(fl.ne()) { pc = 0x28850; continue; }
            pc = 0x28847;
        }
        0x28847 => {
            // block 28847
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 28847 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28847 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2884a lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 2884e jmp 0x28858
            pc = 0x28858; continue;
        }
        0x28850 => {
            // block 28850
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 28850 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28850 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 28854 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x28858;
        }
        0x28858 => {
            // block 28858
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 28858 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28858 mov rdi,r14
            rdi = r14;
            // 2885b call 0xebb88
            { m.outregs(0x28860, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28860 mov rdi,r15
            rdi = r15;
            // 28863 call 0xec032
            { m.outregs(0x28868, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28868 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2886c test rax,rax
            fl = logic::<64>(rax & rax);
            // 2886f je 0x2887e
            if crate::cov::br(fl.e()) { pc = 0x2887e; continue; }
            pc = 0x28871;
        }
        0x28871 => {
            // block 28871
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 28871 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28871 mov rbx,rax
            rbx = rax;
            // 28874 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28877 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2887a jne 0x28871
            if crate::cov::br(fl.ne()) { pc = 0x28871; continue; }
            pc = 0x2887c;
        }
        0x2887c => {
            // block 2887c
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 2887c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2887c jmp 0x2888d
            pc = 0x2888d; continue;
        }
        0x2887e => {
            // block 2887e
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 2887e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2887e mov rax,rbx
            rax = rbx;
            pc = 0x28881;
        }
        0x28881 => {
            // block 28881
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 28881 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28881 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 28885 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 28888 mov rax,rbx
            rax = rbx;
            // 2888b jne 0x28881
            if crate::cov::br(fl.ne()) { pc = 0x28881; continue; }
            pc = 0x2888d;
        }
        0x2888d => {
            // block 2888d
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 2888d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2888d cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28890 mov rsi,r13
            rsi = r13;
            // 28893 lea rax,[rip+0x11042e] # 0x138cc8
            rax = m.env.data_va(316565);
            // 2889a cmove rsi,rax
            if fl.e() { rsi = rax; }
            // 2889e setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 288a1 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 288a4 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 288a7 mov rdi,r14
            rdi = r14;
            // 288aa call 0xebb88
            { m.outregs(0x288af, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 288af cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 288b2 jne 0x28831
            if crate::cov::br(fl.ne()) { pc = 0x28831; continue; }
            pc = 0x288b8;
        }
        0x288b8 => {
            // block 288b8
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 288b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 288b8 mov rax,r14
            rax = r14;
            // 288bb add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 288bf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 288c0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 288c2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 288c4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 288c6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 288c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 288c9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x288ca => {
            // block 288ca
            #[cfg(feature = "trace")] eprintln!("f_000287ca pc 288ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 288ca mov rbx,rax
            rbx = rax;
            // 288cd lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 288d1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 288d6 mov rdi,rbx
            rdi = rbx;
            // 288d9 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x286b5 size=277
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000286b5<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x286b5;
    loop {
        match pc {
        0x286b5 => {
            // block 286b5
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 286b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 286b5 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 286b6 mov rbp,rsp
            rbp = rsp;
            // 286b9 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 286bb push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 286bd push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 286bf push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 286c1 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 286c2 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 286c6 mov r12,rsi
            r12 = rsi;
            // 286c9 mov r14,rdi
            r14 = rdi;
            // 286cc mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 286d0 mov rsi,QWORD PTR [r12+0x10]
            rsi = m.r64(r12.wrapping_add(0x10)) as u64;
            // 286d5 call 0xec0a4
            { m.outregs(0x286da, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 286da cmp QWORD PTR [r12+0x10],0x1
            fl = sub::<64>(m.r64(r12.wrapping_add(0x10)) as u64, 0x1u64).1;
            // 286e0 lea rcx,[rip+0x1020e6] # 0x12a7cd
            rcx = m.env.data_va(257946);
            // 286e7 lea rsi,[rip+0x1020f0] # 0x12a7de
            rsi = m.env.data_va(257963);
            // 286ee cmove rsi,rcx
            if fl.e() { rsi = rcx; }
            // 286f2 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 286f5 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 286f8 lea rdx,[rcx+rcx*2+0x10]
            rdx = rcx.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x10);
            // 286fd mov rdi,rax
            rdi = rax;
            // 28700 call 0xebb88
            { m.outregs(0x28705, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28705 add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // 28709 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 2870c je 0x287a4
            if crate::cov::br(fl.e()) { pc = 0x287a4; continue; }
            pc = 0x28712;
        }
        0x28712 => {
            // block 28712
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 28712 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28712 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 28716 lea r13,[rip+0x101d80] # 0x12a49d
            r13 = m.env.data_va(257130);
            pc = 0x2871d;
        }
        0x2871d => {
            // block 2871d
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 2871d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2871d mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 28720 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28722 mov rdi,r15
            rdi = r15;
            // 28725 call 0x61a4c
            { m.outregs(0x2872a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x61a4c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2872a movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 2872e test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 28731 jne 0x2873c
            if crate::cov::br(fl.ne()) { pc = 0x2873c; continue; }
            pc = 0x28733;
        }
        0x28733 => {
            // block 28733
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 28733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28733 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 28736 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 2873a jmp 0x28744
            pc = 0x28744; continue;
        }
        0x2873c => {
            // block 2873c
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 2873c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2873c mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 28740 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x28744;
        }
        0x28744 => {
            // block 28744
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 28744 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28744 mov rdi,r14
            rdi = r14;
            // 28747 call 0xebb88
            { m.outregs(0x2874c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2874c mov rdi,r15
            rdi = r15;
            // 2874f call 0xec032
            { m.outregs(0x28754, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28754 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 28758 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2875b je 0x2876a
            if crate::cov::br(fl.e()) { pc = 0x2876a; continue; }
            pc = 0x2875d;
        }
        0x2875d => {
            // block 2875d
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 2875d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2875d mov rbx,rax
            rbx = rax;
            // 28760 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28763 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28766 jne 0x2875d
            if crate::cov::br(fl.ne()) { pc = 0x2875d; continue; }
            pc = 0x28768;
        }
        0x28768 => {
            // block 28768
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 28768 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28768 jmp 0x28779
            pc = 0x28779; continue;
        }
        0x2876a => {
            // block 2876a
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 2876a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2876a mov rax,rbx
            rax = rbx;
            pc = 0x2876d;
        }
        0x2876d => {
            // block 2876d
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 2876d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2876d mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 28771 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 28774 mov rax,rbx
            rax = rbx;
            // 28777 jne 0x2876d
            if crate::cov::br(fl.ne()) { pc = 0x2876d; continue; }
            pc = 0x28779;
        }
        0x28779 => {
            // block 28779
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 28779 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28779 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 2877c mov rsi,r13
            rsi = r13;
            // 2877f lea rax,[rip+0x110542] # 0x138cc8
            rax = m.env.data_va(316565);
            // 28786 cmove rsi,rax
            if fl.e() { rsi = rax; }
            // 2878a setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 2878d movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 28790 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 28793 mov rdi,r14
            rdi = r14;
            // 28796 call 0xebb88
            { m.outregs(0x2879b, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2879b cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 2879e jne 0x2871d
            if crate::cov::br(fl.ne()) { pc = 0x2871d; continue; }
            pc = 0x287a4;
        }
        0x287a4 => {
            // block 287a4
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 287a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 287a4 mov rax,r14
            rax = r14;
            // 287a7 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 287ab pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 287ac pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 287ae pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 287b0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 287b2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 287b4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 287b5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x287b6 => {
            // block 287b6
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 287b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 287b6 mov rbx,rax
            rbx = rax;
            // 287b9 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 287bd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 287c2 mov rdi,rbx
            rdi = rbx;
            // 287c5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9113 size=283
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9113<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb9113;
    loop {
        match pc {
        0xb9113 => {
            // block b9113
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9113 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9113 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9114 mov rbp,rsp
            rbp = rsp;
            // b9117 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b9119 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b911b push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b911d push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b911f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9120 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b9121 mov r14,rsi
            r14 = rsi;
            // b9124 mov r13,rdi
            r13 = rdi;
            // b9127 mov rbx,QWORD PTR [r14]
            rbx = m.r64(r14) as u64;
            // b912a add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // b912e cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // b9131 je 0xb921c
            if crate::cov::br(fl.e()) { pc = 0xb921c; continue; }
            pc = 0xb9137;
        }
        0xb9137 => {
            // block b9137
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9137 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9137 lea r15,[rip+0x71337] # 0x12a475
            r15 = m.env.data_va(257090);
            // b913e jmp 0xb9157
            pc = 0xb9157; continue;
        }
        0xb9140 => {
            // block b9140
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9140 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // b9145 mov rdi,r13
            rdi = r13;
            // b9148 lea rsi,[rip+0x7134e] # 0x12a49d
            rsi = m.env.data_va(257130);
            // b914f call 0xebb88
            { m.outregs(0xb9154, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9154 mov rbx,r12
            rbx = r12;
            pc = 0xb9157;
        }
        0xb9157 => {
            // block b9157
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9157 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9157 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // b915c mov rdi,r13
            rdi = r13;
            // b915f mov rsi,r15
            rsi = r15;
            // b9162 call 0xebb88
            { m.outregs(0xb9167, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9167 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b916a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b916e mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b9172 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b9175 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b9178 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b917c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b917f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b9183 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b918b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b918e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b9192 mov QWORD PTR [rcx+rax*1+0x18],0x2
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x2u64) as u64);
            // b919b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b919e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b91a2 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b91ad movzx esi,BYTE PTR [rbx+0x19]
            rsi = (m.r8(rbx.wrapping_add(0x19)) as u64) as u32 as u64;
            // b91b1 mov rdi,rax
            rdi = rax;
            // b91b4 call 0xec0aa
            { m.outregs(0xb91b9, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b91b9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b91bc mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b91c0 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b91c4 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b91c7 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b91ca mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b91ce mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b91d1 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b91d5 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // b91e0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b91e3 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b91e7 and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b91ef mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b91f3 test rax,rax
            fl = logic::<64>(rax & rax);
            // b91f6 je 0xb9206
            if crate::cov::br(fl.e()) { pc = 0xb9206; continue; }
            pc = 0xb91f8;
        }
        0xb91f8 => {
            // block b91f8
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b91f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b91f8 mov r12,rax
            r12 = rax;
            // b91fb mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b91ff test rax,rax
            fl = logic::<64>(rax & rax);
            // b9202 jne 0xb91f8
            if crate::cov::br(fl.ne()) { pc = 0xb91f8; continue; }
            pc = 0xb9204;
        }
        0xb9204 => {
            // block b9204
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9204 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9204 jmp 0xb9213
            pc = 0xb9213; continue;
        }
        0xb9206 => {
            // block b9206
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9206 mov r12,QWORD PTR [rbx+0x10]
            r12 = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b920a cmp QWORD PTR [r12],rbx
            fl = sub::<64>(m.r64(r12) as u64, rbx).1;
            // b920e mov rbx,r12
            rbx = r12;
            // b9211 jne 0xb9206
            if crate::cov::br(fl.ne()) { pc = 0xb9206; continue; }
            pc = 0xb9213;
        }
        0xb9213 => {
            // block b9213
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b9213 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9213 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // b9216 jne 0xb9140
            if crate::cov::br(fl.ne()) { pc = 0xb9140; continue; }
            pc = 0xb921c;
        }
        0xb921c => {
            // block b921c
            #[cfg(feature = "trace")] eprintln!("f_000b9113 pc b921c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b921c mov rax,r13
            rax = r13;
            // b921f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b9223 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9224 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b9226 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b9228 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b922a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b922c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b922d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x27191 size=285
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00027191<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x27191;
    loop {
        match pc {
        0x27191 => {
            // block 27191
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27191 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27191 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 27192 mov rbp,rsp
            rbp = rsp;
            // 27195 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 27197 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 27199 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2719b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2719d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2719e sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 271a2 mov r14,rsi
            r14 = rsi;
            // 271a5 mov rbx,rdi
            rbx = rdi;
            // 271a8 mov rsi,QWORD PTR [r14+0x10]
            rsi = m.r64(r14.wrapping_add(0x10)) as u64;
            // 271ac call 0xec0a4
            { m.outregs(0x271b1, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 271b1 lea rsi,[rip+0x1032e2] # 0x12a49a
            rsi = m.env.data_va(257127);
            // 271b8 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 271bd mov rdi,rax
            rdi = rax;
            // 271c0 call 0xebb88
            { m.outregs(0x271c5, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 271c5 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // 271c8 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 271cc cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 271cf je 0x27277
            if crate::cov::br(fl.e()) { pc = 0x27277; continue; }
            pc = 0x271d5;
        }
        0x271d5 => {
            // block 271d5
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 271d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 271d5 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 271d9 jmp 0x271f2
            pc = 0x271f2; continue;
        }
        0x271db => {
            // block 271db
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 271db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 271db mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 271e0 mov rdi,rbx
            rdi = rbx;
            // 271e3 lea rsi,[rip+0x1032bb] # 0x12a4a5
            rsi = m.env.data_va(257138);
            // 271ea call 0xebb88
            { m.outregs(0x271ef, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 271ef mov r13,r12
            r13 = r12;
            pc = 0x271f2;
        }
        0x271f2 => {
            // block 271f2
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 271f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 271f2 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 271f6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 271f8 mov rdi,r15
            rdi = r15;
            // 271fb call 0x60974
            { m.outregs(0x27200, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x60974); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27200 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 27204 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 27207 jne 0x27212
            if crate::cov::br(fl.ne()) { pc = 0x27212; continue; }
            pc = 0x27209;
        }
        0x27209 => {
            // block 27209
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27209 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27209 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2720c lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 27210 jmp 0x2721a
            pc = 0x2721a; continue;
        }
        0x27212 => {
            // block 27212
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27212 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27212 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 27216 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x2721a;
        }
        0x2721a => {
            // block 2721a
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 2721a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2721a mov rdi,rbx
            rdi = rbx;
            // 2721d call 0xebb88
            { m.outregs(0x27222, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27222 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 27227 mov rdi,rax
            rdi = rax;
            // 2722a lea rsi,[rip+0x10326f] # 0x12a4a0
            rsi = m.env.data_va(257133);
            // 27231 call 0xebb88
            { m.outregs(0x27236, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27236 lea rsi,[r13+0x20]
            rsi = r13.wrapping_add(0x20);
            // 2723a mov rdi,rax
            rdi = rax;
            // 2723d call 0x26f9d
            { m.outregs(0x27242, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x26f9d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27242 mov rdi,r15
            rdi = r15;
            // 27245 call 0xec032
            { m.outregs(0x2724a, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2724a mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 2724e test rax,rax
            fl = logic::<64>(rax & rax);
            // 27251 je 0x27261
            if crate::cov::br(fl.e()) { pc = 0x27261; continue; }
            pc = 0x27253;
        }
        0x27253 => {
            // block 27253
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27253 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27253 mov r12,rax
            r12 = rax;
            // 27256 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2725a test rax,rax
            fl = logic::<64>(rax & rax);
            // 2725d jne 0x27253
            if crate::cov::br(fl.ne()) { pc = 0x27253; continue; }
            pc = 0x2725f;
        }
        0x2725f => {
            // block 2725f
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 2725f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2725f jmp 0x2726e
            pc = 0x2726e; continue;
        }
        0x27261 => {
            // block 27261
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27261 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27261 mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // 27265 cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // 27269 mov r13,r12
            r13 = r12;
            // 2726c jne 0x27261
            if crate::cov::br(fl.ne()) { pc = 0x27261; continue; }
            pc = 0x2726e;
        }
        0x2726e => {
            // block 2726e
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 2726e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2726e cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 27271 jne 0x271db
            if crate::cov::br(fl.ne()) { pc = 0x271db; continue; }
            pc = 0x27277;
        }
        0x27277 => {
            // block 27277
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 27277 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27277 lea rsi,[rip+0x1031d6] # 0x12a454
            rsi = m.env.data_va(257057);
            // 2727e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 27283 mov rdi,rbx
            rdi = rbx;
            // 27286 call 0xebb88
            { m.outregs(0x2728b, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2728b add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2728f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 27290 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 27292 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 27294 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 27296 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 27298 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 27299 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2729a => {
            // block 2729a
            #[cfg(feature = "trace")] eprintln!("f_00027191 pc 2729a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2729a mov rbx,rax
            rbx = rax;
            // 2729d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 272a1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 272a6 mov rdi,rbx
            rdi = rbx;
            // 272a9 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2346 size=288
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2346<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2346;
    loop {
        match pc {
        0xe2346 => {
            // block e2346
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2346 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2346 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2347 mov rbp,rsp
            rbp = rsp;
            // e234a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e234c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e234e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e2350 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2352 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2353 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e2357 mov r13,r9
            r13 = r9;
            // e235a mov ebx,r8d
            rbx = ((r8 as u32 as u64)) as u32 as u64;
            // e235d mov DWORD PTR [rbp-0x6c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rcx as u32 as u64)) as u32);
            // e2360 mov DWORD PTR [rbp-0x5c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rdx as u32 as u64)) as u32);
            // e2363 mov QWORD PTR [rbp-0x68],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rsi) as u64);
            // e2367 mov r15,rdi
            r15 = rdi;
            // e236a call 0xe49c4
            { m.outregs(0xe236f, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe49c4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e236f mov cl,0x1
            rcx = (rcx & !0xff) | ((0x1u64) as u8 as u64);
            // e2371 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e2373 je 0xe242c
            if crate::cov::br(fl.e()) { pc = 0xe242c; continue; }
            pc = 0xe2379;
        }
        0xe2379 => {
            // block e2379
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2379 mov rdi,r15
            rdi = r15;
            // e237c call 0xe494e
            { m.outregs(0xe2381, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe494e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2381 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2383 je 0xe242a
            if crate::cov::br(fl.e()) { pc = 0xe242a; continue; }
            pc = 0xe2389;
        }
        0xe2389 => {
            // block e2389
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2389 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2389 mov DWORD PTR [rbp-0x70],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), ((rbx as u32 as u64)) as u32);
            // e238c mov r12d,0xffffffe2
            r12 = (0xffffffe2u64) as u32 as u64;
            // e2392 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2396 lea r14,[rbp-0x58]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0xe239a;
        }
        0xe239a => {
            // block e239a
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e239a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e239a mov rdi,rbx
            rdi = rbx;
            // e239d mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e23a1 call 0xec026
            { m.outregs(0xe23a6, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23a6 mov rdi,r15
            rdi = r15;
            // e23a9 mov rsi,rbx
            rsi = rbx;
            // e23ac mov edx,DWORD PTR [rbp-0x5c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // e23af call 0xe2466
            { m.outregs(0xe23b4, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe2466); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23b4 mov rdi,rbx
            rdi = rbx;
            // e23b7 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e23b9 call 0xec032
            { m.outregs(0xe23be, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23be test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e23c0 je 0xe242a
            if crate::cov::br(fl.e()) { pc = 0xe242a; continue; }
            pc = 0xe23c2;
        }
        0xe23c2 => {
            // block e23c2
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e23c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e23c2 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e23c5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e23c8 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e23ca mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // e23cf call QWORD PTR [rax+0xbd0]
            { m.outregs(0xe23d5, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23d5 mov rdi,r14
            rdi = r14;
            // e23d8 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e23dc call 0xec026
            { m.outregs(0xe23e1, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23e1 mov rdi,r15
            rdi = r15;
            // e23e4 mov rsi,r14
            rsi = r14;
            // e23e7 mov edx,DWORD PTR [rbp-0x5c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // e23ea mov ecx,DWORD PTR [rbp-0x6c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // e23ed mov r8d,DWORD PTR [rbp-0x70]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // e23f1 mov r9,r13
            r9 = r13;
            // e23f4 mov rbx,r13
            rbx = r13;
            // e23f7 call 0xe26e8
            { m.outregs(0xe23fc, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe26e8); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e23fc mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // e23ff mov rdi,r14
            rdi = r14;
            // e2402 call 0xec032
            { m.outregs(0xe2407, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2407 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // e240a je 0xe2422
            if crate::cov::br(fl.e()) { pc = 0xe2422; continue; }
            pc = 0xe240c;
        }
        0xe240c => {
            // block e240c
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e240c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e240c cmp r13d,0x1
            fl = sub::<32>((r13 as u32 as u64), 0x1u64).1;
            // e2410 je 0xe243d
            if crate::cov::br(fl.e()) { pc = 0xe243d; continue; }
            pc = 0xe2412;
        }
        0xe2412 => {
            // block e2412
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2412 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2412 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // e2415 mov r13,rbx
            r13 = rbx;
            // e2418 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e241c jne 0xe239a
            if crate::cov::br(fl.ne()) { pc = 0xe239a; continue; }
            pc = 0xe2422;
        }
        0xe2422 => {
            // block e2422
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2422 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2422 mov rdi,r15
            rdi = r15;
            // e2425 call 0xe49f0
            { m.outregs(0xe242a, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe49f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe242a;
        }
        0xe242a => {
            // block e242a
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e242a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e242a xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe242c;
        }
        0xe242c => {
            // block e242c
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e242c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e242c mov al,cl
            rax = (rax & !0xff) | (((rcx as u8 as u64)) as u8 as u64);
            // e242e add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e2432 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2433 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2435 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e2437 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2439 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e243b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e243c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe243d => {
            // block e243d
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e243d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e243d mov rdi,r15
            rdi = r15;
            // e2440 call 0xe49f0
            { m.outregs(0xe2445, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe49f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2445 mov cl,0x1
            rcx = (rcx & !0xff) | ((0x1u64) as u8 as u64);
            // e2447 jmp 0xe242c
            pc = 0xe242c; continue;
        }
        0xe2449 => {
            // block e2449
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2449 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2449 mov rbx,rax
            rbx = rax;
            // e244c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2450 jmp 0xe2459
            pc = 0xe2459; continue;
        }
        0xe2452 => {
            // block e2452
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2452 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2452 mov rbx,rax
            rbx = rax;
            // e2455 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0xe2459;
        }
        0xe2459 => {
            // block e2459
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2459 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2459 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e245e mov rdi,rbx
            rdi = rbx;
            // e2461 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdfc90 size=292
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000dfc90<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdfc90;
    loop {
        match pc {
        0xdfc90 => {
            // block dfc90
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfc90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfc90 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dfc91 mov rbp,rsp
            rbp = rsp;
            // dfc94 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dfc96 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dfc98 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dfc9a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dfc9c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dfc9d sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // dfca1 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // dfca5 mov r12,rdi
            r12 = rdi;
            // dfca8 mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // dfcac mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // dfcb1 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // dfcb4 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // dfcb8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // dfcbb mov rax,rbx
            rax = rbx;
            // dfcbe shr rax,0x3d
            { let (r, f) = shr::<64>(rax, 0x3du64, fl); rax = r; fl = f; }
            // dfcc2 je 0xdfcd0
            if crate::cov::br(fl.e()) { pc = 0xdfcd0; continue; }
            pc = 0xdfcc4;
        }
        0xdfcc4 => {
            // block dfcc4
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfcc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfcc4 mov rdi,r12
            rdi = r12;
            // dfcc7 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0xdfcd0 => {
            // block dfcd0
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfcd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfcd0 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // dfcd5 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // dfcda mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // dfcde sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // dfce1 mov rcx,r15
            rcx = r15;
            // dfce4 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // dfce8 movabs rdx,0xffffffffffffffe
            rdx = 0xfff_ffff_ffff_fffeu64;
            // dfcf2 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // dfcf5 ja 0xdfd04
            if crate::cov::br(fl.a()) { pc = 0xdfd04; continue; }
            pc = 0xdfcf7;
        }
        0xdfcf7 => {
            // block dfcf7
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfcf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfcf7 sar r15,0x2
            { let (r, f) = sar::<64>(r15, 0x2u64, fl); r15 = r; fl = f; }
            // dfcfb cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // dfcfe cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // dfd02 jmp 0xdfd0e
            pc = 0xdfd0e; continue;
        }
        0xdfd04 => {
            // block dfd04
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfd04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfd04 movabs r15,0x1fffffffffffffff
            r15 = 0x1fff_ffff_ffff_ffffu64;
            pc = 0xdfd0e;
        }
        0xdfd0e => {
            // block dfd0e
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfd0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfd0e mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // dfd13 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // dfd17 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // dfd1a mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // dfd1e mov r13,r14
            r13 = r14;
            // dfd21 sar r13,0x3
            { let (r, f) = sar::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // dfd25 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // dfd2d mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // dfd31 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // dfd33 test r15,r15
            fl = logic::<64>(r15 & r15);
            // dfd36 je 0xdfd48
            if crate::cov::br(fl.e()) { pc = 0xdfd48; continue; }
            pc = 0xdfd38;
        }
        0xdfd38 => {
            // block dfd38
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfd38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfd38 lea rdi,[r15*8+0x0]
            rdi = r15.wrapping_mul(8);
            // dfd40 call 0xec146
            { m.outregs(0xdfd45, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfd45 mov rbx,rax
            rbx = rax;
            pc = 0xdfd48;
        }
        0xdfd48 => {
            // block dfd48
            #[cfg(feature = "trace")] eprintln!("f_000dfc90 pc dfd48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfd48 lea rax,[rbx+r15*8]
            rax = rbx.wrapping_add(r15.wrapping_mul(8));
            // dfd4c mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // dfd50 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // dfd54 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dfd57 mov QWORD PTR [rbx+r13*8],rax
            m.w64(rbx.wrapping_add(r13.wrapping_mul(8)), (rax) as u64);
            // dfd5b lea r13,[rbx+r13*8+0x8]
            r13 = rbx.wrapping_add(r13.wrapping_mul(8)).wrapping_add(0x8);
            // dfd60 mov rdi,rbx
            rdi = rbx;
            // dfd63 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // dfd67 mov rsi,r15
            rsi = r15;
            // dfd6a mov rdx,r14
            rdx = r14;
            // dfd6d call 0xec21e
            { m.outregs(0xdfd72, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfd72 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // dfd76 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // dfd7a mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // dfd7f mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // dfd83 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // dfd87 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // dfd8b mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // dfd90 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // dfd94 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // dfd98 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // dfd9c lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // dfda0 call 0xebedc
            { m.outregs(0xdfda5, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x6a9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfda5 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // dfda9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dfdaa pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dfdac pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dfdae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dfdb0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dfdb2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dfdb3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19dea size=294
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00019dea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x19dea;
    loop {
        match pc {
        0x19dea => {
            // block 19dea
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19dea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19dea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19deb mov rbp,rsp
            rbp = rsp;
            // 19dee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19df0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19df2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 19df4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 19df6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19df7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19df8 mov r12,rdx
            r12 = rdx;
            // 19dfb mov r15,rdi
            r15 = rdi;
            // 19dfe mov r14,r12
            r14 = r12;
            // 19e01 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 19e04 mov rbx,r14
            rbx = r14;
            // 19e07 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19e0b mov r13,QWORD PTR [r15]
            r13 = m.r64(r15) as u64;
            // 19e0e mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19e12 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19e15 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19e19 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19e1c jbe 0x19e63
            if crate::cov::br(fl.be()) { pc = 0x19e63; continue; }
            pc = 0x19e1e;
        }
        0x19e1e => {
            // block 19e1e
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e1e mov r12,rsi
            r12 = rsi;
            // 19e21 mov rdi,r15
            rdi = r15;
            // 19e24 call 0xeba2c
            { m.outregs(0x19e29, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19e29 mov rax,rbx
            rax = rbx;
            // 19e2c shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19e30 je 0x19e3a
            if crate::cov::br(fl.e()) { pc = 0x19e3a; continue; }
            pc = 0x19e32;
        }
        0x19e32 => {
            // block 19e32
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e32 mov rdi,r15
            rdi = r15;
            // 19e35 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x19e3a => {
            // block 19e3a
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e3a mov rsi,QWORD PTR [r15+0x10]
            rsi = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19e3e sub rsi,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rsi, m.r64(r15) as u64);
              rsi = r;
              fl = f; }
            // 19e41 mov rax,rsi
            rax = rsi;
            // 19e44 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19e48 movabs rcx,0x1ffffffffffffffe
            rcx = 0x1fff_ffff_ffff_fffeu64;
            // 19e52 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 19e55 ja 0x19e7c
            if crate::cov::br(fl.a()) { pc = 0x19e7c; continue; }
            pc = 0x19e57;
        }
        0x19e57 => {
            // block 19e57
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e57 sar rsi,1
            { let (r, f) = sar::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 19e5a cmp rsi,rbx
            fl = sub::<64>(rsi, rbx).1;
            // 19e5d cmovb rsi,rbx
            if fl.b() { rsi = rbx; }
            // 19e61 jmp 0x19e86
            pc = 0x19e86; continue;
        }
        0x19e63 => {
            // block 19e63
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e63 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19e67 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19e6a sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19e6e cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19e71 jbe 0x19ea3
            if crate::cov::br(fl.be()) { pc = 0x19ea3; continue; }
            pc = 0x19e73;
        }
        0x19e73 => {
            // block 19e73
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e73 lea rbx,[rsi+rax*4]
            rbx = rsi.wrapping_add(rax.wrapping_mul(4));
            // 19e77 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 19e7a jmp 0x19ea9
            pc = 0x19ea9; continue;
        }
        0x19e7c => {
            // block 19e7c
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e7c movabs rsi,0x3fffffffffffffff
            rsi = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x19e86;
        }
        0x19e86 => {
            // block 19e86
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e86 mov rdi,r15
            rdi = r15;
            // 19e89 call 0xeba44
            { m.outregs(0x19e8e, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x211); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19e8e mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19e92 mov rsi,r12
            rsi = r12;
            // 19e95 mov rdx,r14
            rdx = r14;
            // 19e98 call 0xec21e
            { m.outregs(0x19e9d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19e9d shl rbx,0x2
            { let (r, f) = shl::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19ea1 jmp 0x19eda
            pc = 0x19eda; continue;
        }
        0x19ea3 => {
            // block 19ea3
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ea3 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 19ea6 mov rbx,r12
            rbx = r12;
            pc = 0x19ea9;
        }
        0x19ea9 => {
            // block 19ea9
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ea9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ea9 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 19ead sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 19eb0 mov rdi,r13
            rdi = r13;
            // 19eb3 mov rdx,rbx
            rdx = rbx;
            // 19eb6 call 0xec224
            { m.outregs(0x19ebb, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19ebb mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19ebf test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 19ec2 je 0x19ee0
            if crate::cov::br(fl.e()) { pc = 0x19ee0; continue; }
            pc = 0x19ec4;
        }
        0x19ec4 => {
            // block 19ec4
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ec4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ec4 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 19ec8 sub r12,rsi
            { let (r, f) = sub::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 19ecb mov rbx,r12
            rbx = r12;
            // 19ece and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19ed2 mov rdx,r12
            rdx = r12;
            // 19ed5 call 0xec21e
            { m.outregs(0x19eda, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x19eda;
        }
        0x19eda => {
            // block 19eda
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19eda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19eda add QWORD PTR [r15+0x8],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rbx);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 19ede jmp 0x19f01
            pc = 0x19f01; continue;
        }
        0x19ee0 => {
            // block 19ee0
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ee0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ee0 and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19ee4 add r13,rbx
            { let (r, f) = add::<64>(r13, rbx);
              r13 = r;
              fl = f; }
            // 19ee7 cmp rdi,r13
            fl = sub::<64>(rdi, r13).1;
            // 19eea je 0x19f01
            if crate::cov::br(fl.e()) { pc = 0x19f01; continue; }
            pc = 0x19eec;
        }
        0x19eec => {
            // block 19eec
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19eec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19eec lea rax,[rdi-0x4]
            rax = rdi.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19ef0 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19ef3 not rax
            { let v = !rax; rax = v; }
            // 19ef6 and rax,0xfffffffffffffffc
            { let r = rax & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 19efa add rax,rdi
            { let (r, f) = add::<64>(rax, rdi);
              rax = r;
              fl = f; }
            // 19efd mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            pc = 0x19f01;
        }
        0x19f01 => {
            // block 19f01
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19f01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f01 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19f05 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19f06 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 19f08 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 19f0a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19f0c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 19f0e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19f0f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4982 size=300
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004982<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4982;
    loop {
        match pc {
        0x4982 => {
            // block 4982
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4982 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4982 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4983 mov rbp,rsp
            rbp = rsp;
            // 4986 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4988 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 498a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 498c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 498e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 498f sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4993 mov r12,rcx
            r12 = rcx;
            // 4996 mov r15,rdx
            r15 = rdx;
            // 4999 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 499b mov r14,rdi
            r14 = rdi;
            // 499e mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 49a5 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 49ac mov DWORD PTR [rbp-0x34],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0x0u64) as u32);
            // 49b3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49b6 call QWORD PTR [rax+0x1d28]
            { m.outregs(0x49bc, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49bc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 49be je 0x49c8
            if crate::cov::br(fl.e()) { pc = 0x49c8; continue; }
            pc = 0x49c0;
        }
        0x49c0 => {
            // block 49c0
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 49c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49c0 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 49c3 jmp 0x4a9b
            pc = 0x4a9b; continue;
        }
        0x49c8 => {
            // block 49c8
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 49c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49c8 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 49cc mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49cf mov ebx,ebx
            rbx = ((rbx as u32 as u64)) as u32 as u64;
            // 49d1 lea rcx,[rip+0xfa798] # 0xff170
            rcx = m.env.data_va(80189);
            // 49d8 mov r13d,DWORD PTR [rcx+rbx*4]
            r13 = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 49dc lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 49e0 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 49e3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 49e8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 49eb mov rdi,r14
            rdi = r14;
            // 49ee mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 49f1 call QWORD PTR [rax+0x38]
            { m.outregs(0x49f4, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49f4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 49f6 je 0x4a9b
            if crate::cov::br(fl.e()) { pc = 0x4a9b; continue; }
            pc = 0x49fc;
        }
        0x49fc => {
            // block 49fc
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 49fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49fc lea rax,[rip+0xfa79d] # 0xff1a0
            rax = m.env.data_va(80237);
            // 4a03 mov eax,DWORD PTR [rax+rbx*4]
            rax = (m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a06 test DWORD PTR [rbp-0x2c],eax
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & (rax as u32 as u64));
            // 4a09 je 0x4a31
            if crate::cov::br(fl.e()) { pc = 0x4a31; continue; }
            pc = 0x4a0b;
        }
        0x4a0b => {
            // block 4a0b
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a0b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a0e lea rcx,[rip+0xfa7bb] # 0xff1d0
            rcx = m.env.data_va(80285);
            // 4a15 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a18 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 4a1c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4a1f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4a24 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4a27 mov rdi,r14
            rdi = r14;
            // 4a2a call QWORD PTR [rax+0x38]
            { m.outregs(0x4a2d, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a2d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4a2f je 0x4a9b
            if crate::cov::br(fl.e()) { pc = 0x4a9b; continue; }
            pc = 0x4a31;
        }
        0x4a31 => {
            // block 4a31
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a31 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a34 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 4a38 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4a3b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4a40 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4a43 mov rdi,r14
            rdi = r14;
            // 4a46 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4a49 call QWORD PTR [rax+0x38]
            { m.outregs(0x4a4c, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a4c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4a4e je 0x4a9b
            if crate::cov::br(fl.e()) { pc = 0x4a9b; continue; }
            pc = 0x4a50;
        }
        0x4a50 => {
            // block 4a50
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a50 lea rax,[rip+0xfa7a9] # 0xff200
            rax = m.env.data_va(80333);
            // 4a57 mov eax,DWORD PTR [rax+rbx*4]
            rax = (m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a5a test DWORD PTR [rbp-0x2c],eax
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & (rax as u32 as u64));
            // 4a5d je 0x4a85
            if crate::cov::br(fl.e()) { pc = 0x4a85; continue; }
            pc = 0x4a5f;
        }
        0x4a5f => {
            // block 4a5f
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a5f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4a62 lea rcx,[rip+0xfa7c7] # 0xff230
            rcx = m.env.data_va(80381);
            // 4a69 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4a6c lea rdx,[rbp-0x34]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 4a70 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 4a73 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4a78 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4a7b mov rdi,r14
            rdi = r14;
            // 4a7e call QWORD PTR [rax+0x38]
            { m.outregs(0x4a81, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4a81 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4a83 je 0x4a9b
            if crate::cov::br(fl.e()) { pc = 0x4a9b; continue; }
            pc = 0x4a85;
        }
        0x4a85 => {
            // block 4a85
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a85 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 4a88 bswap eax
            rax = (rax as u32).swap_bytes() as u64;
            // 4a8a mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            // 4a8d mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // 4a90 bswap eax
            rax = (rax as u32).swap_bytes() as u64;
            // 4a92 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 4a96 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // 4a98 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x4a9b;
        }
        0x4a9b => {
            // block 4a9b
            #[cfg(feature = "trace")] eprintln!("f_00004982 pc 4a9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4a9b mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 4a9e add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4aa2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4aa3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4aa5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4aa7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4aa9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4aab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4aac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x12e88 size=300
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012e88<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x12e88;
    loop {
        match pc {
        0x12e88 => {
            // block 12e88
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12e88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12e88 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12e89 mov rbp,rsp
            rbp = rsp;
            // 12e8c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12e8e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12e90 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 12e92 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12e94 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12e95 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 12e99 mov r15,rdx
            r15 = rdx;
            // 12e9c mov r13,rsi
            r13 = rsi;
            // 12e9f mov r14,rdi
            r14 = rdi;
            // 12ea2 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 12ea6 mov rdi,r13
            rdi = r13;
            // 12ea9 call QWORD PTR [rax+0x18]
            { m.outregs(0x12eac, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12eac mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 12eb3 lea r12,[r13+0x248]
            r12 = r13.wrapping_add(0x248);
            // 12eba xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x12ebc;
        }
        0x12ebc => {
            // block 12ebc
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12ebc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ebc lea rsi,[r15+rbx*1]
            rsi = r15.wrapping_add(rbx);
            // 12ec0 mov rax,QWORD PTR [r13+0x250]
            rax = m.r64(r13.wrapping_add(0x250)) as u64;
            // 12ec7 cmp rax,QWORD PTR [r13+0x258]
            fl = sub::<64>(rax, m.r64(r13.wrapping_add(0x258)) as u64).1;
            // 12ece je 0x12edd
            if crate::cov::br(fl.e()) { pc = 0x12edd; continue; }
            pc = 0x12ed0;
        }
        0x12ed0 => {
            // block 12ed0
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12ed0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ed0 mov cl,BYTE PTR [rsi]
            rcx = (rcx & !0xff) | ((m.r8(rsi) as u64) as u8 as u64);
            // 12ed2 mov BYTE PTR [rax],cl
            m.w8(rax, ((rcx as u8 as u64)) as u8);
            // 12ed4 inc QWORD PTR [r13+0x250]
            { let (r, f) = inc::<64>(m.r64(r13.wrapping_add(0x250)) as u64, fl); m.w64(r13.wrapping_add(0x250), (r) as u64); fl = f; }
            // 12edb jmp 0x12ee5
            pc = 0x12ee5; continue;
        }
        0x12edd => {
            // block 12edd
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12edd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12edd mov rdi,r12
            rdi = r12;
            // 12ee0 call 0xebb0a
            { m.outregs(0x12ee5, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x2d7); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x12ee5;
        }
        0x12ee5 => {
            // block 12ee5
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12ee5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ee5 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 12ee8 cmp rbx,0xb7
            fl = sub::<64>(rbx, 0xb7u64).1;
            // 12eef jne 0x12ebc
            if crate::cov::br(fl.ne()) { pc = 0x12ebc; continue; }
            pc = 0x12ef1;
        }
        0x12ef1 => {
            // block 12ef1
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12ef1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12ef1 lea r15,[r13+0x2c8]
            r15 = r13.wrapping_add(0x2c8);
            // 12ef8 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 12efc lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 12f00 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 12f04 mov rsi,r13
            rsi = r13;
            // 12f07 call QWORD PTR [rax+0x90]
            { m.outregs(0x12f0d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x90)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12f0d test BYTE PTR [r13+0x2c8],0x1
            fl = logic::<8>(m.r8(r13.wrapping_add(0x2c8)) as u64 & 0x1u64);
            // 12f15 jne 0x12f1f
            if crate::cov::br(fl.ne()) { pc = 0x12f1f; continue; }
            pc = 0x12f17;
        }
        0x12f17 => {
            // block 12f17
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f17 mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 12f1d jmp 0x12f34
            pc = 0x12f34; continue;
        }
        0x12f1f => {
            // block 12f1f
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f1f mov rax,QWORD PTR [r13+0x2d8]
            rax = m.r64(r13.wrapping_add(0x2d8)) as u64;
            // 12f26 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 12f29 mov QWORD PTR [r13+0x2d0],0x0
            m.w64(r13.wrapping_add(0x2d0), (0x0u64) as u64);
            pc = 0x12f34;
        }
        0x12f34 => {
            // block 12f34
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f34 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 12f36 mov rdi,r15
            rdi = r15;
            // 12f39 call 0xec014
            { m.outregs(0x12f3e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12f3e mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 12f42 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 12f46 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 12f4a mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 12f4e mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 12f52 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 12f55 xorps xmm0,xmm0
            x0 = 0;
            // 12f58 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 12f5c mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 12f64 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 12f68 call 0xec032
            { m.outregs(0x12f6d, 0xd, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12f6d movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 12f71 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 12f73 jne 0x12f7a
            if crate::cov::br(fl.ne()) { pc = 0x12f7a; continue; }
            pc = 0x12f75;
        }
        0x12f75 => {
            // block 12f75
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f75 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 12f78 jmp 0x12f81
            pc = 0x12f81; continue;
        }
        0x12f7a => {
            // block 12f7a
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f7a mov rax,QWORD PTR [r13+0x2d0]
            rax = m.r64(r13.wrapping_add(0x2d0)) as u64;
            pc = 0x12f81;
        }
        0x12f81 => {
            // block 12f81
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f81 test rax,rax
            fl = logic::<64>(rax & rax);
            // 12f84 jne 0x12f8e
            if crate::cov::br(fl.ne()) { pc = 0x12f8e; continue; }
            pc = 0x12f86;
        }
        0x12f86 => {
            // block 12f86
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f86 mov BYTE PTR [r13+0x2e8],0x0
            m.w8(r13.wrapping_add(0x2e8), (0x0u64) as u8);
            pc = 0x12f8e;
        }
        0x12f8e => {
            // block 12f8e
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12f8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12f8e mov rdi,r14
            rdi = r14;
            // 12f91 mov rsi,r15
            rsi = r15;
            // 12f94 call 0xec026
            { m.outregs(0x12f99, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12f99 mov rax,r14
            rax = r14;
            // 12f9c add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 12fa0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 12fa1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 12fa3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 12fa5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 12fa7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12fa9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12faa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x12fab => {
            // block 12fab
            #[cfg(feature = "trace")] eprintln!("f_00012e88 pc 12fab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12fab mov rdi,rax
            rdi = rax;
            // 12fae call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6d9a size=302
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6d9a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd6d9a;
    loop {
        match pc {
        0xd6d9a => {
            // block d6d9a
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6d9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6d9a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6d9b mov rbp,rsp
            rbp = rsp;
            // d6d9e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d6da0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d6da2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d6da4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d6da6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6da7 sub rsp,0x298
            { let (r, f) = sub::<64>(rsp, 0x298u64);
              rsp = r;
              fl = f; }
            // d6dae mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // d6db1 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // d6db3 mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // d6db6 mov r15,rdi
            r15 = rdi;
            // d6db9 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // d6dbc xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d6dbf xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d6dc1 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6dc4 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d6dc6 call QWORD PTR [rbx+0x10]
            { m.outregs(0xd6dc9, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rbx.wrapping_add(0x10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6dc9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6dcb je 0xd6e9b
            if crate::cov::br(fl.e()) { pc = 0xd6e9b; continue; }
            pc = 0xd6dd1;
        }
        0xd6dd1 => {
            // block d6dd1
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6dd1 lea r12,[rbp-0x2b0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6dd8 mov rdi,r12
            rdi = r12;
            // d6ddb call 0xb6722
            { m.outregs(0xd6de0, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xb6722); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6de0 movzx edi,r13w
            rdi = ((r13 as u16 as u64)) as u32 as u64;
            // d6de4 mov rsi,r12
            rsi = r12;
            // d6de7 call 0x17202
            { m.outregs(0xd6dec, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x17202); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6dec mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // d6df3 mov rax,QWORD PTR [rax+0x100]
            rax = m.r64(rax.wrapping_add(0x100)) as u64;
            // d6dfa lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e01 call rax
            { m.outregs(0xd6e03, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6e03 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6e05 je 0xd6e68
            if crate::cov::br(fl.e()) { pc = 0xd6e68; continue; }
            pc = 0xd6e07;
        }
        0xd6e07 => {
            // block d6e07
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e07 mov DWORD PTR [rbp-0x2b4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c), (0x0u64) as u32);
            // d6e11 mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // d6e18 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d6e1c lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e23 lea rdx,[rbp-0x2b4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fd4c);
            // d6e2a mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // d6e2f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d6e34 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6e37 call rax
            { m.outregs(0xd6e39, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6e39 test BYTE PTR [rbp-0x2b4],0x4
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64 & 0x4u64);
            // d6e40 je 0xd6e68
            if crate::cov::br(fl.e()) { pc = 0xd6e68; continue; }
            pc = 0xd6e42;
        }
        0xd6e42 => {
            // block d6e42
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e42 mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // d6e49 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d6e4d lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e54 mov esi,0x40002
            rsi = (0x40002u64) as u32 as u64;
            // d6e59 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d6e5e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d6e63 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6e66 call rax
            { m.outregs(0xd6e68, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6e68;
        }
        0xd6e68 => {
            // block d6e68
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e68 mov rdi,r15
            rdi = r15;
            // d6e6b call 0xd69d6
            { m.outregs(0xd6e70, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd69d6); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6e70 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d6e73 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d6e77 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d6e79 mov ecx,0x30000
            rcx = (0x30000u64) as u32 as u64;
            // d6e7e mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // d6e84 mov rdi,r15
            rdi = r15;
            // d6e87 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d6e8a call rax
            { m.outregs(0xd6e8c, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6e8c mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d6e8f lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e96 call 0xb68ee
            { m.outregs(0xd6e9b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6e9b;
        }
        0xd6e9b => {
            // block d6e9b
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e9b mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // d6e9e add rsp,0x298
            { let (r, f) = add::<64>(rsp, 0x298u64);
              rsp = r;
              fl = f; }
            // d6ea5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6ea6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6ea8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d6eaa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6eac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d6eae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6eaf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd6eb0 => {
            // block d6eb0
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6eb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6eb0 mov rbx,rax
            rbx = rax;
            // d6eb3 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6eba call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6ebf mov rdi,rbx
            rdi = rbx;
            // d6ec2 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7b00 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007b00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x7b00;
    loop {
        match pc {
        0x7b00 => {
            // block 7b00
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7b01 mov rbp,rsp
            rbp = rsp;
            // 7b04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7b06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7b08 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7b0a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7b0c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7b0d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7b11 mov r15,rsi
            r15 = rsi;
            // 7b14 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 7b18 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 7b1b je 0x7c1b
            if crate::cov::br(fl.e()) { pc = 0x7c1b; continue; }
            pc = 0x7b21;
        }
        0x7b21 => {
            // block 7b21
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b21 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 7b25 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 7b2a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 7b2e mov r14,QWORD PTR [rbp-0x38]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7b32;
        }
        0x7b32 => {
            // block 7b32
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b32 movsd xmm0,QWORD PTR [r15-0x8]
            x0 = m.r64(r15.wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7b38 addsd xmm0,QWORD PTR [rip+0xf6d98] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 7b40 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 7b45 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 7b47 mov ebx,0x0
            rbx = (0x0u64) as u32 as u64;
            // 7b4c js 0x7b5d
            if crate::cov::br(fl.s()) { pc = 0x7b5d; continue; }
            pc = 0x7b4e;
        }
        0x7b4e => {
            // block 7b4e
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b4e cmp eax,0x3ff
            fl = sub::<32>((rax as u32 as u64), 0x3ffu64).1;
            // 7b53 mov ecx,0x3ff
            rcx = (0x3ffu64) as u32 as u64;
            // 7b58 cmovg eax,ecx
            if fl.g() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 7b5b mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x7b5d;
        }
        0x7b5d => {
            // block 7b5d
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b5d mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7b61 lea r12,[rax+0x1ff]
            r12 = rax.wrapping_add(0x1ff);
            // 7b68 movsd xmm0,QWORD PTR [r15]
            x0 = m.r64(r15) as u128;
            // 7b6d addsd xmm0,QWORD PTR [rip+0xf6d63] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 7b75 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7b7a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7b7c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7b7e js 0x7b90
            if crate::cov::br(fl.s()) { pc = 0x7b90; continue; }
            pc = 0x7b80;
        }
        0x7b80 => {
            // block 7b80
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b80 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7b86 mov eax,0x3ff
            rax = (0x3ffu64) as u32 as u64;
            // 7b8b cmovg ecx,eax
            if fl.g() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7b8e mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7b90;
        }
        0x7b90 => {
            // block 7b90
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b90 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7b93 shl ebx,0x6
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x6u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 7b96 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 7b98 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7b9b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 7b9e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7ba3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7ba6 mov rdi,r14
            rdi = r14;
            // 7ba9 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bac mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bae call QWORD PTR [rax+0x30]
            { m.outregs(0x7bb1, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bb1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7bb3 je 0x7c1e
            if crate::cov::br(fl.e()) { pc = 0x7c1e; continue; }
            pc = 0x7bb5;
        }
        0x7bb5 => {
            // block 7bb5
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7bb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bb5 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7bb8 add r12,0x200
            { let (r, f) = add::<64>(r12, 0x200u64);
              r12 = r;
              fl = f; }
            // 7bbf xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 7bc2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7bc7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7bca mov rdi,r14
            rdi = r14;
            // 7bcd mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bd0 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bd2 call QWORD PTR [rax+0x30]
            { m.outregs(0x7bd5, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bd5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7bd7 je 0x7c1e
            if crate::cov::br(fl.e()) { pc = 0x7c1e; continue; }
            pc = 0x7bd9;
        }
        0x7bd9 => {
            // block 7bd9
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7bd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7bd9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7bdc add r12d,0x200
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x200u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 7be3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7be8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7beb mov rdi,r14
            rdi = r14;
            // 7bee mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bf1 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bf3 call QWORD PTR [rax+0x30]
            { m.outregs(0x7bf6, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bf6 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 7bf9 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7bfd cmp rax,0x1ff
            fl = sub::<64>(rax, 0x1ffu64).1;
            // 7c03 ja 0x7c1e
            if crate::cov::br(fl.a()) { pc = 0x7c1e; continue; }
            pc = 0x7c05;
        }
        0x7c05 => {
            // block 7c05
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c05 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // 7c09 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 7c0c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 7c10 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // 7c13 jne 0x7b32
            if crate::cov::br(fl.ne()) { pc = 0x7b32; continue; }
            pc = 0x7c19;
        }
        0x7c19 => {
            // block 7c19
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c19 jmp 0x7c1e
            pc = 0x7c1e; continue;
        }
        0x7c1b => {
            // block 7c1b
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c1b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x7c1e;
        }
        0x7c1e => {
            // block 7c1e
            #[cfg(feature = "trace")] eprintln!("f_00007b00 pc 7c1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c1e mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 7c21 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7c25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7c26 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7c28 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7c2a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7c2c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7c2e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7c2f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x331bc size=308
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000331bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x331bc;
    loop {
        match pc {
        0x331bc => {
            // block 331bc
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 331bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 331bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 331bd mov rbp,rsp
            rbp = rsp;
            // 331c0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 331c2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 331c4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 331c6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 331c8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 331c9 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 331cd mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 331d1 mov QWORD PTR [rbp-0x30],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdx) as u64);
            // 331d5 mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // 331d9 mov r13,rdi
            r13 = rdi;
            // 331dc mov DWORD PTR [rcx],0xffffffff
            m.w32(rcx, (0xffffffffu64) as u32);
            // 331e2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 331e4 cmp rsi,rdx
            fl = sub::<64>(rsi, rdx).1;
            // 331e7 je 0x332e0
            if crate::cov::br(fl.e()) { pc = 0x332e0; continue; }
            pc = 0x331ed;
        }
        0x331ed => {
            // block 331ed
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 331ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 331ed test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 331f0 je 0x332e0
            if crate::cov::br(fl.e()) { pc = 0x332e0; continue; }
            pc = 0x331f6;
        }
        0x331f6 => {
            // block 331f6
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 331f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 331f6 cmp QWORD PTR [rbp-0x30],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x0u64).1;
            // 331fb je 0x332e0
            if crate::cov::br(fl.e()) { pc = 0x332e0; continue; }
            pc = 0x33201;
        }
        0x33201 => {
            // block 33201
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33201 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33201 cmp DWORD PTR [r13+0x0],0x0
            fl = sub::<32>(m.r32(r13) as u64, 0x0u64).1;
            // 33206 je 0x332de
            if crate::cov::br(fl.e()) { pc = 0x332de; continue; }
            pc = 0x3320c;
        }
        0x3320c => {
            // block 3320c
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3320c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3320c cmp DWORD PTR [r13+0x4],0x0
            fl = sub::<32>(m.r32(r13.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 33211 je 0x332de
            if crate::cov::br(fl.e()) { pc = 0x332de; continue; }
            pc = 0x33217;
        }
        0x33217 => {
            // block 33217
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33217 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33217 mov cx,WORD PTR [r13+0x34]
            rcx = (rcx & !0xffff) | ((m.r16(r13.wrapping_add(0x34)) as u64) as u16 as u64);
            // 3321c test cx,cx
            fl = logic::<16>((rcx as u16 as u64) & (rcx as u16 as u64));
            // 3321f je 0x332de
            if crate::cov::br(fl.e()) { pc = 0x332de; continue; }
            pc = 0x33225;
        }
        0x33225 => {
            // block 33225
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33225 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33225 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33227 cmp DWORD PTR [r13+0x24],0x0
            fl = sub::<32>(m.r32(r13.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 3322c je 0x332e0
            if crate::cov::br(fl.e()) { pc = 0x332e0; continue; }
            pc = 0x33232;
        }
        0x33232 => {
            // block 33232
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33232 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33232 movzx eax,cx
            rax = ((rcx as u16 as u64)) as u32 as u64;
            // 33235 mov DWORD PTR [rbp-0x44],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((rax as u32 as u64)) as u32);
            // 33238 mov DWORD PTR [rbp-0x48],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u32);
            pc = 0x3323f;
        }
        0x3323f => {
            // block 3323f
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3323f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3323f mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 33243 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // 33249 cmp DWORD PTR [r13+0x0],0x0
            fl = sub::<32>(m.r32(r13) as u64, 0x0u64).1;
            // 3324e je 0x332bc
            if crate::cov::br(fl.e()) { pc = 0x332bc; continue; }
            pc = 0x33250;
        }
        0x33250 => {
            // block 33250
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33250 mov eax,DWORD PTR [rbp-0x48]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 33253 movzx ebx,ax
            rbx = ((rax as u16 as u64)) as u32 as u64;
            // 33256 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            pc = 0x3325c;
        }
        0x3325c => {
            // block 3325c
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3325c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3325c lea r15d,[r12-0x1]
            r15 = (r12.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33261 mov rdi,r13
            rdi = r13;
            // 33264 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 33267 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 33269 call 0x330d0
            { m.outregs(0x3326e, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x330d0); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3326e mov r14,rax
            r14 = rax;
            // 33271 mov rdi,r13
            rdi = r13;
            // 33274 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 33278 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 3327b mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 3327d call 0x330d0
            { m.outregs(0x33282, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x330d0); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33282 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 33284 cmp DWORD PTR [rbp-0x44],ebx
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64, (rbx as u32 as u64)).1;
            // 33287 jbe 0x3328e
            if crate::cov::br(fl.be()) { pc = 0x3328e; continue; }
            pc = 0x33289;
        }
        0x33289 => {
            // block 33289
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 33289 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33289 mov ecx,DWORD PTR [r13+rbx*4+0x24]
            rcx = (m.r32(r13.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x24)) as u64) as u32 as u64;
            pc = 0x3328e;
        }
        0x3328e => {
            // block 3328e
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 3328e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3328e mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 33290 mov rdi,r14
            rdi = r14;
            // 33293 mov rsi,rax
            rsi = rax;
            // 33296 call 0xec218
            { m.outregs(0x3329b, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3329b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 3329d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 3329f test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 332a1 mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 332a5 jne 0x332e0
            if crate::cov::br(fl.ne()) { pc = 0x332e0; continue; }
            pc = 0x332a7;
        }
        0x332a7 => {
            // block 332a7
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332a7 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 332ab mov DWORD PTR [rax],r12d
            m.w32(rax, ((r12 as u32 as u64)) as u32);
            // 332ae cmp r12d,DWORD PTR [r13+0x0]
            fl = sub::<32>((r12 as u32 as u64), m.r32(r13) as u64).1;
            // 332b2 lea eax,[r12+0x1]
            rax = (r12.wrapping_add(0x1)) as u32 as u64;
            // 332b7 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 332ba jb 0x3325c
            if crate::cov::br(fl.b()) { pc = 0x3325c; continue; }
            pc = 0x332bc;
        }
        0x332bc => {
            // block 332bc
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332bc mov eax,DWORD PTR [rbp-0x48]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 332bf inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 332c1 mov DWORD PTR [rbp-0x48],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((rax as u32 as u64)) as u32);
            // 332c4 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 332c7 cmp eax,DWORD PTR [rbp-0x44]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64).1;
            // 332ca jb 0x3323f
            if crate::cov::br(fl.b()) { pc = 0x3323f; continue; }
            pc = 0x332d0;
        }
        0x332d0 => {
            // block 332d0
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332d0 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 332d4 mov DWORD PTR [rax],0xffffffff
            m.w32(rax, (0xffffffffu64) as u32);
            // 332da mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 332dc jmp 0x332e0
            pc = 0x332e0; continue;
        }
        0x332de => {
            // block 332de
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332de xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x332e0;
        }
        0x332e0 => {
            // block 332e0
            #[cfg(feature = "trace")] eprintln!("f_000331bc pc 332e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 332e0 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 332e4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 332e5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 332e7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 332e9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 332eb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 332ed pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 332ee ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68fea size=308
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068fea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x68fea;
    loop {
        match pc {
        0x68fea => {
            // block 68fea
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 68fea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68fea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68feb mov rbp,rsp
            rbp = rsp;
            // 68fee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 68ff0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68ff2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 68ff4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 68ff6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68ff7 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 68ffb mov r12,rdx
            r12 = rdx;
            // 68ffe mov DWORD PTR [rbp-0x2c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u32 as u64)) as u32);
            // 69001 mov r15,rdi
            r15 = rdi;
            // 69004 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 6900b mov rsi,r12
            rsi = r12;
            // 6900e call 0xebbd6
            { m.outregs(0x69013, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x3a3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69013 mov rbx,rax
            rbx = rax;
            // 69016 add r15,0x1a0
            { let (r, f) = add::<64>(r15, 0x1a0u64);
              r15 = r;
              fl = f; }
            // 6901d cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69020 je 0x6910d
            if crate::cov::br(fl.e()) { pc = 0x6910d; continue; }
            pc = 0x69026;
        }
        0x69026 => {
            // block 69026
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69026 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69026 movzx r10d,BYTE PTR [r12]
            r10 = (m.r8(r12) as u64) as u32 as u64;
            // 6902b mov r11b,r10b
            r11 = (r11 & !0xff) | (((r10 as u8 as u64)) as u8 as u64);
            // 6902e and r11b,0x1
            { let r = (r11 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r11 = (r11 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 69032 shr r10,1
            { let (r, f) = shr::<64>(r10, 0x1u64, fl); r10 = r; fl = f; }
            // 69035 mov r8,QWORD PTR [r12+0x8]
            r8 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 6903a mov r13,QWORD PTR [r12+0x10]
            r13 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 6903f lea r14,[r12+0x1]
            r14 = r12.wrapping_add(0x1);
            // 69044 mov QWORD PTR [rbp-0x40],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r14) as u64);
            pc = 0x69048;
        }
        0x69048 => {
            // block 69048
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69048 movzx edx,BYTE PTR [rbx+0x20]
            rdx = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 6904c mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 6904e and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 69050 jne 0x69057
            if crate::cov::br(fl.ne()) { pc = 0x69057; continue; }
            pc = 0x69052;
        }
        0x69052 => {
            // block 69052
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69052 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 69055 jmp 0x6905b
            pc = 0x6905b; continue;
        }
        0x69057 => {
            // block 69057
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69057 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69057 mov rdx,QWORD PTR [rbx+0x28]
            rdx = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x6905b;
        }
        0x6905b => {
            // block 6905b
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6905b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6905b test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // 6905e mov rcx,r8
            rcx = r8;
            // 69061 cmove rcx,r10
            if fl.e() { rcx = r10; }
            // 69065 cmp rdx,rcx
            fl = sub::<64>(rdx, rcx).1;
            // 69068 jne 0x6910d
            if crate::cov::br(fl.ne()) { pc = 0x6910d; continue; }
            pc = 0x6906e;
        }
        0x6906e => {
            // block 6906e
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6906e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6906e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 69070 jne 0x6907b
            if crate::cov::br(fl.ne()) { pc = 0x6907b; continue; }
            pc = 0x69072;
        }
        0x69072 => {
            // block 69072
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69072 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69072 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 69076 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 69079 jmp 0x6907f
            pc = 0x6907f; continue;
        }
        0x6907b => {
            // block 6907b
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6907b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6907b mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x6907f;
        }
        0x6907f => {
            // block 6907f
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6907f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6907f test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // 69082 mov rsi,r13
            rsi = r13;
            // 69085 cmove rsi,r14
            if fl.e() { rsi = r14; }
            // 69089 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6908b jne 0x690a9
            if crate::cov::br(fl.ne()) { pc = 0x690a9; continue; }
            pc = 0x6908d;
        }
        0x6908d => {
            // block 6908d
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6908d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6908d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 69090 je 0x690d5
            if crate::cov::br(fl.e()) { pc = 0x690d5; continue; }
            pc = 0x69092;
        }
        0x69092 => {
            // block 69092
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69092 movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 69095 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 69098 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 6909a jne 0x6910d
            if crate::cov::br(fl.ne()) { pc = 0x6910d; continue; }
            pc = 0x6909c;
        }
        0x6909c => {
            // block 6909c
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6909c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6909c inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 6909f inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 690a2 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 690a5 jne 0x69092
            if crate::cov::br(fl.ne()) { pc = 0x69092; continue; }
            pc = 0x690a7;
        }
        0x690a7 => {
            // block 690a7
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690a7 jmp 0x690d5
            pc = 0x690d5; continue;
        }
        0x690a9 => {
            // block 690a9
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690a9 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 690ac je 0x690d5
            if crate::cov::br(fl.e()) { pc = 0x690d5; continue; }
            pc = 0x690ae;
        }
        0x690ae => {
            // block 690ae
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690ae mov r12,r8
            r12 = r8;
            // 690b1 mov QWORD PTR [rbp-0x38],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r15) as u64);
            // 690b5 mov r15,r10
            r15 = r10;
            // 690b8 mov r14b,r11b
            r14 = (r14 & !0xff) | (((r11 as u8 as u64)) as u8 as u64);
            // 690bb call 0xec218
            { m.outregs(0x690c0, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 690c0 mov r11b,r14b
            r11 = (r11 & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 690c3 mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 690c7 mov r10,r15
            r10 = r15;
            // 690ca mov r15,QWORD PTR [rbp-0x38]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 690ce mov r8,r12
            r8 = r12;
            // 690d1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 690d3 jne 0x6910d
            if crate::cov::br(fl.ne()) { pc = 0x6910d; continue; }
            pc = 0x690d5;
        }
        0x690d5 => {
            // block 690d5
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690d5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 690d7 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 690da cmp DWORD PTR [rbx+0x38],ecx
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x38)) as u64, (rcx as u32 as u64)).1;
            // 690dd je 0x6910f
            if crate::cov::br(fl.e()) { pc = 0x6910f; continue; }
            pc = 0x690df;
        }
        0x690df => {
            // block 690df
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690df mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 690e3 test rax,rax
            fl = logic::<64>(rax & rax);
            // 690e6 je 0x690f5
            if crate::cov::br(fl.e()) { pc = 0x690f5; continue; }
            pc = 0x690e8;
        }
        0x690e8 => {
            // block 690e8
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690e8 mov rbx,rax
            rbx = rax;
            // 690eb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 690ee test rax,rax
            fl = logic::<64>(rax & rax);
            // 690f1 jne 0x690e8
            if crate::cov::br(fl.ne()) { pc = 0x690e8; continue; }
            pc = 0x690f3;
        }
        0x690f3 => {
            // block 690f3
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690f3 jmp 0x69104
            pc = 0x69104; continue;
        }
        0x690f5 => {
            // block 690f5
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690f5 mov rax,rbx
            rax = rbx;
            pc = 0x690f8;
        }
        0x690f8 => {
            // block 690f8
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690f8 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 690fc cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 690ff mov rax,rbx
            rax = rbx;
            // 69102 jne 0x690f8
            if crate::cov::br(fl.ne()) { pc = 0x690f8; continue; }
            pc = 0x69104;
        }
        0x69104 => {
            // block 69104
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69104 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69104 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69107 jne 0x69048
            if crate::cov::br(fl.ne()) { pc = 0x69048; continue; }
            pc = 0x6910d;
        }
        0x6910d => {
            // block 6910d
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6910d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6910d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x6910f;
        }
        0x6910f => {
            // block 6910f
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6910f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6910f add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 69113 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 69114 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 69116 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 69118 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6911a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6911c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6911d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x95bc size=320
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000095bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x95bc;
    loop {
        match pc {
        0x95bc => {
            // block 95bc
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 95bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 95bd mov rbp,rsp
            rbp = rsp;
            // 95c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 95c2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 95c3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 95c7 mov r14,rsi
            r14 = rsi;
            // 95ca mov rbx,rdi
            rbx = rdi;
            // 95cd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 95d0 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 95d4 call QWORD PTR [rax+0xde0]
            { m.outregs(0x95da, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xde0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 95da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 95dd lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 95e1 mov rdi,rbx
            rdi = rbx;
            // 95e4 call QWORD PTR [rax+0xdc0]
            { m.outregs(0x95ea, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xdc0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 95ea mov DWORD PTR [r14],0x25
            m.w32(r14, (0x25u64) as u32);
            // 95f1 mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 95f4 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // 95f8 jbe 0x9605
            if crate::cov::br(fl.be()) { pc = 0x9605; continue; }
            pc = 0x95fa;
        }
        0x95fa => {
            // block 95fa
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 95fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95fa xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x95fc;
        }
        0x95fc => {
            // block 95fc
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 95fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 95fc add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 9600 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9601 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9603 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9604 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x9605 => {
            // block 9605
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9605 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9605 lea rcx,[rip+0xdc] # 0x96e8
            rcx = m.env.code_va(0x96e8);
            // 960c movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 9610 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 9613 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x9615 => {
            // block 9615
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9615 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9615 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 9617 mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 961a cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 961d jne 0x967c
            if crate::cov::br(fl.ne()) { pc = 0x967c; continue; }
            pc = 0x961f;
        }
        0x961f => {
            // block 961f
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 961f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 961f mov DWORD PTR [r14],0x1
            m.w32(r14, (0x1u64) as u32);
            // 9626 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x9628 => {
            // block 9628
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9628 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9628 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 962a mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 962d cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 9630 jne 0x9691
            if crate::cov::br(fl.ne()) { pc = 0x9691; continue; }
            pc = 0x9632;
        }
        0x9632 => {
            // block 9632
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9632 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9632 mov DWORD PTR [r14],0x3
            m.w32(r14, (0x3u64) as u32);
            // 9639 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x963b => {
            // block 963b
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 963b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 963b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 963d mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 9640 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 9643 je 0x96a6
            if crate::cov::br(fl.e()) { pc = 0x96a6; continue; }
            pc = 0x9645;
        }
        0x9645 => {
            // block 9645
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9645 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // 9648 jne 0x96be
            if crate::cov::br(fl.ne()) { pc = 0x96be; continue; }
            pc = 0x964a;
        }
        0x964a => {
            // block 964a
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 964a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 964a mov DWORD PTR [r14],0x5
            m.w32(r14, (0x5u64) as u32);
            // 9651 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x9653 => {
            // block 9653
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9653 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9653 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 9655 mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 9658 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 965b je 0x96b2
            if crate::cov::br(fl.e()) { pc = 0x96b2; continue; }
            pc = 0x965d;
        }
        0x965d => {
            // block 965d
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 965d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 965d cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // 9660 jne 0x96d2
            if crate::cov::br(fl.ne()) { pc = 0x96d2; continue; }
            pc = 0x9662;
        }
        0x9662 => {
            // block 9662
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9662 mov DWORD PTR [r14],0x7
            m.w32(r14, (0x7u64) as u32);
            // 9669 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x966b => {
            // block 966b
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 966b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 966b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 966d cmp DWORD PTR [rbp-0x18],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64, 0x1u64).1;
            // 9671 jne 0x95fc
            if crate::cov::br(fl.ne()) { pc = 0x95fc; continue; }
            pc = 0x9673;
        }
        0x9673 => {
            // block 9673
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9673 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9673 mov DWORD PTR [r14],0x1e
            m.w32(r14, (0x1eu64) as u32);
            // 967a jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x967c => {
            // block 967c
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 967c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 967c cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 967f jne 0x95fc
            if crate::cov::br(fl.ne()) { pc = 0x95fc; continue; }
            pc = 0x9685;
        }
        0x9685 => {
            // block 9685
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9685 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9685 mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // 968c jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x9691 => {
            // block 9691
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 9691 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9691 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 9694 jne 0x95fc
            if crate::cov::br(fl.ne()) { pc = 0x95fc; continue; }
            pc = 0x969a;
        }
        0x969a => {
            // block 969a
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 969a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 969a mov DWORD PTR [r14],0x2
            m.w32(r14, (0x2u64) as u32);
            // 96a1 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96a6 => {
            // block 96a6
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96a6 mov DWORD PTR [r14],0x17
            m.w32(r14, (0x17u64) as u32);
            // 96ad jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96b2 => {
            // block 96b2
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96b2 mov DWORD PTR [r14],0x18
            m.w32(r14, (0x18u64) as u32);
            // 96b9 jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96be => {
            // block 96be
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96be test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 96c0 jne 0x95fc
            if crate::cov::br(fl.ne()) { pc = 0x95fc; continue; }
            pc = 0x96c6;
        }
        0x96c6 => {
            // block 96c6
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96c6 mov DWORD PTR [r14],0x4
            m.w32(r14, (0x4u64) as u32);
            // 96cd jmp 0x95fc
            pc = 0x95fc; continue;
        }
        0x96d2 => {
            // block 96d2
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96d2 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 96d4 jne 0x95fc
            if crate::cov::br(fl.ne()) { pc = 0x95fc; continue; }
            pc = 0x96da;
        }
        0x96da => {
            // block 96da
            #[cfg(feature = "trace")] eprintln!("f_000095bc pc 96da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96da mov DWORD PTR [r14],0x6
            m.w32(r14, (0x6u64) as u32);
            // 96e1 jmp 0x95fc
            pc = 0x95fc; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x586aa size=332
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000586aa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x586aa;
    loop {
        match pc {
        0x586aa => {
            // block 586aa
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586aa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 586ab mov rbp,rsp
            rbp = rsp;
            // 586ae push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 586b0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 586b2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 586b4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 586b6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 586b7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 586b8 mov rbx,rdx
            rbx = rdx;
            // 586bb mov r14,rsi
            r14 = rsi;
            // 586be mov r15,rdi
            r15 = rdi;
            // 586c1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 586c4 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 586c8 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 586cb je 0x586e5
            if crate::cov::br(fl.e()) { pc = 0x586e5; continue; }
            pc = 0x586cd;
        }
        0x586cd => {
            // block 586cd
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586cd lea rdx,[rcx-0x2]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffe);
            // 586d1 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 586d4 not rdx
            { let v = !rdx; rdx = v; }
            // 586d7 and rdx,0xfffffffffffffffe
            { let r = rdx & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 586db add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 586de mov QWORD PTR [rbx+0x8],rdx
            m.w64(rbx.wrapping_add(0x8), (rdx) as u64);
            // 586e2 mov rax,rdx
            rax = rdx;
            pc = 0x586e5;
        }
        0x586e5 => {
            // block 586e5
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586e5 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 586e8 je 0x58716
            if crate::cov::br(fl.e()) { pc = 0x58716; continue; }
            pc = 0x586ea;
        }
        0x586ea => {
            // block 586ea
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586ea cmp DWORD PTR [r14],0x0
            fl = sub::<32>(m.r32(r14) as u64, 0x0u64).1;
            // 586ee je 0x58716
            if crate::cov::br(fl.e()) { pc = 0x58716; continue; }
            pc = 0x586f0;
        }
        0x586f0 => {
            // block 586f0
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586f0 mov ecx,DWORD PTR [r14+0x4]
            rcx = (m.r32(r14.wrapping_add(0x4)) as u64) as u32 as u64;
            // 586f4 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 586f6 je 0x58716
            if crate::cov::br(fl.e()) { pc = 0x58716; continue; }
            pc = 0x586f8;
        }
        0x586f8 => {
            // block 586f8
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586f8 cmp WORD PTR [r14+0x34],0x0
            fl = sub::<16>(m.r16(r14.wrapping_add(0x34)) as u64, 0x0u64).1;
            // 586fe je 0x58716
            if crate::cov::br(fl.e()) { pc = 0x58716; continue; }
            pc = 0x58700;
        }
        0x58700 => {
            // block 58700
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58700 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58700 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 58703 cmp DWORD PTR [r14+0x24],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 58708 je 0x58719
            if crate::cov::br(fl.e()) { pc = 0x58719; continue; }
            pc = 0x5870a;
        }
        0x5870a => {
            // block 5870a
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5870a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5870a cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 5870d jb 0x58719
            if crate::cov::br(fl.b()) { pc = 0x58719; continue; }
            pc = 0x5870f;
        }
        0x5870f => {
            // block 5870f
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5870f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5870f cmp DWORD PTR [r14+0x18],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x18)) as u64, 0x0u64).1;
            // 58714 je 0x5872b
            if crate::cov::br(fl.e()) { pc = 0x5872b; continue; }
            pc = 0x58716;
        }
        0x58716 => {
            // block 58716
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58716 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58716 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x58719;
        }
        0x58719 => {
            // block 58719
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58719 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58719 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 5871c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 58720 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58721 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 58723 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 58725 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58727 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58729 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5872a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5872b => {
            // block 5872b
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5872b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5872b mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // 5872e cmp DWORD PTR [r14+0x8],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x8)) as u64, 0x0u64).1;
            // 58733 je 0x58719
            if crate::cov::br(fl.e()) { pc = 0x58719; continue; }
            pc = 0x58735;
        }
        0x58735 => {
            // block 58735
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58735 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58735 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 58738 jmp 0x58741
            pc = 0x58741; continue;
        }
        0x5873a => {
            // block 5873a
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5873a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5873a mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 5873e inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            pc = 0x58741;
        }
        0x58741 => {
            // block 58741
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58741 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58741 mov ecx,DWORD PTR [r15+r13*4]
            rcx = (m.r32(r15.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 58745 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 58747 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5874d mov WORD PTR [rbp-0x2a],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd6), ((rsi as u16 as u64)) as u16);
            // 58751 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 58755 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 58758 jae 0x58767
            if crate::cov::br(fl.ae()) { pc = 0x58767; continue; }
            pc = 0x5875a;
        }
        0x5875a => {
            // block 5875a
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5875a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5875a mov WORD PTR [rax],si
            m.w16(rax, ((rsi as u16 as u64)) as u16);
            // 5875d add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 58761 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 58765 jmp 0x5877f
            pc = 0x5877f; continue;
        }
        0x58767 => {
            // block 58767
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58767 mov rdi,rbx
            rdi = rbx;
            // 5876a lea rsi,[rbp-0x2a]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd6);
            // 5876e call 0xebbc4
            { m.outregs(0x58773, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58773 mov ecx,DWORD PTR [r15+r13*4]
            rcx = (m.r32(r15.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 58777 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 5877b mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x5877f;
        }
        0x5877f => {
            // block 5877f
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5877f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5877f mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 58781 shr esi,0xa
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xau64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 58784 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5878a mov WORD PTR [rbp-0x2c],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u16 as u64)) as u16);
            // 5878e cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 58791 jae 0x587a0
            if crate::cov::br(fl.ae()) { pc = 0x587a0; continue; }
            pc = 0x58793;
        }
        0x58793 => {
            // block 58793
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58793 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58793 mov WORD PTR [rax],si
            m.w16(rax, ((rsi as u16 as u64)) as u16);
            // 58796 add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 5879a mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 5879e jmp 0x587b8
            pc = 0x587b8; continue;
        }
        0x587a0 => {
            // block 587a0
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587a0 mov rdi,rbx
            rdi = rbx;
            // 587a3 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 587a7 call 0xebbc4
            { m.outregs(0x587ac, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 587ac mov ecx,DWORD PTR [r15+r13*4]
            rcx = (m.r32(r15.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 587b0 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 587b4 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x587b8;
        }
        0x587b8 => {
            // block 587b8
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587b8 shr ecx,0x14
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x14u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 587bb and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 587c1 mov WORD PTR [rbp-0x2e],cx
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2), ((rcx as u16 as u64)) as u16);
            // 587c5 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 587c8 jae 0x587d7
            if crate::cov::br(fl.ae()) { pc = 0x587d7; continue; }
            pc = 0x587ca;
        }
        0x587ca => {
            // block 587ca
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587ca mov WORD PTR [rax],cx
            m.w16(rax, ((rcx as u16 as u64)) as u16);
            // 587cd add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 587d1 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 587d5 jmp 0x587e3
            pc = 0x587e3; continue;
        }
        0x587d7 => {
            // block 587d7
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587d7 mov rdi,rbx
            rdi = rbx;
            // 587da lea rsi,[rbp-0x2e]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd2);
            // 587de call 0xebbc4
            { m.outregs(0x587e3, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x587e3;
        }
        0x587e3 => {
            // block 587e3
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587e3 lea eax,[r13+0x1]
            rax = (r13.wrapping_add(0x1)) as u32 as u64;
            // 587e7 cmp eax,DWORD PTR [r14+0x8]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x8)) as u64).1;
            // 587eb jae 0x58719
            if crate::cov::br(fl.ae()) { pc = 0x58719; continue; }
            pc = 0x587f1;
        }
        0x587f1 => {
            // block 587f1
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587f1 jmp 0x5873a
            pc = 0x5873a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4444 size=340
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4444<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4444;
    loop {
        match pc {
        0xd4444 => {
            // block d4444
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4444 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4444 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4445 mov rbp,rsp
            rbp = rsp;
            // d4448 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d444a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d444c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d444e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d444f sub rsp,0xd0
            { let (r, f) = sub::<64>(rsp, 0xd0u64);
              rsp = r;
              fl = f; }
            // d4456 mov r15,rdx
            r15 = rdx;
            // d4459 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d445c mov r12,rdi
            r12 = rdi;
            // d445f mov rdi,r15
            rdi = r15;
            // d4462 call 0x2a25e
            { m.outregs(0xd4467, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x2a25e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4467 lea rbx,[r15+0x20]
            rbx = r15.wrapping_add(0x20);
            // d446b mov rdi,r12
            rdi = r12;
            // d446e mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d4471 mov rdx,rbx
            rdx = rbx;
            // d4474 call 0xd40e0
            { m.outregs(0xd4479, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd40e0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4479 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d447b je 0xd455c
            if crate::cov::br(fl.e()) { pc = 0xd455c; continue; }
            pc = 0xd4481;
        }
        0xd4481 => {
            // block d4481
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4481 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4481 mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // d4483 cmp eax,0x11
            fl = sub::<32>((rax as u32 as u64), 0x11u64).1;
            // d4486 ja 0xd4560
            if crate::cov::br(fl.a()) { pc = 0xd4560; continue; }
            pc = 0xd448c;
        }
        0xd448c => {
            // block d448c
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d448c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d448c mov ecx,0x3ffcf
            rcx = (0x3ffcfu64) as u32 as u64;
            // d4491 bt ecx,eax
            fl.cf = bit::<32>((rcx as u32 as u64), (rax as u32 as u64));
            // d4494 jae 0xd4560
            if crate::cov::br(fl.ae()) { pc = 0xd4560; continue; }
            pc = 0xd449a;
        }
        0xd449a => {
            // block d449a
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d449a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d449a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d449e mov rdi,r12
            rdi = r12;
            // d44a1 mov rsi,r15
            rsi = r15;
            // d44a4 call QWORD PTR [rax+0xd0]
            { m.outregs(0xd44aa, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xd0)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d44aa mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d44ac mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d44ae test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d44b0 jne 0xd4588
            if crate::cov::br(fl.ne()) { pc = 0xd4588; continue; }
            pc = 0xd44b6;
        }
        0xd44b6 => {
            // block d44b6
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d44b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d44b6 xorps xmm0,xmm0
            x0 = 0;
            // d44b9 movaps XMMWORD PTR [rbp-0xb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff50), x0);
            // d44c0 movaps XMMWORD PTR [rbp-0xc0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff40), x0);
            // d44c7 movaps XMMWORD PTR [rbp-0xd0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff30), x0);
            // d44ce movaps XMMWORD PTR [rbp-0xe0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff20), x0);
            // d44d5 movaps XMMWORD PTR [rbp-0xf0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff10), x0);
            // d44dc mov DWORD PTR [rbp-0x98],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x0u64) as u32);
            // d44e6 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // d44f1 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d44f4 call 0x5c6f2
            { m.outregs(0xd44f9, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x5c6f2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d44f9 mov DWORD PTR [rbp-0xf0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), ((rax as u32 as u64)) as u32);
            // d44ff mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // d4503 lea rdx,[rbp-0xf0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // d450a mov rdi,r12
            rdi = r12;
            // d450d mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d450f call QWORD PTR [rcx+0xc18]
            { m.outregs(0xd4515, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rcx.wrapping_add(0xc18)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4515 cmp DWORD PTR [rbp-0xec],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff14)) as u64, 0x0u64).1;
            // d451c jne 0xd454b
            if crate::cov::br(fl.ne()) { pc = 0xd454b; continue; }
            pc = 0xd451e;
        }
        0xd451e => {
            // block d451e
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d451e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d451e mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d4521 call 0x5c70e
            { m.outregs(0xd4526, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x5c70e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4526 mov DWORD PTR [rbp-0xf0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), ((rax as u32 as u64)) as u32);
            // d452c mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // d4530 lea rdx,[rbp-0xf0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // d4537 mov rdi,r12
            rdi = r12;
            // d453a mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d453c call QWORD PTR [rcx+0xc18]
            { m.outregs(0xd4542, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rcx.wrapping_add(0xc18)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4542 cmp DWORD PTR [rbp-0xec],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff14)) as u64, 0x0u64).1;
            // d4549 je 0xd4560
            if crate::cov::br(fl.e()) { pc = 0xd4560; continue; }
            pc = 0xd454b;
        }
        0xd454b => {
            // block d454b
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d454b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d454b lea rsi,[rbp-0xf0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // d4552 mov rdi,r15
            rdi = r15;
            // d4555 call 0x2a1d8
            { m.outregs(0xd455a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x2a1d8); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d455a jmp 0xd4588
            pc = 0xd4588; continue;
        }
        0xd455c => {
            // block d455c
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d455c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d455c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d455e jmp 0xd4588
            pc = 0xd4588; continue;
        }
        0xd4560 => {
            // block d4560
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4560 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d4563 call 0x5c70e
            { m.outregs(0xd4568, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x5c70e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4568 lea rbx,[rbp-0x90]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d456f mov rdi,rbx
            rdi = rbx;
            // d4572 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d4574 call 0x2a0c4
            { m.outregs(0xd4579, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x2a0c4); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4579 mov ecx,0x1c
            rcx = (0x1cu64) as u32 as u64;
            // d457e mov rdi,r15
            rdi = r15;
            // d4581 mov rsi,rbx
            rsi = rbx;
            // d4584 rep movs DWORD PTR [rdi],DWORD PTR [rsi]
            while rcx != 0 { let v = m.r32(rsi); m.w32(rdi, v); rsi = rsi.wrapping_add(4); rdi = rdi.wrapping_add(4); rcx = rcx.wrapping_sub(1); }
            // d4586 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd4588;
        }
        0xd4588 => {
            // block d4588
            #[cfg(feature = "trace")] eprintln!("f_000d4444 pc d4588 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4588 add rsp,0xd0
            { let (r, f) = add::<64>(rsp, 0xd0u64);
              rsp = r;
              fl = f; }
            // d458f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4590 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d4592 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4594 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d4596 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4597 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdf04e size=342
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000df04e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdf04e;
    loop {
        match pc {
        0xdf04e => {
            // block df04e
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df04e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df04e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df04f mov rbp,rsp
            rbp = rsp;
            // df052 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // df054 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // df056 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // df058 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // df05a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df05b sub rsp,0x78
            { let (r, f) = sub::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // df05f mov QWORD PTR [rbp-0x88],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (r8) as u64);
            // df066 mov QWORD PTR [rbp-0x90],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (rcx) as u64);
            // df06d mov r12,rdx
            r12 = rdx;
            // df070 mov r13,rsi
            r13 = rsi;
            // df073 mov rbx,rdi
            rbx = rdi;
            // df076 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df079 mov edx,DWORD PTR [rbx+0x250]
            rdx = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // df07f lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // df083 call QWORD PTR [rax+0x3f0]
            { m.outregs(0xdf089, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df089 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df08c mov edx,DWORD PTR [rbx+0x250]
            rdx = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // df092 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // df096 mov rdi,rbx
            rdi = rbx;
            // df099 call QWORD PTR [rax+0x388]
            { m.outregs(0xdf09f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df09f mov r14d,DWORD PTR [rbp-0x30]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // df0a3 lea ecx,[r14-0x5]
            rcx = (r14.wrapping_add(0xffff_ffff_ffff_fffb)) as u32 as u64;
            // df0a7 cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // df0aa ja 0xdf0bb
            if crate::cov::br(fl.a()) { pc = 0xdf0bb; continue; }
            pc = 0xdf0ac;
        }
        0xdf0ac => {
            // block df0ac
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0ac mov r15d,0x121
            r15 = (0x121u64) as u32 as u64;
            // df0b2 shr r15d,cl
            { let (r, f) = shr::<32>((r15 as u32 as u64), (rcx as u8 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // df0b5 and r15d,0x1
            { let r = (r15 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // df0b9 jmp 0xdf0be
            pc = 0xdf0be; continue;
        }
        0xdf0bb => {
            // block df0bb
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0bb xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xdf0be;
        }
        0xdf0be => {
            // block df0be
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0be mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // df0c1 mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // df0c7 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // df0cb xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // df0cd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df0d0 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // df0d3 call 0x32d57
            { m.outregs(0xdf0d8, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x32d57); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df0d8 mov eax,DWORD PTR [rbp-0x6c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // df0db mov DWORD PTR [rbp-0x74],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), ((rax as u32 as u64)) as u32);
            // df0de mov eax,DWORD PTR [rbp-0x68]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // df0e1 mov DWORD PTR [rbp-0x78],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff88), ((rax as u32 as u64)) as u32);
            // df0e4 mov eax,DWORD PTR [rbp-0x70]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // df0e7 mov DWORD PTR [rbp-0x7c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff84), ((rax as u32 as u64)) as u32);
            // df0ea mov eax,DWORD PTR [rbp-0x64]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // df0ed mov DWORD PTR [rbp-0x94],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c), ((rax as u32 as u64)) as u32);
            // df0f3 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // df0f6 je 0xdf10c
            if crate::cov::br(fl.e()) { pc = 0xdf10c; continue; }
            pc = 0xdf0f8;
        }
        0xdf0f8 => {
            // block df0f8
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0f8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df0fb lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df0ff lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df103 mov rdi,rbx
            rdi = rbx;
            // df106 call QWORD PTR [rax+0x1a00]
            { m.outregs(0xdf10c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdf10c;
        }
        0xdf10c => {
            // block df10c
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df10c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df10c cmp r14d,0x11
            fl = sub::<32>((r14 as u32 as u64), 0x11u64).1;
            // df110 jne 0xdf12c
            if crate::cov::br(fl.ne()) { pc = 0xdf12c; continue; }
            pc = 0xdf112;
        }
        0xdf112 => {
            // block df112
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df112 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df112 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df115 lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df119 lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df11d lea rcx,[rbp-0x7c]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff84);
            // df121 mov rdi,rbx
            rdi = rbx;
            // df124 call QWORD PTR [rax+0x1a08]
            { m.outregs(0xdf12a, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1a08)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df12a jmp 0xdf14a
            pc = 0xdf14a; continue;
        }
        0xdf12c => {
            // block df12c
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df12c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df12c cmp r14d,0x10
            fl = sub::<32>((r14 as u32 as u64), 0x10u64).1;
            // df130 jne 0xdf14a
            if crate::cov::br(fl.ne()) { pc = 0xdf14a; continue; }
            pc = 0xdf132;
        }
        0xdf132 => {
            // block df132
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df132 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df135 lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df139 lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df13d lea rcx,[rbp-0x7c]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff84);
            // df141 mov rdi,rbx
            rdi = rbx;
            // df144 call QWORD PTR [rax+0x1a10]
            { m.outregs(0xdf14a, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1a10)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdf14a;
        }
        0xdf14a => {
            // block df14a
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df14a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df14a test r13,r13
            fl = logic::<64>(r13 & r13);
            // df14d mov rcx,QWORD PTR [rbp-0x88]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // df154 mov rdx,QWORD PTR [rbp-0x90]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // df15b je 0xdf164
            if crate::cov::br(fl.e()) { pc = 0xdf164; continue; }
            pc = 0xdf15d;
        }
        0xdf15d => {
            // block df15d
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df15d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df15d mov eax,DWORD PTR [rbp-0x74]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c)) as u64) as u32 as u64;
            // df160 mov DWORD PTR [r13+0x0],eax
            m.w32(r13, ((rax as u32 as u64)) as u32);
            pc = 0xdf164;
        }
        0xdf164 => {
            // block df164
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df164 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df164 test r12,r12
            fl = logic::<64>(r12 & r12);
            // df167 je 0xdf170
            if crate::cov::br(fl.e()) { pc = 0xdf170; continue; }
            pc = 0xdf169;
        }
        0xdf169 => {
            // block df169
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df169 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df169 mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            // df16c mov DWORD PTR [r12],eax
            m.w32(r12, ((rax as u32 as u64)) as u32);
            pc = 0xdf170;
        }
        0xdf170 => {
            // block df170
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df170 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df170 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // df173 je 0xdf17d
            if crate::cov::br(fl.e()) { pc = 0xdf17d; continue; }
            pc = 0xdf175;
        }
        0xdf175 => {
            // block df175
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df175 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df175 mov eax,DWORD PTR [rbp-0x78]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // df178 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // df17b mov DWORD PTR [rdx],eax
            m.w32(rdx, ((rax as u32 as u64)) as u32);
            pc = 0xdf17d;
        }
        0xdf17d => {
            // block df17d
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df17d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df17d test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // df180 je 0xdf18a
            if crate::cov::br(fl.e()) { pc = 0xdf18a; continue; }
            pc = 0xdf182;
        }
        0xdf182 => {
            // block df182
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df182 mov eax,DWORD PTR [rbp-0x94]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c)) as u64) as u32 as u64;
            // df188 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            pc = 0xdf18a;
        }
        0xdf18a => {
            // block df18a
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df18a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df18a mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            // df18d imul eax,DWORD PTR [rbp-0x78]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64); rax = (r) as u32 as u64; fl = f; }
            // df191 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // df194 add rsp,0x78
            { let (r, f) = add::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // df198 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // df199 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // df19b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // df19d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // df19f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // df1a1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // df1a2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd415c size=352
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d415c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd415c;
    loop {
        match pc {
        0xd415c => {
            // block d415c
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d415c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d415c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d415d mov rbp,rsp
            rbp = rsp;
            // d4160 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4162 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4164 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d4166 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d4168 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4169 sub rsp,0x138
            { let (r, f) = sub::<64>(rsp, 0x138u64);
              rsp = r;
              fl = f; }
            // d4170 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // d4173 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d4176 mov r15,rdi
            r15 = rdi;
            // d4179 cmp r14d,0x7
            fl = sub::<32>((r14 as u32 as u64), 0x7u64).1;
            // d417d ja 0xd42a5
            if crate::cov::br(fl.a()) { pc = 0xd42a5; continue; }
            pc = 0xd4183;
        }
        0xd4183 => {
            // block d4183
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d4183 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4183 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // d4186 mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // d418b mov ebx,0x2
            rbx = (0x2u64) as u32 as u64;
            // d4190 cmovne ebx,eax
            if fl.ne() { rbx = ((rax as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // d4193 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d4196 call 0x5c6f2
            { m.outregs(0xd419b, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x5c6f2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d419b lea r13,[rbp-0x88]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // d41a2 mov rdi,r13
            rdi = r13;
            // d41a5 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // d41a7 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // d41a9 call 0x26090
            { m.outregs(0xd41ae, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d41ae mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d41b1 call 0x5c70e
            { m.outregs(0xd41b6, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x5c70e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d41b6 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // d41bd mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // d41bf mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // d41c1 call 0x26090
            { m.outregs(0xd41c6, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d41c6 mov DWORD PTR [rbp-0xec],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff14), (0x2u64) as u32);
            // d41d0 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // d41d7 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d41d9 call 0x2a0c4
            { m.outregs(0xd41de, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x2a0c4); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d41de mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d41e1 mov rdi,r15
            rdi = r15;
            // d41e4 mov rsi,r13
            rsi = r13;
            // d41e7 call QWORD PTR [rax+0xc8]
            { m.outregs(0xd41ed, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d41ed mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d41f0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d41f3 mov rdi,r15
            rdi = r15;
            // d41f6 lea rsi,[rbp-0xe8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // d41fd call QWORD PTR [rax+0xc8]
            { m.outregs(0xd4203, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4203 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d4205 or bl,r13b
            { let r = (rbx as u8 as u64) | (r13 as u8 as u64); let f = logic::<8>(r);
              rbx = (rbx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d4208 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // d420b jne 0xd42a7
            if crate::cov::br(fl.ne()) { pc = 0xd42a7; continue; }
            pc = 0xd4211;
        }
        0xd4211 => {
            // block d4211
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d4211 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4211 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d4213 xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d4215 jne 0xd42a7
            if crate::cov::br(fl.ne()) { pc = 0xd42a7; continue; }
            pc = 0xd421b;
        }
        0xd421b => {
            // block d421b
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d421b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d421b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d421e lea rdx,[rbp-0xec]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff14);
            // d4225 mov rdi,r15
            rdi = r15;
            // d4228 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d422b call QWORD PTR [rax+0x370]
            { m.outregs(0xd4231, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x370)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4231 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d4233 mov eax,DWORD PTR [rbp-0xec]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff14)) as u64) as u32 as u64;
            // d4239 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d423c jne 0xd4255
            if crate::cov::br(fl.ne()) { pc = 0xd4255; continue; }
            pc = 0xd423e;
        }
        0xd423e => {
            // block d423e
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d423e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d423e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d4241 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d4243 mov rdi,r15
            rdi = r15;
            // d4246 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d4249 call QWORD PTR [rax+0xbe8]
            { m.outregs(0xd424f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xbe8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d424f mov eax,DWORD PTR [rbp-0xec]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff14)) as u64) as u32 as u64;
            pc = 0xd4255;
        }
        0xd4255 => {
            // block d4255
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d4255 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4255 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d4257 jne 0xd426a
            if crate::cov::br(fl.ne()) { pc = 0xd426a; continue; }
            pc = 0xd4259;
        }
        0xd4259 => {
            // block d4259
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d4259 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4259 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d425c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d425e mov rdi,r15
            rdi = r15;
            // d4261 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d4264 call QWORD PTR [rax+0xbd8]
            { m.outregs(0xd426a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xbd8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd426a;
        }
        0xd426a => {
            // block d426a
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d426a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d426a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d426d lea rdx,[rbp-0x160]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // d4274 mov rdi,r15
            rdi = r15;
            // d4277 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d427a call QWORD PTR [rax+0xc60]
            { m.outregs(0xd4280, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xc60)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4280 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d4282 je 0xd42a7
            if crate::cov::br(fl.e()) { pc = 0xd42a7; continue; }
            pc = 0xd4284;
        }
        0xd4284 => {
            // block d4284
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d4284 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4284 cmp DWORD PTR [rbp-0x13c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fec4)) as u64, 0x0u64).1;
            // d428b je 0xd42a7
            if crate::cov::br(fl.e()) { pc = 0xd42a7; continue; }
            pc = 0xd428d;
        }
        0xd428d => {
            // block d428d
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d428d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d428d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d4290 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d4295 mov rdi,r15
            rdi = r15;
            // d4298 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d429b call QWORD PTR [rax+0xc38]
            { m.outregs(0xd42a1, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xc38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d42a1 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d42a3 jmp 0xd42a7
            pc = 0xd42a7; continue;
        }
        0xd42a5 => {
            // block d42a5
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d42a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d42a5 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd42a7;
        }
        0xd42a7 => {
            // block d42a7
            #[cfg(feature = "trace")] eprintln!("f_000d415c pc d42a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d42a7 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d42a9 add rsp,0x138
            { let (r, f) = add::<64>(rsp, 0x138u64);
              rsp = r;
              fl = f; }
            // d42b0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d42b1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d42b3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d42b5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d42b7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d42b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d42ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43edc size=354
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043edc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43edc;
    loop {
        match pc {
        0x43edc => {
            // block 43edc
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43edc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43edc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43edd mov rbp,rsp
            rbp = rsp;
            // 43ee0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43ee2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43ee4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 43ee6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 43ee8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43ee9 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 43eed mov r12,rdx
            r12 = rdx;
            // 43ef0 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 43ef3 mov r15,rdi
            r15 = rdi;
            // 43ef6 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 43efd mov BYTE PTR [rbp-0x2d],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3), (0x0u64) as u8);
            // 43f01 lea rbx,[r12+0x8]
            rbx = r12.wrapping_add(0x8);
            // 43f06 mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 43f0b mov rdi,r12
            rdi = r12;
            // 43f0e call 0xebb6a
            { m.outregs(0x43f13, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f13 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 43f1c mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 43f20 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 43f29 cmp r14d,0x7
            fl = sub::<32>((r14 as u32 as u64), 0x7u64).1;
            // 43f2d ja 0x43fae
            if crate::cov::br(fl.a()) { pc = 0x43fae; continue; }
            pc = 0x43f2f;
        }
        0x43f2f => {
            // block 43f2f
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f2f movzx ebx,r14w
            rbx = ((r14 as u16 as u64)) as u32 as u64;
            // 43f33 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43f37 call 0xcf820
            { m.outregs(0x43f3c, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcf820); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f3c movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 43f3f cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // 43f41 jae 0x43fae
            if crate::cov::br(fl.ae()) { pc = 0x43fae; continue; }
            pc = 0x43f43;
        }
        0x43f43 => {
            // block 43f43
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f43 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43f46 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 43f4a mov rdi,r15
            rdi = r15;
            // 43f4d mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 43f50 call QWORD PTR [rax+0x688]
            { m.outregs(0x43f56, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x688)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f56 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43f58 je 0x43fae
            if crate::cov::br(fl.e()) { pc = 0x43fae; continue; }
            pc = 0x43f5a;
        }
        0x43f5a => {
            // block 43f5a
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f5a mov ebx,DWORD PTR [rbp-0x2c]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 43f5d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43f60 lea rdx,[rbp-0x2d]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd3);
            // 43f64 mov rdi,r15
            rdi = r15;
            // 43f67 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 43f6a call QWORD PTR [rax+0x7e0]
            { m.outregs(0x43f70, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x7e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f70 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 43f72 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43f74 mov r13d,0x0
            r13 = (0x0u64) as u32 as u64;
            // 43f7a je 0x43fb1
            if crate::cov::br(fl.e()) { pc = 0x43fb1; continue; }
            pc = 0x43f7c;
        }
        0x43f7c => {
            // block 43f7c
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f7c cmp BYTE PTR [rbp-0x2d],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd3)) as u64, 0x0u64).1;
            // 43f80 je 0x43fc3
            if crate::cov::br(fl.e()) { pc = 0x43fc3; continue; }
            pc = 0x43f82;
        }
        0x43f82 => {
            // block 43f82
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f82 mov r15d,ebx
            r15 = ((rbx as u32 as u64)) as u32 as u64;
            // 43f85 shr r15d,1
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x1u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 43f88 mov bx,0x1
            rbx = (rbx & !0xffff) | ((0x1u64) as u16 as u64);
            // 43f8c lea r14,[rbp-0x34]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            pc = 0x43f90;
        }
        0x43f90 => {
            // block 43f90
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43f90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43f90 mov DWORD PTR [rbp-0x34],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rcx as u32 as u64)) as u32);
            // 43f93 mov rdi,r12
            rdi = r12;
            // 43f96 mov rsi,r14
            rsi = r14;
            // 43f99 call 0xebb58
            { m.outregs(0x43f9e, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x325); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43f9e movzx ecx,bx
            rcx = ((rbx as u16 as u64)) as u32 as u64;
            // 43fa1 lea ebx,[rbx+0x1]
            rbx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 43fa4 cmp ecx,r15d
            fl = sub::<32>((rcx as u32 as u64), (r15 as u32 as u64)).1;
            // 43fa7 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 43faa jbe 0x43f90
            if crate::cov::br(fl.be()) { pc = 0x43f90; continue; }
            pc = 0x43fac;
        }
        0x43fac => {
            // block 43fac
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fac jmp 0x43fb1
            pc = 0x43fb1; continue;
        }
        0x43fae => {
            // block 43fae
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fae xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x43fb1;
        }
        0x43fb1 => {
            // block 43fb1
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fb1 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 43fb4 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 43fb8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43fb9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 43fbb pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 43fbd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43fbf pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43fc1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43fc2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x43fc3 => {
            // block 43fc3
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fc3 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43fc7 call 0xcc811
            { m.outregs(0x43fcc, 0xd, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcc811); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43fcc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43fce je 0x44036
            if crate::cov::br(fl.e()) { pc = 0x44036; continue; }
            pc = 0x43fd0;
        }
        0x43fd0 => {
            // block 43fd0
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 43fd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43fd0 mov DWORD PTR [rbp-0x38],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u32);
            // 43fd7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43fda cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // 43fde setg cl
            rcx = (rcx & !0xff) | (((fl.g() as u64)) as u8 as u64);
            // 43fe1 movzx esi,cl
            rsi = ((rcx as u8 as u64)) as u32 as u64;
            // 43fe4 or esi,0x1f0
            { let r = (rsi as u32 as u64) | 0x1f0u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 43fea lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 43fee xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 43ff1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 43ff6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 43ff9 mov rdi,r15
            rdi = r15;
            // 43ffc call QWORD PTR [rax+0x38]
            { m.outregs(0x43fff, 0x1d, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43fff test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44001 je 0x43fb1
            if crate::cov::br(fl.e()) { pc = 0x43fb1; continue; }
            pc = 0x44003;
        }
        0x44003 => {
            // block 44003
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44003 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44003 mov DWORD PTR [rbp-0x3c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0x0u64) as u32);
            // 4400a shl r14d,0x3
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x3u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 4400e lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0x44012;
        }
        0x44012 => {
            // block 44012
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44012 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 44015 lea ecx,[r14+r13*1]
            rcx = (r14.wrapping_add(r13)) as u32 as u64;
            // 44019 bt eax,ecx
            fl.cf = bit::<32>((rax as u32 as u64), (rcx as u32 as u64));
            // 4401c jae 0x44029
            if crate::cov::br(fl.ae()) { pc = 0x44029; continue; }
            pc = 0x4401e;
        }
        0x4401e => {
            // block 4401e
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 4401e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4401e mov rdi,r12
            rdi = r12;
            // 44021 mov rsi,r15
            rsi = r15;
            // 44024 call 0xebb52
            { m.outregs(0x44029, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x31f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x44029;
        }
        0x44029 => {
            // block 44029
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44029 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44029 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // 4402c mov DWORD PTR [rbp-0x3c],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((r13 as u32 as u64)) as u32);
            // 44030 cmp r13d,0x8
            fl = sub::<32>((r13 as u32 as u64), 0x8u64).1;
            // 44034 jb 0x44012
            if crate::cov::br(fl.b()) { pc = 0x44012; continue; }
            pc = 0x44036;
        }
        0x44036 => {
            // block 44036
            #[cfg(feature = "trace")] eprintln!("f_00043edc pc 44036 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44036 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 44039 jmp 0x43fb1
            pc = 0x43fb1; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
