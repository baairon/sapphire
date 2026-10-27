//! Mechanically lifted functions (scripts/lift_run.py), module h009. Each one is checked against the original
//! in the world harness (tests/world_h009.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xb7c4a size=264
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7c4a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb7c4a;
    loop {
        match pc {
        0xb7c4a => {
            // block b7c4a
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7c4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c4a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7c4b mov rbp,rsp
            rbp = rsp;
            // b7c4e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b7c50 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7c52 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b7c54 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b7c56 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7c57 sub rsp,0x468
            { let (r, f) = sub::<64>(rsp, 0x468u64);
              rsp = r;
              fl = f; }
            // b7c5e mov r15,rcx
            r15 = rcx;
            // b7c61 mov r12,rdx
            r12 = rdx;
            // b7c64 mov rbx,rdi
            rbx = rdi;
            // b7c67 mov rax,QWORD PTR [rip+0x8876a] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // b7c6e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b7c71 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b7c75 test BYTE PTR [r12],0x1
            fl = logic::<8>(m.r8(r12) as u64 & 0x1u64);
            // b7c7a jne 0xb7c85
            if fl.ne() { pc = 0xb7c85; continue; }
            pc = 0xb7c7c;
        }
        0xb7c7c => {
            // block b7c7c
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7c7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c7c mov WORD PTR [r12],0x0
            m.w16(r12, (0x0u64) as u16);
            // b7c83 jmp 0xb7c96
            pc = 0xb7c96; continue;
        }
        0xb7c85 => {
            // block b7c85
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7c85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c85 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // b7c8a mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7c8d mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7c96;
        }
        0xb7c96 => {
            // block b7c96
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7c96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c96 test BYTE PTR [r15],0x1
            fl = logic::<8>(m.r8(r15) as u64 & 0x1u64);
            // b7c9a jne 0xb7ca4
            if fl.ne() { pc = 0xb7ca4; continue; }
            pc = 0xb7c9c;
        }
        0xb7c9c => {
            // block b7c9c
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7c9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c9c mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // b7ca2 jmp 0xb7cb3
            pc = 0xb7cb3; continue;
        }
        0xb7ca4 => {
            // block b7ca4
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7ca4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ca4 mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // b7ca8 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7cab mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7cb3;
        }
        0xb7cb3 => {
            // block b7cb3
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7cb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7cb3 mov DWORD PTR [rsi],0x0
            m.w32(rsi, (0x0u64) as u32);
            // b7cb9 cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // b7cbd je 0xb7d24
            if fl.e() { pc = 0xb7d24; continue; }
            pc = 0xb7cbf;
        }
        0xb7cbf => {
            // block b7cbf
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7cbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7cbf mov QWORD PTR [rbp-0x490],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb70), (rsi) as u64);
            // b7cc6 lea r14,[rbp-0x488]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // b7ccd mov esi,0x454
            rsi = (0x454u64) as u32 as u64;
            // b7cd2 mov rdi,r14
            rdi = r14;
            // b7cd5 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7cda mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7cdd xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // b7ce0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b7ce2 mov rdi,rbx
            rdi = rbx;
            // b7ce5 mov rsi,r14
            rsi = r14;
            // b7ce8 call QWORD PTR [rax+0x140]
            { let r = { let t = m.r64(rax.wrapping_add(0x140)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7cee test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7cf0 je 0xb7d27
            if fl.e() { pc = 0xb7d27; continue; }
            pc = 0xb7cf2;
        }
        0xb7cf2 => {
            // block b7cf2
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7cf2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7cf2 lea rsi,[rbp-0x478]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb88);
            // b7cf9 mov rdi,r12
            rdi = r12;
            // b7cfc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7d01 lea rsi,[rbp-0x468]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb98);
            // b7d08 mov rdi,r15
            rdi = r15;
            // b7d0b call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7d10 mov eax,DWORD PTR [rbp-0x47c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fb84)) as u64) as u32 as u64;
            // b7d16 mov rcx,QWORD PTR [rbp-0x490]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb70)) as u64;
            // b7d1d mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // b7d1f mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // b7d22 jmp 0xb7d27
            pc = 0xb7d27; continue;
        }
        0xb7d24 => {
            // block b7d24
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7d24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7d24 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xb7d27;
        }
        0xb7d27 => {
            // block b7d27
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7d27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7d27 mov rax,QWORD PTR [rip+0x886aa] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // b7d2e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b7d31 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // b7d35 jne 0xb7d4c
            if fl.ne() { pc = 0xb7d4c; continue; }
            pc = 0xb7d37;
        }
        0xb7d37 => {
            // block b7d37
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7d37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7d37 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // b7d3a add rsp,0x468
            { let (r, f) = add::<64>(rsp, 0x468u64);
              rsp = r;
              fl = f; }
            // b7d41 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7d42 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b7d44 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b7d46 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7d48 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b7d4a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7d4b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb7d4c => {
            // block b7d4c
            #[cfg(feature = "trace")] eprintln!("f_000b7c4a pc b7d4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7d4c call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9608 size=312
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9608<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9608;
    loop {
        match pc {
        0xe9608 => {
            // block e9608
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9608 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9608 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9609 mov rbp,rsp
            rbp = rsp;
            // e960c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e960e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9610 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9611 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e9612 mov r14,rsi
            r14 = rsi;
            // e9615 mov rbx,rdi
            rbx = rdi;
            // e9618 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e961f movzx eax,BYTE PTR [rdi]
            rax = (m.r8(rdi) as u64) as u32 as u64;
            // e9622 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e9624 jne 0xe962b
            if fl.ne() { pc = 0xe962b; continue; }
            pc = 0xe9626;
        }
        0xe9626 => {
            // block e9626
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9626 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9626 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // e9629 jmp 0xe962f
            pc = 0xe962f; continue;
        }
        0xe962b => {
            // block e962b
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e962b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e962b mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            pc = 0xe962f;
        }
        0xe962f => {
            // block e962f
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e962f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e962f test rax,rax
            fl = logic::<64>(rax & rax);
            // e9632 je 0xe9662
            if fl.e() { pc = 0xe9662; continue; }
            pc = 0xe9634;
        }
        0xe9634 => {
            // block e9634
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9634 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9634 lea rsi,[rip+0x407ed] # 0x129e28
            rsi = m.env.data_va(255477);
            // e963b call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9640 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9642 je 0xe9662
            if fl.e() { pc = 0xe9662; continue; }
            pc = 0xe9644;
        }
        0xe9644 => {
            // block e9644
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9644 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9644 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e964b mov WORD PTR [rbp-0x1e],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffe2), (0x0u64) as u16);
            // e9651 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e9658 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e965b jne 0xe966e
            if fl.ne() { pc = 0xe966e; continue; }
            pc = 0xe965d;
        }
        0xe965d => {
            // block e965d
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e965d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e965d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e9660 jmp 0xe9672
            pc = 0xe9672; continue;
        }
        0xe9662 => {
            // block e9662
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9662 mov DWORD PTR [r14+0x8],0x0
            m.w32(r14.wrapping_add(0x8), (0x0u64) as u32);
            // e966a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e966c jmp 0xe96ca
            pc = 0xe96ca; continue;
        }
        0xe966e => {
            // block e966e
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e966e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e966e mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe9672;
        }
        0xe9672 => {
            // block e9672
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9672 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9672 lea r15,[r14+0x2]
            r15 = r14.wrapping_add(0x2);
            // e9676 lea rsi,[rip+0x53caa] # 0x13d327
            rsi = m.env.data_va(334580);
            // e967d lea rdx,[rbp-0x1e]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe2);
            // e9681 lea r8,[rbp-0x1c]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e9685 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9687 mov rcx,r15
            rcx = r15;
            // e968a call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e968f mov al,BYTE PTR [rbp-0x1e]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe2)) as u64) as u8 as u64);
            // e9692 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e9695 mov DWORD PTR [r14+0x4],0x0
            m.w32(r14.wrapping_add(0x4), (0x0u64) as u32);
            // e969d mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e96a0 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // e96a4 ja 0xe96c0
            if fl.a() { pc = 0xe96c0; continue; }
            pc = 0xe96a6;
        }
        0xe96a6 => {
            // block e96a6
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96a6 lea rcx,[rip+0x7f] # 0xe972c
            rcx = m.env.code_va(0xe972c);
            // e96ad movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // e96b1 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // e96b4 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xe96b6 => {
            // block e96b6
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96b6 mov DWORD PTR [r14+0x8],0x1
            m.w32(r14.wrapping_add(0x8), (0x1u64) as u32);
            // e96be jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96c0 => {
            // block e96c0
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96c0 mov DWORD PTR [r14+0x8],0x0
            m.w32(r14.wrapping_add(0x8), (0x0u64) as u32);
            pc = 0xe96c8;
        }
        0xe96c8 => {
            // block e96c8
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96c8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe96ca;
        }
        0xe96ca => {
            // block e96ca
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96ca add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e96ce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e96cf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e96d1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e96d3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e96d4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe96d5 => {
            // block e96d5
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96d5 mov DWORD PTR [r14+0x8],0x2
            m.w32(r14.wrapping_add(0x8), (0x2u64) as u32);
            // e96dd jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96df => {
            // block e96df
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96df mov DWORD PTR [r14+0x8],0x3
            m.w32(r14.wrapping_add(0x8), (0x3u64) as u32);
            // e96e7 jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96e9 => {
            // block e96e9
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96e9 mov DWORD PTR [r14+0x8],0x4
            m.w32(r14.wrapping_add(0x8), (0x4u64) as u32);
            // e96f1 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e96f8 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e96fb jne 0xe9702
            if fl.ne() { pc = 0xe9702; continue; }
            pc = 0xe96fd;
        }
        0xe96fd => {
            // block e96fd
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e96fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e96fd inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e9700 jmp 0xe9706
            pc = 0xe9706; continue;
        }
        0xe9702 => {
            // block e9702
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9702 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9702 mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe9706;
        }
        0xe9706 => {
            // block e9706
            #[cfg(feature = "trace")] eprintln!("f_000e9608 pc e9706 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9706 lea rsi,[rip+0x53c2a] # 0x13d337
            rsi = m.env.data_va(334596);
            // e970d lea rdx,[rbp-0x1e]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe2);
            // e9711 lea r8,[rbp-0x1c]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e9715 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9717 mov r9,r15
            r9 = r15;
            // e971a mov rcx,r15
            rcx = r15;
            // e971d call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9722 mov al,BYTE PTR [rbp-0x1e]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe2)) as u64) as u8 as u64);
            // e9725 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e9728 jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe8100 size=404
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e8100<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe8100;
    loop {
        match pc {
        0xe8100 => {
            // block e8100
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8100 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8100 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e8101 mov rbp,rsp
            rbp = rsp;
            // e8104 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e8106 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e8108 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e810a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e810c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e810d sub rsp,0x498
            { let (r, f) = sub::<64>(rsp, 0x498u64);
              rsp = r;
              fl = f; }
            // e8114 mov r14,rsi
            r14 = rsi;
            // e8117 mov rbx,rdi
            rbx = rdi;
            // e811a mov rax,QWORD PTR [rip+0x582b7] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // e8121 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // e8124 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // e8128 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e812b call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e812e lea r15,[rbx+0x290]
            r15 = rbx.wrapping_add(0x290);
            // e8135 test BYTE PTR [rbx+0x290],0x1
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x290)) as u64 & 0x1u64);
            // e813c jne 0xe8146
            if fl.ne() { pc = 0xe8146; continue; }
            pc = 0xe813e;
        }
        0xe813e => {
            // block e813e
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e813e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e813e mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // e8144 jmp 0xe815b
            pc = 0xe815b; continue;
        }
        0xe8146 => {
            // block e8146
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8146 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8146 mov rax,QWORD PTR [rbx+0x2a0]
            rax = m.r64(rbx.wrapping_add(0x2a0)) as u64;
            // e814d mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // e8150 mov QWORD PTR [rbx+0x298],0x0
            m.w64(rbx.wrapping_add(0x298), (0x0u64) as u64);
            pc = 0xe815b;
        }
        0xe815b => {
            // block e815b
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e815b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e815b test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // e815f jne 0xe8167
            if fl.ne() { pc = 0xe8167; continue; }
            pc = 0xe8161;
        }
        0xe8161 => {
            // block e8161
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8161 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8161 lea rdi,[r14+0x1]
            rdi = r14.wrapping_add(0x1);
            // e8165 jmp 0xe816b
            pc = 0xe816b; continue;
        }
        0xe8167 => {
            // block e8167
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8167 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8167 mov rdi,QWORD PTR [r14+0x10]
            rdi = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xe816b;
        }
        0xe816b => {
            // block e816b
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e816b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e816b lea rsi,[rbp-0x4b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // e8172 call 0xec2a2
            { let r = { let t = m.env.stub_va(0xa6f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8177 mov eax,DWORD PTR [rbp-0x450]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64) as u32 as u64;
            // e817d mov DWORD PTR [rbx+0x260],eax
            m.w32(rbx.wrapping_add(0x260), ((rax as u32 as u64)) as u32);
            // e8183 lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // e818a call 0xec1fa
            { let r = { let t = m.env.stub_va(0x9c7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e818f mov r12,rax
            r12 = rax;
            // e8192 lea r13,[rbp-0x4b8]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // e8199 mov rdi,r13
            rdi = r13;
            // e819c call 0xec2ea
            { let r = { let t = m.env.stub_va(0xab7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81a1 mov rdi,r13
            rdi = r13;
            // e81a4 call 0xec1fa
            { let r = { let t = m.env.stub_va(0x9c7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81a9 mov rdi,rax
            rdi = rax;
            // e81ac call 0xec1a0
            { let r = { let t = m.env.stub_va(0x96d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81b1 mov r13,rax
            r13 = rax;
            // e81b4 mov rdi,r12
            rdi = r12;
            // e81b7 call 0xec1a0
            { let r = { let t = m.env.stub_va(0x96d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81bc mov rcx,rax
            rcx = rax;
            // e81bf lea rsi,[rip+0x55048] # 0x13d20e
            rsi = m.env.data_va(334299);
            // e81c6 lea r12,[rbp-0x420]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fbe0);
            // e81cd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e81cf mov rdi,r12
            rdi = r12;
            // e81d2 mov rdx,r13
            rdx = r13;
            // e81d5 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81da mov rdi,r15
            rdi = r15;
            // e81dd mov rsi,r12
            rsi = r12;
            // e81e0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81e5 lea r15,[rbx+0x8]
            r15 = rbx.wrapping_add(0x8);
            // e81e9 test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // e81ed jne 0xe81f4
            if fl.ne() { pc = 0xe81f4; continue; }
            pc = 0xe81ef;
        }
        0xe81ef => {
            // block e81ef
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e81ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e81ef inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // e81f2 jmp 0xe81f8
            pc = 0xe81f8; continue;
        }
        0xe81f4 => {
            // block e81f4
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e81f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e81f4 mov r14,QWORD PTR [r14+0x10]
            r14 = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xe81f8;
        }
        0xe81f8 => {
            // block e81f8
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e81f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e81f8 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // e81fd mov rdi,r15
            rdi = r15;
            // e8200 mov rsi,r14
            rsi = r14;
            // e8203 call 0xeb918
            { let r = { let t = m.env.stub_va(0xe5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8208 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e820b mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e820f test BYTE PTR [rax+r15*1+0x20],0x5
            fl = logic::<8>(m.r8(rax.wrapping_add(r15).wrapping_add(0x20)) as u64 & 0x5u64);
            // e8215 je 0xe821c
            if fl.e() { pc = 0xe821c; continue; }
            pc = 0xe8217;
        }
        0xe8217 => {
            // block e8217
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8217 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8217 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e821a jmp 0xe8269
            pc = 0xe8269; continue;
        }
        0xe821c => {
            // block e821c
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e821c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e821c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e821f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e8221 mov rdi,rbx
            rdi = rbx;
            // e8224 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8227 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e822a mov rdi,rbx
            rdi = rbx;
            // e822d call QWORD PTR [rax+0xa0]
            { let r = { let t = m.r64(rax.wrapping_add(0xa0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8233 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // e8236 cmp QWORD PTR [rbx+0x90],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x90)) as u64, 0x0u64).1;
            // e823e je 0xe8269
            if fl.e() { pc = 0xe8269; continue; }
            pc = 0xe8240;
        }
        0xe8240 => {
            // block e8240
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8240 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8240 add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // e8244 mov rdi,rbx
            rdi = rbx;
            // e8247 call 0xebf12
            { let r = { let t = m.env.stub_va(0x6df); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e824c test rax,rax
            fl = logic::<64>(rax & rax);
            // e824f jne 0xe8269
            if fl.ne() { pc = 0xe8269; continue; }
            pc = 0xe8251;
        }
        0xe8251 => {
            // block e8251
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8251 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8251 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e8254 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e8258 lea rdi,[r15+rax*1]
            rdi = r15.wrapping_add(rax);
            // e825c mov esi,DWORD PTR [r15+rax*1+0x20]
            rsi = (m.r32(r15.wrapping_add(rax).wrapping_add(0x20)) as u64) as u32 as u64;
            // e8261 or esi,0x4
            { let r = (rsi as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e8264 call 0xec0fe
            { let r = { let t = m.env.stub_va(0x8cb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe8269;
        }
        0xe8269 => {
            // block e8269
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8269 mov rax,QWORD PTR [rip+0x58168] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // e8270 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // e8273 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // e8277 jne 0xe828e
            if fl.ne() { pc = 0xe828e; continue; }
            pc = 0xe8279;
        }
        0xe8279 => {
            // block e8279
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8279 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8279 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // e827c add rsp,0x498
            { let (r, f) = add::<64>(rsp, 0x498u64);
              rsp = r;
              fl = f; }
            // e8283 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e8284 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e8286 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e8288 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e828a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e828c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e828d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe828e => {
            // block e828e
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e828e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e828e call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1044 size=418
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e1044<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe1044;
    loop {
        match pc {
        0xe1044 => {
            // block e1044
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1044 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1044 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1045 mov rbp,rsp
            rbp = rsp;
            // e1048 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e104a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e104c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e104e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e1050 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1051 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e1052 mov DWORD PTR [rbp-0x30],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rdx as u32 as u64)) as u32);
            // e1055 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e1058 mov r12,rdi
            r12 = rdi;
            // e105b mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e105f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e1061 call QWORD PTR [rax+0x3c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3c0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1067 mov r14,rax
            r14 = rax;
            // e106a test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // e106d je 0xe11d6
            if fl.e() { pc = 0xe11d6; continue; }
            pc = 0xe1073;
        }
        0xe1073 => {
            // block e1073
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1073 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1073 mov rbx,r14
            rbx = r14;
            // e1076 shr rbx,0x20
            { let (r, f) = shr::<64>(rbx, 0x20u64, fl); rbx = r; fl = f; }
            // e107a test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // e107c je 0xe11d6
            if fl.e() { pc = 0xe11d6; continue; }
            pc = 0xe1082;
        }
        0xe1082 => {
            // block e1082
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1082 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1082 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e1086 mov rdi,r12
            rdi = r12;
            // e1089 call QWORD PTR [rax+0x1a90]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a90)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e108f cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e1092 jne 0xe10b4
            if fl.ne() { pc = 0xe10b4; continue; }
            pc = 0xe1094;
        }
        0xe1094 => {
            // block e1094
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1094 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1094 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e1097 cmp ebx,0x2cf
            fl = sub::<32>((rbx as u32 as u64), 0x2cfu64).1;
            // e109d jg 0xe10ea
            if fl.g() { pc = 0xe10ea; continue; }
            pc = 0xe109f;
        }
        0xe109f => {
            // block e109f
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e109f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e109f cmp ebx,0x1e6
            fl = sub::<32>((rbx as u32 as u64), 0x1e6u64).1;
            // e10a5 jne 0xe110b
            if fl.ne() { pc = 0xe110b; continue; }
            pc = 0xe10a7;
        }
        0xe10a7 => {
            // block e10a7
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10a7 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e10ac mov r13d,0xf4
            r13 = (0xf4u64) as u32 as u64;
            // e10b2 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe10b4 => {
            // block e10b4
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10b4 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // e10b7 jne 0xe11d6
            if fl.ne() { pc = 0xe11d6; continue; }
            pc = 0xe10bd;
        }
        0xe10bd => {
            // block e10bd
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10bd cmp r14d,0x2d0
            fl = sub::<32>((r14 as u32 as u64), 0x2d0u64).1;
            // e10c4 je 0xe1125
            if fl.e() { pc = 0xe1125; continue; }
            pc = 0xe10c6;
        }
        0xe10c6 => {
            // block e10c6
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10c6 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // e10cd je 0xe10e0
            if fl.e() { pc = 0xe10e0; continue; }
            pc = 0xe10cf;
        }
        0xe10cf => {
            // block e10cf
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10cf xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e10d2 cmp r14d,0x780
            fl = sub::<32>((r14 as u32 as u64), 0x780u64).1;
            // e10d9 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e10de jne 0xe1130
            if fl.ne() { pc = 0xe1130; continue; }
            pc = 0xe10e0;
        }
        0xe10e0 => {
            // block e10e0
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10e0 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e10e5 mov r13d,r14d
            r13 = ((r14 as u32 as u64)) as u32 as u64;
            // e10e8 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe10ea => {
            // block e10ea
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10ea cmp ebx,0x2d0
            fl = sub::<32>((rbx as u32 as u64), 0x2d0u64).1;
            // e10f0 je 0xe10ff
            if fl.e() { pc = 0xe10ff; continue; }
            pc = 0xe10f2;
        }
        0xe10f2 => {
            // block e10f2
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10f2 cmp ebx,0x438
            fl = sub::<32>((rbx as u32 as u64), 0x438u64).1;
            // e10f8 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e10fd jne 0xe1130
            if fl.ne() { pc = 0xe1130; continue; }
            pc = 0xe10ff;
        }
        0xe10ff => {
            // block e10ff
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10ff inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // e1101 mov eax,0x7
            rax = (0x7u64) as u32 as u64;
            // e1106 mov r13d,ebx
            r13 = ((rbx as u32 as u64)) as u32 as u64;
            // e1109 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe110b => {
            // block e110b
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e110b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e110b cmp ebx,0x240
            fl = sub::<32>((rbx as u32 as u64), 0x240u64).1;
            // e1111 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e1116 jne 0xe1130
            if fl.ne() { pc = 0xe1130; continue; }
            pc = 0xe1118;
        }
        0xe1118 => {
            // block e1118
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1118 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1118 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e111d mov r13d,0x121
            r13 = (0x121u64) as u32 as u64;
            // e1123 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe1125 => {
            // block e1125
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1125 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1125 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e112a mov r13d,0x5a0
            r13 = (0x5a0u64) as u32 as u64;
            pc = 0xe1130;
        }
        0xe1130 => {
            // block e1130
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1130 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1130 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e1132 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e1136 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // e113a mov rdi,r12
            rdi = r12;
            // e113d call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1143 mov ecx,0xc0000000
            rcx = (0xc0000000u64) as u32 as u64;
            // e1148 and ecx,DWORD PTR [rbp-0x2c]
            { let r = (rcx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e114b mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // e114e imul r15d,r13d
            { let (r, f) = imul::<32>((r15 as u32 as u64), (r13 as u32 as u64)); r15 = (r) as u32 as u64; fl = f; }
            // e1152 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // e1155 js 0xe1167
            if fl.s() { pc = 0xe1167; continue; }
            pc = 0xe1157;
        }
        0xe1157 => {
            // block e1157
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1157 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1157 add r15d,0x8000
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x8000u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // e115e sar r15d,0x10
            { let (r, f) = sar::<32>((r15 as u32 as u64), 0x10u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // e1162 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e1165 jmp 0xe1174
            pc = 0xe1174; continue;
        }
        0xe1167 => {
            // block e1167
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1167 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1167 mov esi,0x8000
            rsi = (0x8000u64) as u32 as u64;
            // e116c sub esi,r15d
            { let (r, f) = sub::<32>((rsi as u32 as u64), (r15 as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e116f sar esi,0x10
            { let (r, f) = sar::<32>((rsi as u32 as u64), 0x10u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e1172 neg esi
            { let (r, f) = neg::<32>((rsi as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            pc = 0xe1174;
        }
        0xe1174 => {
            // block e1174
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1174 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1174 mov edi,DWORD PTR [rbp-0x30]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e1177 dec r13d
            { let (r, f) = dec::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // e117a imul r13d,edi
            { let (r, f) = imul::<32>((r13 as u32 as u64), (rdi as u32 as u64)); r13 = (r) as u32 as u64; fl = f; }
            // e117e cmp r13d,0xfffffffe
            fl = sub::<32>((r13 as u32 as u64), 0xfffffffeu64).1;
            // e1182 jg 0xe1193
            if fl.g() { pc = 0xe1193; continue; }
            pc = 0xe1184;
        }
        0xe1184 => {
            // block e1184
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1184 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1184 mov edx,0x7fff
            rdx = (0x7fffu64) as u32 as u64;
            // e1189 sub edx,r13d
            { let (r, f) = sub::<32>((rdx as u32 as u64), (r13 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e118c sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e118f neg edx
            { let (r, f) = neg::<32>((rdx as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // e1191 jmp 0xe11a1
            pc = 0xe11a1; continue;
        }
        0xe1193 => {
            // block e1193
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1193 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1193 add r13d,0x8001
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x8001u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e119a sar r13d,0x10
            { let (r, f) = sar::<32>((r13 as u32 as u64), 0x10u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // e119e mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            pc = 0xe11a1;
        }
        0xe11a1 => {
            // block e11a1
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11a1 mov eax,0x1fff0000
            rax = (0x1fff0000u64) as u32 as u64;
            // e11a6 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // e11a8 je 0xe11c0
            if fl.e() { pc = 0xe11c0; continue; }
            pc = 0xe11aa;
        }
        0xe11aa => {
            // block e11aa
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11aa test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e11ac mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // e11b1 cmovne edi,edx
            if fl.ne() { rdi = ((rdx as u32 as u64)) as u32 as u64; } else { rdi = rdi as u32 as u64; }
            // e11b4 mov eax,0x1fff
            rax = (0x1fffu64) as u32 as u64;
            // e11b9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e11bb div edi
            { let (q, r) = div::<32>(rdx, rax, (rdi as u32 as u64)); rax = q; rdx = r; }
            // e11bd shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0xe11c0;
        }
        0xe11c0 => {
            // block e11c0
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11c0 mov rdx,QWORD PTR [r12]
            rdx = m.r64(r12) as u64;
            // e11c4 add esi,ebx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e11c6 shl esi,0x2
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x2u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e11c9 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e11cb or esi,ecx
            { let r = (rsi as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e11cd mov rdi,r12
            rdi = r12;
            // e11d0 call QWORD PTR [rdx+0x1ad0]
            { let r = { let t = m.r64(rdx.wrapping_add(0x1ad0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe11d6;
        }
        0xe11d6 => {
            // block e11d6
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11d6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e11da pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e11db pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e11dd pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e11df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e11e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e11e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e11e4 ret
            return Ret { rax, rdx, xmm0: x0 };
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
            { let r = { let t = m.env.stub_va(0x6d9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(r10.wrapping_add(0x18)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea1a7 mov rbx,rax
            rbx = rax;
            // ea1aa mov rdi,rbx
            rdi = rbx;
            // ea1ad call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea1b2 mov rsi,QWORD PTR [rip+0x56027] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // ea1b9 mov rdx,QWORD PTR [rip+0x55fe8] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // ea1c0 mov rdi,rbx
            rdi = rbx;
            // ea1c3 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
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

/// parity: P2v @0xe938c size=496
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e938c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe938c;
    loop {
        match pc {
        0xe938c => {
            // block e938c
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e938c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e938c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e938d mov rbp,rsp
            rbp = rsp;
            // e9390 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9392 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9394 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e9396 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e9398 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9399 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // e939d mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e93a0 mov r14,rdi
            r14 = rdi;
            // e93a3 lea rsi,[rip+0x53f63] # 0x13d30d
            rsi = m.env.data_va(334554);
            // e93aa lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e93ae mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // e93b3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e93b8 test BYTE PTR [rbp-0x40],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64 & 0x1u64);
            // e93bc jne 0xe93c7
            if fl.ne() { pc = 0xe93c7; continue; }
            pc = 0xe93be;
        }
        0xe93be => {
            // block e93be
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e93be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e93be lea r12,[rbp-0x3f]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // e93c2 mov rdi,r12
            rdi = r12;
            // e93c5 jmp 0xe93cf
            pc = 0xe93cf; continue;
        }
        0xe93c7 => {
            // block e93c7
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e93c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e93c7 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e93cb lea r12,[rbp-0x3f]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            pc = 0xe93cf;
        }
        0xe93cf => {
            // block e93cf
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e93cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e93cf add rdi,0x9
            { let (r, f) = add::<64>(rdi, 0x9u64);
              rdi = r;
              fl = f; }
            // e93d3 movzx edx,r15w
            rdx = ((r15 as u16 as u64)) as u32 as u64;
            // e93d7 lea rsi,[rip+0x53f3f] # 0x13d31d
            rsi = m.env.data_va(334570);
            // e93de xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e93e0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e93e2 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e93e7 mov cl,BYTE PTR [rbp-0x40]
            rcx = (rcx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u8 as u64);
            // e93ea and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e93ed mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e93f2 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xe93f6;
        }
        0xe93f6 => {
            // block e93f6
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e93f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e93f6 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e93f8 mov rax,rdi
            rax = rdi;
            // e93fb cmove rax,r12
            if fl.e() { rax = r12; }
            // e93ff movzx eax,BYTE PTR [rax+rsi*1]
            rax = (m.r8(rax.wrapping_add(rsi)) as u64) as u32 as u64;
            // e9403 lea eax,[rbx+rax*1+0xd0]
            rax = (rbx.wrapping_add(rax).wrapping_add(0xd0)) as u32 as u64;
            // e940a inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // e940d movzx ebx,al
            rbx = ((rax as u8 as u64)) as u32 as u64;
            // e9410 cmp rsi,0xd
            fl = sub::<64>(rsi, 0xdu64).1;
            // e9414 jne 0xe93f6
            if fl.ne() { pc = 0xe93f6; continue; }
            pc = 0xe9416;
        }
        0xe9416 => {
            // block e9416
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9416 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9416 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e9418 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e941a cmove rdi,r12
            if fl.e() { rdi = r12; }
            // e941e add rdi,0xd
            { let (r, f) = add::<64>(rdi, 0xdu64);
              rdi = r;
              fl = f; }
            // e9422 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // e9425 lea rsi,[rip+0x53ef6] # 0x13d322
            rsi = m.env.data_va(334575);
            // e942c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e942e call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9433 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e9437 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e943b lea r8,[rbp-0x60]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e943f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e9441 mov ecx,0xd
            rcx = (0xdu64) as u32 as u64;
            // e9446 call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e944b mov rbx,QWORD PTR [r14+0x268]
            rbx = m.r64(r14.wrapping_add(0x268)) as u64;
            // e9452 mov rax,QWORD PTR [r14+0x270]
            rax = m.r64(r14.wrapping_add(0x270)) as u64;
            // e9459 mov QWORD PTR [r14+0x280],rax
            m.w64(r14.wrapping_add(0x280), (rax) as u64);
            // e9460 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // e9463 je 0xe952d
            if fl.e() { pc = 0xe952d; continue; }
            pc = 0xe9469;
        }
        0xe9469 => {
            // block e9469
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9469 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9469 lea r13,[rbp-0x77]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff89);
            // e946d lea r15,[rbp-0x78]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e9471 lea r12,[rbp-0x80]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            pc = 0xe9475;
        }
        0xe9475 => {
            // block e9475
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9475 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9475 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e9477 mov ecx,0xd
            rcx = (0xdu64) as u32 as u64;
            // e947c mov rdi,r15
            rdi = r15;
            // e947f mov rsi,rbx
            rsi = rbx;
            // e9482 mov r8,r12
            r8 = r12;
            // e9485 call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e948a movzx edx,BYTE PTR [rbp-0x58]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // e948e mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // e9490 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e9492 jne 0xe9499
            if fl.ne() { pc = 0xe9499; continue; }
            pc = 0xe9494;
        }
        0xe9494 => {
            // block e9494
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9494 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9494 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e9497 jmp 0xe949d
            pc = 0xe949d; continue;
        }
        0xe9499 => {
            // block e9499
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9499 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9499 mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0xe949d;
        }
        0xe949d => {
            // block e949d
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e949d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e949d movzx esi,BYTE PTR [rbp-0x78]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // e94a1 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // e94a4 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e94a7 jne 0xe94ae
            if fl.ne() { pc = 0xe94ae; continue; }
            pc = 0xe94a9;
        }
        0xe94a9 => {
            // block e94a9
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94a9 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // e94ac jmp 0xe94b2
            pc = 0xe94b2; continue;
        }
        0xe94ae => {
            // block e94ae
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94ae mov rsi,QWORD PTR [rbp-0x70]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0xe94b2;
        }
        0xe94b2 => {
            // block e94b2
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94b2 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // e94b5 jne 0xe94fb
            if fl.ne() { pc = 0xe94fb; continue; }
            pc = 0xe94b7;
        }
        0xe94b7 => {
            // block e94b7
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94b7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e94b9 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e94bd cmove rsi,r13
            if fl.e() { rsi = r13; }
            // e94c1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e94c3 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e94c7 lea rax,[rbp-0x57]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa9);
            // e94cb cmove rdi,rax
            if fl.e() { rdi = rax; }
            // e94cf jne 0xe94ed
            if fl.ne() { pc = 0xe94ed; continue; }
            pc = 0xe94d1;
        }
        0xe94d1 => {
            // block e94d1
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94d1 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e94d4 je 0xe9516
            if fl.e() { pc = 0xe9516; continue; }
            pc = 0xe94d6;
        }
        0xe94d6 => {
            // block e94d6
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94d6 movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // e94d9 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // e94dc cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // e94de jne 0xe94fb
            if fl.ne() { pc = 0xe94fb; continue; }
            pc = 0xe94e0;
        }
        0xe94e0 => {
            // block e94e0
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94e0 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e94e3 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // e94e6 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // e94e9 jne 0xe94d6
            if fl.ne() { pc = 0xe94d6; continue; }
            pc = 0xe94eb;
        }
        0xe94eb => {
            // block e94eb
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94eb jmp 0xe9516
            pc = 0xe9516; continue;
        }
        0xe94ed => {
            // block e94ed
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94ed test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e94f0 je 0xe9516
            if fl.e() { pc = 0xe9516; continue; }
            pc = 0xe94f2;
        }
        0xe94f2 => {
            // block e94f2
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94f2 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e94f7 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e94f9 je 0xe9516
            if fl.e() { pc = 0xe9516; continue; }
            pc = 0xe94fb;
        }
        0xe94fb => {
            // block e94fb
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94fb add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // e94ff mov rdi,r15
            rdi = r15;
            // e9502 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9507 cmp rbx,QWORD PTR [r14+0x270]
            fl = sub::<64>(rbx, m.r64(r14.wrapping_add(0x270)) as u64).1;
            // e950e jne 0xe9475
            if fl.ne() { pc = 0xe9475; continue; }
            pc = 0xe9514;
        }
        0xe9514 => {
            // block e9514
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9514 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9514 jmp 0xe952d
            pc = 0xe952d; continue;
        }
        0xe9516 => {
            // block e9516
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9516 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9516 mov QWORD PTR [r14+0x280],rbx
            m.w64(r14.wrapping_add(0x280), (rbx) as u64);
            // e951d lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e9521 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9526 mov rbx,QWORD PTR [r14+0x270]
            rbx = m.r64(r14.wrapping_add(0x270)) as u64;
            pc = 0xe952d;
        }
        0xe952d => {
            // block e952d
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e952d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e952d cmp QWORD PTR [r14+0x280],rbx
            fl = sub::<64>(m.r64(r14.wrapping_add(0x280)) as u64, rbx).1;
            // e9534 setne bl
            rbx = (rbx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // e9537 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e953b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9540 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e9544 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9549 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e954b add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // e954f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9550 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e9552 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e9554 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9556 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e9558 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9559 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe955a => {
            // block e955a
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e955a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e955a mov rbx,rax
            rbx = rax;
            // e955d lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e9561 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9566 jmp 0xe956b
            pc = 0xe956b; continue;
        }
        0xe9568 => {
            // block e9568
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9568 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9568 mov rbx,rax
            rbx = rax;
            pc = 0xe956b;
        }
        0xe956b => {
            // block e956b
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e956b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e956b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e956f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9574 mov rdi,rbx
            rdi = rbx;
            // e9577 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9b24 size=604
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
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
            { let r = { let t = m.r64(rax.wrapping_add(0x68)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rcx.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x98b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x40)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe9d55;
        }
        0xe9d55 => {
            // block e9d55
            #[cfg(feature = "trace")] eprintln!("f_000e9b24 pc e9d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9d55 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // e9d5a call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9d5f mov rbx,rax
            rbx = rax;
            // e9d62 mov rdi,rbx
            rdi = rbx;
            // e9d65 call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9d6a mov rsi,QWORD PTR [rip+0x5646f] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // e9d71 mov rdx,QWORD PTR [rip+0x56430] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // e9d78 mov rdi,rbx
            rdi = rbx;
            // e9d7b call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3b22 size=690
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3b22<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd3b22;
    loop {
        match pc {
        0xd3b22 => {
            // block d3b22
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3b22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3b22 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3b23 mov rbp,rsp
            rbp = rsp;
            // d3b26 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3b28 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d3b2a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d3b2c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d3b2e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d3b2f sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // d3b36 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d3b39 mov rbx,rdi
            rbx = rdi;
            // d3b3c lea rax,[rbp-0xa0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d3b43 mov QWORD PTR [rbp-0x98],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x0u64) as u64);
            // d3b4e mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // d3b59 mov QWORD PTR [rbp-0xa8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rax) as u64);
            // d3b60 mov DWORD PTR [rbp-0xac],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff54), (0x0u64) as u32);
            // d3b6a mov DWORD PTR [rbp-0xb0],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u32);
            // d3b74 mov DWORD PTR [rcx],0x0
            m.w32(rcx, (0x0u64) as u32);
            // d3b7a mov DWORD PTR [rdx],0x0
            m.w32(rdx, (0x0u64) as u32);
            // d3b80 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // d3b83 je 0xd3d7f
            if fl.e() { pc = 0xd3d7f; continue; }
            pc = 0xd3b89;
        }
        0xd3b89 => {
            // block d3b89
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3b89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3b89 mov al,BYTE PTR [rbx+0xa]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0xa)) as u64) as u8 as u64);
            // d3b8c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d3b8e je 0xd3d7f
            if fl.e() { pc = 0xd3d7f; continue; }
            pc = 0xd3b94;
        }
        0xd3b94 => {
            // block d3b94
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3b94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3b94 mov QWORD PTR [rbp-0xc8],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (rdx) as u64);
            // d3b9b mov QWORD PTR [rbp-0xc0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rcx) as u64);
            // d3ba2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3ba5 mov rax,QWORD PTR [rax+0x470]
            rax = m.r64(rax.wrapping_add(0x470)) as u64;
            // d3bac lea rsi,[rbp-0xac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff54);
            // d3bb3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d3bb5 mov rdi,rbx
            rdi = rbx;
            // d3bb8 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3bba mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3bbd mov rax,QWORD PTR [rax+0x470]
            rax = m.r64(rax.wrapping_add(0x470)) as u64;
            // d3bc4 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // d3bcb mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // d3bd0 mov rdi,rbx
            rdi = rbx;
            // d3bd3 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3bd5 mov DWORD PTR [rbp-0xb8],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff48), ((r14 as u32 as u64)) as u32);
            // d3bdc xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d3bdf lea r15,[rbp-0xa8]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            pc = 0xd3be6;
        }
        0xd3be6 => {
            // block d3be6
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3be6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3be6 lea rax,[rip+0x2f313] # 0x102f00
            rax = m.env.data_va(95949);
            // d3bed mov esi,DWORD PTR [rax+r14*4]
            rsi = (m.r32(rax.wrapping_add(r14.wrapping_mul(4))) as u64) as u32 as u64;
            // d3bf1 xorps xmm0,xmm0
            x0 = 0;
            // d3bf4 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // d3bf8 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // d3bfc movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // d3c00 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // d3c04 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // d3c0b mov DWORD PTR [rbp-0x38],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u32);
            // d3c12 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // d3c1a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3c1d mov rax,QWORD PTR [rax+0xc18]
            rax = m.r64(rax.wrapping_add(0xc18)) as u64;
            // d3c24 mov rdi,rbx
            rdi = rbx;
            // d3c27 lea rdx,[rbp-0x90]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d3c2e call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3c30 cmp DWORD PTR [rbp-0x8c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff74)) as u64, 0x0u64).1;
            // d3c37 je 0xd3cb2
            if fl.e() { pc = 0xd3cb2; continue; }
            pc = 0xd3c39;
        }
        0xd3c39 => {
            // block d3c39
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c39 xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d3c3b jne 0xd3cb2
            if fl.ne() { pc = 0xd3cb2; continue; }
            pc = 0xd3c3d;
        }
        0xd3c3d => {
            // block d3c3d
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c3d mov r13d,DWORD PTR [rbp-0x84]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c)) as u64) as u32 as u64;
            // d3c44 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // d3c47 and eax,0xfffffff7
            { let r = (rax as u32 as u64) & 0xfffffff7u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d3c4a jne 0xd3c5f
            if fl.ne() { pc = 0xd3c5f; continue; }
            pc = 0xd3c4c;
        }
        0xd3c4c => {
            // block d3c4c
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c4c mov ecx,DWORD PTR [rbp-0xac]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff54)) as u64) as u32 as u64;
            // d3c52 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3c54 je 0xd3c5f
            if fl.e() { pc = 0xd3c5f; continue; }
            pc = 0xd3c56;
        }
        0xd3c56 => {
            // block d3c56
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c56 mov r12d,DWORD PTR [rbp-0x88]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // d3c5d jmp 0xd3c75
            pc = 0xd3c75; continue;
        }
        0xd3c5f => {
            // block d3c5f
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c5f mov r12d,DWORD PTR [rbp-0x88]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // d3c66 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // d3c69 jne 0xd3c7d
            if fl.ne() { pc = 0xd3c7d; continue; }
            pc = 0xd3c6b;
        }
        0xd3c6b => {
            // block d3c6b
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c6b mov eax,DWORD PTR [rbp-0xb0]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64) as u32 as u64;
            // d3c71 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d3c73 je 0xd3c7d
            if fl.e() { pc = 0xd3c7d; continue; }
            pc = 0xd3c75;
        }
        0xd3c75 => {
            // block d3c75
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c75 sub r13d,r12d
            { let (r, f) = sub::<32>((r13 as u32 as u64), (r12 as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // d3c78 lea r13d,[r12+r13*4+0x3]
            r13 = (r12.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x3)) as u32 as u64;
            pc = 0xd3c7d;
        }
        0xd3c7d => {
            // block d3c7d
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c7d mov DWORD PTR [rbp-0xb4],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff4c), ((r12 as u32 as u64)) as u32);
            // d3c84 cmp r12d,r13d
            fl = sub::<32>((r12 as u32 as u64), (r13 as u32 as u64)).1;
            // d3c87 jg 0xd3cb2
            if fl.g() { pc = 0xd3cb2; continue; }
            pc = 0xd3c89;
        }
        0xd3c89 => {
            // block d3c89
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c89 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            pc = 0xd3c8c;
        }
        0xd3c8c => {
            // block d3c8c
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3c8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3c8c mov rdi,r15
            rdi = r15;
            // d3c8f lea rsi,[rbp-0xb4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff4c);
            // d3c96 call 0xebea0
            { let r = { let t = m.env.stub_va(0x66d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3c9b mov DWORD PTR [rbp-0xb4],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff4c), ((r12 as u32 as u64)) as u32);
            // d3ca2 lea eax,[r12+0x1]
            rax = (r12.wrapping_add(0x1)) as u32 as u64;
            // d3ca7 dec r12d
            { let (r, f) = dec::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // d3caa cmp r12d,r13d
            fl = sub::<32>((r12 as u32 as u64), (r13 as u32 as u64)).1;
            // d3cad mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // d3cb0 jl 0xd3c8c
            if fl.l() { pc = 0xd3c8c; continue; }
            pc = 0xd3cb2;
        }
        0xd3cb2 => {
            // block d3cb2
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3cb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cb2 inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // d3cb5 cmp r14,0x10
            fl = sub::<64>(r14, 0x10u64).1;
            // d3cb9 jb 0xd3be6
            if fl.b() { pc = 0xd3be6; continue; }
            pc = 0xd3cbf;
        }
        0xd3cbf => {
            // block d3cbf
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3cbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cbf mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d3cc2 call 0xd19e0
            { let r = { let t = m.env.code_va(0xd19e0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3cc7 mov ecx,DWORD PTR [rbp-0xb8]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // d3ccd dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d3ccf movzx r8d,cl
            r8 = ((rcx as u8 as u64)) as u32 as u64;
            // d3cd3 mov rdi,QWORD PTR [rbp-0xa8]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // d3cda lea r9d,[r8-0x1]
            r9 = (r8.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // d3cde xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // d3ce1 lea rdx,[rbp-0xa0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d3ce8 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // d3ceb je 0xd3d54
            if fl.e() { pc = 0xd3d54; continue; }
            pc = 0xd3ced;
        }
        0xd3ced => {
            // block d3ced
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3ced rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3ced xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // d3cf0 mov r11d,r8d
            r11 = ((r8 as u32 as u64)) as u32 as u64;
            pc = 0xd3cf3;
        }
        0xd3cf3 => {
            // block d3cf3
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3cf3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cf3 mov r14d,DWORD PTR [rdi+0x1c]
            r14 = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // d3cf7 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            pc = 0xd3cfa;
        }
        0xd3cfa => {
            // block d3cfa
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3cfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cfa mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // d3cfc mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // d3d00 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // d3d03 je 0xd3d12
            if fl.e() { pc = 0xd3d12; continue; }
            pc = 0xd3d05;
        }
        0xd3d05 => {
            // block d3d05
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d05 mov rdi,rcx
            rdi = rcx;
            // d3d08 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // d3d0b test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // d3d0e jne 0xd3d05
            if fl.ne() { pc = 0xd3d05; continue; }
            pc = 0xd3d10;
        }
        0xd3d10 => {
            // block d3d10
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d10 jmp 0xd3d21
            pc = 0xd3d21; continue;
        }
        0xd3d12 => {
            // block d3d12
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d12 mov rcx,rdi
            rcx = rdi;
            pc = 0xd3d15;
        }
        0xd3d15 => {
            // block d3d15
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d15 mov rdi,QWORD PTR [rcx+0x10]
            rdi = m.r64(rcx.wrapping_add(0x10)) as u64;
            // d3d19 cmp QWORD PTR [rdi],rcx
            fl = sub::<64>(m.r64(rdi) as u64, rcx).1;
            // d3d1c mov rcx,rdi
            rcx = rdi;
            // d3d1f jne 0xd3d15
            if fl.ne() { pc = 0xd3d15; continue; }
            pc = 0xd3d21;
        }
        0xd3d21 => {
            // block d3d21
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d21 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // d3d24 mov rcx,rdx
            rcx = rdx;
            // d3d27 je 0xd3d36
            if fl.e() { pc = 0xd3d36; continue; }
            pc = 0xd3d29;
        }
        0xd3d29 => {
            // block d3d29
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d29 mov ecx,DWORD PTR [rdi+0x1c]
            rcx = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // d3d2c lea ebx,[rsi+0x1]
            rbx = (rsi.wrapping_add(0x1)) as u32 as u64;
            // d3d2f cmp ecx,ebx
            fl = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64)).1;
            // d3d31 je 0xd3cfa
            if fl.e() { pc = 0xd3cfa; continue; }
            pc = 0xd3d33;
        }
        0xd3d33 => {
            // block d3d33
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d33 mov rcx,rdi
            rcx = rdi;
            pc = 0xd3d36;
        }
        0xd3d36 => {
            // block d3d36
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d36 cmp r10d,r14d
            fl = sub::<32>((r10 as u32 as u64), (r14 as u32 as u64)).1;
            // d3d39 jge 0xd3d40
            if fl.ge() { pc = 0xd3d40; continue; }
            pc = 0xd3d3b;
        }
        0xd3d3b => {
            // block d3d3b
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d3b cmp r11d,r14d
            fl = sub::<32>((r11 as u32 as u64), (r14 as u32 as u64)).1;
            // d3d3e jle 0xd3d54
            if fl.le() { pc = 0xd3d54; continue; }
            pc = 0xd3d40;
        }
        0xd3d40 => {
            // block d3d40
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d40 lea r10d,[rsi+0x1]
            r10 = (rsi.wrapping_add(0x1)) as u32 as u64;
            // d3d44 lea r11d,[rsi+r8*1+0x1]
            r11 = (rsi.wrapping_add(r8).wrapping_add(0x1)) as u32 as u64;
            // d3d49 add esi,r8d
            { let (r, f) = add::<32>((rsi as u32 as u64), (r8 as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // d3d4c cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // d3d4f mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // d3d52 jne 0xd3cf3
            if fl.ne() { pc = 0xd3cf3; continue; }
            pc = 0xd3d54;
        }
        0xd3d54 => {
            // block d3d54
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d54 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d3d5b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3d5d cmp r10d,eax
            fl = sub::<32>((r10 as u32 as u64), (rax as u32 as u64)).1;
            // d3d60 jg 0xd3d88
            if fl.g() { pc = 0xd3d88; continue; }
            pc = 0xd3d62;
        }
        0xd3d62 => {
            // block d3d62
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d62 cmp r9d,eax
            fl = sub::<32>((r9 as u32 as u64), (rax as u32 as u64)).1;
            // d3d65 jg 0xd3d88
            if fl.g() { pc = 0xd3d88; continue; }
            pc = 0xd3d67;
        }
        0xd3d67 => {
            // block d3d67
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d67 mov rax,QWORD PTR [rbp-0xc8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff38)) as u64;
            // d3d6e mov DWORD PTR [rax],r10d
            m.w32(rax, ((r10 as u32 as u64)) as u32);
            // d3d71 mov rax,QWORD PTR [rbp-0xc0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // d3d78 mov DWORD PTR [rax],r9d
            m.w32(rax, ((r9 as u32 as u64)) as u32);
            // d3d7b mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // d3d7d jmp 0xd3d88
            pc = 0xd3d88; continue;
        }
        0xd3d7f => {
            // block d3d7f
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d7f lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d3d86 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3d88;
        }
        0xd3d88 => {
            // block d3d88
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3d88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3d88 mov rsi,QWORD PTR [rbp-0xa0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // d3d8f call 0xebeac
            { let r = { let t = m.env.stub_va(0x679); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3d94 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d3d96 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // d3d9d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3d9e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d3da0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d3da2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3da4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3da6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3da7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd3da8 => {
            // block d3da8
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3da8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3da8 mov r14,rax
            r14 = rax;
            pc = 0xd3dab;
        }
        0xd3dab => {
            // block d3dab
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3dab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3dab mov rsi,QWORD PTR [rbp-0xa0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // d3db2 mov rdi,r15
            rdi = r15;
            // d3db5 call 0xebeac
            { let r = { let t = m.env.stub_va(0x679); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3dba mov rdi,r14
            rdi = r14;
            // d3dbd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3dc2 mov r14,rax
            r14 = rax;
            // d3dc5 jmp 0xd3dab
            pc = 0xd3dab; continue;
        }
        0xd3dc7 => {
            // block d3dc7
            #[cfg(feature = "trace")] eprintln!("f_000d3b22 pc d3dc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3dc7 mov r14,rax
            r14 = rax;
            // d3dca lea r15,[rbp-0xa8]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d3dd1 jmp 0xd3dab
            pc = 0xd3dab; continue;
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

/// parity: P2v @0xe957c size=140
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e957c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe957c;
    loop {
        match pc {
        0xe957c => {
            // block e957c
            #[cfg(feature = "trace")] eprintln!("f_000e957c pc e957c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e957c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e957d mov rbp,rsp
            rbp = rsp;
            // e9580 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9581 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9588 mov rbx,rsi
            rbx = rsi;
            // e958b mov BYTE PTR [rbp-0xa0],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u8);
            // e9592 mov WORD PTR [rbp-0x9e],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff62), (0x0u64) as u16);
            // e959b mov DWORD PTR [rbp-0x9c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff64), (0x0u64) as u32);
            // e95a5 mov DWORD PTR [rbp-0x98],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x0u64) as u32);
            // e95af mov BYTE PTR [rbp-0x10],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u8);
            // e95b3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e95b6 lea rsi,[rbp-0xa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // e95bd call QWORD PTR [rax+0x98]
            { let r = { let t = m.r64(rax.wrapping_add(0x98)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e95c3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e95c5 je 0xe95fe
            if fl.e() { pc = 0xe95fe; continue; }
            pc = 0xe95c7;
        }
        0xe95c7 => {
            // block e95c7
            #[cfg(feature = "trace")] eprintln!("f_000e957c pc e95c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e95c7 mov ecx,DWORD PTR [rbp-0xa0]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64) as u32 as u64;
            // e95cd mov BYTE PTR [rbx],cl
            m.w8(rbx, ((rcx as u8 as u64)) as u8);
            // e95cf mov edx,DWORD PTR [rbp-0x98]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64) as u32 as u64;
            // e95d5 mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // e95d8 shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e95db mov WORD PTR [rbx+0x2],cx
            m.w16(rbx.wrapping_add(0x2), ((rcx as u16 as u64)) as u16);
            // e95df mov rcx,QWORD PTR [rbp-0x90]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // e95e6 mov rdx,QWORD PTR [rbp-0x88]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // e95ed mov QWORD PTR [rbx+0x18],rdx
            m.w64(rbx.wrapping_add(0x18), (rdx) as u64);
            // e95f1 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // e95f5 mov cl,BYTE PTR [rbp-0x10]
            rcx = (rcx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64) as u8 as u64);
            // e95f8 mov BYTE PTR [rbx+0x90],cl
            m.w8(rbx.wrapping_add(0x90), ((rcx as u8 as u64)) as u8);
            pc = 0xe95fe;
        }
        0xe95fe => {
            // block e95fe
            #[cfg(feature = "trace")] eprintln!("f_000e957c pc e95fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e95fe add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9605 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9606 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9607 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb799c size=236
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
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
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9740 size=668
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9740<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9740;
    loop {
        match pc {
        0xe9740 => {
            // block e9740
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9740 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9740 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9741 mov rbp,rsp
            rbp = rsp;
            // e9744 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9746 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9748 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e974a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e974c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e974d sub rsp,0xc8
            { let (r, f) = sub::<64>(rsp, 0xc8u64);
              rsp = r;
              fl = f; }
            // e9754 mov ebx,r8d
            rbx = ((r8 as u32 as u64)) as u32 as u64;
            // e9757 mov r15,rcx
            r15 = rcx;
            // e975a mov DWORD PTR [rbp-0xe4],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), ((rdx as u32 as u64)) as u32);
            // e9760 mov r13,rsi
            r13 = rsi;
            // e9763 mov r14,rdi
            r14 = rdi;
            // e9766 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e9769 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e976c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e976f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e9771 je 0xe99a7
            if fl.e() { pc = 0xe99a7; continue; }
            pc = 0xe9777;
        }
        0xe9777 => {
            // block e9777
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9777 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9777 mov BYTE PTR [rbp-0xc0],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u8);
            // e977e mov WORD PTR [rbp-0xbe],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff42), (0x0u64) as u16);
            // e9787 mov DWORD PTR [rbp-0xbc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff44), (0x0u64) as u32);
            // e9791 mov DWORD PTR [rbp-0xb8],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (0x0u64) as u32);
            // e979b mov BYTE PTR [rbp-0x30],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u8);
            // e979f test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e97a1 je 0xe97d3
            if fl.e() { pc = 0xe97d3; continue; }
            pc = 0xe97a3;
        }
        0xe97a3 => {
            // block e97a3
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e97a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e97a3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e97a6 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e97ad mov rdi,r14
            rdi = r14;
            // e97b0 call QWORD PTR [rax+0x98]
            { let r = { let t = m.r64(rax.wrapping_add(0x98)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e97b6 mov ax,WORD PTR [rbp-0xbe]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff42)) as u64) as u16 as u64);
            // e97bd mov DWORD PTR [rbp-0xe4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), ((rax as u32 as u64)) as u32);
            // e97c3 mov rax,QWORD PTR [r14+0x288]
            rax = m.r64(r14.wrapping_add(0x288)) as u64;
            // e97ca mov QWORD PTR [r14+0x280],rax
            m.w64(r14.wrapping_add(0x280), (rax) as u64);
            // e97d1 jmp 0xe97fe
            pc = 0xe97fe; continue;
        }
        0xe97d3 => {
            // block e97d3
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e97d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e97d3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e97d6 mov ecx,DWORD PTR [rbp-0xe4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c)) as u64) as u32 as u64;
            // e97dc movzx esi,cx
            rsi = ((rcx as u16 as u64)) as u32 as u64;
            // e97df mov rdi,r14
            rdi = r14;
            // e97e2 call QWORD PTR [rax+0x40]
            { let r = { let t = m.r64(rax.wrapping_add(0x40)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e97e5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e97e8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e97ea je 0xe99a7
            if fl.e() { pc = 0xe99a7; continue; }
            pc = 0xe97f0;
        }
        0xe97f0 => {
            // block e97f0
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e97f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e97f0 mov rax,QWORD PTR [r14+0x280]
            rax = m.r64(r14.wrapping_add(0x280)) as u64;
            // e97f7 mov QWORD PTR [r14+0x288],rax
            m.w64(r14.wrapping_add(0x288), (rax) as u64);
            pc = 0xe97fe;
        }
        0xe97fe => {
            // block e97fe
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e97fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e97fe add rax,0x18
            { let (r, f) = add::<64>(rax, 0x18u64);
              rax = r;
              fl = f; }
            // e9802 mov QWORD PTR [r14+0x288],rax
            m.w64(r14.wrapping_add(0x288), (rax) as u64);
            // e9809 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e980c lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e9813 mov rdi,r14
            rdi = r14;
            // e9816 call QWORD PTR [rax+0x98]
            { let r = { let t = m.r64(rax.wrapping_add(0x98)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e981c mov eax,DWORD PTR [rbp-0xb8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // e9822 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e9825 jne 0xe9837
            if fl.ne() { pc = 0xe9837; continue; }
            pc = 0xe9827;
        }
        0xe9827 => {
            // block e9827
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9827 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9827 mov ax,WORD PTR [rbp-0xbe]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff42)) as u64) as u16 as u64);
            // e982e mov WORD PTR [r15],ax
            m.w16(r15, ((rax as u16 as u64)) as u16);
            // e9832 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            pc = 0xe9837;
        }
        0xe9837 => {
            // block e9837
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9837 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9837 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e983a jne 0xe9989
            if fl.ne() { pc = 0xe9989; continue; }
            pc = 0xe9840;
        }
        0xe9840 => {
            // block e9840
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9840 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9840 lea rbx,[rbp-0xd8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e9847 lea r12,[rbp-0xdc]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff24);
            pc = 0xe984e;
        }
        0xe984e => {
            // block e984e
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e984e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e984e mov r15,QWORD PTR [r14+0x288]
            r15 = m.r64(r14.wrapping_add(0x288)) as u64;
            // e9855 test BYTE PTR [r15],0x1
            fl = logic::<8>(m.r8(r15) as u64 & 0x1u64);
            // e9859 jne 0xe9860
            if fl.ne() { pc = 0xe9860; continue; }
            pc = 0xe985b;
        }
        0xe985b => {
            // block e985b
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e985b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e985b inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // e985e jmp 0xe9864
            pc = 0xe9864; continue;
        }
        0xe9860 => {
            // block e9860
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9860 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9860 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0xe9864;
        }
        0xe9864 => {
            // block e9864
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9864 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9864 mov rdi,r15
            rdi = r15;
            // e9867 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e986c mov rdi,rbx
            rdi = rbx;
            // e986f mov rsi,r15
            rsi = r15;
            // e9872 mov rdx,rax
            rdx = rax;
            // e9875 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e987a cmp BYTE PTR [rbp-0xc0],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64, 0x0u64).1;
            // e9881 lea r15,[rip+0x53a9a] # 0x13d322
            r15 = m.env.data_va(334575);
            // e9888 je 0xe9905
            if fl.e() { pc = 0xe9905; continue; }
            pc = 0xe988a;
        }
        0xe988a => {
            // block e988a
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e988a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e988a mov ebx,0x9
            rbx = (0x9u64) as u32 as u64;
            pc = 0xe988f;
        }
        0xe988f => {
            // block e988f
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e988f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e988f mov DWORD PTR [rbp-0xdc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff24), (0x0u64) as u32);
            // e9899 mov rdi,QWORD PTR [r14+0x288]
            rdi = m.r64(r14.wrapping_add(0x288)) as u64;
            // e98a0 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e98a3 jne 0xe98aa
            if fl.ne() { pc = 0xe98aa; continue; }
            pc = 0xe98a5;
        }
        0xe98a5 => {
            // block e98a5
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98a5 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e98a8 jmp 0xe98ae
            pc = 0xe98ae; continue;
        }
        0xe98aa => {
            // block e98aa
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98aa mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe98ae;
        }
        0xe98ae => {
            // block e98ae
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98ae add rdi,rbx
            { let (r, f) = add::<64>(rdi, rbx);
              rdi = r;
              fl = f; }
            // e98b1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e98b3 mov rsi,r15
            rsi = r15;
            // e98b6 mov rdx,r12
            rdx = r12;
            // e98b9 call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e98be mov al,BYTE PTR [rbp-0xdc]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff24)) as u64) as u8 as u64);
            // e98c4 mov BYTE PTR [rbp-0xdd],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff23), ((rax as u8 as u64)) as u8);
            // e98ca mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // e98ce cmp rcx,QWORD PTR [r13+0x10]
            fl = sub::<64>(rcx, m.r64(r13.wrapping_add(0x10)) as u64).1;
            // e98d2 jae 0xe98dc
            if fl.ae() { pc = 0xe98dc; continue; }
            pc = 0xe98d4;
        }
        0xe98d4 => {
            // block e98d4
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98d4 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // e98d6 inc QWORD PTR [r13+0x8]
            { let (r, f) = inc::<64>(m.r64(r13.wrapping_add(0x8)) as u64, fl); m.w64(r13.wrapping_add(0x8), (r) as u64); fl = f; }
            // e98da jmp 0xe98eb
            pc = 0xe98eb; continue;
        }
        0xe98dc => {
            // block e98dc
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98dc mov rdi,r13
            rdi = r13;
            // e98df lea rsi,[rbp-0xdd]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff23);
            // e98e6 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe98eb;
        }
        0xe98eb => {
            // block e98eb
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e98eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e98eb movzx eax,BYTE PTR [rbp-0xc0]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64) as u32 as u64;
            // e98f2 add rax,rax
            { let (r, f) = add::<64>(rax, rax);
              rax = r;
              fl = f; }
            // e98f5 lea rcx,[rbx+0x2]
            rcx = rbx.wrapping_add(0x2);
            // e98f9 add rbx,0xfffffffffffffff9
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_fff9u64);
              rbx = r;
              fl = f; }
            // e98fd cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // e9900 mov rbx,rcx
            rbx = rcx;
            // e9903 jl 0xe988f
            if fl.l() { pc = 0xe988f; continue; }
            pc = 0xe9905;
        }
        0xe9905 => {
            // block e9905
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9905 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9905 add QWORD PTR [r14+0x288],0x18
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x288)) as u64, 0x18u64);
              m.w64(r14.wrapping_add(0x288), (r) as u64);
              fl = f; }
            // e990d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e9910 mov rax,QWORD PTR [rax+0x98]
            rax = m.r64(rax.wrapping_add(0x98)) as u64;
            // e9917 mov rdi,r14
            rdi = r14;
            // e991a lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e9921 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9923 lea rbx,[rbp-0xd8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e992a mov eax,DWORD PTR [rbp-0xb8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // e9930 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e9933 je 0xe9974
            if fl.e() { pc = 0xe9974; continue; }
            pc = 0xe9935;
        }
        0xe9935 => {
            // block e9935
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9935 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // e9938 jne 0xe9993
            if fl.ne() { pc = 0xe9993; continue; }
            pc = 0xe993a;
        }
        0xe993a => {
            // block e993a
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e993a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e993a mov eax,DWORD PTR [rbp-0xe4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c)) as u64) as u32 as u64;
            // e9940 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e9942 mov DWORD PTR [rbp-0xe4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), ((rax as u32 as u64)) as u32);
            // e9948 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // e994b movzx ecx,WORD PTR [rbp-0xbe]
            rcx = (m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff42)) as u64) as u32 as u64;
            // e9952 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // e9954 jne 0xe9993
            if fl.ne() { pc = 0xe9993; continue; }
            pc = 0xe9956;
        }
        0xe9956 => {
            // block e9956
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9956 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9956 add QWORD PTR [r14+0x288],0x18
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x288)) as u64, 0x18u64);
              m.w64(r14.wrapping_add(0x288), (r) as u64);
              fl = f; }
            // e995e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e9961 mov rax,QWORD PTR [rax+0x98]
            rax = m.r64(rax.wrapping_add(0x98)) as u64;
            // e9968 mov rdi,r14
            rdi = r14;
            // e996b lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e9972 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe9974;
        }
        0xe9974 => {
            // block e9974
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9974 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9974 mov rdi,rbx
            rdi = rbx;
            // e9977 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e997c cmp DWORD PTR [rbp-0xb8],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64, 0x1u64).1;
            // e9983 je 0xe984e
            if fl.e() { pc = 0xe984e; continue; }
            pc = 0xe9989;
        }
        0xe9989 => {
            // block e9989
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9989 mov r12d,DWORD PTR [r13+0x8]
            r12 = (m.r32(r13.wrapping_add(0x8)) as u64) as u32 as u64;
            // e998d sub r12d,DWORD PTR [r13+0x0]
            { let (r, f) = sub::<32>((r12 as u32 as u64), m.r32(r13) as u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e9991 jmp 0xe99a7
            pc = 0xe99a7; continue;
        }
        0xe9993 => {
            // block e9993
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e9993 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9993 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            // e9997 sub r12,QWORD PTR [r13+0x0]
            { let (r, f) = sub::<64>(r12, m.r64(r13) as u64);
              r12 = r;
              fl = f; }
            // e999b lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e99a2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe99a7;
        }
        0xe99a7 => {
            // block e99a7
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e99a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e99a7 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // e99aa add rsp,0xc8
            { let (r, f) = add::<64>(rsp, 0xc8u64);
              rsp = r;
              fl = f; }
            // e99b1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e99b2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e99b4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e99b6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e99b8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e99ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e99bb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe99bc => {
            // block e99bc
            #[cfg(feature = "trace")] eprintln!("f_000e9740 pc e99bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e99bc mov rbx,rax
            rbx = rax;
            // e99bf lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e99c6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e99cb mov rdi,rbx
            rdi = rbx;
            // e99ce call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e99d3 jmp 0xe99bc
            pc = 0xe99bc; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb922e size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b922e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb922e;
    loop {
        match pc {
        0xb922e => {
            // block b922e
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b922e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b922e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b922f mov rbp,rsp
            rbp = rsp;
            // b9232 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b9234 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b9236 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9237 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b9238 mov r14,rdx
            r14 = rdx;
            // b923b mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // b923e mov rbx,rdi
            rbx = rdi;
            // b9241 mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // b9245 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9248 call 0xcbc54
            { let r = { let t = m.env.code_va(0xcbc54); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b924d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b924f je 0xb92b0
            if fl.e() { pc = 0xb92b0; continue; }
            pc = 0xb9251;
        }
        0xb9251 => {
            // block b9251
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b9251 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9251 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9254 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9259 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b925b je 0xb92b0
            if fl.e() { pc = 0xb92b0; continue; }
            pc = 0xb925d;
        }
        0xb925d => {
            // block b925d
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b925d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b925d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9260 call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9265 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // b9269 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b926c cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b926e jbe 0xb92b0
            if fl.be() { pc = 0xb92b0; continue; }
            pc = 0xb9270;
        }
        0xb9270 => {
            // block b9270
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b9270 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9270 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // b9277 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b927a lea rdx,[rip+0x496cf] # 0x102950
            rdx = m.env.data_va(94493);
            // b9281 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b9284 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // b9288 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // b928b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b9290 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9293 mov rdi,rbx
            rdi = rbx;
            // b9296 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9299 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b929b je 0xb92b3
            if fl.e() { pc = 0xb92b3; continue; }
            pc = 0xb929d;
        }
        0xb929d => {
            // block b929d
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b929d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b929d mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // b92a0 shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // b92a3 not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // b92a5 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b92a8 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // b92ab mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // b92ae jmp 0xb92b3
            pc = 0xb92b3; continue;
        }
        0xb92b0 => {
            // block b92b0
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b92b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b92b0 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xb92b3;
        }
        0xb92b3 => {
            // block b92b3
            #[cfg(feature = "trace")] eprintln!("f_000b922e pc b92b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b92b3 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b92b6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b92ba pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b92bb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b92bd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b92bf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b92c0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb92c2 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b92c2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb92c2;
    loop {
        match pc {
        0xb92c2 => {
            // block b92c2
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b92c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b92c2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b92c3 mov rbp,rsp
            rbp = rsp;
            // b92c6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b92c8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b92ca push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b92cb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b92cc mov r14,rdx
            r14 = rdx;
            // b92cf mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // b92d2 mov rbx,rdi
            rbx = rdi;
            // b92d5 mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // b92d9 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b92dc call 0xcc8c5
            { let r = { let t = m.env.code_va(0xcc8c5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b92e1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b92e3 je 0xb9344
            if fl.e() { pc = 0xb9344; continue; }
            pc = 0xb92e5;
        }
        0xb92e5 => {
            // block b92e5
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b92e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b92e5 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b92e8 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b92ed test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b92ef je 0xb9344
            if fl.e() { pc = 0xb9344; continue; }
            pc = 0xb92f1;
        }
        0xb92f1 => {
            // block b92f1
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b92f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b92f1 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b92f4 call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b92f9 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // b92fd movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b9300 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b9302 jbe 0xb9344
            if fl.be() { pc = 0xb9344; continue; }
            pc = 0xb9304;
        }
        0xb9304 => {
            // block b9304
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b9304 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9304 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // b930b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b930e lea rdx,[rip+0x4966b] # 0x102980
            rdx = m.env.data_va(94541);
            // b9315 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b9318 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // b931c xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // b931f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b9324 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9327 mov rdi,rbx
            rdi = rbx;
            // b932a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b932d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b932f je 0xb9347
            if fl.e() { pc = 0xb9347; continue; }
            pc = 0xb9331;
        }
        0xb9331 => {
            // block b9331
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b9331 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9331 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // b9334 shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // b9337 not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // b9339 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b933c mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // b933f mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // b9342 jmp 0xb9347
            pc = 0xb9347; continue;
        }
        0xb9344 => {
            // block b9344
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b9344 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9344 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9347;
        }
        0xb9347 => {
            // block b9347
            #[cfg(feature = "trace")] eprintln!("f_000b92c2 pc b9347 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9347 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b934a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b934e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b934f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9351 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9353 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9354 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe48b0 size=158
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e48b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe48b0;
    loop {
        match pc {
        0xe48b0 => {
            // block e48b0
            #[cfg(feature = "trace")] eprintln!("f_000e48b0 pc e48b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e48b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e48b1 mov rbp,rsp
            rbp = rsp;
            // e48b4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e48b6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e48b7 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // e48bb mov rbx,rsi
            rbx = rsi;
            // e48be mov r14,rdi
            r14 = rdi;
            // e48c1 mov esi,DWORD PTR [r14+0x8]
            rsi = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // e48c5 cmp esi,0x1a
            fl = sub::<32>((rsi as u32 as u64), 0x1au64).1;
            // e48c8 jne 0xe48d8
            if fl.ne() { pc = 0xe48d8; continue; }
            pc = 0xe48ca;
        }
        0xe48ca => {
            // block e48ca
            #[cfg(feature = "trace")] eprintln!("f_000e48b0 pc e48ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e48ca lea rsi,[r14+0x10]
            rsi = r14.wrapping_add(0x10);
            // e48ce mov rdi,rbx
            rdi = rbx;
            // e48d1 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e48d6 jmp 0xe4935
            pc = 0xe4935; continue;
        }
        0xe48d8 => {
            // block e48d8
            #[cfg(feature = "trace")] eprintln!("f_000e48b0 pc e48d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e48d8 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e48dc call 0x62fd6
            { let r = { let t = m.env.code_va(0x62fd6); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e48e1 test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // e48e4 jne 0xe48ed
            if fl.ne() { pc = 0xe48ed; continue; }
            pc = 0xe48e6;
        }
        0xe48e6 => {
            // block e48e6
            #[cfg(feature = "trace")] eprintln!("f_000e48b0 pc e48e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e48e6 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // e48eb jmp 0xe48fc
            pc = 0xe48fc; continue;
        }
        0xe48ed => {
            // block e48ed
            #[cfg(feature = "trace")] eprintln!("f_000e48b0 pc e48ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e48ed mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e48f1 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // e48f4 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xe48fc;
        }
        0xe48fc => {
            // block e48fc
            #[cfg(feature = "trace")] eprintln!("f_000e48b0 pc e48fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e48fc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e48fe mov rdi,rbx
            rdi = rbx;
            // e4901 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4906 mov rax,QWORD PTR [rbp-0x20]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // e490a mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e490e mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e4912 mov rcx,QWORD PTR [rbp-0x28]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // e4916 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // e491a mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e491d xorps xmm0,xmm0
            x0 = 0;
            // e4920 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // e4924 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // e492c lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e4930 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe4935;
        }
        0xe4935 => {
            // block e4935
            #[cfg(feature = "trace")] eprintln!("f_000e48b0 pc e4935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4935 mov DWORD PTR [r14+0x8],0x0
            m.w32(r14.wrapping_add(0x8), (0x0u64) as u32);
            // e493d add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // e4941 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4942 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4944 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4945 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe4946 => {
            // block e4946
            #[cfg(feature = "trace")] eprintln!("f_000e48b0 pc e4946 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4946 mov rdi,rax
            rdi = rax;
            // e4949 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba782 size=206
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba782<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba782;
    loop {
        match pc {
        0xba782 => {
            // block ba782
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba782 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba782 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba783 mov rbp,rsp
            rbp = rsp;
            // ba786 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba788 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba78a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ba78c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ba78e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba78f sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // ba793 mov r14,rsi
            r14 = rsi;
            // ba796 mov rbx,rdi
            rbx = rdi;
            // ba799 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // ba79c add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // ba7a0 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // ba7a3 je 0xba82a
            if fl.e() { pc = 0xba82a; continue; }
            pc = 0xba7a9;
        }
        0xba7a9 => {
            // block ba7a9
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba7a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba7a9 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ba7ad jmp 0xba7c6
            pc = 0xba7c6; continue;
        }
        0xba7af => {
            // block ba7af
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba7af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba7af mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // ba7b4 mov rdi,rbx
            rdi = rbx;
            // ba7b7 lea rsi,[rip+0x6fcdf] # 0x12a49d
            rsi = m.env.data_va(257130);
            // ba7be call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba7c3 mov r13,r12
            r13 = r12;
            pc = 0xba7c6;
        }
        0xba7c6 => {
            // block ba7c6
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba7c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba7c6 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // ba7ca mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // ba7cf mov rdi,r15
            rdi = r15;
            // ba7d2 call 0x5e61c
            { let r = { let t = m.env.code_va(0x5e61c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba7d7 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // ba7db test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // ba7de jne 0xba7e9
            if fl.ne() { pc = 0xba7e9; continue; }
            pc = 0xba7e0;
        }
        0xba7e0 => {
            // block ba7e0
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba7e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba7e0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // ba7e3 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // ba7e7 jmp 0xba7f1
            pc = 0xba7f1; continue;
        }
        0xba7e9 => {
            // block ba7e9
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba7e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba7e9 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // ba7ed mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xba7f1;
        }
        0xba7f1 => {
            // block ba7f1
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba7f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba7f1 mov rdi,rbx
            rdi = rbx;
            // ba7f4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba7f9 mov rdi,r15
            rdi = r15;
            // ba7fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba801 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // ba805 test rax,rax
            fl = logic::<64>(rax & rax);
            // ba808 je 0xba818
            if fl.e() { pc = 0xba818; continue; }
            pc = 0xba80a;
        }
        0xba80a => {
            // block ba80a
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba80a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba80a mov r12,rax
            r12 = rax;
            // ba80d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // ba811 test rax,rax
            fl = logic::<64>(rax & rax);
            // ba814 jne 0xba80a
            if fl.ne() { pc = 0xba80a; continue; }
            pc = 0xba816;
        }
        0xba816 => {
            // block ba816
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba816 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba816 jmp 0xba825
            pc = 0xba825; continue;
        }
        0xba818 => {
            // block ba818
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba818 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba818 mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // ba81c cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // ba820 mov r13,r12
            r13 = r12;
            // ba823 jne 0xba818
            if fl.ne() { pc = 0xba818; continue; }
            pc = 0xba825;
        }
        0xba825 => {
            // block ba825
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba825 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba825 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // ba828 jne 0xba7af
            if fl.ne() { pc = 0xba7af; continue; }
            pc = 0xba82a;
        }
        0xba82a => {
            // block ba82a
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba82a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba82a mov rax,rbx
            rax = rbx;
            // ba82d add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // ba831 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba832 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ba834 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ba836 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba838 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba83a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba83b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xba83c => {
            // block ba83c
            #[cfg(feature = "trace")] eprintln!("f_000ba782 pc ba83c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba83c mov rbx,rax
            rbx = rax;
            // ba83f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ba843 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba848 mov rdi,rbx
            rdi = rbx;
            // ba84b call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba850 size=206
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba850<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba850;
    loop {
        match pc {
        0xba850 => {
            // block ba850
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba850 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba850 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba851 mov rbp,rsp
            rbp = rsp;
            // ba854 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba856 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba858 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ba85a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ba85c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba85d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // ba861 mov r14,rsi
            r14 = rsi;
            // ba864 mov rbx,rdi
            rbx = rdi;
            // ba867 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // ba86a add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // ba86e cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // ba871 je 0xba8f8
            if fl.e() { pc = 0xba8f8; continue; }
            pc = 0xba877;
        }
        0xba877 => {
            // block ba877
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba877 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba877 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ba87b jmp 0xba894
            pc = 0xba894; continue;
        }
        0xba87d => {
            // block ba87d
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba87d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba87d mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // ba882 mov rdi,rbx
            rdi = rbx;
            // ba885 lea rsi,[rip+0x6fc11] # 0x12a49d
            rsi = m.env.data_va(257130);
            // ba88c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba891 mov r13,r12
            r13 = r12;
            pc = 0xba894;
        }
        0xba894 => {
            // block ba894
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba894 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba894 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // ba898 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // ba89d mov rdi,r15
            rdi = r15;
            // ba8a0 call 0x5d8db
            { let r = { let t = m.env.code_va(0x5d8db); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba8a5 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // ba8a9 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // ba8ac jne 0xba8b7
            if fl.ne() { pc = 0xba8b7; continue; }
            pc = 0xba8ae;
        }
        0xba8ae => {
            // block ba8ae
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba8ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba8ae shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // ba8b1 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // ba8b5 jmp 0xba8bf
            pc = 0xba8bf; continue;
        }
        0xba8b7 => {
            // block ba8b7
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba8b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba8b7 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // ba8bb mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xba8bf;
        }
        0xba8bf => {
            // block ba8bf
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba8bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba8bf mov rdi,rbx
            rdi = rbx;
            // ba8c2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba8c7 mov rdi,r15
            rdi = r15;
            // ba8ca call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba8cf mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // ba8d3 test rax,rax
            fl = logic::<64>(rax & rax);
            // ba8d6 je 0xba8e6
            if fl.e() { pc = 0xba8e6; continue; }
            pc = 0xba8d8;
        }
        0xba8d8 => {
            // block ba8d8
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba8d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba8d8 mov r12,rax
            r12 = rax;
            // ba8db mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // ba8df test rax,rax
            fl = logic::<64>(rax & rax);
            // ba8e2 jne 0xba8d8
            if fl.ne() { pc = 0xba8d8; continue; }
            pc = 0xba8e4;
        }
        0xba8e4 => {
            // block ba8e4
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba8e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba8e4 jmp 0xba8f3
            pc = 0xba8f3; continue;
        }
        0xba8e6 => {
            // block ba8e6
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba8e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba8e6 mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // ba8ea cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // ba8ee mov r13,r12
            r13 = r12;
            // ba8f1 jne 0xba8e6
            if fl.ne() { pc = 0xba8e6; continue; }
            pc = 0xba8f3;
        }
        0xba8f3 => {
            // block ba8f3
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba8f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba8f3 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // ba8f6 jne 0xba87d
            if fl.ne() { pc = 0xba87d; continue; }
            pc = 0xba8f8;
        }
        0xba8f8 => {
            // block ba8f8
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba8f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba8f8 mov rax,rbx
            rax = rbx;
            // ba8fb add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // ba8ff pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba900 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ba902 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ba904 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba906 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba908 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba909 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xba90a => {
            // block ba90a
            #[cfg(feature = "trace")] eprintln!("f_000ba850 pc ba90a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba90a mov rbx,rax
            rbx = rax;
            // ba90d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ba911 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba916 mov rdi,rbx
            rdi = rbx;
            // ba919 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x6858c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baa6f mov edi,DWORD PTR [rbp-0x2c]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // baa72 mov rsi,rbx
            rsi = rbx;
            // baa75 call 0x2ca4c
            { let r = { let t = m.env.code_va(0x2ca4c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x5d1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x4bd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9380 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // b9388 mov QWORD PTR [r13+0x0],rbx
            m.w64(r13, (rbx) as u64);
            // b938c mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // b9394 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9398 call 0xcbc54
            { let r = { let t = m.env.code_va(0xcbc54); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b939d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b939f je 0xb9438
            if fl.e() { pc = 0xb9438; continue; }
            pc = 0xb93a5;
        }
        0xb93a5 => {
            // block b93a5
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93a5 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b93a9 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b93ae test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b93b0 je 0xb9438
            if fl.e() { pc = 0xb9438; continue; }
            pc = 0xb93b6;
        }
        0xb93b6 => {
            // block b93b6
            #[cfg(feature = "trace")] eprintln!("f_000b9356 pc b93b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b93b6 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b93ba call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.be() { pc = 0xb943a; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0xb9423; continue; }
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
            { let r = { let t = m.env.stub_va(0x4b1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xb940a; continue; }
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
            if fl.ne() { pc = 0xb93e4; continue; }
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
            if fl.e() { pc = 0xb8e3f; continue; }
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
            if fl.e() { pc = 0xb8e53; continue; }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8dd2 mov rbx,rax
            rbx = rax;
            // b8dd5 mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b8dd9 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b8dde lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8de2 call 0x60c44
            { let r = { let t = m.env.code_va(0x60c44); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8de7 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // b8deb test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b8dee jne 0xb8df9
            if fl.ne() { pc = 0xb8df9; continue; }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8e09 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8e0d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8e12 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // b8e16 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b8e19 je 0xb8e28
            if fl.e() { pc = 0xb8e28; continue; }
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
            if fl.ne() { pc = 0xb8e1b; continue; }
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
            if fl.ne() { pc = 0xb8e28; continue; }
            pc = 0xb8e34;
        }
        0xb8e34 => {
            // block b8e34
            #[cfg(feature = "trace")] eprintln!("f_000b8d6b pc b8e34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e34 cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // b8e37 jne 0xb8da5
            if fl.ne() { pc = 0xb8da5; continue; }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0xe3f00 size=312
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e3f00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe3f00;
    loop {
        match pc {
        0xe3f00 => {
            // block e3f00
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e3f01 mov rbp,rsp
            rbp = rsp;
            // e3f04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e3f06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e3f08 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e3f0a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e3f0c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e3f0d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e3f0e mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // e3f11 mov r12,rsi
            r12 = rsi;
            // e3f14 mov r14,rdi
            r14 = rdi;
            // e3f17 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e3f1b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // e3f20 mov rsi,r14
            rsi = r14;
            // e3f23 call 0xec050
            { let r = { let t = m.env.stub_va(0x81d); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3f28 cmp BYTE PTR [rbp-0x30],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x0u64).1;
            // e3f2c je 0xe400e
            if fl.e() { pc = 0xe400e; continue; }
            pc = 0xe3f32;
        }
        0xe3f32 => {
            // block e3f32
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f32 test BYTE PTR [r12],0x1
            fl = logic::<8>(m.r8(r12) as u64 & 0x1u64);
            // e3f37 jne 0xe3f47
            if fl.ne() { pc = 0xe3f47; continue; }
            pc = 0xe3f39;
        }
        0xe3f39 => {
            // block e3f39
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f39 mov WORD PTR [r12],0x0
            m.w16(r12, (0x0u64) as u16);
            // e3f40 lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // e3f45 jmp 0xe3f5d
            pc = 0xe3f5d; continue;
        }
        0xe3f47 => {
            // block e3f47
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f47 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // e3f4c mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // e3f4f lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // e3f54 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xe3f5d;
        }
        0xe3f5d => {
            // block e3f5d
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f5d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e3f5f movzx r15d,r15b
            r15 = ((r15 as u8 as u64)) as u32 as u64;
            pc = 0xe3f63;
        }
        0xe3f63 => {
            // block e3f63
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f63 mov rbx,rax
            rbx = rax;
            // e3f66 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e3f69 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e3f6d mov rdi,QWORD PTR [rax+r14*1+0x28]
            rdi = m.r64(rax.wrapping_add(r14).wrapping_add(0x28)) as u64;
            // e3f72 mov rax,QWORD PTR [rdi+0x18]
            rax = m.r64(rdi.wrapping_add(0x18)) as u64;
            // e3f76 cmp rax,QWORD PTR [rdi+0x20]
            fl = sub::<64>(rax, m.r64(rdi.wrapping_add(0x20)) as u64).1;
            // e3f7a je 0xe3f89
            if fl.e() { pc = 0xe3f89; continue; }
            pc = 0xe3f7c;
        }
        0xe3f7c => {
            // block e3f7c
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f7c lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // e3f80 mov QWORD PTR [rdi+0x18],rcx
            m.w64(rdi.wrapping_add(0x18), (rcx) as u64);
            // e3f84 movzx eax,BYTE PTR [rax]
            rax = (m.r8(rax) as u64) as u32 as u64;
            // e3f87 jmp 0xe3f9c
            pc = 0xe3f9c; continue;
        }
        0xe3f89 => {
            // block e3f89
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f89 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e3f8c mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // e3f90 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3f92 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // e3f97 cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // e3f9a je 0xe3fcb
            if fl.e() { pc = 0xe3fcb; continue; }
            pc = 0xe3f9c;
        }
        0xe3f9c => {
            // block e3f9c
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3f9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3f9c movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // e3f9f xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e3fa1 cmp edx,r15d
            fl = sub::<32>((rdx as u32 as u64), (r15 as u32 as u64)).1;
            // e3fa4 je 0xe3fc8
            if fl.e() { pc = 0xe3fc8; continue; }
            pc = 0xe3fa6;
        }
        0xe3fa6 => {
            // block e3fa6
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fa6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fa6 movsx esi,al
            rsi = (sx::<8>((rax as u8 as u64)) as u64) as u32 as u64;
            // e3fa9 mov rdi,r12
            rdi = r12;
            // e3fac call 0xec020
            { let r = { let t = m.env.stub_va(0x7ed); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3fb1 lea rax,[rbx+0x1]
            rax = rbx.wrapping_add(0x1);
            // e3fb5 test BYTE PTR [r12],0x1
            fl = logic::<8>(m.r8(r12) as u64 & 0x1u64);
            // e3fba je 0xe3f63
            if fl.e() { pc = 0xe3f63; continue; }
            pc = 0xe3fbc;
        }
        0xe3fbc => {
            // block e3fbc
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fbc mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // e3fc1 cmp QWORD PTR [r13+0x0],0xffffffffffffffef
            fl = sub::<64>(m.r64(r13) as u64, 0xffff_ffff_ffff_ffefu64).1;
            // e3fc6 jne 0xe3f63
            if fl.ne() { pc = 0xe3f63; continue; }
            pc = 0xe3fc8;
        }
        0xe3fc8 => {
            // block e3fc8
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fc8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fc8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            pc = 0xe3fcb;
        }
        0xe3fcb => {
            // block e3fcb
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fcb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fcb mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // e3fcd or esi,0x4
            { let r = (rsi as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e3fd0 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // e3fd3 cmovne esi,ecx
            if fl.ne() { rsi = ((rcx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // e3fd6 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e3fd9 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e3fdd lea rdi,[r14+rax*1]
            rdi = r14.wrapping_add(rax);
            // e3fe1 or esi,DWORD PTR [r14+rax*1+0x20]
            { let r = (rsi as u32 as u64) | m.r32(r14.wrapping_add(rax).wrapping_add(0x20)) as u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e3fe6 call 0xec0fe
            { let r = { let t = m.env.stub_va(0x8cb); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3feb jmp 0xe400e
            pc = 0xe400e; continue;
        }
        0xe3fed => {
            // block e3fed
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fed jmp 0xe3fef
            pc = 0xe3fef; continue;
        }
        0xe3fef => {
            // block e3fef
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e3fef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3fef mov rbx,r14
            rbx = r14;
            // e3ff2 mov rdi,rax
            rdi = rax;
            // e3ff5 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3ffa mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e3ffd add rbx,QWORD PTR [rax-0x18]
            { let (r, f) = add::<64>(rbx, m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64);
              rbx = r;
              fl = f; }
            // e4001 mov rdi,rbx
            rdi = rbx;
            // e4004 call 0xec0f2
            { let r = { let t = m.env.stub_va(0x8bf); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4009 call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe400e;
        }
        0xe400e => {
            // block e400e
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e400e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e400e mov rax,r14
            rax = r14;
            // e4011 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e4015 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4016 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e4018 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e401a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e401c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e401e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e401f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe4020 => {
            // block e4020
            #[cfg(feature = "trace")] eprintln!("f_000e3f00 pc e4020 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4020 mov rbx,rax
            rbx = rax;
            // e4023 call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4028 mov rdi,rbx
            rdi = rbx;
            // e402b call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4030 mov rdi,rax
            rdi = rax;
            // e4033 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc362 size=370
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc362<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbc362;
    loop {
        match pc {
        0xbc362 => {
            // block bc362
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc362 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc363 mov rbp,rsp
            rbp = rsp;
            // bc366 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc368 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc36a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bc36c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc36e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc36f sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bc373 mov rbx,rsi
            rbx = rsi;
            // bc376 mov r14,rdi
            r14 = rdi;
            // bc379 mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // bc37c mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc380 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bc383 je 0xbc39a
            if fl.e() { pc = 0xbc39a; continue; }
            pc = 0xbc385;
        }
        0xbc385 => {
            // block bc385
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc385 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc385 lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bc389 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bc38c not rdx
            rdx = !rdx;
            // bc38f and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bc393 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bc396 mov QWORD PTR [rbx+0x8],rdx
            m.w64(rbx.wrapping_add(0x8), (rdx) as u64);
            pc = 0xbc39a;
        }
        0xbc39a => {
            // block bc39a
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc39a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc39a lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bc39e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // bc3a6 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // bc3ae mov QWORD PTR [rbp-0x40],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r15) as u64);
            // bc3b2 mov DWORD PTR [rbp-0x44],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), (0x0u64) as u32);
            // bc3b9 mov DWORD PTR [rbp-0x48],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u32);
            // bc3c0 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // bc3c3 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // bc3c7 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // bc3c9 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // bc3cc je 0xbc4a2
            if fl.e() { pc = 0xbc4a2; continue; }
            pc = 0xbc3d2;
        }
        0xbc3d2 => {
            // block bc3d2
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3d2 lea r12,[rbp-0x48]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            pc = 0xbc3d6;
        }
        0xbc3d6 => {
            // block bc3d6
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3d6 mov edi,DWORD PTR [r13+0x1c]
            rdi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // bc3da lea rsi,[rbp-0x44]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // bc3de mov rdx,r12
            rdx = r12;
            // bc3e1 call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc3e6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // bc3e8 je 0xbc46e
            if fl.e() { pc = 0xbc46e; continue; }
            pc = 0xbc3ee;
        }
        0xbc3ee => {
            // block bc3ee
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3ee mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bc3f2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // bc3f5 mov eax,DWORD PTR [rbp-0x44]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64) as u32 as u64;
            // bc3f8 mov rdx,r15
            rdx = r15;
            // bc3fb je 0xbc42a
            if fl.e() { pc = 0xbc42a; continue; }
            pc = 0xbc3fd;
        }
        0xbc3fd => {
            // block bc3fd
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3fd mov rcx,rsi
            rcx = rsi;
            pc = 0xbc400;
        }
        0xbc400 => {
            // block bc400
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc400 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc400 cmp DWORD PTR [rcx+0x1c],eax
            fl = sub::<32>(m.r32(rcx.wrapping_add(0x1c)) as u64, (rax as u32 as u64)).1;
            // bc403 jae 0xbc410
            if fl.ae() { pc = 0xbc410; continue; }
            pc = 0xbc405;
        }
        0xbc405 => {
            // block bc405
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc405 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc405 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // bc409 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // bc40c jne 0xbc400
            if fl.ne() { pc = 0xbc400; continue; }
            pc = 0xbc40e;
        }
        0xbc40e => {
            // block bc40e
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc40e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc40e jmp 0xbc41d
            pc = 0xbc41d; continue;
        }
        0xbc410 => {
            // block bc410
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc410 mov rsi,QWORD PTR [rcx]
            rsi = m.r64(rcx) as u64;
            // bc413 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // bc416 mov rdx,rcx
            rdx = rcx;
            // bc419 jne 0xbc3fd
            if fl.ne() { pc = 0xbc3fd; continue; }
            pc = 0xbc41b;
        }
        0xbc41b => {
            // block bc41b
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc41b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc41b jmp 0xbc420
            pc = 0xbc420; continue;
        }
        0xbc41d => {
            // block bc41d
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc41d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc41d mov rcx,rdx
            rcx = rdx;
            pc = 0xbc420;
        }
        0xbc420 => {
            // block bc420
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc420 cmp rcx,r15
            fl = sub::<64>(rcx, r15).1;
            // bc423 je 0xbc42a
            if fl.e() { pc = 0xbc42a; continue; }
            pc = 0xbc425;
        }
        0xbc425 => {
            // block bc425
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc425 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc425 cmp eax,DWORD PTR [rcx+0x1c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx.wrapping_add(0x1c)) as u64).1;
            // bc428 jae 0xbc46e
            if fl.ae() { pc = 0xbc46e; continue; }
            pc = 0xbc42a;
        }
        0xbc42a => {
            // block bc42a
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc42a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc42a mov DWORD PTR [rbp-0x58],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((rax as u32 as u64)) as u32);
            // bc42d mov DWORD PTR [rbp-0x54],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), (0x0u64) as u32);
            // bc434 mov DWORD PTR [rbp-0x50],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0xffffffffu64) as u32);
            // bc43b mov DWORD PTR [rbp-0x4c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), (0x0u64) as u32);
            // bc442 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc446 cmp rax,QWORD PTR [rbx+0x10]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // bc44a jae 0xbc462
            if fl.ae() { pc = 0xbc462; continue; }
            pc = 0xbc44c;
        }
        0xbc44c => {
            // block bc44c
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc44c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc44c mov rcx,QWORD PTR [rbp-0x58]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // bc450 mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // bc454 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // bc458 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // bc45b add QWORD PTR [rbx+0x8],0x10
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, 0x10u64);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // bc460 jmp 0xbc46e
            pc = 0xbc46e; continue;
        }
        0xbc462 => {
            // block bc462
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc462 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc462 mov rdi,rbx
            rdi = rbx;
            // bc465 lea rsi,[rbp-0x58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // bc469 call 0xebbbe
            { let r = { let t = m.env.stub_va(0x38b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc46e;
        }
        0xbc46e => {
            // block bc46e
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc46e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc46e mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bc472 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc475 je 0xbc485
            if fl.e() { pc = 0xbc485; continue; }
            pc = 0xbc477;
        }
        0xbc477 => {
            // block bc477
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc477 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc477 mov r13,rax
            r13 = rax;
            // bc47a mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // bc47e test rax,rax
            fl = logic::<64>(rax & rax);
            // bc481 jne 0xbc477
            if fl.ne() { pc = 0xbc477; continue; }
            pc = 0xbc483;
        }
        0xbc483 => {
            // block bc483
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc483 jmp 0xbc495
            pc = 0xbc495; continue;
        }
        0xbc485 => {
            // block bc485
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc485 mov rax,r13
            rax = r13;
            pc = 0xbc488;
        }
        0xbc488 => {
            // block bc488
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc488 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc488 mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc48c cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // bc490 mov rax,r13
            rax = r13;
            // bc493 jne 0xbc488
            if fl.ne() { pc = 0xbc488; continue; }
            pc = 0xbc495;
        }
        0xbc495 => {
            // block bc495
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc495 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc495 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // bc498 jne 0xbc3d6
            if fl.ne() { pc = 0xbc3d6; continue; }
            pc = 0xbc49e;
        }
        0xbc49e => {
            // block bc49e
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc49e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc49e mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0xbc4a2;
        }
        0xbc4a2 => {
            // block bc4a2
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc4a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc4a2 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc4a6 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc4ab mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // bc4ad add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bc4b1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc4b2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc4b4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bc4b6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc4b8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc4ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc4bb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbc4bc => {
            // block bc4bc
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc4bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc4bc mov rbx,rax
            rbx = rax;
            // bc4bf mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bc4c3 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc4c7 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc4cc mov rdi,rbx
            rdi = rbx;
            // bc4cf call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbaafa size=414
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000baafa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbaafa;
    loop {
        match pc {
        0xbaafa => {
            // block baafa
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc baafa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baafa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // baafb mov rbp,rsp
            rbp = rsp;
            // baafe push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bab00 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bab02 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bab04 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bab06 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bab07 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bab0b mov r13,rsi
            r13 = rsi;
            // bab0e mov r14,rdi
            r14 = rdi;
            // bab11 mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // bab15 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bab19 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bab1c je 0xbab33
            if fl.e() { pc = 0xbab33; continue; }
            pc = 0xbab1e;
        }
        0xbab1e => {
            // block bab1e
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab1e lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bab22 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bab25 not rdx
            rdx = !rdx;
            // bab28 and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bab2c add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bab2f mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            pc = 0xbab33;
        }
        0xbab33 => {
            // block bab33
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab33 mov r15,QWORD PTR [r14+0x8]
            r15 = m.r64(r14.wrapping_add(0x8)) as u64;
            // bab37 add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // bab3b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // bab3d cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // bab40 je 0xbac80
            if fl.e() { pc = 0xbac80; continue; }
            pc = 0xbab46;
        }
        0xbab46 => {
            // block bab46
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab46 mov edi,DWORD PTR [r15+0x1c]
            rdi = (m.r32(r15.wrapping_add(0x1c)) as u64) as u32 as u64;
            // bab4a mov r12d,DWORD PTR [r15+0x20]
            r12 = (m.r32(r15.wrapping_add(0x20)) as u64) as u32 as u64;
            // bab4e mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // bab55 mov DWORD PTR [rbp-0x30],0x3e7
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x3e7u64) as u32);
            // bab5c lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // bab60 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // bab64 call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bab69 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // bab6c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // bab6e sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // bab71 mov edx,DWORD PTR [rbp-0x30]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // bab74 cmp rdx,0x3
            fl = sub::<64>(rdx, 0x3u64).1;
            // bab78 ja 0xbac03
            if fl.a() { pc = 0xbac03; continue; }
            pc = 0xbab7e;
        }
        0xbab7e => {
            // block bab7e
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab7e xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // bab80 or al,bl
            { let r = (rax as u8 as u64) | (rbx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // bab82 jne 0xbac03
            if fl.ne() { pc = 0xbac03; continue; }
            pc = 0xbab84;
        }
        0xbab84 => {
            // block bab84
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab84 lea rax,[rip+0x47e75] # 0x102a00
            rax = m.env.data_va(94669);
            // bab8b mov eax,DWORD PTR [rax+rdx*4]
            rax = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // bab8e lea rsi,[rip+0x47e7b] # 0x102a10
            rsi = m.env.data_va(94685);
            // bab95 mov edx,DWORD PTR [rsi+rdx*4]
            rdx = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // bab98 mov DWORD PTR [rbp-0x40],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), ((rcx as u32 as u64)) as u32);
            // bab9b mov DWORD PTR [rbp-0x3c],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((r12 as u32 as u64)) as u32);
            // bab9f mov DWORD PTR [rbp-0x38],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rax as u32 as u64)) as u32);
            // baba2 mov DWORD PTR [rbp-0x34],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rdx as u32 as u64)) as u32);
            // baba5 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // baba9 cmp rax,QWORD PTR [r13+0x10]
            fl = sub::<64>(rax, m.r64(r13.wrapping_add(0x10)) as u64).1;
            // babad je 0xbabc5
            if fl.e() { pc = 0xbabc5; continue; }
            pc = 0xbabaf;
        }
        0xbabaf => {
            // block babaf
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babaf mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // babb3 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // babb7 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // babbb mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // babbe add QWORD PTR [r13+0x8],0x10
            { let (r, f) = add::<64>(m.r64(r13.wrapping_add(0x8)) as u64, 0x10u64);
              m.w64(r13.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // babc3 jmp 0xbabd1
            pc = 0xbabd1; continue;
        }
        0xbabc5 => {
            // block babc5
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babc5 mov rdi,r13
            rdi = r13;
            // babc8 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // babcc call 0xebb2e
            { let r = { let t = m.env.stub_va(0x2fb); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbabd1;
        }
        0xbabd1 => {
            // block babd1
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babd1 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // babd5 test rax,rax
            fl = logic::<64>(rax & rax);
            // babd8 je 0xbabe7
            if fl.e() { pc = 0xbabe7; continue; }
            pc = 0xbabda;
        }
        0xbabda => {
            // block babda
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babda mov r15,rax
            r15 = rax;
            // babdd mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // babe0 test rax,rax
            fl = logic::<64>(rax & rax);
            // babe3 jne 0xbabda
            if fl.ne() { pc = 0xbabda; continue; }
            pc = 0xbabe5;
        }
        0xbabe5 => {
            // block babe5
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babe5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babe5 jmp 0xbabf6
            pc = 0xbabf6; continue;
        }
        0xbabe7 => {
            // block babe7
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babe7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babe7 mov rax,r15
            rax = r15;
            pc = 0xbabea;
        }
        0xbabea => {
            // block babea
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babea mov r15,QWORD PTR [rax+0x10]
            r15 = m.r64(rax.wrapping_add(0x10)) as u64;
            // babee cmp QWORD PTR [r15],rax
            fl = sub::<64>(m.r64(r15) as u64, rax).1;
            // babf1 mov rax,r15
            rax = r15;
            // babf4 jne 0xbabea
            if fl.ne() { pc = 0xbabea; continue; }
            pc = 0xbabf6;
        }
        0xbabf6 => {
            // block babf6
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babf6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babf6 cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // babf9 jne 0xbab46
            if fl.ne() { pc = 0xbab46; continue; }
            pc = 0xbabff;
        }
        0xbabff => {
            // block babff
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babff mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // bac01 jmp 0xbac80
            pc = 0xbac80; continue;
        }
        0xbac03 => {
            // block bac03
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac03 mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // bac07 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bac0b cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bac0e je 0xbac7e
            if fl.e() { pc = 0xbac7e; continue; }
            pc = 0xbac10;
        }
        0xbac10 => {
            // block bac10
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac10 lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bac14 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bac17 not rdx
            rdx = !rdx;
            // bac1a and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bac1e add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bac21 mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            // bac25 jmp 0xbac7e
            pc = 0xbac7e; continue;
        }
        0xbac27 => {
            // block bac27
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac27 mov r14,rax
            r14 = rax;
            // bac2a cmp edx,0x1
            fl = sub::<32>((rdx as u32 as u64), 0x1u64).1;
            // bac2d jne 0xbac8f
            if fl.ne() { pc = 0xbac8f; continue; }
            pc = 0xbac2f;
        }
        0xbac2f => {
            // block bac2f
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac2f mov rdi,r14
            rdi = r14;
            // bac32 call 0xec164
            { let r = { let t = m.env.stub_va(0x931); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bac37 mov rax,QWORD PTR [rip+0x8578a] # 0x1403c8
            rax = m.r64(m.env.data_va(347029)) as u64;
            // bac3e add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // bac42 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // bac46 mov rdi,r14
            rdi = r14;
            // bac49 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bac4e mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // bac52 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bac56 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bac59 je 0xbac70
            if fl.e() { pc = 0xbac70; continue; }
            pc = 0xbac5b;
        }
        0xbac5b => {
            // block bac5b
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac5b lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bac5f sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bac62 not rdx
            rdx = !rdx;
            // bac65 and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bac69 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bac6c mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            pc = 0xbac70;
        }
        0xbac70 => {
            // block bac70
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac70 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // bac74 call 0xec128
            { let r = { let t = m.env.stub_va(0x8f5); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bac79 call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbac7e;
        }
        0xbac7e => {
            // block bac7e
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac7e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xbac80;
        }
        0xbac80 => {
            // block bac80
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac80 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bac84 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bac85 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bac87 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bac89 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bac8b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bac8d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bac8e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbac8f => {
            // block bac8f
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac8f mov rdi,r14
            rdi = r14;
            // bac92 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7a88 size=450
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7a88<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb7a88;
    loop {
        match pc {
        0xb7a88 => {
            // block b7a88
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7a88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a88 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7a89 mov rbp,rsp
            rbp = rsp;
            // b7a8c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7a8e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7a8f sub rsp,0x60
            { let (r, f) = sub::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // b7a93 mov rbx,rsi
            rbx = rsi;
            // b7a96 mov r14,rdi
            r14 = rdi;
            // b7a99 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b7a9c call QWORD PTR [rax+0x1d0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7aa2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b7aa6 mov rsi,rax
            rsi = rax;
            // b7aa9 call 0xb799c
            { let r = { let t = m.env.code_va(0xb799c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7aae test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // b7ab1 jne 0xb7aba
            if fl.ne() { pc = 0xb7aba; continue; }
            pc = 0xb7ab3;
        }
        0xb7ab3 => {
            // block b7ab3
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7ab3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ab3 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // b7ab8 jmp 0xb7ac9
            pc = 0xb7ac9; continue;
        }
        0xb7aba => {
            // block b7aba
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7aba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7aba mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b7abe mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7ac1 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7ac9;
        }
        0xb7ac9 => {
            // block b7ac9
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7ac9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ac9 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b7acb mov rdi,rbx
            rdi = rbx;
            // b7ace call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7ad3 mov rax,QWORD PTR [rbp-0x20]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // b7ad7 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // b7adb mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b7adf mov rcx,QWORD PTR [rbp-0x28]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b7ae3 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // b7ae7 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b7aea xorps xmm0,xmm0
            x0 = 0;
            // b7aed movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // b7af1 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // b7af9 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b7afd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7b02 movzx eax,BYTE PTR [rbx]
            rax = (m.r8(rbx) as u64) as u32 as u64;
            // b7b05 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // b7b07 jne 0xb7b0e
            if fl.ne() { pc = 0xb7b0e; continue; }
            pc = 0xb7b09;
        }
        0xb7b09 => {
            // block b7b09
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b09 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // b7b0c jmp 0xb7b12
            pc = 0xb7b12; continue;
        }
        0xb7b0e => {
            // block b7b0e
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b0e mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0xb7b12;
        }
        0xb7b12 => {
            // block b7b12
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b12 test rax,rax
            fl = logic::<64>(rax & rax);
            // b7b15 je 0xb7b49
            if fl.e() { pc = 0xb7b49; continue; }
            pc = 0xb7b17;
        }
        0xb7b17 => {
            // block b7b17
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b17 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b7b1a mov rdi,r14
            rdi = r14;
            // b7b1d call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7b23 cmp eax,0x10710800
            fl = sub::<32>((rax as u32 as u64), 0x10710800u64).1;
            // b7b28 jne 0xb7b60
            if fl.ne() { pc = 0xb7b60; continue; }
            pc = 0xb7b2a;
        }
        0xb7b2a => {
            // block b7b2a
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b2a lea rsi,[rip+0x802ca] # 0x137dfb
            rsi = m.env.data_va(312776);
            // b7b31 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b7b35 mov rdx,rbx
            rdx = rbx;
            // b7b38 call 0xebcf6
            { let r = { let t = m.env.stub_va(0x4c3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7b3d test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // b7b40 jne 0xb7b90
            if fl.ne() { pc = 0xb7b90; continue; }
            pc = 0xb7b42;
        }
        0xb7b42 => {
            // block b7b42
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b42 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // b7b47 jmp 0xb7b9f
            pc = 0xb7b9f; continue;
        }
        0xb7b49 => {
            // block b7b49
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b49 lea rsi,[rip+0x802a2] # 0x137df2
            rsi = m.env.data_va(312767);
            // b7b50 mov rdi,rbx
            rdi = rbx;
            // b7b53 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7b58 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b7b5b jmp 0xb7c25
            pc = 0xb7c25; continue;
        }
        0xb7b60 => {
            // block b7b60
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b60 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b7b63 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // b7b66 cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // b7b6b jne 0xb7c25
            if fl.ne() { pc = 0xb7c25; continue; }
            pc = 0xb7b71;
        }
        0xb7b71 => {
            // block b7b71
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b71 lea rsi,[rip+0x7d2e8] # 0x134e60
            rsi = m.env.data_va(300589);
            // b7b78 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b7b7c mov rdx,rbx
            rdx = rbx;
            // b7b7f call 0xebcf6
            { let r = { let t = m.env.stub_va(0x4c3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7b84 test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // b7b87 jne 0xb7bdd
            if fl.ne() { pc = 0xb7bdd; continue; }
            pc = 0xb7b89;
        }
        0xb7b89 => {
            // block b7b89
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b89 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // b7b8e jmp 0xb7bec
            pc = 0xb7bec; continue;
        }
        0xb7b90 => {
            // block b7b90
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b90 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b7b94 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7b97 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7b9f;
        }
        0xb7b9f => {
            // block b7b9f
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b9f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b7ba1 mov rdi,rbx
            rdi = rbx;
            // b7ba4 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7ba9 mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b7bad mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // b7bb1 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // b7bb5 mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b7bb9 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // b7bbd mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b7bc0 xorps xmm0,xmm0
            x0 = 0;
            // b7bc3 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // b7bc7 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // b7bcf lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b7bd3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7bd8 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // b7bdb jmp 0xb7c25
            pc = 0xb7c25; continue;
        }
        0xb7bdd => {
            // block b7bdd
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7bdd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7bdd mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b7be1 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7be4 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7bec;
        }
        0xb7bec => {
            // block b7bec
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7bec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7bec xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b7bee mov rdi,rbx
            rdi = rbx;
            // b7bf1 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7bf6 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // b7bfa mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // b7bfe mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // b7c02 mov rcx,QWORD PTR [rbp-0x68]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // b7c06 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // b7c0a mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b7c0d xorps xmm0,xmm0
            x0 = 0;
            // b7c10 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // b7c14 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // b7c1c lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b7c20 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb7c25;
        }
        0xb7c25 => {
            // block b7c25
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7c25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c25 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b7c28 add rsp,0x60
            { let (r, f) = add::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // b7c2c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7c2d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7c2f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7c30 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb7c31 => {
            // block b7c31
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7c31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c31 mov rdi,rax
            rdi = rax;
            // b7c34 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7c39 mov rdi,rax
            rdi = rax;
            // b7c3c call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7c41 mov rdi,rax
            rdi = rax;
            // b7c44 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6b52 size=600
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6b52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb6b52;
    loop {
        match pc {
        0xb6b52 => {
            // block b6b52
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6b52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6b52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6b53 mov rbp,rsp
            rbp = rsp;
            // b6b56 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b6b58 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b6b5a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b6b5c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b6b5e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b6b5f sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // b6b66 mov r12,rsi
            r12 = rsi;
            // b6b69 mov r13,rdi
            r13 = rdi;
            // b6b6c lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b6b73 lea rbx,[rbp-0x140]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // b6b7a mov rax,QWORD PTR [rip+0x8964f] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b6b81 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b6b85 mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // b6b8c add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b6b90 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // b6b97 mov rsi,rbx
            rsi = rbx;
            // b6b9a call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6b9f mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // b6ba7 mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // b6bae mov r14,QWORD PTR [rip+0x8980b] # 0x1403c0
            r14 = m.r64(m.env.data_va(347021)) as u64;
            // b6bb5 lea rax,[r14+0x18]
            rax = r14.wrapping_add(0x18);
            // b6bb9 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b6bc0 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // b6bc7 add r14,0x40
            { let (r, f) = add::<64>(r14, 0x40u64);
              r14 = r;
              fl = f; }
            // b6bcb mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // b6bd2 mov rdi,rbx
            rdi = rbx;
            // b6bd5 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6bda mov rdi,rbx
            rdi = rbx;
            // b6bdd mov r15,QWORD PTR [rip+0x897c4] # 0x1403a8
            r15 = m.r64(m.env.data_va(346997)) as u64;
            // b6be4 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // b6be8 mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // b6bef mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // b6bfa mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // b6c05 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // b6c10 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // b6c1b mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // b6c25 xorps xmm0,xmm0
            x0 = 0;
            // b6c28 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // b6c2c mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // b6c34 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6c38 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6c3d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6c41 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6c46 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b6c4a mov rax,QWORD PTR [rax+0x198]
            rax = m.r64(rax.wrapping_add(0x198)) as u64;
            // b6c51 mov rdi,r12
            rdi = r12;
            // b6c54 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6c56 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b6c5d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b6c5f mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // b6c61 call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6c66 movzx edx,BYTE PTR [rbp-0x160]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64) as u32 as u64;
            // b6c6d test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b6c70 jne 0xb6c7e
            if fl.ne() { pc = 0xb6c7e; continue; }
            pc = 0xb6c72;
        }
        0xb6c72 => {
            // block b6c72
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6c72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6c72 lea rsi,[rbp-0x15f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea1);
            // b6c79 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b6c7c jmp 0xb6c8c
            pc = 0xb6c8c; continue;
        }
        0xb6c7e => {
            // block b6c7e
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6c7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6c7e mov rdx,QWORD PTR [rbp-0x158]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // b6c85 mov rsi,QWORD PTR [rbp-0x150]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            pc = 0xb6c8c;
        }
        0xb6c8c => {
            // block b6c8c
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6c8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6c8c lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b6c93 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6c98 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b6c9f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6ca4 mov rdi,r13
            rdi = r13;
            // b6ca7 lea rsi,[rbp-0x140]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // b6cae call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6cb3 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // b6cba mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // b6cc1 mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // b6cc8 mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // b6ccf lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // b6cd6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6cdb lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // b6ce2 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6ce7 mov rsi,QWORD PTR [rip+0x8951a] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b6cee add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b6cf2 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b6cf9 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6cfe lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b6d05 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6d0a mov rax,r13
            rax = r13;
            // b6d0d add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // b6d14 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b6d15 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b6d17 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b6d19 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b6d1b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b6d1d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6d1e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb6d1f => {
            // block b6d1f
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6d1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6d1f mov rbx,rax
            rbx = rax;
            // b6d22 jmp 0xb6d4b
            pc = 0xb6d4b; continue;
        }
        0xb6d24 => {
            // block b6d24
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6d24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6d24 mov rbx,rax
            rbx = rax;
            // b6d27 jmp 0xb6d96
            pc = 0xb6d96; continue;
        }
        0xb6d29 => {
            // block b6d29
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6d29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6d29 mov rbx,rax
            rbx = rax;
            // b6d2c jmp 0xb6d7f
            pc = 0xb6d7f; continue;
        }
        0xb6d2e => {
            // block b6d2e
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6d2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6d2e mov rbx,rax
            rbx = rax;
            // b6d31 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6d35 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6d3a jmp 0xb6d67
            pc = 0xb6d67; continue;
        }
        0xb6d3c => {
            // block b6d3c
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6d3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6d3c mov rbx,rax
            rbx = rax;
            // b6d3f lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b6d46 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6d4b;
        }
        0xb6d4b => {
            // block b6d4b
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6d4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6d4b mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // b6d52 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // b6d59 mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // b6d60 mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            pc = 0xb6d67;
        }
        0xb6d67 => {
            // block b6d67
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6d67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6d67 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // b6d6e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6d73 lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // b6d7a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6d7f;
        }
        0xb6d7f => {
            // block b6d7f
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6d7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6d7f mov rsi,QWORD PTR [rip+0x89482] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b6d86 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b6d8a lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b6d91 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6d96;
        }
        0xb6d96 => {
            // block b6d96
            #[cfg(feature = "trace")] eprintln!("f_000b6b52 pc b6d96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6d96 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b6d9d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6da2 mov rdi,rbx
            rdi = rbx;
            // b6da5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6daa size=652
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6daa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb6daa;
    loop {
        match pc {
        0xb6daa => {
            // block b6daa
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b6daa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6daa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6dab mov rbp,rsp
            rbp = rsp;
            // b6dae push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b6db0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b6db2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b6db4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b6db6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b6db7 sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // b6dbe mov r15,rsi
            r15 = rsi;
            // b6dc1 mov r14,rdi
            r14 = rdi;
            // b6dc4 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b6dcb lea rbx,[rbp-0x140]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // b6dd2 mov rax,QWORD PTR [rip+0x893f7] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b6dd9 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b6ddd mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // b6de4 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b6de8 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // b6def mov rsi,rbx
            rsi = rbx;
            // b6df2 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6df7 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // b6dff mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // b6e06 mov r12,QWORD PTR [rip+0x895b3] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // b6e0d lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // b6e12 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b6e19 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // b6e20 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // b6e24 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // b6e2b mov rdi,rbx
            rdi = rbx;
            // b6e2e call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6e33 mov r13,QWORD PTR [rip+0x8956e] # 0x1403a8
            r13 = m.r64(m.env.data_va(346997)) as u64;
            // b6e3a add r13,0x10
            { let (r, f) = add::<64>(r13, 0x10u64);
              r13 = r;
              fl = f; }
            // b6e3e mov QWORD PTR [rbp-0x140],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r13) as u64);
            // b6e45 mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // b6e50 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // b6e5b mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // b6e66 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // b6e71 mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // b6e7b xorps xmm0,xmm0
            x0 = 0;
            // b6e7e movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // b6e82 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // b6e8a lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6e8e mov rdi,rbx
            rdi = rbx;
            // b6e91 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6e96 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6e9a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6e9f mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b6ea2 mov rax,QWORD PTR [rax+0x198]
            rax = m.r64(rax.wrapping_add(0x198)) as u64;
            // b6ea9 mov rdi,r15
            rdi = r15;
            // b6eac call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6eae lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b6eb5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b6eb7 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // b6eb9 call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6ebe mov QWORD PTR [rbp-0x170],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (r14) as u64);
            // b6ec5 movzx edx,BYTE PTR [rbp-0x160]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64) as u32 as u64;
            // b6ecc test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b6ecf jne 0xb6edd
            if fl.ne() { pc = 0xb6edd; continue; }
            pc = 0xb6ed1;
        }
        0xb6ed1 => {
            // block b6ed1
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b6ed1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6ed1 lea rsi,[rbp-0x15f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea1);
            // b6ed8 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b6edb jmp 0xb6eeb
            pc = 0xb6eeb; continue;
        }
        0xb6edd => {
            // block b6edd
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b6edd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6edd mov rdx,QWORD PTR [rbp-0x158]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // b6ee4 mov rsi,QWORD PTR [rbp-0x150]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            pc = 0xb6eeb;
        }
        0xb6eeb => {
            // block b6eeb
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b6eeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6eeb lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b6ef2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6ef7 lea rsi,[rip+0x7338a] # 0x12a288
            rsi = m.env.data_va(256597);
            // b6efe mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b6f03 mov rdi,rax
            rdi = rax;
            // b6f06 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6f0b mov r14,rax
            r14 = rax;
            // b6f0e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b6f11 mov rax,QWORD PTR [rax+0x1a0]
            rax = m.r64(rax.wrapping_add(0x1a0)) as u64;
            // b6f18 mov rdi,r15
            rdi = r15;
            // b6f1b call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6f1d movzx esi,ax
            rsi = ((rax as u16 as u64)) as u32 as u64;
            // b6f20 mov rdi,r14
            rdi = r14;
            // b6f23 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6f28 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b6f2f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6f34 mov r14,QWORD PTR [rbp-0x170]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // b6f3b mov rdi,r14
            rdi = r14;
            // b6f3e mov rsi,rbx
            rsi = rbx;
            // b6f41 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6f46 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // b6f4d mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // b6f54 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // b6f5b mov QWORD PTR [rbp-0x140],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r13) as u64);
            // b6f62 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // b6f69 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6f6e mov rdi,rbx
            rdi = rbx;
            // b6f71 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6f76 mov rsi,QWORD PTR [rip+0x8928b] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b6f7d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b6f81 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b6f88 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6f8d lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b6f94 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6f99 mov rax,r14
            rax = r14;
            // b6f9c add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // b6fa3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b6fa4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b6fa6 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b6fa8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b6faa pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b6fac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6fad ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb6fae => {
            // block b6fae
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b6fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6fae mov r14,rax
            r14 = rax;
            // b6fb1 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b6fb8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6fbd jmp 0xb6fc2
            pc = 0xb6fc2; continue;
        }
        0xb6fbf => {
            // block b6fbf
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b6fbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6fbf mov r14,rax
            r14 = rax;
            pc = 0xb6fc2;
        }
        0xb6fc2 => {
            // block b6fc2
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b6fc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6fc2 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // b6fc9 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // b6fd0 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // b6fd7 mov QWORD PTR [rbp-0x140],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r13) as u64);
            pc = 0xb6fde;
        }
        0xb6fde => {
            // block b6fde
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b6fde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6fde lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // b6fe5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6fea mov rdi,rbx
            rdi = rbx;
            // b6fed call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6ff2;
        }
        0xb6ff2 => {
            // block b6ff2
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b6ff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6ff2 mov rsi,QWORD PTR [rip+0x8920f] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b6ff9 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b6ffd lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b7004 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb7009;
        }
        0xb7009 => {
            // block b7009
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b7009 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7009 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b7010 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7015 mov rdi,r14
            rdi = r14;
            // b7018 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b701d mov r14,rax
            r14 = rax;
            // b7020 jmp 0xb7009
            pc = 0xb7009; continue;
        }
        0xb7022 => {
            // block b7022
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b7022 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7022 mov r14,rax
            r14 = rax;
            // b7025 jmp 0xb6ff2
            pc = 0xb6ff2; continue;
        }
        0xb7027 => {
            // block b7027
            #[cfg(feature = "trace")] eprintln!("f_000b6daa pc b7027 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7027 mov r14,rax
            r14 = rax;
            // b702a lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b702e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7033 jmp 0xb6fde
            pc = 0xb6fde; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdf1fa size=722
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000df1fa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdf1fa;
    loop {
        match pc {
        0xdf1fa => {
            // block df1fa
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df1fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df1fa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df1fb mov rbp,rsp
            rbp = rsp;
            // df1fe push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // df200 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // df202 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // df204 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df205 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // df209 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // df20c mov r12,rdx
            r12 = rdx;
            // df20f mov r15,rsi
            r15 = rsi;
            // df212 mov rbx,rdi
            rbx = rdi;
            // df215 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df218 lea rsi,[rbp-0x21]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdf);
            // df21c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // df21e call QWORD PTR [rax+0x410]
            { let r = { let t = m.r64(rax.wrapping_add(0x410)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df224 mov eax,DWORD PTR [rbx+0x274]
            rax = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // df22a cmp rax,0x14
            fl = sub::<64>(rax, 0x14u64).1;
            // df22e ja 0xdf250
            if fl.a() { pc = 0xdf250; continue; }
            pc = 0xdf230;
        }
        0xdf230 => {
            // block df230
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df230 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df230 lea rcx,[rip+0x241] # 0xdf478
            rcx = m.env.code_va(0xdf478);
            // df237 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // df23b add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // df23e jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xdf240 => {
            // block df240
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df240 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df240 mov rdi,r15
            rdi = r15;
            // df243 mov rsi,r12
            rsi = r12;
            // df246 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df249 call 0x674ee
            { let r = { let t = m.env.code_va(0x674ee); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df24e jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf250 => {
            // block df250
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df250 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df250 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df253 lea rsi,[rip+0x5a116] # 0x139370
            rsi = m.env.data_va(318269);
            // df25a mov rdi,rbx
            rdi = rbx;
            // df25d call QWORD PTR [rax+0x170]
            { let r = { let t = m.r64(rax.wrapping_add(0x170)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdf263;
        }
        0xdf263 => {
            // block df263
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df263 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df263 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // df267 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // df268 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // df26a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // df26c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // df26e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // df26f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdf270 => {
            // block df270
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df270 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df270 mov rdi,r15
            rdi = r15;
            // df273 mov rsi,r12
            rsi = r12;
            // df276 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df279 call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df27e jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf280 => {
            // block df280
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df280 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df280 movzx ecx,BYTE PTR [rbp-0x21]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64) as u32 as u64;
            // df284 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df287 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // df28a mov rdi,r15
            rdi = r15;
            // df28d mov rsi,r12
            rsi = r12;
            // df290 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df293 call 0x663c4
            { let r = { let t = m.env.code_va(0x663c4); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df298 jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf29a => {
            // block df29a
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df29a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df29a movzx ecx,BYTE PTR [rbp-0x21]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64) as u32 as u64;
            // df29e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df2a1 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // df2a4 mov rdi,r15
            rdi = r15;
            // df2a7 mov rsi,r12
            rsi = r12;
            // df2aa mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df2ad call 0x663c4
            { let r = { let t = m.env.code_va(0x663c4); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df2b2 mov rdi,r12
            rdi = r12;
            // df2b5 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // df2b8 call 0x67556
            { let r = { let t = m.env.code_va(0x67556); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df2bd jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf2bf => {
            // block df2bf
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df2bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df2bf movzx ecx,BYTE PTR [rbp-0x21]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64) as u32 as u64;
            // df2c3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df2c6 mov rdi,r15
            rdi = r15;
            // df2c9 mov rsi,r12
            rsi = r12;
            // df2cc mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df2cf call 0x66c24
            { let r = { let t = m.env.code_va(0x66c24); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df2d4 mov rdi,r12
            rdi = r12;
            // df2d7 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // df2da call 0x6768e
            { let r = { let t = m.env.code_va(0x6768e); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df2df jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf2e1 => {
            // block df2e1
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df2e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df2e1 mov rdi,r15
            rdi = r15;
            // df2e4 mov rsi,r12
            rsi = r12;
            // df2e7 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df2ea call 0x674ee
            { let r = { let t = m.env.code_va(0x674ee); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df2ef mov rdi,r12
            rdi = r12;
            // df2f2 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // df2f5 call 0x67511
            { let r = { let t = m.env.code_va(0x67511); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df2fa jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf2ff => {
            // block df2ff
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df2ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df2ff movzx ecx,BYTE PTR [rbp-0x21]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64) as u32 as u64;
            // df303 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df306 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // df309 mov rdi,r15
            rdi = r15;
            // df30c mov rsi,r12
            rsi = r12;
            // df30f mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df312 call 0x663c4
            { let r = { let t = m.env.code_va(0x663c4); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df317 mov rdi,r12
            rdi = r12;
            // df31a mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // df31d call 0x67592
            { let r = { let t = m.env.code_va(0x67592); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df322 jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf327 => {
            // block df327
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df327 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df327 movzx ecx,BYTE PTR [rbp-0x21]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64) as u32 as u64;
            // df32b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df32e mov rdi,r15
            rdi = r15;
            // df331 mov rsi,r12
            rsi = r12;
            // df334 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df337 call 0x66c24
            { let r = { let t = m.env.code_va(0x66c24); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df33c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // df341 jmp 0xdf419
            pc = 0xdf419; continue;
        }
        0xdf346 => {
            // block df346
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df346 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df346 mov rdi,r15
            rdi = r15;
            // df349 mov rsi,r12
            rsi = r12;
            // df34c mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df34f call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df354 mov rdi,r12
            rdi = r12;
            // df357 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // df35a call 0x58c54
            { let r = { let t = m.env.code_va(0x58c54); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df35f jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf364 => {
            // block df364
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df364 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df364 movzx ecx,BYTE PTR [rbp-0x21]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64) as u32 as u64;
            // df368 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df36b xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // df36e mov rdi,r15
            rdi = r15;
            // df371 mov rsi,r12
            rsi = r12;
            // df374 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df377 call 0x663c4
            { let r = { let t = m.env.code_va(0x663c4); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df37c mov rdi,r12
            rdi = r12;
            // df37f mov rsi,r12
            rsi = r12;
            // df382 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df385 call 0x675ba
            { let r = { let t = m.env.code_va(0x675ba); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df38a jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf38f => {
            // block df38f
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df38f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df38f movzx ecx,BYTE PTR [rbp-0x21]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64) as u32 as u64;
            // df393 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df396 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // df399 mov rdi,r15
            rdi = r15;
            // df39c mov rsi,r12
            rsi = r12;
            // df39f mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df3a2 call 0x663c4
            { let r = { let t = m.env.code_va(0x663c4); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df3a7 mov rdi,r12
            rdi = r12;
            // df3aa mov rsi,r12
            rsi = r12;
            // df3ad mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df3b0 call 0x6765a
            { let r = { let t = m.env.code_va(0x6765a); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df3b5 jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf3ba => {
            // block df3ba
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df3ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df3ba mov rdi,r15
            rdi = r15;
            // df3bd mov rsi,r12
            rsi = r12;
            // df3c0 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df3c3 call 0x6744c
            { let r = { let t = m.env.code_va(0x6744c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df3c8 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // df3cb jle 0xdf263
            if fl.le() { pc = 0xdf263; continue; }
            pc = 0xdf3d1;
        }
        0xdf3d1 => {
            // block df3d1
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df3d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df3d1 mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // df3d5 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // df3d7 and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // df3dd and eax,0xffc00
            { let r = (rax as u32 as u64) & 0xffc00u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // df3e2 mov edx,0x3ec
            rdx = (0x3ecu64) as u32 as u64;
            // df3e7 sub edx,ecx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // df3e9 shl ecx,0x14
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x14u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // df3ec add ecx,edx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rdx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // df3ee or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // df3f0 mov DWORD PTR [r12],ecx
            m.w32(r12, ((rcx as u32 as u64)) as u32);
            // df3f4 add r12,0x4
            { let (r, f) = add::<64>(r12, 0x4u64);
              r12 = r;
              fl = f; }
            // df3f8 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // df3fb jne 0xdf3d1
            if fl.ne() { pc = 0xdf3d1; continue; }
            pc = 0xdf3fd;
        }
        0xdf3fd => {
            // block df3fd
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df3fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df3fd jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf402 => {
            // block df402
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df402 movzx ecx,BYTE PTR [rbp-0x21]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64) as u32 as u64;
            // df406 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df409 mov rdi,r15
            rdi = r15;
            // df40c mov rsi,r12
            rsi = r12;
            // df40f mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df412 call 0x66c24
            { let r = { let t = m.env.code_va(0x66c24); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df417 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xdf419;
        }
        0xdf419 => {
            // block df419
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df419 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df419 mov rdi,r12
            rdi = r12;
            // df41c mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // df41f call 0x676c0
            { let r = { let t = m.env.code_va(0x676c0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df424 jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf429 => {
            // block df429
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df429 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df429 movzx ecx,BYTE PTR [rbp-0x21]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64) as u32 as u64;
            // df42d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df430 mov rdi,r15
            rdi = r15;
            // df433 mov rsi,r12
            rsi = r12;
            // df436 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df439 call 0x67768
            { let r = { let t = m.env.code_va(0x67768); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df43e mov rdi,r12
            rdi = r12;
            // df441 mov rsi,r12
            rsi = r12;
            // df444 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df447 call 0x67628
            { let r = { let t = m.env.code_va(0x67628); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df44c jmp 0xdf263
            pc = 0xdf263; continue;
        }
        0xdf451 => {
            // block df451
            #[cfg(feature = "trace")] eprintln!("f_000df1fa pc df451 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df451 movzx ecx,BYTE PTR [rbp-0x21]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64) as u32 as u64;
            // df455 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df458 mov rdi,r15
            rdi = r15;
            // df45b mov rsi,r12
            rsi = r12;
            // df45e mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // df461 call 0x66c24
            { let r = { let t = m.env.code_va(0x66c24); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df466 mov rdi,r12
            rdi = r12;
            // df469 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // df46c call 0x676f9
            { let r = { let t = m.env.code_va(0x676f9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df471 jmp 0xdf263
            pc = 0xdf263; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbac98 size=940
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bac98<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbac98;
    loop {
        match pc {
        0xbac98 => {
            // block bac98
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bac98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac98 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bac99 mov rbp,rsp
            rbp = rsp;
            // bac9c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bac9e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // baca0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // baca2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // baca4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // baca5 sub rsp,0x88
            { let (r, f) = sub::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // bacac mov r14,rsi
            r14 = rsi;
            // bacaf mov rbx,rdi
            rbx = rdi;
            // bacb2 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // bacb4 je 0xbae79
            if fl.e() { pc = 0xbae79; continue; }
            pc = 0xbacba;
        }
        0xbacba => {
            // block bacba
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bacba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bacba mov rsi,QWORD PTR [rbx+0x18]
            rsi = m.r64(rbx.wrapping_add(0x18)) as u64;
            // bacbe mov rdi,r14
            rdi = r14;
            // bacc1 mov r15,r14
            r15 = r14;
            // bacc4 call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bacc9 lea rsi,[rip+0x7d134] # 0x137e04
            rsi = m.env.data_va(312785);
            // bacd0 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // bacd5 mov rdi,rax
            rdi = rax;
            // bacd8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bacdd mov r12,rax
            r12 = rax;
            // bace0 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // bace4 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // bace8 add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // baceb lea r14,[rbp-0x40]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bacef mov rdi,r14
            rdi = r14;
            // bacf2 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bacf7 mov rsi,QWORD PTR [rip+0x85492] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // bacfe mov rdi,r14
            rdi = r14;
            // bad01 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bad06 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // bad09 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // bad0d mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // bad12 mov rdi,rax
            rdi = rax;
            // bad15 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bad17 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // bad1a lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bad1e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bad23 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // bad27 mov rdi,r12
            rdi = r12;
            // bad2a call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bad2f mov rdi,r12
            rdi = r12;
            // bad32 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bad37 mov r13,QWORD PTR [rbx+0x8]
            r13 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bad3b add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // bad3f mov QWORD PTR [rbp-0xa8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rbx) as u64);
            // bad46 cmp r13,rbx
            fl = sub::<64>(r13, rbx).1;
            // bad49 mov r14,r15
            r14 = r15;
            // bad4c je 0xbafc6
            if fl.e() { pc = 0xbafc6; continue; }
            pc = 0xbad52;
        }
        0xbad52 => {
            // block bad52
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bad52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bad52 lea r12,[rbp-0x58]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0xbad56;
        }
        0xbad56 => {
            // block bad56
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bad56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bad56 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // bad5a mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // bad5f mov rdi,r12
            rdi = r12;
            // bad62 call 0x5d8db
            { let r = { let t = m.env.code_va(0x5d8db); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bad67 movzx edx,BYTE PTR [rbp-0x58]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // bad6b test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // bad6e jne 0xbad79
            if fl.ne() { pc = 0xbad79; continue; }
            pc = 0xbad70;
        }
        0xbad70 => {
            // block bad70
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bad70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bad70 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // bad73 lea rsi,[rbp-0x57]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa9);
            // bad77 jmp 0xbad81
            pc = 0xbad81; continue;
        }
        0xbad79 => {
            // block bad79
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bad79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bad79 mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // bad7d mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xbad81;
        }
        0xbad81 => {
            // block bad81
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bad81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bad81 mov rdi,r14
            rdi = r14;
            // bad84 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bad89 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // bad8e mov rdi,rax
            rdi = rax;
            // bad91 lea rsi,[rip+0x79ff4] # 0x134d8c
            rsi = m.env.data_va(300377);
            // bad98 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bad9d mov rbx,rax
            rbx = rax;
            // bada0 mov esi,DWORD PTR [r13+0x20]
            rsi = (m.r32(r13.wrapping_add(0x20)) as u64) as u32 as u64;
            // bada4 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // bada9 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // badad call 0x5e61c
            { let r = { let t = m.env.code_va(0x5e61c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // badb2 movzx edx,BYTE PTR [rbp-0x70]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // badb6 mov r15,r14
            r15 = r14;
            // badb9 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // badbc jne 0xbadc7
            if fl.ne() { pc = 0xbadc7; continue; }
            pc = 0xbadbe;
        }
        0xbadbe => {
            // block badbe
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc badbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // badbe shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // badc1 lea rsi,[rbp-0x6f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff91);
            // badc5 jmp 0xbadcf
            pc = 0xbadcf; continue;
        }
        0xbadc7 => {
            // block badc7
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc badc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // badc7 mov rdx,QWORD PTR [rbp-0x68]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // badcb mov rsi,QWORD PTR [rbp-0x60]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            pc = 0xbadcf;
        }
        0xbadcf => {
            // block badcf
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc badcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // badcf mov rdi,rbx
            rdi = rbx;
            // badd2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // badd7 mov r14,rax
            r14 = rax;
            // badda mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // baddd mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // bade1 add rsi,r14
            { let (r, f) = add::<64>(rsi, r14);
              rsi = r;
              fl = f; }
            // bade4 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bade8 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baded lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // badf1 mov rsi,QWORD PTR [rip+0x85398] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // badf8 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // badfd mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // bae00 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // bae04 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // bae09 mov rdi,rax
            rdi = rax;
            // bae0c call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bae0e mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // bae10 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bae14 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bae19 movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // bae1c mov rdi,r14
            rdi = r14;
            // bae1f call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bae24 mov rdi,r14
            rdi = r14;
            // bae27 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bae2c lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // bae30 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bae35 mov rdi,r12
            rdi = r12;
            // bae38 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bae3d mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bae41 test rax,rax
            fl = logic::<64>(rax & rax);
            // bae44 je 0xbae54
            if fl.e() { pc = 0xbae54; continue; }
            pc = 0xbae46;
        }
        0xbae46 => {
            // block bae46
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bae46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bae46 mov r13,rax
            r13 = rax;
            // bae49 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // bae4d test rax,rax
            fl = logic::<64>(rax & rax);
            // bae50 jne 0xbae46
            if fl.ne() { pc = 0xbae46; continue; }
            pc = 0xbae52;
        }
        0xbae52 => {
            // block bae52
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bae52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bae52 jmp 0xbae64
            pc = 0xbae64; continue;
        }
        0xbae54 => {
            // block bae54
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bae54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bae54 mov rax,r13
            rax = r13;
            pc = 0xbae57;
        }
        0xbae57 => {
            // block bae57
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bae57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bae57 mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // bae5b cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // bae5f mov rax,r13
            rax = r13;
            // bae62 jne 0xbae57
            if fl.ne() { pc = 0xbae57; continue; }
            pc = 0xbae64;
        }
        0xbae64 => {
            // block bae64
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bae64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bae64 cmp r13,QWORD PTR [rbp-0xa8]
            fl = sub::<64>(r13, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64).1;
            // bae6b mov r14,r15
            r14 = r15;
            // bae6e jne 0xbad56
            if fl.ne() { pc = 0xbad56; continue; }
            pc = 0xbae74;
        }
        0xbae74 => {
            // block bae74
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bae74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bae74 jmp 0xbafc6
            pc = 0xbafc6; continue;
        }
        0xbae79 => {
            // block bae79
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bae79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bae79 mov r13,QWORD PTR [rbx+0x8]
            r13 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bae7d add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // bae81 mov QWORD PTR [rbp-0xa8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rbx) as u64);
            // bae88 cmp r13,rbx
            fl = sub::<64>(r13, rbx).1;
            // bae8b je 0xbafc6
            if fl.e() { pc = 0xbafc6; continue; }
            pc = 0xbae91;
        }
        0xbae91 => {
            // block bae91
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bae91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bae91 lea r12,[rbp-0x88]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            pc = 0xbae98;
        }
        0xbae98 => {
            // block bae98
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bae98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bae98 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // bae9c mov rdi,r12
            rdi = r12;
            // bae9f call 0xbb044
            { let r = { let t = m.env.code_va(0xbb044); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baea4 movzx edx,BYTE PTR [rbp-0x88]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // baeab test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // baeae jne 0xbaebc
            if fl.ne() { pc = 0xbaebc; continue; }
            pc = 0xbaeb0;
        }
        0xbaeb0 => {
            // block baeb0
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc baeb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baeb0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // baeb3 lea rsi,[rbp-0x87]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff79);
            // baeba jmp 0xbaec4
            pc = 0xbaec4; continue;
        }
        0xbaebc => {
            // block baebc
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc baebc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baebc mov rdx,QWORD PTR [rbp-0x80]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // baec0 mov rsi,QWORD PTR [rbp-0x78]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            pc = 0xbaec4;
        }
        0xbaec4 => {
            // block baec4
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc baec4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baec4 mov rdi,r14
            rdi = r14;
            // baec7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baecc mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // baed1 mov rdi,rax
            rdi = rax;
            // baed4 lea rsi,[rip+0x79eb1] # 0x134d8c
            rsi = m.env.data_va(300377);
            // baedb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baee0 mov rbx,rax
            rbx = rax;
            // baee3 mov esi,DWORD PTR [r13+0x20]
            rsi = (m.r32(r13.wrapping_add(0x20)) as u64) as u32 as u64;
            // baee7 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // baeee call 0xbb098
            { let r = { let t = m.env.code_va(0xbb098); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baef3 movzx edx,BYTE PTR [rbp-0xa0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64) as u32 as u64;
            // baefa test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // baefd jne 0xbaf0b
            if fl.ne() { pc = 0xbaf0b; continue; }
            pc = 0xbaeff;
        }
        0xbaeff => {
            // block baeff
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc baeff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baeff shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // baf02 lea rsi,[rbp-0x9f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff61);
            // baf09 jmp 0xbaf19
            pc = 0xbaf19; continue;
        }
        0xbaf0b => {
            // block baf0b
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc baf0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baf0b mov rdx,QWORD PTR [rbp-0x98]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // baf12 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            pc = 0xbaf19;
        }
        0xbaf19 => {
            // block baf19
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc baf19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baf19 mov rdi,rbx
            rdi = rbx;
            // baf1c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baf21 mov rbx,rax
            rbx = rax;
            // baf24 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // baf27 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // baf2b add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // baf2e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // baf32 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baf37 mov r15,r14
            r15 = r14;
            // baf3a lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // baf3e mov rsi,QWORD PTR [rip+0x8524b] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // baf45 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baf4a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // baf4d mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // baf51 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // baf56 mov rdi,rax
            rdi = rax;
            // baf59 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baf5b mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // baf5e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // baf62 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baf67 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // baf6b mov rdi,rbx
            rdi = rbx;
            // baf6e call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baf73 mov r14,r15
            r14 = r15;
            // baf76 mov rdi,rbx
            rdi = rbx;
            // baf79 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baf7e lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // baf85 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baf8a mov rdi,r12
            rdi = r12;
            // baf8d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baf92 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // baf96 test rax,rax
            fl = logic::<64>(rax & rax);
            // baf99 je 0xbafa9
            if fl.e() { pc = 0xbafa9; continue; }
            pc = 0xbaf9b;
        }
        0xbaf9b => {
            // block baf9b
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc baf9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baf9b mov r13,rax
            r13 = rax;
            // baf9e mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // bafa2 test rax,rax
            fl = logic::<64>(rax & rax);
            // bafa5 jne 0xbaf9b
            if fl.ne() { pc = 0xbaf9b; continue; }
            pc = 0xbafa7;
        }
        0xbafa7 => {
            // block bafa7
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bafa7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bafa7 jmp 0xbafb9
            pc = 0xbafb9; continue;
        }
        0xbafa9 => {
            // block bafa9
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bafa9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bafa9 mov rax,r13
            rax = r13;
            pc = 0xbafac;
        }
        0xbafac => {
            // block bafac
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bafac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bafac mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // bafb0 cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // bafb4 mov rax,r13
            rax = r13;
            // bafb7 jne 0xbafac
            if fl.ne() { pc = 0xbafac; continue; }
            pc = 0xbafb9;
        }
        0xbafb9 => {
            // block bafb9
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bafb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bafb9 cmp r13,QWORD PTR [rbp-0xa8]
            fl = sub::<64>(r13, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64).1;
            // bafc0 jne 0xbae98
            if fl.ne() { pc = 0xbae98; continue; }
            pc = 0xbafc6;
        }
        0xbafc6 => {
            // block bafc6
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bafc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bafc6 mov rax,r14
            rax = r14;
            // bafc9 add rsp,0x88
            { let (r, f) = add::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // bafd0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bafd1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bafd3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bafd5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bafd7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bafd9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bafda ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbafdb => {
            // block bafdb
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bafdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bafdb mov rbx,rax
            rbx = rax;
            // bafde jmp 0xbaffb
            pc = 0xbaffb; continue;
        }
        0xbafe0 => {
            // block bafe0
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bafe0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bafe0 mov rbx,rax
            rbx = rax;
            // bafe3 jmp 0xbb016
            pc = 0xbb016; continue;
        }
        0xbafe5 => {
            // block bafe5
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bafe5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bafe5 mov rbx,rax
            rbx = rax;
            // bafe8 jmp 0xbb004
            pc = 0xbb004; continue;
        }
        0xbafea => {
            // block bafea
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bafea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bafea mov rbx,rax
            rbx = rax;
            // bafed jmp 0xbb022
            pc = 0xbb022; continue;
        }
        0xbafef => {
            // block bafef
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bafef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bafef mov rbx,rax
            rbx = rax;
            // baff2 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // baff6 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbaffb;
        }
        0xbaffb => {
            // block baffb
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc baffb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baffb lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // bafff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbb004;
        }
        0xbb004 => {
            // block bb004
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bb004 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb004 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // bb008 jmp 0xbb029
            pc = 0xbb029; continue;
        }
        0xbb00a => {
            // block bb00a
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bb00a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb00a mov rbx,rax
            rbx = rax;
            // bb00d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // bb011 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbb016;
        }
        0xbb016 => {
            // block bb016
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bb016 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb016 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // bb01d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbb022;
        }
        0xbb022 => {
            // block bb022
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bb022 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb022 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            pc = 0xbb029;
        }
        0xbb029 => {
            // block bb029
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bb029 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb029 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbb02e;
        }
        0xbb02e => {
            // block bb02e
            #[cfg(feature = "trace")] eprintln!("f_000bac98 pc bb02e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb02e mov rdi,rbx
            rdi = rbx;
            // bb031 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bb036 mov rbx,rax
            rbx = rax;
            // bb039 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bb03d call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bb042 jmp 0xbb02e
            pc = 0xbb02e; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4bd2 size=1074
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d4bd2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4bd2;
    loop {
        match pc {
        0xd4bd2 => {
            // block d4bd2
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4bd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4bd2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4bd3 mov rbp,rsp
            rbp = rsp;
            // d4bd6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4bd8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4bd9 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // d4bdd mov r14,rsi
            r14 = rsi;
            // d4be0 mov rbx,rdi
            rbx = rdi;
            // d4be3 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d4be6 call 0xcd8c1
            { let r = { let t = m.env.code_va(0xcd8c1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4beb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d4bed je 0xd4c3e
            if fl.e() { pc = 0xd4c3e; continue; }
            pc = 0xd4bef;
        }
        0xd4bef => {
            // block d4bef
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4bef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4bef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d4bf2 mov rdi,rbx
            rdi = rbx;
            // d4bf5 mov rsi,r14
            rsi = r14;
            // d4bf8 call QWORD PTR [rax+0x178]
            { let r = { let t = m.r64(rax.wrapping_add(0x178)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4bfe mov DWORD PTR [r14+0x94],0x0
            m.w32(r14.wrapping_add(0x94), (0x0u64) as u32);
            // d4c09 mov eax,DWORD PTR [rbx+0x10]
            rax = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d4c0c cmp eax,0x106344ff
            fl = sub::<32>((rax as u32 as u64), 0x106344ffu64).1;
            // d4c11 jg 0xd4c45
            if fl.g() { pc = 0xd4c45; continue; }
            pc = 0xd4c13;
        }
        0xd4c13 => {
            // block d4c13
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c13 cmp eax,0x105183ff
            fl = sub::<32>((rax as u32 as u64), 0x105183ffu64).1;
            // d4c18 jg 0xd4c72
            if fl.g() { pc = 0xd4c72; continue; }
            pc = 0xd4c1a;
        }
        0xd4c1a => {
            // block d4c1a
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c1a cmp eax,0x102447ff
            fl = sub::<32>((rax as u32 as u64), 0x102447ffu64).1;
            // d4c1f jg 0xd4cc9
            if fl.g() { pc = 0xd4cc9; continue; }
            pc = 0xd4c25;
        }
        0xd4c25 => {
            // block d4c25
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c25 cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // d4c28 jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4c2e;
        }
        0xd4c2e => {
            // block d4c2e
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c2e mov DWORD PTR [r14+0x94],0x0
            m.w32(r14.wrapping_add(0x94), (0x0u64) as u32);
            // d4c39 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4c3e => {
            // block d4c3e
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c3e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d4c40 jmp 0xd4ed6
            pc = 0xd4ed6; continue;
        }
        0xd4c45 => {
            // block d4c45
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c45 cmp eax,0x106681ff
            fl = sub::<32>((rax as u32 as u64), 0x106681ffu64).1;
            // d4c4a jg 0xd4c98
            if fl.g() { pc = 0xd4c98; continue; }
            pc = 0xd4c4c;
        }
        0xd4c4c => {
            // block d4c4c
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c4c cmp eax,0x106466ff
            fl = sub::<32>((rax as u32 as u64), 0x106466ffu64).1;
            // d4c51 jg 0xd4d05
            if fl.g() { pc = 0xd4d05; continue; }
            pc = 0xd4c57;
        }
        0xd4c57 => {
            // block d4c57
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c57 cmp eax,0x10634500
            fl = sub::<32>((rax as u32 as u64), 0x10634500u64).1;
            // d4c5c jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4c62;
        }
        0xd4c62 => {
            // block d4c62
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c62 mov DWORD PTR [r14+0x94],0x2b
            m.w32(r14.wrapping_add(0x94), (0x2bu64) as u32);
            // d4c6d jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4c72 => {
            // block d4c72
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c72 cmp eax,0x105381ff
            fl = sub::<32>((rax as u32 as u64), 0x105381ffu64).1;
            // d4c77 jg 0xd4d33
            if fl.g() { pc = 0xd4d33; continue; }
            pc = 0xd4c7d;
        }
        0xd4c7d => {
            // block d4c7d
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c7d cmp eax,0x10518400
            fl = sub::<32>((rax as u32 as u64), 0x10518400u64).1;
            // d4c82 jne 0xd4d61
            if fl.ne() { pc = 0xd4d61; continue; }
            pc = 0xd4c88;
        }
        0xd4c88 => {
            // block d4c88
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c88 mov DWORD PTR [r14+0x94],0x27
            m.w32(r14.wrapping_add(0x94), (0x27u64) as u32);
            // d4c93 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4c98 => {
            // block d4c98
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4c98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c98 cmp eax,0x1071084f
            fl = sub::<32>((rax as u32 as u64), 0x1071084fu64).1;
            // d4c9d jg 0xd4d4a
            if fl.g() { pc = 0xd4d4a; continue; }
            pc = 0xd4ca3;
        }
        0xd4ca3 => {
            // block d4ca3
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4ca3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4ca3 cmp eax,0x10668200
            fl = sub::<32>((rax as u32 as u64), 0x10668200u64).1;
            // d4ca8 je 0xd4e61
            if fl.e() { pc = 0xd4e61; continue; }
            pc = 0xd4cae;
        }
        0xd4cae => {
            // block d4cae
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4cae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cae cmp eax,0x10710800
            fl = sub::<32>((rax as u32 as u64), 0x10710800u64).1;
            // d4cb3 jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4cb9;
        }
        0xd4cb9 => {
            // block d4cb9
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4cb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cb9 mov DWORD PTR [r14+0x94],0x32
            m.w32(r14.wrapping_add(0x94), (0x32u64) as u32);
            // d4cc4 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4cc9 => {
            // block d4cc9
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4cc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cc9 cmp eax,0x1032294f
            fl = sub::<32>((rax as u32 as u64), 0x1032294fu64).1;
            // d4cce jg 0xd4d7c
            if fl.g() { pc = 0xd4d7c; continue; }
            pc = 0xd4cd4;
        }
        0xd4cd4 => {
            // block d4cd4
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4cd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cd4 cmp eax,0x102802ff
            fl = sub::<32>((rax as u32 as u64), 0x102802ffu64).1;
            // d4cd9 jg 0xd4ddf
            if fl.g() { pc = 0xd4ddf; continue; }
            pc = 0xd4cdf;
        }
        0xd4cdf => {
            // block d4cdf
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4cdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cdf cmp eax,0x10244800
            fl = sub::<32>((rax as u32 as u64), 0x10244800u64).1;
            // d4ce4 je 0xd4e4d
            if fl.e() { pc = 0xd4e4d; continue; }
            pc = 0xd4cea;
        }
        0xd4cea => {
            // block d4cea
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4cea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cea cmp eax,0x10266400
            fl = sub::<32>((rax as u32 as u64), 0x10266400u64).1;
            // d4cef jne 0xd4e5a
            if fl.ne() { pc = 0xd4e5a; continue; }
            pc = 0xd4cf5;
        }
        0xd4cf5 => {
            // block d4cf5
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4cf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cf5 mov DWORD PTR [r14+0x94],0x1b
            m.w32(r14.wrapping_add(0x94), (0x1bu64) as u32);
            // d4d00 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4d05 => {
            // block d4d05
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d05 add eax,0xef9b9900
            { let (r, f) = add::<32>((rax as u32 as u64), 0xef9b9900u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // d4d0a cmp eax,0x6
            fl = sub::<32>((rax as u32 as u64), 0x6u64).1;
            // d4d0d ja 0xd4e6c
            if fl.a() { pc = 0xd4e6c; continue; }
            pc = 0xd4d13;
        }
        0xd4d13 => {
            // block d4d13
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d13 lea rcx,[rip+0x2ce] # 0xd4fe8
            rcx = m.env.code_va(0xd4fe8);
            // d4d1a movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // d4d1e add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // d4d21 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd4d23 => {
            // block d4d23
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d23 mov DWORD PTR [r14+0x94],0x2c
            m.w32(r14.wrapping_add(0x94), (0x2cu64) as u32);
            // d4d2e jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4d33 => {
            // block d4d33
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d33 cmp eax,0x10538200
            fl = sub::<32>((rax as u32 as u64), 0x10538200u64).1;
            // d4d38 jne 0xd4da9
            if fl.ne() { pc = 0xd4da9; continue; }
            pc = 0xd4d3a;
        }
        0xd4d3a => {
            // block d4d3a
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d3a mov DWORD PTR [r14+0x94],0x29
            m.w32(r14.wrapping_add(0x94), (0x29u64) as u32);
            // d4d45 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4d4a => {
            // block d4d4a
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d4a cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // d4d4f jne 0xd4dc4
            if fl.ne() { pc = 0xd4dc4; continue; }
            pc = 0xd4d51;
        }
        0xd4d51 => {
            // block d4d51
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d51 mov DWORD PTR [r14+0x94],0x33
            m.w32(r14.wrapping_add(0x94), (0x33u64) as u32);
            // d4d5c jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4d61 => {
            // block d4d61
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d61 cmp eax,0x10518450
            fl = sub::<32>((rax as u32 as u64), 0x10518450u64).1;
            // d4d66 jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4d6c;
        }
        0xd4d6c => {
            // block d4d6c
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d6c mov DWORD PTR [r14+0x94],0x28
            m.w32(r14.wrapping_add(0x94), (0x28u64) as u32);
            // d4d77 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4d7c => {
            // block d4d7c
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d7c cmp eax,0x104020ff
            fl = sub::<32>((rax as u32 as u64), 0x104020ffu64).1;
            // d4d81 jg 0xd4dfe
            if fl.g() { pc = 0xd4dfe; continue; }
            pc = 0xd4d83;
        }
        0xd4d83 => {
            // block d4d83
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d83 cmp eax,0x10322950
            fl = sub::<32>((rax as u32 as u64), 0x10322950u64).1;
            // d4d88 je 0xd4edf
            if fl.e() { pc = 0xd4edf; continue; }
            pc = 0xd4d8e;
        }
        0xd4d8e => {
            // block d4d8e
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d8e cmp eax,0x10352300
            fl = sub::<32>((rax as u32 as u64), 0x10352300u64).1;
            // d4d93 jne 0xd4eec
            if fl.ne() { pc = 0xd4eec; continue; }
            pc = 0xd4d99;
        }
        0xd4d99 => {
            // block d4d99
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4d99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4d99 mov DWORD PTR [r14+0x94],0x1f
            m.w32(r14.wrapping_add(0x94), (0x1fu64) as u32);
            // d4da4 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4da9 => {
            // block d4da9
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4da9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4da9 cmp eax,0x10565400
            fl = sub::<32>((rax as u32 as u64), 0x10565400u64).1;
            // d4dae jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4db4;
        }
        0xd4db4 => {
            // block d4db4
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4db4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4db4 mov DWORD PTR [r14+0x94],0x2a
            m.w32(r14.wrapping_add(0x94), (0x2au64) as u32);
            // d4dbf jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4dc4 => {
            // block d4dc4
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4dc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4dc4 cmp eax,0x10710851
            fl = sub::<32>((rax as u32 as u64), 0x10710851u64).1;
            // d4dc9 jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4dcf;
        }
        0xd4dcf => {
            // block d4dcf
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4dcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4dcf mov DWORD PTR [r14+0x94],0x34
            m.w32(r14.wrapping_add(0x94), (0x34u64) as u32);
            // d4dda jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4ddf => {
            // block d4ddf
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4ddf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4ddf cmp eax,0x102946ff
            fl = sub::<32>((rax as u32 as u64), 0x102946ffu64).1;
            // d4de4 jg 0xd4e1d
            if fl.g() { pc = 0xd4e1d; continue; }
            pc = 0xd4de6;
        }
        0xd4de6 => {
            // block d4de6
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4de6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4de6 cmp eax,0x10280300
            fl = sub::<32>((rax as u32 as u64), 0x10280300u64).1;
            // d4deb jne 0xd4f67
            if fl.ne() { pc = 0xd4f67; continue; }
            pc = 0xd4df1;
        }
        0xd4df1 => {
            // block d4df1
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4df1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4df1 mov DWORD PTR [r14+0x94],0x1c
            m.w32(r14.wrapping_add(0x94), (0x1cu64) as u32);
            // d4dfc jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4dfe => {
            // block d4dfe
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4dfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4dfe cmp eax,0x104782ff
            fl = sub::<32>((rax as u32 as u64), 0x104782ffu64).1;
            // d4e03 jg 0xd4e35
            if fl.g() { pc = 0xd4e35; continue; }
            pc = 0xd4e05;
        }
        0xd4e05 => {
            // block d4e05
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4e05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4e05 cmp eax,0x10402100
            fl = sub::<32>((rax as u32 as u64), 0x10402100u64).1;
            // d4e0a jne 0xd4f82
            if fl.ne() { pc = 0xd4f82; continue; }
            pc = 0xd4e10;
        }
        0xd4e10 => {
            // block d4e10
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4e10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4e10 mov DWORD PTR [r14+0x94],0x21
            m.w32(r14.wrapping_add(0x94), (0x21u64) as u32);
            // d4e1b jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4e1d => {
            // block d4e1d
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4e1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4e1d cmp eax,0x10294700
            fl = sub::<32>((rax as u32 as u64), 0x10294700u64).1;
            // d4e22 jne 0xd4f9d
            if fl.ne() { pc = 0xd4f9d; continue; }
            pc = 0xd4e28;
        }
        0xd4e28 => {
            // block d4e28
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4e28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4e28 mov DWORD PTR [r14+0x94],0x18
            m.w32(r14.wrapping_add(0x94), (0x18u64) as u32);
            // d4e33 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4e35 => {
            // block d4e35
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4e35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4e35 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // d4e3a jne 0xd4fb8
            if fl.ne() { pc = 0xd4fb8; continue; }
            pc = 0xd4e40;
        }
        0xd4e40 => {
            // block d4e40
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4e40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4e40 mov DWORD PTR [r14+0x94],0x25
            m.w32(r14.wrapping_add(0x94), (0x25u64) as u32);
            // d4e4b jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4e4d => {
            // block d4e4d
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4e4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4e4d mov DWORD PTR [r14+0x94],0x16
            m.w32(r14.wrapping_add(0x94), (0x16u64) as u32);
            // d4e58 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4e5a => {
            // block d4e5a
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4e5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4e5a cmp eax,0x10266401
            fl = sub::<32>((rax as u32 as u64), 0x10266401u64).1;
            // d4e5f jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4e61;
        }
        0xd4e61 => {
            // block d4e61
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4e61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4e61 mov DWORD PTR [r14+0x94],0x36
            m.w32(r14.wrapping_add(0x94), (0x36u64) as u32);
            pc = 0xd4e6c;
        }
        0xd4e6c => {
            // block d4e6c
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4e6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4e6c mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // d4e73 mov DWORD PTR [r14+0x8],0x454
            m.w32(r14.wrapping_add(0x8), (0x454u64) as u32);
            // d4e7b mov DWORD PTR [r14+0xa8],0x0
            m.w32(r14.wrapping_add(0xa8), (0x0u64) as u32);
            // d4e86 add r14,0x30
            { let (r, f) = add::<64>(r14, 0x30u64);
              r14 = r;
              fl = f; }
            // d4e8a mov rdi,r14
            rdi = r14;
            // d4e8d call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4e92 lea rbx,[rbp-0x28]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // d4e96 mov rdi,rbx
            rdi = rbx;
            // d4e99 mov rsi,r14
            rsi = r14;
            // d4e9c mov rdx,rax
            rdx = rax;
            // d4e9f call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4ea4 lea rsi,[rip+0x63e44] # 0x138cef
            rsi = m.env.data_va(316604);
            // d4eab mov rdi,rbx
            rdi = rbx;
            // d4eae call 0xebff6
            { let r = { let t = m.env.stub_va(0x7c3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4eb3 test BYTE PTR [rbp-0x28],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64 & 0x1u64);
            // d4eb7 jne 0xd4ebf
            if fl.ne() { pc = 0xd4ebf; continue; }
            pc = 0xd4eb9;
        }
        0xd4eb9 => {
            // block d4eb9
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4eb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4eb9 lea rsi,[rbp-0x27]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd9);
            // d4ebd jmp 0xd4ec3
            pc = 0xd4ec3; continue;
        }
        0xd4ebf => {
            // block d4ebf
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4ebf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4ebf mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            pc = 0xd4ec3;
        }
        0xd4ec3 => {
            // block d4ec3
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4ec3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4ec3 mov rdi,r14
            rdi = r14;
            // d4ec6 call 0xec2b4
            { let r = { let t = m.env.stub_va(0xa81); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4ecb lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // d4ecf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4ed4 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd4ed6;
        }
        0xd4ed6 => {
            // block d4ed6
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4ed6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4ed6 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // d4eda pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4edb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4edd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4ede ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd4edf => {
            // block d4edf
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4edf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4edf mov DWORD PTR [r14+0x94],0x1e
            m.w32(r14.wrapping_add(0x94), (0x1eu64) as u32);
            // d4eea jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4eec => {
            // block d4eec
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4eec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4eec cmp eax,0x10378800
            fl = sub::<32>((rax as u32 as u64), 0x10378800u64).1;
            // d4ef1 jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4ef7;
        }
        0xd4ef7 => {
            // block d4ef7
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4ef7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4ef7 mov DWORD PTR [r14+0x94],0x20
            m.w32(r14.wrapping_add(0x94), (0x20u64) as u32);
            // d4f02 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4f07 => {
            // block d4f07
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f07 mov DWORD PTR [r14+0x94],0x2d
            m.w32(r14.wrapping_add(0x94), (0x2du64) as u32);
            // d4f12 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4f17 => {
            // block d4f17
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f17 mov DWORD PTR [r14+0x94],0x2e
            m.w32(r14.wrapping_add(0x94), (0x2eu64) as u32);
            // d4f22 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4f27 => {
            // block d4f27
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f27 mov DWORD PTR [r14+0x94],0x2f
            m.w32(r14.wrapping_add(0x94), (0x2fu64) as u32);
            // d4f32 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4f37 => {
            // block d4f37
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f37 mov DWORD PTR [r14+0x94],0x30
            m.w32(r14.wrapping_add(0x94), (0x30u64) as u32);
            // d4f42 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4f47 => {
            // block d4f47
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f47 mov DWORD PTR [r14+0x94],0x31
            m.w32(r14.wrapping_add(0x94), (0x31u64) as u32);
            // d4f52 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4f57 => {
            // block d4f57
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f57 mov DWORD PTR [r14+0x94],0x35
            m.w32(r14.wrapping_add(0x94), (0x35u64) as u32);
            // d4f62 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4f67 => {
            // block d4f67
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f67 cmp eax,0x10293000
            fl = sub::<32>((rax as u32 as u64), 0x10293000u64).1;
            // d4f6c jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4f72;
        }
        0xd4f72 => {
            // block d4f72
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f72 mov DWORD PTR [r14+0x94],0x17
            m.w32(r14.wrapping_add(0x94), (0x17u64) as u32);
            // d4f7d jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4f82 => {
            // block d4f82
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f82 cmp eax,0x10416000
            fl = sub::<32>((rax as u32 as u64), 0x10416000u64).1;
            // d4f87 jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4f8d;
        }
        0xd4f8d => {
            // block d4f8d
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f8d mov DWORD PTR [r14+0x94],0x23
            m.w32(r14.wrapping_add(0x94), (0x23u64) as u32);
            // d4f98 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4f9d => {
            // block d4f9d
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4f9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4f9d cmp eax,0x10294900
            fl = sub::<32>((rax as u32 as u64), 0x10294900u64).1;
            // d4fa2 jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4fa8;
        }
        0xd4fa8 => {
            // block d4fa8
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4fa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4fa8 mov DWORD PTR [r14+0x94],0x1d
            m.w32(r14.wrapping_add(0x94), (0x1du64) as u32);
            // d4fb3 jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4fb8 => {
            // block d4fb8
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4fb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4fb8 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // d4fbd jne 0xd4e6c
            if fl.ne() { pc = 0xd4e6c; continue; }
            pc = 0xd4fc3;
        }
        0xd4fc3 => {
            // block d4fc3
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4fc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4fc3 mov DWORD PTR [r14+0x94],0x26
            m.w32(r14.wrapping_add(0x94), (0x26u64) as u32);
            // d4fce jmp 0xd4e6c
            pc = 0xd4e6c; continue;
        }
        0xd4fd3 => {
            // block d4fd3
            #[cfg(feature = "trace")] eprintln!("f_000d4bd2 pc d4fd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4fd3 mov rbx,rax
            rbx = rax;
            // d4fd6 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // d4fda call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4fdf mov rdi,rbx
            rdi = rbx;
            // d4fe2 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb87c4 size=1400
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b87c4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb87c4;
    loop {
        match pc {
        0xb87c4 => {
            // block b87c4
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b87c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b87c4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b87c5 mov rbp,rsp
            rbp = rsp;
            // b87c8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b87ca push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b87cc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b87cd sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b87d1 mov r14,rdx
            r14 = rdx;
            // b87d4 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b87d6 mov r15,rdi
            r15 = rdi;
            // b87d9 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // b87e0 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // b87e7 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // b87ee mov DWORD PTR [rbp-0x28],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0xffffffffu64) as u32);
            // b87f5 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // b87fc mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // b8803 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8806 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b880a call QWORD PTR [rax+0x1d78]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d78)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8810 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b8812 je 0xb883e
            if fl.e() { pc = 0xb883e; continue; }
            pc = 0xb8814;
        }
        0xb8814 => {
            // block b8814
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8814 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8814 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8817 mov esi,DWORD PTR [rbp-0x30]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // b881a mov ecx,DWORD PTR [rbp-0x28]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // b881d mov r8d,DWORD PTR [rbp-0x24]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // b8821 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // b8825 mov rdi,r15
            rdi = r15;
            // b8828 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b882b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b882d je 0xb8845
            if fl.e() { pc = 0xb8845; continue; }
            pc = 0xb882f;
        }
        0xb882f => {
            // block b882f
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b882f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b882f cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // b8833 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // b8836 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b8839 jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb883e => {
            // block b883e
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b883e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b883e mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // b8840 cmp ebx,0x2b
            fl = sub::<32>((rbx as u32 as u64), 0x2bu64).1;
            // b8843 jbe 0xb884c
            if fl.be() { pc = 0xb884c; continue; }
            pc = 0xb8845;
        }
        0xb8845 => {
            // block b8845
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8845 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8845 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b8847 jmp 0xb8c81
            pc = 0xb8c81; continue;
        }
        0xb884c => {
            // block b884c
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b884c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b884c lea rcx,[rip+0x439] # 0xb8c8c
            rcx = m.env.code_va(0xb8c8c);
            // b8853 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // b8857 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // b885a jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xb885c => {
            // block b885c
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b885c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b885c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b885f mov rdi,r15
            rdi = r15;
            // b8862 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8868 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b886a call 0xce42f
            { let r = { let t = m.env.code_va(0xce42f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b886f jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb8874 => {
            // block b8874
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8874 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8874 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8877 mov rdi,r15
            rdi = r15;
            // b887a call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8880 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8882 call 0xce568
            { let r = { let t = m.env.code_va(0xce568); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8887 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb888c => {
            // block b888c
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b888c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b888c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b888f mov rdi,r15
            rdi = r15;
            // b8892 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8898 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b889a call 0xce5e7
            { let r = { let t = m.env.code_va(0xce5e7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b889f jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb88a4 => {
            // block b88a4
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b88a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b88a4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b88a7 mov rdi,r15
            rdi = r15;
            // b88aa call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b88b0 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b88b2 call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b88b7 jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb88bc => {
            // block b88bc
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b88bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b88bc mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b88bf mov rdi,r15
            rdi = r15;
            // b88c2 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b88c8 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b88ca call 0xce76c
            { let r = { let t = m.env.code_va(0xce76c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b88cf jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb88d4 => {
            // block b88d4
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b88d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b88d4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b88d7 mov rdi,r15
            rdi = r15;
            // b88da call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b88e0 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b88e2 call 0xce908
            { let r = { let t = m.env.code_va(0xce908); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b88e7 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb88ec => {
            // block b88ec
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b88ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b88ec mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b88ef mov rdi,r15
            rdi = r15;
            // b88f2 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b88f8 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b88fa call 0xceac8
            { let r = { let t = m.env.code_va(0xceac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b88ff jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb8904 => {
            // block b8904
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8904 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8904 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8907 mov rdi,r15
            rdi = r15;
            // b890a call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8910 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8912 call 0xcecbc
            { let r = { let t = m.env.code_va(0xcecbc); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8917 jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb891c => {
            // block b891c
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b891c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b891c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b891f mov rdi,r15
            rdi = r15;
            // b8922 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8928 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b892a call 0xcee78
            { let r = { let t = m.env.code_va(0xcee78); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b892f jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb8934 => {
            // block b8934
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8934 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8934 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8937 mov rdi,r15
            rdi = r15;
            // b893a call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8940 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8942 call 0xcf034
            { let r = { let t = m.env.code_va(0xcf034); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8947 jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb894c => {
            // block b894c
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b894c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b894c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b894f mov rdi,r15
            rdi = r15;
            // b8952 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8958 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b895a call 0xcf200
            { let r = { let t = m.env.code_va(0xcf200); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b895f jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb8964 => {
            // block b8964
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8964 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8964 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8967 mov rdi,r15
            rdi = r15;
            // b896a call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8970 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8972 call 0xcf3bc
            { let r = { let t = m.env.code_va(0xcf3bc); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8977 jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb897c => {
            // block b897c
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b897c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b897c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b897f mov rdi,r15
            rdi = r15;
            // b8982 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8988 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b898a call 0xcf4f1
            { let r = { let t = m.env.code_va(0xcf4f1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b898f jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8994 => {
            // block b8994
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8994 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8997 mov rdi,r15
            rdi = r15;
            // b899a call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b89a0 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b89a2 call 0xcf556
            { let r = { let t = m.env.code_va(0xcf556); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b89a7 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb89ac => {
            // block b89ac
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b89ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b89ac mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b89af mov rdi,r15
            rdi = r15;
            // b89b2 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b89b8 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b89ba call 0xcf5fd
            { let r = { let t = m.env.code_va(0xcf5fd); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b89bf jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb89c4 => {
            // block b89c4
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b89c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b89c4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b89c7 mov rdi,r15
            rdi = r15;
            // b89ca call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b89d0 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b89d2 call 0xcf682
            { let r = { let t = m.env.code_va(0xcf682); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b89d7 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb89dc => {
            // block b89dc
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b89dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b89dc mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b89df mov rdi,r15
            rdi = r15;
            // b89e2 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b89e8 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b89ea call 0xcf6db
            { let r = { let t = m.env.code_va(0xcf6db); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b89ef jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb89f4 => {
            // block b89f4
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b89f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b89f4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b89f7 mov rdi,r15
            rdi = r15;
            // b89fa call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a00 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8a02 call 0xcf778
            { let r = { let t = m.env.code_va(0xcf778); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a07 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8a0c => {
            // block b8a0c
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8a0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8a0c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8a0f mov rdi,r15
            rdi = r15;
            // b8a12 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a18 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8a1a call 0xcf7a1
            { let r = { let t = m.env.code_va(0xcf7a1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a1f jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8a24 => {
            // block b8a24
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8a24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8a24 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8a27 mov rdi,r15
            rdi = r15;
            // b8a2a call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a30 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8a32 call 0xcf820
            { let r = { let t = m.env.code_va(0xcf820); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a37 movzx ebx,ax
            rbx = ((rax as u16 as u64)) as u32 as u64;
            // b8a3a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8a3d mov rdi,r15
            rdi = r15;
            // b8a40 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a46 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b8a49 add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // b8a4b jmp 0xb8c7c
            pc = 0xb8c7c; continue;
        }
        0xb8a50 => {
            // block b8a50
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8a50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8a50 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8a53 mov rdi,r15
            rdi = r15;
            // b8a56 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a5c mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8a5e call 0xcf9e8
            { let r = { let t = m.env.code_va(0xcf9e8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a63 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8a68 => {
            // block b8a68
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8a68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8a68 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8a6b mov rdi,r15
            rdi = r15;
            // b8a6e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a74 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8a76 call 0xcfa3b
            { let r = { let t = m.env.code_va(0xcfa3b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a7b jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8a80 => {
            // block b8a80
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8a80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8a80 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8a83 mov rdi,r15
            rdi = r15;
            // b8a86 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a8c mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8a8e call 0xcfbe8
            { let r = { let t = m.env.code_va(0xcfbe8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8a93 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8a98 => {
            // block b8a98
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8a98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8a98 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8a9b mov rdi,r15
            rdi = r15;
            // b8a9e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8aa4 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8aa6 call 0xcfc55
            { let r = { let t = m.env.code_va(0xcfc55); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8aab jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8ab0 => {
            // block b8ab0
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8ab0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8ab0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8ab3 mov rdi,r15
            rdi = r15;
            // b8ab6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8abc mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8abe call 0xcfde4
            { let r = { let t = m.env.code_va(0xcfde4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8ac3 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8ac8 => {
            // block b8ac8
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8ac8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8ac8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8acb mov rdi,r15
            rdi = r15;
            // b8ace call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8ad4 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8ad6 call 0xcff64
            { let r = { let t = m.env.code_va(0xcff64); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8adb jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8ae0 => {
            // block b8ae0
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8ae0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8ae3 mov rdi,r15
            rdi = r15;
            // b8ae6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8aec mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8aee call 0xd0134
            { let r = { let t = m.env.code_va(0xd0134); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8af3 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8af8 => {
            // block b8af8
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8af8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8af8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8afb mov rdi,r15
            rdi = r15;
            // b8afe call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b04 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8b06 call 0xd013c
            { let r = { let t = m.env.code_va(0xd013c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b0b jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8b10 => {
            // block b8b10
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8b10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8b10 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8b13 mov rdi,r15
            rdi = r15;
            // b8b16 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b1c mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8b1e call 0xd01a9
            { let r = { let t = m.env.code_va(0xd01a9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b23 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8b28 => {
            // block b8b28
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8b28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8b28 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8b2b mov rdi,r15
            rdi = r15;
            // b8b2e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b34 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8b36 call 0xd02ac
            { let r = { let t = m.env.code_va(0xd02ac); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b3b jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8b40 => {
            // block b8b40
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8b40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8b40 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8b43 mov rdi,r15
            rdi = r15;
            // b8b46 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b4c mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8b4e call 0xd0319
            { let r = { let t = m.env.code_va(0xd0319); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b53 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8b58 => {
            // block b8b58
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8b58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8b58 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8b5b mov rdi,r15
            rdi = r15;
            // b8b5e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b64 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8b66 call 0xd041c
            { let r = { let t = m.env.code_va(0xd041c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b6b jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8b70 => {
            // block b8b70
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8b70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8b70 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8b73 mov rdi,r15
            rdi = r15;
            // b8b76 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b7c mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8b7e call 0xd0489
            { let r = { let t = m.env.code_va(0xd0489); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b83 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8b88 => {
            // block b8b88
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8b88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8b88 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8b8b mov rdi,r15
            rdi = r15;
            // b8b8e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b94 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8b96 call 0xd05f0
            { let r = { let t = m.env.code_va(0xd05f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8b9b jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8ba0 => {
            // block b8ba0
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8ba0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8ba0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8ba3 mov rdi,r15
            rdi = r15;
            // b8ba6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8bac mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8bae call 0xd0758
            { let r = { let t = m.env.code_va(0xd0758); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8bb3 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8bb8 => {
            // block b8bb8
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8bb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8bb8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8bbb mov rdi,r15
            rdi = r15;
            // b8bbe call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8bc4 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8bc6 call 0xd07d7
            { let r = { let t = m.env.code_va(0xd07d7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8bcb jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8bd0 => {
            // block b8bd0
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8bd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8bd0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8bd3 mov rdi,r15
            rdi = r15;
            // b8bd6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8bdc mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8bde call 0xd0960
            { let r = { let t = m.env.code_va(0xd0960); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8be3 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8be8 => {
            // block b8be8
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8be8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8be8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8beb mov rdi,r15
            rdi = r15;
            // b8bee call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8bf4 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8bf6 call 0xd0a75
            { let r = { let t = m.env.code_va(0xd0a75); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8bfb jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8bfd => {
            // block b8bfd
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8bfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8bfd mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8c00 mov rdi,r15
            rdi = r15;
            // b8c03 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c09 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8c0b call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c10 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8c12 => {
            // block b8c12
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8c12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8c12 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8c15 mov rdi,r15
            rdi = r15;
            // b8c18 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c1e mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8c20 call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c25 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8c27 => {
            // block b8c27
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8c27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8c27 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8c2a mov rdi,r15
            rdi = r15;
            // b8c2d call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c33 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8c35 call 0xd0e28
            { let r = { let t = m.env.code_va(0xd0e28); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c3a jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8c3c => {
            // block b8c3c
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8c3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8c3c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8c3f mov rdi,r15
            rdi = r15;
            // b8c42 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c48 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8c4a call 0xd0e8c
            { let r = { let t = m.env.code_va(0xd0e8c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c4f jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8c51 => {
            // block b8c51
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8c51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8c51 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8c54 mov rdi,r15
            rdi = r15;
            // b8c57 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c5d mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8c5f call 0xd0ea0
            { let r = { let t = m.env.code_va(0xd0ea0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c64 jmp 0xb8c79
            pc = 0xb8c79; continue;
        }
        0xb8c66 => {
            // block b8c66
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8c66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8c66 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // b8c69 mov rdi,r15
            rdi = r15;
            // b8c6c call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8c72 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8c74 call 0xd0fb9
            { let r = { let t = m.env.code_va(0xd0fb9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb8c79;
        }
        0xb8c79 => {
            // block b8c79
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8c79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8c79 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            pc = 0xb8c7c;
        }
        0xb8c7c => {
            // block b8c7c
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8c7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8c7c mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // b8c7f mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xb8c81;
        }
        0xb8c81 => {
            // block b8c81
            #[cfg(feature = "trace")] eprintln!("f_000b87c4 pc b8c81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8c81 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b8c85 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8c86 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8c88 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b8c8a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8c8b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
