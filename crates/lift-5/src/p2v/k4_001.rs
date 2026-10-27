//! Mechanically lifted functions (scripts/lift_run.py), module k4_001. Each one is checked against the original
//! in the world harness (tests/world_k4_001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xd35ce size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
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
            if crate::cov::br(fl.e()) { pc = 0xd3662; continue; }
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
            { m.outregs(0xd3637, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3637 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d363a lea rsi,[rbp-0x1c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // d3641 mov rdi,r15
            rdi = r15;
            // d3644 call QWORD PTR [rax+0x168]
            { m.outregs(0xd364a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d364a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d364c je 0xd3662
            if crate::cov::br(fl.e()) { pc = 0xd3662; continue; }
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
            { m.outregs(0xd365e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if crate::cov::br(fl.ne()) { pc = 0xd367a; continue; }
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
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe8310 size=380
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000e8310<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe8310;
    loop {
        match pc {
        0xe8310 => {
            // block e8310
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8310 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8310 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e8311 mov rbp,rsp
            rbp = rsp;
            // e8314 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e8316 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e8317 sub rsp,0x340
            { let (r, f) = sub::<64>(rsp, 0x340u64);
              rsp = r;
              fl = f; }
            // e831e mov rbx,rdi
            rbx = rdi;
            // e8321 mov r14,QWORD PTR [rip+0x580b0] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // e8328 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // e832b mov QWORD PTR [rbp-0x18],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (r14) as u64);
            // e832f mov WORD PTR [rbp-0x312],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_fcee), (0x0u64) as u16);
            // e8338 xorps xmm0,xmm0
            x0 = 0;
            // e833b movaps XMMWORD PTR [rbp-0x330],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fcd0), x0);
            // e8342 mov QWORD PTR [rbp-0x320],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fce0), (0x0u64) as u64);
            // e834d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e8350 mov rax,QWORD PTR [rax+0x50]
            rax = m.r64(rax.wrapping_add(0x50)) as u64;
            // e8354 lea rsi,[rbp-0x330]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcd0);
            // e835b lea rcx,[rbp-0x312]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_fcee);
            // e8362 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e8364 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e8367 call rax
            { m.outregs(0xe8369, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8369 mov rax,QWORD PTR [rbp-0x328]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8)) as u64;
            // e8370 cmp rax,QWORD PTR [rbp-0x330]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64).1;
            // e8377 je 0xe8401
            if crate::cov::br(fl.e()) { pc = 0xe8401; continue; }
            pc = 0xe837d;
        }
        0xe837d => {
            // block e837d
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e837d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e837d lea rdi,[rbp-0x310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // e8384 call 0x12b8c
            { m.outregs(0xe8389, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x12b8c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8389 mov rdx,QWORD PTR [rbp-0x330]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // e8390 lea rdi,[rbp-0x348]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb8);
            // e8397 lea rsi,[rbp-0x310]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // e839e call 0x12e88
            { m.outregs(0xe83a3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x12e88); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83a3 lea rdi,[rbp-0x348]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcb8);
            // e83aa call 0xec032
            { m.outregs(0xe83af, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83af lea rdi,[rbx+0x2c0]
            rdi = rbx.wrapping_add(0x2c0);
            // e83b6 lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // e83bd call 0xec038
            { m.outregs(0xe83c2, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83c2 lea rdi,[rbx+0x2f0]
            rdi = rbx.wrapping_add(0x2f0);
            // e83c9 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e83cd call 0xec038
            { m.outregs(0xe83d2, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83d2 lea rdi,[rbx+0x308]
            rdi = rbx.wrapping_add(0x308);
            // e83d9 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e83dd call 0xec038
            { m.outregs(0xe83e2, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83e2 lea rdi,[rbx+0x2d8]
            rdi = rbx.wrapping_add(0x2d8);
            // e83e9 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // e83f0 call 0xec038
            { m.outregs(0xe83f5, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e83f5 lea rdi,[rbp-0x310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // e83fc call 0x14c0c
            { m.outregs(0xe8401, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x14c0c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe8401;
        }
        0xe8401 => {
            // block e8401
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8401 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8401 mov rsi,QWORD PTR [rbx+0x268]
            rsi = m.r64(rbx.wrapping_add(0x268)) as u64;
            // e8408 add rbx,0x2a8
            { let (r, f) = add::<64>(rbx, 0x2a8u64);
              rbx = r;
              fl = f; }
            // e840f mov rdi,rbx
            rdi = rbx;
            // e8412 call 0xec038
            { m.outregs(0xe8417, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8417 mov rdi,QWORD PTR [rbp-0x330]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // e841e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e8421 je 0xe8438
            if crate::cov::br(fl.e()) { pc = 0xe8438; continue; }
            pc = 0xe8423;
        }
        0xe8423 => {
            // block e8423
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8423 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8423 cmp QWORD PTR [rbp-0x328],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8)) as u64, rdi).1;
            // e842a je 0xe8433
            if crate::cov::br(fl.e()) { pc = 0xe8433; continue; }
            pc = 0xe842c;
        }
        0xe842c => {
            // block e842c
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e842c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e842c mov QWORD PTR [rbp-0x328],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8), (rdi) as u64);
            pc = 0xe8433;
        }
        0xe8433 => {
            // block e8433
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8433 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8433 call 0xec13a
            { m.outregs(0xe8438, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe8438;
        }
        0xe8438 => {
            // block e8438
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8438 cmp r14,QWORD PTR [rbp-0x18]
            fl = sub::<64>(r14, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64).1;
            // e843c jne 0xe8487
            if crate::cov::br(fl.ne()) { pc = 0xe8487; continue; }
            pc = 0xe843e;
        }
        0xe843e => {
            // block e843e
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e843e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e843e add rsp,0x340
            { let (r, f) = add::<64>(rsp, 0x340u64);
              rsp = r;
              fl = f; }
            // e8445 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e8446 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e8448 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e8449 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe844a => {
            // block e844a
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e844a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e844a mov rbx,rax
            rbx = rax;
            // e844d lea rdi,[rbp-0x310]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fcf0);
            // e8454 call 0x14c0c
            { let r = { let t = m.env.code_va(0x14c0c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8459 jmp 0xe845e
            pc = 0xe845e; continue;
        }
        0xe845b => {
            // block e845b
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e845b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e845b mov rbx,rax
            rbx = rax;
            pc = 0xe845e;
        }
        0xe845e => {
            // block e845e
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e845e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e845e mov rdi,QWORD PTR [rbp-0x330]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd0)) as u64;
            // e8465 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e8468 je 0xe847f
            if crate::cov::br(fl.e()) { pc = 0xe847f; continue; }
            pc = 0xe846a;
        }
        0xe846a => {
            // block e846a
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e846a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e846a cmp QWORD PTR [rbp-0x328],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8)) as u64, rdi).1;
            // e8471 je 0xe847a
            if crate::cov::br(fl.e()) { pc = 0xe847a; continue; }
            pc = 0xe8473;
        }
        0xe8473 => {
            // block e8473
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8473 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8473 mov QWORD PTR [rbp-0x328],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fcd8), (rdi) as u64);
            pc = 0xe847a;
        }
        0xe847a => {
            // block e847a
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e847a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e847a call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe847f;
        }
        0xe847f => {
            // block e847f
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e847f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e847f mov rdi,rbx
            rdi = rbx;
            // e8482 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe8487;
        }
        0xe8487 => {
            // block e8487
            #[cfg(feature = "trace")] eprintln!("f_000e8310 pc e8487 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8487 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x120e8 size=788
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000120e8_rec)
pub fn f_000120e8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000120e8_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000120e8_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0x120e8;
    loop {
        match pc {
        0x120e8 => {
            // block 120e8
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 120e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120e8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 120e9 mov rbp,rsp
            rbp = rsp;
            // 120ec push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 120ee push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 120f0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 120f2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 120f4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 120f5 sub rsp,0x88
            { let (r, f) = sub::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // 120fc mov r10,r9
            r10 = r9;
            // 120ff mov QWORD PTR [rbp-0x70],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rcx) as u64);
            // 12103 mov r15,rsi
            r15 = rsi;
            // 12106 mov rbx,rdi
            rbx = rdi;
            // 12109 test r10,r10
            fl = logic::<64>(r10 & r10);
            // 1210c je 0x123d6
            if crate::cov::br(fl.e()) { pc = 0x123d6; continue; }
            pc = 0x12112;
        }
        0x12112 => {
            // block 12112
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12112 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12112 mov rax,QWORD PTR [rbp+0x18]
            rax = m.r64(rbp.wrapping_add(0x18)) as u64;
            // 12116 mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 1211a mov rax,QWORD PTR [rbp+0x10]
            rax = m.r64(rbp.wrapping_add(0x10)) as u64;
            // 1211e mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 12122 movabs r9,0x6db6db6db6db6db7
            r9 = 0x6db6_db6d_b6db_6db7u64;
            pc = 0x1212c;
        }
        0x1212c => {
            // block 1212c
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1212c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1212c mov QWORD PTR [rbp-0x78],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rdx) as u64);
            // 12130 mov r13,r8
            r13 = r8;
            // 12133 mov r14,r15
            r14 = r15;
            pc = 0x12136;
        }
        0x12136 => {
            // block 12136
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12136 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12136 cmp r10,QWORD PTR [rbp-0x68]
            fl = sub::<64>(r10, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 1213a jle 0x12384
            if crate::cov::br(fl.le()) { pc = 0x12384; continue; }
            pc = 0x12140;
        }
        0x12140 => {
            // block 12140
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12140 cmp r13,QWORD PTR [rbp-0x68]
            fl = sub::<64>(r13, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 12144 jle 0x12384
            if crate::cov::br(fl.le()) { pc = 0x12384; continue; }
            pc = 0x1214a;
        }
        0x1214a => {
            // block 1214a
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1214a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1214a test r13,r13
            fl = logic::<64>(r13 & r13);
            // 1214d je 0x123d6
            if crate::cov::br(fl.e()) { pc = 0x123d6; continue; }
            pc = 0x12153;
        }
        0x12153 => {
            // block 12153
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12153 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12153 mov eax,DWORD PTR [r14+0x18]
            rax = (m.r32(r14.wrapping_add(0x18)) as u64) as u32 as u64;
            // 12157 neg r13
            { let (r, f) = neg::<64>(r13); r13 = r; fl = f; }
            // 1215a mov r12,r13
            r12 = r13;
            pc = 0x1215d;
        }
        0x1215d => {
            // block 1215d
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1215d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1215d cmp eax,DWORD PTR [rbx+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbx.wrapping_add(0x18)) as u64).1;
            // 12160 jl 0x12170
            if crate::cov::br(fl.l()) { pc = 0x12170; continue; }
            pc = 0x12162;
        }
        0x12162 => {
            // block 12162
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12162 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12162 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12166 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 12169 jne 0x1215d
            if crate::cov::br(fl.ne()) { pc = 0x1215d; continue; }
            pc = 0x1216b;
        }
        0x1216b => {
            // block 1216b
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1216b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1216b jmp 0x123d6
            pc = 0x123d6; continue;
        }
        0x12170 => {
            // block 12170
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12170 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12170 mov rax,r12
            rax = r12;
            // 12173 neg rax
            { let (r, f) = neg::<64>(rax); rax = r; fl = f; }
            // 12176 cmp rax,r10
            fl = sub::<64>(rax, r10).1;
            // 12179 jge 0x121a8
            if crate::cov::br(fl.ge()) { pc = 0x121a8; continue; }
            pc = 0x1217b;
        }
        0x1217b => {
            // block 1217b
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1217b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1217b mov r11,r10
            r11 = r10;
            // 1217e shr r11,0x3f
            { let (r, f) = shr::<64>(r11, 0x3fu64, fl); r11 = r; fl = f; }
            // 12182 add r11,r10
            { let (r, f) = add::<64>(r11, r10);
              r11 = r;
              fl = f; }
            // 12185 sar r11,1
            { let (r, f) = sar::<64>(r11, 0x1u64, fl); r11 = r; fl = f; }
            // 12188 imul rax,r11,0x38
            { let (r, f) = imul::<64>(r11, 0x38u64); rax = r; fl = f; }
            // 1218c lea r13,[r14+rax*1]
            r13 = r14.wrapping_add(rax);
            // 12190 mov rdi,r14
            rdi = r14;
            // 12193 sub rdi,rbx
            { let (r, f) = sub::<64>(rdi, rbx);
              rdi = r;
              fl = f; }
            // 12196 sar rdi,0x3
            { let (r, f) = sar::<64>(rdi, 0x3u64, fl); rdi = r; fl = f; }
            // 1219a imul rdi,r9
            { let (r, f) = imul::<64>(rdi, r9); rdi = r; fl = f; }
            // 1219e mov eax,DWORD PTR [r14+rax*1+0x18]
            rax = (m.r32(r14.wrapping_add(rax).wrapping_add(0x18)) as u64) as u32 as u64;
            // 121a3 mov r15,rbx
            r15 = rbx;
            // 121a6 jmp 0x121ec
            pc = 0x121ec; continue;
        }
        0x121a8 => {
            // block 121a8
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 121a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 121a8 cmp r12,0xffffffffffffffff
            fl = sub::<64>(r12, 0xffff_ffff_ffff_ffffu64).1;
            // 121ac je 0x123aa
            if crate::cov::br(fl.e()) { pc = 0x123aa; continue; }
            pc = 0x121b2;
        }
        0x121b2 => {
            // block 121b2
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 121b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 121b2 mov r8,rax
            r8 = rax;
            // 121b5 shr r8,0x3f
            { let (r, f) = shr::<64>(r8, 0x3fu64, fl); r8 = r; fl = f; }
            // 121b9 add r8,rax
            { let (r, f) = add::<64>(r8, rax);
              r8 = r;
              fl = f; }
            // 121bc sar r8,1
            { let (r, f) = sar::<64>(r8, 0x1u64, fl); r8 = r; fl = f; }
            // 121bf imul rax,r8,0x38
            { let (r, f) = imul::<64>(r8, 0x38u64); rax = r; fl = f; }
            // 121c3 lea r15,[rbx+rax*1]
            r15 = rbx.wrapping_add(rax);
            // 121c7 mov rdi,rdx
            rdi = rdx;
            // 121ca sub rdi,r14
            { let (r, f) = sub::<64>(rdi, r14);
              rdi = r;
              fl = f; }
            // 121cd sar rdi,0x3
            { let (r, f) = sar::<64>(rdi, 0x3u64, fl); rdi = r; fl = f; }
            // 121d1 imul rdi,r9
            { let (r, f) = imul::<64>(rdi, r9); rdi = r; fl = f; }
            // 121d5 mov eax,DWORD PTR [rbx+rax*1+0x18]
            rax = (m.r32(rbx.wrapping_add(rax).wrapping_add(0x18)) as u64) as u32 as u64;
            // 121d9 mov r13,r14
            r13 = r14;
            // 121dc jmp 0x1222c
            pc = 0x1222c; continue;
        }
        0x121de => {
            // block 121de
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 121de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 121de lea r15,[rsi+r15*1+0x38]
            r15 = rsi.wrapping_add(r15).wrapping_add(0x38);
            // 121e3 dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // 121e6 sub rcx,rdi
            { let (r, f) = sub::<64>(rcx, rdi);
              rcx = r;
              fl = f; }
            // 121e9 mov rdi,rcx
            rdi = rcx;
            pc = 0x121ec;
        }
        0x121ec => {
            // block 121ec
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 121ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 121ec mov rcx,rdi
            rcx = rdi;
            // 121ef test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 121f2 je 0x1220e
            if crate::cov::br(fl.e()) { pc = 0x1220e; continue; }
            pc = 0x121f4;
        }
        0x121f4 => {
            // block 121f4
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 121f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 121f4 mov rdi,rcx
            rdi = rcx;
            // 121f7 shr rdi,0x3f
            { let (r, f) = shr::<64>(rdi, 0x3fu64, fl); rdi = r; fl = f; }
            // 121fb add rdi,rcx
            { let (r, f) = add::<64>(rdi, rcx);
              rdi = r;
              fl = f; }
            // 121fe sar rdi,1
            { let (r, f) = sar::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // 12201 imul rsi,rdi,0x38
            { let (r, f) = imul::<64>(rdi, 0x38u64); rsi = r; fl = f; }
            // 12205 cmp eax,DWORD PTR [r15+rsi*1+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(r15.wrapping_add(rsi).wrapping_add(0x18)) as u64).1;
            // 1220a jl 0x121ec
            if crate::cov::br(fl.l()) { pc = 0x121ec; continue; }
            pc = 0x1220c;
        }
        0x1220c => {
            // block 1220c
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1220c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1220c jmp 0x121de
            pc = 0x121de; continue;
        }
        0x1220e => {
            // block 1220e
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1220e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1220e mov r8,r15
            r8 = r15;
            // 12211 sub r8,rbx
            { let (r, f) = sub::<64>(r8, rbx);
              r8 = r;
              fl = f; }
            // 12214 sar r8,0x3
            { let (r, f) = sar::<64>(r8, 0x3u64, fl); r8 = r; fl = f; }
            // 12218 imul r8,r9
            { let (r, f) = imul::<64>(r8, r9); r8 = r; fl = f; }
            // 1221c jmp 0x1225c
            pc = 0x1225c; continue;
        }
        0x1221e => {
            // block 1221e
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1221e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1221e lea r13,[rsi+r13*1+0x38]
            r13 = rsi.wrapping_add(r13).wrapping_add(0x38);
            // 12223 dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // 12226 sub rcx,rdi
            { let (r, f) = sub::<64>(rcx, rdi);
              rcx = r;
              fl = f; }
            // 12229 mov rdi,rcx
            rdi = rcx;
            pc = 0x1222c;
        }
        0x1222c => {
            // block 1222c
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1222c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1222c mov rcx,rdi
            rcx = rdi;
            // 1222f test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 12232 je 0x1224e
            if crate::cov::br(fl.e()) { pc = 0x1224e; continue; }
            pc = 0x12234;
        }
        0x12234 => {
            // block 12234
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12234 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12234 mov rdi,rcx
            rdi = rcx;
            // 12237 shr rdi,0x3f
            { let (r, f) = shr::<64>(rdi, 0x3fu64, fl); rdi = r; fl = f; }
            // 1223b add rdi,rcx
            { let (r, f) = add::<64>(rdi, rcx);
              rdi = r;
              fl = f; }
            // 1223e sar rdi,1
            { let (r, f) = sar::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // 12241 imul rsi,rdi,0x38
            { let (r, f) = imul::<64>(rdi, 0x38u64); rsi = r; fl = f; }
            // 12245 cmp DWORD PTR [r13+rsi*1+0x18],eax
            fl = sub::<32>(m.r32(r13.wrapping_add(rsi).wrapping_add(0x18)) as u64, (rax as u32 as u64)).1;
            // 1224a jge 0x1222c
            if crate::cov::br(fl.ge()) { pc = 0x1222c; continue; }
            pc = 0x1224c;
        }
        0x1224c => {
            // block 1224c
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1224c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1224c jmp 0x1221e
            pc = 0x1221e; continue;
        }
        0x1224e => {
            // block 1224e
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1224e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1224e mov r11,r13
            r11 = r13;
            // 12251 sub r11,r14
            { let (r, f) = sub::<64>(r11, r14);
              r11 = r;
              fl = f; }
            // 12254 sar r11,0x3
            { let (r, f) = sar::<64>(r11, 0x3u64, fl); r11 = r; fl = f; }
            // 12258 imul r11,r9
            { let (r, f) = imul::<64>(r11, r9); r11 = r; fl = f; }
            pc = 0x1225c;
        }
        0x1225c => {
            // block 1225c
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1225c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1225c cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // 1225f mov rcx,r13
            rcx = r13;
            // 12262 je 0x1229f
            if crate::cov::br(fl.e()) { pc = 0x1229f; continue; }
            pc = 0x12264;
        }
        0x12264 => {
            // block 12264
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12264 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12264 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 12267 mov rcx,r15
            rcx = r15;
            // 1226a je 0x1229f
            if crate::cov::br(fl.e()) { pc = 0x1229f; continue; }
            pc = 0x1226c;
        }
        0x1226c => {
            // block 1226c
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1226c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1226c mov rdi,r15
            rdi = r15;
            // 1226f mov rsi,r14
            rsi = r14;
            // 12272 mov rdx,r13
            rdx = r13;
            // 12275 mov QWORD PTR [rbp-0x90],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (r8) as u64);
            // 1227c mov QWORD PTR [rbp-0x98],r10
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (r10) as u64);
            // 12283 mov r14,r11
            r14 = r11;
            // 12286 call 0xeb8be
            { m.outregs(0x1228b, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1228b mov r11,r14
            r11 = r14;
            // 1228e mov r10,QWORD PTR [rbp-0x98]
            r10 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 12295 mov r8,QWORD PTR [rbp-0x90]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 1229c mov rcx,rax
            rcx = rax;
            pc = 0x1229f;
        }
        0x1229f => {
            // block 1229f
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1229f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1229f mov QWORD PTR [rbp-0x88],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (r13) as u64);
            // 122a6 mov r13,r8
            r13 = r8;
            // 122a9 neg r13
            { let (r, f) = neg::<64>(r13); r13 = r; fl = f; }
            // 122ac mov r14,r10
            r14 = r10;
            // 122af sub r14,r11
            { let (r, f) = sub::<64>(r14, r11);
              r14 = r;
              fl = f; }
            // 122b2 lea rax,[r8+r11*1]
            rax = r8.wrapping_add(r11);
            // 122b6 mov rdx,r10
            rdx = r10;
            // 122b9 sub rdx,r8
            { let (r, f) = sub::<64>(rdx, r8);
              rdx = r;
              fl = f; }
            // 122bc sub rdx,r11
            { let (r, f) = sub::<64>(rdx, r11);
              rdx = r;
              fl = f; }
            // 122bf sub rdx,r12
            { let (r, f) = sub::<64>(rdx, r12);
              rdx = r;
              fl = f; }
            // 122c2 sub r13,r12
            { let (r, f) = sub::<64>(r13, r12);
              r13 = r;
              fl = f; }
            // 122c5 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 122c8 jge 0x12323
            if crate::cov::br(fl.ge()) { pc = 0x12323; continue; }
            pc = 0x122ca;
        }
        0x122ca => {
            // block 122ca
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 122ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 122ca mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 122ce mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 122d2 mov QWORD PTR [rsp+0x8],rax
            m.w64(rsp.wrapping_add(0x8), (rax) as u64);
            // 122d7 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 122db mov QWORD PTR [rsp],rax
            m.w64(rsp, (rax) as u64);
            // 122df mov rdi,rbx
            rdi = rbx;
            // 122e2 mov rsi,r15
            rsi = r15;
            // 122e5 mov rdx,rcx
            rdx = rcx;
            // 122e8 mov rbx,rcx
            rbx = rcx;
            // 122eb mov rcx,QWORD PTR [rbp-0x70]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 122ef mov r9,r11
            r9 = r11;
            // 122f2 mov r12,r10
            r12 = r10;
            // 122f5 mov r15,r11
            r15 = r11;
            // 122f8 call 0x120e8
            { let r = { let sa = [m.r64(rsp), m.r64(rsp.wrapping_add(8))]; f_000120e8(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &sa) }; rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 122fd cmp r12,r15
            fl = sub::<64>(r12, r15).1;
            // 12300 mov r10,r14
            r10 = r14;
            // 12303 mov r14,QWORD PTR [rbp-0x88]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 1230a mov rdx,QWORD PTR [rbp-0x78]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1230e movabs r9,0x6db6db6db6db6db7
            r9 = 0x6db6_db6d_b6db_6db7u64;
            // 12318 jne 0x12136
            if crate::cov::br(fl.ne()) { pc = 0x12136; continue; }
            pc = 0x1231e;
        }
        0x1231e => {
            // block 1231e
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 1231e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1231e jmp 0x123d6
            pc = 0x123d6; continue;
        }
        0x12323 => {
            // block 12323
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12323 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12323 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 12327 mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 1232b mov QWORD PTR [rsp+0x8],rax
            m.w64(rsp.wrapping_add(0x8), (rax) as u64);
            // 12330 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 12334 mov QWORD PTR [rsp],rax
            m.w64(rsp, (rax) as u64);
            // 12338 mov rdi,rcx
            rdi = rcx;
            // 1233b mov rsi,QWORD PTR [rbp-0x88]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 12342 mov rdx,QWORD PTR [rbp-0x78]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 12346 mov r12,rcx
            r12 = rcx;
            // 12349 mov rcx,QWORD PTR [rbp-0x70]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1234d mov QWORD PTR [rbp-0x90],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (r8) as u64);
            // 12354 mov r8,r13
            r8 = r13;
            // 12357 mov r9,r14
            r9 = r14;
            // 1235a mov r14,r11
            r14 = r11;
            // 1235d call 0x120e8
            { let r = { let sa = [m.r64(rsp), m.r64(rsp.wrapping_add(8))]; f_000120e8(&mut *m.env, [rdi, rsi, rdx, rcx, r8, r9], [x0 as u64, 0, 0, 0, 0, 0, 0, 0], &sa) }; rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12362 mov r10,r14
            r10 = r14;
            // 12365 test r10,r10
            fl = logic::<64>(r10 & r10);
            // 12368 mov rdx,r12
            rdx = r12;
            // 1236b movabs r9,0x6db6db6db6db6db7
            r9 = 0x6db6_db6d_b6db_6db7u64;
            // 12375 mov r8,QWORD PTR [rbp-0x90]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 1237c jne 0x1212c
            if crate::cov::br(fl.ne()) { pc = 0x1212c; continue; }
            pc = 0x12382;
        }
        0x12382 => {
            // block 12382
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12382 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12382 jmp 0x123d6
            pc = 0x123d6; continue;
        }
        0x12384 => {
            // block 12384
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 12384 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12384 mov rdi,rbx
            rdi = rbx;
            // 12387 mov rsi,r14
            rsi = r14;
            // 1238a mov rcx,QWORD PTR [rbp-0x70]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1238e mov r8,r13
            r8 = r13;
            // 12391 mov r9,r10
            r9 = r10;
            // 12394 add rsp,0x88
            { let (r, f) = add::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // 1239b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1239c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1239e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 123a0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 123a2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 123a4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 123a5 jmp 0xeb8e8
            { let r = { let t = m.env.stub_va(0xb5); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
        0x123aa => {
            // block 123aa
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 123aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 123aa lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 123ae mov rsi,rbx
            rsi = rbx;
            // 123b1 call 0xe06fe
            { m.outregs(0x123b6, 0x27, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 123b6 mov rdi,rbx
            rdi = rbx;
            // 123b9 mov rsi,r14
            rsi = r14;
            // 123bc call 0xe064a
            { m.outregs(0x123c1, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 123c1 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 123c5 mov rdi,r14
            rdi = r14;
            // 123c8 call 0xe064a
            { m.outregs(0x123cd, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 123cd lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 123d1 call 0xe05fa
            { m.outregs(0x123d6, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x123d6;
        }
        0x123d6 => {
            // block 123d6
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 123d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 123d6 add rsp,0x88
            { let (r, f) = add::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // 123dd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 123de pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 123e0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 123e2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 123e4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 123e6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 123e7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x123e8 => {
            // block 123e8
            #[cfg(feature = "trace")] eprintln!("f_000120e8 pc 123e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 123e8 mov rbx,rax
            rbx = rax;
            // 123eb lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 123ef call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 123f4 mov rdi,rbx
            rdi = rbx;
            // 123f7 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe790c size=804
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e790c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe790c;
    loop {
        match pc {
        0xe790c => {
            // block e790c
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e790c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e790c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e790d mov rbp,rsp
            rbp = rsp;
            // e7910 cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // e7916 jg 0xe7941
            if crate::cov::br(fl.g()) { pc = 0xe7941; continue; }
            pc = 0xe7918;
        }
        0xe7918 => {
            // block e7918
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7918 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7918 cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // e791e jg 0xe796d
            if crate::cov::br(fl.g()) { pc = 0xe796d; continue; }
            pc = 0xe7920;
        }
        0xe7920 => {
            // block e7920
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7920 cmp edi,0x102447ff
            fl = sub::<32>((rdi as u32 as u64), 0x102447ffu64).1;
            // e7926 jg 0xe79b5
            if crate::cov::br(fl.g()) { pc = 0xe79b5; continue; }
            pc = 0xe792c;
        }
        0xe792c => {
            // block e792c
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e792c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e792c cmp edi,0xffffffff
            fl = sub::<32>((rdi as u32 as u64), 0xffffffffu64).1;
            // e792f jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7935;
        }
        0xe7935 => {
            // block e7935
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7935 lea rax,[rip+0x442a4] # 0x12bbe0
            rax = m.env.data_va(263085);
            // e793c jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7941 => {
            // block e7941
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7941 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7941 cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // e7947 jg 0xe7991
            if crate::cov::br(fl.g()) { pc = 0xe7991; continue; }
            pc = 0xe7949;
        }
        0xe7949 => {
            // block e7949
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7949 cmp edi,0x106466ff
            fl = sub::<32>((rdi as u32 as u64), 0x106466ffu64).1;
            // e794f jg 0xe79f1
            if crate::cov::br(fl.g()) { pc = 0xe79f1; continue; }
            pc = 0xe7955;
        }
        0xe7955 => {
            // block e7955
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7955 cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // e795b jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7961;
        }
        0xe7961 => {
            // block e7961
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7961 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7961 lea rax,[rip+0x4419f] # 0x12bb07
            rax = m.env.data_va(262868);
            // e7968 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe796d => {
            // block e796d
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e796d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e796d cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // e7973 jg 0xe7a1c
            if crate::cov::br(fl.g()) { pc = 0xe7a1c; continue; }
            pc = 0xe7979;
        }
        0xe7979 => {
            // block e7979
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7979 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7979 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // e797f jne 0xe7a4c
            if crate::cov::br(fl.ne()) { pc = 0xe7a4c; continue; }
            pc = 0xe7985;
        }
        0xe7985 => {
            // block e7985
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7985 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7985 lea rax,[rip+0x4412f] # 0x12babb
            rax = m.env.data_va(262792);
            // e798c jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7991 => {
            // block e7991
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7991 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7991 cmp edi,0x1071084f
            fl = sub::<32>((rdi as u32 as u64), 0x1071084fu64).1;
            // e7997 jg 0xe7a34
            if crate::cov::br(fl.g()) { pc = 0xe7a34; continue; }
            pc = 0xe799d;
        }
        0xe799d => {
            // block e799d
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e799d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e799d cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // e79a3 jne 0xe7a64
            if crate::cov::br(fl.ne()) { pc = 0xe7a64; continue; }
            pc = 0xe79a9;
        }
        0xe79a9 => {
            // block e79a9
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e79a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e79a9 lea rax,[rip+0x441cc] # 0x12bb7c
            rax = m.env.data_va(262985);
            // e79b0 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe79b5 => {
            // block e79b5
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e79b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e79b5 cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // e79bb jg 0xe7a7c
            if crate::cov::br(fl.g()) { pc = 0xe7a7c; continue; }
            pc = 0xe79c1;
        }
        0xe79c1 => {
            // block e79c1
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e79c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e79c1 cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // e79c7 jg 0xe7ad8
            if crate::cov::br(fl.g()) { pc = 0xe7ad8; continue; }
            pc = 0xe79cd;
        }
        0xe79cd => {
            // block e79cd
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e79cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e79cd cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // e79d3 je 0xe7b48
            if crate::cov::br(fl.e()) { pc = 0xe7b48; continue; }
            pc = 0xe79d9;
        }
        0xe79d9 => {
            // block e79d9
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e79d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e79d9 cmp edi,0x10266400
            fl = sub::<32>((rdi as u32 as u64), 0x10266400u64).1;
            // e79df jne 0xe7b54
            if crate::cov::br(fl.ne()) { pc = 0xe7b54; continue; }
            pc = 0xe79e5;
        }
        0xe79e5 => {
            // block e79e5
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e79e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e79e5 lea rax,[rip+0x43fde] # 0x12b9ca
            rax = m.env.data_va(262551);
            // e79ec jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe79f1 => {
            // block e79f1
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e79f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e79f1 add edi,0xef9b9900
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xef9b9900u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // e79f7 cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // e79fa ja 0xe7c0a
            if crate::cov::br(fl.a()) { pc = 0xe7c0a; continue; }
            pc = 0xe7a00;
        }
        0xe7a00 => {
            // block e7a00
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a00 lea rax,[rip+0x20d] # 0xe7c14
            rax = m.env.code_va(0xe7c14);
            // e7a07 movsxd rcx,DWORD PTR [rax+rdi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // e7a0b add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // e7a0e jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xe7a10 => {
            // block e7a10
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a10 lea rax,[rip+0x51da5] # 0x1397bc
            rax = m.env.data_va(319369);
            // e7a17 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7a1c => {
            // block e7a1c
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a1c cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // e7a22 jne 0xe7aa8
            if crate::cov::br(fl.ne()) { pc = 0xe7aa8; continue; }
            pc = 0xe7a28;
        }
        0xe7a28 => {
            // block e7a28
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a28 lea rax,[rip+0x440b0] # 0x12badf
            rax = m.env.data_va(262828);
            // e7a2f jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7a34 => {
            // block e7a34
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a34 cmp edi,0x10710850
            fl = sub::<32>((rdi as u32 as u64), 0x10710850u64).1;
            // e7a3a jne 0xe7ac0
            if crate::cov::br(fl.ne()) { pc = 0xe7ac0; continue; }
            pc = 0xe7a40;
        }
        0xe7a40 => {
            // block e7a40
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a40 lea rax,[rip+0x51df7] # 0x13983e
            rax = m.env.data_va(319499);
            // e7a47 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7a4c => {
            // block e7a4c
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a4c cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // e7a52 jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7a58;
        }
        0xe7a58 => {
            // block e7a58
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a58 lea rax,[rip+0x4406d] # 0x12bacc
            rax = m.env.data_va(262809);
            // e7a5f jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7a64 => {
            // block e7a64
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a64 cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // e7a6a jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7a70;
        }
        0xe7a70 => {
            // block e7a70
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a70 lea rax,[rip+0x51dc1] # 0x139838
            rax = m.env.data_va(319493);
            // e7a77 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7a7c => {
            // block e7a7c
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a7c cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // e7a82 jg 0xe7af8
            if crate::cov::br(fl.g()) { pc = 0xe7af8; continue; }
            pc = 0xe7a84;
        }
        0xe7a84 => {
            // block e7a84
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a84 cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // e7a8a je 0xe7b6c
            if crate::cov::br(fl.e()) { pc = 0xe7b6c; continue; }
            pc = 0xe7a90;
        }
        0xe7a90 => {
            // block e7a90
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a90 cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // e7a96 jne 0xe7b78
            if crate::cov::br(fl.ne()) { pc = 0xe7b78; continue; }
            pc = 0xe7a9c;
        }
        0xe7a9c => {
            // block e7a9c
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7a9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7a9c lea rax,[rip+0x51d0d] # 0x1397b0
            rax = m.env.data_va(319357);
            // e7aa3 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7aa8 => {
            // block e7aa8
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7aa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7aa8 cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // e7aae jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7ab4;
        }
        0xe7ab4 => {
            // block e7ab4
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7ab4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ab4 lea rax,[rip+0x44037] # 0x12baf2
            rax = m.env.data_va(262847);
            // e7abb jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7ac0 => {
            // block e7ac0
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7ac0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ac0 cmp edi,0x10710851
            fl = sub::<32>((rdi as u32 as u64), 0x10710851u64).1;
            // e7ac6 jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7acc;
        }
        0xe7acc => {
            // block e7acc
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7acc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7acc lea rax,[rip+0x51d76] # 0x139849
            rax = m.env.data_va(319510);
            // e7ad3 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7ad8 => {
            // block e7ad8
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7ad8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ad8 cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // e7ade jg 0xe7b18
            if crate::cov::br(fl.g()) { pc = 0xe7b18; continue; }
            pc = 0xe7ae0;
        }
        0xe7ae0 => {
            // block e7ae0
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ae0 cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // e7ae6 jne 0xe7bc6
            if crate::cov::br(fl.ne()) { pc = 0xe7bc6; continue; }
            pc = 0xe7aec;
        }
        0xe7aec => {
            // block e7aec
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7aec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7aec lea rax,[rip+0x43ef7] # 0x12b9ea
            rax = m.env.data_va(262583);
            // e7af3 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7af8 => {
            // block e7af8
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7af8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7af8 cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // e7afe jg 0xe7b30
            if crate::cov::br(fl.g()) { pc = 0xe7b30; continue; }
            pc = 0xe7b00;
        }
        0xe7b00 => {
            // block e7b00
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b00 cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // e7b06 jne 0xe7bd7
            if crate::cov::br(fl.ne()) { pc = 0xe7bd7; continue; }
            pc = 0xe7b0c;
        }
        0xe7b0c => {
            // block e7b0c
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b0c lea rax,[rip+0x43f77] # 0x12ba8a
            rax = m.env.data_va(262743);
            // e7b13 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7b18 => {
            // block e7b18
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b18 cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // e7b1e jne 0xe7be8
            if crate::cov::br(fl.ne()) { pc = 0xe7be8; continue; }
            pc = 0xe7b24;
        }
        0xe7b24 => {
            // block e7b24
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b24 lea rax,[rip+0x43f1e] # 0x12ba49
            rax = m.env.data_va(262678);
            // e7b2b jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7b30 => {
            // block e7b30
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b30 cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // e7b36 jne 0xe7bf9
            if crate::cov::br(fl.ne()) { pc = 0xe7bf9; continue; }
            pc = 0xe7b3c;
        }
        0xe7b3c => {
            // block e7b3c
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b3c lea rax,[rip+0x43f5b] # 0x12ba9e
            rax = m.env.data_va(262763);
            // e7b43 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7b48 => {
            // block e7b48
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b48 lea rax,[rip+0x51c59] # 0x1397a8
            rax = m.env.data_va(319349);
            // e7b4f jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7b54 => {
            // block e7b54
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b54 cmp edi,0x10266401
            fl = sub::<32>((rdi as u32 as u64), 0x10266401u64).1;
            // e7b5a jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7b60;
        }
        0xe7b60 => {
            // block e7b60
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b60 lea rax,[rip+0x43e78] # 0x12b9df
            rax = m.env.data_va(262572);
            // e7b67 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7b6c => {
            // block e7b6c
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b6c lea rax,[rip+0x43eea] # 0x12ba5d
            rax = m.env.data_va(262698);
            // e7b73 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7b78 => {
            // block e7b78
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b78 cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // e7b7e jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7b84;
        }
        0xe7b84 => {
            // block e7b84
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b84 lea rax,[rip+0x43ef0] # 0x12ba7b
            rax = m.env.data_va(262728);
            // e7b8b jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7b90 => {
            // block e7b90
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b90 lea rax,[rip+0x51c31] # 0x1397c8
            rax = m.env.data_va(319381);
            // e7b97 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7b99 => {
            // block e7b99
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7b99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7b99 lea rax,[rip+0x51c38] # 0x1397d8
            rax = m.env.data_va(319397);
            // e7ba0 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7ba2 => {
            // block e7ba2
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7ba2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ba2 lea rax,[rip+0x51c47] # 0x1397f0
            rax = m.env.data_va(319421);
            // e7ba9 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7bab => {
            // block e7bab
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7bab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7bab lea rax,[rip+0x51c52] # 0x139804
            rax = m.env.data_va(319441);
            // e7bb2 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7bb4 => {
            // block e7bb4
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7bb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7bb4 lea rax,[rip+0x51c5d] # 0x139818
            rax = m.env.data_va(319461);
            // e7bbb jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7bbd => {
            // block e7bbd
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7bbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7bbd lea rax,[rip+0x51c68] # 0x13982c
            rax = m.env.data_va(319481);
            // e7bc4 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7bc6 => {
            // block e7bc6
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7bc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7bc6 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // e7bcc jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7bce;
        }
        0xe7bce => {
            // block e7bce
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7bce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7bce lea rax,[rip+0x43e50] # 0x12ba25
            rax = m.env.data_va(262642);
            // e7bd5 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7bd7 => {
            // block e7bd7
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7bd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7bd7 cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // e7bdd jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7bdf;
        }
        0xe7bdf => {
            // block e7bdf
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7bdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7bdf lea rax,[rip+0x43eb3] # 0x12ba99
            rax = m.env.data_va(262758);
            // e7be6 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7be8 => {
            // block e7be8
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7be8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7be8 cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // e7bee jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7bf0;
        }
        0xe7bf0 => {
            // block e7bf0
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7bf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7bf0 lea rax,[rip+0x43e41] # 0x12ba38
            rax = m.env.data_va(262661);
            // e7bf7 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7bf9 => {
            // block e7bf9
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7bf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7bf9 cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // e7bff jne 0xe7c0a
            if crate::cov::br(fl.ne()) { pc = 0xe7c0a; continue; }
            pc = 0xe7c01;
        }
        0xe7c01 => {
            // block e7c01
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7c01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7c01 lea rax,[rip+0x43ea4] # 0x12baac
            rax = m.env.data_va(262777);
            // e7c08 jmp 0xe7c11
            pc = 0xe7c11; continue;
        }
        0xe7c0a => {
            // block e7c0a
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7c0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7c0a lea rax,[rip+0x510b7] # 0x138cc8
            rax = m.env.data_va(316565);
            pc = 0xe7c11;
        }
        0xe7c11 => {
            // block e7c11
            #[cfg(feature = "trace")] eprintln!("f_000e790c pc e7c11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7c11 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7c12 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb61ec size=842
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b61ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb61ec;
    loop {
        match pc {
        0xb61ec => {
            // block b61ec
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b61ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b61ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b61ed mov rbp,rsp
            rbp = rsp;
            // b61f0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b61f2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b61f4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b61f6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b61f8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b61f9 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b61fd mov QWORD PTR [rbp-0x30],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdx) as u64);
            // b6201 mov r8,rsi
            r8 = rsi;
            // b6204 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // b6208 lea rax,[rdi+0x8]
            rax = rdi.wrapping_add(0x8);
            // b620c mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // b6210 cmp rax,r8
            fl = sub::<64>(rax, r8).1;
            // b6213 je 0xb62af
            if crate::cov::br(fl.e()) { pc = 0xb62af; continue; }
            pc = 0xb6219;
        }
        0xb6219 => {
            // block b6219
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6219 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6219 lea rdx,[r8+0x20]
            rdx = r8.wrapping_add(0x20);
            // b621d movzx eax,BYTE PTR [rcx]
            rax = (m.r8(rcx) as u64) as u32 as u64;
            // b6220 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b6222 and bl,0x1
            { let r = (rbx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rbx = (rbx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b6225 jne 0xb622f
            if crate::cov::br(fl.ne()) { pc = 0xb622f; continue; }
            pc = 0xb6227;
        }
        0xb6227 => {
            // block b6227
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6227 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6227 mov r14,rax
            r14 = rax;
            // b622a shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b622d jmp 0xb6233
            pc = 0xb6233; continue;
        }
        0xb622f => {
            // block b622f
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b622f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b622f mov r14,QWORD PTR [rcx+0x8]
            r14 = m.r64(rcx.wrapping_add(0x8)) as u64;
            pc = 0xb6233;
        }
        0xb6233 => {
            // block b6233
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6233 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6233 movzx esi,BYTE PTR [rdx]
            rsi = (m.r8(rdx) as u64) as u32 as u64;
            // b6236 mov r15b,sil
            r15 = (r15 & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // b6239 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b623d jne 0xb6247
            if crate::cov::br(fl.ne()) { pc = 0xb6247; continue; }
            pc = 0xb623f;
        }
        0xb623f => {
            // block b623f
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b623f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b623f mov r12,rsi
            r12 = rsi;
            // b6242 shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // b6245 jmp 0xb624b
            pc = 0xb624b; continue;
        }
        0xb6247 => {
            // block b6247
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6247 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6247 mov r12,QWORD PTR [r8+0x28]
            r12 = m.r64(r8.wrapping_add(0x28)) as u64;
            pc = 0xb624b;
        }
        0xb624b => {
            // block b624b
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b624b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b624b test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // b624d jne 0xb6255
            if crate::cov::br(fl.ne()) { pc = 0xb6255; continue; }
            pc = 0xb624f;
        }
        0xb624f => {
            // block b624f
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b624f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b624f lea rdi,[rcx+0x1]
            rdi = rcx.wrapping_add(0x1);
            // b6253 jmp 0xb6259
            pc = 0xb6259; continue;
        }
        0xb6255 => {
            // block b6255
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6255 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6255 mov rdi,QWORD PTR [rcx+0x10]
            rdi = m.r64(rcx.wrapping_add(0x10)) as u64;
            pc = 0xb6259;
        }
        0xb6259 => {
            // block b6259
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6259 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6259 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // b625d mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // b6261 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b6264 jne 0xb6274
            if crate::cov::br(fl.ne()) { pc = 0xb6274; continue; }
            pc = 0xb6266;
        }
        0xb6266 => {
            // block b6266
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6266 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6266 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // b626a lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // b626e mov QWORD PTR [rbp-0x58],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rdx) as u64);
            // b6272 jmp 0xb6280
            pc = 0xb6280; continue;
        }
        0xb6274 => {
            // block b6274
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6274 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6274 mov QWORD PTR [rbp-0x58],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rdx) as u64);
            // b6278 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // b627c mov rsi,QWORD PTR [r8+0x30]
            rsi = m.r64(r8.wrapping_add(0x30)) as u64;
            pc = 0xb6280;
        }
        0xb6280 => {
            // block b6280
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6280 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6280 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // b6283 mov rdx,r14
            rdx = r14;
            // b6286 cmovb rdx,r12
            if fl.b() { rdx = r12; }
            // b628a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b628d je 0xb62a6
            if crate::cov::br(fl.e()) { pc = 0xb62a6; continue; }
            pc = 0xb628f;
        }
        0xb628f => {
            // block b628f
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b628f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b628f mov r13,r8
            r13 = r8;
            // b6292 call 0xec218
            { m.outregs(0xb6297, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6297 mov r8,r13
            r8 = r13;
            // b629a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b629c je 0xb62a6
            if crate::cov::br(fl.e()) { pc = 0xb62a6; continue; }
            pc = 0xb629e;
        }
        0xb629e => {
            // block b629e
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b629e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b629e mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b62a2 jns 0xb62e0
            if crate::cov::br(fl.ns()) { pc = 0xb62e0; continue; }
            pc = 0xb62a4;
        }
        0xb62a4 => {
            // block b62a4
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62a4 jmp 0xb62af
            pc = 0xb62af; continue;
        }
        0xb62a6 => {
            // block b62a6
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62a6 cmp r14,r12
            fl = sub::<64>(r14, r12).1;
            // b62a9 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b62ad jae 0xb62e0
            if crate::cov::br(fl.ae()) { pc = 0xb62e0; continue; }
            pc = 0xb62af;
        }
        0xb62af => {
            // block b62af
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62af mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b62b3 cmp QWORD PTR [rax],r8
            fl = sub::<64>(m.r64(rax) as u64, r8).1;
            // b62b6 mov rax,QWORD PTR [r8]
            rax = m.r64(r8) as u64;
            // b62b9 mov rbx,r8
            rbx = r8;
            // b62bc je 0xb6428
            if crate::cov::br(fl.e()) { pc = 0xb6428; continue; }
            pc = 0xb62c2;
        }
        0xb62c2 => {
            // block b62c2
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62c2 test rax,rax
            fl = logic::<64>(rax & rax);
            // b62c5 je 0xb6394
            if crate::cov::br(fl.e()) { pc = 0xb6394; continue; }
            pc = 0xb62cb;
        }
        0xb62cb => {
            // block b62cb
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62cb mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0xb62cf;
        }
        0xb62cf => {
            // block b62cf
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62cf mov rbx,rax
            rbx = rax;
            // b62d2 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b62d6 test rax,rax
            fl = logic::<64>(rax & rax);
            // b62d9 jne 0xb62cf
            if crate::cov::br(fl.ne()) { pc = 0xb62cf; continue; }
            pc = 0xb62db;
        }
        0xb62db => {
            // block b62db
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62db jmp 0xb63a7
            pc = 0xb63a7; continue;
        }
        0xb62e0 => {
            // block b62e0
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62e0 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b62e3 jne 0xb62ee
            if crate::cov::br(fl.ne()) { pc = 0xb62ee; continue; }
            pc = 0xb62e5;
        }
        0xb62e5 => {
            // block b62e5
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62e5 mov r9,QWORD PTR [rbp-0x60]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // b62e9 shr r9,1
            { let (r, f) = shr::<64>(r9, 0x1u64, fl); r9 = r; fl = f; }
            // b62ec jmp 0xb62f2
            pc = 0xb62f2; continue;
        }
        0xb62ee => {
            // block b62ee
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62ee mov r9,QWORD PTR [r8+0x28]
            r9 = m.r64(r8.wrapping_add(0x28)) as u64;
            pc = 0xb62f2;
        }
        0xb62f2 => {
            // block b62f2
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62f2 mov r13,QWORD PTR [rbp-0x50]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // b62f6 mov rdi,QWORD PTR [rbp-0x58]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // b62fa test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // b62fc jne 0xb6306
            if crate::cov::br(fl.ne()) { pc = 0xb6306; continue; }
            pc = 0xb62fe;
        }
        0xb62fe => {
            // block b62fe
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b62fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b62fe mov r14,r13
            r14 = r13;
            // b6301 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b6304 jmp 0xb630a
            pc = 0xb630a; continue;
        }
        0xb6306 => {
            // block b6306
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6306 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6306 mov r14,QWORD PTR [rcx+0x8]
            r14 = m.r64(rcx.wrapping_add(0x8)) as u64;
            pc = 0xb630a;
        }
        0xb630a => {
            // block b630a
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b630a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b630a test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b630d jne 0xb6314
            if crate::cov::br(fl.ne()) { pc = 0xb6314; continue; }
            pc = 0xb630f;
        }
        0xb630f => {
            // block b630f
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b630f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b630f inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // b6312 jmp 0xb6318
            pc = 0xb6318; continue;
        }
        0xb6314 => {
            // block b6314
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6314 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6314 mov rdi,QWORD PTR [r8+0x30]
            rdi = m.r64(r8.wrapping_add(0x30)) as u64;
            pc = 0xb6318;
        }
        0xb6318 => {
            // block b6318
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6318 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6318 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // b631a jne 0xb6328
            if crate::cov::br(fl.ne()) { pc = 0xb6328; continue; }
            pc = 0xb631c;
        }
        0xb631c => {
            // block b631c
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b631c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b631c mov BYTE PTR [rbp-0x40],bl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), ((rbx as u8 as u64)) as u8);
            // b631f mov rbx,r8
            rbx = r8;
            // b6322 lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // b6326 jmp 0xb6332
            pc = 0xb6332; continue;
        }
        0xb6328 => {
            // block b6328
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6328 mov BYTE PTR [rbp-0x40],bl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), ((rbx as u8 as u64)) as u8);
            // b632b mov rbx,r8
            rbx = r8;
            // b632e mov rsi,QWORD PTR [rcx+0x10]
            rsi = m.r64(rcx.wrapping_add(0x10)) as u64;
            pc = 0xb6332;
        }
        0xb6332 => {
            // block b6332
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6332 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6332 mov r15,rcx
            r15 = rcx;
            // b6335 cmp r14,r9
            fl = sub::<64>(r14, r9).1;
            // b6338 mov rdx,r9
            rdx = r9;
            // b633b cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // b633f test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b6342 je 0xb635d
            if crate::cov::br(fl.e()) { pc = 0xb635d; continue; }
            pc = 0xb6344;
        }
        0xb6344 => {
            // block b6344
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6344 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6344 mov r12,r13
            r12 = r13;
            // b6347 mov r13,r9
            r13 = r9;
            // b634a call 0xec218
            { m.outregs(0xb634f, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b634f mov r9,r13
            r9 = r13;
            // b6352 mov r13,r12
            r13 = r12;
            // b6355 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b6357 je 0xb635d
            if crate::cov::br(fl.e()) { pc = 0xb635d; continue; }
            pc = 0xb6359;
        }
        0xb6359 => {
            // block b6359
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6359 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6359 jns 0xb6388
            if crate::cov::br(fl.ns()) { pc = 0xb6388; continue; }
            pc = 0xb635b;
        }
        0xb635b => {
            // block b635b
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b635b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b635b jmp 0xb6362
            pc = 0xb6362; continue;
        }
        0xb635d => {
            // block b635d
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b635d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b635d cmp r9,r14
            fl = sub::<64>(r9, r14).1;
            // b6360 jae 0xb6388
            if crate::cov::br(fl.ae()) { pc = 0xb6388; continue; }
            pc = 0xb6362;
        }
        0xb6362 => {
            // block b6362
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6362 mov rcx,rbx
            rcx = rbx;
            // b6365 mov rax,QWORD PTR [rcx+0x8]
            rax = m.r64(rcx.wrapping_add(0x8)) as u64;
            // b6369 test rax,rax
            fl = logic::<64>(rax & rax);
            // b636c je 0xb645c
            if crate::cov::br(fl.e()) { pc = 0xb645c; continue; }
            pc = 0xb6372;
        }
        0xb6372 => {
            // block b6372
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6372 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6372 mov r8,r15
            r8 = r15;
            // b6375 mov dl,BYTE PTR [rbp-0x40]
            rdx = (rdx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u8 as u64);
            pc = 0xb6378;
        }
        0xb6378 => {
            // block b6378
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6378 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6378 mov rbx,rax
            rbx = rax;
            // b637b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b637e test rax,rax
            fl = logic::<64>(rax & rax);
            // b6381 jne 0xb6378
            if crate::cov::br(fl.ne()) { pc = 0xb6378; continue; }
            pc = 0xb6383;
        }
        0xb6383 => {
            // block b6383
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6383 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6383 jmp 0xb6471
            pc = 0xb6471; continue;
        }
        0xb6388 => {
            // block b6388
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6388 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6388 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b638c mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // b638f jmp 0xb64fb
            pc = 0xb64fb; continue;
        }
        0xb6394 => {
            // block b6394
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6394 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6394 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // b6398 mov rax,r8
            rax = r8;
            pc = 0xb639b;
        }
        0xb639b => {
            // block b639b
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b639b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b639b mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // b639f cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // b63a2 mov rax,rbx
            rax = rbx;
            // b63a5 je 0xb639b
            if crate::cov::br(fl.e()) { pc = 0xb639b; continue; }
            pc = 0xb63a7;
        }
        0xb63a7 => {
            // block b63a7
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63a7 movzx r12d,BYTE PTR [rbx+0x20]
            r12 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b63ac mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // b63af and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b63b1 jne 0xb63b8
            if crate::cov::br(fl.ne()) { pc = 0xb63b8; continue; }
            pc = 0xb63b3;
        }
        0xb63b3 => {
            // block b63b3
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63b3 shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // b63b6 jmp 0xb63bc
            pc = 0xb63bc; continue;
        }
        0xb63b8 => {
            // block b63b8
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63b8 mov r12,QWORD PTR [rbx+0x28]
            r12 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xb63bc;
        }
        0xb63bc => {
            // block b63bc
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63bc movzx r13d,BYTE PTR [rcx]
            r13 = (m.r8(rcx) as u64) as u32 as u64;
            // b63c0 mov dl,r13b
            rdx = (rdx & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // b63c3 and dl,0x1
            { let r = (rdx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b63c6 jne 0xb63cd
            if crate::cov::br(fl.ne()) { pc = 0xb63cd; continue; }
            pc = 0xb63c8;
        }
        0xb63c8 => {
            // block b63c8
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63c8 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // b63cb jmp 0xb63d1
            pc = 0xb63d1; continue;
        }
        0xb63cd => {
            // block b63cd
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63cd mov r13,QWORD PTR [rcx+0x8]
            r13 = m.r64(rcx.wrapping_add(0x8)) as u64;
            pc = 0xb63d1;
        }
        0xb63d1 => {
            // block b63d1
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63d1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b63d3 jne 0xb63de
            if crate::cov::br(fl.ne()) { pc = 0xb63de; continue; }
            pc = 0xb63d5;
        }
        0xb63d5 => {
            // block b63d5
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63d5 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // b63d9 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // b63dc jmp 0xb63e2
            pc = 0xb63e2; continue;
        }
        0xb63de => {
            // block b63de
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63de mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xb63e2;
        }
        0xb63e2 => {
            // block b63e2
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63e2 mov r14,r8
            r14 = r8;
            // b63e5 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // b63e7 jne 0xb63ef
            if crate::cov::br(fl.ne()) { pc = 0xb63ef; continue; }
            pc = 0xb63e9;
        }
        0xb63e9 => {
            // block b63e9
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63e9 lea rsi,[rcx+0x1]
            rsi = rcx.wrapping_add(0x1);
            // b63ed jmp 0xb63f3
            pc = 0xb63f3; continue;
        }
        0xb63ef => {
            // block b63ef
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63ef mov rsi,QWORD PTR [rcx+0x10]
            rsi = m.r64(rcx.wrapping_add(0x10)) as u64;
            pc = 0xb63f3;
        }
        0xb63f3 => {
            // block b63f3
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b63f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63f3 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // b63f6 mov rdx,r12
            rdx = r12;
            // b63f9 cmovb rdx,r13
            if fl.b() { rdx = r13; }
            // b63fd test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b6400 je 0xb641c
            if crate::cov::br(fl.e()) { pc = 0xb641c; continue; }
            pc = 0xb6402;
        }
        0xb6402 => {
            // block b6402
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6402 mov r15,rcx
            r15 = rcx;
            // b6405 call 0xec218
            { m.outregs(0xb640a, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b640a mov rcx,r15
            rcx = r15;
            // b640d test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b640f je 0xb641c
            if crate::cov::br(fl.e()) { pc = 0xb641c; continue; }
            pc = 0xb6411;
        }
        0xb6411 => {
            // block b6411
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6411 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6411 mov r8,r14
            r8 = r14;
            // b6414 mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b6418 jns 0xb643d
            if crate::cov::br(fl.ns()) { pc = 0xb643d; continue; }
            pc = 0xb641a;
        }
        0xb641a => {
            // block b641a
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b641a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b641a jmp 0xb6428
            pc = 0xb6428; continue;
        }
        0xb641c => {
            // block b641c
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b641c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b641c cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // b641f mov r8,r14
            r8 = r14;
            // b6422 mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b6426 jae 0xb643d
            if crate::cov::br(fl.ae()) { pc = 0xb643d; continue; }
            pc = 0xb6428;
        }
        0xb6428 => {
            // block b6428
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6428 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6428 test rax,rax
            fl = logic::<64>(rax & rax);
            // b642b je 0xb644d
            if crate::cov::br(fl.e()) { pc = 0xb644d; continue; }
            pc = 0xb642d;
        }
        0xb642d => {
            // block b642d
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b642d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b642d mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b6431 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // b6434 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // b6438 jmp 0xb64f8
            pc = 0xb64f8; continue;
        }
        0xb643d => {
            // block b643d
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b643d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b643d mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b6441 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b6445 mov rdx,rcx
            rdx = rcx;
            // b6448 jmp 0xb6512
            pc = 0xb6512; continue;
        }
        0xb644d => {
            // block b644d
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b644d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b644d mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b6451 mov QWORD PTR [rax],r8
            m.w64(rax, (r8) as u64);
            // b6454 mov rax,r8
            rax = r8;
            // b6457 jmp 0xb64fb
            pc = 0xb64fb; continue;
        }
        0xb645c => {
            // block b645c
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b645c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b645c mov rax,rcx
            rax = rcx;
            // b645f mov r8,r15
            r8 = r15;
            // b6462 mov dl,BYTE PTR [rbp-0x40]
            rdx = (rdx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u8 as u64);
            pc = 0xb6465;
        }
        0xb6465 => {
            // block b6465
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6465 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6465 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // b6469 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // b646c mov rax,rbx
            rax = rbx;
            // b646f jne 0xb6465
            if crate::cov::br(fl.ne()) { pc = 0xb6465; continue; }
            pc = 0xb6471;
        }
        0xb6471 => {
            // block b6471
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6471 cmp rbx,QWORD PTR [rbp-0x48]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64).1;
            // b6475 je 0xb64ea
            if crate::cov::br(fl.e()) { pc = 0xb64ea; continue; }
            pc = 0xb6477;
        }
        0xb6477 => {
            // block b6477
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6477 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6477 lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // b647b test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // b647d jne 0xb6484
            if crate::cov::br(fl.ne()) { pc = 0xb6484; continue; }
            pc = 0xb647f;
        }
        0xb647f => {
            // block b647f
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b647f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b647f shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // b6482 jmp 0xb6488
            pc = 0xb6488; continue;
        }
        0xb6484 => {
            // block b6484
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6484 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6484 mov r13,QWORD PTR [r8+0x8]
            r13 = m.r64(r8.wrapping_add(0x8)) as u64;
            pc = 0xb6488;
        }
        0xb6488 => {
            // block b6488
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6488 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6488 movzx r14d,BYTE PTR [rsi]
            r14 = (m.r8(rsi) as u64) as u32 as u64;
            // b648c mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b648f and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b6491 jne 0xb6498
            if crate::cov::br(fl.ne()) { pc = 0xb6498; continue; }
            pc = 0xb6493;
        }
        0xb6493 => {
            // block b6493
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6493 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6493 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b6496 jmp 0xb649c
            pc = 0xb649c; continue;
        }
        0xb6498 => {
            // block b6498
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6498 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6498 mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xb649c;
        }
        0xb649c => {
            // block b649c
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b649c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b649c test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // b649e jne 0xb64a6
            if crate::cov::br(fl.ne()) { pc = 0xb64a6; continue; }
            pc = 0xb64a0;
        }
        0xb64a0 => {
            // block b64a0
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64a0 lea rdi,[r8+0x1]
            rdi = r8.wrapping_add(0x1);
            // b64a4 jmp 0xb64aa
            pc = 0xb64aa; continue;
        }
        0xb64a6 => {
            // block b64a6
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64a6 mov rdi,QWORD PTR [r8+0x10]
            rdi = m.r64(r8.wrapping_add(0x10)) as u64;
            pc = 0xb64aa;
        }
        0xb64aa => {
            // block b64aa
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64aa mov r15,rcx
            r15 = rcx;
            // b64ad mov r12,r8
            r12 = r8;
            // b64b0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b64b2 jne 0xb64b9
            if crate::cov::br(fl.ne()) { pc = 0xb64b9; continue; }
            pc = 0xb64b4;
        }
        0xb64b4 => {
            // block b64b4
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64b4 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // b64b7 jmp 0xb64bd
            pc = 0xb64bd; continue;
        }
        0xb64b9 => {
            // block b64b9
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64b9 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xb64bd;
        }
        0xb64bd => {
            // block b64bd
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64bd cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // b64c0 mov rdx,r13
            rdx = r13;
            // b64c3 cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // b64c7 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b64ca je 0xb64df
            if crate::cov::br(fl.e()) { pc = 0xb64df; continue; }
            pc = 0xb64cc;
        }
        0xb64cc => {
            // block b64cc
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64cc call 0xec218
            { m.outregs(0xb64d1, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b64d1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b64d3 je 0xb64df
            if crate::cov::br(fl.e()) { pc = 0xb64df; continue; }
            pc = 0xb64d5;
        }
        0xb64d5 => {
            // block b64d5
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64d5 mov rdx,r12
            rdx = r12;
            // b64d8 mov rcx,r15
            rcx = r15;
            // b64db jns 0xb650a
            if crate::cov::br(fl.ns()) { pc = 0xb650a; continue; }
            pc = 0xb64dd;
        }
        0xb64dd => {
            // block b64dd
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64dd jmp 0xb64ea
            pc = 0xb64ea; continue;
        }
        0xb64df => {
            // block b64df
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64df cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // b64e2 mov rdx,r12
            rdx = r12;
            // b64e5 mov rcx,r15
            rcx = r15;
            // b64e8 jae 0xb650a
            if crate::cov::br(fl.ae()) { pc = 0xb650a; continue; }
            pc = 0xb64ea;
        }
        0xb64ea => {
            // block b64ea
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64ea cmp QWORD PTR [rcx+0x8],0x0
            fl = sub::<64>(m.r64(rcx.wrapping_add(0x8)) as u64, 0x0u64).1;
            // b64ef je 0xb6525
            if crate::cov::br(fl.e()) { pc = 0xb6525; continue; }
            pc = 0xb64f1;
        }
        0xb64f1 => {
            // block b64f1
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64f1 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b64f5 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            pc = 0xb64f8;
        }
        0xb64f8 => {
            // block b64f8
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64f8 mov rax,rbx
            rax = rbx;
            pc = 0xb64fb;
        }
        0xb64fb => {
            // block b64fb
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b64fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b64fb add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b64ff pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b6500 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b6502 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b6504 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b6506 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b6508 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6509 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb650a => {
            // block b650a
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b650a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b650a mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b650e mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xb6512;
        }
        0xb6512 => {
            // block b6512
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6512 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6512 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b6516 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b6517 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b6519 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b651b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b651d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b651f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6520 jmp 0xebc96
            { let r = { let t = m.env.stub_va(0x463); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xb6525 => {
            // block b6525
            #[cfg(feature = "trace")] eprintln!("f_000b61ec pc b6525 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6525 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b6529 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // b652c add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // b6530 mov rax,rcx
            rax = rcx;
            // b6533 jmp 0xb64fb
            pc = 0xb64fb; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6947e size=856
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006947e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6947e;
    loop {
        match pc {
        0x6947e => {
            // block 6947e
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6947e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6947e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6947f mov rbp,rsp
            rbp = rsp;
            // 69482 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 69484 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 69486 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 69488 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 6948a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6948b sub rsp,0xb8
            { let (r, f) = sub::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // 69492 mov DWORD PTR [rbp-0xd4],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c), ((rcx as u32 as u64)) as u32);
            // 69498 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 6949b mov r14,rsi
            r14 = rsi;
            // 6949e mov r13,rdi
            r13 = rdi;
            // 694a1 lea rax,[r13+0x8]
            rax = r13.wrapping_add(0x8);
            // 694a5 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 694ac mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 694b4 mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 694bc mov QWORD PTR [r13+0x0],rax
            m.w64(r13, (rax) as u64);
            // 694c0 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 694c3 call 0xceac8
            { m.outregs(0x694c8, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xceac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 694c8 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 694ca mov QWORD PTR [rbp-0xd0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (r14) as u64);
            // 694d1 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 694d8 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 694de lea r15,[rbp-0x2c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            pc = 0x694e2;
        }
        0x694e2 => {
            // block 694e2
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 694e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 694e2 mov rdi,r13
            rdi = r13;
            // 694e5 mov rsi,r15
            rsi = r15;
            // 694e8 call 0xebbb8
            { m.outregs(0x694ed, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 694ed mov DWORD PTR [rbp-0x2c],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r14 as u32 as u64)) as u32);
            // 694f1 cmp r14d,ebx
            fl = sub::<32>((r14 as u32 as u64), (rbx as u32 as u64)).1;
            // 694f4 lea eax,[r14+0x1]
            rax = (r14.wrapping_add(0x1)) as u32 as u64;
            // 694f8 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 694fb jbe 0x694e2
            if crate::cov::br(fl.be()) { pc = 0x694e2; continue; }
            pc = 0x694fd;
        }
        0x694fd => {
            // block 694fd
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 694fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 694fd mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 69500 call 0xcbe98
            { m.outregs(0x69505, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69505 mov r14,QWORD PTR [rbp-0xd0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff30)) as u64;
            // 6950c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6950e je 0x6958b
            if crate::cov::br(fl.e()) { pc = 0x6958b; continue; }
            pc = 0x69510;
        }
        0x69510 => {
            // block 69510
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69510 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69510 lea rsi,[rip+0xcc065] # 0x13557c
            rsi = m.env.data_va(302409);
            // 69517 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 6951b mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 69520 call 0xebfe4
            { m.outregs(0x69525, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69525 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 69529 lea rdx,[rbp-0x60]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 6952d mov rsi,r14
            rsi = r14;
            // 69530 call 0x69328
            { m.outregs(0x69535, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x69328); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69535 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 69539 call 0xec032
            { m.outregs(0x6953e, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6953e mov rbx,QWORD PTR [rbp-0x48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 69542 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 69546 jmp 0x69579
            pc = 0x69579; continue;
        }
        0x69548 => {
            // block 69548
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69548 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69548 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 6954c mov rdi,r13
            rdi = r13;
            // 6954f call 0xebbb8
            { m.outregs(0x69554, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69554 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 69558 test rax,rax
            fl = logic::<64>(rax & rax);
            // 6955b je 0x6956a
            if crate::cov::br(fl.e()) { pc = 0x6956a; continue; }
            pc = 0x6955d;
        }
        0x6955d => {
            // block 6955d
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6955d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6955d mov rbx,rax
            rbx = rax;
            // 69560 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 69563 test rax,rax
            fl = logic::<64>(rax & rax);
            // 69566 jne 0x6955d
            if crate::cov::br(fl.ne()) { pc = 0x6955d; continue; }
            pc = 0x69568;
        }
        0x69568 => {
            // block 69568
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69568 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69568 jmp 0x69579
            pc = 0x69579; continue;
        }
        0x6956a => {
            // block 6956a
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6956a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6956a mov rax,rbx
            rax = rbx;
            pc = 0x6956d;
        }
        0x6956d => {
            // block 6956d
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6956d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6956d mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 69571 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 69574 mov rax,rbx
            rax = rbx;
            // 69577 jne 0x6956d
            if crate::cov::br(fl.ne()) { pc = 0x6956d; continue; }
            pc = 0x69579;
        }
        0x69579 => {
            // block 69579
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69579 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69579 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 6957c jne 0x69548
            if crate::cov::br(fl.ne()) { pc = 0x69548; continue; }
            pc = 0x6957e;
        }
        0x6957e => {
            // block 6957e
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6957e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6957e mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 69582 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 69586 call 0xebcd2
            { m.outregs(0x6958b, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6958b;
        }
        0x6958b => {
            // block 6958b
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6958b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6958b mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 6958e call 0xcd01e
            { m.outregs(0x69593, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcd01e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69593 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 69595 je 0x6961f
            if crate::cov::br(fl.e()) { pc = 0x6961f; continue; }
            pc = 0x6959b;
        }
        0x6959b => {
            // block 6959b
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6959b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6959b lea rsi,[rip+0xcc2ca] # 0x13586c
            rsi = m.env.data_va(303161);
            // 695a2 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 695a9 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 695ae call 0xebfe4
            { m.outregs(0x695b3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 695b3 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 695b7 lea rdx,[rbp-0x90]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 695be mov rsi,r14
            rsi = r14;
            // 695c1 call 0x69328
            { m.outregs(0x695c6, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x69328); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 695c6 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 695cd call 0xec032
            { m.outregs(0x695d2, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 695d2 mov rbx,QWORD PTR [rbp-0x78]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 695d6 lea r15,[rbp-0x70]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 695da jmp 0x6960d
            pc = 0x6960d; continue;
        }
        0x695dc => {
            // block 695dc
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 695dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 695dc lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 695e0 mov rdi,r13
            rdi = r13;
            // 695e3 call 0xebbb8
            { m.outregs(0x695e8, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 695e8 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 695ec test rax,rax
            fl = logic::<64>(rax & rax);
            // 695ef je 0x695fe
            if crate::cov::br(fl.e()) { pc = 0x695fe; continue; }
            pc = 0x695f1;
        }
        0x695f1 => {
            // block 695f1
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 695f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 695f1 mov rbx,rax
            rbx = rax;
            // 695f4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 695f7 test rax,rax
            fl = logic::<64>(rax & rax);
            // 695fa jne 0x695f1
            if crate::cov::br(fl.ne()) { pc = 0x695f1; continue; }
            pc = 0x695fc;
        }
        0x695fc => {
            // block 695fc
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 695fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 695fc jmp 0x6960d
            pc = 0x6960d; continue;
        }
        0x695fe => {
            // block 695fe
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 695fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 695fe mov rax,rbx
            rax = rbx;
            pc = 0x69601;
        }
        0x69601 => {
            // block 69601
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69601 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 69605 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 69608 mov rax,rbx
            rax = rbx;
            // 6960b jne 0x69601
            if crate::cov::br(fl.ne()) { pc = 0x69601; continue; }
            pc = 0x6960d;
        }
        0x6960d => {
            // block 6960d
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6960d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6960d cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69610 jne 0x695dc
            if crate::cov::br(fl.ne()) { pc = 0x695dc; continue; }
            pc = 0x69612;
        }
        0x69612 => {
            // block 69612
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69612 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69612 mov rsi,QWORD PTR [rbp-0x70]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 69616 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 6961a call 0xebcd2
            { m.outregs(0x6961f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6961f;
        }
        0x6961f => {
            // block 6961f
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6961f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6961f mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 69622 call 0xce3db
            { m.outregs(0x69627, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xce3db); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69627 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 69629 je 0x69693
            if crate::cov::br(fl.e()) { pc = 0x69693; continue; }
            pc = 0x6962b;
        }
        0x6962b => {
            // block 6962b
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6962b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6962b mov DWORD PTR [rbp-0x94],0x900
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c), (0x900u64) as u32);
            // 69635 mov ebx,0x901
            rbx = (0x901u64) as u32 as u64;
            // 6963a lea r15,[rbp-0x94]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff6c);
            pc = 0x69641;
        }
        0x69641 => {
            // block 69641
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69641 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69641 mov rdi,r13
            rdi = r13;
            // 69644 mov rsi,r15
            rsi = r15;
            // 69647 call 0xebbb8
            { m.outregs(0x6964c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6964c mov DWORD PTR [rbp-0x94],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c), ((rbx as u32 as u64)) as u32);
            // 69652 cmp ebx,0x908
            fl = sub::<32>((rbx as u32 as u64), 0x908u64).1;
            // 69658 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 6965b mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 6965d jb 0x69641
            if crate::cov::br(fl.b()) { pc = 0x69641; continue; }
            pc = 0x6965f;
        }
        0x6965f => {
            // block 6965f
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6965f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6965f mov DWORD PTR [rbp-0x98],0x90e
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x90eu64) as u32);
            // 69669 mov ebx,0x90f
            rbx = (0x90fu64) as u32 as u64;
            // 6966e lea r15,[rbp-0x98]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            pc = 0x69675;
        }
        0x69675 => {
            // block 69675
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69675 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69675 mov rdi,r13
            rdi = r13;
            // 69678 mov rsi,r15
            rsi = r15;
            // 6967b call 0xebbb8
            { m.outregs(0x69680, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69680 mov DWORD PTR [rbp-0x98],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), ((rbx as u32 as u64)) as u32);
            // 69686 cmp ebx,0x919
            fl = sub::<32>((rbx as u32 as u64), 0x919u64).1;
            // 6968c lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 6968f mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 69691 jb 0x69675
            if crate::cov::br(fl.b()) { pc = 0x69675; continue; }
            pc = 0x69693;
        }
        0x69693 => {
            // block 69693
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69693 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69693 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 69696 call 0xce392
            { m.outregs(0x6969b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xce392); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6969b mov eax,DWORD PTR [rbp-0xd4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c)) as u64) as u32 as u64;
            // 696a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 696a3 je 0x6973c
            if crate::cov::br(fl.e()) { pc = 0x6973c; continue; }
            pc = 0x696a9;
        }
        0x696a9 => {
            // block 696a9
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 696a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 696a9 lea rsi,[rip+0xce6a6] # 0x137d56
            rsi = m.env.data_va(312611);
            // 696b0 lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // 696b7 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 696bc call 0xebfe4
            { m.outregs(0x696c1, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 696c1 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 696c8 lea rdx,[rbp-0xc8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // 696cf mov rsi,r14
            rsi = r14;
            // 696d2 call 0x69328
            { m.outregs(0x696d7, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x69328); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 696d7 lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // 696de call 0xec032
            { m.outregs(0x696e3, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 696e3 mov rbx,QWORD PTR [rbp-0xb0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 696ea lea r14,[rbp-0xa8]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 696f1 jmp 0x69724
            pc = 0x69724; continue;
        }
        0x696f3 => {
            // block 696f3
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 696f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 696f3 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 696f7 mov rdi,r13
            rdi = r13;
            // 696fa call 0xebbb8
            { m.outregs(0x696ff, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 696ff mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 69703 test rax,rax
            fl = logic::<64>(rax & rax);
            // 69706 je 0x69715
            if crate::cov::br(fl.e()) { pc = 0x69715; continue; }
            pc = 0x69708;
        }
        0x69708 => {
            // block 69708
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69708 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69708 mov rbx,rax
            rbx = rax;
            // 6970b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6970e test rax,rax
            fl = logic::<64>(rax & rax);
            // 69711 jne 0x69708
            if crate::cov::br(fl.ne()) { pc = 0x69708; continue; }
            pc = 0x69713;
        }
        0x69713 => {
            // block 69713
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69713 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69713 jmp 0x69724
            pc = 0x69724; continue;
        }
        0x69715 => {
            // block 69715
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69715 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69715 mov rax,rbx
            rax = rbx;
            pc = 0x69718;
        }
        0x69718 => {
            // block 69718
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69718 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69718 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 6971c cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 6971f mov rax,rbx
            rax = rbx;
            // 69722 jne 0x69718
            if crate::cov::br(fl.ne()) { pc = 0x69718; continue; }
            pc = 0x69724;
        }
        0x69724 => {
            // block 69724
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69724 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 69727 jne 0x696f3
            if crate::cov::br(fl.ne()) { pc = 0x696f3; continue; }
            pc = 0x69729;
        }
        0x69729 => {
            // block 69729
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69729 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69729 mov rsi,QWORD PTR [rbp-0xa8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // 69730 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 69737 call 0xebcd2
            { m.outregs(0x6973c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6973c;
        }
        0x6973c => {
            // block 6973c
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6973c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6973c mov rax,r13
            rax = r13;
            // 6973f add rsp,0xb8
            { let (r, f) = add::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // 69746 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 69747 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 69749 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 6974b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6974d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6974f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 69750 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x69751 => {
            // block 69751
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69751 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69751 jmp 0x69793
            pc = 0x69793; continue;
        }
        0x69753 => {
            // block 69753
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69753 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69753 jmp 0x69793
            pc = 0x69793; continue;
        }
        0x69755 => {
            // block 69755
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69755 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69755 jmp 0x69793
            pc = 0x69793; continue;
        }
        0x69757 => {
            // block 69757
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69757 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69757 mov r14,rax
            r14 = rax;
            // 6975a mov rsi,QWORD PTR [rbp-0x40]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 6975e lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 69762 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69767 jmp 0x69796
            pc = 0x69796; continue;
        }
        0x69769 => {
            // block 69769
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69769 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69769 mov r14,rax
            r14 = rax;
            // 6976c mov rsi,QWORD PTR [rbp-0x70]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 69770 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 69774 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69779 jmp 0x69796
            pc = 0x69796; continue;
        }
        0x6977b => {
            // block 6977b
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 6977b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6977b mov r14,rax
            r14 = rax;
            // 6977e mov rsi,QWORD PTR [rbp-0xa8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // 69785 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 6978c call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69791 jmp 0x69796
            pc = 0x69796; continue;
        }
        0x69793 => {
            // block 69793
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69793 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69793 mov r14,rax
            r14 = rax;
            pc = 0x69796;
        }
        0x69796 => {
            // block 69796
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 69796 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69796 mov rax,QWORD PTR [rbp-0xe0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff20)) as u64;
            // 6979d mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 697a0 mov rdi,r13
            rdi = r13;
            // 697a3 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 697a8 mov rdi,r14
            rdi = r14;
            // 697ab call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 697b0 mov r14,rax
            r14 = rax;
            // 697b3 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 697b7 jmp 0x697cf
            pc = 0x697cf; continue;
        }
        0x697b9 => {
            // block 697b9
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 697b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 697b9 mov r14,rax
            r14 = rax;
            // 697bc lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 697c3 jmp 0x697cf
            pc = 0x697cf; continue;
        }
        0x697c5 => {
            // block 697c5
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 697c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 697c5 mov r14,rax
            r14 = rax;
            // 697c8 lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            pc = 0x697cf;
        }
        0x697cf => {
            // block 697cf
            #[cfg(feature = "trace")] eprintln!("f_0006947e pc 697cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 697cf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 697d4 jmp 0x69796
            pc = 0x69796; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2cf8e size=898
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002cf8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2cf8e;
    loop {
        match pc {
        0x2cf8e => {
            // block 2cf8e
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2cf8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cf8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2cf8f mov rbp,rsp
            rbp = rsp;
            // 2cf92 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2cf94 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2cf96 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2cf98 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2cf9a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2cf9b sub rsp,0x168
            { let (r, f) = sub::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // 2cfa2 mov rbx,rsi
            rbx = rsi;
            // 2cfa5 mov QWORD PTR [rbp-0x178],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rdi) as u64);
            // 2cfac lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 2cfb3 lea r14,[rbp-0x150]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 2cfba mov rax,QWORD PTR [rip+0x11320f] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 2cfc1 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 2cfc5 mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // 2cfcc add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 2cfd0 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // 2cfd7 mov rsi,r14
            rsi = r14;
            // 2cfda call 0xec0f8
            { m.outregs(0x2cfdf, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2cfdf mov QWORD PTR [rbp-0x188],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rbx) as u64);
            // 2cfe6 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 2cfee mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // 2cff5 mov r13,QWORD PTR [rip+0x1133c4] # 0x1403c0
            r13 = m.r64(m.env.data_va(347021)) as u64;
            // 2cffc lea rbx,[r13+0x18]
            rbx = r13.wrapping_add(0x18);
            // 2d000 mov QWORD PTR [rbp-0x158],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rbx) as u64);
            // 2d007 add r13,0x40
            { let (r, f) = add::<64>(r13, 0x40u64);
              r13 = r;
              fl = f; }
            // 2d00b mov QWORD PTR [rbp-0xe8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r13) as u64);
            // 2d012 mov rdi,r14
            rdi = r14;
            // 2d015 call 0xec0c2
            { m.outregs(0x2d01a, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d01a mov QWORD PTR [rbp-0x180],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rbx) as u64);
            // 2d021 mov r12,QWORD PTR [rip+0x113380] # 0x1403a8
            r12 = m.r64(m.env.data_va(346997)) as u64;
            // 2d028 add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // 2d02c mov QWORD PTR [rbp-0x150],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r12) as u64);
            // 2d033 lea r15,[rbp-0x110]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 2d03a mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 2d045 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 2d050 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // 2d05b mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 2d066 mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // 2d070 xorps xmm0,xmm0
            x0 = 0;
            // 2d073 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 2d077 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 2d07f lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2d083 mov rdi,r14
            rdi = r14;
            // 2d086 call 0xebaa4
            { m.outregs(0x2d08b, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d08b lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2d08f call 0xec032
            { m.outregs(0x2d094, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d094 mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 2d09b lea r14,[rbp-0x158]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 2d0a2 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2d0a6 add rsi,r14
            { let (r, f) = add::<64>(rsi, r14);
              rsi = r;
              fl = f; }
            // 2d0a9 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 2d0b0 call 0xebfd2
            { m.outregs(0x2d0b5, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d0b5 mov edi,0x30
            rdi = (0x30u64) as u32 as u64;
            // 2d0ba call 0xec146
            { m.outregs(0x2d0bf, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d0bf mov r15,rax
            r15 = rax;
            // 2d0c2 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2d0c4 mov rdi,r15
            rdi = r15;
            // 2d0c7 call 0xec10a
            { m.outregs(0x2d0cc, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8d7); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d0cc mov rax,QWORD PTR [rip+0x11313d] # 0x140210
            rax = m.r64(m.env.data_va(346589)) as u64;
            // 2d0d3 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 2d0d7 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 2d0da mov rdi,QWORD PTR [rip+0x1130bf] # 0x1401a0
            rdi = m.r64(m.env.data_va(346477)) as u64;
            // 2d0e1 call 0xec0d4
            { m.outregs(0x2d0e6, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8a1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d0e6 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 2d0ed lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 2d0f4 mov rdx,r15
            rdx = r15;
            // 2d0f7 mov rcx,rax
            rcx = rax;
            // 2d0fa call 0xec0ce
            { m.outregs(0x2d0ff, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x89b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d0ff lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 2d106 call 0xec0e0
            { m.outregs(0x2d10b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d10b mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 2d112 mov r15,QWORD PTR [rax-0x18]
            r15 = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2d116 lea rbx,[rbp+r15*1-0x158]
            rbx = rbp.wrapping_add(r15).wrapping_add(0xffff_ffff_ffff_fea8);
            // 2d11e lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 2d125 mov rsi,rbx
            rsi = rbx;
            // 2d128 call 0xebfd2
            { m.outregs(0x2d12d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d12d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2d131 lea rdx,[rbp-0x160]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 2d138 mov rsi,rbx
            rsi = rbx;
            // 2d13b call 0xec104
            { m.outregs(0x2d140, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8d1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d140 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2d144 call 0xec0e0
            { m.outregs(0x2d149, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d149 mov rbx,QWORD PTR [r15+r14*1+0x28]
            rbx = m.r64(r15.wrapping_add(r14).wrapping_add(0x28)) as u64;
            // 2d14e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2d151 je 0x2d190
            if crate::cov::br(fl.e()) { pc = 0x2d190; continue; }
            pc = 0x2d153;
        }
        0x2d153 => {
            // block 2d153
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d153 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d153 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2d156 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 2d15a lea rsi,[rbp-0x160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 2d161 mov rdi,rbx
            rdi = rbx;
            // 2d164 call rax
            { m.outregs(0x2d166, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d166 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2d16a lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2d16e mov rdi,r15
            rdi = r15;
            // 2d171 mov rsi,rbx
            rsi = rbx;
            // 2d174 call 0xec0da
            { m.outregs(0x2d179, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8a7); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d179 lea rsi,[rbp-0x160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 2d180 mov rdi,rbx
            rdi = rbx;
            // 2d183 call 0xec0e6
            { m.outregs(0x2d188, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8b3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d188 mov rdi,r15
            rdi = r15;
            // 2d18b call 0xec0e0
            { m.outregs(0x2d190, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2d190;
        }
        0x2d190 => {
            // block 2d190
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d190 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d190 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 2d197 call 0xec0e0
            { m.outregs(0x2d19c, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d19c mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 2d1a3 mov esi,DWORD PTR [rax]
            rsi = (m.r32(rax) as u64) as u32 as u64;
            // 2d1a5 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 2d1ac call 0xec09e
            { m.outregs(0x2d1b1, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d1b1 mov rdi,QWORD PTR [rbp-0x178]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 2d1b8 lea rsi,[rbp-0x150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 2d1bf call 0xeb8f4
            { m.outregs(0x2d1c4, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d1c4 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 2d1cb call 0xec0e0
            { m.outregs(0x2d1d0, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d1d0 mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 2d1d7 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 2d1de mov QWORD PTR [rbp-0xe8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r13) as u64);
            // 2d1e5 mov QWORD PTR [rbp-0x150],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r12) as u64);
            // 2d1ec lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 2d1f3 call 0xec032
            { m.outregs(0x2d1f8, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d1f8 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 2d1ff call 0xec0c8
            { m.outregs(0x2d204, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d204 mov rsi,QWORD PTR [rip+0x112ffd] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 2d20b add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 2d20f lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 2d216 call 0xec080
            { m.outregs(0x2d21b, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d21b lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 2d222 call 0xec116
            { m.outregs(0x2d227, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d227 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 2d22e add rsp,0x168
            { let (r, f) = add::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // 2d235 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d236 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2d238 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2d23a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d23c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d23e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d23f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2d240 => {
            // block 2d240
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d240 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d240 mov rbx,rax
            rbx = rax;
            // 2d243 jmp 0x2d298
            pc = 0x2d298; continue;
        }
        0x2d245 => {
            // block 2d245
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d245 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d245 mov rbx,rax
            rbx = rax;
            // 2d248 jmp 0x2d259
            pc = 0x2d259; continue;
        }
        0x2d24a => {
            // block 2d24a
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d24a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d24a mov rbx,rax
            rbx = rax;
            // 2d24d lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 2d254 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2d259;
        }
        0x2d259 => {
            // block 2d259
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d259 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d259 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // 2d260 jmp 0x2d29f
            pc = 0x2d29f; continue;
        }
        0x2d262 => {
            // block 2d262
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d262 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d262 mov rbx,rax
            rbx = rax;
            // 2d265 jmp 0x2d2ef
            pc = 0x2d2ef; continue;
        }
        0x2d26a => {
            // block 2d26a
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d26a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d26a mov rbx,rax
            rbx = rax;
            // 2d26d jmp 0x2d2d8
            pc = 0x2d2d8; continue;
        }
        0x2d26f => {
            // block 2d26f
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d26f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d26f mov rbx,rax
            rbx = rax;
            // 2d272 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2d276 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d27b mov rdi,r15
            rdi = r15;
            // 2d27e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d283 mov rdi,r14
            rdi = r14;
            // 2d286 jmp 0x2d2d3
            pc = 0x2d2d3; continue;
        }
        0x2d288 => {
            // block 2d288
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d288 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d288 mov rbx,rax
            rbx = rax;
            // 2d28b jmp 0x2d2a4
            pc = 0x2d2a4; continue;
        }
        0x2d28d => {
            // block 2d28d
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d28d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d28d mov rbx,rax
            rbx = rax;
            // 2d290 mov rdi,r15
            rdi = r15;
            // 2d293 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2d298;
        }
        0x2d298 => {
            // block 2d298
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d298 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d298 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            pc = 0x2d29f;
        }
        0x2d29f => {
            // block 2d29f
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d29f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d29f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2d2a4;
        }
        0x2d2a4 => {
            // block 2d2a4
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d2a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d2a4 mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 2d2ab mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 2d2b2 mov QWORD PTR [rbp-0xe8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r13) as u64);
            // 2d2b9 mov QWORD PTR [rbp-0x150],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r12) as u64);
            // 2d2c0 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // 2d2c7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d2cc lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            pc = 0x2d2d3;
        }
        0x2d2d3 => {
            // block 2d2d3
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d2d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d2d3 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2d2d8;
        }
        0x2d2d8 => {
            // block 2d2d8
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d2d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d2d8 mov rsi,QWORD PTR [rip+0x112f29] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 2d2df add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 2d2e3 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // 2d2ea call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2d2ef;
        }
        0x2d2ef => {
            // block 2d2ef
            #[cfg(feature = "trace")] eprintln!("f_0002cf8e pc 2d2ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d2ef lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // 2d2f6 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d2fb mov rdi,rbx
            rdi = rbx;
            // 2d2fe call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd1b6c size=926
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d1b6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd1b6c;
    loop {
        match pc {
        0xd1b6c => {
            // block d1b6c
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1b6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1b6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d1b6d mov rbp,rsp
            rbp = rsp;
            // d1b70 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // d1b76 cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // d1b7c jg 0xd1ba6
            if crate::cov::br(fl.g()) { pc = 0xd1ba6; continue; }
            pc = 0xd1b7e;
        }
        0xd1b7e => {
            // block d1b7e
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1b7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1b7e cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // d1b84 jg 0xd1bef
            if crate::cov::br(fl.g()) { pc = 0xd1bef; continue; }
            pc = 0xd1b86;
        }
        0xd1b86 => {
            // block d1b86
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1b86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1b86 lea eax,[rdi-0x10266400]
            rax = (rdi.wrapping_add(0xffff_ffff_efd9_9c00)) as u32 as u64;
            // d1b8c cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1b8f jb 0xd1cc6
            if crate::cov::br(fl.b()) { pc = 0xd1cc6; continue; }
            pc = 0xd1b95;
        }
        0xd1b95 => {
            // block d1b95
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1b95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1b95 cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // d1b9b je 0xd1cc6
            if crate::cov::br(fl.e()) { pc = 0xd1cc6; continue; }
            pc = 0xd1ba1;
        }
        0xd1ba1 => {
            // block d1ba1
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ba1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ba1 jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1ba6 => {
            // block d1ba6
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ba6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ba6 cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // d1bac jg 0xd1c14
            if crate::cov::br(fl.g()) { pc = 0xd1c14; continue; }
            pc = 0xd1bae;
        }
        0xd1bae => {
            // block d1bae
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1bae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1bae cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // d1bb4 jg 0xd1c4b
            if crate::cov::br(fl.g()) { pc = 0xd1c4b; continue; }
            pc = 0xd1bba;
        }
        0xd1bba => {
            // block d1bba
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1bba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1bba cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // d1bc0 jg 0xd1cae
            if crate::cov::br(fl.g()) { pc = 0xd1cae; continue; }
            pc = 0xd1bc6;
        }
        0xd1bc6 => {
            // block d1bc6
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1bc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1bc6 cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // d1bcc je 0xd1d00
            if crate::cov::br(fl.e()) { pc = 0xd1d00; continue; }
            pc = 0xd1bd2;
        }
        0xd1bd2 => {
            // block d1bd2
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1bd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1bd2 cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // d1bd8 je 0xd1cc6
            if crate::cov::br(fl.e()) { pc = 0xd1cc6; continue; }
            pc = 0xd1bde;
        }
        0xd1bde => {
            // block d1bde
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1bde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1bde cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // d1be4 je 0xd1cc6
            if crate::cov::br(fl.e()) { pc = 0xd1cc6; continue; }
            pc = 0xd1bea;
        }
        0xd1bea => {
            // block d1bea
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1bea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1bea jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1bef => {
            // block d1bef
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1bef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1bef cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // d1bf5 jg 0xd1c74
            if crate::cov::br(fl.g()) { pc = 0xd1c74; continue; }
            pc = 0xd1bf7;
        }
        0xd1bf7 => {
            // block d1bf7
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1bf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1bf7 cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // d1bfd je 0xd1cc6
            if crate::cov::br(fl.e()) { pc = 0xd1cc6; continue; }
            pc = 0xd1c03;
        }
        0xd1c03 => {
            // block d1c03
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c03 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // d1c09 je 0xd1cc6
            if crate::cov::br(fl.e()) { pc = 0xd1cc6; continue; }
            pc = 0xd1c0f;
        }
        0xd1c0f => {
            // block d1c0f
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c0f jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1c14 => {
            // block d1c14
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c14 cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // d1c1a jg 0xd1c8d
            if crate::cov::br(fl.g()) { pc = 0xd1c8d; continue; }
            pc = 0xd1c1c;
        }
        0xd1c1c => {
            // block d1c1c
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c1c lea eax,[rdi-0x10646700]
            rax = (rdi.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d1c22 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // d1c25 jb 0xd1d00
            if crate::cov::br(fl.b()) { pc = 0xd1d00; continue; }
            pc = 0xd1c2b;
        }
        0xd1c2b => {
            // block d1c2b
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c2b lea eax,[rdi-0x10646705]
            rax = (rdi.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d1c31 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1c34 jb 0xd1d00
            if crate::cov::br(fl.b()) { pc = 0xd1d00; continue; }
            pc = 0xd1c3a;
        }
        0xd1c3a => {
            // block d1c3a
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c3a cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // d1c40 je 0xd1d00
            if crate::cov::br(fl.e()) { pc = 0xd1d00; continue; }
            pc = 0xd1c46;
        }
        0xd1c46 => {
            // block d1c46
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c46 jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1c4b => {
            // block d1c4b
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c4b cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // d1c51 jg 0xd1cde
            if crate::cov::br(fl.g()) { pc = 0xd1cde; continue; }
            pc = 0xd1c57;
        }
        0xd1c57 => {
            // block d1c57
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c57 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // d1c5d je 0xd1d00
            if crate::cov::br(fl.e()) { pc = 0xd1d00; continue; }
            pc = 0xd1c63;
        }
        0xd1c63 => {
            // block d1c63
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c63 cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // d1c69 je 0xd1d00
            if crate::cov::br(fl.e()) { pc = 0xd1d00; continue; }
            pc = 0xd1c6f;
        }
        0xd1c6f => {
            // block d1c6f
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c6f jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1c74 => {
            // block d1c74
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c74 cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // d1c7a je 0xd1d00
            if crate::cov::br(fl.e()) { pc = 0xd1d00; continue; }
            pc = 0xd1c80;
        }
        0xd1c80 => {
            // block d1c80
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c80 cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // d1c86 je 0xd1cc6
            if crate::cov::br(fl.e()) { pc = 0xd1cc6; continue; }
            pc = 0xd1c88;
        }
        0xd1c88 => {
            // block d1c88
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c88 jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1c8d => {
            // block d1c8d
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c8d lea eax,[rdi-0x10710850]
            rax = (rdi.wrapping_add(0xffff_ffff_ef8e_f7b0)) as u32 as u64;
            // d1c93 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1c96 jb 0xd1d00
            if crate::cov::br(fl.b()) { pc = 0xd1d00; continue; }
            pc = 0xd1c98;
        }
        0xd1c98 => {
            // block d1c98
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1c98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1c98 cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // d1c9e je 0xd1d00
            if crate::cov::br(fl.e()) { pc = 0xd1d00; continue; }
            pc = 0xd1ca0;
        }
        0xd1ca0 => {
            // block d1ca0
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ca0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ca0 cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // d1ca6 jne 0xd1d37
            if crate::cov::br(fl.ne()) { pc = 0xd1d37; continue; }
            pc = 0xd1cac;
        }
        0xd1cac => {
            // block d1cac
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1cac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cac jmp 0xd1d00
            pc = 0xd1d00; continue;
        }
        0xd1cae => {
            // block d1cae
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1cae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cae cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // d1cb4 jg 0xd1cf0
            if crate::cov::br(fl.g()) { pc = 0xd1cf0; continue; }
            pc = 0xd1cb6;
        }
        0xd1cb6 => {
            // block d1cb6
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1cb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cb6 cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // d1cbc je 0xd1d00
            if crate::cov::br(fl.e()) { pc = 0xd1d00; continue; }
            pc = 0xd1cbe;
        }
        0xd1cbe => {
            // block d1cbe
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1cbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cbe cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // d1cc4 jne 0xd1d37
            if crate::cov::br(fl.ne()) { pc = 0xd1d37; continue; }
            pc = 0xd1cc6;
        }
        0xd1cc6 => {
            // block d1cc6
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1cc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cc6 add esi,0xfffffff8
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffff8u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d1cc9 cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // d1ccc jae 0xd1d37
            if crate::cov::br(fl.ae()) { pc = 0xd1d37; continue; }
            pc = 0xd1cce;
        }
        0xd1cce => {
            // block d1cce
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1cce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cce movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // d1cd1 lea rcx,[rip+0x311d8] # 0x102eb0
            rcx = m.env.data_va(95869);
            // d1cd8 mov r8d,DWORD PTR [rcx+rax*4]
            r8 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // d1cdc jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1cde => {
            // block d1cde
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1cde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cde cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // d1ce4 je 0xd1d00
            if crate::cov::br(fl.e()) { pc = 0xd1d00; continue; }
            pc = 0xd1ce6;
        }
        0xd1ce6 => {
            // block d1ce6
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ce6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ce6 cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // d1cec je 0xd1d00
            if crate::cov::br(fl.e()) { pc = 0xd1d00; continue; }
            pc = 0xd1cee;
        }
        0xd1cee => {
            // block d1cee
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1cee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cee jmp 0xd1d37
            pc = 0xd1d37; continue;
        }
        0xd1cf0 => {
            // block d1cf0
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1cf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cf0 cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // d1cf6 je 0xd1d00
            if crate::cov::br(fl.e()) { pc = 0xd1d00; continue; }
            pc = 0xd1cf8;
        }
        0xd1cf8 => {
            // block d1cf8
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1cf8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cf8 cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // d1cfe jne 0xd1d37
            if crate::cov::br(fl.ne()) { pc = 0xd1d37; continue; }
            pc = 0xd1d00;
        }
        0xd1d00 => {
            // block d1d00
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d00 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // d1d05 cmp esi,0x11
            fl = sub::<32>((rsi as u32 as u64), 0x11u64).1;
            // d1d08 ja 0xd1d25
            if crate::cov::br(fl.a()) { pc = 0xd1d25; continue; }
            pc = 0xd1d0a;
        }
        0xd1d0a => {
            // block d1d0a
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d0a mov ecx,0x23d30
            rcx = (0x23d30u64) as u32 as u64;
            // d1d0f bt ecx,esi
            fl.cf = bit::<32>((rcx as u32 as u64), (rsi as u32 as u64));
            // d1d12 jae 0xd1eac
            if crate::cov::br(fl.ae()) { pc = 0xd1eac; continue; }
            pc = 0xd1d18;
        }
        0xd1d18 => {
            // block d1d18
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d18 cmp edx,0x16
            fl = sub::<32>((rdx as u32 as u64), 0x16u64).1;
            // d1d1b sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d1d1e movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // d1d21 lea eax,[rax+rax*1+0x2]
            rax = (rax.wrapping_add(rax).wrapping_add(0x2)) as u32 as u64;
            pc = 0xd1d25;
        }
        0xd1d25 => {
            // block d1d25
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d25 and esi,0xfffffffe
            { let r = (rsi as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d1d28 lea r8d,[rax*4+0x0]
            r8 = (rax.wrapping_mul(4)) as u32 as u64;
            // d1d30 cmp esi,0x10
            fl = sub::<32>((rsi as u32 as u64), 0x10u64).1;
            // d1d33 cmovne r8d,eax
            if fl.ne() { r8 = ((rax as u32 as u64)) as u32 as u64; } else { r8 = r8 as u32 as u64; }
            pc = 0xd1d37;
        }
        0xd1d37 => {
            // block d1d37
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d37 cmp edx,0x10
            fl = sub::<32>((rdx as u32 as u64), 0x10u64).1;
            // d1d3a sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d1d3d shl r8d,cl
            { let (r, f) = shl::<32>((r8 as u32 as u64), (rcx as u8 as u64), fl); r8 = (r) as u32 as u64; fl = f; }
            // d1d40 mov eax,0x10
            rax = (0x10u64) as u32 as u64;
            // d1d45 cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // d1d4b jg 0xd1d75
            if crate::cov::br(fl.g()) { pc = 0xd1d75; continue; }
            pc = 0xd1d4d;
        }
        0xd1d4d => {
            // block d1d4d
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d4d cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // d1d53 jg 0xd1dc3
            if crate::cov::br(fl.g()) { pc = 0xd1dc3; continue; }
            pc = 0xd1d55;
        }
        0xd1d55 => {
            // block d1d55
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d55 lea eax,[rdi-0x10266400]
            rax = (rdi.wrapping_add(0xffff_ffff_efd9_9c00)) as u32 as u64;
            // d1d5b cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1d5e jb 0xd1e58
            if crate::cov::br(fl.b()) { pc = 0xd1e58; continue; }
            pc = 0xd1d64;
        }
        0xd1d64 => {
            // block d1d64
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d64 cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // d1d6a je 0xd1e58
            if crate::cov::br(fl.e()) { pc = 0xd1e58; continue; }
            pc = 0xd1d70;
        }
        0xd1d70 => {
            // block d1d70
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d70 jmp 0xd1f01
            pc = 0xd1f01; continue;
        }
        0xd1d75 => {
            // block d1d75
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d75 cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // d1d7b jg 0xd1de8
            if crate::cov::br(fl.g()) { pc = 0xd1de8; continue; }
            pc = 0xd1d7d;
        }
        0xd1d7d => {
            // block d1d7d
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d7d cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // d1d83 jg 0xd1e1f
            if crate::cov::br(fl.g()) { pc = 0xd1e1f; continue; }
            pc = 0xd1d89;
        }
        0xd1d89 => {
            // block d1d89
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d89 cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // d1d8f jg 0xd1e83
            if crate::cov::br(fl.g()) { pc = 0xd1e83; continue; }
            pc = 0xd1d95;
        }
        0xd1d95 => {
            // block d1d95
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1d95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1d95 cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // d1d9b je 0xd1e93
            if crate::cov::br(fl.e()) { pc = 0xd1e93; continue; }
            pc = 0xd1da1;
        }
        0xd1da1 => {
            // block d1da1
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1da1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1da1 cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // d1da7 je 0xd1f03
            if crate::cov::br(fl.e()) { pc = 0xd1f03; continue; }
            pc = 0xd1dad;
        }
        0xd1dad => {
            // block d1dad
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1dad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1dad cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // d1db3 jne 0xd1f01
            if crate::cov::br(fl.ne()) { pc = 0xd1f01; continue; }
            pc = 0xd1db9;
        }
        0xd1db9 => {
            // block d1db9
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1db9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1db9 mov eax,0x18
            rax = (0x18u64) as u32 as u64;
            // d1dbe jmp 0xd1f03
            pc = 0xd1f03; continue;
        }
        0xd1dc3 => {
            // block d1dc3
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1dc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1dc3 cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // d1dc9 jg 0xd1e44
            if crate::cov::br(fl.g()) { pc = 0xd1e44; continue; }
            pc = 0xd1dcb;
        }
        0xd1dcb => {
            // block d1dcb
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1dcb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1dcb cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // d1dd1 je 0xd1f03
            if crate::cov::br(fl.e()) { pc = 0xd1f03; continue; }
            pc = 0xd1dd7;
        }
        0xd1dd7 => {
            // block d1dd7
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1dd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1dd7 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // d1ddd jne 0xd1f01
            if crate::cov::br(fl.ne()) { pc = 0xd1f01; continue; }
            pc = 0xd1de3;
        }
        0xd1de3 => {
            // block d1de3
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1de3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1de3 jmp 0xd1e93
            pc = 0xd1e93; continue;
        }
        0xd1de8 => {
            // block d1de8
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1de8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1de8 cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // d1dee jg 0xd1e62
            if crate::cov::br(fl.g()) { pc = 0xd1e62; continue; }
            pc = 0xd1df0;
        }
        0xd1df0 => {
            // block d1df0
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1df0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1df0 lea eax,[rdi-0x10646700]
            rax = (rdi.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d1df6 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // d1df9 jb 0xd1ef2
            if crate::cov::br(fl.b()) { pc = 0xd1ef2; continue; }
            pc = 0xd1dff;
        }
        0xd1dff => {
            // block d1dff
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1dff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1dff lea eax,[rdi-0x10646705]
            rax = (rdi.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d1e05 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1e08 jb 0xd1ef2
            if crate::cov::br(fl.b()) { pc = 0xd1ef2; continue; }
            pc = 0xd1e0e;
        }
        0xd1e0e => {
            // block d1e0e
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e0e cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // d1e14 je 0xd1ef2
            if crate::cov::br(fl.e()) { pc = 0xd1ef2; continue; }
            pc = 0xd1e1a;
        }
        0xd1e1a => {
            // block d1e1a
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e1a jmp 0xd1f01
            pc = 0xd1f01; continue;
        }
        0xd1e1f => {
            // block d1e1f
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e1f cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // d1e25 jg 0xd1e9a
            if crate::cov::br(fl.g()) { pc = 0xd1e9a; continue; }
            pc = 0xd1e27;
        }
        0xd1e27 => {
            // block d1e27
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e27 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // d1e2d je 0xd1ef2
            if crate::cov::br(fl.e()) { pc = 0xd1ef2; continue; }
            pc = 0xd1e33;
        }
        0xd1e33 => {
            // block d1e33
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e33 cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // d1e39 jne 0xd1f01
            if crate::cov::br(fl.ne()) { pc = 0xd1f01; continue; }
            pc = 0xd1e3f;
        }
        0xd1e3f => {
            // block d1e3f
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e3f jmp 0xd1ef2
            pc = 0xd1ef2; continue;
        }
        0xd1e44 => {
            // block d1e44
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e44 cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // d1e4a je 0xd1e79
            if crate::cov::br(fl.e()) { pc = 0xd1e79; continue; }
            pc = 0xd1e4c;
        }
        0xd1e4c => {
            // block d1e4c
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e4c cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // d1e52 jne 0xd1f01
            if crate::cov::br(fl.ne()) { pc = 0xd1f01; continue; }
            pc = 0xd1e58;
        }
        0xd1e58 => {
            // block d1e58
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e58 mov eax,0x20
            rax = (0x20u64) as u32 as u64;
            // d1e5d jmp 0xd1f03
            pc = 0xd1f03; continue;
        }
        0xd1e62 => {
            // block d1e62
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e62 lea eax,[rdi-0x10710850]
            rax = (rdi.wrapping_add(0xffff_ffff_ef8e_f7b0)) as u32 as u64;
            // d1e68 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d1e6b jb 0xd1ef2
            if crate::cov::br(fl.b()) { pc = 0xd1ef2; continue; }
            pc = 0xd1e71;
        }
        0xd1e71 => {
            // block d1e71
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e71 cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // d1e77 jne 0xd1eea
            if crate::cov::br(fl.ne()) { pc = 0xd1eea; continue; }
            pc = 0xd1e79;
        }
        0xd1e79 => {
            // block d1e79
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e79 mov eax,0x38
            rax = (0x38u64) as u32 as u64;
            // d1e7e jmp 0xd1f03
            pc = 0xd1f03; continue;
        }
        0xd1e83 => {
            // block d1e83
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e83 cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // d1e89 jg 0xd1ed8
            if crate::cov::br(fl.g()) { pc = 0xd1ed8; continue; }
            pc = 0xd1e8b;
        }
        0xd1e8b => {
            // block d1e8b
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e8b cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // d1e91 jne 0xd1ef9
            if crate::cov::br(fl.ne()) { pc = 0xd1ef9; continue; }
            pc = 0xd1e93;
        }
        0xd1e93 => {
            // block d1e93
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e93 mov eax,0x40
            rax = (0x40u64) as u32 as u64;
            // d1e98 jmp 0xd1f03
            pc = 0xd1f03; continue;
        }
        0xd1e9a => {
            // block d1e9a
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1e9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e9a cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // d1ea0 je 0xd1ef2
            if crate::cov::br(fl.e()) { pc = 0xd1ef2; continue; }
            pc = 0xd1ea2;
        }
        0xd1ea2 => {
            // block d1ea2
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ea2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ea2 cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // d1ea8 jne 0xd1f01
            if crate::cov::br(fl.ne()) { pc = 0xd1f01; continue; }
            pc = 0xd1eaa;
        }
        0xd1eaa => {
            // block d1eaa
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1eaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1eaa jmp 0xd1ef2
            pc = 0xd1ef2; continue;
        }
        0xd1eac => {
            // block d1eac
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1eac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1eac mov ecx,0x10001
            rcx = (0x10001u64) as u32 as u64;
            // d1eb1 bt ecx,esi
            fl.cf = bit::<32>((rcx as u32 as u64), (rsi as u32 as u64));
            // d1eb4 jae 0xd1d25
            if crate::cov::br(fl.ae()) { pc = 0xd1d25; continue; }
            pc = 0xd1eba;
        }
        0xd1eba => {
            // block d1eba
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1eba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1eba lea ecx,[rdx-0x10]
            rcx = (rdx.wrapping_add(0xffff_ffff_ffff_fff0)) as u32 as u64;
            // d1ebd cmp ecx,0x7
            fl = sub::<32>((rcx as u32 as u64), 0x7u64).1;
            // d1ec0 jae 0xd1d25
            if crate::cov::br(fl.ae()) { pc = 0xd1d25; continue; }
            pc = 0xd1ec6;
        }
        0xd1ec6 => {
            // block d1ec6
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ec6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ec6 movsxd rax,ecx
            rax = sx::<32>((rcx as u32 as u64)) as u64;
            // d1ec9 lea rcx,[rip+0x31000] # 0x102ed0
            rcx = m.env.data_va(95901);
            // d1ed0 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // d1ed3 jmp 0xd1d25
            pc = 0xd1d25; continue;
        }
        0xd1ed8 => {
            // block d1ed8
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ed8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ed8 cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // d1ede je 0xd1ef2
            if crate::cov::br(fl.e()) { pc = 0xd1ef2; continue; }
            pc = 0xd1ee0;
        }
        0xd1ee0 => {
            // block d1ee0
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ee0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ee0 cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // d1ee6 jne 0xd1f01
            if crate::cov::br(fl.ne()) { pc = 0xd1f01; continue; }
            pc = 0xd1ee8;
        }
        0xd1ee8 => {
            // block d1ee8
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ee8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ee8 jmp 0xd1ef2
            pc = 0xd1ef2; continue;
        }
        0xd1eea => {
            // block d1eea
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1eea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1eea cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // d1ef0 jne 0xd1f01
            if crate::cov::br(fl.ne()) { pc = 0xd1f01; continue; }
            pc = 0xd1ef2;
        }
        0xd1ef2 => {
            // block d1ef2
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ef2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ef2 mov eax,0x6f
            rax = (0x6fu64) as u32 as u64;
            // d1ef7 jmp 0xd1f03
            pc = 0xd1f03; continue;
        }
        0xd1ef9 => {
            // block d1ef9
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1ef9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ef9 cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // d1eff je 0xd1f03
            if crate::cov::br(fl.e()) { pc = 0xd1f03; continue; }
            pc = 0xd1f01;
        }
        0xd1f01 => {
            // block d1f01
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1f01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1f01 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd1f03;
        }
        0xd1f03 => {
            // block d1f03
            #[cfg(feature = "trace")] eprintln!("f_000d1b6c pc d1f03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1f03 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d1f05 div r8d
            { let (q, r) = div::<32>(rdx, rax, (r8 as u32 as u64)); rax = q; rdx = r; }
            // d1f08 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d1f09 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd9854 size=964
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d9854<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd9854;
    loop {
        match pc {
        0xd9854 => {
            // block d9854
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9854 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9854 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d9855 mov rbp,rsp
            rbp = rsp;
            // d9858 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d985a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d985c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d985e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d9860 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d9861 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // d9865 mov r15,rsi
            r15 = rsi;
            // d9868 mov rbx,rdi
            rbx = rdi;
            // d986b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d986e call QWORD PTR [rax+0x100]
            { m.outregs(0xd9874, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9874 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d9876 je 0xd98a6
            if crate::cov::br(fl.e()) { pc = 0xd98a6; continue; }
            pc = 0xd9878;
        }
        0xd9878 => {
            // block d9878
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9878 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9878 cmp QWORD PTR [rbx+0x6b8],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x6b8)) as u64, 0x0u64).1;
            // d9880 je 0xd98ad
            if crate::cov::br(fl.e()) { pc = 0xd98ad; continue; }
            pc = 0xd9882;
        }
        0xd9882 => {
            // block d9882
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9882 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9882 mov QWORD PTR [rbp-0x68],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rbx) as u64);
            // d9886 xorps xmm0,xmm0
            x0 = 0;
            // d9889 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d988d mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // d9895 mov cl,BYTE PTR [r15]
            rcx = (rcx & !0xff) | ((m.r8(r15) as u64) as u8 as u64);
            // d9898 test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // d989b jne 0xd98ec
            if crate::cov::br(fl.ne()) { pc = 0xd98ec; continue; }
            pc = 0xd989d;
        }
        0xd989d => {
            // block d989d
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d989d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d989d lea r14,[r15+0x1]
            r14 = r15.wrapping_add(0x1);
            // d98a1 mov rbx,r14
            rbx = r14;
            // d98a4 jmp 0xd98f4
            pc = 0xd98f4; continue;
        }
        0xd98a6 => {
            // block d98a6
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d98a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d98a6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d98a8 jmp 0xd9bc6
            pc = 0xd9bc6; continue;
        }
        0xd98ad => {
            // block d98ad
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d98ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d98ad mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // d98b2 mov rdi,rbx
            rdi = rbx;
            // d98b5 call 0xd7b18
            { m.outregs(0xd98ba, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd7b18); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d98ba mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // d98bf mov rdi,rbx
            rdi = rbx;
            // d98c2 call 0xd7528
            { m.outregs(0xd98c7, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d98c7 mov r13d,DWORD PTR [rbx+0x350]
            r13 = (m.r32(rbx.wrapping_add(0x350)) as u64) as u32 as u64;
            // d98ce mov QWORD PTR [rbp-0x68],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rbx) as u64);
            // d98d2 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // d98d6 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d98d8 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d98db mov BYTE PTR [rbp-0x71],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff8f), ((rcx as u8 as u64)) as u8);
            // d98de jne 0xd9a4d
            if crate::cov::br(fl.ne()) { pc = 0xd9a4d; continue; }
            pc = 0xd98e4;
        }
        0xd98e4 => {
            // block d98e4
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d98e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d98e4 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // d98e7 jmp 0xd9a51
            pc = 0xd9a51; continue;
        }
        0xd98ec => {
            // block d98ec
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d98ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d98ec mov rbx,QWORD PTR [r15+0x10]
            rbx = m.r64(r15.wrapping_add(0x10)) as u64;
            // d98f0 lea r14,[r15+0x1]
            r14 = r15.wrapping_add(0x1);
            pc = 0xd98f4;
        }
        0xd98f4 => {
            // block d98f4
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d98f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d98f4 lea r12,[rbp-0x40]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d98f8 lea r13,[rbp-0x41]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffbf);
            // d98fc jmp 0xd9904
            pc = 0xd9904; continue;
        }
        0xd98fe => {
            // block d98fe
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d98fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d98fe inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // d9901 mov cl,BYTE PTR [r15]
            rcx = (rcx & !0xff) | ((m.r8(r15) as u64) as u8 as u64);
            pc = 0xd9904;
        }
        0xd9904 => {
            // block d9904
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9904 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9904 test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // d9907 jne 0xd9914
            if crate::cov::br(fl.ne()) { pc = 0xd9914; continue; }
            pc = 0xd9909;
        }
        0xd9909 => {
            // block d9909
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9909 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9909 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // d990c shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // d990f mov rcx,r14
            rcx = r14;
            // d9912 jmp 0xd991c
            pc = 0xd991c; continue;
        }
        0xd9914 => {
            // block d9914
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9914 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // d9918 mov rcx,QWORD PTR [r15+0x10]
            rcx = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0xd991c;
        }
        0xd991c => {
            // block d991c
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d991c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d991c add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d991f cmp rbx,rcx
            fl = sub::<64>(rbx, rcx).1;
            // d9922 je 0xd994d
            if crate::cov::br(fl.e()) { pc = 0xd994d; continue; }
            pc = 0xd9924;
        }
        0xd9924 => {
            // block d9924
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9924 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9924 mov al,BYTE PTR [rbx]
            rax = (rax & !0xff) | ((m.r8(rbx) as u64) as u8 as u64);
            // d9926 mov BYTE PTR [rbp-0x41],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffbf), ((rax as u8 as u64)) as u8);
            // d9929 mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d992d cmp rcx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // d9931 jae 0xd993b
            if crate::cov::br(fl.ae()) { pc = 0xd993b; continue; }
            pc = 0xd9933;
        }
        0xd9933 => {
            // block d9933
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9933 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9933 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // d9935 inc QWORD PTR [rbp-0x38]
            { let (r, f) = inc::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, fl); m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r) as u64); fl = f; }
            // d9939 jmp 0xd98fe
            pc = 0xd98fe; continue;
        }
        0xd993b => {
            // block d993b
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d993b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d993b mov rdi,r12
            rdi = r12;
            // d993e mov rsi,r13
            rsi = r13;
            // d9941 call 0xeb948
            { m.outregs(0xd9946, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9946 jmp 0xd98fe
            pc = 0xd98fe; continue;
        }
        0xd9948 => {
            // block d9948
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9948 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9948 jmp 0xd9bd7
            pc = 0xd9bd7; continue;
        }
        0xd994d => {
            // block d994d
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d994d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d994d mov BYTE PTR [rbp-0x42],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffbe), (0x0u64) as u8);
            // d9951 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d9955 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // d9959 jae 0xd9968
            if crate::cov::br(fl.ae()) { pc = 0xd9968; continue; }
            pc = 0xd995b;
        }
        0xd995b => {
            // block d995b
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d995b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d995b mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // d995e inc QWORD PTR [rbp-0x38]
            { let (r, f) = inc::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, fl); m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r) as u64); fl = f; }
            // d9962 mov r15,QWORD PTR [rbp-0x68]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // d9966 jmp 0xd9979
            pc = 0xd9979; continue;
        }
        0xd9968 => {
            // block d9968
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9968 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d996c lea rsi,[rbp-0x42]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffbe);
            // d9970 call 0xeb948
            { m.outregs(0xd9975, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9975 mov r15,QWORD PTR [rbp-0x68]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0xd9979;
        }
        0xd9979 => {
            // block d9979
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9979 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9979 mov rdi,QWORD PTR [r15+0x6b8]
            rdi = m.r64(r15.wrapping_add(0x6b8)) as u64;
            // d9980 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d9983 mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // d9987 call rax
            { m.outregs(0xd9989, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9989 mov BYTE PTR [rbp-0x68],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff98), ((rax as u8 as u64)) as u8);
            // d998c mov rdi,QWORD PTR [r15+0x6b8]
            rdi = m.r64(r15.wrapping_add(0x6b8)) as u64;
            // d9993 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d9996 mov rax,QWORD PTR [rax+0x40]
            rax = m.r64(rax.wrapping_add(0x40)) as u64;
            // d999a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d999c call rax
            { m.outregs(0xd999e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d999e mov rdi,QWORD PTR [r15+0x6b8]
            rdi = m.r64(r15.wrapping_add(0x6b8)) as u64;
            // d99a5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d99a8 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d99ac mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // d99b1 call rax
            { m.outregs(0xd99b3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d99b3 mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // d99b6 mov rdi,QWORD PTR [r15+0x6b8]
            rdi = m.r64(r15.wrapping_add(0x6b8)) as u64;
            // d99bd mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d99c0 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // d99c4 mov edx,DWORD PTR [rbp-0x38]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // d99c7 sub edx,DWORD PTR [rbp-0x40]
            { let (r, f) = sub::<32>((rdx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d99ca mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // d99cd call rax
            { m.outregs(0xd99cf, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d99cf mov r13,QWORD PTR [r15+0x6b8]
            r13 = m.r64(r15.wrapping_add(0x6b8)) as u64;
            // d99d6 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d99da mov r14,QWORD PTR [rax+0x18]
            r14 = m.r64(rax.wrapping_add(0x18)) as u64;
            // d99de lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // d99e2 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d99e6 call 0xebb16
            { m.outregs(0xd99eb, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x2e3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d99eb mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // d99ee sub ecx,DWORD PTR [rbp-0x40]
            { let (r, f) = sub::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d99f1 lea rdx,[rbp-0x60]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // d99f5 mov rdi,r13
            rdi = r13;
            // d99f8 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // d99fb call r14
            { m.outregs(0xd99fe, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = r14; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d99fe mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d9a00 mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // d9a04 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d9a07 je 0xd9a18
            if crate::cov::br(fl.e()) { pc = 0xd9a18; continue; }
            pc = 0xd9a09;
        }
        0xd9a09 => {
            // block d9a09
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a09 cmp QWORD PTR [rbp-0x58],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64, rdi).1;
            // d9a0d je 0xd9a13
            if crate::cov::br(fl.e()) { pc = 0xd9a13; continue; }
            pc = 0xd9a0f;
        }
        0xd9a0f => {
            // block d9a0f
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a0f mov QWORD PTR [rbp-0x58],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rdi) as u64);
            pc = 0xd9a13;
        }
        0xd9a13 => {
            // block d9a13
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a13 call 0xec13a
            { m.outregs(0xd9a18, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd9a18;
        }
        0xd9a18 => {
            // block d9a18
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a18 mov rdi,QWORD PTR [r15+0x6b8]
            rdi = m.r64(r15.wrapping_add(0x6b8)) as u64;
            // d9a1f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d9a22 mov rax,QWORD PTR [rax+0x40]
            rax = m.r64(rax.wrapping_add(0x40)) as u64;
            // d9a26 movzx esi,BYTE PTR [rbp-0x68]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // d9a2a call rax
            { m.outregs(0xd9a2c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9a2c mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d9a30 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d9a33 je 0xd9bc6
            if crate::cov::br(fl.e()) { pc = 0xd9bc6; continue; }
            pc = 0xd9a39;
        }
        0xd9a39 => {
            // block d9a39
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a39 cmp QWORD PTR [rbp-0x38],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, rdi).1;
            // d9a3d je 0xd9a43
            if crate::cov::br(fl.e()) { pc = 0xd9a43; continue; }
            pc = 0xd9a3f;
        }
        0xd9a3f => {
            // block d9a3f
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a3f mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            pc = 0xd9a43;
        }
        0xd9a43 => {
            // block d9a43
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a43 call 0xec13a
            { m.outregs(0xd9a48, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9a48 jmp 0xd9bc6
            pc = 0xd9bc6; continue;
        }
        0xd9a4d => {
            // block d9a4d
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a4d mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0xd9a51;
        }
        0xd9a51 => {
            // block d9a51
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a51 mov QWORD PTR [rbp-0x70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rax) as u64);
            // d9a55 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // d9a58 sar r14d,0x1f
            { let (r, f) = sar::<32>((r14 as u32 as u64), 0x1fu64, fl); r14 = (r) as u32 as u64; fl = f; }
            // d9a5c shr r14d,0x1e
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x1eu64, fl); r14 = (r) as u32 as u64; fl = f; }
            // d9a60 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // d9a63 sar r14d,0x2
            { let (r, f) = sar::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // d9a67 lea eax,[r14*4+0x8]
            rax = (r14.wrapping_mul(4).wrapping_add(0x8)) as u32 as u64;
            // d9a6f movsxd rbx,eax
            rbx = sx::<32>((rax as u32 as u64)) as u64;
            // d9a72 mov rdi,rbx
            rdi = rbx;
            // d9a75 call 0xec20c
            { m.outregs(0xd9a7a, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9d9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9a7a mov r12,rax
            r12 = rax;
            // d9a7d mov rdi,r12
            rdi = r12;
            // d9a80 mov rsi,rbx
            rsi = rbx;
            // d9a83 call 0xec188
            { m.outregs(0xd9a88, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9a88 cmp BYTE PTR [rbp-0x71],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff8f)) as u64, 0x0u64).1;
            // d9a8c jne 0xd9a93
            if crate::cov::br(fl.ne()) { pc = 0xd9a93; continue; }
            pc = 0xd9a8e;
        }
        0xd9a8e => {
            // block d9a8e
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a8e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // d9a91 jmp 0xd9a97
            pc = 0xd9a97; continue;
        }
        0xd9a93 => {
            // block d9a93
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a93 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0xd9a97;
        }
        0xd9a97 => {
            // block d9a97
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9a97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9a97 mov rdi,r12
            rdi = r12;
            // d9a9a mov rsi,r15
            rsi = r15;
            // d9a9d call 0xec2a8
            { m.outregs(0xd9aa2, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa75); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9aa2 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // d9aa7 mov r15,QWORD PTR [rbp-0x68]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // d9aab mov rdi,r15
            rdi = r15;
            // d9aae call 0xd7528
            { m.outregs(0xd9ab3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9ab3 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // d9ab7 cmp eax,0xfffffff9
            fl = sub::<32>((rax as u32 as u64), 0xfffffff9u64).1;
            // d9aba jl 0xd9ae0
            if crate::cov::br(fl.l()) { pc = 0xd9ae0; continue; }
            pc = 0xd9abc;
        }
        0xd9abc => {
            // block d9abc
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9abc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9abc add r14d,0x2
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x2u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d9ac0 movsxd r14,r14d
            r14 = sx::<32>((r14 as u32 as u64)) as u64;
            // d9ac3 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd9ac5;
        }
        0xd9ac5 => {
            // block d9ac5
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9ac5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9ac5 mov edx,DWORD PTR [r12+rbx*4]
            rdx = (m.r32(r12.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // d9ac9 mov rdi,r15
            rdi = r15;
            // d9acc mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // d9acf call 0xd8140
            { m.outregs(0xd9ad4, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd8140); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9ad4 add r13d,0x4
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x4u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d9ad8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // d9adb cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // d9ade jl 0xd9ac5
            if crate::cov::br(fl.l()) { pc = 0xd9ac5; continue; }
            pc = 0xd9ae0;
        }
        0xd9ae0 => {
            // block d9ae0
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9ae0 mov rdi,r12
            rdi = r12;
            // d9ae3 call 0xec1d0
            { m.outregs(0xd9ae8, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x99d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9ae8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9aeb mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d9af0 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d9af5 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9afa xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9afd mov rdi,r15
            rdi = r15;
            // d9b00 call QWORD PTR [rax+0x30]
            { m.outregs(0xd9b03, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b03 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b06 mov rdi,r15
            rdi = r15;
            // d9b09 call QWORD PTR [rax+0x9e0]
            { m.outregs(0xd9b0f, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b0f mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b12 mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d9b17 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // d9b1c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9b21 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9b24 mov rdi,r15
            rdi = r15;
            // d9b27 call QWORD PTR [rax+0x30]
            { m.outregs(0xd9b2a, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b2a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b2d mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d9b32 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d9b37 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9b3c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9b3f mov rdi,r15
            rdi = r15;
            // d9b42 call QWORD PTR [rax+0x30]
            { m.outregs(0xd9b45, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b45 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b48 mov rdi,r15
            rdi = r15;
            // d9b4b call QWORD PTR [rax+0x9e0]
            { m.outregs(0xd9b51, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b51 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b54 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d9b59 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d9b5e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9b63 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9b66 mov rdi,r15
            rdi = r15;
            // d9b69 call QWORD PTR [rax+0x30]
            { m.outregs(0xd9b6c, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b6c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b6f mov rdi,r15
            rdi = r15;
            // d9b72 call QWORD PTR [rax+0x9e0]
            { m.outregs(0xd9b78, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b78 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b7b mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d9b80 mov edx,0x9c
            rdx = (0x9cu64) as u32 as u64;
            // d9b85 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9b8a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9b8d mov rdi,r15
            rdi = r15;
            // d9b90 call QWORD PTR [rax+0x30]
            { m.outregs(0xd9b93, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9b93 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9b96 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d9b9b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d9ba0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d9ba5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d9ba8 mov rdi,r15
            rdi = r15;
            // d9bab call QWORD PTR [rax+0x30]
            { m.outregs(0xd9bae, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9bae mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d9bb1 mov rdi,r15
            rdi = r15;
            // d9bb4 call QWORD PTR [rax+0x9e0]
            { m.outregs(0xd9bba, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9bba xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d9bbc mov rdi,r15
            rdi = r15;
            // d9bbf call 0xd7528
            { m.outregs(0xd9bc4, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9bc4 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd9bc6;
        }
        0xd9bc6 => {
            // block d9bc6
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9bc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9bc6 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d9bc8 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // d9bcc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d9bcd pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d9bcf pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d9bd1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d9bd3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d9bd5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d9bd6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd9bd7 => {
            // block d9bd7
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9bd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9bd7 mov rbx,rax
            rbx = rax;
            pc = 0xd9bda;
        }
        0xd9bda => {
            // block d9bda
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9bda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9bda mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // d9bde test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d9be1 je 0xd9bf2
            if crate::cov::br(fl.e()) { pc = 0xd9bf2; continue; }
            pc = 0xd9be3;
        }
        0xd9be3 => {
            // block d9be3
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9be3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9be3 cmp QWORD PTR [rbp-0x38],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, rdi).1;
            // d9be7 je 0xd9bed
            if crate::cov::br(fl.e()) { pc = 0xd9bed; continue; }
            pc = 0xd9be9;
        }
        0xd9be9 => {
            // block d9be9
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9be9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9be9 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            pc = 0xd9bed;
        }
        0xd9bed => {
            // block d9bed
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9bed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9bed call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd9bf2;
        }
        0xd9bf2 => {
            // block d9bf2
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9bf2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9bf2 mov rdi,rbx
            rdi = rbx;
            // d9bf5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9bfa mov rbx,rax
            rbx = rax;
            // d9bfd mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // d9c01 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d9c04 je 0xd9bda
            if crate::cov::br(fl.e()) { pc = 0xd9bda; continue; }
            pc = 0xd9c06;
        }
        0xd9c06 => {
            // block d9c06
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9c06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9c06 cmp QWORD PTR [rbp-0x58],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64, rdi).1;
            // d9c0a je 0xd9c10
            if crate::cov::br(fl.e()) { pc = 0xd9c10; continue; }
            pc = 0xd9c0c;
        }
        0xd9c0c => {
            // block d9c0c
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9c0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9c0c mov QWORD PTR [rbp-0x58],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rdi) as u64);
            pc = 0xd9c10;
        }
        0xd9c10 => {
            // block d9c10
            #[cfg(feature = "trace")] eprintln!("f_000d9854 pc d9c10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d9c10 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d9c15 jmp 0xd9bda
            pc = 0xd9bda; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd858e size=968
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000d858e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd858e;
    loop {
        match pc {
        0xd858e => {
            // block d858e
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d858e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d858e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d858f mov rbp,rsp
            rbp = rsp;
            // d8592 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d8594 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d8596 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d8598 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d859a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d859b sub rsp,0xc8
            { let (r, f) = sub::<64>(rsp, 0xc8u64);
              rsp = r;
              fl = f; }
            // d85a2 mov rbx,rdi
            rbx = rdi;
            // d85a5 mov QWORD PTR [rbp-0xc8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (rbx) as u64);
            // d85ac mov rax,QWORD PTR [rip+0x67e25] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // d85b3 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d85b6 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // d85ba lea rdi,[rip+0x608d7] # 0x138e98
            rdi = m.env.data_va(317029);
            // d85c1 call 0xec272
            { m.outregs(0xd85c6, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa3f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d85c6 cmp DWORD PTR [rbx+0x334],0x0
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x334)) as u64, 0x0u64).1;
            // d85cd je 0xd8916
            if crate::cov::br(fl.e()) { pc = 0xd8916; continue; }
            pc = 0xd85d3;
        }
        0xd85d3 => {
            // block d85d3
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d85d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d85d3 lea r13,[rip+0x608c9] # 0x138ea3
            r13 = m.env.data_va(317040);
            // d85da xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d85dc mov QWORD PTR [rbp-0xb8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rax) as u64);
            // d85e3 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d85e6 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd85e9;
        }
        0xd85e9 => {
            // block d85e9
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d85e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d85e9 mov rdi,rbx
            rdi = rbx;
            // d85ec call 0xd8956
            { m.outregs(0xd85f1, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd8956); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d85f1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d85f3 je 0xd8697
            if crate::cov::br(fl.e()) { pc = 0xd8697; continue; }
            pc = 0xd85f9;
        }
        0xd85f9 => {
            // block d85f9
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d85f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d85f9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d85fb mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // d85fe div DWORD PTR [rbx+0x338]
            { let (q, r) = div::<32>(rdx, rax, m.r32(rbx.wrapping_add(0x338)) as u64); rax = q; rdx = r; }
            // d8604 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // d8606 jne 0xd8697
            if crate::cov::br(fl.ne()) { pc = 0xd8697; continue; }
            pc = 0xd860c;
        }
        0xd860c => {
            // block d860c
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d860c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d860c add r12d,r15d
            { let (r, f) = add::<32>((r12 as u32 as u64), (r15 as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // d860f cmp r12d,0x1000000
            fl = sub::<32>((r12 as u32 as u64), 0x1000000u64).1;
            // d8616 je 0xd863f
            if crate::cov::br(fl.e()) { pc = 0xd863f; continue; }
            pc = 0xd8618;
        }
        0xd8618 => {
            // block d8618
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d8618 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8618 cmp r12d,0x2000000
            fl = sub::<32>((r12 as u32 as u64), 0x2000000u64).1;
            // d861f jne 0xd865d
            if crate::cov::br(fl.ne()) { pc = 0xd865d; continue; }
            pc = 0xd8621;
        }
        0xd8621 => {
            // block d8621
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d8621 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8621 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // d8626 mov rdi,rbx
            rdi = rbx;
            // d8629 call 0xd7528
            { m.outregs(0xd862e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d862e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8630 mov QWORD PTR [rbp-0xb8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rax) as u64);
            // d8637 mov r12d,0x2000000
            r12 = (0x2000000u64) as u32 as u64;
            // d863d jmp 0xd8697
            pc = 0xd8697; continue;
        }
        0xd863f => {
            // block d863f
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d863f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d863f mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // d8644 mov rdi,rbx
            rdi = rbx;
            // d8647 call 0xd7528
            { m.outregs(0xd864c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d864c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d864e mov QWORD PTR [rbp-0xb8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rax) as u64);
            // d8655 mov r12d,0x1000000
            r12 = (0x1000000u64) as u32 as u64;
            // d865b jmp 0xd8697
            pc = 0xd8697; continue;
        }
        0xd865d => {
            // block d865d
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d865d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d865d cmp r12d,0x3000000
            fl = sub::<32>((r12 as u32 as u64), 0x3000000u64).1;
            // d8664 jne 0xd8684
            if crate::cov::br(fl.ne()) { pc = 0xd8684; continue; }
            pc = 0xd8666;
        }
        0xd8666 => {
            // block d8666
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d8666 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8666 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d866b mov rdi,rbx
            rdi = rbx;
            // d866e call 0xd7528
            { m.outregs(0xd8673, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8673 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8675 mov QWORD PTR [rbp-0xb8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rax) as u64);
            // d867c mov r12d,0x3000000
            r12 = (0x3000000u64) as u32 as u64;
            // d8682 jmp 0xd8697
            pc = 0xd8697; continue;
        }
        0xd8684 => {
            // block d8684
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d8684 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8684 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8686 mov QWORD PTR [rbp-0xb8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rax) as u64);
            // d868d xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d868f mov rdi,rbx
            rdi = rbx;
            // d8692 call 0xd7528
            { m.outregs(0xd8697, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd8697;
        }
        0xd8697 => {
            // block d8697
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d8697 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8697 mov QWORD PTR [rbp-0xd8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r12) as u64);
            // d869e mov ecx,DWORD PTR [rbx+0x334]
            rcx = (m.r32(rbx.wrapping_add(0x334)) as u64) as u32 as u64;
            // d86a4 sub ecx,r15d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r15 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // d86a7 mov DWORD PTR [rbp-0xcc],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff34), ((r15 as u32 as u64)) as u32);
            // d86ae cmp ecx,0x20
            fl = sub::<32>((rcx as u32 as u64), 0x20u64).1;
            // d86b1 mov eax,0x20
            rax = (0x20u64) as u32 as u64;
            // d86b6 cmovae ecx,eax
            if fl.ae() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // d86b9 mov QWORD PTR [rbp-0xc0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rcx) as u64);
            // d86c0 mov BYTE PTR [rbp-0xa0],0x53
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x53u64) as u8);
            // d86c7 mov BYTE PTR [rbp-0x9f],0x33
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff61), (0x33u64) as u8);
            // d86ce lea eax,[rcx+0x5]
            rax = (rcx.wrapping_add(0x5)) as u32 as u64;
            // d86d1 movzx edx,ax
            rdx = ((rax as u16 as u64)) as u32 as u64;
            // d86d4 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d86d7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d86d9 lea rdi,[rbp-0x9e]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff62);
            // d86e0 mov rsi,r13
            rsi = r13;
            // d86e3 call 0xec296
            { m.outregs(0xd86e8, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d86e8 mov rax,QWORD PTR [rbp-0xb8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // d86ef lea r15d,[rax+r12*1]
            r15 = (rax.wrapping_add(r12)) as u32 as u64;
            // d86f3 mov r12d,r15d
            r12 = ((r15 as u32 as u64)) as u32 as u64;
            // d86f6 shr r12d,0x18
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x18u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // d86fa xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d86fc lea rdi,[rbp-0x9c]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff64);
            // d8703 mov rsi,r13
            rsi = r13;
            // d8706 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // d8709 call 0xec296
            { m.outregs(0xd870e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d870e mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // d8711 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d8714 mov DWORD PTR [rbp-0xb0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff50), ((rax as u32 as u64)) as u32);
            // d871a movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // d871d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d871f lea rdi,[rbp-0x9a]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff66);
            // d8726 mov rsi,r13
            rsi = r13;
            // d8729 call 0xec296
            { m.outregs(0xd872e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d872e mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // d8731 shr eax,0x8
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // d8734 mov DWORD PTR [rbp-0xdc],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff24), ((rax as u32 as u64)) as u32);
            // d873a movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // d873d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d873f lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // d8746 mov rsi,r13
            rsi = r13;
            // d8749 call 0xec296
            { m.outregs(0xd874e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d874e movzx edx,r15b
            rdx = ((r15 as u8 as u64)) as u32 as u64;
            // d8752 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8754 lea rdi,[rbp-0x96]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff6a);
            // d875b mov rsi,r13
            rsi = r13;
            // d875e call 0xec296
            { m.outregs(0xd8763, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8763 add r12d,r15d
            { let (r, f) = add::<32>((r12 as u32 as u64), (r15 as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // d8766 add r12d,DWORD PTR [rbp-0xb0]
            { let (r, f) = add::<32>((r12 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d876d add r12d,DWORD PTR [rbp-0xdc]
            { let (r, f) = add::<32>((r12 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff24)) as u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d8774 mov rax,QWORD PTR [rbp-0xc0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // d877b lea r15d,[rax+r12*1+0x5]
            r15 = (rax.wrapping_add(r12).wrapping_add(0x5)) as u32 as u64;
            // d8780 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d8782 mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // d8787 je 0xd88c9
            if crate::cov::br(fl.e()) { pc = 0xd88c9; continue; }
            pc = 0xd878d;
        }
        0xd878d => {
            // block d878d
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d878d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d878d mov QWORD PTR [rbp-0xb0],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (rdx) as u64);
            // d8794 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8797 mov rcx,QWORD PTR [rbp-0xb8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // d879e mov rdx,QWORD PTR [rbp-0xb0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // d87a5 lea edx,[rcx+rdx*1]
            rdx = (rcx.wrapping_add(rdx)) as u32 as u64;
            // d87a8 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d87ad mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d87b2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d87b5 mov rdi,rbx
            rdi = rbx;
            // d87b8 call QWORD PTR [rax+0x30]
            { m.outregs(0xd87bb, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d87bb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d87be mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d87c3 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d87c8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d87cd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d87d0 mov rdi,rbx
            rdi = rbx;
            // d87d3 call QWORD PTR [rax+0x30]
            { m.outregs(0xd87d6, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d87d6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d87d9 mov rdi,rbx
            rdi = rbx;
            // d87dc call QWORD PTR [rax+0x9e0]
            { m.outregs(0xd87e2, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d87e2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d87e5 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d87ea mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d87ef xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d87f2 mov rdi,rbx
            rdi = rbx;
            // d87f5 lea rdx,[rbp-0xa4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff5c);
            // d87fc call QWORD PTR [rax+0x38]
            { m.outregs(0xd87ff, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d87ff mov ebx,DWORD PTR [rbp-0xa4]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c)) as u64) as u32 as u64;
            // d8805 movzx edx,bl
            rdx = ((rbx as u8 as u64)) as u32 as u64;
            // d8808 mov r12,r14
            r12 = r14;
            // d880b lea rdi,[rbp+r12*1-0x94]
            rdi = rbp.wrapping_add(r12).wrapping_add(0xffff_ffff_ffff_ff6c);
            // d8813 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8815 mov rsi,r13
            rsi = r13;
            // d8818 call 0xec296
            { m.outregs(0xd881d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d881d add r15d,ebx
            { let (r, f) = add::<32>((r15 as u32 as u64), (rbx as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8820 mov r14d,DWORD PTR [rbp-0xa4]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c)) as u64) as u32 as u64;
            // d8827 shr r14d,0x8
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x8u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // d882b movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // d882f lea rdi,[rbp+r12*1-0x92]
            rdi = rbp.wrapping_add(r12).wrapping_add(0xffff_ffff_ffff_ff6e);
            // d8837 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d8839 mov rsi,r13
            rsi = r13;
            // d883c call 0xec296
            { m.outregs(0xd8841, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8841 add r14d,r15d
            { let (r, f) = add::<32>((r14 as u32 as u64), (r15 as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // d8844 mov ebx,DWORD PTR [rbp-0xa4]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c)) as u64) as u32 as u64;
            // d884a shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // d884d movzx edx,bl
            rdx = ((rbx as u8 as u64)) as u32 as u64;
            // d8850 lea rdi,[rbp+r12*1-0x90]
            rdi = rbp.wrapping_add(r12).wrapping_add(0xffff_ffff_ffff_ff70);
            // d8858 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d885a mov rsi,r13
            rsi = r13;
            // d885d call 0xec296
            { m.outregs(0xd8862, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8862 add ebx,r14d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r14 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // d8865 mov r14d,DWORD PTR [rbp-0xa4]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff5c)) as u64) as u32 as u64;
            // d886c shr r14d,0x18
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x18u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // d8870 lea rdi,[rbp+r12*1-0x8e]
            rdi = rbp.wrapping_add(r12).wrapping_add(0xffff_ffff_ffff_ff72);
            // d8878 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d887a mov rsi,r13
            rsi = r13;
            // d887d mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d8880 call 0xec296
            { m.outregs(0xd8885, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8885 mov rdx,QWORD PTR [rbp-0xb0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // d888c movzx r15d,bl
            r15 = ((rbx as u8 as u64)) as u32 as u64;
            // d8890 add r15d,r14d
            { let (r, f) = add::<32>((r15 as u32 as u64), (r14 as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8893 mov rbx,QWORD PTR [rbp-0xc8]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff38)) as u64;
            // d889a add edx,0x4
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x4u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d889d add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // d88a1 mov r14,r12
            r14 = r12;
            // d88a4 mov rax,QWORD PTR [rbp-0xc0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // d88ab cmp edx,eax
            fl = sub::<32>((rdx as u32 as u64), (rax as u32 as u64)).1;
            // d88ad jb 0xd878d
            if crate::cov::br(fl.b()) { pc = 0xd878d; continue; }
            pc = 0xd88b3;
        }
        0xd88b3 => {
            // block d88b3
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d88b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d88b3 add r14,0xc
            { let (r, f) = add::<64>(r14, 0xcu64);
              r14 = r;
              fl = f; }
            // d88b7 mov rax,QWORD PTR [rbp-0xb8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // d88be add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // d88c0 mov QWORD PTR [rbp-0xb8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rax) as u64);
            // d88c7 jmp 0xd88cf
            pc = 0xd88cf; continue;
        }
        0xd88c9 => {
            // block d88c9
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d88c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d88c9 mov r14d,0xc
            r14 = (0xcu64) as u32 as u64;
            pc = 0xd88cf;
        }
        0xd88cf => {
            // block d88cf
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d88cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d88cf movsxd rax,r14d
            rax = sx::<32>((r14 as u32 as u64)) as u64;
            // d88d2 lea rdi,[rbp+rax*1-0xa0]
            rdi = rbp.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_ff60);
            // d88da not r15d
            { let v = !(r15 as u32 as u64); r15 = (v) as u32 as u64; }
            // d88dd movzx edx,r15b
            rdx = ((r15 as u8 as u64)) as u32 as u64;
            // d88e1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d88e3 mov rsi,r13
            rsi = r13;
            // d88e6 call 0xec296
            { m.outregs(0xd88eb, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d88eb lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d88f2 call 0xec272
            { m.outregs(0xd88f7, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa3f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d88f7 mov r15d,DWORD PTR [rbp-0xcc]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff34)) as u64) as u32 as u64;
            // d88fe add r15d,0x20
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x20u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // d8902 cmp r15d,DWORD PTR [rbx+0x334]
            fl = sub::<32>((r15 as u32 as u64), m.r32(rbx.wrapping_add(0x334)) as u64).1;
            // d8909 mov r12,QWORD PTR [rbp-0xd8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff28)) as u64;
            // d8910 jb 0xd85e9
            if crate::cov::br(fl.b()) { pc = 0xd85e9; continue; }
            pc = 0xd8916;
        }
        0xd8916 => {
            // block d8916
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d8916 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8916 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d8918 mov rdi,rbx
            rdi = rbx;
            // d891b call 0xd7528
            { m.outregs(0xd8920, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8920 lea rdi,[rip+0x60581] # 0x138ea8
            rdi = m.env.data_va(317045);
            // d8927 call 0xec272
            { m.outregs(0xd892c, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa3f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d892c mov rax,QWORD PTR [rip+0x67aa5] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // d8933 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d8936 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // d893a jne 0xd8950
            if crate::cov::br(fl.ne()) { pc = 0xd8950; continue; }
            pc = 0xd893c;
        }
        0xd893c => {
            // block d893c
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d893c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d893c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d893e add rsp,0xc8
            { let (r, f) = add::<64>(rsp, 0xc8u64);
              rsp = r;
              fl = f; }
            // d8945 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d8946 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d8948 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d894a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d894c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d894e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d894f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd8950 => {
            // block d8950
            #[cfg(feature = "trace")] eprintln!("f_000d858e pc d8950 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8950 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdddb0 size=1000
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000dddb0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdddb0;
    loop {
        match pc {
        0xdddb0 => {
            // block dddb0
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc dddb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dddb0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dddb1 mov rbp,rsp
            rbp = rsp;
            // dddb4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dddb6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dddb8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dddba push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dddbc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dddbd sub rsp,0xa078
            { let (r, f) = sub::<64>(rsp, 0xa078u64);
              rsp = r;
              fl = f; }
            // dddc4 mov r12,rdi
            r12 = rdi;
            // dddc7 mov rax,QWORD PTR [rip+0x6260a] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // dddce mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dddd1 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // dddd5 mov QWORD PTR [rbp-0xa038],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5fc8), (0x0u64) as u64);
            // ddde0 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // ddde4 lea rsi,[rbp-0xa03c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fc4);
            // dddeb xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ddded call QWORD PTR [rax+0x3f0]
            { m.outregs(0xdddf3, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dddf3 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dddf7 lea rsi,[rbp-0xa040]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fc0);
            // dddfe xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dde00 mov rdi,r12
            rdi = r12;
            // dde03 call QWORD PTR [rax+0x388]
            { m.outregs(0xdde09, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dde09 mov ebx,DWORD PTR [rbp-0xa040]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5fc0)) as u64) as u32 as u64;
            // dde0f cmp ebx,0xa
            fl = sub::<32>((rbx as u32 as u64), 0xau64).1;
            // dde12 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // dde16 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dde1a lea rsi,[rbp-0xa041]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fbf);
            // dde21 lea rdx,[rbp-0xa042]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5fbe);
            // dde28 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dde2a mov rdi,r12
            rdi = r12;
            // dde2d call QWORD PTR [rax+0x5e0]
            { m.outregs(0xdde33, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x5e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dde33 mov esi,DWORD PTR [rbp-0xa03c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5fc4)) as u64) as u32 as u64;
            // dde39 mov edx,DWORD PTR [r12+0x274]
            rdx = (m.r32(r12.wrapping_add(0x274)) as u64) as u32 as u64;
            // dde41 movzx r9d,BYTE PTR [rbp-0xa042]
            r9 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_5fbe)) as u64) as u32 as u64;
            // dde49 movzx ecx,BYTE PTR [rbp-0xa041]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_5fbf)) as u64) as u32 as u64;
            // dde50 movzx r8d,r14b
            r8 = ((r14 as u8 as u64)) as u32 as u64;
            // dde54 lea rdi,[rbp-0xa080]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5f80);
            // dde5b call 0x32d57
            { m.outregs(0xdde60, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x32d57); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dde60 cmp ebx,0xa
            fl = sub::<32>((rbx as u32 as u64), 0xau64).1;
            // dde63 mov eax,DWORD PTR [rbp-0xa07c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f84)) as u64) as u32 as u64;
            // dde69 mov DWORD PTR [rbp-0xa084],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c), ((rax as u32 as u64)) as u32);
            // dde6f mov eax,DWORD PTR [rbp-0xa078]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f88)) as u64) as u32 as u64;
            // dde75 mov DWORD PTR [rbp-0xa088],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f78), ((rax as u32 as u64)) as u32);
            // dde7b mov eax,DWORD PTR [rbp-0xa080]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f80)) as u64) as u32 as u64;
            // dde81 mov DWORD PTR [rbp-0xa090],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f70), ((rax as u32 as u64)) as u32);
            // dde87 mov ebx,DWORD PTR [rbp-0xa074]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f8c)) as u64) as u32 as u64;
            // dde8d jne 0xddeaa
            if crate::cov::br(fl.ne()) { pc = 0xddeaa; continue; }
            pc = 0xdde8f;
        }
        0xdde8f => {
            // block dde8f
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc dde8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dde8f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dde93 lea rsi,[rbp-0xa084]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5f7c);
            // dde9a lea rdx,[rbp-0xa088]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5f78);
            // ddea1 mov rdi,r12
            rdi = r12;
            // ddea4 call QWORD PTR [rax+0x1a00]
            { m.outregs(0xddeaa, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xddeaa;
        }
        0xddeaa => {
            // block ddeaa
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddeaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddeaa mov r15,QWORD PTR [r12+0x280]
            r15 = m.r64(r12.wrapping_add(0x280)) as u64;
            // ddeb2 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // ddeb5 test r15,r15
            fl = logic::<64>(r15 & r15);
            // ddeb8 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // ddebd jne 0xddf0a
            if crate::cov::br(fl.ne()) { pc = 0xddf0a; continue; }
            pc = 0xddebf;
        }
        0xddebf => {
            // block ddebf
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddebf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddebf cmp BYTE PTR [r12+0x279],0x0
            fl = sub::<8>(m.r8(r12.wrapping_add(0x279)) as u64, 0x0u64).1;
            // ddec8 je 0xddee5
            if crate::cov::br(fl.e()) { pc = 0xddee5; continue; }
            pc = 0xddeca;
        }
        0xddeca => {
            // block ddeca
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddeca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddeca mov edi,DWORD PTR [rbp-0xa088]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // dded0 imul edi,DWORD PTR [rbp-0xa090]
            { let (r, f) = imul::<32>((rdi as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f70)) as u64); rdi = (r) as u32 as u64; fl = f; }
            // dded7 shl rdi,0x2
            { let (r, f) = shl::<64>(rdi, 0x2u64, fl); rdi = r; fl = f; }
            // ddedb call 0xec140
            { m.outregs(0xddee0, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddee0 mov r15,rax
            r15 = rax;
            // ddee3 jmp 0xddf0a
            pc = 0xddf0a; continue;
        }
        0xddee5 => {
            // block ddee5
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddee5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddee5 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // ddee9 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // ddef1 lea rdx,[rbp-0xa038]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5fc8);
            // ddef8 mov rdi,r12
            rdi = r12;
            // ddefb call QWORD PTR [rax+0xa10]
            { m.outregs(0xddf01, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddf01 mov r15,QWORD PTR [rbp-0xa038]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5fc8)) as u64;
            // ddf08 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xddf0a;
        }
        0xddf0a => {
            // block ddf0a
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddf0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddf0a mov QWORD PTR [rbp-0xa0a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_5f60), (rax) as u64);
            // ddf11 lea rdi,[rbp-0xa030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // ddf18 mov esi,0x800
            rsi = (0x800u64) as u32 as u64;
            // ddf1d call 0x5932c
            { m.outregs(0xddf22, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x5932c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddf22 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // ddf24 je 0xddf80
            if crate::cov::br(fl.e()) { pc = 0xddf80; continue; }
            pc = 0xddf26;
        }
        0xddf26 => {
            // block ddf26
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddf26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddf26 lea r14,[rbp-0xa030]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // ddf2d lea r13,[rbp-0x8030]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // ddf34 mov DWORD PTR [rbp-0xa094],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f6c), ((rbx as u32 as u64)) as u32);
            pc = 0xddf3a;
        }
        0xddf3a => {
            // block ddf3a
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddf3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddf3a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // ddf3e mov ecx,DWORD PTR [rbp-0xa084]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c)) as u64) as u32 as u64;
            // ddf44 mov rdi,r12
            rdi = r12;
            // ddf47 mov rsi,r14
            rsi = r14;
            // ddf4a mov rdx,r13
            rdx = r13;
            // ddf4d call QWORD PTR [rax+0x19e0]
            { m.outregs(0xddf53, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddf53 mov edx,DWORD PTR [rbp-0xa088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // ddf59 shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ddf5c mov rdi,r15
            rdi = r15;
            // ddf5f mov rsi,r13
            rsi = r13;
            // ddf62 call 0xec21e
            { m.outregs(0xddf67, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ddf67 mov eax,DWORD PTR [rbp-0xa088]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // ddf6d lea r15,[r15+rax*4]
            r15 = r15.wrapping_add(rax.wrapping_mul(4));
            // ddf71 dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // ddf73 jne 0xddf3a
            if crate::cov::br(fl.ne()) { pc = 0xddf3a; continue; }
            pc = 0xddf75;
        }
        0xddf75 => {
            // block ddf75
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddf75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddf75 mov eax,DWORD PTR [rbp-0xa094]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f6c)) as u64) as u32 as u64;
            // ddf7b mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // ddf7e jmp 0xddf86
            pc = 0xddf86; continue;
        }
        0xddf80 => {
            // block ddf80
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddf80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddf80 mov DWORD PTR [rbp-0xa094],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f6c), ((rbx as u32 as u64)) as u32);
            pc = 0xddf86;
        }
        0xddf86 => {
            // block ddf86
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddf86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddf86 cmp r13d,DWORD PTR [rbp-0xa090]
            fl = sub::<32>((r13 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f70)) as u64).1;
            // ddf8d jae 0xde084
            if crate::cov::br(fl.ae()) { pc = 0xde084; continue; }
            pc = 0xddf93;
        }
        0xddf93 => {
            // block ddf93
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddf93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddf93 lea rbx,[rbp-0x8030]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            // ddf9a xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xddf9d;
        }
        0xddf9d => {
            // block ddf9d
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddf9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddf9d mov ecx,DWORD PTR [rbp-0xa084]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f7c)) as u64) as u32 as u64;
            // ddfa3 test ecx,0x7fffffff
            fl = logic::<32>((rcx as u32 as u64) & 0x7fffffffu64);
            // ddfa9 je 0xde03f
            if crate::cov::br(fl.e()) { pc = 0xde03f; continue; }
            pc = 0xddfaf;
        }
        0xddfaf => {
            // block ddfaf
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddfaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddfaf mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // ddfb2 sub eax,DWORD PTR [rbp-0xa094]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f6c)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // ddfb8 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // ddfba shr edx,0x2
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ddfbd imul rdx,rdx,0x4ad012b5
            { let (r, f) = imul::<64>(rdx, 0x4ad012b5u64); rdx = r; fl = f; }
            // ddfc4 shr rdx,0x26
            { let (r, f) = shr::<64>(rdx, 0x26u64, fl); rdx = r; fl = f; }
            // ddfc8 imul edx,edx,0x36c
            { let (r, f) = imul::<32>((rdx as u32 as u64), 0x36cu64); rdx = (r) as u32 as u64; fl = f; }
            // ddfce sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // ddfd0 lea edx,[rcx+rcx*1]
            rdx = (rcx.wrapping_add(rcx)) as u32 as u64;
            // ddfd3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xddfd5;
        }
        0xddfd5 => {
            // block ddfd5
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddfd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddfd5 test sil,0x1
            fl = logic::<8>((rsi as u8 as u64) & 0x1u64);
            // ddfd9 jne 0xddff2
            if crate::cov::br(fl.ne()) { pc = 0xddff2; continue; }
            pc = 0xddfdb;
        }
        0xddfdb => {
            // block ddfdb
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddfdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddfdb test sil,0x2
            fl = logic::<8>((rsi as u8 as u64) & 0x2u64);
            // ddfdf movzx edi,WORD PTR [r12+0x270]
            rdi = (m.r16(r12.wrapping_add(0x270)) as u64) as u32 as u64;
            // ddfe8 jne 0xde01a
            if crate::cov::br(fl.ne()) { pc = 0xde01a; continue; }
            pc = 0xddfea;
        }
        0xddfea => {
            // block ddfea
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddfea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddfea test dil,0x2
            fl = logic::<8>((rdi as u8 as u64) & 0x2u64);
            // ddfee jne 0xde02c
            if crate::cov::br(fl.ne()) { pc = 0xde02c; continue; }
            pc = 0xddff0;
        }
        0xddff0 => {
            // block ddff0
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddff0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddff0 jmp 0xde020
            pc = 0xde020; continue;
        }
        0xddff2 => {
            // block ddff2
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc ddff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ddff2 mov di,0x40
            rdi = (rdi & !0xffff) | ((0x40u64) as u16 as u64);
            // ddff6 test BYTE PTR [r12+0x270],0x1
            fl = logic::<8>(m.r8(r12.wrapping_add(0x270)) as u64 & 0x1u64);
            // ddfff je 0xde004
            if crate::cov::br(fl.e()) { pc = 0xde004; continue; }
            pc = 0xde001;
        }
        0xde001 => {
            // block de001
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de001 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de001 lea edi,[rax+0x40]
            rdi = (rax.wrapping_add(0x40)) as u32 as u64;
            pc = 0xde004;
        }
        0xde004 => {
            // block de004
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de004 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de004 mov WORD PTR [rbp+rsi*2-0xa030],di
            m.w16(rbp.wrapping_add(rsi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((rdi as u16 as u64)) as u16);
            // de00c cmp eax,0x36b
            fl = sub::<32>((rax as u32 as u64), 0x36bu64).1;
            // de011 lea eax,[rax+0x1]
            rax = (rax.wrapping_add(0x1)) as u32 as u64;
            // de014 cmovg eax,r14d
            if fl.g() { rax = ((r14 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // de018 jmp 0xde037
            pc = 0xde037; continue;
        }
        0xde01a => {
            // block de01a
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de01a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de01a test dil,0x4
            fl = logic::<8>((rdi as u8 as u64) & 0x4u64);
            // de01e jne 0xde02c
            if crate::cov::br(fl.ne()) { pc = 0xde02c; continue; }
            pc = 0xde020;
        }
        0xde020 => {
            // block de020
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de020 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de020 mov WORD PTR [rbp+rsi*2-0xa030],0x200
            m.w16(rbp.wrapping_add(rsi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), (0x200u64) as u16);
            // de02a jmp 0xde037
            pc = 0xde037; continue;
        }
        0xde02c => {
            // block de02c
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de02c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de02c lea edi,[rax+0x40]
            rdi = (rax.wrapping_add(0x40)) as u32 as u64;
            // de02f mov WORD PTR [rbp+rsi*2-0xa030],di
            m.w16(rbp.wrapping_add(rsi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((rdi as u16 as u64)) as u16);
            pc = 0xde037;
        }
        0xde037 => {
            // block de037
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de037 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de037 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // de03a cmp rsi,rdx
            fl = sub::<64>(rsi, rdx).1;
            // de03d jb 0xddfd5
            if crate::cov::br(fl.b()) { pc = 0xddfd5; continue; }
            pc = 0xde03f;
        }
        0xde03f => {
            // block de03f
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de03f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de03f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // de043 mov rdi,r12
            rdi = r12;
            // de046 lea rsi,[rbp-0xa030]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // de04d mov rdx,rbx
            rdx = rbx;
            // de050 call QWORD PTR [rax+0x19e0]
            { m.outregs(0xde056, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de056 mov edx,DWORD PTR [rbp-0xa088]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de05c shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // de05f mov rdi,r15
            rdi = r15;
            // de062 mov rsi,rbx
            rsi = rbx;
            // de065 call 0xec21e
            { m.outregs(0xde06a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de06a mov eax,DWORD PTR [rbp-0xa088]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64) as u32 as u64;
            // de070 lea r15,[r15+rax*4]
            r15 = r15.wrapping_add(rax.wrapping_mul(4));
            // de074 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // de077 cmp r13d,DWORD PTR [rbp-0xa090]
            fl = sub::<32>((r13 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f70)) as u64).1;
            // de07e jne 0xddf9d
            if crate::cov::br(fl.ne()) { pc = 0xddf9d; continue; }
            pc = 0xde084;
        }
        0xde084 => {
            // block de084
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de084 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de084 mov rbx,QWORD PTR [rbp-0xa0a0]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_5f60)) as u64;
            // de08b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // de08e je 0xde10d
            if crate::cov::br(fl.e()) { pc = 0xde10d; continue; }
            pc = 0xde090;
        }
        0xde090 => {
            // block de090
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de090 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de090 mov al,BYTE PTR [r12+0x279]
            rax = (rax & !0xff) | ((m.r8(r12.wrapping_add(0x279)) as u64) as u8 as u64);
            // de098 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // de09a je 0xde10d
            if crate::cov::br(fl.e()) { pc = 0xde10d; continue; }
            pc = 0xde09c;
        }
        0xde09c => {
            // block de09c
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de09c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de09c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // de0a0 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // de0a8 lea rdx,[rbp-0xa08c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_5f74);
            // de0af mov rdi,r12
            rdi = r12;
            // de0b2 call QWORD PTR [rax+0x538]
            { m.outregs(0xde0b8, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de0b8 mov r14d,DWORD PTR [rbp-0xa08c]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f74)) as u64) as u32 as u64;
            // de0bf mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // de0c3 mov rdi,r12
            rdi = r12;
            // de0c6 call QWORD PTR [rax+0x128]
            { m.outregs(0xde0cc, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de0cc cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // de0cf jbe 0xde0df
            if crate::cov::br(fl.be()) { pc = 0xde0df; continue; }
            pc = 0xde0d1;
        }
        0xde0d1 => {
            // block de0d1
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de0d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de0d1 mov DWORD PTR [rbp-0xa08c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_5f74), (0x0u64) as u32);
            // de0db xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // de0dd jmp 0xde0e5
            pc = 0xde0e5; continue;
        }
        0xde0df => {
            // block de0df
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de0df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de0df mov esi,DWORD PTR [rbp-0xa08c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f74)) as u64) as u32 as u64;
            pc = 0xde0e5;
        }
        0xde0e5 => {
            // block de0e5
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de0e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de0e5 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // de0e9 mov ecx,DWORD PTR [rbp-0xa090]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f70)) as u64) as u32 as u64;
            // de0ef imul ecx,DWORD PTR [rbp-0xa088]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_5f78)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // de0f6 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // de0f9 mov rdi,r12
            rdi = r12;
            // de0fc mov rdx,rbx
            rdx = rbx;
            // de0ff call QWORD PTR [rax+0x2c0]
            { m.outregs(0xde105, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de105 mov rdi,rbx
            rdi = rbx;
            // de108 call 0xec134
            { m.outregs(0xde10d, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xde10d;
        }
        0xde10d => {
            // block de10d
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de10d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de10d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // de111 movzx esi,BYTE PTR [r12+0x278]
            rsi = (m.r8(r12.wrapping_add(0x278)) as u64) as u32 as u64;
            // de11a mov rdi,r12
            rdi = r12;
            // de11d call QWORD PTR [rax+0x570]
            { m.outregs(0xde123, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de123 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // de127 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // de12f mov edx,DWORD PTR [r12+0x274]
            rdx = (m.r32(r12.wrapping_add(0x274)) as u64) as u32 as u64;
            // de137 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // de13c mov rdi,r12
            rdi = r12;
            // de13f call QWORD PTR [rax+0x318]
            { m.outregs(0xde145, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de145 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // de149 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // de151 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // de153 mov rdi,r12
            rdi = r12;
            // de156 call QWORD PTR [rax+0x4f8]
            { m.outregs(0xde15c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de15c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // de160 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // de168 mov rdi,r12
            rdi = r12;
            // de16b call QWORD PTR [rax+0x618]
            { m.outregs(0xde171, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // de171 mov rax,QWORD PTR [rip+0x62260] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // de178 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // de17b cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // de17f jne 0xde193
            if crate::cov::br(fl.ne()) { pc = 0xde193; continue; }
            pc = 0xde181;
        }
        0xde181 => {
            // block de181
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de181 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de181 add rsp,0xa078
            { let (r, f) = add::<64>(rsp, 0xa078u64);
              rsp = r;
              fl = f; }
            // de188 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // de189 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // de18b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // de18d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // de18f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // de191 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // de192 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xde193 => {
            // block de193
            #[cfg(feature = "trace")] eprintln!("f_000dddb0 pc de193 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de193 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1912a size=1008
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001912a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1912a;
    loop {
        match pc {
        0x1912a => {
            // block 1912a
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1912a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1912a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1912b mov rbp,rsp
            rbp = rsp;
            // 1912e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19130 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19132 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19133 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 19137 mov r14,rdx
            r14 = rdx;
            // 1913a mov rbx,rsi
            rbx = rsi;
            // 1913d mov rcx,QWORD PTR [rbx+0x60]
            rcx = m.r64(rbx.wrapping_add(0x60)) as u64;
            // 19141 mov rax,QWORD PTR [rbx+0x68]
            rax = m.r64(rbx.wrapping_add(0x68)) as u64;
            // 19145 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 19148 je 0x1915f
            if crate::cov::br(fl.e()) { pc = 0x1915f; continue; }
            pc = 0x1914a;
        }
        0x1914a => {
            // block 1914a
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1914a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1914a lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 1914e sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 19151 not rdx
            { let v = !rdx; rdx = v; }
            // 19154 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 19158 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 1915b mov QWORD PTR [rbx+0x68],rdx
            m.w64(rbx.wrapping_add(0x68), (rdx) as u64);
            pc = 0x1915f;
        }
        0x1915f => {
            // block 1915f
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1915f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1915f mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // 19163 mov rax,QWORD PTR [rbx+0x80]
            rax = m.r64(rbx.wrapping_add(0x80)) as u64;
            // 1916a cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 1916d je 0x19187
            if crate::cov::br(fl.e()) { pc = 0x19187; continue; }
            pc = 0x1916f;
        }
        0x1916f => {
            // block 1916f
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1916f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1916f lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19173 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 19176 not rdx
            { let v = !rdx; rdx = v; }
            // 19179 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 1917d add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 19180 mov QWORD PTR [rbx+0x80],rdx
            m.w64(rbx.wrapping_add(0x80), (rdx) as u64);
            pc = 0x19187;
        }
        0x19187 => {
            // block 19187
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19187 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19187 mov rcx,QWORD PTR [rbx+0x90]
            rcx = m.r64(rbx.wrapping_add(0x90)) as u64;
            // 1918e mov rax,QWORD PTR [rbx+0x98]
            rax = m.r64(rbx.wrapping_add(0x98)) as u64;
            // 19195 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 19198 je 0x191b2
            if crate::cov::br(fl.e()) { pc = 0x191b2; continue; }
            pc = 0x1919a;
        }
        0x1919a => {
            // block 1919a
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1919a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1919a lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 1919e sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 191a1 not rdx
            { let v = !rdx; rdx = v; }
            // 191a4 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 191a8 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 191ab mov QWORD PTR [rbx+0x98],rdx
            m.w64(rbx.wrapping_add(0x98), (rdx) as u64);
            pc = 0x191b2;
        }
        0x191b2 => {
            // block 191b2
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 191b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 191b2 mov rcx,QWORD PTR [rbx+0xa8]
            rcx = m.r64(rbx.wrapping_add(0xa8)) as u64;
            // 191b9 mov rax,QWORD PTR [rbx+0xb0]
            rax = m.r64(rbx.wrapping_add(0xb0)) as u64;
            // 191c0 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 191c3 je 0x191dd
            if crate::cov::br(fl.e()) { pc = 0x191dd; continue; }
            pc = 0x191c5;
        }
        0x191c5 => {
            // block 191c5
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 191c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 191c5 lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 191c9 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 191cc not rdx
            { let v = !rdx; rdx = v; }
            // 191cf and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 191d3 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 191d6 mov QWORD PTR [rbx+0xb0],rdx
            m.w64(rbx.wrapping_add(0xb0), (rdx) as u64);
            pc = 0x191dd;
        }
        0x191dd => {
            // block 191dd
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 191dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 191dd mov rcx,QWORD PTR [rbx+0xc0]
            rcx = m.r64(rbx.wrapping_add(0xc0)) as u64;
            // 191e4 mov rax,QWORD PTR [rbx+0xc8]
            rax = m.r64(rbx.wrapping_add(0xc8)) as u64;
            // 191eb cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 191ee je 0x19208
            if crate::cov::br(fl.e()) { pc = 0x19208; continue; }
            pc = 0x191f0;
        }
        0x191f0 => {
            // block 191f0
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 191f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 191f0 lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 191f4 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 191f7 not rdx
            { let v = !rdx; rdx = v; }
            // 191fa and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 191fe add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 19201 mov QWORD PTR [rbx+0xc8],rdx
            m.w64(rbx.wrapping_add(0xc8), (rdx) as u64);
            pc = 0x19208;
        }
        0x19208 => {
            // block 19208
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19208 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19208 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 1920a call 0xcf820
            { m.outregs(0x1920f, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcf820); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1920f test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 19212 je 0x1949e
            if crate::cov::br(fl.e()) { pc = 0x1949e; continue; }
            pc = 0x19218;
        }
        0x19218 => {
            // block 19218
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19218 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19218 lea r15,[rbx+0x60]
            r15 = rbx.wrapping_add(0x60);
            // 1921c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1921f lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 19223 mov esi,0x18
            rsi = (0x18u64) as u32 as u64;
            // 19228 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 1922d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 19230 mov rdi,r14
            rdi = r14;
            // 19233 call QWORD PTR [rax+0x38]
            { m.outregs(0x19236, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19236 mov DWORD PTR [rbp-0x20],0xbb80
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0xbb80u64) as u32);
            // 1923d mov rax,QWORD PTR [rbx+0x68]
            rax = m.r64(rbx.wrapping_add(0x68)) as u64;
            // 19241 cmp rax,QWORD PTR [rbx+0x70]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x70)) as u64).1;
            // 19245 jae 0x19257
            if crate::cov::br(fl.ae()) { pc = 0x19257; continue; }
            pc = 0x19247;
        }
        0x19247 => {
            // block 19247
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19247 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19247 mov DWORD PTR [rax],0xbb80
            m.w32(rax, (0xbb80u64) as u32);
            // 1924d add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 19251 mov QWORD PTR [rbx+0x68],rax
            m.w64(rbx.wrapping_add(0x68), (rax) as u64);
            // 19255 jmp 0x19263
            pc = 0x19263; continue;
        }
        0x19257 => {
            // block 19257
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19257 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19257 lea rsi,[rbp-0x20]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // 1925b mov rdi,r15
            rdi = r15;
            // 1925e call 0xeba14
            { m.outregs(0x19263, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x1e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x19263;
        }
        0x19263 => {
            // block 19263
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19263 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19263 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19265 call 0xcba78
            { m.outregs(0x1926a, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcba78); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1926a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1926c je 0x1929b
            if crate::cov::br(fl.e()) { pc = 0x1929b; continue; }
            pc = 0x1926e;
        }
        0x1926e => {
            // block 1926e
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1926e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1926e mov DWORD PTR [rbp-0x24],0x17700
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x17700u64) as u32);
            // 19275 mov rax,QWORD PTR [rbx+0x68]
            rax = m.r64(rbx.wrapping_add(0x68)) as u64;
            // 19279 cmp rax,QWORD PTR [rbx+0x70]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x70)) as u64).1;
            // 1927d jae 0x1928f
            if crate::cov::br(fl.ae()) { pc = 0x1928f; continue; }
            pc = 0x1927f;
        }
        0x1927f => {
            // block 1927f
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1927f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1927f mov DWORD PTR [rax],0x17700
            m.w32(rax, (0x17700u64) as u32);
            // 19285 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 19289 mov QWORD PTR [rbx+0x68],rax
            m.w64(rbx.wrapping_add(0x68), (rax) as u64);
            // 1928d jmp 0x1929b
            pc = 0x1929b; continue;
        }
        0x1928f => {
            // block 1928f
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1928f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1928f lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 19293 mov rdi,r15
            rdi = r15;
            // 19296 call 0xeba14
            { m.outregs(0x1929b, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x1e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1929b;
        }
        0x1929b => {
            // block 1929b
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1929b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1929b mov DWORD PTR [rbp-0x28],0x20
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x20u64) as u32);
            // 192a2 mov rax,QWORD PTR [rbx+0x98]
            rax = m.r64(rbx.wrapping_add(0x98)) as u64;
            // 192a9 cmp rax,QWORD PTR [rbx+0xa0]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0xa0)) as u64).1;
            // 192b0 jae 0x192c5
            if crate::cov::br(fl.ae()) { pc = 0x192c5; continue; }
            pc = 0x192b2;
        }
        0x192b2 => {
            // block 192b2
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 192b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 192b2 mov DWORD PTR [rax],0x20
            m.w32(rax, (0x20u64) as u32);
            // 192b8 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 192bc mov QWORD PTR [rbx+0x98],rax
            m.w64(rbx.wrapping_add(0x98), (rax) as u64);
            // 192c3 jmp 0x192d5
            pc = 0x192d5; continue;
        }
        0x192c5 => {
            // block 192c5
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 192c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 192c5 lea rdi,[rbx+0x90]
            rdi = rbx.wrapping_add(0x90);
            // 192cc lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 192d0 call 0xeba38
            { m.outregs(0x192d5, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x205); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x192d5;
        }
        0x192d5 => {
            // block 192d5
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 192d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 192d5 lea r14,[rbx+0xa8]
            r14 = rbx.wrapping_add(0xa8);
            // 192dc mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 192e3 mov rax,QWORD PTR [rbx+0xb0]
            rax = m.r64(rbx.wrapping_add(0xb0)) as u64;
            // 192ea cmp rax,QWORD PTR [rbx+0xb8]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0xb8)) as u64).1;
            // 192f1 jae 0x19306
            if crate::cov::br(fl.ae()) { pc = 0x19306; continue; }
            pc = 0x192f3;
        }
        0x192f3 => {
            // block 192f3
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 192f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 192f3 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // 192f9 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 192fd mov QWORD PTR [rbx+0xb0],rax
            m.w64(rbx.wrapping_add(0xb0), (rax) as u64);
            // 19304 jmp 0x19312
            pc = 0x19312; continue;
        }
        0x19306 => {
            // block 19306
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19306 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19306 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 1930a mov rdi,r14
            rdi = r14;
            // 1930d call 0xeb9f0
            { m.outregs(0x19312, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x1bd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x19312;
        }
        0x19312 => {
            // block 19312
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19312 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19312 test BYTE PTR [rbp-0x1a],0x20
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6)) as u64 & 0x20u64);
            // 19316 je 0x1934e
            if crate::cov::br(fl.e()) { pc = 0x1934e; continue; }
            pc = 0x19318;
        }
        0x19318 => {
            // block 19318
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19318 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19318 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 1931f mov rax,QWORD PTR [rbx+0xb0]
            rax = m.r64(rbx.wrapping_add(0xb0)) as u64;
            // 19326 cmp rax,QWORD PTR [rbx+0xb8]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0xb8)) as u64).1;
            // 1932d jae 0x19342
            if crate::cov::br(fl.ae()) { pc = 0x19342; continue; }
            pc = 0x1932f;
        }
        0x1932f => {
            // block 1932f
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1932f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1932f mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // 19335 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 19339 mov QWORD PTR [rbx+0xb0],rax
            m.w64(rbx.wrapping_add(0xb0), (rax) as u64);
            // 19340 jmp 0x1934e
            pc = 0x1934e; continue;
        }
        0x19342 => {
            // block 19342
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19342 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19342 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 19346 mov rdi,r14
            rdi = r14;
            // 19349 call 0xeb9f0
            { m.outregs(0x1934e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x1bd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1934e;
        }
        0x1934e => {
            // block 1934e
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1934e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1934e mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19350 call 0xcb7f5
            { m.outregs(0x19355, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcb7f5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19355 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 19357 je 0x1938f
            if crate::cov::br(fl.e()) { pc = 0x1938f; continue; }
            pc = 0x19359;
        }
        0x19359 => {
            // block 19359
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19359 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19359 mov DWORD PTR [rbp-0x34],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0x0u64) as u32);
            // 19360 mov rax,QWORD PTR [rbx+0xb0]
            rax = m.r64(rbx.wrapping_add(0xb0)) as u64;
            // 19367 cmp rax,QWORD PTR [rbx+0xb8]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0xb8)) as u64).1;
            // 1936e jae 0x19383
            if crate::cov::br(fl.ae()) { pc = 0x19383; continue; }
            pc = 0x19370;
        }
        0x19370 => {
            // block 19370
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19370 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19370 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // 19376 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 1937a mov QWORD PTR [rbx+0xb0],rax
            m.w64(rbx.wrapping_add(0xb0), (rax) as u64);
            // 19381 jmp 0x1938f
            pc = 0x1938f; continue;
        }
        0x19383 => {
            // block 19383
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19383 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19383 lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 19387 mov rdi,r14
            rdi = r14;
            // 1938a call 0xeb9f0
            { m.outregs(0x1938f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x1bd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1938f;
        }
        0x1938f => {
            // block 1938f
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1938f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1938f mov DWORD PTR [rbp-0x38],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u32);
            // 19396 mov rax,QWORD PTR [rbx+0xc8]
            rax = m.r64(rbx.wrapping_add(0xc8)) as u64;
            // 1939d cmp rax,QWORD PTR [rbx+0xd0]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0xd0)) as u64).1;
            // 193a4 jae 0x193b9
            if crate::cov::br(fl.ae()) { pc = 0x193b9; continue; }
            pc = 0x193a6;
        }
        0x193a6 => {
            // block 193a6
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 193a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 193a6 mov DWORD PTR [rax],0x0
            m.w32(rax, (0x0u64) as u32);
            // 193ac add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 193b0 mov QWORD PTR [rbx+0xc8],rax
            m.w64(rbx.wrapping_add(0xc8), (rax) as u64);
            // 193b7 jmp 0x193c9
            pc = 0x193c9; continue;
        }
        0x193b9 => {
            // block 193b9
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 193b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 193b9 lea rdi,[rbx+0xc0]
            rdi = rbx.wrapping_add(0xc0);
            // 193c0 lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 193c4 call 0xeb9f0
            { m.outregs(0x193c9, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x1bd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x193c9;
        }
        0x193c9 => {
            // block 193c9
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 193c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 193c9 lea r14,[rbx+0x78]
            r14 = rbx.wrapping_add(0x78);
            // 193cd mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 193cf call 0xcb923
            { m.outregs(0x193d4, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcb923); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 193d4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 193d6 je 0x1940e
            if crate::cov::br(fl.e()) { pc = 0x1940e; continue; }
            pc = 0x193d8;
        }
        0x193d8 => {
            // block 193d8
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 193d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 193d8 mov DWORD PTR [rbp-0x3c],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0x2u64) as u32);
            // 193df mov rax,QWORD PTR [rbx+0x80]
            rax = m.r64(rbx.wrapping_add(0x80)) as u64;
            // 193e6 cmp rax,QWORD PTR [rbx+0x88]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x88)) as u64).1;
            // 193ed jae 0x19402
            if crate::cov::br(fl.ae()) { pc = 0x19402; continue; }
            pc = 0x193ef;
        }
        0x193ef => {
            // block 193ef
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 193ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 193ef mov DWORD PTR [rax],0x2
            m.w32(rax, (0x2u64) as u32);
            // 193f5 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 193f9 mov QWORD PTR [rbx+0x80],rax
            m.w64(rbx.wrapping_add(0x80), (rax) as u64);
            // 19400 jmp 0x1940e
            pc = 0x1940e; continue;
        }
        0x19402 => {
            // block 19402
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19402 lea rsi,[rbp-0x3c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // 19406 mov rdi,r14
            rdi = r14;
            // 19409 call 0xeba5c
            { m.outregs(0x1940e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x229); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1940e;
        }
        0x1940e => {
            // block 1940e
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1940e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1940e mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19410 call 0xcb92b
            { m.outregs(0x19415, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcb92b); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19415 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 19417 je 0x1944f
            if crate::cov::br(fl.e()) { pc = 0x1944f; continue; }
            pc = 0x19419;
        }
        0x19419 => {
            // block 19419
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19419 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19419 mov DWORD PTR [rbp-0x40],0x6
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x6u64) as u32);
            // 19420 mov rax,QWORD PTR [rbx+0x80]
            rax = m.r64(rbx.wrapping_add(0x80)) as u64;
            // 19427 cmp rax,QWORD PTR [rbx+0x88]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x88)) as u64).1;
            // 1942e jae 0x19443
            if crate::cov::br(fl.ae()) { pc = 0x19443; continue; }
            pc = 0x19430;
        }
        0x19430 => {
            // block 19430
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19430 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19430 mov DWORD PTR [rax],0x6
            m.w32(rax, (0x6u64) as u32);
            // 19436 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 1943a mov QWORD PTR [rbx+0x80],rax
            m.w64(rbx.wrapping_add(0x80), (rax) as u64);
            // 19441 jmp 0x1944f
            pc = 0x1944f; continue;
        }
        0x19443 => {
            // block 19443
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19443 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19443 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 19447 mov rdi,r14
            rdi = r14;
            // 1944a call 0xeba5c
            { m.outregs(0x1944f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x229); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1944f;
        }
        0x1944f => {
            // block 1944f
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1944f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1944f mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19451 call 0xcb933
            { m.outregs(0x19456, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcb933); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19456 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 19458 je 0x19490
            if crate::cov::br(fl.e()) { pc = 0x19490; continue; }
            pc = 0x1945a;
        }
        0x1945a => {
            // block 1945a
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1945a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1945a mov DWORD PTR [rbp-0x44],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), (0x8u64) as u32);
            // 19461 mov rax,QWORD PTR [rbx+0x80]
            rax = m.r64(rbx.wrapping_add(0x80)) as u64;
            // 19468 cmp rax,QWORD PTR [rbx+0x88]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x88)) as u64).1;
            // 1946f jae 0x19484
            if crate::cov::br(fl.ae()) { pc = 0x19484; continue; }
            pc = 0x19471;
        }
        0x19471 => {
            // block 19471
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19471 mov DWORD PTR [rax],0x8
            m.w32(rax, (0x8u64) as u32);
            // 19477 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 1947b mov QWORD PTR [rbx+0x80],rax
            m.w64(rbx.wrapping_add(0x80), (rax) as u64);
            // 19482 jmp 0x19490
            pc = 0x19490; continue;
        }
        0x19484 => {
            // block 19484
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19484 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19484 lea rsi,[rbp-0x44]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // 19488 mov rdi,r14
            rdi = r14;
            // 1948b call 0xeba5c
            { m.outregs(0x19490, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x229); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x19490;
        }
        0x19490 => {
            // block 19490
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 19490 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19490 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19492 call 0xcf820
            { m.outregs(0x19497, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcf820); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19497 mov WORD PTR [rbx+0xd8],ax
            m.w16(rbx.wrapping_add(0xd8), ((rax as u16 as u64)) as u16);
            pc = 0x1949e;
        }
        0x1949e => {
            // block 1949e
            #[cfg(feature = "trace")] eprintln!("f_0001912a pc 1949e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1949e mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 194a0 call 0xcf682
            { m.outregs(0x194a5, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcf682); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 194a5 mov WORD PTR [rbx+0xda],ax
            m.w16(rbx.wrapping_add(0xda), ((rax as u16 as u64)) as u16);
            // 194ac mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 194ae call 0xcf556
            { m.outregs(0x194b3, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcf556); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 194b3 mov WORD PTR [rbx+0xdc],ax
            m.w16(rbx.wrapping_add(0xdc), ((rax as u16 as u64)) as u16);
            // 194ba mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 194bc call 0xcfc55
            { m.outregs(0x194c1, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcfc55); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 194c1 mov WORD PTR [rbx+0xde],ax
            m.w16(rbx.wrapping_add(0xde), ((rax as u16 as u64)) as u16);
            // 194c8 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 194ca call 0xd013c
            { m.outregs(0x194cf, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd013c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 194cf mov WORD PTR [rbx+0xe0],ax
            m.w16(rbx.wrapping_add(0xe0), ((rax as u16 as u64)) as u16);
            // 194d6 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 194d8 call 0xcf6db
            { m.outregs(0x194dd, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcf6db); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 194dd mov WORD PTR [rbx+0xe2],ax
            m.w16(rbx.wrapping_add(0xe2), ((rax as u16 as u64)) as u16);
            // 194e4 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 194e6 call 0xcf5fd
            { m.outregs(0x194eb, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcf5fd); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 194eb mov WORD PTR [rbx+0xe4],ax
            m.w16(rbx.wrapping_add(0xe4), ((rax as u16 as u64)) as u16);
            // 194f2 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 194f4 call 0xcfde4
            { m.outregs(0x194f9, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xcfde4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 194f9 mov WORD PTR [rbx+0xe6],ax
            m.w16(rbx.wrapping_add(0xe6), ((rax as u16 as u64)) as u16);
            // 19500 mov edi,DWORD PTR [rbx]
            rdi = (m.r32(rbx) as u64) as u32 as u64;
            // 19502 call 0xd01a9
            { m.outregs(0x19507, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xd01a9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19507 mov WORD PTR [rbx+0xe8],ax
            m.w16(rbx.wrapping_add(0xe8), ((rax as u16 as u64)) as u16);
            // 1950e add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 19512 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19513 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19515 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 19517 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19518 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10690 size=1138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010690<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x10690;
    loop {
        match pc {
        0x10690 => {
            // block 10690
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10690 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10691 mov rbp,rsp
            rbp = rsp;
            // 10694 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10696 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10698 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1069a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1069c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1069d sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 106a1 mov r15,rdi
            r15 = rdi;
            // 106a4 mov rax,QWORD PTR [r15+0x20]
            rax = m.r64(r15.wrapping_add(0x20)) as u64;
            // 106a8 cmp rax,0x400
            fl = sub::<64>(rax, 0x400u64).1;
            // 106ae jb 0x10752
            if crate::cov::br(fl.b()) { pc = 0x10752; continue; }
            pc = 0x106b4;
        }
        0x106b4 => {
            // block 106b4
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 106b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 106b4 add rax,0xfffffffffffffc00
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fc00u64);
              rax = r;
              fl = f; }
            // 106ba mov QWORD PTR [r15+0x20],rax
            m.w64(r15.wrapping_add(0x20), (rax) as u64);
            // 106be mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 106c2 mov r12,QWORD PTR [rbx]
            r12 = m.r64(rbx) as u64;
            // 106c5 lea r13,[rbx+0x8]
            r13 = rbx.wrapping_add(0x8);
            // 106c9 mov QWORD PTR [r15+0x8],r13
            m.w64(r15.wrapping_add(0x8), (r13) as u64);
            // 106cd mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 106d1 mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 106d5 cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // 106d8 jne 0x109e6
            if crate::cov::br(fl.ne()) { pc = 0x109e6; continue; }
            pc = 0x106de;
        }
        0x106de => {
            // block 106de
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 106de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 106de mov QWORD PTR [rbp-0x70],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r12) as u64);
            // 106e2 mov rdx,QWORD PTR [r15]
            rdx = m.r64(r15) as u64;
            // 106e5 mov rax,r13
            rax = r13;
            // 106e8 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 106eb jbe 0x10859
            if crate::cov::br(fl.be()) { pc = 0x10859; continue; }
            pc = 0x106f1;
        }
        0x106f1 => {
            // block 106f1
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 106f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 106f1 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 106f5 lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 106f9 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 106fd lea rax,[rax+rcx*1+0x1]
            rax = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 10702 sar rax,1
            { let (r, f) = sar::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 10705 mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 10709 lea rcx,[rax*8+0x0]
            rcx = rax.wrapping_mul(8);
            // 10711 mov QWORD PTR [rbp-0x78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rcx) as u64);
            // 10715 mov rdi,rbx
            rdi = rbx;
            // 10718 sub rdi,rcx
            { let (r, f) = sub::<64>(rdi, rcx);
              rdi = r;
              fl = f; }
            // 1071b add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 1071f sub r14,r13
            { let (r, f) = sub::<64>(r14, r13);
              r14 = r;
              fl = f; }
            // 10722 mov r12,r14
            r12 = r14;
            // 10725 shr r12,0x3
            { let (r, f) = shr::<64>(r12, 0x3u64, fl); r12 = r; fl = f; }
            // 10729 mov rsi,r13
            rsi = r13;
            // 1072c mov rdx,r14
            rdx = r14;
            // 1072f call 0xec224
            { m.outregs(0x10734, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10734 sub r12,QWORD PTR [rbp-0x80]
            { let (r, f) = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64);
              r12 = r;
              fl = f; }
            // 10738 lea r14,[rbx+r12*8+0x8]
            r14 = rbx.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x8);
            // 1073d mov QWORD PTR [r15+0x10],r14
            m.w64(r15.wrapping_add(0x10), (r14) as u64);
            // 10741 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 10745 sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10749 mov r12,QWORD PTR [rbp-0x70]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1074d jmp 0x109e6
            pc = 0x109e6; continue;
        }
        0x10752 => {
            // block 10752
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10752 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10752 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10756 mov r12,QWORD PTR [r15+0x18]
            r12 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 1075a mov rbx,r14
            rbx = r14;
            // 1075d sub rbx,QWORD PTR [r15+0x8]
            { let (r, f) = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64);
              rbx = r;
              fl = f; }
            // 10761 mov rax,r12
            rax = r12;
            // 10764 sub rax,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rax, m.r64(r15) as u64);
              rax = r;
              fl = f; }
            // 10767 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 1076a jae 0x10794
            if crate::cov::br(fl.ae()) { pc = 0x10794; continue; }
            pc = 0x1076c;
        }
        0x1076c => {
            // block 1076c
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 1076c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1076c mov edi,0x1000
            rdi = (0x1000u64) as u32 as u64;
            // 10771 call 0xec146
            { m.outregs(0x10776, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10776 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 10779 je 0x10908
            if crate::cov::br(fl.e()) { pc = 0x10908; continue; }
            pc = 0x1077f;
        }
        0x1077f => {
            // block 1077f
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 1077f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1077f mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 10783 lea rsi,[rbp-0x58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 10787 mov rdi,r15
            rdi = r15;
            // 1078a call 0xeb846
            { m.outregs(0x1078f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x13); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1078f jmp 0x109ee
            pc = 0x109ee; continue;
        }
        0x10794 => {
            // block 10794
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10794 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10794 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10798 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 1079e cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 107a2 and rbx,0xfffffffffffffff8
            { let r = rbx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 107a6 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 107aa mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 107ae lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 107b6 call 0xec146
            { m.outregs(0x107bb, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 107bb mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 107bf add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 107c2 mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 107c6 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 107ca lea rax,[rax+r14*8]
            rax = rax.wrapping_add(r14.wrapping_mul(8));
            // 107ce mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 107d2 mov edi,0x1000
            rdi = (0x1000u64) as u32 as u64;
            // 107d7 call 0xec146
            { m.outregs(0x107dc, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 107dc mov rbx,rax
            rbx = rax;
            // 107df mov QWORD PTR [rbp-0x68],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rbx) as u64);
            // 107e3 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 107e7 lea rsi,[rbp-0x68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 107eb call 0xeb858
            { m.outregs(0x107f0, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x25); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 107f0 mov rbx,QWORD PTR [r15+0x10]
            rbx = m.r64(r15.wrapping_add(0x10)) as u64;
            // 107f4 lea r14,[rbp-0x50]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            pc = 0x107f8;
        }
        0x107f8 => {
            // block 107f8
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 107f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 107f8 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 107fc cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 107ff je 0x10826
            if crate::cov::br(fl.e()) { pc = 0x10826; continue; }
            pc = 0x10801;
        }
        0x10801 => {
            // block 10801
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10801 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10801 add rbx,0xfffffffffffffff8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_fff8u64);
              rbx = r;
              fl = f; }
            // 10805 mov rdi,r14
            rdi = r14;
            // 10808 mov rsi,rbx
            rsi = rbx;
            // 1080b call 0xeb852
            { m.outregs(0x10810, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x1f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10810 jmp 0x107f8
            pc = 0x107f8; continue;
        }
        0x10812 => {
            // block 10812
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10812 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10812 mov r14,rax
            r14 = rax;
            pc = 0x10815;
        }
        0x10815 => {
            // block 10815
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10815 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10815 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10819 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1081e mov rdi,r14
            rdi = r14;
            // 10821 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x10826;
        }
        0x10826 => {
            // block 10826
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10826 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10826 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 10829 movaps xmm0,XMMWORD PTR [rbp-0x50]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0));
            // 1082d mov QWORD PTR [rbp-0x50],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rcx) as u64);
            // 10831 movups XMMWORD PTR [r15],xmm0
            m.w128(r15, x0);
            // 10835 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 10839 movaps xmm0,XMMWORD PTR [rbp-0x40]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0));
            // 1083d movups xmm1,XMMWORD PTR [r15+0x10]
            x1 = m.r128(r15.wrapping_add(0x10));
            // 10842 movups XMMWORD PTR [r15+0x10],xmm0
            m.w128(r15.wrapping_add(0x10), x0);
            // 10847 movaps XMMWORD PTR [rbp-0x40],xmm1
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x1);
            // 1084b lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1084f call 0xeb85e
            { m.outregs(0x10854, 0xd, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10854 jmp 0x109ee
            pc = 0x109ee; continue;
        }
        0x10859 => {
            // block 10859
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10859 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10859 mov QWORD PTR [rbp-0x80],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rcx) as u64);
            // 1085d mov rax,r14
            rax = r14;
            // 10860 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 10863 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10867 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 1086d cmovne r12,rax
            if fl.ne() { r12 = rax; }
            // 10871 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 1087b and rax,r12
            { let r = rax & r12; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 1087e mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 10882 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 10886 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1088a lea rdi,[r12*8+0x0]
            rdi = r12.wrapping_mul(8);
            // 10892 mov QWORD PTR [rbp-0x88],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdx) as u64);
            // 10899 call 0xec146
            { m.outregs(0x1089e, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1089e mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 108a2 mov rcx,QWORD PTR [rbp-0x78]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 108a6 lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 108aa mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 108ae mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 108b2 lea rdx,[rax+r12*8]
            rdx = rax.wrapping_add(r12.wrapping_mul(8));
            // 108b6 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 108ba cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 108bd je 0x109a8
            if crate::cov::br(fl.e()) { pc = 0x109a8; continue; }
            pc = 0x108c3;
        }
        0x108c3 => {
            // block 108c3
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 108c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 108c3 mov r12,QWORD PTR [rbp-0x70]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x108c7;
        }
        0x108c7 => {
            // block 108c7
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 108c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 108c7 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 108cb mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 108ce add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 108d2 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 108d6 mov rax,rbx
            rax = rbx;
            // 108d9 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 108dd cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 108e0 mov rbx,r13
            rbx = r13;
            // 108e3 mov r13,rax
            r13 = rax;
            // 108e6 jne 0x108c7
            if crate::cov::br(fl.ne()) { pc = 0x108c7; continue; }
            pc = 0x108e8;
        }
        0x108e8 => {
            // block 108e8
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 108e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 108e8 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 108eb mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 108ef mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 108f3 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 108f7 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 108fb mov rdi,QWORD PTR [r15+0x18]
            rdi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 108ff mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10903 jmp 0x109ba
            pc = 0x109ba; continue;
        }
        0x10908 => {
            // block 10908
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10908 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10908 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 1090c lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 10910 mov rdi,r15
            rdi = r15;
            // 10913 call 0xeb840
            { m.outregs(0x10918, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10918 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1091c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1091f lea r13,[rbx+0x8]
            r13 = rbx.wrapping_add(0x8);
            // 10923 mov QWORD PTR [r15+0x8],r13
            m.w64(r15.wrapping_add(0x8), (r13) as u64);
            // 10927 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 1092b mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 1092f cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // 10932 jne 0x10adf
            if crate::cov::br(fl.ne()) { pc = 0x10adf; continue; }
            pc = 0x10938;
        }
        0x10938 => {
            // block 10938
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10938 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10938 mov QWORD PTR [rbp-0x70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rax) as u64);
            // 1093c mov rdx,QWORD PTR [r15]
            rdx = m.r64(r15) as u64;
            // 1093f mov rax,r13
            rax = r13;
            // 10942 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 10945 jbe 0x109fd
            if crate::cov::br(fl.be()) { pc = 0x109fd; continue; }
            pc = 0x1094b;
        }
        0x1094b => {
            // block 1094b
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 1094b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1094b sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 1094f lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 10953 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 10957 lea rax,[rax+rcx*1+0x1]
            rax = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 1095c sar rax,1
            { let (r, f) = sar::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 1095f mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 10963 lea rcx,[rax*8+0x0]
            rcx = rax.wrapping_mul(8);
            // 1096b mov QWORD PTR [rbp-0x78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rcx) as u64);
            // 1096f mov rdi,rbx
            rdi = rbx;
            // 10972 sub rdi,rcx
            { let (r, f) = sub::<64>(rdi, rcx);
              rdi = r;
              fl = f; }
            // 10975 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 10979 sub r14,r13
            { let (r, f) = sub::<64>(r14, r13);
              r14 = r;
              fl = f; }
            // 1097c mov r12,r14
            r12 = r14;
            // 1097f shr r12,0x3
            { let (r, f) = shr::<64>(r12, 0x3u64, fl); r12 = r; fl = f; }
            // 10983 mov rsi,r13
            rsi = r13;
            // 10986 mov rdx,r14
            rdx = r14;
            // 10989 call 0xec224
            { m.outregs(0x1098e, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1098e sub r12,QWORD PTR [rbp-0x80]
            { let (r, f) = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64);
              r12 = r;
              fl = f; }
            // 10992 lea r14,[rbx+r12*8+0x8]
            r14 = rbx.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x8);
            // 10997 mov QWORD PTR [r15+0x10],r14
            m.w64(r15.wrapping_add(0x10), (r14) as u64);
            // 1099b mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1099f sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 109a3 jmp 0x10adb
            pc = 0x10adb; continue;
        }
        0x109a8 => {
            // block 109a8
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 109a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 109a8 mov rsi,rcx
            rsi = rcx;
            // 109ab mov r12,QWORD PTR [rbp-0x70]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 109af mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 109b3 mov rbx,QWORD PTR [rbp-0x88]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0x109ba;
        }
        0x109ba => {
            // block 109ba
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 109ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 109ba mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 109bd mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // 109c1 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 109c5 mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 109c9 mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 109cd mov QWORD PTR [rbp-0x40],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r14) as u64);
            // 109d1 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 109d5 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 109d9 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 109dd call 0xeb85e
            { m.outregs(0x109e2, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 109e2 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x109e6;
        }
        0x109e6 => {
            // block 109e6
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 109e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 109e6 mov QWORD PTR [r14],r12
            m.w64(r14, (r12) as u64);
            // 109e9 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            pc = 0x109ee;
        }
        0x109ee => {
            // block 109ee
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 109ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 109ee add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 109f2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 109f3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 109f5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 109f7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 109f9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 109fb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 109fc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x109fd => {
            // block 109fd
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 109fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 109fd mov QWORD PTR [rbp-0x80],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rcx) as u64);
            // 10a01 mov rax,r14
            rax = r14;
            // 10a04 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 10a07 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10a0b mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 10a11 cmovne r12,rax
            if fl.ne() { r12 = rax; }
            // 10a15 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 10a1f and rax,r12
            { let r = rax & r12; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 10a22 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 10a26 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 10a2a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 10a2e lea rdi,[r12*8+0x0]
            rdi = r12.wrapping_mul(8);
            // 10a36 mov QWORD PTR [rbp-0x88],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdx) as u64);
            // 10a3d call 0xec146
            { m.outregs(0x10a42, 0xd, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10a42 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 10a46 mov rcx,QWORD PTR [rbp-0x78]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 10a4a lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 10a4e mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10a52 mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 10a56 lea rdx,[rax+r12*8]
            rdx = rax.wrapping_add(r12.wrapping_mul(8));
            // 10a5a mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 10a5e cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 10a61 je 0x10aa1
            if crate::cov::br(fl.e()) { pc = 0x10aa1; continue; }
            pc = 0x10a63;
        }
        0x10a63 => {
            // block 10a63
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10a63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10a63 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 10a67 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 10a6a add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 10a6e mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10a72 mov rax,rbx
            rax = rbx;
            // 10a75 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 10a79 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 10a7c mov rbx,r13
            rbx = r13;
            // 10a7f mov r13,rax
            r13 = rax;
            // 10a82 jne 0x10a63
            if crate::cov::br(fl.ne()) { pc = 0x10a63; continue; }
            pc = 0x10a84;
        }
        0x10a84 => {
            // block 10a84
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10a84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10a84 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 10a87 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10a8b mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 10a8f mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 10a93 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10a97 mov rdi,QWORD PTR [r15+0x18]
            rdi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10a9b mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10a9f jmp 0x10aaf
            pc = 0x10aaf; continue;
        }
        0x10aa1 => {
            // block 10aa1
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10aa1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10aa1 mov rsi,rcx
            rsi = rcx;
            // 10aa4 mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 10aa8 mov rbx,QWORD PTR [rbp-0x88]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0x10aaf;
        }
        0x10aaf => {
            // block 10aaf
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10aaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10aaf mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 10ab2 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // 10ab6 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 10aba mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 10abe mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 10ac2 mov QWORD PTR [rbp-0x40],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r14) as u64);
            // 10ac6 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 10aca mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 10ace lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10ad2 call 0xeb85e
            { m.outregs(0x10ad7, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10ad7 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x10adb;
        }
        0x10adb => {
            // block 10adb
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10adb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10adb mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x10adf;
        }
        0x10adf => {
            // block 10adf
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10adf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10adf mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 10ae2 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10ae7 jmp 0x109ee
            pc = 0x109ee; continue;
        }
        0x10aec => {
            // block 10aec
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10aec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10aec jmp 0x10812
            pc = 0x10812; continue;
        }
        0x10af1 => {
            // block 10af1
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10af1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10af1 mov r14,rax
            r14 = rax;
            // 10af4 mov rdi,rbx
            rdi = rbx;
            // 10af7 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10afc jmp 0x10815
            pc = 0x10815; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdd560 size=1168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000dd560<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdd560;
    loop {
        match pc {
        0xdd560 => {
            // block dd560
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd560 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dd561 mov rbp,rsp
            rbp = rsp;
            // dd564 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dd566 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dd568 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dd56a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dd56c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dd56d sub rsp,0xc088
            { let (r, f) = sub::<64>(rsp, 0xc088u64);
              rsp = r;
              fl = f; }
            // dd574 mov r13,rdi
            r13 = rdi;
            // dd577 mov rax,QWORD PTR [rip+0x62e5a] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // dd57e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dd581 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // dd585 mov QWORD PTR [rbp-0xc038],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_3fc8), (0x0u64) as u64);
            // dd590 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // dd594 lea rsi,[rbp-0xc03c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_3fc4);
            // dd59b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dd59d call QWORD PTR [rax+0x3f0]
            { m.outregs(0xdd5a3, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd5a3 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // dd5a7 lea rsi,[rbp-0xc040]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_3fc0);
            // dd5ae xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dd5b0 mov rdi,r13
            rdi = r13;
            // dd5b3 call QWORD PTR [rax+0x388]
            { m.outregs(0xdd5b9, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd5b9 mov r15d,DWORD PTR [rbp-0xc040]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fc0)) as u64) as u32 as u64;
            // dd5c0 cmp r15d,0xa
            fl = sub::<32>((r15 as u32 as u64), 0xau64).1;
            // dd5c4 sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // dd5c7 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // dd5cb lea rsi,[rbp-0xc049]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_3fb7);
            // dd5d2 lea rdx,[rbp-0xc04a]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_3fb6);
            // dd5d9 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dd5db mov rdi,r13
            rdi = r13;
            // dd5de call QWORD PTR [rax+0x5e0]
            { m.outregs(0xdd5e4, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x5e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd5e4 mov esi,DWORD PTR [rbp-0xc03c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fc4)) as u64) as u32 as u64;
            // dd5ea mov edx,DWORD PTR [r13+0x274]
            rdx = (m.r32(r13.wrapping_add(0x274)) as u64) as u32 as u64;
            // dd5f1 movzx r9d,BYTE PTR [rbp-0xc04a]
            r9 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_3fb6)) as u64) as u32 as u64;
            // dd5f9 movzx ecx,BYTE PTR [rbp-0xc049]
            rcx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_3fb7)) as u64) as u32 as u64;
            // dd600 movzx r8d,bl
            r8 = ((rbx as u8 as u64)) as u32 as u64;
            // dd604 lea rdi,[rbp-0xc088]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_3f78);
            // dd60b call 0x32d57
            { m.outregs(0xdd610, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x32d57); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd610 mov eax,DWORD PTR [rbp-0xc084]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f7c)) as u64) as u32 as u64;
            // dd616 mov DWORD PTR [rbp-0xc048],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3fb8), ((rax as u32 as u64)) as u32);
            // dd61c mov eax,DWORD PTR [rbp-0xc080]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f80)) as u64) as u32 as u64;
            // dd622 mov DWORD PTR [rbp-0xc044],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc), ((rax as u32 as u64)) as u32);
            // dd628 movzx eax,WORD PTR [r13+0x270]
            rax = (m.r16(r13.wrapping_add(0x270)) as u64) as u32 as u64;
            // dd630 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // dd632 mov cx,0x40
            rcx = (rcx & !0xffff) | ((0x40u64) as u16 as u64);
            // dd636 mov r12w,0x3ac
            r12 = (r12 & !0xffff) | ((0x3acu64) as u16 as u64);
            // dd63b cmove r12w,cx
            if fl.e() { r12 = (r12 & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            // dd640 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // dd643 shl r14d,0x5
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x5u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // dd647 not r14d
            { let v = !(r14 as u32 as u64); r14 = (v) as u32 as u64; }
            // dd64a and r14d,0x40
            { let r = (r14 as u32 as u64) & 0x40u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // dd64e test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // dd650 mov ax,0x200
            rax = (rax & !0xffff) | ((0x200u64) as u16 as u64);
            // dd654 mov cx,0x3c0
            rcx = (rcx & !0xffff) | ((0x3c0u64) as u16 as u64);
            // dd658 cmove cx,ax
            if fl.e() { rcx = (rcx & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            // dd65c mov WORD PTR [rbp-0xc098],cx
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_3f68), ((rcx as u16 as u64)) as u16);
            // dd663 cmp r15d,0xa
            fl = sub::<32>((r15 as u32 as u64), 0xau64).1;
            // dd667 mov r15,r13
            r15 = r13;
            // dd66a mov r13d,DWORD PTR [rbp-0xc088]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f78)) as u64) as u32 as u64;
            // dd671 jne 0xdd68d
            if crate::cov::br(fl.ne()) { pc = 0xdd68d; continue; }
            pc = 0xdd673;
        }
        0xdd673 => {
            // block dd673
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd673 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd673 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dd676 lea rsi,[rbp-0xc048]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_3fb8);
            // dd67d lea rdx,[rbp-0xc044]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_3fbc);
            // dd684 mov rdi,r15
            rdi = r15;
            // dd687 call QWORD PTR [rax+0x1a00]
            { m.outregs(0xdd68d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdd68d;
        }
        0xdd68d => {
            // block dd68d
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd68d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd68d add r14d,0x1c0
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x1c0u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // dd694 mov DWORD PTR [rbp-0xc08c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3f74), (0x0u64) as u32);
            // dd69e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dd6a1 lea rdx,[rbp-0xc08c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_3f74);
            // dd6a8 mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // dd6ad mov ecx,0x800
            rcx = (0x800u64) as u32 as u64;
            // dd6b2 mov r8d,0xb
            r8 = (0xbu64) as u32 as u64;
            // dd6b8 mov rdi,r15
            rdi = r15;
            // dd6bb call QWORD PTR [rax+0x38]
            { m.outregs(0xdd6be, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd6be cmp DWORD PTR [r15+0x274],0x0
            fl = sub::<32>(m.r32(r15.wrapping_add(0x274)) as u64, 0x0u64).1;
            // dd6c6 jne 0xdd6fe
            if crate::cov::br(fl.ne()) { pc = 0xdd6fe; continue; }
            pc = 0xdd6c8;
        }
        0xdd6c8 => {
            // block dd6c8
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd6c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd6c8 mov eax,DWORD PTR [rbp-0xc08c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f74)) as u64) as u32 as u64;
            // dd6ce test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // dd6d0 je 0xdd6fe
            if crate::cov::br(fl.e()) { pc = 0xdd6fe; continue; }
            pc = 0xdd6d2;
        }
        0xdd6d2 => {
            // block dd6d2
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd6d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd6d2 and r13d,0xfffe
            { let r = (r13 as u32 as u64) & 0xfffeu64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // dd6d9 shr DWORD PTR [rbp-0xc048],1
            { let (r, f) = shr::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fb8)) as u64, 0x1u64, fl); m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3fb8), (r) as u32); fl = f; }
            // dd6df shr r13d,1
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x1u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // dd6e2 mov eax,DWORD PTR [rbp-0xc03c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fc4)) as u64) as u32 as u64;
            // dd6e8 lea ecx,[rax-0x2]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // dd6eb cmp ecx,0x4
            fl = sub::<32>((rcx as u32 as u64), 0x4u64).1;
            // dd6ee jb 0xdd6f8
            if crate::cov::br(fl.b()) { pc = 0xdd6f8; continue; }
            pc = 0xdd6f0;
        }
        0xdd6f0 => {
            // block dd6f0
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd6f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd6f0 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // dd6f2 jne 0xdd9bf
            if crate::cov::br(fl.ne()) { pc = 0xdd9bf; continue; }
            pc = 0xdd6f8;
        }
        0xdd6f8 => {
            // block dd6f8
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd6f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd6f8 shr DWORD PTR [rbp-0xc044],1
            { let (r, f) = shr::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc)) as u64, 0x1u64, fl); m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc), (r) as u32); fl = f; }
            pc = 0xdd6fe;
        }
        0xdd6fe => {
            // block dd6fe
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd6fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd6fe mov rbx,QWORD PTR [r15+0x280]
            rbx = m.r64(r15.wrapping_add(0x280)) as u64;
            // dd705 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dd707 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // dd70a jne 0xdd752
            if crate::cov::br(fl.ne()) { pc = 0xdd752; continue; }
            pc = 0xdd70c;
        }
        0xdd70c => {
            // block dd70c
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd70c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd70c cmp BYTE PTR [r15+0x279],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x279)) as u64, 0x0u64).1;
            // dd714 je 0xdd72f
            if crate::cov::br(fl.e()) { pc = 0xdd72f; continue; }
            pc = 0xdd716;
        }
        0xdd716 => {
            // block dd716
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd716 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd716 movzx edi,r13w
            rdi = ((r13 as u16 as u64)) as u32 as u64;
            // dd71a imul edi,DWORD PTR [rbp-0xc044]
            { let (r, f) = imul::<32>((rdi as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc)) as u64); rdi = (r) as u32 as u64; fl = f; }
            // dd721 shl rdi,0x2
            { let (r, f) = shl::<64>(rdi, 0x2u64, fl); rdi = r; fl = f; }
            // dd725 call 0xec140
            { m.outregs(0xdd72a, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd72a mov rbx,rax
            rbx = rax;
            // dd72d jmp 0xdd752
            pc = 0xdd752; continue;
        }
        0xdd72f => {
            // block dd72f
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd72f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd72f mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dd732 mov esi,DWORD PTR [r15+0x250]
            rsi = (m.r32(r15.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd739 lea rdx,[rbp-0xc038]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_3fc8);
            // dd740 mov rdi,r15
            rdi = r15;
            // dd743 call QWORD PTR [rax+0xa10]
            { m.outregs(0xdd749, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd749 mov rbx,QWORD PTR [rbp-0xc038]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_3fc8)) as u64;
            // dd750 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xdd752;
        }
        0xdd752 => {
            // block dd752
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd752 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd752 mov QWORD PTR [rbp-0xc0a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_3f58), (rax) as u64);
            // dd759 lea rdi,[rbp-0xa030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dd760 mov esi,0x800
            rsi = (0x800u64) as u32 as u64;
            // dd765 call 0x5932c
            { m.outregs(0xdd76a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x5932c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd76a mov WORD PTR [rbp-0xa030],r14w
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_5fd0), ((r14 as u16 as u64)) as u16);
            // dd772 mov WORD PTR [rbp-0xa02e],r12w
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_5fd2), ((r12 as u16 as u64)) as u16);
            // dd77a mov di,WORD PTR [rbp-0xc098]
            rdi = (rdi & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_3f68)) as u64) as u16 as u64);
            // dd781 mov WORD PTR [rbp-0xa02c],di
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_5fd4), ((rdi as u16 as u64)) as u16);
            // dd788 mov eax,DWORD PTR [rbp-0xc048]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fb8)) as u64) as u32 as u64;
            // dd78e lea ecx,[rax+rax*1-0x1]
            rcx = (rax.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // dd792 mov WORD PTR [rbp+rcx*2-0xa030],r12w
            m.w16(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((r12 as u16 as u64)) as u16);
            // dd79b lea ecx,[rax+rax*1-0x2]
            rcx = (rax.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // dd79f mov WORD PTR [rbp+rcx*2-0xa030],di
            m.w16(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((rdi as u16 as u64)) as u16);
            // dd7a7 movzx ecx,WORD PTR [r15+0x270]
            rcx = (m.r16(r15.wrapping_add(0x270)) as u64) as u32 as u64;
            // dd7af mov si,r12w
            rsi = (rsi & !0xffff) | (((r12 as u16 as u64)) as u16 as u64);
            // dd7b3 mov r12,r15
            r12 = r15;
            // dd7b6 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // dd7b9 lea ecx,[rax+rax*1-0x3]
            rcx = (rax.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_fffd)) as u32 as u64;
            // dd7bd mov dx,0x40
            rdx = (rdx & !0xffff) | ((0x40u64) as u16 as u64);
            // dd7c1 cmovne dx,si
            if fl.ne() { rdx = (rdx & !0xffff) | (((rsi as u16 as u64)) as u16 as u64); }
            // dd7c5 mov WORD PTR [rbp+rcx*2-0xa030],dx
            m.w16(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((rdx as u16 as u64)) as u16);
            // dd7cd lea eax,[rax+rax*1-0x4]
            rax = (rax.wrapping_add(rax).wrapping_add(0xffff_ffff_ffff_fffc)) as u32 as u64;
            // dd7d1 mov WORD PTR [rbp+rax*2-0xa030],r14w
            m.w16(rbp.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_5fd0), ((r14 as u16 as u64)) as u16);
            // dd7da movzx esi,si
            rsi = ((rsi as u16 as u64)) as u32 as u64;
            // dd7dd movzx edx,r14w
            rdx = ((r14 as u16 as u64)) as u32 as u64;
            // dd7e1 movzx ecx,di
            rcx = ((rdi as u16 as u64)) as u32 as u64;
            // dd7e4 lea r14,[rbp-0xc030]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_3fd0);
            // dd7eb mov r8d,0x800
            r8 = (0x800u64) as u32 as u64;
            // dd7f1 mov rdi,r14
            rdi = r14;
            // dd7f4 call 0x59372
            { m.outregs(0xdd7f9, 0x15, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x59372); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd7f9 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // dd7fd mov QWORD PTR [rbp-0xc098],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_3f68), (rax) as u64);
            // dd804 test r13w,r13w
            fl = logic::<16>((r13 as u16 as u64) & (r13 as u16 as u64));
            // dd808 je 0xdd8af
            if crate::cov::br(fl.e()) { pc = 0xdd8af; continue; }
            pc = 0xdd80e;
        }
        0xdd80e => {
            // block dd80e
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd80e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd80e mov rax,QWORD PTR [rbp-0xc098]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_3f68)) as u64;
            // dd815 lea eax,[rax-0x1]
            rax = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // dd818 mov DWORD PTR [rbp-0xc09c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3f64), ((rax as u32 as u64)) as u32);
            // dd81e xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // dd821 lea r13,[rbp-0x8030]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_7fd0);
            pc = 0xdd828;
        }
        0xdd828 => {
            // block dd828
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd828 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd828 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // dd82b mov rsi,r14
            rsi = r14;
            // dd82e je 0xdd865
            if crate::cov::br(fl.e()) { pc = 0xdd865; continue; }
            pc = 0xdd830;
        }
        0xdd830 => {
            // block dd830
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd830 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd830 cmp DWORD PTR [rbp-0xc09c],r15d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f64)) as u64, (r15 as u32 as u64)).1;
            // dd837 mov rsi,r14
            rsi = r14;
            // dd83a je 0xdd865
            if crate::cov::br(fl.e()) { pc = 0xdd865; continue; }
            pc = 0xdd83c;
        }
        0xdd83c => {
            // block dd83c
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd83c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd83c cmp r15d,DWORD PTR [rbp-0xc07c]
            fl = sub::<32>((r15 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f84)) as u64).1;
            // dd843 mov rsi,r14
            rsi = r14;
            // dd846 je 0xdd865
            if crate::cov::br(fl.e()) { pc = 0xdd865; continue; }
            pc = 0xdd848;
        }
        0xdd848 => {
            // block dd848
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd848 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd848 cmp DWORD PTR [rbp-0xc03c],0x5
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fc4)) as u64, 0x5u64).1;
            // dd84f lea rax,[rbp-0xa030]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_5fd0);
            // dd856 mov rsi,rax
            rsi = rax;
            // dd859 cmove rsi,r14
            if fl.e() { rsi = r14; }
            // dd85d cmp r15d,0x20
            fl = sub::<32>((r15 as u32 as u64), 0x20u64).1;
            // dd861 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            pc = 0xdd865;
        }
        0xdd865 => {
            // block dd865
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd865 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd865 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd869 mov ecx,DWORD PTR [rbp-0xc048]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fb8)) as u64) as u32 as u64;
            // dd86f mov rdi,r12
            rdi = r12;
            // dd872 mov rdx,r13
            rdx = r13;
            // dd875 call QWORD PTR [rax+0x19e0]
            { m.outregs(0xdd87b, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x19e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd87b mov edx,DWORD PTR [rbp-0xc044]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc)) as u64) as u32 as u64;
            // dd881 shl edx,0x2
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // dd884 mov rdi,rbx
            rdi = rbx;
            // dd887 mov rsi,r13
            rsi = r13;
            // dd88a call 0xec21e
            { m.outregs(0xdd88f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd88f mov eax,DWORD PTR [rbp-0xc044]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc)) as u64) as u32 as u64;
            // dd895 lea rbx,[rbx+rax*4]
            rbx = rbx.wrapping_add(rax.wrapping_mul(4));
            // dd899 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // dd89c movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // dd8a0 mov rcx,QWORD PTR [rbp-0xc098]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_3f68)) as u64;
            // dd8a7 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // dd8a9 jne 0xdd828
            if crate::cov::br(fl.ne()) { pc = 0xdd828; continue; }
            pc = 0xdd8af;
        }
        0xdd8af => {
            // block dd8af
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd8af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd8af mov r15,QWORD PTR [rbp-0xc0a8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_3f58)) as u64;
            // dd8b6 test r15,r15
            fl = logic::<64>(r15 & r15);
            // dd8b9 je 0xdd939
            if crate::cov::br(fl.e()) { pc = 0xdd939; continue; }
            pc = 0xdd8bb;
        }
        0xdd8bb => {
            // block dd8bb
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd8bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd8bb mov al,BYTE PTR [r12+0x279]
            rax = (rax & !0xff) | ((m.r8(r12.wrapping_add(0x279)) as u64) as u8 as u64);
            // dd8c3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dd8c5 je 0xdd939
            if crate::cov::br(fl.e()) { pc = 0xdd939; continue; }
            pc = 0xdd8c7;
        }
        0xdd8c7 => {
            // block dd8c7
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd8c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd8c7 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd8cb mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd8d3 lea rdx,[rbp-0xc090]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_3f70);
            // dd8da mov rdi,r12
            rdi = r12;
            // dd8dd call QWORD PTR [rax+0x538]
            { m.outregs(0xdd8e3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd8e3 mov r14d,DWORD PTR [rbp-0xc090]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f70)) as u64) as u32 as u64;
            // dd8ea mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd8ee mov rdi,r12
            rdi = r12;
            // dd8f1 call QWORD PTR [rax+0x128]
            { m.outregs(0xdd8f7, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd8f7 cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // dd8fa jbe 0xdd90a
            if crate::cov::br(fl.be()) { pc = 0xdd90a; continue; }
            pc = 0xdd8fc;
        }
        0xdd8fc => {
            // block dd8fc
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd8fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd8fc mov DWORD PTR [rbp-0xc090],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3f70), (0x0u64) as u32);
            // dd906 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dd908 jmp 0xdd910
            pc = 0xdd910; continue;
        }
        0xdd90a => {
            // block dd90a
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd90a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd90a mov esi,DWORD PTR [rbp-0xc090]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3f70)) as u64) as u32 as u64;
            pc = 0xdd910;
        }
        0xdd910 => {
            // block dd910
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd910 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd910 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd914 mov rcx,QWORD PTR [rbp-0xc098]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_3f68)) as u64;
            // dd91b imul ecx,DWORD PTR [rbp-0xc044]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // dd922 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // dd925 mov rdi,r12
            rdi = r12;
            // dd928 mov rdx,r15
            rdx = r15;
            // dd92b call QWORD PTR [rax+0x2c0]
            { m.outregs(0xdd931, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x2c0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd931 mov rdi,r15
            rdi = r15;
            // dd934 call 0xec134
            { m.outregs(0xdd939, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdd939;
        }
        0xdd939 => {
            // block dd939
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd939 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd939 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd93d movzx esi,BYTE PTR [r12+0x278]
            rsi = (m.r8(r12.wrapping_add(0x278)) as u64) as u32 as u64;
            // dd946 mov rdi,r12
            rdi = r12;
            // dd949 call QWORD PTR [rax+0x570]
            { m.outregs(0xdd94f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd94f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd953 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd95b mov edx,DWORD PTR [r12+0x274]
            rdx = (m.r32(r12.wrapping_add(0x274)) as u64) as u32 as u64;
            // dd963 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dd968 mov rdi,r12
            rdi = r12;
            // dd96b call QWORD PTR [rax+0x318]
            { m.outregs(0xdd971, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd971 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd975 mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd97d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dd97f mov rdi,r12
            rdi = r12;
            // dd982 call QWORD PTR [rax+0x4f8]
            { m.outregs(0xdd988, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd988 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // dd98c mov esi,DWORD PTR [r12+0x250]
            rsi = (m.r32(r12.wrapping_add(0x250)) as u64) as u32 as u64;
            // dd994 mov rdi,r12
            rdi = r12;
            // dd997 call QWORD PTR [rax+0x618]
            { m.outregs(0xdd99d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd99d mov rax,QWORD PTR [rip+0x62a34] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // dd9a4 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dd9a7 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // dd9ab jne 0xdd9eb
            if crate::cov::br(fl.ne()) { pc = 0xdd9eb; continue; }
            pc = 0xdd9ad;
        }
        0xdd9ad => {
            // block dd9ad
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd9ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd9ad add rsp,0xc088
            { let (r, f) = add::<64>(rsp, 0xc088u64);
              rsp = r;
              fl = f; }
            // dd9b4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dd9b5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dd9b7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dd9b9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dd9bb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dd9bd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dd9be ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdd9bf => {
            // block dd9bf
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd9bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd9bf cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // dd9c2 jne 0xdd9d3
            if crate::cov::br(fl.ne()) { pc = 0xdd9d3; continue; }
            pc = 0xdd9c4;
        }
        0xdd9c4 => {
            // block dd9c4
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd9c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd9c4 mov DWORD PTR [rbp-0xc044],0x1b0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_3fbc), (0x1b0u64) as u32);
            // dd9ce jmp 0xdd6fe
            pc = 0xdd6fe; continue;
        }
        0xdd9d3 => {
            // block dd9d3
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd9d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd9d3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // dd9d6 lea rsi,[rip+0x5b9bf] # 0x13939c
            rsi = m.env.data_va(318313);
            // dd9dd mov rdi,r15
            rdi = r15;
            // dd9e0 call QWORD PTR [rax+0x170]
            { m.outregs(0xdd9e6, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x170)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dd9e6 jmp 0xdd6fe
            pc = 0xdd6fe; continue;
        }
        0xdd9eb => {
            // block dd9eb
            #[cfg(feature = "trace")] eprintln!("f_000dd560 pc dd9eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd9eb call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc57e size=1232
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc57e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbc57e;
    loop {
        match pc {
        0xbc57e => {
            // block bc57e
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc57e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc57e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc57f mov rbp,rsp
            rbp = rsp;
            // bc582 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc584 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc586 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bc588 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc58a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc58b sub rsp,0x108
            { let (r, f) = sub::<64>(rsp, 0x108u64);
              rsp = r;
              fl = f; }
            // bc592 mov rbx,rsi
            rbx = rsi;
            // bc595 mov rax,rdi
            rax = rdi;
            // bc598 xorps xmm0,xmm0
            x0 = 0;
            // bc59b movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // bc59f mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // bc5a7 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // bc5ae mov rsi,rax
            rsi = rax;
            // bc5b1 call 0xbbdbe
            { m.outregs(0xbc5b6, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xbbdbe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc5b6 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // bc5bd lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // bc5c4 call 0xbbe30
            { m.outregs(0xbc5c9, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xbbe30); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc5c9 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // bc5d0 call 0xec032
            { m.outregs(0xbc5d5, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc5d5 mov rdi,rbx
            rdi = rbx;
            // bc5d8 mov QWORD PTR [rbp-0x130],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rdi) as u64);
            // bc5df mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // bc5e2 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // bc5e6 call rax
            { m.outregs(0xbc5e8, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc5e8 lea rsi,[rip+0x7b898] # 0x137e87
            rsi = m.env.data_va(312916);
            // bc5ef lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // bc5f6 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // bc5fb call 0xebfe4
            { m.outregs(0xbc600, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc600 mov rbx,QWORD PTR [rbp-0x80]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // bc604 jmp 0xbc613
            pc = 0xbc613; continue;
        }
        0xbc606 => {
            // block bc606
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc606 add rdi,0xffffffffffffffe8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffe8u64);
              rdi = r;
              fl = f; }
            // bc60a mov QWORD PTR [rbp-0x78],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rdi) as u64);
            // bc60e call 0xec032
            { m.outregs(0xbc613, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc613;
        }
        0xbc613 => {
            // block bc613
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc613 mov rdi,QWORD PTR [rbp-0x78]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // bc617 cmp rdi,rbx
            fl = sub::<64>(rdi, rbx).1;
            // bc61a jne 0xbc606
            if crate::cov::br(fl.ne()) { pc = 0xbc606; continue; }
            pc = 0xbc61c;
        }
        0xbc61c => {
            // block bc61c
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc61c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc61c xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // bc61f lea r13,[rbp-0x60]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // bc623 jmp 0xbc62b
            pc = 0xbc62b; continue;
        }
        0xbc625 => {
            // block bc625
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc625 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc625 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // bc628 mov r9,rbx
            r9 = rbx;
            pc = 0xbc62b;
        }
        0xbc62b => {
            // block bc62b
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc62b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc62b movzx edx,BYTE PTR [rbp-0x98]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64) as u32 as u64;
            // bc632 mov r8b,dl
            r8 = (r8 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // bc635 and r8b,0x1
            { let r = (r8 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r8 = (r8 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // bc639 jne 0xbc64a
            if crate::cov::br(fl.ne()) { pc = 0xbc64a; continue; }
            pc = 0xbc63b;
        }
        0xbc63b => {
            // block bc63b
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc63b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc63b mov rdi,rdx
            rdi = rdx;
            // bc63e shr rdi,1
            { let (r, f) = shr::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // bc641 lea r11,[rbp-0x97]
            r11 = rbp.wrapping_add(0xffff_ffff_ffff_ff69);
            // bc648 jmp 0xbc658
            pc = 0xbc658; continue;
        }
        0xbc64a => {
            // block bc64a
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc64a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc64a mov rdi,QWORD PTR [rbp-0x90]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // bc651 mov r11,QWORD PTR [rbp-0x88]
            r11 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0xbc658;
        }
        0xbc658 => {
            // block bc658
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc658 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc658 movzx esi,BYTE PTR [rbp-0xc8]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff38)) as u64) as u32 as u64;
            // bc65f test sil,0x1
            fl = logic::<8>((rsi as u8 as u64) & 0x1u64);
            // bc663 jne 0xbc671
            if crate::cov::br(fl.ne()) { pc = 0xbc671; continue; }
            pc = 0xbc665;
        }
        0xbc665 => {
            // block bc665
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc665 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // bc668 lea r15,[rbp-0xc7]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff39);
            // bc66f jmp 0xbc67f
            pc = 0xbc67f; continue;
        }
        0xbc671 => {
            // block bc671
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc671 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc671 mov rsi,QWORD PTR [rbp-0xc0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // bc678 mov r15,QWORD PTR [rbp-0xb8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            pc = 0xbc67f;
        }
        0xbc67f => {
            // block bc67f
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc67f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc67f cmp rdi,r9
            fl = sub::<64>(rdi, r9).1;
            // bc682 jbe 0xbc759
            if crate::cov::br(fl.be()) { pc = 0xbc759; continue; }
            pc = 0xbc688;
        }
        0xbc688 => {
            // block bc688
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc688 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc688 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // bc68b je 0xbc759
            if crate::cov::br(fl.e()) { pc = 0xbc759; continue; }
            pc = 0xbc691;
        }
        0xbc691 => {
            // block bc691
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc691 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc691 cmp rdi,r9
            fl = sub::<64>(rdi, r9).1;
            // bc694 je 0xbc759
            if crate::cov::br(fl.e()) { pc = 0xbc759; continue; }
            pc = 0xbc69a;
        }
        0xbc69a => {
            // block bc69a
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc69a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc69a add rdi,r11
            { let (r, f) = add::<64>(rdi, r11);
              rdi = r;
              fl = f; }
            // bc69d lea rbx,[r11+r9*1]
            rbx = r11.wrapping_add(r9);
            pc = 0xbc6a1;
        }
        0xbc6a1 => {
            // block bc6a1
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc6a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc6a1 mov al,BYTE PTR [rbx]
            rax = (rax & !0xff) | ((m.r8(rbx) as u64) as u8 as u64);
            // bc6a3 mov rcx,rsi
            rcx = rsi;
            // bc6a6 mov r10,r15
            r10 = r15;
            pc = 0xbc6a9;
        }
        0xbc6a9 => {
            // block bc6a9
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc6a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc6a9 movzx r12d,BYTE PTR [r10]
            r12 = (m.r8(r10) as u64) as u32 as u64;
            // bc6ad movzx r14d,al
            r14 = ((rax as u8 as u64)) as u32 as u64;
            // bc6b1 cmp r14d,r12d
            fl = sub::<32>((r14 as u32 as u64), (r12 as u32 as u64)).1;
            // bc6b4 je 0xbc6cb
            if crate::cov::br(fl.e()) { pc = 0xbc6cb; continue; }
            pc = 0xbc6b6;
        }
        0xbc6b6 => {
            // block bc6b6
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc6b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc6b6 inc r10
            { let (r, f) = inc::<64>(r10, fl); r10 = r; fl = f; }
            // bc6b9 dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // bc6bc jne 0xbc6a9
            if crate::cov::br(fl.ne()) { pc = 0xbc6a9; continue; }
            pc = 0xbc6be;
        }
        0xbc6be => {
            // block bc6be
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc6be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc6be inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // bc6c1 cmp rbx,rdi
            fl = sub::<64>(rbx, rdi).1;
            // bc6c4 jne 0xbc6a1
            if crate::cov::br(fl.ne()) { pc = 0xbc6a1; continue; }
            pc = 0xbc6c6;
        }
        0xbc6c6 => {
            // block bc6c6
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc6c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc6c6 jmp 0xbc759
            pc = 0xbc759; continue;
        }
        0xbc6cb => {
            // block bc6cb
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc6cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc6cb cmp rbx,rdi
            fl = sub::<64>(rbx, rdi).1;
            // bc6ce je 0xbc759
            if crate::cov::br(fl.e()) { pc = 0xbc759; continue; }
            pc = 0xbc6d4;
        }
        0xbc6d4 => {
            // block bc6d4
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc6d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc6d4 sub rbx,r11
            { let (r, f) = sub::<64>(rbx, r11);
              rbx = r;
              fl = f; }
            // bc6d7 cmp rbx,0xffffffffffffffff
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffffu64).1;
            // bc6db je 0xbc759
            if crate::cov::br(fl.e()) { pc = 0xbc759; continue; }
            pc = 0xbc6dd;
        }
        0xbc6dd => {
            // block bc6dd
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc6dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc6dd mov rdx,rbx
            rdx = rbx;
            // bc6e0 sub rdx,r9
            { let (r, f) = sub::<64>(rdx, r9);
              rdx = r;
              fl = f; }
            // bc6e3 je 0xbc625
            if crate::cov::br(fl.e()) { pc = 0xbc625; continue; }
            pc = 0xbc6e9;
        }
        0xbc6e9 => {
            // block bc6e9
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc6e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc6e9 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // bc6ec mov rsi,QWORD PTR [rbp-0x88]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // bc6f3 lea rax,[rbp-0x97]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff69);
            // bc6fa cmove rsi,rax
            if fl.e() { rsi = rax; }
            // bc6fe add rsi,r9
            { let (r, f) = add::<64>(rsi, r9);
              rsi = r;
              fl = f; }
            // bc701 mov rdi,r13
            rdi = r13;
            // bc704 call 0xebfe4
            { m.outregs(0xbc709, 0x2f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc709 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // bc70d cmp rax,QWORD PTR [rbp-0x70]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64).1;
            // bc711 jae 0xbc740
            if crate::cov::br(fl.ae()) { pc = 0xbc740; continue; }
            pc = 0xbc713;
        }
        0xbc713 => {
            // block bc713
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc713 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc713 mov rcx,QWORD PTR [rbp-0x50]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // bc717 mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // bc71b mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // bc71f mov rdx,QWORD PTR [rbp-0x58]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // bc723 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // bc727 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // bc72a xorps xmm0,xmm0
            x0 = 0;
            // bc72d movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // bc731 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // bc739 add QWORD PTR [rbp-0x78],0x18
            { let (r, f) = add::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64, 0x18u64);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (r) as u64);
              fl = f; }
            // bc73e jmp 0xbc74c
            pc = 0xbc74c; continue;
        }
        0xbc740 => {
            // block bc740
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc740 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc740 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // bc744 mov rsi,r13
            rsi = r13;
            // bc747 call 0xebe8e
            { m.outregs(0xbc74c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x65b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc74c;
        }
        0xbc74c => {
            // block bc74c
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc74c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc74c mov rdi,r13
            rdi = r13;
            // bc74f call 0xec032
            { m.outregs(0xbc754, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc754 jmp 0xbc625
            pc = 0xbc625; continue;
        }
        0xbc759 => {
            // block bc759
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc759 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc759 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // bc75c jne 0xbc763
            if crate::cov::br(fl.ne()) { pc = 0xbc763; continue; }
            pc = 0xbc75e;
        }
        0xbc75e => {
            // block bc75e
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc75e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc75e shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // bc761 jmp 0xbc76a
            pc = 0xbc76a; continue;
        }
        0xbc763 => {
            // block bc763
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc763 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc763 mov rdx,QWORD PTR [rbp-0x90]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            pc = 0xbc76a;
        }
        0xbc76a => {
            // block bc76a
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc76a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc76a sub rdx,r9
            { let (r, f) = sub::<64>(rdx, r9);
              rdx = r;
              fl = f; }
            // bc76d je 0xbc7da
            if crate::cov::br(fl.e()) { pc = 0xbc7da; continue; }
            pc = 0xbc76f;
        }
        0xbc76f => {
            // block bc76f
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc76f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc76f test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // bc772 lea rsi,[rbp-0x97]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff69);
            // bc779 cmovne rsi,QWORD PTR [rbp-0x88]
            if fl.ne() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64; }
            // bc781 add rsi,r9
            { let (r, f) = add::<64>(rsi, r9);
              rsi = r;
              fl = f; }
            // bc784 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc788 call 0xebfe4
            { m.outregs(0xbc78d, 0x27, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc78d mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // bc791 cmp rax,QWORD PTR [rbp-0x70]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64).1;
            // bc795 jae 0xbc7c4
            if crate::cov::br(fl.ae()) { pc = 0xbc7c4; continue; }
            pc = 0xbc797;
        }
        0xbc797 => {
            // block bc797
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc797 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc797 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // bc79b mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // bc79f mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // bc7a3 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bc7a7 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // bc7ab mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // bc7ae xorps xmm0,xmm0
            x0 = 0;
            // bc7b1 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // bc7b5 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // bc7bd add QWORD PTR [rbp-0x78],0x18
            { let (r, f) = add::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64, 0x18u64);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (r) as u64);
              fl = f; }
            // bc7c2 jmp 0xbc7d1
            pc = 0xbc7d1; continue;
        }
        0xbc7c4 => {
            // block bc7c4
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc7c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc7c4 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // bc7c8 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc7cc call 0xebe8e
            { m.outregs(0xbc7d1, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x65b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc7d1;
        }
        0xbc7d1 => {
            // block bc7d1
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc7d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc7d1 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc7d5 call 0xec032
            { m.outregs(0xbc7da, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc7da;
        }
        0xbc7da => {
            // block bc7da
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc7da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc7da lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            // bc7e1 call 0xec032
            { m.outregs(0xbc7e6, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc7e6 mov r12,QWORD PTR [rbp-0x80]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // bc7ea mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // bc7ec cmp r12,QWORD PTR [rbp-0x78]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64).1;
            // bc7f0 je 0xbc989
            if crate::cov::br(fl.e()) { pc = 0xbc989; continue; }
            pc = 0xbc7f6;
        }
        0xbc7f6 => {
            // block bc7f6
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc7f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc7f6 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // bc7fd lea r14,[rbp-0x128]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // bc804 lea r15,[rip+0x7b67f] # 0x137e8a
            r15 = m.env.data_va(312919);
            pc = 0xbc80b;
        }
        0xbc80b => {
            // block bc80b
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc80b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc80b movzx edx,BYTE PTR [r12]
            rdx = (m.r8(r12) as u64) as u32 as u64;
            // bc810 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // bc813 jne 0xbc81f
            if crate::cov::br(fl.ne()) { pc = 0xbc81f; continue; }
            pc = 0xbc815;
        }
        0xbc815 => {
            // block bc815
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc815 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc815 lea rax,[r12+0x1]
            rax = r12.wrapping_add(0x1);
            // bc81a shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // bc81d jmp 0xbc829
            pc = 0xbc829; continue;
        }
        0xbc81f => {
            // block bc81f
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc81f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc81f mov rdx,QWORD PTR [r12+0x8]
            rdx = m.r64(r12.wrapping_add(0x8)) as u64;
            // bc824 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            pc = 0xbc829;
        }
        0xbc829 => {
            // block bc829
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc829 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc829 cmp rdx,0x3
            fl = sub::<64>(rdx, 0x3u64).1;
            // bc82d jb 0xbc987
            if crate::cov::br(fl.b()) { pc = 0xbc987; continue; }
            pc = 0xbc833;
        }
        0xbc833 => {
            // block bc833
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc833 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc833 cmp rdx,0x3
            fl = sub::<64>(rdx, 0x3u64).1;
            // bc837 jl 0xbc987
            if crate::cov::br(fl.l()) { pc = 0xbc987; continue; }
            pc = 0xbc83d;
        }
        0xbc83d => {
            // block bc83d
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc83d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc83d mov rcx,rdx
            rcx = rdx;
            // bc840 add rcx,0xfffffffffffffffe
            { let (r, f) = add::<64>(rcx, 0xffff_ffff_ffff_fffeu64);
              rcx = r;
              fl = f; }
            // bc844 je 0xbc987
            if crate::cov::br(fl.e()) { pc = 0xbc987; continue; }
            pc = 0xbc84a;
        }
        0xbc84a => {
            // block bc84a
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc84a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc84a lea r8,[rax+rdx*1]
            r8 = rax.wrapping_add(rdx);
            // bc84e lea rdx,[rax+rdx*1-0x2]
            rdx = rax.wrapping_add(rdx).wrapping_add(0xffff_ffff_ffff_fffe);
            // bc853 mov r13,rax
            r13 = rax;
            pc = 0xbc856;
        }
        0xbc856 => {
            // block bc856
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc856 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc856 movzx esi,BYTE PTR [r13+0x0]
            rsi = (m.r8(r13) as u64) as u32 as u64;
            // bc85b cmp esi,0x3c
            fl = sub::<32>((rsi as u32 as u64), 0x3cu64).1;
            // bc85e mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // bc863 jne 0xbc87d
            if crate::cov::br(fl.ne()) { pc = 0xbc87d; continue; }
            pc = 0xbc865;
        }
        0xbc865 => {
            // block bc865
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc865 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc865 cmp rsi,0x3
            fl = sub::<64>(rsi, 0x3u64).1;
            // bc869 je 0xbc88a
            if crate::cov::br(fl.e()) { pc = 0xbc88a; continue; }
            pc = 0xbc86b;
        }
        0xbc86b => {
            // block bc86b
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc86b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc86b movzx ecx,BYTE PTR [rsi+r15*1]
            rcx = (m.r8(rsi.wrapping_add(r15)) as u64) as u32 as u64;
            // bc870 movzx ebx,BYTE PTR [r13+rsi*1+0x0]
            rbx = (m.r8(r13.wrapping_add(rsi)) as u64) as u32 as u64;
            // bc876 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // bc879 cmp ebx,ecx
            fl = sub::<32>((rbx as u32 as u64), (rcx as u32 as u64)).1;
            // bc87b je 0xbc865
            if crate::cov::br(fl.e()) { pc = 0xbc865; continue; }
            pc = 0xbc87d;
        }
        0xbc87d => {
            // block bc87d
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc87d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc87d inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // bc880 cmp r13,rdx
            fl = sub::<64>(r13, rdx).1;
            // bc883 jne 0xbc856
            if crate::cov::br(fl.ne()) { pc = 0xbc856; continue; }
            pc = 0xbc885;
        }
        0xbc885 => {
            // block bc885
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc885 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc885 jmp 0xbc987
            pc = 0xbc987; continue;
        }
        0xbc88a => {
            // block bc88a
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc88a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc88a cmp r13,r8
            fl = sub::<64>(r13, r8).1;
            // bc88d je 0xbc987
            if crate::cov::br(fl.e()) { pc = 0xbc987; continue; }
            pc = 0xbc893;
        }
        0xbc893 => {
            // block bc893
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc893 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc893 sub r13,rax
            { let (r, f) = sub::<64>(r13, rax);
              r13 = r;
              fl = f; }
            // bc896 cmp r13,0xffffffffffffffff
            fl = sub::<64>(r13, 0xffff_ffff_ffff_ffffu64).1;
            // bc89a je 0xbc987
            if crate::cov::br(fl.e()) { pc = 0xbc987; continue; }
            pc = 0xbc8a0;
        }
        0xbc8a0 => {
            // block bc8a0
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc8a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc8a0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bc8a2 mov rsi,r12
            rsi = r12;
            // bc8a5 mov rcx,r13
            rcx = r13;
            // bc8a8 mov rbx,rdi
            rbx = rdi;
            // bc8ab mov r8,r12
            r8 = r12;
            // bc8ae call 0xec02c
            { m.outregs(0xbc8b3, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc8b3 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // bc8ba mov rsi,rbx
            rsi = rbx;
            // bc8bd call 0xbbe30
            { m.outregs(0xbc8c2, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xbbe30); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc8c2 mov rdi,rbx
            rdi = rbx;
            // bc8c5 call 0xec032
            { m.outregs(0xbc8ca, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc8ca add r13,0x3
            { let (r, f) = add::<64>(r13, 0x3u64);
              r13 = r;
              fl = f; }
            // bc8ce movzx ecx,BYTE PTR [r12]
            rcx = (m.r8(r12) as u64) as u32 as u64;
            // bc8d3 test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // bc8d6 jne 0xbc8dd
            if crate::cov::br(fl.ne()) { pc = 0xbc8dd; continue; }
            pc = 0xbc8d8;
        }
        0xbc8d8 => {
            // block bc8d8
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc8d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc8d8 shr rcx,1
            { let (r, f) = shr::<64>(rcx, 0x1u64, fl); rcx = r; fl = f; }
            // bc8db jmp 0xbc8e2
            pc = 0xbc8e2; continue;
        }
        0xbc8dd => {
            // block bc8dd
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc8dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc8dd mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            pc = 0xbc8e2;
        }
        0xbc8e2 => {
            // block bc8e2
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc8e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc8e2 mov rdi,r14
            rdi = r14;
            // bc8e5 mov rsi,r12
            rsi = r12;
            // bc8e8 mov rdx,r13
            rdx = r13;
            // bc8eb mov r8,r12
            r8 = r12;
            // bc8ee call 0xec02c
            { m.outregs(0xbc8f3, 0x1f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc8f3 lea r13,[rbp-0x110]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // bc8fa mov rdi,r13
            rdi = r13;
            // bc8fd mov rsi,r14
            rsi = r14;
            // bc900 call 0xbbe30
            { m.outregs(0xbc905, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xbbe30); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc905 mov rdi,r14
            rdi = r14;
            // bc908 call 0xec032
            { m.outregs(0xbc90d, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc90d lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // bc914 call 0xbbd44
            { m.outregs(0xbc919, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xbbd44); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc919 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // bc91b mov rdi,r13
            rdi = r13;
            // bc91e call 0xbbfc0
            { m.outregs(0xbc923, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0xbbfc0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc923 cmp ebx,0xffffffff
            fl = sub::<32>((rbx as u32 as u64), 0xffffffffu64).1;
            // bc926 je 0xbc96f
            if crate::cov::br(fl.e()) { pc = 0xbc96f; continue; }
            pc = 0xbc928;
        }
        0xbc928 => {
            // block bc928
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc928 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc928 mov rdi,QWORD PTR [rbp-0x130]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // bc92f mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // bc932 mov rcx,QWORD PTR [rcx+0x10]
            rcx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // bc936 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // bc938 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // bc93a call rcx
            { m.outregs(0xbc93c, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc93c mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // bc93e mov rdi,r13
            rdi = r13;
            // bc941 call 0xec032
            { m.outregs(0xbc946, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc946 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // bc94d call 0xec032
            { m.outregs(0xbc952, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc952 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // bc954 je 0xbc987
            if crate::cov::br(fl.e()) { pc = 0xbc987; continue; }
            pc = 0xbc956;
        }
        0xbc956 => {
            // block bc956
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc956 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc956 add r12,0x18
            { let (r, f) = add::<64>(r12, 0x18u64);
              r12 = r;
              fl = f; }
            // bc95a cmp r12,QWORD PTR [rbp-0x78]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64).1;
            // bc95e lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // bc965 jne 0xbc80b
            if crate::cov::br(fl.ne()) { pc = 0xbc80b; continue; }
            pc = 0xbc96b;
        }
        0xbc96b => {
            // block bc96b
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc96b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc96b mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // bc96d jmp 0xbc989
            pc = 0xbc989; continue;
        }
        0xbc96f => {
            // block bc96f
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc96f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc96f lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // bc976 call 0xec032
            { m.outregs(0xbc97b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc97b lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // bc982 call 0xec032
            { m.outregs(0xbc987, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc987;
        }
        0xbc987 => {
            // block bc987
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc987 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc987 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xbc989;
        }
        0xbc989 => {
            // block bc989
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc989 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // bc990 call 0xec032
            { m.outregs(0xbc995, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc995 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // bc999 call 0xeba98
            { m.outregs(0xbc99e, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc99e mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // bc9a0 add rsp,0x108
            { let (r, f) = add::<64>(rsp, 0x108u64);
              rsp = r;
              fl = f; }
            // bc9a7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc9a8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc9aa pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bc9ac pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc9ae pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc9b0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc9b1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbc9b2 => {
            // block bc9b2
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc9b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc9b2 mov rbx,rax
            rbx = rax;
            // bc9b5 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // bc9bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc9c1 jmp 0xbc9ef
            pc = 0xbc9ef; continue;
        }
        0xbc9c3 => {
            // block bc9c3
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc9c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc9c3 mov rbx,rax
            rbx = rax;
            // bc9c6 jmp 0xbca04
            pc = 0xbca04; continue;
        }
        0xbc9c8 => {
            // block bc9c8
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc9c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc9c8 jmp 0xbc9ca
            pc = 0xbc9ca; continue;
        }
        0xbc9ca => {
            // block bc9ca
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc9ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc9ca mov rbx,rax
            rbx = rax;
            // bc9cd jmp 0xbca10
            pc = 0xbca10; continue;
        }
        0xbc9cf => {
            // block bc9cf
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc9cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc9cf mov rbx,rax
            rbx = rax;
            // bc9d2 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // bc9d9 jmp 0xbca0b
            pc = 0xbca0b; continue;
        }
        0xbc9db => {
            // block bc9db
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc9db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc9db mov rbx,rax
            rbx = rax;
            // bc9de jmp 0xbc9ef
            pc = 0xbc9ef; continue;
        }
        0xbc9e0 => {
            // block bc9e0
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc9e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc9e0 mov rbx,rax
            rbx = rax;
            // bc9e3 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // bc9ea call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc9ef;
        }
        0xbc9ef => {
            // block bc9ef
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc9ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc9ef lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // bc9f6 jmp 0xbca0b
            pc = 0xbca0b; continue;
        }
        0xbc9f8 => {
            // block bc9f8
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bc9f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc9f8 mov rbx,rax
            rbx = rax;
            // bc9fb lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // bc9ff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbca04;
        }
        0xbca04 => {
            // block bca04
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bca04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bca04 lea rdi,[rbp-0xc8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff38);
            pc = 0xbca0b;
        }
        0xbca0b => {
            // block bca0b
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bca0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bca0b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbca10;
        }
        0xbca10 => {
            // block bca10
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bca10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bca10 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            pc = 0xbca17;
        }
        0xbca17 => {
            // block bca17
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bca17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bca17 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbca1c;
        }
        0xbca1c => {
            // block bca1c
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bca1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bca1c lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // bca20 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bca25 mov rdi,rbx
            rdi = rbx;
            // bca28 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bca2d mov rbx,rax
            rbx = rax;
            // bca30 jmp 0xbca1c
            pc = 0xbca1c; continue;
        }
        0xbca32 => {
            // block bca32
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bca32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bca32 mov rbx,rax
            rbx = rax;
            // bca35 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // bca3c jmp 0xbca17
            pc = 0xbca17; continue;
        }
        0xbca3e => {
            // block bca3e
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bca3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bca3e jmp 0xbc9c3
            pc = 0xbc9c3; continue;
        }
        0xbca40 => {
            // block bca40
            #[cfg(feature = "trace")] eprintln!("f_000bc57e pc bca40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bca40 mov rbx,rax
            rbx = rax;
            // bca43 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bca47 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bca4c jmp 0xbca04
            pc = 0xbca04; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3348a size=1418
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0003348a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3348a;
    loop {
        match pc {
        0x3348a => {
            // block 3348a
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3348a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3348a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3348b mov rbp,rsp
            rbp = rsp;
            // 3348e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 33490 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 33492 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 33494 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 33496 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 33497 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 3349b mov rbx,rsi
            rbx = rsi;
            // 3349e mov r15,rdi
            r15 = rdi;
            // 334a1 mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 334a4 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 334a6 je 0x334c3
            if crate::cov::br(fl.e()) { pc = 0x334c3; continue; }
            pc = 0x334a8;
        }
        0x334a8 => {
            // block 334a8
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 334a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 334a8 cmp DWORD PTR [r15+0x4],0x0
            fl = sub::<32>(m.r32(r15.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 334ad je 0x334c3
            if crate::cov::br(fl.e()) { pc = 0x334c3; continue; }
            pc = 0x334af;
        }
        0x334af => {
            // block 334af
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 334af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 334af cmp WORD PTR [r15+0x34],0x0
            fl = sub::<16>(m.r16(r15.wrapping_add(0x34)) as u64, 0x0u64).1;
            // 334b5 je 0x334c3
            if crate::cov::br(fl.e()) { pc = 0x334c3; continue; }
            pc = 0x334b7;
        }
        0x334b7 => {
            // block 334b7
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 334b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 334b7 cmp DWORD PTR [r15+0x24],0x0
            fl = sub::<32>(m.r32(r15.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 334bc je 0x334c3
            if crate::cov::br(fl.e()) { pc = 0x334c3; continue; }
            pc = 0x334be;
        }
        0x334be => {
            // block 334be
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 334be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 334be mov DWORD PTR [rbp-0x74],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), ((rdx as u32 as u64)) as u32);
            // 334c1 jmp 0x334dd
            pc = 0x334dd; continue;
        }
        0x334c3 => {
            // block 334c3
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 334c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 334c3 mov DWORD PTR [rbp-0x74],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), ((rdx as u32 as u64)) as u32);
            // 334c6 lea rsi,[rip+0xf776d] # 0x12ac3a
            rsi = m.env.data_va(259079);
            // 334cd mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 334d2 mov rdi,rbx
            rdi = rbx;
            // 334d5 call 0xebb88
            { m.outregs(0x334da, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 334da mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            pc = 0x334dd;
        }
        0x334dd => {
            // block 334dd
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 334dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 334dd lea r14,[r15+0x34]
            r14 = r15.wrapping_add(0x34);
            // 334e1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 334e4 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 334e8 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 334ec and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 334ef or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 334f2 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 334f6 mov rdi,rbx
            rdi = rbx;
            // 334f9 call 0xec09e
            { m.outregs(0x334fe, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 334fe lea rsi,[rip+0xf773f] # 0x12ac44
            rsi = m.env.data_va(259089);
            // 33505 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 3350a mov rdi,rax
            rdi = rax;
            // 3350d call 0xebb88
            { m.outregs(0x33512, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33512 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 33515 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 33519 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 3351d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33520 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33523 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 33527 mov esi,DWORD PTR [r15+0x4]
            rsi = (m.r32(r15.wrapping_add(0x4)) as u64) as u32 as u64;
            // 3352b mov rdi,rax
            rdi = rax;
            // 3352e call 0xec09e
            { m.outregs(0x33533, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33533 lea rsi,[rip+0xf7713] # 0x12ac4d
            rsi = m.env.data_va(259098);
            // 3353a mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 3353f mov rdi,rax
            rdi = rax;
            // 33542 call 0xebb88
            { m.outregs(0x33547, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33547 movzx eax,WORD PTR [r15+0x34]
            rax = (m.r16(r15.wrapping_add(0x34)) as u64) as u32 as u64;
            // 3354c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 3354f lea r13,[rip+0xf7707] # 0x12ac5d
            r13 = m.env.data_va(259114);
            pc = 0x33556;
        }
        0x33556 => {
            // block 33556
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33556 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33556 movzx ecx,ax
            rcx = ((rax as u16 as u64)) as u32 as u64;
            // 33559 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // 3355c ja 0x33574
            if crate::cov::br(fl.a()) { pc = 0x33574; continue; }
            pc = 0x3355e;
        }
        0x3355e => {
            // block 3355e
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3355e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3355e mov ecx,DWORD PTR [r15+0x18]
            rcx = (m.r32(r15.wrapping_add(0x18)) as u64) as u32 as u64;
            // 33562 add ecx,0xfffffff6
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xfffffff6u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 33565 cmp ecx,0x15
            fl = sub::<32>((rcx as u32 as u64), 0x15u64).1;
            // 33568 ja 0x335ac
            if crate::cov::br(fl.a()) { pc = 0x335ac; continue; }
            pc = 0x3356a;
        }
        0x3356a => {
            // block 3356a
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3356a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3356a mov edx,0x3f2001
            rdx = (0x3f2001u64) as u32 as u64;
            // 3356f bt edx,ecx
            fl.cf = bit::<32>((rdx as u32 as u64), (rcx as u32 as u64));
            // 33572 jae 0x335ac
            if crate::cov::br(fl.ae()) { pc = 0x335ac; continue; }
            pc = 0x33574;
        }
        0x33574 => {
            // block 33574
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33574 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33574 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 33579 mov rdi,rbx
            rdi = rbx;
            // 3357c lea rsi,[rip+0xf76d4] # 0x12ac57
            rsi = m.env.data_va(259108);
            // 33583 call 0xebb88
            { m.outregs(0x33588, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33588 movzx esi,r12w
            rsi = ((r12 as u16 as u64)) as u32 as u64;
            // 3358c mov rdi,rax
            rdi = rax;
            // 3358f call 0xec0aa
            { m.outregs(0x33594, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33594 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 33599 mov rdi,rax
            rdi = rax;
            // 3359c lea rsi,[rip+0xf76b8] # 0x12ac5b
            rsi = m.env.data_va(259112);
            // 335a3 call 0xebb88
            { m.outregs(0x335a8, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 335a8 movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            pc = 0x335ac;
        }
        0x335ac => {
            // block 335ac
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 335ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 335ac mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 335af mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 335b3 mov edx,DWORD PTR [rcx+rbx*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 335b7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 335ba or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 335bd mov DWORD PTR [rcx+rbx*1+0x8],edx
            m.w32(rcx.wrapping_add(rbx).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 335c1 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 335c3 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 335c6 movzx ecx,r12w
            rcx = ((r12 as u16 as u64)) as u32 as u64;
            // 335ca cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // 335cc jbe 0x335d3
            if crate::cov::br(fl.be()) { pc = 0x335d3; continue; }
            pc = 0x335ce;
        }
        0x335ce => {
            // block 335ce
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 335ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 335ce mov esi,DWORD PTR [r15+r12*4+0x24]
            rsi = (m.r32(r15.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x24)) as u64) as u32 as u64;
            pc = 0x335d3;
        }
        0x335d3 => {
            // block 335d3
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 335d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 335d3 mov rdi,rbx
            rdi = rbx;
            // 335d6 call 0xec09e
            { m.outregs(0x335db, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 335db mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 335e0 mov rdi,rax
            rdi = rax;
            // 335e3 mov rsi,r13
            rsi = r13;
            // 335e6 call 0xebb88
            { m.outregs(0x335eb, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 335eb movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // 335ef inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 335f2 movzx ecx,r12w
            rcx = ((r12 as u16 as u64)) as u32 as u64;
            // 335f6 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 335f8 jb 0x33556
            if crate::cov::br(fl.b()) { pc = 0x33556; continue; }
            pc = 0x335fe;
        }
        0x335fe => {
            // block 335fe
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 335fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 335fe lea rsi,[rip+0xf7664] # 0x12ac69
            rsi = m.env.data_va(259126);
            // 33605 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 3360a mov rdi,rbx
            rdi = rbx;
            // 3360d call 0xebb88
            { m.outregs(0x33612, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33612 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 33615 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 33619 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 3361d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33620 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33623 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 33627 mov esi,DWORD PTR [r15+0xc]
            rsi = (m.r32(r15.wrapping_add(0xc)) as u64) as u32 as u64;
            // 3362b mov rdi,rax
            rdi = rax;
            // 3362e call 0xec09e
            { m.outregs(0x33633, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33633 mov eax,DWORD PTR [rbp-0x74]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c)) as u64) as u32 as u64;
            // 33636 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 33638 je 0x339db
            if crate::cov::br(fl.e()) { pc = 0x339db; continue; }
            pc = 0x3363e;
        }
        0x3363e => {
            // block 3363e
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3363e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3363e movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // 33642 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 33645 ja 0x3372a
            if crate::cov::br(fl.a()) { pc = 0x3372a; continue; }
            pc = 0x3364b;
        }
        0x3364b => {
            // block 3364b
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3364b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3364b mov eax,DWORD PTR [r15+0x18]
            rax = (m.r32(r15.wrapping_add(0x18)) as u64) as u32 as u64;
            // 3364f add eax,0xfffffff6
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffff6u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33652 cmp eax,0x15
            fl = sub::<32>((rax as u32 as u64), 0x15u64).1;
            // 33655 ja 0x33665
            if crate::cov::br(fl.a()) { pc = 0x33665; continue; }
            pc = 0x33657;
        }
        0x33657 => {
            // block 33657
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33657 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33657 mov ecx,0x3f2001
            rcx = (0x3f2001u64) as u32 as u64;
            // 3365c bt ecx,eax
            fl.cf = bit::<32>((rcx as u32 as u64), (rax as u32 as u64));
            // 3365f jb 0x3372a
            if crate::cov::br(fl.b()) { pc = 0x3372a; continue; }
            pc = 0x33665;
        }
        0x33665 => {
            // block 33665
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33665 lea rsi,[rip+0xf7614] # 0x12ac80
            rsi = m.env.data_va(259149);
            // 3366c mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 33671 mov rdi,rbx
            rdi = rbx;
            // 33674 call 0xebb88
            { m.outregs(0x33679, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33679 lea rsi,[rip+0xf6df5] # 0x12a475
            rsi = m.env.data_va(257090);
            // 33680 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 33685 mov rdi,rax
            rdi = rax;
            // 33688 call 0xebb88
            { m.outregs(0x3368d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3368d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 33690 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 33694 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 33698 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3369b or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3369e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 336a2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 336a5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 336a9 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // 336b1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 336b4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 336b8 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 336c1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 336c4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 336c8 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 336d3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 336d5 cmp WORD PTR [r14],0x0
            fl = sub::<16>(m.r16(r14) as u64, 0x0u64).1;
            // 336da je 0x336e4
            if crate::cov::br(fl.e()) { pc = 0x336e4; continue; }
            pc = 0x336dc;
        }
        0x336dc => {
            // block 336dc
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 336dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 336dc mov esi,DWORD PTR [r15+0x24]
            rsi = (m.r32(r15.wrapping_add(0x24)) as u64) as u32 as u64;
            // 336e0 imul esi,DWORD PTR [r15]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r15) as u64); rsi = (r) as u32 as u64; fl = f; }
            pc = 0x336e4;
        }
        0x336e4 => {
            // block 336e4
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 336e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 336e4 mov r13,r15
            r13 = r15;
            // 336e7 mov rdi,rax
            rdi = rax;
            // 336ea call 0xec09e
            { m.outregs(0x336ef, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 336ef mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 336f2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 336f6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 336fa and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 336fd or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33700 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 33704 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 33707 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 3370b mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // 33716 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 33719 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 3371d and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // 33725 jmp 0x3382a
            pc = 0x3382a; continue;
        }
        0x3372a => {
            // block 3372a
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3372a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3372a mov r13,r15
            r13 = r15;
            // 3372d xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x33730;
        }
        0x33730 => {
            // block 33730
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33730 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33730 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 33735 mov rdi,rbx
            rdi = rbx;
            // 33738 lea rsi,[rip+0xf7534] # 0x12ac73
            rsi = m.env.data_va(259136);
            // 3373f call 0xebb88
            { m.outregs(0x33744, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33744 movzx r15d,r12w
            r15 = ((r12 as u16 as u64)) as u32 as u64;
            // 33748 mov rdi,rax
            rdi = rax;
            // 3374b mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 3374e call 0xec0aa
            { m.outregs(0x33753, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33753 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 33758 mov rdi,rax
            rdi = rax;
            // 3375b lea rsi,[rip+0xf7516] # 0x12ac78
            rsi = m.env.data_va(259141);
            // 33762 call 0xebb88
            { m.outregs(0x33767, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33767 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 3376c mov rdi,rax
            rdi = rax;
            // 3376f lea rsi,[rip+0xf6cff] # 0x12a475
            rsi = m.env.data_va(257090);
            // 33776 call 0xebb88
            { m.outregs(0x3377b, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3377b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 3377e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 33782 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 33786 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33789 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3378c mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 33790 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 33793 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 33797 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // 3379f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 337a2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 337a6 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 337af mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 337b2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 337b6 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 337c1 movzx ecx,WORD PTR [r14]
            rcx = (m.r16(r14) as u64) as u32 as u64;
            // 337c5 cmp ecx,r15d
            fl = sub::<32>((rcx as u32 as u64), (r15 as u32 as u64)).1;
            // 337c8 mov esi,0x0
            rsi = (0x0u64) as u32 as u64;
            // 337cd jbe 0x337d9
            if crate::cov::br(fl.be()) { pc = 0x337d9; continue; }
            pc = 0x337cf;
        }
        0x337cf => {
            // block 337cf
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 337cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 337cf mov esi,DWORD PTR [r13+r12*4+0x24]
            rsi = (m.r32(r13.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x24)) as u64) as u32 as u64;
            // 337d4 imul esi,DWORD PTR [r13+0x0]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r13) as u64); rsi = (r) as u32 as u64; fl = f; }
            pc = 0x337d9;
        }
        0x337d9 => {
            // block 337d9
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 337d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 337d9 mov rdi,rax
            rdi = rax;
            // 337dc call 0xec09e
            { m.outregs(0x337e1, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 337e1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 337e4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 337e8 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 337ec and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 337ef or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 337f2 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 337f6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 337f9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 337fd mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // 33808 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 3380b mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 3380f and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // 33817 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 3381a movzx eax,r12w
            rax = ((r12 as u16 as u64)) as u32 as u64;
            // 3381e movzx ecx,WORD PTR [r14]
            rcx = (m.r16(r14) as u64) as u32 as u64;
            // 33822 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // 33824 jb 0x33730
            if crate::cov::br(fl.b()) { pc = 0x33730; continue; }
            pc = 0x3382a;
        }
        0x3382a => {
            // block 3382a
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3382a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3382a mov eax,DWORD PTR [r13+0x14]
            rax = (m.r32(r13.wrapping_add(0x14)) as u64) as u32 as u64;
            // 3382e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 33830 je 0x338ae
            if crate::cov::br(fl.e()) { pc = 0x338ae; continue; }
            pc = 0x33832;
        }
        0x33832 => {
            // block 33832
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33832 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33832 lea ecx,[rax-0x1]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33835 cmp ecx,0x1e
            fl = sub::<32>((rcx as u32 as u64), 0x1eu64).1;
            // 33838 jb 0x33876
            if crate::cov::br(fl.b()) { pc = 0x33876; continue; }
            pc = 0x3383a;
        }
        0x3383a => {
            // block 3383a
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3383a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3383a lea ecx,[rax-0x6e]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_ff92)) as u32 as u64;
            // 3383d cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // 33840 jb 0x33876
            if crate::cov::br(fl.b()) { pc = 0x33876; continue; }
            pc = 0x33842;
        }
        0x33842 => {
            // block 33842
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33842 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33842 lea edx,[rax-0x20]
            rdx = (rax.wrapping_add(0xffff_ffff_ffff_ffe0)) as u32 as u64;
            // 33845 cmp edx,0x6
            fl = sub::<32>((rdx as u32 as u64), 0x6u64).1;
            // 33848 jb 0x33876
            if crate::cov::br(fl.b()) { pc = 0x33876; continue; }
            pc = 0x3384a;
        }
        0x3384a => {
            // block 3384a
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3384a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3384a cmp eax,0x6d
            fl = sub::<32>((rax as u32 as u64), 0x6du64).1;
            // 3384d jg 0x33869
            if crate::cov::br(fl.g()) { pc = 0x33869; continue; }
            pc = 0x3384f;
        }
        0x3384f => {
            // block 3384f
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3384f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3384f lea ecx,[rax-0xd]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_fff3)) as u32 as u64;
            // 33852 cmp ecx,0x37
            fl = sub::<32>((rcx as u32 as u64), 0x37u64).1;
            // 33855 ja 0x3386e
            if crate::cov::br(fl.a()) { pc = 0x3386e; continue; }
            pc = 0x33857;
        }
        0x33857 => {
            // block 33857
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33857 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33857 movabs rdx,0xf800000000600f
            rdx = 0xf8_0000_0000_600fu64;
            // 33861 bt rdx,rcx
            fl.cf = bit::<64>(rdx, rcx);
            // 33865 jb 0x33876
            if crate::cov::br(fl.b()) { pc = 0x33876; continue; }
            pc = 0x33867;
        }
        0x33867 => {
            // block 33867
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33867 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33867 jmp 0x3386e
            pc = 0x3386e; continue;
        }
        0x33869 => {
            // block 33869
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33869 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33869 cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // 3386c jb 0x33876
            if crate::cov::br(fl.b()) { pc = 0x33876; continue; }
            pc = 0x3386e;
        }
        0x3386e => {
            // block 3386e
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3386e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3386e add eax,0xffffffb0
            { let (r, f) = add::<32>((rax as u32 as u64), 0xffffffb0u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33871 cmp eax,0x1d
            fl = sub::<32>((rax as u32 as u64), 0x1du64).1;
            // 33874 ja 0x338ae
            if crate::cov::br(fl.a()) { pc = 0x338ae; continue; }
            pc = 0x33876;
        }
        0x33876 => {
            // block 33876
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33876 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33876 lea rsi,[rip+0xf740c] # 0x12ac89
            rsi = m.env.data_va(259158);
            // 3387d mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 33882 mov rdi,rbx
            rdi = rbx;
            // 33885 call 0xebb88
            { m.outregs(0x3388a, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3388a mov r15,rax
            r15 = rax;
            // 3388d mov esi,DWORD PTR [r13+0x14]
            rsi = (m.r32(r13.wrapping_add(0x14)) as u64) as u32 as u64;
            // 33891 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 33895 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33897 call 0x61a4c
            { m.outregs(0x3389c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x61a4c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3389c movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 338a0 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 338a3 jne 0x338e6
            if crate::cov::br(fl.ne()) { pc = 0x338e6; continue; }
            pc = 0x338a5;
        }
        0x338a5 => {
            // block 338a5
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 338a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 338a5 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 338a9 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 338ac jmp 0x338ee
            pc = 0x338ee; continue;
        }
        0x338ae => {
            // block 338ae
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 338ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 338ae lea rsi,[rip+0xf6be8] # 0x12a49d
            rsi = m.env.data_va(257130);
            // 338b5 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 338ba mov rdi,rbx
            rdi = rbx;
            // 338bd call 0xebb88
            { m.outregs(0x338c2, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 338c2 mov r15,rax
            r15 = rax;
            // 338c5 mov esi,DWORD PTR [r13+0x10]
            rsi = (m.r32(r13.wrapping_add(0x10)) as u64) as u32 as u64;
            // 338c9 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 338cd xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 338cf call 0x5b5c9
            { m.outregs(0x338d4, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x5b5c9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 338d4 movzx edx,BYTE PTR [rbp-0x58]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // 338d8 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 338db jne 0x33910
            if crate::cov::br(fl.ne()) { pc = 0x33910; continue; }
            pc = 0x338dd;
        }
        0x338dd => {
            // block 338dd
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 338dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 338dd lea rsi,[rbp-0x57]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa9);
            // 338e1 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 338e4 jmp 0x33918
            pc = 0x33918; continue;
        }
        0x338e6 => {
            // block 338e6
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 338e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 338e6 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 338ea mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x338ee;
        }
        0x338ee => {
            // block 338ee
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 338ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 338ee mov rdi,r15
            rdi = r15;
            // 338f1 call 0xebb88
            { m.outregs(0x338f6, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 338f6 lea rsi,[rip+0xf678b] # 0x12a088
            rsi = m.env.data_va(256085);
            // 338fd mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 33902 mov rdi,rax
            rdi = rax;
            // 33905 call 0xebb88
            { m.outregs(0x3390a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3390a lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 3390e jmp 0x3394a
            pc = 0x3394a; continue;
        }
        0x33910 => {
            // block 33910
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33910 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33910 mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 33914 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0x33918;
        }
        0x33918 => {
            // block 33918
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33918 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33918 mov rdi,r15
            rdi = r15;
            // 3391b call 0xebb88
            { m.outregs(0x33920, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33920 movzx ecx,BYTE PTR [r13+0x20]
            rcx = (m.r8(r13.wrapping_add(0x20)) as u64) as u32 as u64;
            // 33925 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 33928 lea rdx,[rip+0xf735d] # 0x12ac8c
            rdx = m.env.data_va(259161);
            // 3392f lea rsi,[rip+0x105392] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 33936 cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // 3393a lea rdx,[rcx+rcx*2]
            rdx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // 3393e mov rdi,rax
            rdi = rax;
            // 33941 call 0xebb88
            { m.outregs(0x33946, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33946 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0x3394a;
        }
        0x3394a => {
            // block 3394a
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3394a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3394a call 0xec032
            { m.outregs(0x3394f, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3394f mov eax,DWORD PTR [r13+0x1c]
            rax = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 33953 lea ecx,[rax-0x1]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33956 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // 33959 ja 0x33983
            if crate::cov::br(fl.a()) { pc = 0x33983; continue; }
            pc = 0x3395b;
        }
        0x3395b => {
            // block 3395b
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3395b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3395b cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 3395e sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 33961 lea rcx,[rip+0xf7328] # 0x12ac90
            rcx = m.env.data_va(259165);
            // 33968 lea rsi,[rip+0xf732d] # 0x12ac9c
            rsi = m.env.data_va(259177);
            // 3396f cmove rsi,rcx
            if fl.e() { rsi = rcx; }
            // 33973 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 33976 lea rdx,[rax+rax*1+0x9]
            rdx = rax.wrapping_add(rax).wrapping_add(0x9);
            // 3397b mov rdi,rbx
            rdi = rbx;
            // 3397e call 0xebb88
            { m.outregs(0x33983, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x33983;
        }
        0x33983 => {
            // block 33983
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33983 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33983 cmp DWORD PTR [r13+0x18],0x1f
            fl = sub::<32>(m.r32(r13.wrapping_add(0x18)) as u64, 0x1fu64).1;
            // 33988 ja 0x339db
            if crate::cov::br(fl.a()) { pc = 0x339db; continue; }
            pc = 0x3398a;
        }
        0x3398a => {
            // block 3398a
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 3398a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3398a lea rsi,[rip+0xf6b0c] # 0x12a49d
            rsi = m.env.data_va(257130);
            // 33991 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 33996 mov rdi,rbx
            rdi = rbx;
            // 33999 call 0xebb88
            { m.outregs(0x3399e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3399e mov r15,rax
            r15 = rax;
            // 339a1 mov esi,DWORD PTR [r13+0x18]
            rsi = (m.r32(r13.wrapping_add(0x18)) as u64) as u32 as u64;
            // 339a5 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 339a9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 339ab call 0x61aed
            { m.outregs(0x339b0, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x61aed); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 339b0 movzx edx,BYTE PTR [rbp-0x70]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // 339b4 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 339b7 jne 0x339c2
            if crate::cov::br(fl.ne()) { pc = 0x339c2; continue; }
            pc = 0x339b9;
        }
        0x339b9 => {
            // block 339b9
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 339b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 339b9 lea rsi,[rbp-0x6f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff91);
            // 339bd shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 339c0 jmp 0x339ca
            pc = 0x339ca; continue;
        }
        0x339c2 => {
            // block 339c2
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 339c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 339c2 mov rdx,QWORD PTR [rbp-0x68]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 339c6 mov rsi,QWORD PTR [rbp-0x60]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            pc = 0x339ca;
        }
        0x339ca => {
            // block 339ca
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 339ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 339ca mov rdi,r15
            rdi = r15;
            // 339cd call 0xebb88
            { m.outregs(0x339d2, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 339d2 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 339d6 call 0xec032
            { m.outregs(0x339db, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x339db;
        }
        0x339db => {
            // block 339db
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 339db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 339db mov rax,rbx
            rax = rbx;
            // 339de add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 339e2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 339e3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 339e5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 339e7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 339e9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 339eb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 339ec ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x339ed => {
            // block 339ed
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 339ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 339ed mov rbx,rax
            rbx = rax;
            // 339f0 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 339f4 jmp 0x33a06
            pc = 0x33a06; continue;
        }
        0x339f6 => {
            // block 339f6
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 339f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 339f6 mov rbx,rax
            rbx = rax;
            // 339f9 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 339fd jmp 0x33a06
            pc = 0x33a06; continue;
        }
        0x339ff => {
            // block 339ff
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 339ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 339ff mov rbx,rax
            rbx = rax;
            // 33a02 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            pc = 0x33a06;
        }
        0x33a06 => {
            // block 33a06
            #[cfg(feature = "trace")] eprintln!("f_0003348a pc 33a06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a06 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33a0b mov rdi,rbx
            rdi = rbx;
            // 33a0e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x63714 size=1439
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00063714<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x63714;
    loop {
        match pc {
        0x63714 => {
            // block 63714
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63714 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63714 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 63715 mov rbp,rsp
            rbp = rsp;
            // 63718 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6371a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6371c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6371d sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // 63724 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 63726 mov r15,rdi
            r15 = rdi;
            // 63729 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 6372d call 0x633ff
            { m.outregs(0x63732, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x633ff); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63732 movzx edx,BYTE PTR [r15]
            rdx = (m.r8(r15) as u64) as u32 as u64;
            // 63736 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63738 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6373a jne 0x63741
            if crate::cov::br(fl.ne()) { pc = 0x63741; continue; }
            pc = 0x6373c;
        }
        0x6373c => {
            // block 6373c
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6373c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6373c shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 6373f jmp 0x63745
            pc = 0x63745; continue;
        }
        0x63741 => {
            // block 63741
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63741 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63741 mov rdx,QWORD PTR [r15+0x8]
            rdx = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x63745;
        }
        0x63745 => {
            // block 63745
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63745 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63745 movzx esi,BYTE PTR [rbp-0x30]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 63749 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 6374c and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6374f jne 0x63756
            if crate::cov::br(fl.ne()) { pc = 0x63756; continue; }
            pc = 0x63751;
        }
        0x63751 => {
            // block 63751
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63751 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63751 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63754 jmp 0x6375a
            pc = 0x6375a; continue;
        }
        0x63756 => {
            // block 63756
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63756 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63756 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            pc = 0x6375a;
        }
        0x6375a => {
            // block 6375a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6375a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6375a cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 6375d jne 0x637bb
            if crate::cov::br(fl.ne()) { pc = 0x637bb; continue; }
            pc = 0x6375f;
        }
        0x6375f => {
            // block 6375f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6375f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6375f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63761 jne 0x63769
            if crate::cov::br(fl.ne()) { pc = 0x63769; continue; }
            pc = 0x63763;
        }
        0x63763 => {
            // block 63763
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63763 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63763 lea rdi,[r15+0x1]
            rdi = r15.wrapping_add(0x1);
            // 63767 jmp 0x6376d
            pc = 0x6376d; continue;
        }
        0x63769 => {
            // block 63769
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63769 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63769 mov rdi,QWORD PTR [r15+0x10]
            rdi = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x6376d;
        }
        0x6376d => {
            // block 6376d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6376d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6376d test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 6376f jne 0x63777
            if crate::cov::br(fl.ne()) { pc = 0x63777; continue; }
            pc = 0x63771;
        }
        0x63771 => {
            // block 63771
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63771 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63771 lea rsi,[rbp-0x2f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd1);
            // 63775 jmp 0x6377b
            pc = 0x6377b; continue;
        }
        0x63777 => {
            // block 63777
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63777 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63777 mov rsi,QWORD PTR [rbp-0x20]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            pc = 0x6377b;
        }
        0x6377b => {
            // block 6377b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6377b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6377b mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 6377e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63780 jne 0x637a5
            if crate::cov::br(fl.ne()) { pc = 0x637a5; continue; }
            pc = 0x63782;
        }
        0x63782 => {
            // block 63782
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63782 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63782 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63785 je 0x63c85
            if crate::cov::br(fl.e()) { pc = 0x63c85; continue; }
            pc = 0x6378b;
        }
        0x6378b => {
            // block 6378b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6378b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6378b movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 6378e movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63791 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63793 jne 0x637bb
            if crate::cov::br(fl.ne()) { pc = 0x637bb; continue; }
            pc = 0x63795;
        }
        0x63795 => {
            // block 63795
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63795 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63795 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63798 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 6379b dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 6379e jne 0x6378b
            if crate::cov::br(fl.ne()) { pc = 0x6378b; continue; }
            pc = 0x637a0;
        }
        0x637a0 => {
            // block 637a0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637a0 jmp 0x63c85
            pc = 0x63c85; continue;
        }
        0x637a5 => {
            // block 637a5
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637a5 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 637a8 je 0x63c85
            if crate::cov::br(fl.e()) { pc = 0x63c85; continue; }
            pc = 0x637ae;
        }
        0x637ae => {
            // block 637ae
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637ae call 0xec218
            { m.outregs(0x637b3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 637b3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 637b5 je 0x63c85
            if crate::cov::br(fl.e()) { pc = 0x63c85; continue; }
            pc = 0x637bb;
        }
        0x637bb => {
            // block 637bb
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637bb xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 637be cmp ebx,0x104782ff
            fl = sub::<32>((rbx as u32 as u64), 0x104782ffu64).1;
            // 637c4 jg 0x637f9
            if crate::cov::br(fl.g()) { pc = 0x637f9; continue; }
            pc = 0x637c6;
        }
        0x637c6 => {
            // block 637c6
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637c6 cmp ebx,0x10294700
            fl = sub::<32>((rbx as u32 as u64), 0x10294700u64).1;
            // 637cc jne 0x6383e
            if crate::cov::br(fl.ne()) { pc = 0x6383e; continue; }
            pc = 0x637ce;
        }
        0x637ce => {
            // block 637ce
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637ce lea rsi,[rip+0xce0b8] # 0x13188d
            rsi = m.env.data_va(286810);
            // 637d5 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 637d9 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 637de call 0xebfe4
            { m.outregs(0x637e3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 637e3 movzx edx,BYTE PTR [rbp-0x60]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 637e7 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 637e9 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 637eb jne 0x63922
            if crate::cov::br(fl.ne()) { pc = 0x63922; continue; }
            pc = 0x637f1;
        }
        0x637f1 => {
            // block 637f1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637f1 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 637f4 jmp 0x63926
            pc = 0x63926; continue;
        }
        0x637f9 => {
            // block 637f9
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637f9 cmp ebx,0x105183ff
            fl = sub::<32>((rbx as u32 as u64), 0x105183ffu64).1;
            // 637ff jg 0x63875
            if crate::cov::br(fl.g()) { pc = 0x63875; continue; }
            pc = 0x63801;
        }
        0x63801 => {
            // block 63801
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63801 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63801 cmp ebx,0x10478300
            fl = sub::<32>((rbx as u32 as u64), 0x10478300u64).1;
            // 63807 jne 0x638a8
            if crate::cov::br(fl.ne()) { pc = 0x638a8; continue; }
            pc = 0x6380d;
        }
        0x6380d => {
            // block 6380d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6380d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6380d lea rsi,[rip+0xce03b] # 0x13184f
            rsi = m.env.data_va(286748);
            // 63814 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 6381b mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 63820 call 0xebfe4
            { m.outregs(0x63825, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63825 movzx edx,BYTE PTR [rbp-0xa8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64) as u32 as u64;
            // 6382c mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 6382e and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63830 jne 0x6394e
            if crate::cov::br(fl.ne()) { pc = 0x6394e; continue; }
            pc = 0x63836;
        }
        0x63836 => {
            // block 63836
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63836 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63836 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 63839 jmp 0x63955
            pc = 0x63955; continue;
        }
        0x6383e => {
            // block 6383e
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6383e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6383e cmp ebx,0x10322950
            fl = sub::<32>((rbx as u32 as u64), 0x10322950u64).1;
            // 63844 jne 0x63c85
            if crate::cov::br(fl.ne()) { pc = 0x63c85; continue; }
            pc = 0x6384a;
        }
        0x6384a => {
            // block 6384a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6384a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6384a lea rsi,[rip+0xce030] # 0x131881
            rsi = m.env.data_va(286798);
            // 63851 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 63855 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 6385a call 0xebfe4
            { m.outregs(0x6385f, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6385f movzx edx,BYTE PTR [rbp-0x48]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 63863 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63865 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63867 jne 0x63966
            if crate::cov::br(fl.ne()) { pc = 0x63966; continue; }
            pc = 0x6386d;
        }
        0x6386d => {
            // block 6386d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6386d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6386d shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 63870 jmp 0x6396a
            pc = 0x6396a; continue;
        }
        0x63875 => {
            // block 63875
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63875 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63875 cmp ebx,0x10518400
            fl = sub::<32>((rbx as u32 as u64), 0x10518400u64).1;
            // 6387b jne 0x638e5
            if crate::cov::br(fl.ne()) { pc = 0x638e5; continue; }
            pc = 0x6387d;
        }
        0x6387d => {
            // block 6387d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6387d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6387d lea rsi,[rip+0xce026] # 0x1318aa
            rsi = m.env.data_va(286839);
            // 63884 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 63888 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 6388d call 0xebfe4
            { m.outregs(0x63892, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63892 movzx edx,BYTE PTR [rbp-0x78]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // 63896 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63898 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6389a jne 0x6397b
            if crate::cov::br(fl.ne()) { pc = 0x6397b; continue; }
            pc = 0x638a0;
        }
        0x638a0 => {
            // block 638a0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638a0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 638a3 jmp 0x6397f
            pc = 0x6397f; continue;
        }
        0x638a8 => {
            // block 638a8
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638a8 cmp ebx,0x10478350
            fl = sub::<32>((rbx as u32 as u64), 0x10478350u64).1;
            // 638ae jne 0x63c85
            if crate::cov::br(fl.ne()) { pc = 0x63c85; continue; }
            pc = 0x638b4;
        }
        0x638b4 => {
            // block 638b4
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638b4 lea rsi,[rip+0xcdf87] # 0x131842
            rsi = m.env.data_va(286735);
            // 638bb lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 638c2 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 638c7 call 0xebfe4
            { m.outregs(0x638cc, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 638cc movzx edx,BYTE PTR [rbp-0xc0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64) as u32 as u64;
            // 638d3 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 638d5 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 638d7 jne 0x63a43
            if crate::cov::br(fl.ne()) { pc = 0x63a43; continue; }
            pc = 0x638dd;
        }
        0x638dd => {
            // block 638dd
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638dd shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 638e0 jmp 0x63a4a
            pc = 0x63a4a; continue;
        }
        0x638e5 => {
            // block 638e5
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638e5 cmp ebx,0x10518450
            fl = sub::<32>((rbx as u32 as u64), 0x10518450u64).1;
            // 638eb jne 0x63c85
            if crate::cov::br(fl.ne()) { pc = 0x63c85; continue; }
            pc = 0x638f1;
        }
        0x638f1 => {
            // block 638f1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 638f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 638f1 lea rsi,[rip+0xcdfa2] # 0x13189a
            rsi = m.env.data_va(286823);
            // 638f8 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 638ff mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 63904 call 0xebfe4
            { m.outregs(0x63909, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63909 movzx edx,BYTE PTR [rbp-0x90]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64) as u32 as u64;
            // 63910 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63912 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63914 jne 0x63a5b
            if crate::cov::br(fl.ne()) { pc = 0x63a5b; continue; }
            pc = 0x6391a;
        }
        0x6391a => {
            // block 6391a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6391a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6391a shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 6391d jmp 0x63a62
            pc = 0x63a62; continue;
        }
        0x63922 => {
            // block 63922
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63922 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63922 mov rdx,QWORD PTR [rbp-0x58]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x63926;
        }
        0x63926 => {
            // block 63926
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63926 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63926 movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 6392a mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 6392d and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63930 jne 0x63937
            if crate::cov::br(fl.ne()) { pc = 0x63937; continue; }
            pc = 0x63932;
        }
        0x63932 => {
            // block 63932
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63932 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63932 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63935 jmp 0x6393b
            pc = 0x6393b; continue;
        }
        0x63937 => {
            // block 63937
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63937 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63937 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x6393b;
        }
        0x6393b => {
            // block 6393b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6393b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6393b cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 6393e jne 0x63a22
            if crate::cov::br(fl.ne()) { pc = 0x63a22; continue; }
            pc = 0x63944;
        }
        0x63944 => {
            // block 63944
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63944 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63944 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63946 jne 0x63990
            if crate::cov::br(fl.ne()) { pc = 0x63990; continue; }
            pc = 0x63948;
        }
        0x63948 => {
            // block 63948
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63948 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63948 lea rdi,[rbp-0x5f]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa1);
            // 6394c jmp 0x63994
            pc = 0x63994; continue;
        }
        0x6394e => {
            // block 6394e
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6394e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6394e mov rdx,QWORD PTR [rbp-0xa0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            pc = 0x63955;
        }
        0x63955 => {
            // block 63955
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63955 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63955 movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63959 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 6395c and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6395f jne 0x6399d
            if crate::cov::br(fl.ne()) { pc = 0x6399d; continue; }
            pc = 0x63961;
        }
        0x63961 => {
            // block 63961
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63961 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63961 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63964 jmp 0x639a1
            pc = 0x639a1; continue;
        }
        0x63966 => {
            // block 63966
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63966 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63966 mov rdx,QWORD PTR [rbp-0x40]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            pc = 0x6396a;
        }
        0x6396a => {
            // block 6396a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6396a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6396a movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 6396e mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63971 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63974 jne 0x639be
            if crate::cov::br(fl.ne()) { pc = 0x639be; continue; }
            pc = 0x63976;
        }
        0x63976 => {
            // block 63976
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63976 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63979 jmp 0x639c2
            pc = 0x639c2; continue;
        }
        0x6397b => {
            // block 6397b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6397b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6397b mov rdx,QWORD PTR [rbp-0x70]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x6397f;
        }
        0x6397f => {
            // block 6397f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6397f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6397f movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63983 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63986 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63989 jne 0x639dc
            if crate::cov::br(fl.ne()) { pc = 0x639dc; continue; }
            pc = 0x6398b;
        }
        0x6398b => {
            // block 6398b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6398b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6398b shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 6398e jmp 0x639e0
            pc = 0x639e0; continue;
        }
        0x63990 => {
            // block 63990
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63990 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63990 mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0x63994;
        }
        0x63994 => {
            // block 63994
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63994 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63996 jne 0x639fa
            if crate::cov::br(fl.ne()) { pc = 0x639fa; continue; }
            pc = 0x63998;
        }
        0x63998 => {
            // block 63998
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63998 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63998 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 6399b jmp 0x639fe
            pc = 0x639fe; continue;
        }
        0x6399d => {
            // block 6399d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6399d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6399d mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x639a1;
        }
        0x639a1 => {
            // block 639a1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639a1 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 639a4 jne 0x63b1d
            if crate::cov::br(fl.ne()) { pc = 0x63b1d; continue; }
            pc = 0x639aa;
        }
        0x639aa => {
            // block 639aa
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639aa test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639ac jne 0x63a73
            if crate::cov::br(fl.ne()) { pc = 0x63a73; continue; }
            pc = 0x639b2;
        }
        0x639b2 => {
            // block 639b2
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639b2 lea rdi,[rbp-0xa7]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff59);
            // 639b9 jmp 0x63a7a
            pc = 0x63a7a; continue;
        }
        0x639be => {
            // block 639be
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639be mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x639c2;
        }
        0x639c2 => {
            // block 639c2
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639c2 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 639c5 jne 0x63b4a
            if crate::cov::br(fl.ne()) { pc = 0x63b4a; continue; }
            pc = 0x639cb;
        }
        0x639cb => {
            // block 639cb
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639cb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639cd jne 0x63a83
            if crate::cov::br(fl.ne()) { pc = 0x63a83; continue; }
            pc = 0x639d3;
        }
        0x639d3 => {
            // block 639d3
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639d3 lea rdi,[rbp-0x47]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb9);
            // 639d7 jmp 0x63a87
            pc = 0x63a87; continue;
        }
        0x639dc => {
            // block 639dc
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639dc mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x639e0;
        }
        0x639e0 => {
            // block 639e0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639e0 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 639e3 jne 0x63b77
            if crate::cov::br(fl.ne()) { pc = 0x63b77; continue; }
            pc = 0x639e9;
        }
        0x639e9 => {
            // block 639e9
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639e9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639eb jne 0x63a97
            if crate::cov::br(fl.ne()) { pc = 0x63a97; continue; }
            pc = 0x639f1;
        }
        0x639f1 => {
            // block 639f1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639f1 lea rdi,[rbp-0x77]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff89);
            // 639f5 jmp 0x63a9b
            pc = 0x63a9b; continue;
        }
        0x639fa => {
            // block 639fa
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639fa mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x639fe;
        }
        0x639fe => {
            // block 639fe
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639fe mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63a01 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63a03 jne 0x63a27
            if crate::cov::br(fl.ne()) { pc = 0x63a27; continue; }
            pc = 0x63a05;
        }
        0x63a05 => {
            // block 63a05
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a05 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63a08 je 0x63a3a
            if crate::cov::br(fl.e()) { pc = 0x63a3a; continue; }
            pc = 0x63a0a;
        }
        0x63a0a => {
            // block 63a0a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a0a movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63a0e movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63a11 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63a13 jne 0x63a22
            if crate::cov::br(fl.ne()) { pc = 0x63a22; continue; }
            pc = 0x63a15;
        }
        0x63a15 => {
            // block 63a15
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a15 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63a18 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63a1b dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63a1e jne 0x63a0a
            if crate::cov::br(fl.ne()) { pc = 0x63a0a; continue; }
            pc = 0x63a20;
        }
        0x63a20 => {
            // block 63a20
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a20 jmp 0x63a3a
            pc = 0x63a3a; continue;
        }
        0x63a22 => {
            // block 63a22
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a22 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63a25 jmp 0x63a3a
            pc = 0x63a3a; continue;
        }
        0x63a27 => {
            // block 63a27
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a27 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63a2a je 0x63a3a
            if crate::cov::br(fl.e()) { pc = 0x63a3a; continue; }
            pc = 0x63a2c;
        }
        0x63a2c => {
            // block 63a2c
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a2c mov rsi,r15
            rsi = r15;
            // 63a2f call 0xec218
            { m.outregs(0x63a34, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63a34 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63a36 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63a3a;
        }
        0x63a3a => {
            // block 63a3a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a3a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 63a3e jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63a43 => {
            // block 63a43
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a43 mov rdx,QWORD PTR [rbp-0xb8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            pc = 0x63a4a;
        }
        0x63a4a => {
            // block 63a4a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a4a movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63a4e mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63a51 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63a54 jne 0x63aab
            if crate::cov::br(fl.ne()) { pc = 0x63aab; continue; }
            pc = 0x63a56;
        }
        0x63a56 => {
            // block 63a56
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a56 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63a59 jmp 0x63aaf
            pc = 0x63aaf; continue;
        }
        0x63a5b => {
            // block 63a5b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a5b mov rdx,QWORD PTR [rbp-0x88]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0x63a62;
        }
        0x63a62 => {
            // block 63a62
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a62 movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63a66 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63a69 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63a6c jne 0x63acc
            if crate::cov::br(fl.ne()) { pc = 0x63acc; continue; }
            pc = 0x63a6e;
        }
        0x63a6e => {
            // block 63a6e
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a6e shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63a71 jmp 0x63ad0
            pc = 0x63ad0; continue;
        }
        0x63a73 => {
            // block 63a73
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a73 mov rdi,QWORD PTR [rbp-0x98]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            pc = 0x63a7a;
        }
        0x63a7a => {
            // block 63a7a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a7a test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63a7c jne 0x63aed
            if crate::cov::br(fl.ne()) { pc = 0x63aed; continue; }
            pc = 0x63a7e;
        }
        0x63a7e => {
            // block 63a7e
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a7e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63a81 jmp 0x63af1
            pc = 0x63af1; continue;
        }
        0x63a83 => {
            // block 63a83
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a83 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x63a87;
        }
        0x63a87 => {
            // block 63a87
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a87 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63a89 jne 0x63b22
            if crate::cov::br(fl.ne()) { pc = 0x63b22; continue; }
            pc = 0x63a8f;
        }
        0x63a8f => {
            // block 63a8f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a8f inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63a92 jmp 0x63b26
            pc = 0x63b26; continue;
        }
        0x63a97 => {
            // block 63a97
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a97 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0x63a9b;
        }
        0x63a9b => {
            // block 63a9b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a9b test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63a9d jne 0x63b4f
            if crate::cov::br(fl.ne()) { pc = 0x63b4f; continue; }
            pc = 0x63aa3;
        }
        0x63aa3 => {
            // block 63aa3
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63aa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63aa3 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63aa6 jmp 0x63b53
            pc = 0x63b53; continue;
        }
        0x63aab => {
            // block 63aab
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63aab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63aab mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x63aaf;
        }
        0x63aaf => {
            // block 63aaf
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63aaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63aaf cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 63ab2 jne 0x63c18
            if crate::cov::br(fl.ne()) { pc = 0x63c18; continue; }
            pc = 0x63ab8;
        }
        0x63ab8 => {
            // block 63ab8
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ab8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ab8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63aba jne 0x63bd3
            if crate::cov::br(fl.ne()) { pc = 0x63bd3; continue; }
            pc = 0x63ac0;
        }
        0x63ac0 => {
            // block 63ac0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ac0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ac0 lea rdi,[rbp-0xbf]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff41);
            // 63ac7 jmp 0x63bda
            pc = 0x63bda; continue;
        }
        0x63acc => {
            // block 63acc
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63acc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63acc mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x63ad0;
        }
        0x63ad0 => {
            // block 63ad0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ad0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ad0 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 63ad3 jne 0x63c45
            if crate::cov::br(fl.ne()) { pc = 0x63c45; continue; }
            pc = 0x63ad9;
        }
        0x63ad9 => {
            // block 63ad9
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ad9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ad9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63adb jne 0x63be3
            if crate::cov::br(fl.ne()) { pc = 0x63be3; continue; }
            pc = 0x63ae1;
        }
        0x63ae1 => {
            // block 63ae1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ae1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ae1 lea rdi,[rbp-0x8f]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff71);
            // 63ae8 jmp 0x63be7
            pc = 0x63be7; continue;
        }
        0x63aed => {
            // block 63aed
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63aed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63aed mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63af1;
        }
        0x63af1 => {
            // block 63af1
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63af1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63af1 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63af4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63af6 jne 0x63b7c
            if crate::cov::br(fl.ne()) { pc = 0x63b7c; continue; }
            pc = 0x63afc;
        }
        0x63afc => {
            // block 63afc
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63afc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63afc test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63aff je 0x63b8f
            if crate::cov::br(fl.e()) { pc = 0x63b8f; continue; }
            pc = 0x63b05;
        }
        0x63b05 => {
            // block 63b05
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b05 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63b09 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63b0c cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63b0e jne 0x63b1d
            if crate::cov::br(fl.ne()) { pc = 0x63b1d; continue; }
            pc = 0x63b10;
        }
        0x63b10 => {
            // block 63b10
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b10 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63b13 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63b16 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63b19 jne 0x63b05
            if crate::cov::br(fl.ne()) { pc = 0x63b05; continue; }
            pc = 0x63b1b;
        }
        0x63b1b => {
            // block 63b1b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b1b jmp 0x63b8f
            pc = 0x63b8f; continue;
        }
        0x63b1d => {
            // block 63b1d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b1d xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63b20 jmp 0x63b8f
            pc = 0x63b8f; continue;
        }
        0x63b22 => {
            // block 63b22
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b22 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63b26;
        }
        0x63b26 => {
            // block 63b26
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b26 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63b29 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63b2b jne 0x63b9b
            if crate::cov::br(fl.ne()) { pc = 0x63b9b; continue; }
            pc = 0x63b2d;
        }
        0x63b2d => {
            // block 63b2d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b2d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b30 je 0x63bae
            if crate::cov::br(fl.e()) { pc = 0x63bae; continue; }
            pc = 0x63b32;
        }
        0x63b32 => {
            // block 63b32
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b32 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63b36 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63b39 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63b3b jne 0x63b4a
            if crate::cov::br(fl.ne()) { pc = 0x63b4a; continue; }
            pc = 0x63b3d;
        }
        0x63b3d => {
            // block 63b3d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b3d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63b40 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63b43 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63b46 jne 0x63b32
            if crate::cov::br(fl.ne()) { pc = 0x63b32; continue; }
            pc = 0x63b48;
        }
        0x63b48 => {
            // block 63b48
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b48 jmp 0x63bae
            pc = 0x63bae; continue;
        }
        0x63b4a => {
            // block 63b4a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b4a xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63b4d jmp 0x63bae
            pc = 0x63bae; continue;
        }
        0x63b4f => {
            // block 63b4f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b4f mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63b53;
        }
        0x63b53 => {
            // block 63b53
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b53 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63b56 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63b58 jne 0x63bb7
            if crate::cov::br(fl.ne()) { pc = 0x63bb7; continue; }
            pc = 0x63b5a;
        }
        0x63b5a => {
            // block 63b5a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b5a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b5d je 0x63bca
            if crate::cov::br(fl.e()) { pc = 0x63bca; continue; }
            pc = 0x63b5f;
        }
        0x63b5f => {
            // block 63b5f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b5f movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63b63 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63b66 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63b68 jne 0x63b77
            if crate::cov::br(fl.ne()) { pc = 0x63b77; continue; }
            pc = 0x63b6a;
        }
        0x63b6a => {
            // block 63b6a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b6a inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63b6d inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63b70 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63b73 jne 0x63b5f
            if crate::cov::br(fl.ne()) { pc = 0x63b5f; continue; }
            pc = 0x63b75;
        }
        0x63b75 => {
            // block 63b75
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b75 jmp 0x63bca
            pc = 0x63bca; continue;
        }
        0x63b77 => {
            // block 63b77
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b77 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63b7a jmp 0x63bca
            pc = 0x63bca; continue;
        }
        0x63b7c => {
            // block 63b7c
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b7c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b7f je 0x63b8f
            if crate::cov::br(fl.e()) { pc = 0x63b8f; continue; }
            pc = 0x63b81;
        }
        0x63b81 => {
            // block 63b81
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b81 mov rsi,r15
            rsi = r15;
            // 63b84 call 0xec218
            { m.outregs(0x63b89, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63b89 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63b8b sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63b8f;
        }
        0x63b8f => {
            // block 63b8f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b8f lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 63b96 jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63b9b => {
            // block 63b9b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b9b test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b9e je 0x63bae
            if crate::cov::br(fl.e()) { pc = 0x63bae; continue; }
            pc = 0x63ba0;
        }
        0x63ba0 => {
            // block 63ba0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ba0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ba0 mov rsi,r15
            rsi = r15;
            // 63ba3 call 0xec218
            { m.outregs(0x63ba8, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63ba8 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63baa sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63bae;
        }
        0x63bae => {
            // block 63bae
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bae lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 63bb2 jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63bb7 => {
            // block 63bb7
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bb7 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63bba je 0x63bca
            if crate::cov::br(fl.e()) { pc = 0x63bca; continue; }
            pc = 0x63bbc;
        }
        0x63bbc => {
            // block 63bbc
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bbc mov rsi,r15
            rsi = r15;
            // 63bbf call 0xec218
            { m.outregs(0x63bc4, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63bc4 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63bc6 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63bca;
        }
        0x63bca => {
            // block 63bca
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bca lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 63bce jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63bd3 => {
            // block 63bd3
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bd3 mov rdi,QWORD PTR [rbp-0xb0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            pc = 0x63bda;
        }
        0x63bda => {
            // block 63bda
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bda test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63bdc jne 0x63bf0
            if crate::cov::br(fl.ne()) { pc = 0x63bf0; continue; }
            pc = 0x63bde;
        }
        0x63bde => {
            // block 63bde
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bde inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63be1 jmp 0x63bf4
            pc = 0x63bf4; continue;
        }
        0x63be3 => {
            // block 63be3
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63be3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63be3 mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            pc = 0x63be7;
        }
        0x63be7 => {
            // block 63be7
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63be7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63be7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63be9 jne 0x63c1d
            if crate::cov::br(fl.ne()) { pc = 0x63c1d; continue; }
            pc = 0x63beb;
        }
        0x63beb => {
            // block 63beb
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63beb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63beb inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63bee jmp 0x63c21
            pc = 0x63c21; continue;
        }
        0x63bf0 => {
            // block 63bf0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bf0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bf0 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63bf4;
        }
        0x63bf4 => {
            // block 63bf4
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bf4 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63bf7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63bf9 jne 0x63c4a
            if crate::cov::br(fl.ne()) { pc = 0x63c4a; continue; }
            pc = 0x63bfb;
        }
        0x63bfb => {
            // block 63bfb
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bfb test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63bfe je 0x63c5d
            if crate::cov::br(fl.e()) { pc = 0x63c5d; continue; }
            pc = 0x63c00;
        }
        0x63c00 => {
            // block 63c00
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c00 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63c04 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63c07 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63c09 jne 0x63c18
            if crate::cov::br(fl.ne()) { pc = 0x63c18; continue; }
            pc = 0x63c0b;
        }
        0x63c0b => {
            // block 63c0b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c0b inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63c0e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63c11 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63c14 jne 0x63c00
            if crate::cov::br(fl.ne()) { pc = 0x63c00; continue; }
            pc = 0x63c16;
        }
        0x63c16 => {
            // block 63c16
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c16 jmp 0x63c5d
            pc = 0x63c5d; continue;
        }
        0x63c18 => {
            // block 63c18
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c18 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63c1b jmp 0x63c5d
            pc = 0x63c5d; continue;
        }
        0x63c1d => {
            // block 63c1d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c1d mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63c21;
        }
        0x63c21 => {
            // block 63c21
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c21 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63c24 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63c26 jne 0x63c66
            if crate::cov::br(fl.ne()) { pc = 0x63c66; continue; }
            pc = 0x63c28;
        }
        0x63c28 => {
            // block 63c28
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c28 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63c2b je 0x63c79
            if crate::cov::br(fl.e()) { pc = 0x63c79; continue; }
            pc = 0x63c2d;
        }
        0x63c2d => {
            // block 63c2d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c2d movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63c31 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63c34 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63c36 jne 0x63c45
            if crate::cov::br(fl.ne()) { pc = 0x63c45; continue; }
            pc = 0x63c38;
        }
        0x63c38 => {
            // block 63c38
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c38 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63c3b inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63c3e dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63c41 jne 0x63c2d
            if crate::cov::br(fl.ne()) { pc = 0x63c2d; continue; }
            pc = 0x63c43;
        }
        0x63c43 => {
            // block 63c43
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c43 jmp 0x63c79
            pc = 0x63c79; continue;
        }
        0x63c45 => {
            // block 63c45
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c45 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63c48 jmp 0x63c79
            pc = 0x63c79; continue;
        }
        0x63c4a => {
            // block 63c4a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c4a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63c4d je 0x63c5d
            if crate::cov::br(fl.e()) { pc = 0x63c5d; continue; }
            pc = 0x63c4f;
        }
        0x63c4f => {
            // block 63c4f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c4f mov rsi,r15
            rsi = r15;
            // 63c52 call 0xec218
            { m.outregs(0x63c57, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c57 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63c59 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63c5d;
        }
        0x63c5d => {
            // block 63c5d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c5d lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 63c64 jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63c66 => {
            // block 63c66
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c66 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63c69 je 0x63c79
            if crate::cov::br(fl.e()) { pc = 0x63c79; continue; }
            pc = 0x63c6b;
        }
        0x63c6b => {
            // block 63c6b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c6b mov rsi,r15
            rsi = r15;
            // 63c6e call 0xec218
            { m.outregs(0x63c73, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c73 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63c75 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63c79;
        }
        0x63c79 => {
            // block 63c79
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c79 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            pc = 0x63c80;
        }
        0x63c80 => {
            // block 63c80
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c80 call 0xec032
            { m.outregs(0x63c85, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x63c85;
        }
        0x63c85 => {
            // block 63c85
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c85 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 63c89 call 0xec032
            { m.outregs(0x63c8e, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c8e mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 63c91 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // 63c98 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 63c99 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 63c9b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 63c9d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 63c9e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x63c9f => {
            // block 63c9f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c9f mov rbx,rax
            rbx = rax;
            // 63ca2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 63ca6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63cab mov rdi,rbx
            rdi = rbx;
            // 63cae call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4bd34 size=1526
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004bd34<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4bd34;
    loop {
        match pc {
        0x4bd34 => {
            // block 4bd34
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4bd34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bd34 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4bd35 mov rbp,rsp
            rbp = rsp;
            // 4bd38 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4bd3a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4bd3c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4bd3e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4bd40 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4bd41 sub rsp,0x188
            { let (r, f) = sub::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4bd48 mov r14,rdi
            r14 = rdi;
            // 4bd4b mov rbx,QWORD PTR [rip+0xf4686] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4bd52 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4bd55 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 4bd59 mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 4bd60 mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // 4bd64 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 4bd66 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 4bd6a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4bd6d xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4bd6f call QWORD PTR [rax+0x190]
            { m.outregs(0x4bd75, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bd75 mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 4bd7b test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4bd7d je 0x4bdb7
            if crate::cov::br(fl.e()) { pc = 0x4bdb7; continue; }
            pc = 0x4bd7f;
        }
        0x4bd7f => {
            // block 4bd7f
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4bd7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bd7f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4bd82 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4bd84 mov rdi,r14
            rdi = r14;
            // 4bd87 call QWORD PTR [rax+0x190]
            { m.outregs(0x4bd8d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bd8d lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4bd91 lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 4bd95 mov esi,0x9
            rsi = (0x9u64) as u32 as u64;
            // 4bd9a mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 4bd9f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4bda2 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4bda4 call 0xebf66
            { m.outregs(0x4bda9, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x733); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bda9 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4bdac mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4bdae test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4bdb1 je 0x4c267
            if crate::cov::br(fl.e()) { pc = 0x4c267; continue; }
            pc = 0x4bdb7;
        }
        0x4bdb7 => {
            // block 4bdb7
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4bdb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bdb7 test BYTE PTR [rip+0x17c4d3],0x2 # 0x1c8291
            fl = logic::<8>(m.r8(m.env.data_va(903774)) as u64 & 0x2u64);
            // 4bdbe je 0x4c265
            if crate::cov::br(fl.e()) { pc = 0x4c265; continue; }
            pc = 0x4bdc4;
        }
        0x4bdc4 => {
            // block 4bdc4
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4bdc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bdc4 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4bdcb lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4bdd2 mov rax,QWORD PTR [rip+0xf43f7] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4bdd9 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4bddd mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 4bde4 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4bde8 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4bdef mov rsi,rbx
            rsi = rbx;
            // 4bdf2 call 0xec0f8
            { m.outregs(0x4bdf7, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bdf7 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 4bdff mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 4be06 mov r12,QWORD PTR [rip+0xf45b3] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 4be0d lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 4be12 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 4be19 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4be20 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 4be24 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4be2b mov rdi,rbx
            rdi = rbx;
            // 4be2e call 0xec0c2
            { m.outregs(0x4be33, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4be33 mov rax,QWORD PTR [rip+0xf456e] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4be3a add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4be3e mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // 4be45 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4be4c mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 4be57 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 4be62 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 4be6d mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 4be78 mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 4be82 xorps xmm0,xmm0
            x0 = 0;
            // 4be85 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 4be89 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 4be91 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4be95 mov rdi,rbx
            rdi = rbx;
            // 4be98 call 0xebaa4
            { m.outregs(0x4be9d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4be9d lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4bea1 call 0xec032
            { m.outregs(0x4bea6, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bea6 call 0xec26c
            { m.outregs(0x4beab, 0x4, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa39); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4beab mov r15,rax
            r15 = rax;
            // 4beae lea rsi,[rip+0xdee0c] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4beb5 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4bebc mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4bec1 call 0xebb88
            { m.outregs(0x4bec6, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bec6 lea rsi,[rip+0xdedf8] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4becd mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4bed2 mov rdi,rax
            rdi = rax;
            // 4bed5 call 0xebb88
            { m.outregs(0x4beda, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4beda lea rsi,[rip+0xdea9f] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4bee1 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4bee6 mov rdi,rax
            rdi = rax;
            // 4bee9 call 0xebb88
            { m.outregs(0x4beee, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4beee lea rsi,[rip+0xde580] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4bef5 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4befa mov rdi,rax
            rdi = rax;
            // 4befd call 0xebb88
            { m.outregs(0x4bf02, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bf02 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4bf05 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bf09 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4bf0d and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4bf10 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4bf13 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4bf17 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4bf1a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bf1e mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4bf27 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4bf2a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bf2e mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4bf39 mov rdi,rax
            rdi = rax;
            // 4bf3c mov rsi,r15
            rsi = r15;
            // 4bf3f call 0xec0b6
            { m.outregs(0x4bf44, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bf44 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4bf47 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bf4b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4bf4f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4bf52 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4bf55 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4bf59 lea rsi,[rip+0xdea20] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4bf60 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4bf65 mov rdi,rax
            rdi = rax;
            // 4bf68 call 0xebb88
            { m.outregs(0x4bf6d, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bf6d mov rbx,rax
            rbx = rax;
            // 4bf70 lea rsi,[rip+0xdef4f] # 0x12aec6
            rsi = m.env.data_va(259731);
            // 4bf77 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4bf7e mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 4bf83 call 0xebfe4
            { m.outregs(0x4bf88, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bf88 movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 4bf8f test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4bf92 jne 0x4bfa0
            if crate::cov::br(fl.ne()) { pc = 0x4bfa0; continue; }
            pc = 0x4bf94;
        }
        0x4bf94 => {
            // block 4bf94
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4bf94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bf94 lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 4bf9b shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4bf9e jmp 0x4bfae
            pc = 0x4bfae; continue;
        }
        0x4bfa0 => {
            // block 4bfa0
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4bfa0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bfa0 mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 4bfa7 mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x4bfae;
        }
        0x4bfae => {
            // block 4bfae
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4bfae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4bfae mov rdi,rbx
            rdi = rbx;
            // 4bfb1 call 0xebb88
            { m.outregs(0x4bfb6, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bfb6 lea rsi,[rip+0xde9c3] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4bfbd mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4bfc2 mov rdi,rax
            rdi = rax;
            // 4bfc5 call 0xebb88
            { m.outregs(0x4bfca, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bfca lea rsi,[rip+0xded9f] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4bfd1 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4bfd6 mov rdi,rax
            rdi = rax;
            // 4bfd9 call 0xebb88
            { m.outregs(0x4bfde, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bfde lea rsi,[rip+0xde490] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4bfe5 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4bfea mov rdi,rax
            rdi = rax;
            // 4bfed call 0xebb88
            { m.outregs(0x4bff2, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4bff2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4bff5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4bff9 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4bffd and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4c000 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4c003 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4c007 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4c00a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c00e mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4c017 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4c01a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c01e mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4c029 mov rdi,rax
            rdi = rax;
            // 4c02c mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4c02f call 0xec09e
            { m.outregs(0x4c034, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c034 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4c037 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c03b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4c03f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4c042 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4c045 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4c049 lea rsi,[rip+0xde936] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4c050 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4c055 mov rdi,rax
            rdi = rax;
            // 4c058 call 0xebb88
            { m.outregs(0x4c05d, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c05d mov r15,rax
            r15 = rax;
            // 4c060 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4c063 call 0x474d5
            { m.outregs(0x4c068, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x474d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c068 mov rbx,rax
            rbx = rax;
            // 4c06b mov rdi,rbx
            rdi = rbx;
            // 4c06e call 0xec2c0
            { m.outregs(0x4c073, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c073 mov rdi,r15
            rdi = r15;
            // 4c076 mov rsi,rbx
            rsi = rbx;
            // 4c079 mov rdx,rax
            rdx = rax;
            // 4c07c call 0xebb88
            { m.outregs(0x4c081, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c081 lea rsi,[rip+0xde5fa] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4c088 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4c08d mov rdi,rax
            rdi = rax;
            // 4c090 call 0xebb88
            { m.outregs(0x4c095, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c095 lea rsi,[rip+0xdec5e] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4c09c mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4c0a1 mov rdi,rax
            rdi = rax;
            // 4c0a4 call 0xebb88
            { m.outregs(0x4c0a9, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c0a9 lea rsi,[rip+0xde3c5] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4c0b0 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4c0b5 mov rdi,rax
            rdi = rax;
            // 4c0b8 call 0xebb88
            { m.outregs(0x4c0bd, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c0bd mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4c0c0 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c0c4 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4c0c8 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4c0cb or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4c0ce mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4c0d2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4c0d5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c0d9 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4c0e2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4c0e5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c0e9 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4c0f4 mov rdi,rax
            rdi = rax;
            // 4c0f7 mov rsi,r14
            rsi = r14;
            // 4c0fa call 0xec0b6
            { m.outregs(0x4c0ff, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c0ff mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4c102 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c106 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4c10a and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4c10d or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4c110 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4c114 lea rsi,[rip+0xdebf1] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4c11b mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4c120 mov rdi,rax
            rdi = rax;
            // 4c123 call 0xebb88
            { m.outregs(0x4c128, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c128 lea rsi,[rip+0xde346] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4c12f mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4c134 mov rdi,rax
            rdi = rax;
            // 4c137 call 0xebb88
            { m.outregs(0x4c13c, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c13c mov rbx,rax
            rbx = rax;
            // 4c13f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4c142 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c146 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4c14a and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4c14d or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4c150 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4c154 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4c157 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c15b mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4c164 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4c167 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c16b mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4c176 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4c179 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4c180 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4c182 mov rdi,r14
            rdi = r14;
            // 4c185 call rax
            { m.outregs(0x4c187, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c187 mov rdi,rbx
            rdi = rbx;
            // 4c18a mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4c18c call 0xec09e
            { m.outregs(0x4c191, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c191 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4c194 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4c198 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4c19c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4c19f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4c1a2 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4c1a6 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4c1ad call 0xec032
            { m.outregs(0x4c1b2, 0xd, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c1b2 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4c1b9 lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4c1c0 call 0xeb8f4
            { m.outregs(0x4c1c5, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c1c5 test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 4c1cc jne 0x4c1d7
            if crate::cov::br(fl.ne()) { pc = 0x4c1d7; continue; }
            pc = 0x4c1ce;
        }
        0x4c1ce => {
            // block 4c1ce
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c1ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c1ce lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 4c1d5 jmp 0x4c1de
            pc = 0x4c1de; continue;
        }
        0x4c1d7 => {
            // block 4c1d7
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c1d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c1d7 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x4c1de;
        }
        0x4c1de => {
            // block 4c1de
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c1de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c1de mov rbx,QWORD PTR [rip+0xf41f3] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 4c1e5 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 4c1e8 lea rsi,[rip+0xdeb2a] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4c1ef mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4c1f4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4c1f6 call 0xec2e4
            { m.outregs(0x4c1fb, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xab1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c1fb lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4c202 call 0xec032
            { m.outregs(0x4c207, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c207 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4c20e mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4c215 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4c21c mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4c223 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4c22a lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4c231 call 0xec032
            { m.outregs(0x4c236, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c236 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4c23d call 0xec0c8
            { m.outregs(0x4c242, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c242 mov rsi,QWORD PTR [rip+0xf3fbf] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4c249 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4c24d lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4c254 call 0xec080
            { m.outregs(0x4c259, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c259 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4c260 call 0xec116
            { m.outregs(0x4c265, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4c265;
        }
        0x4c265 => {
            // block 4c265
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c265 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4c267;
        }
        0x4c267 => {
            // block 4c267
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c267 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c267 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 4c26b jne 0x4c2fd
            if crate::cov::br(fl.ne()) { pc = 0x4c2fd; continue; }
            pc = 0x4c271;
        }
        0x4c271 => {
            // block 4c271
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c271 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c271 add rsp,0x188
            { let (r, f) = add::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4c278 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4c279 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4c27b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4c27d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4c27f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4c281 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4c282 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4c283 => {
            // block 4c283
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c283 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c283 mov rbx,rax
            rbx = rax;
            // 4c286 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x4c28d;
        }
        0x4c28d => {
            // block 4c28d
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c28d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c28d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c292 jmp 0x4c297
            pc = 0x4c297; continue;
        }
        0x4c294 => {
            // block 4c294
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c294 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c294 mov rbx,rax
            rbx = rax;
            pc = 0x4c297;
        }
        0x4c297 => {
            // block 4c297
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c297 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c297 mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4c29e mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4c2a5 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 4c2ac mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4c2b3 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x4c2ba;
        }
        0x4c2ba => {
            // block 4c2ba
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c2ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c2ba lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4c2c1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c2c6 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4c2cd call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4c2d2;
        }
        0x4c2d2 => {
            // block 4c2d2
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c2d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c2d2 mov rsi,QWORD PTR [rip+0xf3f2f] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4c2d9 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4c2dd lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4c2e4 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4c2e9;
        }
        0x4c2e9 => {
            // block 4c2e9
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c2e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c2e9 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4c2f0 call 0xec116
            { m.outregs(0x4c2f5, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c2f5 mov rdi,rbx
            rdi = rbx;
            // 4c2f8 call 0xec182
            { m.outregs(0x4c2fd, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4c2fd;
        }
        0x4c2fd => {
            // block 4c2fd
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c2fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c2fd call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x4c307 => {
            // block 4c307
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c307 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c307 mov rbx,rax
            rbx = rax;
            // 4c30a jmp 0x4c2d2
            pc = 0x4c2d2; continue;
        }
        0x4c30c => {
            // block 4c30c
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c30c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c30c mov rbx,rax
            rbx = rax;
            // 4c30f lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4c313 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4c318 jmp 0x4c2ba
            pc = 0x4c2ba; continue;
        }
        0x4c31a => {
            // block 4c31a
            #[cfg(feature = "trace")] eprintln!("f_0004bd34 pc 4c31a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4c31a mov rbx,rax
            rbx = rax;
            // 4c31d lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4c324 jmp 0x4c28d
            pc = 0x4c28d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x49316 size=1528
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_00049316<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x49316;
    loop {
        match pc {
        0x49316 => {
            // block 49316
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 49316 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49316 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 49317 mov rbp,rsp
            rbp = rsp;
            // 4931a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4931c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4931e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 49320 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 49322 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 49323 sub rsp,0x188
            { let (r, f) = sub::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4932a mov r14,rdi
            r14 = rdi;
            // 4932d mov rbx,QWORD PTR [rip+0xf70a4] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 49334 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 49337 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 4933b mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 49342 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 49344 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 49348 movsxd rax,edx
            rax = sx::<32>((rdx as u32 as u64)) as u64;
            // 4934b mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 4934f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49352 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 49354 call QWORD PTR [rax+0x190]
            { m.outregs(0x4935a, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4935a mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 49360 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 49362 je 0x4939c
            if crate::cov::br(fl.e()) { pc = 0x4939c; continue; }
            pc = 0x49364;
        }
        0x49364 => {
            // block 49364
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 49364 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49364 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 49367 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 49369 mov rdi,r14
            rdi = r14;
            // 4936c call QWORD PTR [rax+0x190]
            { m.outregs(0x49372, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49372 lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 49376 lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 4937a mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 4937f mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 49384 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 49387 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 49389 call 0xebf66
            { m.outregs(0x4938e, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x733); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4938e mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 49391 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 49393 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 49396 je 0x4984c
            if crate::cov::br(fl.e()) { pc = 0x4984c; continue; }
            pc = 0x4939c;
        }
        0x4939c => {
            // block 4939c
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 4939c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4939c test BYTE PTR [rip+0x17eeed],0x20 # 0x1c8290
            fl = logic::<8>(m.r8(m.env.data_va(903773)) as u64 & 0x20u64);
            // 493a3 je 0x4984a
            if crate::cov::br(fl.e()) { pc = 0x4984a; continue; }
            pc = 0x493a9;
        }
        0x493a9 => {
            // block 493a9
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 493a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 493a9 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 493b0 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 493b7 mov rax,QWORD PTR [rip+0xf6e12] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 493be lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 493c2 mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 493c9 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 493cd mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 493d4 mov rsi,rbx
            rsi = rbx;
            // 493d7 call 0xec0f8
            { m.outregs(0x493dc, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 493dc mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 493e4 mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 493eb mov r12,QWORD PTR [rip+0xf6fce] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // 493f2 lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // 493f7 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 493fe mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 49405 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // 49409 mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 49410 mov rdi,rbx
            rdi = rbx;
            // 49413 call 0xec0c2
            { m.outregs(0x49418, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49418 mov rax,QWORD PTR [rip+0xf6f89] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4941f add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 49423 mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // 4942a mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 49431 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 4943c mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 49447 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 49452 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 4945d mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 49467 xorps xmm0,xmm0
            x0 = 0;
            // 4946a movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 4946e mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 49476 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4947a mov rdi,rbx
            rdi = rbx;
            // 4947d call 0xebaa4
            { m.outregs(0x49482, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49482 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 49486 call 0xec032
            { m.outregs(0x4948b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4948b call 0xec26c
            { m.outregs(0x49490, 0x4, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa39); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49490 mov r15,rax
            r15 = rax;
            // 49493 lea rsi,[rip+0xe1827] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4949a lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 494a1 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 494a6 call 0xebb88
            { m.outregs(0x494ab, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 494ab lea rsi,[rip+0xe1813] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 494b2 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 494b7 mov rdi,rax
            rdi = rax;
            // 494ba call 0xebb88
            { m.outregs(0x494bf, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 494bf lea rsi,[rip+0xe14ba] # 0x12a980
            rsi = m.env.data_va(258381);
            // 494c6 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 494cb mov rdi,rax
            rdi = rax;
            // 494ce call 0xebb88
            { m.outregs(0x494d3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 494d3 lea rsi,[rip+0xe0f9b] # 0x12a475
            rsi = m.env.data_va(257090);
            // 494da mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 494df mov rdi,rax
            rdi = rax;
            // 494e2 call 0xebb88
            { m.outregs(0x494e7, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 494e7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 494ea mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 494ee mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 494f2 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 494f5 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 494f8 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 494fc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 494ff mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49503 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4950c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4950f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49513 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4951e mov rdi,rax
            rdi = rax;
            // 49521 mov rsi,r15
            rsi = r15;
            // 49524 call 0xec0b6
            { m.outregs(0x49529, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49529 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4952c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49530 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49534 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49537 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4953a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4953e lea rsi,[rip+0xe143b] # 0x12a980
            rsi = m.env.data_va(258381);
            // 49545 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4954a mov rdi,rax
            rdi = rax;
            // 4954d call 0xebb88
            { m.outregs(0x49552, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49552 mov rbx,rax
            rbx = rax;
            // 49555 lea rsi,[rip+0xe18a0] # 0x12adfc
            rsi = m.env.data_va(259529);
            // 4955c lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 49563 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // 49568 call 0xebfe4
            { m.outregs(0x4956d, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4956d movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 49574 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 49577 jne 0x49585
            if crate::cov::br(fl.ne()) { pc = 0x49585; continue; }
            pc = 0x49579;
        }
        0x49579 => {
            // block 49579
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 49579 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49579 lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 49580 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 49583 jmp 0x49593
            pc = 0x49593; continue;
        }
        0x49585 => {
            // block 49585
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 49585 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49585 mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 4958c mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x49593;
        }
        0x49593 => {
            // block 49593
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 49593 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49593 mov rdi,rbx
            rdi = rbx;
            // 49596 call 0xebb88
            { m.outregs(0x4959b, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4959b lea rsi,[rip+0xe13de] # 0x12a980
            rsi = m.env.data_va(258381);
            // 495a2 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 495a7 mov rdi,rax
            rdi = rax;
            // 495aa call 0xebb88
            { m.outregs(0x495af, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 495af lea rsi,[rip+0xe17ba] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 495b6 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 495bb mov rdi,rax
            rdi = rax;
            // 495be call 0xebb88
            { m.outregs(0x495c3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 495c3 lea rsi,[rip+0xe0eab] # 0x12a475
            rsi = m.env.data_va(257090);
            // 495ca mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 495cf mov rdi,rax
            rdi = rax;
            // 495d2 call 0xebb88
            { m.outregs(0x495d7, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 495d7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 495da mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 495de mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 495e2 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 495e5 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 495e8 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 495ec mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 495ef mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 495f3 mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 495fc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 495ff mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49603 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4960e mov rdi,rax
            rdi = rax;
            // 49611 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 49614 call 0xec09e
            { m.outregs(0x49619, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49619 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4961c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49620 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49624 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49627 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4962a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4962e lea rsi,[rip+0xe1351] # 0x12a986
            rsi = m.env.data_va(258387);
            // 49635 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4963a mov rdi,rax
            rdi = rax;
            // 4963d call 0xebb88
            { m.outregs(0x49642, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49642 mov r15,rax
            r15 = rax;
            // 49645 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 49648 call 0x474d5
            { m.outregs(0x4964d, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x474d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4964d mov rbx,rax
            rbx = rax;
            // 49650 mov rdi,rbx
            rdi = rbx;
            // 49653 call 0xec2c0
            { m.outregs(0x49658, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49658 mov rdi,r15
            rdi = r15;
            // 4965b mov rsi,rbx
            rsi = rbx;
            // 4965e mov rdx,rax
            rdx = rax;
            // 49661 call 0xebb88
            { m.outregs(0x49666, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49666 lea rsi,[rip+0xe1015] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4966d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 49672 mov rdi,rax
            rdi = rax;
            // 49675 call 0xebb88
            { m.outregs(0x4967a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4967a lea rsi,[rip+0xe1679] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 49681 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 49686 mov rdi,rax
            rdi = rax;
            // 49689 call 0xebb88
            { m.outregs(0x4968e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4968e lea rsi,[rip+0xe0de0] # 0x12a475
            rsi = m.env.data_va(257090);
            // 49695 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4969a mov rdi,rax
            rdi = rax;
            // 4969d call 0xebb88
            { m.outregs(0x496a2, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 496a2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 496a5 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 496a9 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 496ad and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 496b0 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 496b3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 496b7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 496ba mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 496be mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 496c7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 496ca mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 496ce mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 496d9 mov rdi,rax
            rdi = rax;
            // 496dc mov rsi,r14
            rsi = r14;
            // 496df call 0xec0b6
            { m.outregs(0x496e4, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 496e4 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 496e7 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 496eb mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 496ef and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 496f2 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 496f5 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 496f9 lea rsi,[rip+0xe160c] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 49700 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 49705 mov rdi,rax
            rdi = rax;
            // 49708 call 0xebb88
            { m.outregs(0x4970d, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4970d lea rsi,[rip+0xe0d61] # 0x12a475
            rsi = m.env.data_va(257090);
            // 49714 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 49719 mov rdi,rax
            rdi = rax;
            // 4971c call 0xebb88
            { m.outregs(0x49721, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49721 mov rbx,rax
            rbx = rax;
            // 49724 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 49727 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4972b mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4972f and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 49732 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 49735 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 49739 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4973c mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49740 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 49749 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4974c mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 49750 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4975b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4975e mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 49765 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 49767 mov rdi,r14
            rdi = r14;
            // 4976a call rax
            { m.outregs(0x4976c, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4976c mov rdi,rbx
            rdi = rbx;
            // 4976f mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 49771 call 0xec09e
            { m.outregs(0x49776, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49776 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 49779 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4977d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 49781 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49784 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 49787 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4978b lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 49792 call 0xec032
            { m.outregs(0x49797, 0xd, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49797 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4979e lea rsi,[rbp-0x168]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 497a5 call 0xeb8f4
            { m.outregs(0x497aa, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 497aa test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 497b1 jne 0x497bc
            if crate::cov::br(fl.ne()) { pc = 0x497bc; continue; }
            pc = 0x497b3;
        }
        0x497b3 => {
            // block 497b3
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 497b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 497b3 lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 497ba jmp 0x497c3
            pc = 0x497c3; continue;
        }
        0x497bc => {
            // block 497bc
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 497bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 497bc mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x497c3;
        }
        0x497c3 => {
            // block 497c3
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 497c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 497c3 mov rbx,QWORD PTR [rip+0xf6c0e] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 497ca mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 497cd lea rsi,[rip+0xe1545] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 497d4 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 497d9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 497db call 0xec2e4
            { m.outregs(0x497e0, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xab1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 497e0 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 497e7 call 0xec032
            { m.outregs(0x497ec, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 497ec mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 497f3 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 497fa mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 49801 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 49808 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4980f lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 49816 call 0xec032
            { m.outregs(0x4981b, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4981b lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 49822 call 0xec0c8
            { m.outregs(0x49827, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49827 mov rsi,QWORD PTR [rip+0xf69da] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4982e add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 49832 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 49839 call 0xec080
            { m.outregs(0x4983e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4983e lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 49845 call 0xec116
            { m.outregs(0x4984a, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4984a;
        }
        0x4984a => {
            // block 4984a
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 4984a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4984a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4984c;
        }
        0x4984c => {
            // block 4984c
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 4984c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4984c cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 49850 jne 0x498e2
            if crate::cov::br(fl.ne()) { pc = 0x498e2; continue; }
            pc = 0x49856;
        }
        0x49856 => {
            // block 49856
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 49856 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49856 add rsp,0x188
            { let (r, f) = add::<64>(rsp, 0x188u64);
              rsp = r;
              fl = f; }
            // 4985d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4985e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 49860 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 49862 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 49864 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 49866 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 49867 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x49868 => {
            // block 49868
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 49868 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49868 mov rbx,rax
            rbx = rax;
            // 4986b lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x49872;
        }
        0x49872 => {
            // block 49872
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 49872 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49872 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 49877 jmp 0x4987c
            pc = 0x4987c; continue;
        }
        0x49879 => {
            // block 49879
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 49879 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 49879 mov rbx,rax
            rbx = rax;
            pc = 0x4987c;
        }
        0x4987c => {
            // block 4987c
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 4987c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4987c mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 49883 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4988a mov QWORD PTR [rbp-0x100],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (r12) as u64);
            // 49891 mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 49898 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            pc = 0x4989f;
        }
        0x4989f => {
            // block 4989f
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 4989f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4989f lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 498a6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 498ab lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 498b2 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x498b7;
        }
        0x498b7 => {
            // block 498b7
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 498b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 498b7 mov rsi,QWORD PTR [rip+0xf694a] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 498be add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 498c2 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 498c9 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x498ce;
        }
        0x498ce => {
            // block 498ce
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 498ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 498ce lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 498d5 call 0xec116
            { m.outregs(0x498da, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 498da mov rdi,rbx
            rdi = rbx;
            // 498dd call 0xec182
            { m.outregs(0x498e2, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x498e2;
        }
        0x498e2 => {
            // block 498e2
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 498e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 498e2 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x498ec => {
            // block 498ec
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 498ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 498ec mov rbx,rax
            rbx = rax;
            // 498ef jmp 0x498b7
            pc = 0x498b7; continue;
        }
        0x498f1 => {
            // block 498f1
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 498f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 498f1 mov rbx,rax
            rbx = rax;
            // 498f4 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 498f8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 498fd jmp 0x4989f
            pc = 0x4989f; continue;
        }
        0x498ff => {
            // block 498ff
            #[cfg(feature = "trace")] eprintln!("f_00049316 pc 498ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 498ff mov rbx,rax
            rbx = rax;
            // 49902 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 49909 jmp 0x49872
            pc = 0x49872; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4f47e size=1570
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_0004f47e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4f47e;
    loop {
        match pc {
        0x4f47e => {
            // block 4f47e
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f47e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f47e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4f47f mov rbp,rsp
            rbp = rsp;
            // 4f482 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4f484 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4f486 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4f488 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4f48a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4f48b sub rsp,0x198
            { let (r, f) = sub::<64>(rsp, 0x198u64);
              rsp = r;
              fl = f; }
            // 4f492 mov r14,rdi
            r14 = rdi;
            // 4f495 mov r12,QWORD PTR [rip+0xf0f3c] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // 4f49c mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // 4f4a0 mov QWORD PTR [rbp-0x30],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r12) as u64);
            // 4f4a4 mov DWORD PTR [rbp-0x64],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), (0x0u64) as u32);
            // 4f4ab mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // 4f4af mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 4f4b1 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 4f4b5 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 4f4b7 jne 0x4f9bd
            if crate::cov::br(fl.ne()) { pc = 0x4f9bd; continue; }
            pc = 0x4f4bd;
        }
        0x4f4bd => {
            // block 4f4bd
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f4bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f4bd mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4f4c0 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4f4c2 mov rdi,r14
            rdi = r14;
            // 4f4c5 call QWORD PTR [rax+0x190]
            { m.outregs(0x4f4cb, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f4cb mov r13d,0x5
            r13 = (0x5u64) as u32 as u64;
            // 4f4d1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4f4d3 je 0x4f50d
            if crate::cov::br(fl.e()) { pc = 0x4f50d; continue; }
            pc = 0x4f4d5;
        }
        0x4f4d5 => {
            // block 4f4d5
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f4d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f4d5 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4f4d8 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4f4da mov rdi,r14
            rdi = r14;
            // 4f4dd call QWORD PTR [rax+0x190]
            { m.outregs(0x4f4e3, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f4e3 lea rdx,[rbp-0x40]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 4f4e7 lea r9,[rbp-0x64]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ff9c);
            // 4f4eb mov esi,0x1b
            rsi = (0x1bu64) as u32 as u64;
            // 4f4f0 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 4f4f5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4f4f8 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4f4fa call 0xebf66
            { m.outregs(0x4f4ff, 0x3f, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x733); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f4ff mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4f502 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4f504 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 4f507 je 0x4f9bf
            if crate::cov::br(fl.e()) { pc = 0x4f9bf; continue; }
            pc = 0x4f50d;
        }
        0x4f50d => {
            // block 4f50d
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f50d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f50d test BYTE PTR [rip+0x178d7f],0x8 # 0x1c8293
            fl = logic::<8>(m.r8(m.env.data_va(903776)) as u64 & 0x8u64);
            // 4f514 je 0x4f9bd
            if crate::cov::br(fl.e()) { pc = 0x4f9bd; continue; }
            pc = 0x4f51a;
        }
        0x4f51a => {
            // block 4f51a
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f51a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f51a lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4f521 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4f528 mov rax,QWORD PTR [rip+0xf0ca1] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 4f52f lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4f533 mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 4f53a add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4f53e mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4f545 mov rsi,rbx
            rsi = rbx;
            // 4f548 call 0xec0f8
            { m.outregs(0x4f54d, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f54d mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 4f555 mov DWORD PTR [rbp-0x70],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0xffffffffu64) as u32);
            // 4f55c mov rax,QWORD PTR [rip+0xf0e5d] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 4f563 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 4f567 mov QWORD PTR [rbp-0x1b0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rcx) as u64);
            // 4f56e mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 4f575 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 4f579 mov QWORD PTR [rbp-0x1a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (rax) as u64);
            // 4f580 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4f587 mov rdi,rbx
            rdi = rbx;
            // 4f58a call 0xec0c2
            { m.outregs(0x4f58f, 0xd, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f58f mov rax,QWORD PTR [rip+0xf0e12] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 4f596 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 4f59a mov QWORD PTR [rbp-0x1b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (rax) as u64);
            // 4f5a1 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4f5a8 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // 4f5b3 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // 4f5be mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // 4f5c9 mov QWORD PTR [rbp-0x128],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (0x0u64) as u64);
            // 4f5d4 mov DWORD PTR [rbp-0x108],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x10u64) as u32);
            // 4f5de xorps xmm0,xmm0
            x0 = 0;
            // 4f5e1 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 4f5e5 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 4f5ed lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4f5f1 mov rdi,rbx
            rdi = rbx;
            // 4f5f4 call 0xebaa4
            { m.outregs(0x4f5f9, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f5f9 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4f5fd call 0xec032
            { m.outregs(0x4f602, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f602 call 0xec26c
            { m.outregs(0x4f607, 0x4, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa39); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f607 mov r15,rax
            r15 = rax;
            // 4f60a lea rsi,[rip+0xdb6b0] # 0x12acc1
            rsi = m.env.data_va(259214);
            // 4f611 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4f618 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4f61d call 0xebb88
            { m.outregs(0x4f622, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f622 lea rsi,[rip+0xdb69c] # 0x12acc5
            rsi = m.env.data_va(259218);
            // 4f629 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 4f62e mov rdi,rax
            rdi = rax;
            // 4f631 call 0xebb88
            { m.outregs(0x4f636, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f636 lea rsi,[rip+0xdb343] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4f63d mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4f642 mov rdi,rax
            rdi = rax;
            // 4f645 call 0xebb88
            { m.outregs(0x4f64a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f64a lea rsi,[rip+0xdae24] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4f651 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4f656 mov rdi,rax
            rdi = rax;
            // 4f659 call 0xebb88
            { m.outregs(0x4f65e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f65e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f661 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f665 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f669 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f66c or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f66f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f673 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f676 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f67a mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4f683 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f686 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f68a mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4f695 mov rdi,rax
            rdi = rax;
            // 4f698 mov rsi,r15
            rsi = r15;
            // 4f69b call 0xec0b6
            { m.outregs(0x4f6a0, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f6a0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f6a3 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f6a7 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f6ab and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f6ae or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f6b1 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f6b5 lea rsi,[rip+0xdb2c4] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4f6bc mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4f6c1 mov rdi,rax
            rdi = rax;
            // 4f6c4 call 0xebb88
            { m.outregs(0x4f6c9, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f6c9 mov r15,rax
            r15 = rax;
            // 4f6cc lea rsi,[rip+0xdb898] # 0x12af6b
            rsi = m.env.data_va(259896);
            // 4f6d3 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4f6da mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 4f6df call 0xebfe4
            { m.outregs(0x4f6e4, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f6e4 movzx edx,BYTE PTR [rbp-0x188]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 4f6eb test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 4f6ee jne 0x4f6fc
            if crate::cov::br(fl.ne()) { pc = 0x4f6fc; continue; }
            pc = 0x4f6f0;
        }
        0x4f6f0 => {
            // block 4f6f0
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f6f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f6f0 lea rsi,[rbp-0x187]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe79);
            // 4f6f7 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 4f6fa jmp 0x4f70a
            pc = 0x4f70a; continue;
        }
        0x4f6fc => {
            // block 4f6fc
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f6fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f6fc mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 4f703 mov rsi,QWORD PTR [rbp-0x178]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            pc = 0x4f70a;
        }
        0x4f70a => {
            // block 4f70a
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f70a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f70a mov rdi,r15
            rdi = r15;
            // 4f70d call 0xebb88
            { m.outregs(0x4f712, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f712 lea rsi,[rip+0xdb267] # 0x12a980
            rsi = m.env.data_va(258381);
            // 4f719 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 4f71e mov rdi,rax
            rdi = rax;
            // 4f721 call 0xebb88
            { m.outregs(0x4f726, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f726 lea rsi,[rip+0xdb643] # 0x12ad70
            rsi = m.env.data_va(259389);
            // 4f72d mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 4f732 mov rdi,rax
            rdi = rax;
            // 4f735 call 0xebb88
            { m.outregs(0x4f73a, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f73a lea rsi,[rip+0xdad34] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4f741 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4f746 mov rdi,rax
            rdi = rax;
            // 4f749 call 0xebb88
            { m.outregs(0x4f74e, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f74e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f751 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f755 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f759 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f75c or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f75f mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f763 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f766 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f76a mov QWORD PTR [rcx+rax*1+0x18],0x8
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x8u64) as u64);
            // 4f773 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f776 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f77a mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4f785 mov rdi,rax
            rdi = rax;
            // 4f788 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 4f78b call 0xec09e
            { m.outregs(0x4f790, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f790 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f793 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f797 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f79b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f79e or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f7a1 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f7a5 lea rsi,[rip+0xdb1da] # 0x12a986
            rsi = m.env.data_va(258387);
            // 4f7ac mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4f7b1 mov rdi,rax
            rdi = rax;
            // 4f7b4 call 0xebb88
            { m.outregs(0x4f7b9, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f7b9 mov r15,rax
            r15 = rax;
            // 4f7bc mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // 4f7bf call 0x474d5
            { m.outregs(0x4f7c4, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.code_va(0x474d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f7c4 mov rbx,rax
            rbx = rax;
            // 4f7c7 mov rdi,rbx
            rdi = rbx;
            // 4f7ca call 0xec2c0
            { m.outregs(0x4f7cf, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f7cf mov rdi,r15
            rdi = r15;
            // 4f7d2 mov rsi,rbx
            rsi = rbx;
            // 4f7d5 mov rdx,rax
            rdx = rax;
            // 4f7d8 call 0xebb88
            { m.outregs(0x4f7dd, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f7dd lea rsi,[rip+0xdae9e] # 0x12a682
            rsi = m.env.data_va(257615);
            // 4f7e4 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4f7e9 mov rdi,rax
            rdi = rax;
            // 4f7ec call 0xebb88
            { m.outregs(0x4f7f1, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f7f1 lea rsi,[rip+0xdb502] # 0x12acfa
            rsi = m.env.data_va(259271);
            // 4f7f8 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 4f7fd mov rdi,rax
            rdi = rax;
            // 4f800 call 0xebb88
            { m.outregs(0x4f805, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f805 lea rsi,[rip+0xdac69] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4f80c mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4f811 mov rdi,rax
            rdi = rax;
            // 4f814 call 0xebb88
            { m.outregs(0x4f819, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f819 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f81c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f820 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f824 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f827 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f82a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f82e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f831 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f835 mov QWORD PTR [rcx+rax*1+0x18],0x10
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x10u64) as u64);
            // 4f83e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f841 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f845 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // 4f850 mov rdi,rax
            rdi = rax;
            // 4f853 mov rsi,r14
            rsi = r14;
            // 4f856 call 0xec0b6
            { m.outregs(0x4f85b, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f85b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f85e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f862 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f866 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f869 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f86c mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f870 lea rsi,[rip+0xdb495] # 0x12ad0c
            rsi = m.env.data_va(259289);
            // 4f877 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 4f87c mov rdi,rax
            rdi = rax;
            // 4f87f call 0xebb88
            { m.outregs(0x4f884, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f884 lea rsi,[rip+0xdabea] # 0x12a475
            rsi = m.env.data_va(257090);
            // 4f88b mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 4f890 mov rdi,rax
            rdi = rax;
            // 4f893 call 0xebb88
            { m.outregs(0x4f898, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f898 mov rbx,rax
            rbx = rax;
            // 4f89b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4f89e mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f8a2 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f8a6 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4f8a9 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4f8ac mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 4f8b0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4f8b3 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f8b7 mov QWORD PTR [rax+rbx*1+0x18],0x8
            m.w64(rax.wrapping_add(rbx).wrapping_add(0x18), (0x8u64) as u64);
            // 4f8c0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4f8c3 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f8c7 mov DWORD PTR [rax+rbx*1+0x90],0x30
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x30u64) as u32);
            // 4f8d2 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 4f8d5 mov rax,QWORD PTR [rax+0x190]
            rax = m.r64(rax.wrapping_add(0x190)) as u64;
            // 4f8dc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4f8de mov rdi,r14
            rdi = r14;
            // 4f8e1 call rax
            { m.outregs(0x4f8e3, 0xf, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f8e3 mov rdi,rbx
            rdi = rbx;
            // 4f8e6 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 4f8e8 call 0xec09e
            { m.outregs(0x4f8ed, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f8ed mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 4f8f0 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 4f8f4 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 4f8f8 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f8fb or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 4f8fe mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 4f902 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 4f909 call 0xec032
            { m.outregs(0x4f90e, 0xd, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f90e lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4f915 lea rbx,[rbp-0x168]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4f91c mov rsi,rbx
            rsi = rbx;
            // 4f91f call 0xeb8f4
            { m.outregs(0x4f924, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f924 test BYTE PTR [rbp-0x1a0],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64 & 0x1u64);
            // 4f92b jne 0x4f936
            if crate::cov::br(fl.ne()) { pc = 0x4f936; continue; }
            pc = 0x4f92d;
        }
        0x4f92d => {
            // block 4f92d
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f92d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f92d lea rdx,[rbp-0x19f]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fe61);
            // 4f934 jmp 0x4f93d
            pc = 0x4f93d; continue;
        }
        0x4f936 => {
            // block 4f936
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f936 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f936 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            pc = 0x4f93d;
        }
        0x4f93d => {
            // block 4f93d
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f93d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f93d lea rsi,[rip+0xdb3d5] # 0x12ad19
            rsi = m.env.data_va(259302);
            // 4f944 mov edi,0x3
            rdi = (0x3u64) as u32 as u64;
            // 4f949 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4f94b call 0xec2e4
            { m.outregs(0x4f950, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0xab1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f950 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4f957 call 0xec032
            { m.outregs(0x4f95c, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f95c mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4f963 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4f96a mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4f971 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4f978 mov rax,QWORD PTR [rbp-0x1b8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 4f97f mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4f986 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4f98d call 0xec032
            { m.outregs(0x4f992, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f992 mov rdi,rbx
            rdi = rbx;
            // 4f995 call 0xec0c8
            { m.outregs(0x4f99a, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f99a mov rsi,QWORD PTR [rip+0xf0867] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4f9a1 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4f9a5 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4f9ac call 0xec080
            { m.outregs(0x4f9b1, 0x7, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f9b1 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4f9b8 call 0xec116
            { m.outregs(0x4f9bd, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4f9bd;
        }
        0x4f9bd => {
            // block 4f9bd
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f9bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f9bd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4f9bf;
        }
        0x4f9bf => {
            // block 4f9bf
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f9bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f9bf cmp r12,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 4f9c3 jne 0x4fa5c
            if crate::cov::br(fl.ne()) { pc = 0x4fa5c; continue; }
            pc = 0x4f9c9;
        }
        0x4f9c9 => {
            // block 4f9c9
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f9c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f9c9 add rsp,0x198
            { let (r, f) = add::<64>(rsp, 0x198u64);
              rsp = r;
              fl = f; }
            // 4f9d0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4f9d1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4f9d3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4f9d5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4f9d7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4f9d9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4f9da ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4f9db => {
            // block 4f9db
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f9db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f9db mov rbx,rax
            rbx = rax;
            // 4f9de lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            pc = 0x4f9e5;
        }
        0x4f9e5 => {
            // block 4f9e5
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f9e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f9e5 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4f9ea jmp 0x4f9ef
            pc = 0x4f9ef; continue;
        }
        0x4f9ec => {
            // block 4f9ec
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f9ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f9ec mov rbx,rax
            rbx = rax;
            pc = 0x4f9ef;
        }
        0x4f9ef => {
            // block 4f9ef
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4f9ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4f9ef mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 4f9f6 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 4f9fd mov rax,QWORD PTR [rbp-0x1a8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 4fa04 mov QWORD PTR [rbp-0x100],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rax) as u64);
            // 4fa0b mov rax,QWORD PTR [rbp-0x1b8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 4fa12 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 4fa19 lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4fa20 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fa25 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 4fa2c call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4fa31;
        }
        0x4fa31 => {
            // block 4fa31
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4fa31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fa31 mov rsi,QWORD PTR [rip+0xf07d0] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 4fa38 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 4fa3c lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // 4fa43 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4fa48;
        }
        0x4fa48 => {
            // block 4fa48
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4fa48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fa48 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // 4fa4f call 0xec116
            { m.outregs(0x4fa54, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fa54 mov rdi,rbx
            rdi = rbx;
            // 4fa57 call 0xec182
            { m.outregs(0x4fa5c, 0x5, [rdi, rsi, rdx, rcx, r8, r9]); let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4fa5c;
        }
        0x4fa5c => {
            // block 4fa5c
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4fa5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fa5c call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; return Ret::default(); }
        }
        0x4fa66 => {
            // block 4fa66
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4fa66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fa66 mov rbx,rax
            rbx = rax;
            // 4fa69 jmp 0x4fa31
            pc = 0x4fa31; continue;
        }
        0x4fa6b => {
            // block 4fa6b
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4fa6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fa6b mov r14,rbx
            r14 = rbx;
            // 4fa6e mov rbx,rax
            rbx = rax;
            // 4fa71 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 4fa75 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fa7a lea rdi,[rbp-0x128]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fed8);
            // 4fa81 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fa86 mov rdi,r14
            rdi = r14;
            // 4fa89 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4fa8e jmp 0x4fa31
            pc = 0x4fa31; continue;
        }
        0x4fa90 => {
            // block 4fa90
            #[cfg(feature = "trace")] eprintln!("f_0004f47e pc 4fa90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4fa90 mov rbx,rax
            rbx = rax;
            // 4fa93 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 4fa9a jmp 0x4f9e5
            pc = 0x4f9e5; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
