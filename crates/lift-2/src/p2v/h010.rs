//! Mechanically lifted functions (scripts/lift_run.py), module h010. Each one is checked against the original
//! in the world harness (tests/world_h010.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x2946a size=310
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002946a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2946a;
    loop {
        match pc {
        0x2946a => {
            // block 2946a
            #[cfg(feature = "trace")] eprintln!("f_0002946a pc 2946a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2946a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2946b mov rbp,rsp
            rbp = rsp;
            // 2946e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2946f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 29470 mov rbx,rdi
            rbx = rdi;
            // 29473 movabs rax,0x0
            rax = 0x0u64;
            // 2947d mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 29480 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 29488 movabs rax,0x8000000c8
            rax = 0x0_0008_0000_00c8u64;
            // 29492 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 29496 lea rdi,[rbx+0x50]
            rdi = rbx.wrapping_add(0x50);
            // 2949a mov QWORD PTR [rbx+0x70],0x0
            m.w64(rbx.wrapping_add(0x70), (0x0u64) as u64);
            // 294a2 mov QWORD PTR [rbx+0x68],0x0
            m.w64(rbx.wrapping_add(0x68), (0x0u64) as u64);
            // 294aa mov QWORD PTR [rbx+0x60],0x0
            m.w64(rbx.wrapping_add(0x60), (0x0u64) as u64);
            // 294b2 mov QWORD PTR [rbx+0x58],0x0
            m.w64(rbx.wrapping_add(0x58), (0x0u64) as u64);
            // 294ba mov QWORD PTR [rbx+0x50],0x0
            m.w64(rbx.wrapping_add(0x50), (0x0u64) as u64);
            // 294c2 mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // 294ca mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 294d2 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 294da mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 294e2 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 294ea mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 294f2 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 294fa mov esi,0x144
            rsi = (0x144u64) as u32 as u64;
            // 294ff call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29504 mov rdi,QWORD PTR [rbx+0x50]
            rdi = m.r64(rbx.wrapping_add(0x50)) as u64;
            // 29508 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2950b je 0x29515
            if fl.e() { pc = 0x29515; continue; }
            pc = 0x2950d;
        }
        0x2950d => {
            // block 2950d
            #[cfg(feature = "trace")] eprintln!("f_0002946a pc 2950d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2950d mov esi,DWORD PTR [rbx+0x58]
            rsi = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // 29510 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x29515;
        }
        0x29515 => {
            // block 29515
            #[cfg(feature = "trace")] eprintln!("f_0002946a pc 29515 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29515 mov DWORD PTR [rbx+0xb0],0x0
            m.w32(rbx.wrapping_add(0xb0), (0x0u64) as u32);
            // 2951f mov QWORD PTR [rbx+0xa8],0x0
            m.w64(rbx.wrapping_add(0xa8), (0x0u64) as u64);
            // 2952a mov QWORD PTR [rbx+0xa0],0x0
            m.w64(rbx.wrapping_add(0xa0), (0x0u64) as u64);
            // 29535 mov QWORD PTR [rbx+0x98],0x0
            m.w64(rbx.wrapping_add(0x98), (0x0u64) as u64);
            // 29540 mov QWORD PTR [rbx+0x90],0x0
            m.w64(rbx.wrapping_add(0x90), (0x0u64) as u64);
            // 2954b mov QWORD PTR [rbx+0x88],0x0
            m.w64(rbx.wrapping_add(0x88), (0x0u64) as u64);
            // 29556 mov QWORD PTR [rbx+0x80],0x0
            m.w64(rbx.wrapping_add(0x80), (0x0u64) as u64);
            // 29561 mov QWORD PTR [rbx+0x78],0x0
            m.w64(rbx.wrapping_add(0x78), (0x0u64) as u64);
            // 29569 mov DWORD PTR [rbx+0xb4],0xffffffff
            m.w32(rbx.wrapping_add(0xb4), (0xffffffffu64) as u32);
            // 29573 mov DWORD PTR [rbx+0xb8],0xffffffff
            m.w32(rbx.wrapping_add(0xb8), (0xffffffffu64) as u32);
            // 2957d mov DWORD PTR [rbx+0xbc],0xffffffff
            m.w32(rbx.wrapping_add(0xbc), (0xffffffffu64) as u32);
            // 29587 movabs rax,0x6e74763200000000
            rax = 0x6e74_7632_0000_0000u64;
            // 29591 mov QWORD PTR [rbx+0xc0],rax
            m.w64(rbx.wrapping_add(0xc0), (rax) as u64);
            // 29598 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2959c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2959d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2959e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x26b10 size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00026b10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x26b10;
    loop {
        match pc {
        0x26b10 => {
            // block 26b10
            #[cfg(feature = "trace")] eprintln!("f_00026b10 pc 26b10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26b10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 26b11 mov rbp,rsp
            rbp = rsp;
            // 26b14 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 26b16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 26b17 mov r14,rsi
            r14 = rsi;
            // 26b1a mov rbx,rdi
            rbx = rdi;
            // 26b1d mov eax,DWORD PTR [rbx+0xc]
            rax = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // 26b20 lea rcx,[rip+0x10394b] # 0x12a472
            rcx = m.env.data_va(257087);
            // 26b27 lea rsi,[rip+0x103947] # 0x12a475
            rsi = m.env.data_va(257090);
            // 26b2e test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 26b30 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // 26b34 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 26b39 mov rdi,r14
            rdi = r14;
            // 26b3c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26b41 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26b44 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26b48 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26b4c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26b4f or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26b52 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 26b56 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 26b59 mov rdi,rax
            rdi = rax;
            // 26b5c call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26b61 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26b64 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26b68 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26b6c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26b6f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26b72 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 26b76 lea rsi,[rip+0x1032a9] # 0x129e26
            rsi = m.env.data_va(255475);
            // 26b7d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 26b82 mov rdi,rax
            rdi = rax;
            // 26b85 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26b8a mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 26b8d mov rdi,rax
            rdi = rax;
            // 26b90 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26b95 mov rax,r14
            rax = r14;
            // 26b98 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 26b99 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 26b9b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 26b9c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a600 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a600<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5a600;
    loop {
        match pc {
        0x5a600 => {
            // block 5a600
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a600 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a600 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a601 mov rbp,rsp
            rbp = rsp;
            // 5a604 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5a606 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5a608 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5a609 movzx r9d,cl
            r9 = ((rcx as u8 as u64)) as u32 as u64;
            // 5a60d mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 5a613 xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // 5a616 lea r11,[rip+0xa6733] # 0x100d50
            r11 = m.env.data_va(87325);
            // 5a61d lea rbx,[rip+0xa691c] # 0x100f40
            rbx = m.env.data_va(87821);
            // 5a624 lea r14,[rip+0xa6b05] # 0x101130
            r14 = m.env.data_va(88317);
            // 5a62b lea r15,[rip+0xa6d5e] # 0x101390
            r15 = m.env.data_va(88925);
            pc = 0x5a632;
        }
        0x5a632 => {
            // block 5a632
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a632 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a632 cmp DWORD PTR [r10+r11*1],edi
            fl = sub::<32>(m.r32(r10.wrapping_add(r11)) as u64, (rdi as u32 as u64)).1;
            // 5a636 jne 0x5a669
            if fl.ne() { pc = 0x5a669; continue; }
            pc = 0x5a638;
        }
        0x5a638 => {
            // block 5a638
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a638 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a638 cmp esi,DWORD PTR [r10+rbx*1+0x4]
            fl = sub::<32>((rsi as u32 as u64), m.r32(r10.wrapping_add(rbx).wrapping_add(0x4)) as u64).1;
            // 5a63d jne 0x5a669
            if fl.ne() { pc = 0x5a669; continue; }
            pc = 0x5a63f;
        }
        0x5a63f => {
            // block 5a63f
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a63f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a63f cmp edx,DWORD PTR [r10+r14*1]
            fl = sub::<32>((rdx as u32 as u64), m.r32(r10.wrapping_add(r14)) as u64).1;
            // 5a643 jne 0x5a669
            if fl.ne() { pc = 0x5a669; continue; }
            pc = 0x5a645;
        }
        0x5a645 => {
            // block 5a645
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a645 mov ecx,DWORD PTR [r10+r15*1]
            rcx = (m.r32(r10.wrapping_add(r15)) as u64) as u32 as u64;
            // 5a649 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5a64b cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 5a64e ja 0x5a65c
            if fl.a() { pc = 0x5a65c; continue; }
            pc = 0x5a650;
        }
        0x5a650 => {
            // block 5a650
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a650 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a650 mov eax,0x7a9
            rax = (0x7a9u64) as u32 as u64;
            // 5a655 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5a657 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a65a jmp 0x5a65e
            pc = 0x5a65e; continue;
        }
        0x5a65c => {
            // block 5a65c
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a65c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a65c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5a65e;
        }
        0x5a65e => {
            // block 5a65e
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a65e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a65e movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 5a661 xor eax,0x1
            { let r = (rax as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a664 cmp r9d,eax
            fl = sub::<32>((r9 as u32 as u64), (rax as u32 as u64)).1;
            // 5a667 je 0x5a67a
            if fl.e() { pc = 0x5a67a; continue; }
            pc = 0x5a669;
        }
        0x5a669 => {
            // block 5a669
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a669 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a669 inc r8
            { let (r, f) = inc::<64>(r8, fl); r8 = r; fl = f; }
            // 5a66c add r10,0x4
            { let (r, f) = add::<64>(r10, 0x4u64);
              r10 = r;
              fl = f; }
            // 5a670 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a672 cmp r8,0x7a
            fl = sub::<64>(r8, 0x7au64).1;
            // 5a676 jl 0x5a632
            if fl.l() { pc = 0x5a632; continue; }
            pc = 0x5a678;
        }
        0x5a678 => {
            // block 5a678
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a678 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a678 jmp 0x5a67d
            pc = 0x5a67d; continue;
        }
        0x5a67a => {
            // block 5a67a
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a67a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a67a mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            pc = 0x5a67d;
        }
        0x5a67d => {
            // block 5a67d
            #[cfg(feature = "trace")] eprintln!("f_0005a600 pc 5a67d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a67d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5a67e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5a680 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5a682 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a683 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1b1f4 size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001b1f4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1b1f4;
    loop {
        match pc {
        0x1b1f4 => {
            // block 1b1f4
            #[cfg(feature = "trace")] eprintln!("f_0001b1f4 pc 1b1f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b1f4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1b1f5 mov rbp,rsp
            rbp = rsp;
            // 1b1f8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1b1fa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1b1fc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1b1fe push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1b200 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1b201 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1b205 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1b209 mov r12,rdi
            r12 = rdi;
            // 1b20c mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1b210 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1b215 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1b218 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1b21c inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1b21f mov rax,rbx
            rax = rbx;
            // 1b222 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1b226 je 0x1b234
            if fl.e() { pc = 0x1b234; continue; }
            pc = 0x1b228;
        }
        0x1b228 => {
            // block 1b228
            #[cfg(feature = "trace")] eprintln!("f_0001b1f4 pc 1b228 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b228 mov rdi,r12
            rdi = r12;
            // 1b22b call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x1b234 => {
            // block 1b234
            #[cfg(feature = "trace")] eprintln!("f_0001b1f4 pc 1b234 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b234 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1b239 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1b23e mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1b242 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1b245 mov rcx,r15
            rcx = r15;
            // 1b248 sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1b24c movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1b256 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1b259 ja 0x1b267
            if fl.a() { pc = 0x1b267; continue; }
            pc = 0x1b25b;
        }
        0x1b25b => {
            // block 1b25b
            #[cfg(feature = "trace")] eprintln!("f_0001b1f4 pc 1b25b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b25b sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1b25e cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1b261 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1b265 jmp 0x1b271
            pc = 0x1b271; continue;
        }
        0x1b267 => {
            // block 1b267
            #[cfg(feature = "trace")] eprintln!("f_0001b1f4 pc 1b267 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b267 movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1b271;
        }
        0x1b271 => {
            // block 1b271
            #[cfg(feature = "trace")] eprintln!("f_0001b1f4 pc 1b271 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b271 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1b276 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1b27a sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1b27d mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1b281 mov r13,r14
            r13 = r14;
            // 1b284 sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1b288 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1b290 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1b294 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1b296 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1b299 je 0x1b2ab
            if fl.e() { pc = 0x1b2ab; continue; }
            pc = 0x1b29b;
        }
        0x1b29b => {
            // block 1b29b
            #[cfg(feature = "trace")] eprintln!("f_0001b1f4 pc 1b29b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b29b lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1b2a3 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b2a8 mov rbx,rax
            rbx = rax;
            pc = 0x1b2ab;
        }
        0x1b2ab => {
            // block 1b2ab
            #[cfg(feature = "trace")] eprintln!("f_0001b1f4 pc 1b2ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b2ab lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1b2af mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1b2b3 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1b2b7 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1b2b9 mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1b2bd lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1b2c2 mov rdi,rbx
            rdi = rbx;
            // 1b2c5 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1b2c9 mov rsi,r15
            rsi = r15;
            // 1b2cc mov rdx,r14
            rdx = r14;
            // 1b2cf call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b2d4 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1b2d8 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1b2dc mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1b2e1 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1b2e5 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1b2e9 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1b2ed mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1b2f2 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1b2f6 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1b2fa mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1b2fe lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1b302 call 0xeb972
            { let r = { let t = m.env.stub_va(0x13f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b307 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1b30b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1b30c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1b30e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1b310 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1b312 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1b314 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1b315 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e3a4 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e3a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e3a4;
    loop {
        match pc {
        0x2e3a4 => {
            // block 2e3a4
            #[cfg(feature = "trace")] eprintln!("f_0002e3a4 pc 2e3a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e3a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e3a5 mov rbp,rsp
            rbp = rsp;
            // 2e3a8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e3aa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e3ac push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e3ae push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e3af mov r14,rdi
            r14 = rdi;
            // 2e3b2 lea rdi,[r14+0x28]
            rdi = r14.wrapping_add(0x28);
            // 2e3b6 lea rsi,[rip+0x10a90b] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 2e3bd call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e3c2 mov al,BYTE PTR [r14+0x8]
            rax = (rax & !0xff) | ((m.r8(r14.wrapping_add(0x8)) as u64) as u8 as u64);
            // 2e3c6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2e3c8 jne 0x2e41e
            if fl.ne() { pc = 0x2e41e; continue; }
            pc = 0x2e3ca;
        }
        0x2e3ca => {
            // block 2e3ca
            #[cfg(feature = "trace")] eprintln!("f_0002e3a4 pc 2e3ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e3ca lea r15,[rip+0x18f33f] # 0x1bd710
            r15 = m.env.data_va(859869);
            // 2e3d1 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 2e3d4 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 2e3d7 je 0x2e417
            if fl.e() { pc = 0x2e417; continue; }
            pc = 0x2e3d9;
        }
        0x2e3d9 => {
            // block 2e3d9
            #[cfg(feature = "trace")] eprintln!("f_0002e3a4 pc 2e3d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e3d9 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x2e3dc;
        }
        0x2e3dc => {
            // block 2e3dc
            #[cfg(feature = "trace")] eprintln!("f_0002e3a4 pc 2e3dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e3dc call rcx
            { let r = { let t = rcx; m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e3de jmp 0x2e3f5
            pc = 0x2e3f5; continue;
        }
        0x2e3e0 => {
            // block 2e3e0
            #[cfg(feature = "trace")] eprintln!("f_0002e3a4 pc 2e3e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e3e0 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 2e3e3 mov rdi,r14
            rdi = r14;
            // 2e3e6 mov rsi,rbx
            rsi = rbx;
            // 2e3e9 call QWORD PTR [rax+0x78]
            { let r = { let t = m.r64(rax.wrapping_add(0x78)) as u64; m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e3ec mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2e3ef mov rdi,rbx
            rdi = rbx;
            // 2e3f2 call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e3f5;
        }
        0x2e3f5 => {
            // block 2e3f5
            #[cfg(feature = "trace")] eprintln!("f_0002e3a4 pc 2e3f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e3f5 mov rbx,rax
            rbx = rax;
            // 2e3f8 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2e3fb je 0x2e40a
            if fl.e() { pc = 0x2e40a; continue; }
            pc = 0x2e3fd;
        }
        0x2e3fd => {
            // block 2e3fd
            #[cfg(feature = "trace")] eprintln!("f_0002e3a4 pc 2e3fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e3fd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2e400 mov rdi,rbx
            rdi = rbx;
            // 2e403 call QWORD PTR [rax+0x10]
            { let r = { let t = m.r64(rax.wrapping_add(0x10)) as u64; m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e406 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e408 je 0x2e3e0
            if fl.e() { pc = 0x2e3e0; continue; }
            pc = 0x2e40a;
        }
        0x2e40a => {
            // block 2e40a
            #[cfg(feature = "trace")] eprintln!("f_0002e3a4 pc 2e40a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e40a mov rcx,QWORD PTR [r15+r12*8+0x8]
            rcx = m.r64(r15.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x8)) as u64;
            // 2e40f inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 2e412 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 2e415 jne 0x2e3dc
            if fl.ne() { pc = 0x2e3dc; continue; }
            pc = 0x2e417;
        }
        0x2e417 => {
            // block 2e417
            #[cfg(feature = "trace")] eprintln!("f_0002e3a4 pc 2e417 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e417 mov BYTE PTR [r14+0x8],0x1
            m.w8(r14.wrapping_add(0x8), (0x1u64) as u8);
            // 2e41c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x2e41e;
        }
        0x2e41e => {
            // block 2e41e
            #[cfg(feature = "trace")] eprintln!("f_0002e3a4 pc 2e41e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e41e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e41f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e421 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e423 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e425 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e426 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x198fe size=160
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000198fe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x198fe;
    loop {
        match pc {
        0x198fe => {
            // block 198fe
            #[cfg(feature = "trace")] eprintln!("f_000198fe pc 198fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 198fe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 198ff mov rbp,rsp
            rbp = rsp;
            // 19902 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19904 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19906 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19907 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19908 mov r15,rdi
            r15 = rdi;
            // 1990b mov r14,QWORD PTR [r15]
            r14 = m.r64(r15) as u64;
            // 1990e test r14,r14
            fl = logic::<64>(r14 & r14);
            // 19911 je 0x19984
            if fl.e() { pc = 0x19984; continue; }
            pc = 0x19913;
        }
        0x19913 => {
            // block 19913
            #[cfg(feature = "trace")] eprintln!("f_000198fe pc 19913 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19913 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19917 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1991a je 0x19972
            if fl.e() { pc = 0x19972; continue; }
            pc = 0x1991c;
        }
        0x1991c => {
            // block 1991c
            #[cfg(feature = "trace")] eprintln!("f_000198fe pc 1991c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1991c lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 19923 mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            // 19927 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 1992b call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19930 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 19934 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19939 lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 1993d call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19942 lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 19946 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1994b lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 19952 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19957 add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 1995e mov rdi,rbx
            rdi = rbx;
            // 19961 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19966 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1996a cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1996d jne 0x1991c
            if fl.ne() { pc = 0x1991c; continue; }
            pc = 0x1996f;
        }
        0x1996f => {
            // block 1996f
            #[cfg(feature = "trace")] eprintln!("f_000198fe pc 1996f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1996f mov r14,QWORD PTR [r15]
            r14 = m.r64(r15) as u64;
            pc = 0x19972;
        }
        0x19972 => {
            // block 19972
            #[cfg(feature = "trace")] eprintln!("f_000198fe pc 19972 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19972 mov rdi,r14
            rdi = r14;
            // 19975 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19979 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1997a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1997c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1997e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1997f jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x19984 => {
            // block 19984
            #[cfg(feature = "trace")] eprintln!("f_000198fe pc 19984 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19984 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19988 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19989 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1998b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1998d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1998e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2daea size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002daea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2daea;
    loop {
        match pc {
        0x2daea => {
            // block 2daea
            #[cfg(feature = "trace")] eprintln!("f_0002daea pc 2daea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2daea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2daeb mov rbp,rsp
            rbp = rsp;
            // 2daee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2daf0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2daf2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2daf4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2daf5 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2daf9 mov r15,rsi
            r15 = rsi;
            // 2dafc mov r14,rdi
            r14 = rdi;
            // 2daff lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2db03 mov rdx,r15
            rdx = r15;
            // 2db06 call 0xebad4
            { let r = { let t = m.env.stub_va(0x2a1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2db0b mov r12,rax
            r12 = rax;
            // 2db0e mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 2db12 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2db15 je 0x2db1b
            if fl.e() { pc = 0x2db1b; continue; }
            pc = 0x2db17;
        }
        0x2db17 => {
            // block 2db17
            #[cfg(feature = "trace")] eprintln!("f_0002daea pc 2db17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db17 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2db19 jmp 0x2db42
            pc = 0x2db42; continue;
        }
        0x2db1b => {
            // block 2db1b
            #[cfg(feature = "trace")] eprintln!("f_0002daea pc 2db1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db1b mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 2db20 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2db25 mov rbx,rax
            rbx = rax;
            // 2db28 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 2db2b mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 2db2e mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 2db32 mov rdi,r14
            rdi = r14;
            // 2db35 mov rdx,r12
            rdx = r12;
            // 2db38 mov rcx,rbx
            rcx = rbx;
            // 2db3b call 0xebae0
            { let r = { let t = m.env.stub_va(0x2ad); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2db40 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x2db42;
        }
        0x2db42 => {
            // block 2db42
            #[cfg(feature = "trace")] eprintln!("f_0002daea pc 2db42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db42 mov rax,rbx
            rax = rbx;
            // 2db45 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2db49 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2db4a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2db4c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2db4e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2db50 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2db51 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18b50 size=253
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018b50<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x18b50;
    loop {
        match pc {
        0x18b50 => {
            // block 18b50
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b50 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18b51 mov rbp,rsp
            rbp = rsp;
            // 18b54 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18b56 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18b58 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 18b5a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 18b5c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18b5d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 18b5e mov rbx,rsi
            rbx = rsi;
            // 18b61 mov r14,rdi
            r14 = rdi;
            // 18b64 mov r13,QWORD PTR [rbx]
            r13 = m.r64(rbx) as u64;
            // 18b67 cmp r13,QWORD PTR [rbx+0x8]
            fl = sub::<64>(r13, m.r64(rbx.wrapping_add(0x8)) as u64).1;
            // 18b6b je 0x18c3b
            if fl.e() { pc = 0x18c3b; continue; }
            pc = 0x18b71;
        }
        0x18b71 => {
            // block 18b71
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b71 lea r12,[rip+0x111741] # 0x12a2b9
            r12 = m.env.data_va(256646);
            // 18b78 lea r15,[rip+0x11172f] # 0x12a2ae
            r15 = m.env.data_va(256635);
            pc = 0x18b7f;
        }
        0x18b7f => {
            // block 18b7f
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b7f mov eax,DWORD PTR [r13+0x0]
            rax = (m.r32(r13) as u64) as u32 as u64;
            // 18b83 cmp eax,0x6e6f696d
            fl = sub::<32>((rax as u32 as u64), 0x6e6f696du64).1;
            // 18b88 jg 0x18ba9
            if fl.g() { pc = 0x18ba9; continue; }
            pc = 0x18b8a;
        }
        0x18b8a => {
            // block 18b8a
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b8a cmp eax,0x69616c66
            fl = sub::<32>((rax as u32 as u64), 0x69616c66u64).1;
            // 18b8f jg 0x18bbd
            if fl.g() { pc = 0x18bbd; continue; }
            pc = 0x18b91;
        }
        0x18b91 => {
            // block 18b91
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b91 cmp eax,0x69616474
            fl = sub::<32>((rax as u32 as u64), 0x69616474u64).1;
            // 18b96 jne 0x18bed
            if fl.ne() { pc = 0x18bed; continue; }
            pc = 0x18b98;
        }
        0x18b98 => {
            // block 18b98
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18b98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18b98 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 18b9d mov rdi,r14
            rdi = r14;
            // 18ba0 lea rsi,[rip+0x1116f9] # 0x12a2a0
            rsi = m.env.data_va(256621);
            // 18ba7 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18ba9 => {
            // block 18ba9
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18ba9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18ba9 cmp eax,0x6e6f696e
            fl = sub::<32>((rax as u32 as u64), 0x6e6f696eu64).1;
            // 18bae jne 0x18bd5
            if fl.ne() { pc = 0x18bd5; continue; }
            pc = 0x18bb0;
        }
        0x18bb0 => {
            // block 18bb0
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bb0 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 18bb5 mov rdi,r14
            rdi = r14;
            // 18bb8 mov rsi,r15
            rsi = r15;
            // 18bbb jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bbd => {
            // block 18bbd
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bbd cmp eax,0x69616c67
            fl = sub::<32>((rax as u32 as u64), 0x69616c67u64).1;
            // 18bc2 jne 0x18c05
            if fl.ne() { pc = 0x18c05; continue; }
            pc = 0x18bc4;
        }
        0x18bc4 => {
            // block 18bc4
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bc4 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 18bc9 mov rdi,r14
            rdi = r14;
            // 18bcc lea rsi,[rip+0x1116d3] # 0x12a2a6
            rsi = m.env.data_va(256627);
            // 18bd3 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bd5 => {
            // block 18bd5
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bd5 cmp eax,0x6f757420
            fl = sub::<32>((rax as u32 as u64), 0x6f757420u64).1;
            // 18bda jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18bdc;
        }
        0x18bdc => {
            // block 18bdc
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bdc mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18be1 mov rdi,r14
            rdi = r14;
            // 18be4 lea rsi,[rip+0x1116c9] # 0x12a2b4
            rsi = m.env.data_va(256641);
            // 18beb jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bed => {
            // block 18bed
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bed cmp eax,0x69616573
            fl = sub::<32>((rax as u32 as u64), 0x69616573u64).1;
            // 18bf2 jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18bf4;
        }
        0x18bf4 => {
            // block 18bf4
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18bf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bf4 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18bf9 mov rdi,r14
            rdi = r14;
            // 18bfc lea rsi,[rip+0x111698] # 0x12a29b
            rsi = m.env.data_va(256616);
            // 18c03 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18c05 => {
            // block 18c05
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c05 cmp eax,0x69736469
            fl = sub::<32>((rax as u32 as u64), 0x69736469u64).1;
            // 18c0a jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18c0c;
        }
        0x18c0c => {
            // block 18c0c
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c0c mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18c11 mov rdi,r14
            rdi = r14;
            // 18c14 lea rsi,[rip+0x11167b] # 0x12a296
            rsi = m.env.data_va(256611);
            // 18c1b jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18c1d => {
            // block 18c1d
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c1d mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18c22 mov rdi,r14
            rdi = r14;
            // 18c25 mov rsi,r12
            rsi = r12;
            pc = 0x18c28;
        }
        0x18c28 => {
            // block 18c28
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c28 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18c2d add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // 18c31 cmp r13,QWORD PTR [rbx+0x8]
            fl = sub::<64>(r13, m.r64(rbx.wrapping_add(0x8)) as u64).1;
            // 18c35 jne 0x18b7f
            if fl.ne() { pc = 0x18b7f; continue; }
            pc = 0x18c3b;
        }
        0x18c3b => {
            // block 18c3b
            #[cfg(feature = "trace")] eprintln!("f_00018b50 pc 18c3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18c3b mov rax,r14
            rax = r14;
            // 18c3e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 18c42 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18c43 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18c45 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 18c47 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18c49 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18c4b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18c4c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28f6c size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028f6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28f6c;
    loop {
        match pc {
        0x28f6c => {
            // block 28f6c
            #[cfg(feature = "trace")] eprintln!("f_00028f6c pc 28f6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28f6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28f6d mov rbp,rsp
            rbp = rsp;
            // 28f70 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 28f73 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28f76 je 0x28f8b
            if fl.e() { pc = 0x28f8b; continue; }
            pc = 0x28f78;
        }
        0x28f78 => {
            // block 28f78
            #[cfg(feature = "trace")] eprintln!("f_00028f6c pc 28f78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28f78 mov ecx,DWORD PTR [rdi+0x8]
            rcx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28f7b shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 28f7e je 0x28f8b
            if fl.e() { pc = 0x28f8b; continue; }
            pc = 0x28f80;
        }
        0x28f80 => {
            // block 28f80
            #[cfg(feature = "trace")] eprintln!("f_00028f6c pc 28f80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28f80 mov DWORD PTR [rax],esi
            m.w32(rax, ((rsi as u32 as u64)) as u32);
            // 28f82 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            // 28f86 dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // 28f89 jne 0x28f80
            if fl.ne() { pc = 0x28f80; continue; }
            pc = 0x28f8b;
        }
        0x28f8b => {
            // block 28f8b
            #[cfg(feature = "trace")] eprintln!("f_00028f6c pc 28f8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28f8b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28f8c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2eee0 size=308
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002eee0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2eee0;
    loop {
        match pc {
        0x2eee0 => {
            // block 2eee0
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2eee0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2eee0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2eee1 mov rbp,rsp
            rbp = rsp;
            // 2eee4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2eee6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2eee8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2eee9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2eeea mov r8,QWORD PTR [rdi]
            r8 = m.r64(rdi) as u64;
            // 2eeed mov r9,QWORD PTR [rdi+0x8]
            r9 = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2eef1 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2eef3 cmp r8,r9
            fl = sub::<64>(r8, r9).1;
            // 2eef6 je 0x2f009
            if fl.e() { pc = 0x2f009; continue; }
            pc = 0x2eefc;
        }
        0x2eefc => {
            // block 2eefc
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2eefc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2eefc mov r15,QWORD PTR [rsi]
            r15 = m.r64(rsi) as u64;
            // 2eeff mov rsi,r15
            rsi = r15;
            // 2ef02 mov rcx,r8
            rcx = r8;
            pc = 0x2ef05;
        }
        0x2ef05 => {
            // block 2ef05
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2ef05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ef05 movzx ebx,BYTE PTR [rsi]
            rbx = (m.r8(rsi) as u64) as u32 as u64;
            // 2ef08 movzx edi,BYTE PTR [rcx]
            rdi = (m.r8(rcx) as u64) as u32 as u64;
            // 2ef0b cmp edi,ebx
            fl = sub::<32>((rdi as u32 as u64), (rbx as u32 as u64)).1;
            // 2ef0d jne 0x2ef1f
            if fl.ne() { pc = 0x2ef1f; continue; }
            pc = 0x2ef0f;
        }
        0x2ef0f => {
            // block 2ef0f
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2ef0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ef0f inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 2ef12 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 2ef15 cmp r9,rcx
            fl = sub::<64>(r9, rcx).1;
            // 2ef18 jne 0x2ef05
            if fl.ne() { pc = 0x2ef05; continue; }
            pc = 0x2ef1a;
        }
        0x2ef1a => {
            // block 2ef1a
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2ef1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ef1a jmp 0x2f009
            pc = 0x2f009; continue;
        }
        0x2ef1f => {
            // block 2ef1f
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2ef1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ef1f test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 2ef21 je 0x2f007
            if fl.e() { pc = 0x2f007; continue; }
            pc = 0x2ef27;
        }
        0x2ef27 => {
            // block 2ef27
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2ef27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ef27 mov rbx,r8
            rbx = r8;
            pc = 0x2ef2a;
        }
        0x2ef2a => {
            // block 2ef2a
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2ef2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ef2a movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 2ef2e movzx ecx,BYTE PTR [rbx]
            rcx = (m.r8(rbx) as u64) as u32 as u64;
            // 2ef31 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 2ef33 jne 0x2ef43
            if fl.ne() { pc = 0x2ef43; continue; }
            pc = 0x2ef35;
        }
        0x2ef35 => {
            // block 2ef35
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2ef35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ef35 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2ef38 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 2ef3b cmp r9,rbx
            fl = sub::<64>(r9, rbx).1;
            // 2ef3e jne 0x2ef2a
            if fl.ne() { pc = 0x2ef2a; continue; }
            pc = 0x2ef40;
        }
        0x2ef40 => {
            // block 2ef40
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2ef40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ef40 mov rbx,r9
            rbx = r9;
            pc = 0x2ef43;
        }
        0x2ef43 => {
            // block 2ef43
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2ef43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ef43 mov r14,rbx
            r14 = rbx;
            // 2ef46 sub r14,r8
            { let (r, f) = sub::<64>(r14, r8);
              r14 = r;
              fl = f; }
            // 2ef49 mov rdi,QWORD PTR [rip+0x111238] # 0x140188
            rdi = m.r64(m.env.data_va(346453)) as u64;
            // 2ef50 lea rsi,[rip+0xfbc3a] # 0x12ab91
            rsi = m.env.data_va(258910);
            // 2ef57 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 2ef5c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ef61 mov rdi,rax
            rdi = rax;
            // 2ef64 mov rsi,r14
            rsi = r14;
            // 2ef67 call 0xec0b0
            { let r = { let t = m.env.stub_va(0x87d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ef6c lea rsi,[rip+0xfbc42] # 0x12abb5
            rsi = m.env.data_va(258946);
            // 2ef73 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 2ef78 mov rdi,rax
            rdi = rax;
            // 2ef7b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ef80 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2ef83 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2ef87 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2ef8b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2ef8e or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2ef91 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2ef95 mov cl,BYTE PTR [rbx]
            rcx = (rcx & !0xff) | ((m.r8(rbx) as u64) as u8 as u64);
            // 2ef97 mov BYTE PTR [rbp-0x19],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe7), ((rcx as u8 as u64)) as u8);
            // 2ef9a lea rsi,[rbp-0x19]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe7);
            // 2ef9e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2efa3 mov rdi,rax
            rdi = rax;
            // 2efa6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2efab lea rsi,[rip+0xfbc10] # 0x12abc2
            rsi = m.env.data_va(258959);
            // 2efb2 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 2efb7 mov rdi,rax
            rdi = rax;
            // 2efba call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2efbf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2efc2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2efc6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2efca and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2efcd or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2efd0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2efd4 mov cl,BYTE PTR [r15]
            rcx = (rcx & !0xff) | ((m.r8(r15) as u64) as u8 as u64);
            // 2efd7 mov BYTE PTR [rbp-0x1a],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rcx as u8 as u64)) as u8);
            // 2efda lea rsi,[rbp-0x1a]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // 2efde mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2efe3 mov rdi,rax
            rdi = rax;
            // 2efe6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2efeb lea rsi,[rip+0xfbbdd] # 0x12abcf
            rsi = m.env.data_va(258972);
            // 2eff2 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 2eff7 mov rdi,rax
            rdi = rax;
            // 2effa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2efff mov rdi,rax
            rdi = rax;
            // 2f002 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f007;
        }
        0x2f007 => {
            // block 2f007
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2f007 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f007 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2f009;
        }
        0x2f009 => {
            // block 2f009
            #[cfg(feature = "trace")] eprintln!("f_0002eee0 pc 2f009 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f009 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2f00d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f00e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f010 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2f012 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f013 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x63714 size=1439
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
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
            { let r = { let t = m.env.code_va(0x633ff); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63732 movzx edx,BYTE PTR [r15]
            rdx = (m.r8(r15) as u64) as u32 as u64;
            // 63736 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63738 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6373a jne 0x63741
            if fl.ne() { pc = 0x63741; continue; }
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
            if fl.ne() { pc = 0x63756; continue; }
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
            if fl.ne() { pc = 0x637bb; continue; }
            pc = 0x6375f;
        }
        0x6375f => {
            // block 6375f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 6375f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6375f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63761 jne 0x63769
            if fl.ne() { pc = 0x63769; continue; }
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
            if fl.ne() { pc = 0x63777; continue; }
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
            if fl.ne() { pc = 0x637a5; continue; }
            pc = 0x63782;
        }
        0x63782 => {
            // block 63782
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63782 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63782 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63785 je 0x63c85
            if fl.e() { pc = 0x63c85; continue; }
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
            if fl.ne() { pc = 0x637bb; continue; }
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
            if fl.ne() { pc = 0x6378b; continue; }
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
            if fl.e() { pc = 0x63c85; continue; }
            pc = 0x637ae;
        }
        0x637ae => {
            // block 637ae
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637ae call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 637b3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 637b5 je 0x63c85
            if fl.e() { pc = 0x63c85; continue; }
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
            if fl.g() { pc = 0x637f9; continue; }
            pc = 0x637c6;
        }
        0x637c6 => {
            // block 637c6
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 637c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 637c6 cmp ebx,0x10294700
            fl = sub::<32>((rbx as u32 as u64), 0x10294700u64).1;
            // 637cc jne 0x6383e
            if fl.ne() { pc = 0x6383e; continue; }
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
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 637e3 movzx edx,BYTE PTR [rbp-0x60]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 637e7 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 637e9 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 637eb jne 0x63922
            if fl.ne() { pc = 0x63922; continue; }
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
            if fl.g() { pc = 0x63875; continue; }
            pc = 0x63801;
        }
        0x63801 => {
            // block 63801
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63801 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63801 cmp ebx,0x10478300
            fl = sub::<32>((rbx as u32 as u64), 0x10478300u64).1;
            // 63807 jne 0x638a8
            if fl.ne() { pc = 0x638a8; continue; }
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
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63825 movzx edx,BYTE PTR [rbp-0xa8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64) as u32 as u64;
            // 6382c mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 6382e and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63830 jne 0x6394e
            if fl.ne() { pc = 0x6394e; continue; }
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
            if fl.ne() { pc = 0x63c85; continue; }
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
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6385f movzx edx,BYTE PTR [rbp-0x48]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 63863 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63865 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63867 jne 0x63966
            if fl.ne() { pc = 0x63966; continue; }
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
            if fl.ne() { pc = 0x638e5; continue; }
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
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63892 movzx edx,BYTE PTR [rbp-0x78]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // 63896 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63898 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6389a jne 0x6397b
            if fl.ne() { pc = 0x6397b; continue; }
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
            if fl.ne() { pc = 0x63c85; continue; }
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
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 638cc movzx edx,BYTE PTR [rbp-0xc0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64) as u32 as u64;
            // 638d3 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 638d5 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 638d7 jne 0x63a43
            if fl.ne() { pc = 0x63a43; continue; }
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
            if fl.ne() { pc = 0x63c85; continue; }
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
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63909 movzx edx,BYTE PTR [rbp-0x90]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64) as u32 as u64;
            // 63910 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63912 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63914 jne 0x63a5b
            if fl.ne() { pc = 0x63a5b; continue; }
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
            if fl.ne() { pc = 0x63937; continue; }
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
            if fl.ne() { pc = 0x63a22; continue; }
            pc = 0x63944;
        }
        0x63944 => {
            // block 63944
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63944 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63944 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63946 jne 0x63990
            if fl.ne() { pc = 0x63990; continue; }
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
            if fl.ne() { pc = 0x6399d; continue; }
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
            if fl.ne() { pc = 0x639be; continue; }
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
            if fl.ne() { pc = 0x639dc; continue; }
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
            if fl.ne() { pc = 0x639fa; continue; }
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
            if fl.ne() { pc = 0x63b1d; continue; }
            pc = 0x639aa;
        }
        0x639aa => {
            // block 639aa
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639aa test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639ac jne 0x63a73
            if fl.ne() { pc = 0x63a73; continue; }
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
            if fl.ne() { pc = 0x63b4a; continue; }
            pc = 0x639cb;
        }
        0x639cb => {
            // block 639cb
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639cb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639cd jne 0x63a83
            if fl.ne() { pc = 0x63a83; continue; }
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
            if fl.ne() { pc = 0x63b77; continue; }
            pc = 0x639e9;
        }
        0x639e9 => {
            // block 639e9
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 639e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 639e9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639eb jne 0x63a97
            if fl.ne() { pc = 0x63a97; continue; }
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
            if fl.ne() { pc = 0x63a27; continue; }
            pc = 0x63a05;
        }
        0x63a05 => {
            // block 63a05
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a05 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63a08 je 0x63a3a
            if fl.e() { pc = 0x63a3a; continue; }
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
            if fl.ne() { pc = 0x63a22; continue; }
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
            if fl.ne() { pc = 0x63a0a; continue; }
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
            if fl.e() { pc = 0x63a3a; continue; }
            pc = 0x63a2c;
        }
        0x63a2c => {
            // block 63a2c
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63a2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63a2c mov rsi,r15
            rsi = r15;
            // 63a2f call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0x63aab; continue; }
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
            if fl.ne() { pc = 0x63acc; continue; }
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
            if fl.ne() { pc = 0x63aed; continue; }
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
            if fl.ne() { pc = 0x63b22; continue; }
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
            if fl.ne() { pc = 0x63b4f; continue; }
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
            if fl.ne() { pc = 0x63c18; continue; }
            pc = 0x63ab8;
        }
        0x63ab8 => {
            // block 63ab8
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ab8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ab8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63aba jne 0x63bd3
            if fl.ne() { pc = 0x63bd3; continue; }
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
            if fl.ne() { pc = 0x63c45; continue; }
            pc = 0x63ad9;
        }
        0x63ad9 => {
            // block 63ad9
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ad9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ad9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63adb jne 0x63be3
            if fl.ne() { pc = 0x63be3; continue; }
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
            if fl.ne() { pc = 0x63b7c; continue; }
            pc = 0x63afc;
        }
        0x63afc => {
            // block 63afc
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63afc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63afc test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63aff je 0x63b8f
            if fl.e() { pc = 0x63b8f; continue; }
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
            if fl.ne() { pc = 0x63b1d; continue; }
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
            if fl.ne() { pc = 0x63b05; continue; }
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
            if fl.ne() { pc = 0x63b9b; continue; }
            pc = 0x63b2d;
        }
        0x63b2d => {
            // block 63b2d
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b2d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b30 je 0x63bae
            if fl.e() { pc = 0x63bae; continue; }
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
            if fl.ne() { pc = 0x63b4a; continue; }
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
            if fl.ne() { pc = 0x63b32; continue; }
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
            if fl.ne() { pc = 0x63bb7; continue; }
            pc = 0x63b5a;
        }
        0x63b5a => {
            // block 63b5a
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b5a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b5d je 0x63bca
            if fl.e() { pc = 0x63bca; continue; }
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
            if fl.ne() { pc = 0x63b77; continue; }
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
            if fl.ne() { pc = 0x63b5f; continue; }
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
            if fl.e() { pc = 0x63b8f; continue; }
            pc = 0x63b81;
        }
        0x63b81 => {
            // block 63b81
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63b81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63b81 mov rsi,r15
            rsi = r15;
            // 63b84 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0x63bae; continue; }
            pc = 0x63ba0;
        }
        0x63ba0 => {
            // block 63ba0
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63ba0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63ba0 mov rsi,r15
            rsi = r15;
            // 63ba3 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0x63bca; continue; }
            pc = 0x63bbc;
        }
        0x63bbc => {
            // block 63bbc
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bbc mov rsi,r15
            rsi = r15;
            // 63bbf call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0x63bf0; continue; }
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
            if fl.ne() { pc = 0x63c1d; continue; }
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
            if fl.ne() { pc = 0x63c4a; continue; }
            pc = 0x63bfb;
        }
        0x63bfb => {
            // block 63bfb
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63bfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63bfb test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63bfe je 0x63c5d
            if fl.e() { pc = 0x63c5d; continue; }
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
            if fl.ne() { pc = 0x63c18; continue; }
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
            if fl.ne() { pc = 0x63c00; continue; }
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
            if fl.ne() { pc = 0x63c66; continue; }
            pc = 0x63c28;
        }
        0x63c28 => {
            // block 63c28
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c28 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63c2b je 0x63c79
            if fl.e() { pc = 0x63c79; continue; }
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
            if fl.ne() { pc = 0x63c45; continue; }
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
            if fl.ne() { pc = 0x63c2d; continue; }
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
            if fl.e() { pc = 0x63c5d; continue; }
            pc = 0x63c4f;
        }
        0x63c4f => {
            // block 63c4f
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c4f mov rsi,r15
            rsi = r15;
            // 63c52 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0x63c79; continue; }
            pc = 0x63c6b;
        }
        0x63c6b => {
            // block 63c6b
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c6b mov rsi,r15
            rsi = r15;
            // 63c6e call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x63c85;
        }
        0x63c85 => {
            // block 63c85
            #[cfg(feature = "trace")] eprintln!("f_00063714 pc 63c85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63c85 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 63c89 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa32 size=370
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000a32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xa32;
    loop {
        match pc {
        0xa32 => {
            // block a32
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a33 mov rbp,rsp
            rbp = rsp;
            // a36 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a38 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a3a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a3b sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // a3f mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // a42 mov rbx,rdi
            rbx = rdi;
            // a45 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a48 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4e mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // a51 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a54 lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a58 mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // a5c lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // a60 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // a64 lea rcx,[rbp-0x24]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // a68 lea r8,[rbp-0x28]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // a6c lea r9,[rbp-0x2c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // a70 mov rdi,rbx
            rdi = rbx;
            // a73 call QWORD PTR [rax+0xd10]
            { let r = { let t = m.r64(rax.wrapping_add(0xd10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a79 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // a7b je 0xb26
            if fl.e() { pc = 0xb26; continue; }
            pc = 0xa81;
        }
        0xa81 => {
            // block a81
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc a81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a81 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // a84 call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a89 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // a8c neg ecx
            { let (r, f) = neg::<32>((rcx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // a8e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // a90 cmovne ecx,r14d
            if fl.ne() { rcx = ((r14 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // a94 add ecx,DWORD PTR [rbp-0x1c]
            { let (r, f) = add::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a97 mov DWORD PTR [rbp-0x1c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rcx as u32 as u64)) as u32);
            // a9a mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // a9d cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // a9f jg 0xaa8
            if fl.g() { pc = 0xaa8; continue; }
            pc = 0xaa1;
        }
        0xaa1 => {
            // block aa1
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc aa1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa1 mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // aa4 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // aa6 jge 0xaab
            if fl.ge() { pc = 0xaab; continue; }
            pc = 0xaa8;
        }
        0xaa8 => {
            // block aa8
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc aa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa8 mov DWORD PTR [rbp-0x1c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rax as u32 as u64)) as u32);
            pc = 0xaab;
        }
        0xaab => {
            // block aab
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc aab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // aae lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // ab2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ab4 mov rdi,rbx
            rdi = rbx;
            // ab7 call QWORD PTR [rax+0xd60]
            { let r = { let t = m.r64(rax.wrapping_add(0xd60)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abd mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // ac0 movzx ecx,ax
            rcx = ((rax as u16 as u64)) as u32 as u64;
            // ac3 mov esi,DWORD PTR [rbp-0x1c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // ac6 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // ac9 cmp ecx,esi
            fl = sub::<32>((rcx as u32 as u64), (rsi as u32 as u64)).1;
            // acb je 0xb95
            if fl.e() { pc = 0xb95; continue; }
            pc = 0xad1;
        }
        0xad1 => {
            // block ad1
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc ad1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad1 lea edx,[rcx+0x1]
            rdx = (rcx.wrapping_add(0x1)) as u32 as u64;
            // ad4 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // ad6 jne 0xb2b
            if fl.ne() { pc = 0xb2b; continue; }
            pc = 0xad8;
        }
        0xad8 => {
            // block ad8
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc ad8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad8 and eax,0xffff0000
            { let r = (rax as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // add add esi,0x2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ae0 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ae2 mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // ae5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ae8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aea mov rdi,rbx
            rdi = rbx;
            // aed call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af6 lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // afa mov rdi,rbx
            rdi = rbx;
            // afd call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03 lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0xb07;
        }
        0xb07 => {
            // block b07
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b07 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0a mov rdi,rbx
            rdi = rbx;
            // b0d mov rsi,r15
            rsi = r15;
            // b10 call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b16 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // b19 cmp eax,DWORD PTR [rbp-0x3c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64).1;
            // b1c jne 0xb07
            if fl.ne() { pc = 0xb07; continue; }
            pc = 0xb1e;
        }
        0xb1e => {
            // block b1e
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b1e mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // b21 add esi,0xfffffffe
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffeu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b24 jmp 0xb84
            pc = 0xb84; continue;
        }
        0xb26 => {
            // block b26
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b26 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b29 jmp 0xb95
            pc = 0xb95; continue;
        }
        0xb2b => {
            // block b2b
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b2b dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // b2d and eax,0xffff0000
            { let r = (rax as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b32 mov DWORD PTR [rbp-0x34],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rax as u32 as u64)) as u32);
            // b35 cmp ecx,esi
            fl = sub::<32>((rcx as u32 as u64), (rsi as u32 as u64)).1;
            // b37 jne 0xb82
            if fl.ne() { pc = 0xb82; continue; }
            pc = 0xb39;
        }
        0xb39 => {
            // block b39
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b39 add esi,0xfffffffe
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffeu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b3c or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b3e mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // b41 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b44 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b46 mov rdi,rbx
            rdi = rbx;
            // b49 call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b52 lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b56 mov rdi,rbx
            rdi = rbx;
            // b59 call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5f lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0xb63;
        }
        0xb63 => {
            // block b63
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b63 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b66 mov rdi,rbx
            rdi = rbx;
            // b69 mov rsi,r15
            rsi = r15;
            // b6c call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b72 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // b75 cmp eax,DWORD PTR [rbp-0x3c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64).1;
            // b78 jne 0xb63
            if fl.ne() { pc = 0xb63; continue; }
            pc = 0xb7a;
        }
        0xb7a => {
            // block b7a
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // b7d add esi,0x2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b80 jmp 0xb84
            pc = 0xb84; continue;
        }
        0xb82 => {
            // block b82
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b82 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xb84;
        }
        0xb84 => {
            // block b84
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b84 mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // b87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b8a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b8c mov rdi,rbx
            rdi = rbx;
            // b8f call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb95;
        }
        0xb95 => {
            // block b95
            #[cfg(feature = "trace")] eprintln!("f_00000a32 pc b95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b95 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b98 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b9c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51df0 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051df0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x51df0;
    loop {
        match pc {
        0x51df0 => {
            // block 51df0
            #[cfg(feature = "trace")] eprintln!("f_00051df0 pc 51df0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51df0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51df1 mov rbp,rsp
            rbp = rsp;
            // 51df4 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51df6 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51df9 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51dfd mov esi,0x283d
            rsi = (0x283du64) as u32 as u64;
            // 51e02 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51e07 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51e0a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51e0b jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15460 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00015460<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x15460;
    loop {
        match pc {
        0x15460 => {
            // block 15460
            #[cfg(feature = "trace")] eprintln!("f_00015460 pc 15460 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15460 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15461 mov rbp,rsp
            rbp = rsp;
            // 15464 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15466 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15467 mov rbx,rsi
            rbx = rsi;
            // 1546a mov r14,rdi
            r14 = rdi;
            // 1546d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 15470 je 0x1549e
            if fl.e() { pc = 0x1549e; continue; }
            pc = 0x15472;
        }
        0x15472 => {
            // block 15472
            #[cfg(feature = "trace")] eprintln!("f_00015460 pc 15472 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15472 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 15475 mov rdi,r14
            rdi = r14;
            // 15478 call 0x15460
            { let r = { let t = m.env.code_va(0x15460); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1547d mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 15481 mov rdi,r14
            rdi = r14;
            // 15484 call 0x15460
            { let r = { let t = m.env.code_va(0x15460); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15489 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 1548d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15492 mov rdi,rbx
            rdi = rbx;
            // 15495 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15496 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15498 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15499 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x1549e => {
            // block 1549e
            #[cfg(feature = "trace")] eprintln!("f_00015460 pc 1549e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1549e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1549f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 154a1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 154a2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19a7a size=294
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00019a7a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x19a7a;
    loop {
        match pc {
        0x19a7a => {
            // block 19a7a
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19a7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a7a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19a7b mov rbp,rsp
            rbp = rsp;
            // 19a7e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19a80 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19a82 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 19a84 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 19a86 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19a87 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19a88 mov r12,rdx
            r12 = rdx;
            // 19a8b mov r15,rdi
            r15 = rdi;
            // 19a8e mov r14,r12
            r14 = r12;
            // 19a91 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 19a94 mov rbx,r14
            rbx = r14;
            // 19a97 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19a9b mov r13,QWORD PTR [r15]
            r13 = m.r64(r15) as u64;
            // 19a9e mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19aa2 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19aa5 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19aa9 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19aac jbe 0x19af3
            if fl.be() { pc = 0x19af3; continue; }
            pc = 0x19aae;
        }
        0x19aae => {
            // block 19aae
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19aae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19aae mov r12,rsi
            r12 = rsi;
            // 19ab1 mov rdi,r15
            rdi = r15;
            // 19ab4 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19ab9 mov rax,rbx
            rax = rbx;
            // 19abc shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19ac0 je 0x19aca
            if fl.e() { pc = 0x19aca; continue; }
            pc = 0x19ac2;
        }
        0x19ac2 => {
            // block 19ac2
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19ac2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ac2 mov rdi,r15
            rdi = r15;
            // 19ac5 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x19aca => {
            // block 19aca
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19aca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19aca mov rsi,QWORD PTR [r15+0x10]
            rsi = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19ace sub rsi,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rsi, m.r64(r15) as u64);
              rsi = r;
              fl = f; }
            // 19ad1 mov rax,rsi
            rax = rsi;
            // 19ad4 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19ad8 movabs rcx,0x1ffffffffffffffe
            rcx = 0x1fff_ffff_ffff_fffeu64;
            // 19ae2 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 19ae5 ja 0x19b0c
            if fl.a() { pc = 0x19b0c; continue; }
            pc = 0x19ae7;
        }
        0x19ae7 => {
            // block 19ae7
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19ae7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ae7 sar rsi,1
            { let (r, f) = sar::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 19aea cmp rsi,rbx
            fl = sub::<64>(rsi, rbx).1;
            // 19aed cmovb rsi,rbx
            if fl.b() { rsi = rbx; }
            // 19af1 jmp 0x19b16
            pc = 0x19b16; continue;
        }
        0x19af3 => {
            // block 19af3
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19af3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19af3 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19af7 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19afa sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19afe cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19b01 jbe 0x19b33
            if fl.be() { pc = 0x19b33; continue; }
            pc = 0x19b03;
        }
        0x19b03 => {
            // block 19b03
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b03 lea rbx,[rsi+rax*4]
            rbx = rsi.wrapping_add(rax.wrapping_mul(4));
            // 19b07 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 19b0a jmp 0x19b39
            pc = 0x19b39; continue;
        }
        0x19b0c => {
            // block 19b0c
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b0c movabs rsi,0x3fffffffffffffff
            rsi = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x19b16;
        }
        0x19b16 => {
            // block 19b16
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b16 mov rdi,r15
            rdi = r15;
            // 19b19 call 0xeba20
            { let r = { let t = m.env.stub_va(0x1ed); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19b1e mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19b22 mov rsi,r12
            rsi = r12;
            // 19b25 mov rdx,r14
            rdx = r14;
            // 19b28 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19b2d shl rbx,0x2
            { let (r, f) = shl::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19b31 jmp 0x19b6a
            pc = 0x19b6a; continue;
        }
        0x19b33 => {
            // block 19b33
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b33 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 19b36 mov rbx,r12
            rbx = r12;
            pc = 0x19b39;
        }
        0x19b39 => {
            // block 19b39
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b39 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 19b3d sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 19b40 mov rdi,r13
            rdi = r13;
            // 19b43 mov rdx,rbx
            rdx = rbx;
            // 19b46 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19b4b mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19b4f test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 19b52 je 0x19b70
            if fl.e() { pc = 0x19b70; continue; }
            pc = 0x19b54;
        }
        0x19b54 => {
            // block 19b54
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b54 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 19b58 sub r12,rsi
            { let (r, f) = sub::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 19b5b mov rbx,r12
            rbx = r12;
            // 19b5e and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19b62 mov rdx,r12
            rdx = r12;
            // 19b65 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x19b6a;
        }
        0x19b6a => {
            // block 19b6a
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b6a add QWORD PTR [r15+0x8],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rbx);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 19b6e jmp 0x19b91
            pc = 0x19b91; continue;
        }
        0x19b70 => {
            // block 19b70
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b70 and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19b74 add r13,rbx
            { let (r, f) = add::<64>(r13, rbx);
              r13 = r;
              fl = f; }
            // 19b77 cmp rdi,r13
            fl = sub::<64>(rdi, r13).1;
            // 19b7a je 0x19b91
            if fl.e() { pc = 0x19b91; continue; }
            pc = 0x19b7c;
        }
        0x19b7c => {
            // block 19b7c
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b7c lea rax,[rdi-0x4]
            rax = rdi.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19b80 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19b83 not rax
            rax = !rax;
            // 19b86 and rax,0xfffffffffffffffc
            { let r = rax & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 19b8a add rax,rdi
            { let (r, f) = add::<64>(rax, rdi);
              rax = r;
              fl = f; }
            // 19b8d mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            pc = 0x19b91;
        }
        0x19b91 => {
            // block 19b91
            #[cfg(feature = "trace")] eprintln!("f_00019a7a pc 19b91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19b91 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19b95 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19b96 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 19b98 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 19b9a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19b9c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 19b9e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19b9f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba4 size=146
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000ba4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xba4;
    loop {
        match pc {
        0xba4 => {
            // block ba4
            #[cfg(feature = "trace")] eprintln!("f_00000ba4 pc ba4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba5 mov rbp,rsp
            rbp = rsp;
            // ba8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // baa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bac push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bad sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bb1 mov r14,rsi
            r14 = rsi;
            // bb4 mov rbx,rdi
            rbx = rdi;
            // bb7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bba call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc0 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // bc3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bc6 lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // bca mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // bce lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // bd2 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // bd6 lea rcx,[rbp-0x24]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // bda lea r8,[rbp-0x28]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // bde lea r9,[rbp-0x2c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // be2 mov rdi,rbx
            rdi = rbx;
            // be5 call QWORD PTR [rax+0xd10]
            { let r = { let t = m.r64(rax.wrapping_add(0xd10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // beb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // bed je 0xc1c
            if fl.e() { pc = 0xc1c; continue; }
            pc = 0xbef;
        }
        0xbef => {
            // block bef
            #[cfg(feature = "trace")] eprintln!("f_00000ba4 pc bef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bf2 lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // bf6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bf8 mov rdi,rbx
            rdi = rbx;
            // bfb call QWORD PTR [rax+0xd60]
            { let r = { let t = m.r64(rax.wrapping_add(0xd60)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c01 movzx eax,WORD PTR [rbp-0x34]
            rax = (m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // c05 mov DWORD PTR [rbp-0x34],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rax as u32 as u64)) as u32);
            // c08 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // c0b call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c10 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c12 je 0xc20
            if fl.e() { pc = 0xc20; continue; }
            pc = 0xc14;
        }
        0xc14 => {
            // block c14
            #[cfg(feature = "trace")] eprintln!("f_00000ba4 pc c14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c14 mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // c17 sub eax,DWORD PTR [rbp-0x1c]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c1a jmp 0xc26
            pc = 0xc26; continue;
        }
        0xc1c => {
            // block c1c
            #[cfg(feature = "trace")] eprintln!("f_00000ba4 pc c1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c1e jmp 0xc2b
            pc = 0xc2b; continue;
        }
        0xc20 => {
            // block c20
            #[cfg(feature = "trace")] eprintln!("f_00000ba4 pc c20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c20 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // c23 sub eax,DWORD PTR [rbp-0x34]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc26;
        }
        0xc26 => {
            // block c26
            #[cfg(feature = "trace")] eprintln!("f_00000ba4 pc c26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c26 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // c29 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xc2b;
        }
        0xc2b => {
            // block c2b
            #[cfg(feature = "trace")] eprintln!("f_00000ba4 pc c2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c2b add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c2f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c30 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c32 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c34 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c35 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x198a4 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000198a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x198a4;
    loop {
        match pc {
        0x198a4 => {
            // block 198a4
            #[cfg(feature = "trace")] eprintln!("f_000198a4 pc 198a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 198a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 198a5 mov rbp,rsp
            rbp = rsp;
            // 198a8 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 198ab call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 198ae mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 198b2 sub rcx,QWORD PTR [rax]
            { let (r, f) = sub::<64>(rcx, m.r64(rax) as u64);
              rcx = r;
              fl = f; }
            // 198b5 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 198b9 movabs rax,0xef7bdef7bdef7bdf
            rax = 0xef7b_def7_bdef_7bdfu64;
            // 198c3 imul rax,rcx
            { let (r, f) = imul::<64>(rax, rcx); rax = r; fl = f; }
            // 198c7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 198c8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2df5c size=135
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002df5c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2df5c;
    loop {
        match pc {
        0x2df5c => {
            // block 2df5c
            #[cfg(feature = "trace")] eprintln!("f_0002df5c pc 2df5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2df5c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2df5d mov rbp,rsp
            rbp = rsp;
            // 2df60 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2df62 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2df63 mov rax,QWORD PTR [rip+0x11247e] # 0x1403e8
            rax = m.r64(m.env.data_va(347061)) as u64;
            // 2df6a mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            // 2df6d lea rsi,[rip+0x118c14] # 0x146b88
            rsi = m.env.data_va(373589);
            // 2df74 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2df76 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 2df7b mov rdi,rbx
            rdi = rbx;
            // 2df7e call 0xebf5a
            { let r = { let t = m.env.stub_va(0x727); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2df83 mov r14,rax
            r14 = rax;
            // 2df86 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2df89 je 0x2dfde
            if fl.e() { pc = 0x2dfde; continue; }
            pc = 0x2df8b;
        }
        0x2df8b => {
            // block 2df8b
            #[cfg(feature = "trace")] eprintln!("f_0002df5c pc 2df8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2df8b mov rdi,rbx
            rdi = rbx;
            // 2df8e mov rsi,r14
            rsi = r14;
            // 2df91 call 0xebf36
            { let r = { let t = m.env.stub_va(0x703); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2df96 mov QWORD PTR [rip+0x193abb],rax # 0x1c1a58
            m.w64(m.env.data_va(877093), (rax) as u64);
            // 2df9d test rax,rax
            fl = logic::<64>(rax & rax);
            // 2dfa0 je 0x2dfd2
            if fl.e() { pc = 0x2dfd2; continue; }
            pc = 0x2dfa2;
        }
        0x2dfa2 => {
            // block 2dfa2
            #[cfg(feature = "trace")] eprintln!("f_0002df5c pc 2dfa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dfa2 lea rsi,[rip+0x118bff] # 0x146ba8
            rsi = m.env.data_va(373621);
            // 2dfa9 mov rdi,rax
            rdi = rax;
            // 2dfac call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dfb1 mov QWORD PTR [rip+0x193aa8],rax # 0x1c1a60
            m.w64(m.env.data_va(877101), (rax) as u64);
            // 2dfb8 mov rdi,QWORD PTR [rip+0x193a99] # 0x1c1a58
            rdi = m.r64(m.env.data_va(877093)) as u64;
            // 2dfbf lea rsi,[rip+0x118c02] # 0x146bc8
            rsi = m.env.data_va(373653);
            // 2dfc6 call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dfcb mov QWORD PTR [rip+0x193a96],rax # 0x1c1a68
            m.w64(m.env.data_va(877109), (rax) as u64);
            pc = 0x2dfd2;
        }
        0x2dfd2 => {
            // block 2dfd2
            #[cfg(feature = "trace")] eprintln!("f_0002df5c pc 2dfd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dfd2 mov rdi,r14
            rdi = r14;
            // 2dfd5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2dfd6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2dfd8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dfd9 jmp 0xebf42
            { let r = { let t = m.env.stub_va(0x70f); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2dfde => {
            // block 2dfde
            #[cfg(feature = "trace")] eprintln!("f_0002df5c pc 2dfde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dfde pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2dfdf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2dfe1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dfe2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x58ca2 size=322
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00058ca2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x58ca2;
    loop {
        match pc {
        0x58ca2 => {
            // block 58ca2
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58ca2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58ca2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 58ca3 mov rbp,rsp
            rbp = rsp;
            // 58ca6 cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // 58ca9 ja 0x58dc4
            if fl.a() { pc = 0x58dc4; continue; }
            pc = 0x58caf;
        }
        0x58caf => {
            // block 58caf
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58caf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58caf lea rax,[rip+0x112] # 0x58dc8
            rax = m.env.code_va(0x58dc8);
            // 58cb6 movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // 58cba add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 58cbd jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x58cbf => {
            // block 58cbf
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58cbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58cbf add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58cc1 je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58cc7;
        }
        0x58cc7 => {
            // block 58cc7
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58cc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58cc7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58cc9;
        }
        0x58cc9 => {
            // block 58cc9
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58cc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58cc9 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 58ccb mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58cd1 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // 58cd4 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58cda lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // 58cdd mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58ce3 lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // 58ce6 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58cec add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58cef cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58cf1 jb 0x58cc9
            if fl.b() { pc = 0x58cc9; continue; }
            pc = 0x58cf3;
        }
        0x58cf3 => {
            // block 58cf3
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58cf3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58cf3 jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58cf8 => {
            // block 58cf8
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58cf8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58cf8 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58cfa je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58d00;
        }
        0x58d00 => {
            // block 58d00
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d00 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58d02;
        }
        0x58d02 => {
            // block 58d02
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d02 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 58d04 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d0a lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // 58d0d mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d13 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58d16 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58d18 jb 0x58d02
            if fl.b() { pc = 0x58d02; continue; }
            pc = 0x58d1a;
        }
        0x58d1a => {
            // block 58d1a
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d1a jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58d1f => {
            // block 58d1f
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d1f add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58d21 je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58d27;
        }
        0x58d27 => {
            // block 58d27
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d27 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58d29;
        }
        0x58d29 => {
            // block 58d29
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d29 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // 58d2c mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58d32 lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // 58d35 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d3b lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // 58d3e mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58d44 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58d47 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58d49 jb 0x58d29
            if fl.b() { pc = 0x58d29; continue; }
            pc = 0x58d4b;
        }
        0x58d4b => {
            // block 58d4b
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d4b jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58d4d => {
            // block 58d4d
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d4d add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58d4f je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58d51;
        }
        0x58d51 => {
            // block 58d51
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d51 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58d53;
        }
        0x58d53 => {
            // block 58d53
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d53 lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // 58d56 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d5c add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58d5f cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58d61 jb 0x58d53
            if fl.b() { pc = 0x58d53; continue; }
            pc = 0x58d63;
        }
        0x58d63 => {
            // block 58d63
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d63 jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58d65 => {
            // block 58d65
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d65 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58d67 je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58d69;
        }
        0x58d69 => {
            // block 58d69
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d69 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58d6b;
        }
        0x58d6b => {
            // block 58d6b
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d6b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 58d6d mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d73 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // 58d76 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58d7c lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // 58d7f mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58d85 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58d88 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58d8a jb 0x58d6b
            if fl.b() { pc = 0x58d6b; continue; }
            pc = 0x58d8c;
        }
        0x58d8c => {
            // block 58d8c
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d8c jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58d8e => {
            // block 58d8e
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d8e add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58d90 je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58d92;
        }
        0x58d92 => {
            // block 58d92
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d92 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58d94;
        }
        0x58d94 => {
            // block 58d94
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58d94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58d94 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 58d96 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d9c add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58d9f cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58da1 jb 0x58d94
            if fl.b() { pc = 0x58d94; continue; }
            pc = 0x58da3;
        }
        0x58da3 => {
            // block 58da3
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58da3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58da3 jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58da5 => {
            // block 58da5
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58da5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58da5 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58da7 je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58da9;
        }
        0x58da9 => {
            // block 58da9
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58da9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58da9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58dab;
        }
        0x58dab => {
            // block 58dab
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58dab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58dab lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // 58dae mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58db4 lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // 58db7 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58dbd add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58dc0 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58dc2 jb 0x58dab
            if fl.b() { pc = 0x58dab; continue; }
            pc = 0x58dc4;
        }
        0x58dc4 => {
            // block 58dc4
            #[cfg(feature = "trace")] eprintln!("f_00058ca2 pc 58dc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58dc4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58dc5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
