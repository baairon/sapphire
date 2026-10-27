//! Mechanically lifted functions (scripts/lift_run.py), module u1001. Each one is checked against the original
//! in the world harness (tests/world_u1001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x55960 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00055960<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x55960;
    loop {
        match pc {
        0x55960 => {
            // block 55960
            #[cfg(feature = "trace")] eprintln!("f_00055960 pc 55960 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55960 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 55961 mov rbp,rsp
            rbp = rsp;
            // 55964 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 55966 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 55967 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5596b mov r14,rdi
            r14 = rdi;
            // 5596e lea rdi,[r14+0x180]
            rdi = r14.wrapping_add(0x180);
            // 55975 mov QWORD PTR [rbp-0x18],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (rdi) as u64);
            // 55979 call 0x566f8
            { let r = { let t = m.env.code_va(0x566f8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5597e xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x55980;
        }
        0x55980 => {
            // block 55980
            #[cfg(feature = "trace")] eprintln!("f_00055960 pc 55980 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55980 cmp DWORD PTR [r14+rbx*4],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(rbx.wrapping_mul(4))) as u64, 0x0u64).1;
            // 55985 je 0x55992
            if fl.e() { pc = 0x55992; continue; }
            pc = 0x55987;
        }
        0x55987 => {
            // block 55987
            #[cfg(feature = "trace")] eprintln!("f_00055960 pc 55987 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55987 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 5598a movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // 5598d cmp eax,0x20
            fl = sub::<32>((rax as u32 as u64), 0x20u64).1;
            // 55990 jb 0x55980
            if fl.b() { pc = 0x55980; continue; }
            pc = 0x55992;
        }
        0x55992 => {
            // block 55992
            #[cfg(feature = "trace")] eprintln!("f_00055960 pc 55992 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55992 lea rdi,[rbp-0x18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 55996 call 0x56744
            { let r = { let t = m.env.code_va(0x56744); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5599b movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // 5599e add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 559a2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 559a3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 559a5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 559a6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe463c size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e463c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe463c;
    loop {
        match pc {
        0xe463c => {
            // block e463c
            #[cfg(feature = "trace")] eprintln!("f_000e463c pc e463c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e463c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e463d mov rbp,rsp
            rbp = rsp;
            // e4640 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4642 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4644 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4645 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e4646 mov r14,rsi
            r14 = rsi;
            // e4649 mov rbx,rdi
            rbx = rdi;
            // e464c mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // e464e call 0xe46ec
            { let r = { let t = m.env.code_va(0xe46ec); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4653 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e4656 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // e4659 je 0xe4675
            if fl.e() { pc = 0xe4675; continue; }
            pc = 0xe465b;
        }
        0xe465b => {
            // block e465b
            #[cfg(feature = "trace")] eprintln!("f_000e463c pc e465b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e465b mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e465e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e4661 mov esi,DWORD PTR [rbx+0x30]
            rsi = (m.r32(rbx.wrapping_add(0x30)) as u64) as u32 as u64;
            // e4664 add esi,0x2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e4667 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e466c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e466f mov rdx,r14
            rdx = r14;
            // e4672 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe4675;
        }
        0xe4675 => {
            // block e4675
            #[cfg(feature = "trace")] eprintln!("f_000e463c pc e4675 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4675 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // e4678 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e467c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e467d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e467f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4681 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4682 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28e5e size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028e5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28e5e;
    loop {
        match pc {
        0x28e5e => {
            // block 28e5e
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28e5f mov rbp,rsp
            rbp = rsp;
            // 28e62 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28e64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28e65 mov rbx,rsi
            rbx = rsi;
            // 28e68 mov r14,rdi
            r14 = rdi;
            // 28e6b cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 28e6e je 0x28e9f
            if fl.e() { pc = 0x28e9f; continue; }
            pc = 0x28e70;
        }
        0x28e70 => {
            // block 28e70
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e70 cmp QWORD PTR [rbx],0x0
            fl = sub::<64>(m.r64(rbx) as u64, 0x0u64).1;
            // 28e74 je 0x28e93
            if fl.e() { pc = 0x28e93; continue; }
            pc = 0x28e76;
        }
        0x28e76 => {
            // block 28e76
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e76 mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28e79 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 28e7c je 0x28e93
            if fl.e() { pc = 0x28e93; continue; }
            pc = 0x28e7e;
        }
        0x28e7e => {
            // block 28e7e
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e7e mov rdi,r14
            rdi = r14;
            // 28e81 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e86 mov rdi,r14
            rdi = r14;
            // 28e89 mov rsi,rbx
            rsi = rbx;
            // 28e8c call 0x28dca
            { let r = { let t = m.env.code_va(0x28dca); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e91 jmp 0x28e9f
            pc = 0x28e9f; continue;
        }
        0x28e93 => {
            // block 28e93
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e93 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 28e95 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28e97 mov rdi,r14
            rdi = r14;
            // 28e9a call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x28e9f;
        }
        0x28e9f => {
            // block 28e9f
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e9f mov rax,r14
            rax = r14;
            // 28ea2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28ea3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28ea5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28ea6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44d9a size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044d9a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44d9a;
    loop {
        match pc {
        0x44d9a => {
            // block 44d9a
            #[cfg(feature = "trace")] eprintln!("f_00044d9a pc 44d9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44d9a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44d9b mov rbp,rsp
            rbp = rsp;
            // 44d9e sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // 44da2 mov r10d,ecx
            r10 = ((rcx as u32 as u64)) as u32 as u64;
            // 44da5 mov rax,rdx
            rax = rdx;
            // 44da8 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 44daa mov edx,DWORD PTR [rbp+0x10]
            rdx = (m.r32(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 44dad mov esi,DWORD PTR [rbp+0x18]
            rsi = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 44db0 mov DWORD PTR [rsp+0x18],esi
            m.w32(rsp.wrapping_add(0x18), ((rsi as u32 as u64)) as u32);
            // 44db4 mov DWORD PTR [rsp+0x10],edx
            m.w32(rsp.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // 44db8 mov DWORD PTR [rsp+0x8],r9d
            m.w32(rsp.wrapping_add(0x8), ((r9 as u32 as u64)) as u32);
            // 44dbd mov DWORD PTR [rsp],r8d
            m.w32(rsp, ((r8 as u32 as u64)) as u32);
            // 44dc1 mov DWORD PTR [rsp+0x20],0x1
            m.w32(rsp.wrapping_add(0x20), (0x1u64) as u32);
            // 44dc9 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 44dce mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 44dd3 mov r8,rax
            r8 = rax;
            // 44dd6 mov r9d,r10d
            r9 = ((r10 as u32 as u64)) as u32 as u64;
            // 44dd9 call 0x4e8a2
            { let r = { let t = m.env.code_va(0x4e8a2); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44dde add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // 44de2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44de3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe80a2 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e80a2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe80a2;
    loop {
        match pc {
        0xe80a2 => {
            // block e80a2
            #[cfg(feature = "trace")] eprintln!("f_000e80a2 pc e80a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e80a2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e80a3 mov rbp,rsp
            rbp = rsp;
            // e80a6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e80a7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e80a8 mov rbx,rdi
            rbx = rdi;
            // e80ab cmp QWORD PTR [rbx+0x90],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x90)) as u64, 0x0u64).1;
            // e80b3 je 0xe80c3
            if fl.e() { pc = 0xe80c3; continue; }
            pc = 0xe80b5;
        }
        0xe80b5 => {
            // block e80b5
            #[cfg(feature = "trace")] eprintln!("f_000e80a2 pc e80b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e80b5 lea rdi,[rbx+0x18]
            rdi = rbx.wrapping_add(0x18);
            // e80b9 call 0xebf12
            { let r = { let t = m.env.stub_va(0x6df); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e80be test rax,rax
            fl = logic::<64>(rax & rax);
            // e80c1 je 0xe80ca
            if fl.e() { pc = 0xe80ca; continue; }
            pc = 0xe80c3;
        }
        0xe80c3 => {
            // block e80c3
            #[cfg(feature = "trace")] eprintln!("f_000e80a2 pc e80c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e80c3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e80c7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e80c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e80c9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe80ca => {
            // block e80ca
            #[cfg(feature = "trace")] eprintln!("f_000e80a2 pc e80ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e80ca add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // e80ce mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e80d1 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e80d5 lea rdi,[rbx+rax*1]
            rdi = rbx.wrapping_add(rax);
            // e80d9 mov esi,DWORD PTR [rbx+rax*1+0x20]
            rsi = (m.r32(rbx.wrapping_add(rax).wrapping_add(0x20)) as u64) as u32 as u64;
            // e80dd or esi,0x4
            { let r = (rsi as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e80e0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e80e4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e80e5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e80e6 jmp 0xec0fe
            { let r = { let t = m.env.stub_va(0x8cb); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x27bad size=75
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00027bad<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x27bad;
    loop {
        match pc {
        0x27bad => {
            // block 27bad
            #[cfg(feature = "trace")] eprintln!("f_00027bad pc 27bad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27bad push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 27bae mov rbp,rsp
            rbp = rsp;
            // 27bb1 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 27bb3 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 27bb4 mov rbx,rdi
            rbx = rdi;
            // 27bb7 cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // 27bba ja 0x27bdc
            if fl.a() { pc = 0x27bdc; continue; }
            pc = 0x27bbc;
        }
        0x27bbc => {
            // block 27bbc
            #[cfg(feature = "trace")] eprintln!("f_00027bad pc 27bbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27bbc mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 27bbe lea rcx,[rip+0x1198db] # 0x1414a0
            rcx = m.env.data_va(351341);
            // 27bc5 mov r14,QWORD PTR [rcx+rax*8]
            r14 = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // 27bc9 mov rdi,r14
            rdi = r14;
            // 27bcc call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27bd1 mov rdi,rbx
            rdi = rbx;
            // 27bd4 mov rsi,r14
            rsi = r14;
            // 27bd7 mov rdx,rax
            rdx = rax;
            // 27bda jmp 0x27beb
            pc = 0x27beb; continue;
        }
        0x27bdc => {
            // block 27bdc
            #[cfg(feature = "trace")] eprintln!("f_00027bad pc 27bdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27bdc lea rsi,[rip+0x102cb6] # 0x12a899
            rsi = m.env.data_va(258150);
            // 27be3 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 27be8 mov rdi,rbx
            rdi = rbx;
            pc = 0x27beb;
        }
        0x27beb => {
            // block 27beb
            #[cfg(feature = "trace")] eprintln!("f_00027bad pc 27beb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27beb call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27bf0 mov rax,rbx
            rax = rbx;
            // 27bf3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 27bf4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 27bf6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 27bf7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfc14 size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fc14<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xfc14;
    loop {
        match pc {
        0xfc14 => {
            // block fc14
            #[cfg(feature = "trace")] eprintln!("f_0000fc14 pc fc14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fc14 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fc15 mov rbp,rsp
            rbp = rsp;
            // fc18 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fc19 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // fc1a mov rbx,rdi
            rbx = rdi;
            // fc1d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // fc20 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fc25 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // fc27 je 0xfc56
            if fl.e() { pc = 0xfc56; continue; }
            pc = 0xfc29;
        }
        0xfc29 => {
            // block fc29
            #[cfg(feature = "trace")] eprintln!("f_0000fc14 pc fc29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fc29 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // fc30 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fc33 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // fc37 mov esi,0x150
            rsi = (0x150u64) as u32 as u64;
            // fc3c mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // fc41 mov r8d,0x7
            r8 = (0x7u64) as u32 as u64;
            // fc47 mov rdi,rbx
            rdi = rbx;
            // fc4a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fc4d cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // fc51 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // fc54 jmp 0xfc58
            pc = 0xfc58; continue;
        }
        0xfc56 => {
            // block fc56
            #[cfg(feature = "trace")] eprintln!("f_0000fc14 pc fc56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fc56 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xfc58;
        }
        0xfc58 => {
            // block fc58
            #[cfg(feature = "trace")] eprintln!("f_0000fc14 pc fc58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fc58 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // fc5c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fc5d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fc5e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfcb6 size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fcb6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xfcb6;
    loop {
        match pc {
        0xfcb6 => {
            // block fcb6
            #[cfg(feature = "trace")] eprintln!("f_0000fcb6 pc fcb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fcb6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fcb7 mov rbp,rsp
            rbp = rsp;
            // fcba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fcbb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // fcbc mov rbx,rdi
            rbx = rdi;
            // fcbf mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // fcc2 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fcc7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // fcc9 je 0xfcf8
            if fl.e() { pc = 0xfcf8; continue; }
            pc = 0xfccb;
        }
        0xfccb => {
            // block fccb
            #[cfg(feature = "trace")] eprintln!("f_0000fcb6 pc fccb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fccb mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // fcd2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fcd5 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // fcd9 mov esi,0x150
            rsi = (0x150u64) as u32 as u64;
            // fcde mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // fce3 mov r8d,0x6
            r8 = (0x6u64) as u32 as u64;
            // fce9 mov rdi,rbx
            rdi = rbx;
            // fcec call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fcef cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // fcf3 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // fcf6 jmp 0xfcfa
            pc = 0xfcfa; continue;
        }
        0xfcf8 => {
            // block fcf8
            #[cfg(feature = "trace")] eprintln!("f_0000fcb6 pc fcf8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fcf8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xfcfa;
        }
        0xfcfa => {
            // block fcfa
            #[cfg(feature = "trace")] eprintln!("f_0000fcb6 pc fcfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fcfa add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // fcfe pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fcff pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fd00 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28d7e size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028d7e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28d7e;
    loop {
        match pc {
        0x28d7e => {
            // block 28d7e
            #[cfg(feature = "trace")] eprintln!("f_00028d7e pc 28d7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28d7e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28d7f mov rbp,rsp
            rbp = rsp;
            // 28d82 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28d84 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28d85 mov r14,rsi
            r14 = rsi;
            // 28d88 mov rbx,rdi
            rbx = rdi;
            // 28d8b mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 28d93 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 28d9b mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 28da3 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 28dab mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 28db2 mov esi,DWORD PTR [r14+0x8]
            rsi = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28db6 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28dbb mov rdi,rbx
            rdi = rbx;
            // 28dbe mov rsi,r14
            rsi = r14;
            // 28dc1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28dc2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28dc4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28dc5 jmp 0x28dca
            { let r = { let t = m.env.code_va(0x28dca); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28e12 size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028e12<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28e12;
    loop {
        match pc {
        0x28e12 => {
            // block 28e12
            #[cfg(feature = "trace")] eprintln!("f_00028e12 pc 28e12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e12 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28e13 mov rbp,rsp
            rbp = rsp;
            // 28e16 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28e18 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28e19 mov r14,rsi
            r14 = rsi;
            // 28e1c mov rbx,rdi
            rbx = rdi;
            // 28e1f mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 28e27 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 28e2f mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 28e37 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 28e3f mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 28e46 mov esi,DWORD PTR [r14+0x8]
            rsi = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28e4a call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e4f mov rdi,rbx
            rdi = rbx;
            // 28e52 mov rsi,r14
            rsi = r14;
            // 28e55 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28e56 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28e58 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28e59 jmp 0x28dca
            { let r = { let t = m.env.code_va(0x28dca); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e038 size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e038<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e038;
    loop {
        match pc {
        0x2e038 => {
            // block 2e038
            #[cfg(feature = "trace")] eprintln!("f_0002e038 pc 2e038 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e038 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e039 mov rbp,rsp
            rbp = rsp;
            // 2e03c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e03d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2e03e mov rbx,rdi
            rbx = rdi;
            // 2e041 cmp BYTE PTR [rbx+0x61],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x61)) as u64, 0x0u64).1;
            // 2e045 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2e049 je 0x2e05a
            if fl.e() { pc = 0x2e05a; continue; }
            pc = 0x2e04b;
        }
        0x2e04b => {
            // block 2e04b
            #[cfg(feature = "trace")] eprintln!("f_0002e038 pc 2e04b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e04b test rax,rax
            fl = logic::<64>(rax & rax);
            // 2e04e je 0x2e078
            if fl.e() { pc = 0x2e078; continue; }
            pc = 0x2e050;
        }
        0x2e050 => {
            // block 2e050
            #[cfg(feature = "trace")] eprintln!("f_0002e038 pc 2e050 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e050 mov rdi,rax
            rdi = rax;
            // 2e053 call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e058 jmp 0x2e070
            pc = 0x2e070; continue;
        }
        0x2e05a => {
            // block 2e05a
            #[cfg(feature = "trace")] eprintln!("f_0002e038 pc 2e05a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e05a test rax,rax
            fl = logic::<64>(rax & rax);
            // 2e05d je 0x2e078
            if fl.e() { pc = 0x2e078; continue; }
            pc = 0x2e05f;
        }
        0x2e05f => {
            // block 2e05f
            #[cfg(feature = "trace")] eprintln!("f_0002e038 pc 2e05f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e05f mov rcx,QWORD PTR [rbx+0x68]
            rcx = m.r64(rbx.wrapping_add(0x68)) as u64;
            // 2e063 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 2e066 je 0x2e078
            if fl.e() { pc = 0x2e078; continue; }
            pc = 0x2e068;
        }
        0x2e068 => {
            // block 2e068
            #[cfg(feature = "trace")] eprintln!("f_0002e038 pc 2e068 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e068 mov rdi,rbx
            rdi = rbx;
            // 2e06b mov rsi,rax
            rsi = rax;
            // 2e06e call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e070;
        }
        0x2e070 => {
            // block 2e070
            #[cfg(feature = "trace")] eprintln!("f_0002e038 pc 2e070 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e070 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0x2e078;
        }
        0x2e078 => {
            // block 2e078
            #[cfg(feature = "trace")] eprintln!("f_0002e038 pc 2e078 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e078 mov BYTE PTR [rbx+0x61],0x0
            m.w8(rbx.wrapping_add(0x61), (0x0u64) as u8);
            // 2e07c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2e080 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e081 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e082 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x559a8 size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000559a8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x559a8;
    loop {
        match pc {
        0x559a8 => {
            // block 559a8
            #[cfg(feature = "trace")] eprintln!("f_000559a8 pc 559a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 559a8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 559a9 mov rbp,rsp
            rbp = rsp;
            // 559ac push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 559ae push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 559af sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 559b3 mov r14,rdi
            r14 = rdi;
            // 559b6 lea rdi,[r14+0x180]
            rdi = r14.wrapping_add(0x180);
            // 559bd mov QWORD PTR [rbp-0x18],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (rdi) as u64);
            // 559c1 call 0x566f8
            { let r = { let t = m.env.code_va(0x566f8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 559c6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 559c8 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x559ca;
        }
        0x559ca => {
            // block 559ca
            #[cfg(feature = "trace")] eprintln!("f_000559a8 pc 559ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 559ca mov ecx,DWORD PTR [r14+rax*4]
            rcx = (m.r32(r14.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 559ce test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 559d0 je 0x559df
            if fl.e() { pc = 0x559df; continue; }
            pc = 0x559d2;
        }
        0x559d2 => {
            // block 559d2
            #[cfg(feature = "trace")] eprintln!("f_000559a8 pc 559d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 559d2 add ebx,ecx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 559d4 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 559d7 movzx ecx,ax
            rcx = ((rax as u16 as u64)) as u32 as u64;
            // 559da cmp ecx,0x20
            fl = sub::<32>((rcx as u32 as u64), 0x20u64).1;
            // 559dd jb 0x559ca
            if fl.b() { pc = 0x559ca; continue; }
            pc = 0x559df;
        }
        0x559df => {
            // block 559df
            #[cfg(feature = "trace")] eprintln!("f_000559a8 pc 559df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 559df lea rdi,[rbp-0x18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 559e3 call 0x56744
            { let r = { let t = m.env.code_va(0x56744); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 559e8 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 559ea add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 559ee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 559ef pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 559f1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 559f2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0d78 size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0d78<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0d78;
    loop {
        match pc {
        0xe0d78 => {
            // block e0d78
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d78 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0d79 mov rbp,rsp
            rbp = rsp;
            // e0d7c sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0d80 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // e0d87 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0d8a lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0d8e call QWORD PTR [rax+0x1ae8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0d94 mov ecx,DWORD PTR [rbp-0x4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0d97 shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e0d9a and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0d9d mov DWORD PTR [rbp-0x4],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), ((rcx as u32 as u64)) as u32);
            // e0da0 mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // e0da5 je 0xe0dbd
            if fl.e() { pc = 0xe0dbd; continue; }
            pc = 0xe0da7;
        }
        0xe0da7 => {
            // block e0da7
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0da7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0da7 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // e0daa jne 0xe0db0
            if fl.ne() { pc = 0xe0db0; continue; }
            pc = 0xe0dac;
        }
        0xe0dac => {
            // block e0dac
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0dac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0dac xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0dae jmp 0xe0dbd
            pc = 0xe0dbd; continue;
        }
        0xe0db0 => {
            // block e0db0
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0db0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0db0 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // e0db3 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // e0db6 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // e0db9 lea eax,[rax+rax*1+0x1]
            rax = (rax.wrapping_add(rax).wrapping_add(0x1)) as u32 as u64;
            pc = 0xe0dbd;
        }
        0xe0dbd => {
            // block e0dbd
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0dbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0dbd add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0dc1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0dc2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe3132 size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e3132<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe3132;
    loop {
        match pc {
        0xe3132 => {
            // block e3132
            #[cfg(feature = "trace")] eprintln!("f_000e3132 pc e3132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e3132 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e3133 mov rbp,rsp
            rbp = rsp;
            // e3136 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e3138 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e3139 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e313d mov r14,rdx
            r14 = rdx;
            // e3140 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e3142 shl ebx,0x3
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x3u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e3145 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e3148 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e314b lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e314f mov esi,0x4001d
            rsi = (0x4001du64) as u32 as u64;
            // e3154 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e3159 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e315c call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e315f mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // e3164 mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e3166 shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e3168 and eax,DWORD PTR [rbp-0x14]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e316b mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e316d shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e316f mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // e3172 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e3174 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e3178 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e3179 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e317b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e317c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8a64 size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008a64<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x8a64;
    loop {
        match pc {
        0x8a64 => {
            // block 8a64
            #[cfg(feature = "trace")] eprintln!("f_00008a64 pc 8a64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a64 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8a65 mov rbp,rsp
            rbp = rsp;
            // 8a68 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8a69 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 8a6a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 8a6d lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 8a71 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 8a73 mov esi,0x51
            rsi = (0x51u64) as u32 as u64;
            // 8a78 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 8a7d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8a80 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8a83 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8a85 je 0x8aa9
            if fl.e() { pc = 0x8aa9; continue; }
            pc = 0x8a87;
        }
        0x8a87 => {
            // block 8a87
            #[cfg(feature = "trace")] eprintln!("f_00008a64 pc 8a87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a87 mov esi,DWORD PTR [rbp-0xc]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 8a8a test sil,0x1
            fl = logic::<8>((rsi as u8 as u64) & 0x1u64);
            // 8a8e je 0x8aa9
            if fl.e() { pc = 0x8aa9; continue; }
            pc = 0x8a90;
        }
        0x8a90 => {
            // block 8a90
            #[cfg(feature = "trace")] eprintln!("f_00008a64 pc 8a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a90 mov edi,esi
            rdi = ((rsi as u32 as u64)) as u32 as u64;
            // 8a92 shr edi,0x1c
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x1cu64, fl); rdi = (r) as u32 as u64; fl = f; }
            // 8a95 shr esi,0x18
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x18u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 8a98 and esi,0x7
            { let r = (rsi as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8a9b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 8a9d xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 8a9f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8aa2 call 0x6d8
            { let r = { let t = m.env.code_va(0x6d8); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8aa7 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x8aa9;
        }
        0x8aa9 => {
            // block 8aa9
            #[cfg(feature = "trace")] eprintln!("f_00008a64 pc 8aa9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8aa9 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 8aab add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 8aaf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8ab0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8ab1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x157de size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000157de<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x157de;
    loop {
        match pc {
        0x157de => {
            // block 157de
            #[cfg(feature = "trace")] eprintln!("f_000157de pc 157de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 157de push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 157df mov rbp,rsp
            rbp = rsp;
            // 157e2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 157e4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 157e5 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 157e9 mov r14,rsi
            r14 = rsi;
            // 157ec mov rbx,rdi
            rbx = rdi;
            // 157ef lea rdx,[r14+0x20]
            rdx = r14.wrapping_add(0x20);
            // 157f3 lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 157f7 call 0xeb91e
            { let r = { let t = m.env.stub_va(0xeb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 157fc mov rcx,rax
            rcx = rax;
            // 157ff mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 15802 test rax,rax
            fl = logic::<64>(rax & rax);
            // 15805 je 0x1580b
            if fl.e() { pc = 0x1580b; continue; }
            pc = 0x15807;
        }
        0x15807 => {
            // block 15807
            #[cfg(feature = "trace")] eprintln!("f_000157de pc 15807 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15807 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 15809 jmp 0x15822
            pc = 0x15822; continue;
        }
        0x1580b => {
            // block 1580b
            #[cfg(feature = "trace")] eprintln!("f_000157de pc 1580b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1580b mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 1580f mov rdi,rbx
            rdi = rbx;
            // 15812 mov rdx,rcx
            rdx = rcx;
            // 15815 mov rcx,r14
            rcx = r14;
            // 15818 call 0xeb930
            { let r = { let t = m.env.stub_va(0xfd); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1581d mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            // 1581f mov rax,r14
            rax = r14;
            pc = 0x15822;
        }
        0x15822 => {
            // block 15822
            #[cfg(feature = "trace")] eprintln!("f_000157de pc 15822 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15822 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 15826 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15827 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15829 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1582a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x32264 size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00032264<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x32264;
    loop {
        match pc {
        0x32264 => {
            // block 32264
            #[cfg(feature = "trace")] eprintln!("f_00032264 pc 32264 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32264 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 32265 mov rbp,rsp
            rbp = rsp;
            // 32268 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3226a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3226b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 3226f mov r14,rsi
            r14 = rsi;
            // 32272 mov rbx,rdi
            rbx = rdi;
            // 32275 lea rdx,[r14+0x1c]
            rdx = r14.wrapping_add(0x1c);
            // 32279 lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 3227d call 0xebb1c
            { let r = { let t = m.env.stub_va(0x2e9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32282 mov rcx,rax
            rcx = rax;
            // 32285 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 32288 test rax,rax
            fl = logic::<64>(rax & rax);
            // 3228b je 0x32291
            if fl.e() { pc = 0x32291; continue; }
            pc = 0x3228d;
        }
        0x3228d => {
            // block 3228d
            #[cfg(feature = "trace")] eprintln!("f_00032264 pc 3228d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3228d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3228f jmp 0x322a8
            pc = 0x322a8; continue;
        }
        0x32291 => {
            // block 32291
            #[cfg(feature = "trace")] eprintln!("f_00032264 pc 32291 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32291 mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 32295 mov rdi,rbx
            rdi = rbx;
            // 32298 mov rdx,rcx
            rdx = rcx;
            // 3229b mov rcx,r14
            rcx = r14;
            // 3229e call 0xebcc6
            { let r = { let t = m.env.stub_va(0x493); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 322a3 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            // 322a5 mov rax,r14
            rax = r14;
            pc = 0x322a8;
        }
        0x322a8 => {
            // block 322a8
            #[cfg(feature = "trace")] eprintln!("f_00032264 pc 322a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 322a8 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 322ac pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 322ad pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 322af pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 322b0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb51e0 size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b51e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb51e0;
    loop {
        match pc {
        0xb51e0 => {
            // block b51e0
            #[cfg(feature = "trace")] eprintln!("f_000b51e0 pc b51e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b51e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b51e1 mov rbp,rsp
            rbp = rsp;
            // b51e4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b51e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b51e7 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b51eb mov r14,rsi
            r14 = rsi;
            // b51ee mov rbx,rdi
            rbx = rdi;
            // b51f1 lea rdx,[r14+0x20]
            rdx = r14.wrapping_add(0x20);
            // b51f5 lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // b51f9 call 0xebc4e
            { let r = { let t = m.env.stub_va(0x41b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b51fe mov rcx,rax
            rcx = rax;
            // b5201 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // b5204 test rax,rax
            fl = logic::<64>(rax & rax);
            // b5207 je 0xb520d
            if fl.e() { pc = 0xb520d; continue; }
            pc = 0xb5209;
        }
        0xb5209 => {
            // block b5209
            #[cfg(feature = "trace")] eprintln!("f_000b51e0 pc b5209 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5209 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b520b jmp 0xb5224
            pc = 0xb5224; continue;
        }
        0xb520d => {
            // block b520d
            #[cfg(feature = "trace")] eprintln!("f_000b51e0 pc b520d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b520d mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b5211 mov rdi,rbx
            rdi = rbx;
            // b5214 mov rdx,rcx
            rdx = rcx;
            // b5217 mov rcx,r14
            rcx = r14;
            // b521a call 0xebc5a
            { let r = { let t = m.env.stub_va(0x427); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b521f mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            // b5221 mov rax,r14
            rax = r14;
            pc = 0xb5224;
        }
        0xb5224 => {
            // block b5224
            #[cfg(feature = "trace")] eprintln!("f_000b51e0 pc b5224 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5224 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b5228 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5229 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b522b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b522c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb53e4 size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b53e4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb53e4;
    loop {
        match pc {
        0xb53e4 => {
            // block b53e4
            #[cfg(feature = "trace")] eprintln!("f_000b53e4 pc b53e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b53e4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b53e5 mov rbp,rsp
            rbp = rsp;
            // b53e8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b53ea push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b53eb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b53ef mov r14,rsi
            r14 = rsi;
            // b53f2 mov rbx,rdi
            rbx = rdi;
            // b53f5 lea rdx,[r14+0x20]
            rdx = r14.wrapping_add(0x20);
            // b53f9 lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // b53fd call 0xebc72
            { let r = { let t = m.env.stub_va(0x43f); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5402 mov rcx,rax
            rcx = rax;
            // b5405 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // b5408 test rax,rax
            fl = logic::<64>(rax & rax);
            // b540b je 0xb5411
            if fl.e() { pc = 0xb5411; continue; }
            pc = 0xb540d;
        }
        0xb540d => {
            // block b540d
            #[cfg(feature = "trace")] eprintln!("f_000b53e4 pc b540d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b540d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b540f jmp 0xb5428
            pc = 0xb5428; continue;
        }
        0xb5411 => {
            // block b5411
            #[cfg(feature = "trace")] eprintln!("f_000b53e4 pc b5411 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5411 mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b5415 mov rdi,rbx
            rdi = rbx;
            // b5418 mov rdx,rcx
            rdx = rcx;
            // b541b mov rcx,r14
            rcx = r14;
            // b541e call 0xebc7e
            { let r = { let t = m.env.stub_va(0x44b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5423 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            // b5425 mov rax,r14
            rax = r14;
            pc = 0xb5428;
        }
        0xb5428 => {
            // block b5428
            #[cfg(feature = "trace")] eprintln!("f_000b53e4 pc b5428 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5428 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b542c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b542d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b542f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5430 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4376 size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4376<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4376;
    loop {
        match pc {
        0xc4376 => {
            // block c4376
            #[cfg(feature = "trace")] eprintln!("f_000c4376 pc c4376 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4376 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4377 mov rbp,rsp
            rbp = rsp;
            // c437a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c437c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c437d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c4381 mov r14,rsi
            r14 = rsi;
            // c4384 mov rbx,rdi
            rbx = rdi;
            // c4387 lea rdx,[r14+0x1c]
            rdx = r14.wrapping_add(0x1c);
            // c438b lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // c438f call 0xebdfe
            { let r = { let t = m.env.stub_va(0x5cb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4394 mov rcx,rax
            rcx = rax;
            // c4397 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // c439a test rax,rax
            fl = logic::<64>(rax & rax);
            // c439d je 0xc43a3
            if fl.e() { pc = 0xc43a3; continue; }
            pc = 0xc439f;
        }
        0xc439f => {
            // block c439f
            #[cfg(feature = "trace")] eprintln!("f_000c4376 pc c439f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c439f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c43a1 jmp 0xc43ba
            pc = 0xc43ba; continue;
        }
        0xc43a3 => {
            // block c43a3
            #[cfg(feature = "trace")] eprintln!("f_000c4376 pc c43a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43a3 mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // c43a7 mov rdi,rbx
            rdi = rbx;
            // c43aa mov rdx,rcx
            rdx = rcx;
            // c43ad mov rcx,r14
            rcx = r14;
            // c43b0 call 0xebe0a
            { let r = { let t = m.env.stub_va(0x5d7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c43b5 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            // c43b7 mov rax,r14
            rax = r14;
            pc = 0xc43ba;
        }
        0xc43ba => {
            // block c43ba
            #[cfg(feature = "trace")] eprintln!("f_000c4376 pc c43ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43ba add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c43be pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c43bf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c43c1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c43c2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc80e size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c80e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc80e;
    loop {
        match pc {
        0xc80e => {
            // block c80e
            #[cfg(feature = "trace")] eprintln!("f_0000c80e pc c80e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c80e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c80f mov rbp,rsp
            rbp = rsp;
            // c812 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c814 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c815 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // c818 mov rbx,rdi
            rbx = rdi;
            // c81b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c81e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c824 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c826 call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c82b test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c82d je 0xc856
            if fl.e() { pc = 0xc856; continue; }
            pc = 0xc82f;
        }
        0xc82f => {
            // block c82f
            #[cfg(feature = "trace")] eprintln!("f_0000c80e pc c82f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c82f mov r9,QWORD PTR [rbx]
            r9 = m.r64(rbx) as u64;
            // c832 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // c835 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // c838 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // c83b lea ecx,[rax*8+0x7]
            rcx = (rax.wrapping_mul(8).wrapping_add(0x7)) as u32 as u64;
            // c842 mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // c847 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c84a mov rdi,rbx
            rdi = rbx;
            // c84d mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c850 call QWORD PTR [r9+0x30]
            { let r = { let t = m.r64(r9.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c854 jmp 0xc858
            pc = 0xc858; continue;
        }
        0xc856 => {
            // block c856
            #[cfg(feature = "trace")] eprintln!("f_0000c80e pc c856 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c856 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc858;
        }
        0xc858 => {
            // block c858
            #[cfg(feature = "trace")] eprintln!("f_0000c80e pc c858 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c858 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c859 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c85b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c85c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe564 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e564<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe564;
    loop {
        match pc {
        0xe564 => {
            // block e564
            #[cfg(feature = "trace")] eprintln!("f_0000e564 pc e564 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e564 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e565 mov rbp,rsp
            rbp = rsp;
            // e568 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e56a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e56b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e56f mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e572 mov rbx,rdi
            rbx = rdi;
            // e575 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e578 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e57e mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e580 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e582 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e584 jne 0xe5ab
            if fl.ne() { pc = 0xe5ab; continue; }
            pc = 0xe586;
        }
        0xe586 => {
            // block e586
            #[cfg(feature = "trace")] eprintln!("f_0000e564 pc e586 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e586 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e589 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // e58c lea rdx,[rip+0xf119d] # 0xff730
            rdx = m.env.data_va(81661);
            // e593 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e596 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e59a mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // e59f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e5a2 mov rdi,rbx
            rdi = rbx;
            // e5a5 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e5a8 mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            pc = 0xe5ab;
        }
        0xe5ab => {
            // block e5ab
            #[cfg(feature = "trace")] eprintln!("f_0000e564 pc e5ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5ab add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e5af pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e5b0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e5b2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5b3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x56ae size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000056ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x56ae;
    loop {
        match pc {
        0x56ae => {
            // block 56ae
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 56af mov rbp,rsp
            rbp = rsp;
            // 56b2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 56b4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 56b5 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 56b9 mov r14,rdx
            r14 = rdx;
            // 56bc mov rbx,rsi
            rbx = rsi;
            // 56bf mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // 56c3 mov BYTE PTR [rbp-0x12],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffee), (0x0u64) as u8);
            // 56c7 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 56ca je 0x56f5
            if fl.e() { pc = 0x56f5; continue; }
            pc = 0x56cc;
        }
        0x56cc => {
            // block 56cc
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56cc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 56cf lea rsi,[rbp-0x11]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffef);
            // 56d3 lea rdx,[rbp-0x12]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffee);
            // 56d7 call QWORD PTR [rax+0x5d8]
            { let r = { let t = m.r64(rax.wrapping_add(0x5d8)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 56dd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 56df je 0x56f5
            if fl.e() { pc = 0x56f5; continue; }
            pc = 0x56e1;
        }
        0x56e1 => {
            // block 56e1
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56e1 mov al,BYTE PTR [rbp-0x11]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u8 as u64);
            // 56e4 mov BYTE PTR [rbx],al
            m.w8(rbx, ((rax as u8 as u64)) as u8);
            // 56e6 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 56e8 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 56eb je 0x56f7
            if fl.e() { pc = 0x56f7; continue; }
            pc = 0x56ed;
        }
        0x56ed => {
            // block 56ed
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56ed mov cl,BYTE PTR [rbp-0x12]
            rcx = (rcx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffee)) as u64) as u8 as u64);
            // 56f0 mov BYTE PTR [r14],cl
            m.w8(r14, ((rcx as u8 as u64)) as u8);
            // 56f3 jmp 0x56f7
            pc = 0x56f7; continue;
        }
        0x56f5 => {
            // block 56f5
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x56f7;
        }
        0x56f7 => {
            // block 56f7
            #[cfg(feature = "trace")] eprintln!("f_000056ae pc 56f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56f7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 56fb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 56fc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 56fe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 56ff ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6c96 size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006c96<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6c96;
    loop {
        match pc {
        0x6c96 => {
            // block 6c96
            #[cfg(feature = "trace")] eprintln!("f_00006c96 pc 6c96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6c96 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6c97 mov rbp,rsp
            rbp = rsp;
            // 6c9a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6c9c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6c9e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6c9f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6ca0 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 6ca3 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 6ca6 mov rbx,rdi
            rbx = rdi;
            // 6ca9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6cac call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6cb2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6cb4 je 0x6cba
            if fl.e() { pc = 0x6cba; continue; }
            pc = 0x6cb6;
        }
        0x6cb6 => {
            // block 6cb6
            #[cfg(feature = "trace")] eprintln!("f_00006c96 pc 6cb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6cb6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6cb8 jmp 0x6cdc
            pc = 0x6cdc; continue;
        }
        0x6cba => {
            // block 6cba
            #[cfg(feature = "trace")] eprintln!("f_00006c96 pc 6cba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6cba mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6cbd movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 6cc1 lea rdx,[rip+0xf86f8] # 0xff3c0
            rdx = m.env.data_va(80781);
            // 6cc8 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6ccb mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // 6cd0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6cd3 mov rdi,rbx
            rdi = rbx;
            // 6cd6 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 6cd9 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6cdc;
        }
        0x6cdc => {
            // block 6cdc
            #[cfg(feature = "trace")] eprintln!("f_00006c96 pc 6cdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6cdc add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6ce0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6ce1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6ce3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6ce5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6ce6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x111d0 size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000111d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x111d0;
    loop {
        match pc {
        0x111d0 => {
            // block 111d0
            #[cfg(feature = "trace")] eprintln!("f_000111d0 pc 111d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 111d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 111d1 mov rbp,rsp
            rbp = rsp;
            // 111d4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 111d6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 111d7 mov edi,0x48
            rdi = (0x48u64) as u32 as u64;
            // 111dc call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 111e1 mov rbx,rax
            rbx = rax;
            // 111e4 mov rdi,rbx
            rdi = rbx;
            // 111e7 call 0xe0758
            { let r = { let t = m.env.code_va(0xe0758); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 111ec lea rax,[rip+0x1300ad] # 0x1412a0
            rax = m.env.data_va(350829);
            // 111f3 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 111f6 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 111fe mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 11206 mov rax,rbx
            rax = rbx;
            // 11209 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1120a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1120c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1120d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1120e => {
            // block 1120e
            #[cfg(feature = "trace")] eprintln!("f_000111d0 pc 1120e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1120e mov r14,rax
            r14 = rax;
            // 11211 mov rdi,rbx
            rdi = rbx;
            // 11214 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11219 mov rdi,r14
            rdi = r14;
            // 1121c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e28e size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e28e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e28e;
    loop {
        match pc {
        0x2e28e => {
            // block 2e28e
            #[cfg(feature = "trace")] eprintln!("f_0002e28e pc 2e28e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e28e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e28f mov rbp,rsp
            rbp = rsp;
            // 2e292 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e294 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e295 mov rbx,rdi
            rbx = rdi;
            // 2e298 lea rax,[rip+0x113251] # 0x1414f0
            rax = m.env.data_va(351421);
            // 2e29f mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 2e2a2 call 0x2e220
            { let r = { let t = m.env.code_va(0x2e220); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e2a7 lea rdi,[rbx+0x28]
            rdi = rbx.wrapping_add(0x28);
            // 2e2ab call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e2b0 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 2e2b4 mov rdi,rbx
            rdi = rbx;
            // 2e2b7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e2b8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e2ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e2bb jmp 0xebaf8
            { let r = { let t = m.env.stub_va(0x2c5); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2e2c0 => {
            // block 2e2c0
            #[cfg(feature = "trace")] eprintln!("f_0002e28e pc 2e2c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e2c0 mov r14,rax
            r14 = rax;
            // 2e2c3 lea rdi,[rbx+0x28]
            rdi = rbx.wrapping_add(0x28);
            // 2e2c7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e2cc add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 2e2d0 mov rdi,rbx
            rdi = rbx;
            // 2e2d3 call 0xebaf8
            { let r = { let t = m.env.stub_va(0x2c5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e2d8 mov rdi,r14
            rdi = r14;
            // 2e2db call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4328a size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004328a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4328a;
    loop {
        match pc {
        0x4328a => {
            // block 4328a
            #[cfg(feature = "trace")] eprintln!("f_0004328a pc 4328a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4328a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4328b mov rbp,rsp
            rbp = rsp;
            // 4328e sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43292 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 43295 jle 0x4329b
            if fl.le() { pc = 0x4329b; continue; }
            pc = 0x43297;
        }
        0x43297 => {
            // block 43297
            #[cfg(feature = "trace")] eprintln!("f_0004328a pc 43297 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43297 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43299 jmp 0x432d5
            pc = 0x432d5; continue;
        }
        0x4329b => {
            // block 4329b
            #[cfg(feature = "trace")] eprintln!("f_0004328a pc 4329b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4329b mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // 432a2 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 432a4 mov eax,0xffff
            rax = (0xffffu64) as u32 as u64;
            // 432a9 mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // 432ae cmove ecx,eax
            if fl.e() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 432b1 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 432b4 movzx r8d,al
            r8 = ((rax as u8 as u64)) as u32 as u64;
            // 432b8 shl r8d,0x4
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x4u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // 432bc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 432bf lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // 432c3 mov esi,0x90e
            rsi = (0x90eu64) as u32 as u64;
            // 432c8 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 432cb xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 432cd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 432cf cmovne ecx,DWORD PTR [rbp-0x4]
            if fl.ne() { rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 432d3 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x432d5;
        }
        0x432d5 => {
            // block 432d5
            #[cfg(feature = "trace")] eprintln!("f_0004328a pc 432d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 432d5 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 432d9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 432da ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4383a size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004383a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4383a;
    loop {
        match pc {
        0x4383a => {
            // block 4383a
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 4383a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4383a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4383b mov rbp,rsp
            rbp = rsp;
            // 4383e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43840 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43841 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43845 mov r14,rdx
            r14 = rdx;
            // 43848 mov rbx,rsi
            rbx = rsi;
            // 4384b mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 43852 mov DWORD PTR [rbp-0x18],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0x8u64) as u32);
            // 43859 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 4385c lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 43860 lea rdx,[rbp-0x18]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 43864 call QWORD PTR [rax+0x890]
            { let r = { let t = m.r64(rax.wrapping_add(0x890)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4386a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4386c je 0x43883
            if fl.e() { pc = 0x43883; continue; }
            pc = 0x4386e;
        }
        0x4386e => {
            // block 4386e
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 4386e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4386e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 43871 je 0x43878
            if fl.e() { pc = 0x43878; continue; }
            pc = 0x43873;
        }
        0x43873 => {
            // block 43873
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 43873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43873 mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 43876 mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            pc = 0x43878;
        }
        0x43878 => {
            // block 43878
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 43878 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43878 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 4387b je 0x43883
            if fl.e() { pc = 0x43883; continue; }
            pc = 0x4387d;
        }
        0x4387d => {
            // block 4387d
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 4387d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4387d mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 43880 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            pc = 0x43883;
        }
        0x43883 => {
            // block 43883
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 43883 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43883 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43887 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43888 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4388a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4388b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x684ae size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000684ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x684ae;
    loop {
        match pc {
        0x684ae => {
            // block 684ae
            #[cfg(feature = "trace")] eprintln!("f_000684ae pc 684ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 684ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 684af mov rbp,rsp
            rbp = rsp;
            // 684b2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 684b3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 684b4 mov rbx,rdi
            rbx = rdi;
            // 684b7 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 684ba ja 0x684de
            if fl.a() { pc = 0x684de; continue; }
            pc = 0x684bc;
        }
        0x684bc => {
            // block 684bc
            #[cfg(feature = "trace")] eprintln!("f_000684ae pc 684bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 684bc mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 684be lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // 684c2 lea rcx,[rip+0x15fdf7] # 0x1c82c0
            rcx = m.env.data_va(903821);
            // 684c9 lea rdx,[rcx+rax*8]
            rdx = rcx.wrapping_add(rax.wrapping_mul(8));
            // 684cd lea rsi,[rip+0x15feac] # 0x1c8380
            rsi = m.env.data_va(904013);
            // 684d4 mov rdi,rbx
            rdi = rbx;
            // 684d7 call 0x69328
            { let r = { let t = m.env.code_va(0x69328); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 684dc jmp 0x684f5
            pc = 0x684f5; continue;
        }
        0x684de => {
            // block 684de
            #[cfg(feature = "trace")] eprintln!("f_000684ae pc 684de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 684de lea rax,[rbx+0x8]
            rax = rbx.wrapping_add(0x8);
            // 684e2 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 684ea mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 684f2 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            pc = 0x684f5;
        }
        0x684f5 => {
            // block 684f5
            #[cfg(feature = "trace")] eprintln!("f_000684ae pc 684f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 684f5 mov rax,rbx
            rax = rbx;
            // 684f8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 684fc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 684fd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 684fe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe317e size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e317e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe317e;
    loop {
        match pc {
        0xe317e => {
            // block e317e
            #[cfg(feature = "trace")] eprintln!("f_000e317e pc e317e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e317e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e317f mov rbp,rsp
            rbp = rsp;
            // e3182 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e3183 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e3184 mov rbx,rdi
            rbx = rdi;
            // e3187 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e318a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e318d lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // e3191 mov esi,0x42025
            rsi = (0x42025u64) as u32 as u64;
            // e3196 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e319b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e319e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e31a1 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e31a4 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e31a7 lea rdx,[rbp-0x10]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // e31ab mov esi,0x42026
            rsi = (0x42026u64) as u32 as u64;
            // e31b0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e31b5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e31b8 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e31bb mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // e31be shl rcx,0x20
            { let (r, f) = shl::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // e31c2 mov eax,DWORD PTR [rbp-0x10]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64) as u32 as u64;
            // e31c5 or rax,rcx
            { let r = rax | rcx; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // e31c8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e31cc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e31cd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e31ce ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4b8 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e4b8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4b8;
    loop {
        match pc {
        0xe4b8 => {
            // block e4b8
            #[cfg(feature = "trace")] eprintln!("f_0000e4b8 pc e4b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4b8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4b9 mov rbp,rsp
            rbp = rsp;
            // e4bc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4be push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4bf sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e4c3 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e4c6 mov rbx,rdi
            rbx = rdi;
            // e4c9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e4cc call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4d2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e4d4 je 0xe4da
            if fl.e() { pc = 0xe4da; continue; }
            pc = 0xe4d6;
        }
        0xe4d6 => {
            // block e4d6
            #[cfg(feature = "trace")] eprintln!("f_0000e4b8 pc e4d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4d6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e4d8 jmp 0xe503
            pc = 0xe503; continue;
        }
        0xe4da => {
            // block e4da
            #[cfg(feature = "trace")] eprintln!("f_0000e4b8 pc e4da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e4dd mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // e4e0 lea rdx,[rip+0xf1219] # 0xff700
            rdx = m.env.data_va(81613);
            // e4e7 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e4ea lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e4ee mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // e4f3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4f6 mov rdi,rbx
            rdi = rbx;
            // e4f9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4fc cmp DWORD PTR [rbp-0x14],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64, 0x0u64).1;
            // e500 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0xe503;
        }
        0xe503 => {
            // block e503
            #[cfg(feature = "trace")] eprintln!("f_0000e4b8 pc e503 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e503 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e507 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e508 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e50a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e50b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf3ae size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f3ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf3ae;
    loop {
        match pc {
        0xf3ae => {
            // block f3ae
            #[cfg(feature = "trace")] eprintln!("f_0000f3ae pc f3ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f3ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f3af mov rbp,rsp
            rbp = rsp;
            // f3b2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f3b4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f3b5 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f3b9 mov r14,rsi
            r14 = rsi;
            // f3bc mov rbx,rdi
            rbx = rdi;
            // f3bf mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f3c2 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f3c7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f3c9 je 0xf3f6
            if fl.e() { pc = 0xf3f6; continue; }
            pc = 0xf3cb;
        }
        0xf3cb => {
            // block f3cb
            #[cfg(feature = "trace")] eprintln!("f_0000f3ae pc f3cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f3cb movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f3cf mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f3d2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f3d5 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f3d9 mov esi,0x14a
            rsi = (0x14au64) as u32 as u64;
            // f3de mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // f3e3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // f3e6 mov rdi,rbx
            rdi = rbx;
            // f3e9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f3ec mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f3f0 mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f3f4 jmp 0xf3f8
            pc = 0xf3f8; continue;
        }
        0xf3f6 => {
            // block f3f6
            #[cfg(feature = "trace")] eprintln!("f_0000f3ae pc f3f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f3f6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf3f8;
        }
        0xf3f8 => {
            // block f3f8
            #[cfg(feature = "trace")] eprintln!("f_0000f3ae pc f3f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f3f8 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f3fc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f3fd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f3ff pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f400 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf4e4 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f4e4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf4e4;
    loop {
        match pc {
        0xf4e4 => {
            // block f4e4
            #[cfg(feature = "trace")] eprintln!("f_0000f4e4 pc f4e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f4e4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f4e5 mov rbp,rsp
            rbp = rsp;
            // f4e8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f4ea push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f4eb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f4ef mov r14,rsi
            r14 = rsi;
            // f4f2 mov rbx,rdi
            rbx = rdi;
            // f4f5 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f4f8 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f4fd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f4ff je 0xf52c
            if fl.e() { pc = 0xf52c; continue; }
            pc = 0xf501;
        }
        0xf501 => {
            // block f501
            #[cfg(feature = "trace")] eprintln!("f_0000f4e4 pc f501 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f501 movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f505 mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f508 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f50b lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f50f mov esi,0x14b
            rsi = (0x14bu64) as u32 as u64;
            // f514 mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // f519 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // f51c mov rdi,rbx
            rdi = rbx;
            // f51f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f522 mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f526 mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f52a jmp 0xf52e
            pc = 0xf52e; continue;
        }
        0xf52c => {
            // block f52c
            #[cfg(feature = "trace")] eprintln!("f_0000f4e4 pc f52c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f52c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf52e;
        }
        0xf52e => {
            // block f52e
            #[cfg(feature = "trace")] eprintln!("f_0000f4e4 pc f52e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f52e add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f532 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f533 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f535 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f536 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf61a size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f61a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf61a;
    loop {
        match pc {
        0xf61a => {
            // block f61a
            #[cfg(feature = "trace")] eprintln!("f_0000f61a pc f61a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f61a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f61b mov rbp,rsp
            rbp = rsp;
            // f61e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f620 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f621 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f625 mov r14,rsi
            r14 = rsi;
            // f628 mov rbx,rdi
            rbx = rdi;
            // f62b mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f62e call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f633 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f635 je 0xf662
            if fl.e() { pc = 0xf662; continue; }
            pc = 0xf637;
        }
        0xf637 => {
            // block f637
            #[cfg(feature = "trace")] eprintln!("f_0000f61a pc f637 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f637 movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f63b mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f63e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f641 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f645 mov esi,0x14c
            rsi = (0x14cu64) as u32 as u64;
            // f64a mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // f64f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // f652 mov rdi,rbx
            rdi = rbx;
            // f655 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f658 mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f65c mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f660 jmp 0xf664
            pc = 0xf664; continue;
        }
        0xf662 => {
            // block f662
            #[cfg(feature = "trace")] eprintln!("f_0000f61a pc f662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f662 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf664;
        }
        0xf664 => {
            // block f664
            #[cfg(feature = "trace")] eprintln!("f_0000f61a pc f664 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f664 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f668 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f669 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f66b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f66c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf750 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f750<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf750;
    loop {
        match pc {
        0xf750 => {
            // block f750
            #[cfg(feature = "trace")] eprintln!("f_0000f750 pc f750 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f750 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f751 mov rbp,rsp
            rbp = rsp;
            // f754 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f756 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f757 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f75b mov r14,rsi
            r14 = rsi;
            // f75e mov rbx,rdi
            rbx = rdi;
            // f761 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f764 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f769 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f76b je 0xf798
            if fl.e() { pc = 0xf798; continue; }
            pc = 0xf76d;
        }
        0xf76d => {
            // block f76d
            #[cfg(feature = "trace")] eprintln!("f_0000f750 pc f76d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f76d movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f771 mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f774 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f777 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f77b mov esi,0x14d
            rsi = (0x14du64) as u32 as u64;
            // f780 mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // f785 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // f788 mov rdi,rbx
            rdi = rbx;
            // f78b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f78e mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f792 mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f796 jmp 0xf79a
            pc = 0xf79a; continue;
        }
        0xf798 => {
            // block f798
            #[cfg(feature = "trace")] eprintln!("f_0000f750 pc f798 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f798 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf79a;
        }
        0xf79a => {
            // block f79a
            #[cfg(feature = "trace")] eprintln!("f_0000f750 pc f79a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f79a add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f79e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f79f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f7a1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f7a2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf860 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f860<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf860;
    loop {
        match pc {
        0xf860 => {
            // block f860
            #[cfg(feature = "trace")] eprintln!("f_0000f860 pc f860 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f860 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f861 mov rbp,rsp
            rbp = rsp;
            // f864 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f866 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f867 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f86b mov r14,rsi
            r14 = rsi;
            // f86e mov rbx,rdi
            rbx = rdi;
            // f871 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f874 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f879 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f87b je 0xf8a8
            if fl.e() { pc = 0xf8a8; continue; }
            pc = 0xf87d;
        }
        0xf87d => {
            // block f87d
            #[cfg(feature = "trace")] eprintln!("f_0000f860 pc f87d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f87d movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f881 mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f884 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f887 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f88b mov esi,0x14e
            rsi = (0x14eu64) as u32 as u64;
            // f890 mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // f895 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // f898 mov rdi,rbx
            rdi = rbx;
            // f89b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f89e mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f8a2 mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f8a6 jmp 0xf8aa
            pc = 0xf8aa; continue;
        }
        0xf8a8 => {
            // block f8a8
            #[cfg(feature = "trace")] eprintln!("f_0000f860 pc f8a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f8a8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf8aa;
        }
        0xf8aa => {
            // block f8aa
            #[cfg(feature = "trace")] eprintln!("f_0000f860 pc f8aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f8aa add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f8ae pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f8af pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f8b1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f8b2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf948 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f948<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf948;
    loop {
        match pc {
        0xf948 => {
            // block f948
            #[cfg(feature = "trace")] eprintln!("f_0000f948 pc f948 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f948 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f949 mov rbp,rsp
            rbp = rsp;
            // f94c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f94e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f94f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f953 mov r14,rsi
            r14 = rsi;
            // f956 mov rbx,rdi
            rbx = rdi;
            // f959 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f95c call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f961 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f963 je 0xf990
            if fl.e() { pc = 0xf990; continue; }
            pc = 0xf965;
        }
        0xf965 => {
            // block f965
            #[cfg(feature = "trace")] eprintln!("f_0000f948 pc f965 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f965 movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f969 mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f96c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f96f lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f973 mov esi,0x14f
            rsi = (0x14fu64) as u32 as u64;
            // f978 mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // f97d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // f980 mov rdi,rbx
            rdi = rbx;
            // f983 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f986 mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f98a mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f98e jmp 0xf992
            pc = 0xf992; continue;
        }
        0xf990 => {
            // block f990
            #[cfg(feature = "trace")] eprintln!("f_0000f948 pc f990 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f990 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf992;
        }
        0xf992 => {
            // block f992
            #[cfg(feature = "trace")] eprintln!("f_0000f948 pc f992 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f992 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f996 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f997 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f999 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f99a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfad6 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fad6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xfad6;
    loop {
        match pc {
        0xfad6 => {
            // block fad6
            #[cfg(feature = "trace")] eprintln!("f_0000fad6 pc fad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fad6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fad7 mov rbp,rsp
            rbp = rsp;
            // fada push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // fadc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fadd sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // fae1 mov r14,rsi
            r14 = rsi;
            // fae4 mov rbx,rdi
            rbx = rdi;
            // fae7 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // faea call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // faef test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // faf1 je 0xfb1f
            if fl.e() { pc = 0xfb1f; continue; }
            pc = 0xfaf3;
        }
        0xfaf3 => {
            // block faf3
            #[cfg(feature = "trace")] eprintln!("f_0000fad6 pc faf3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // faf3 movzx eax,BYTE PTR [r14]
            rax = (m.r8(r14) as u64) as u32 as u64;
            // faf7 mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // fafa mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fafd lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // fb01 mov esi,0x150
            rsi = (0x150u64) as u32 as u64;
            // fb06 mov ecx,0xff0000
            rcx = (0xff0000u64) as u32 as u64;
            // fb0b mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // fb11 mov rdi,rbx
            rdi = rbx;
            // fb14 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fb17 mov cl,BYTE PTR [rbp-0x14]
            rcx = (rcx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u8 as u64);
            // fb1a mov BYTE PTR [r14],cl
            m.w8(r14, ((rcx as u8 as u64)) as u8);
            // fb1d jmp 0xfb21
            pc = 0xfb21; continue;
        }
        0xfb1f => {
            // block fb1f
            #[cfg(feature = "trace")] eprintln!("f_0000fad6 pc fb1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fb1f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xfb21;
        }
        0xfb21 => {
            // block fb21
            #[cfg(feature = "trace")] eprintln!("f_0000fad6 pc fb21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fb21 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // fb25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fb26 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // fb28 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fb29 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfb6a size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fb6a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xfb6a;
    loop {
        match pc {
        0xfb6a => {
            // block fb6a
            #[cfg(feature = "trace")] eprintln!("f_0000fb6a pc fb6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fb6a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fb6b mov rbp,rsp
            rbp = rsp;
            // fb6e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // fb70 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fb71 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // fb75 mov r14,rsi
            r14 = rsi;
            // fb78 mov rbx,rdi
            rbx = rdi;
            // fb7b mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // fb7e call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fb83 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // fb85 je 0xfbb3
            if fl.e() { pc = 0xfbb3; continue; }
            pc = 0xfb87;
        }
        0xfb87 => {
            // block fb87
            #[cfg(feature = "trace")] eprintln!("f_0000fb6a pc fb87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fb87 movzx eax,BYTE PTR [r14]
            rax = (m.r8(r14) as u64) as u32 as u64;
            // fb8b mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // fb8e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fb91 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // fb95 mov esi,0x150
            rsi = (0x150u64) as u32 as u64;
            // fb9a mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // fb9f mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // fba5 mov rdi,rbx
            rdi = rbx;
            // fba8 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fbab mov cl,BYTE PTR [rbp-0x14]
            rcx = (rcx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u8 as u64);
            // fbae mov BYTE PTR [r14],cl
            m.w8(r14, ((rcx as u8 as u64)) as u8);
            // fbb1 jmp 0xfbb5
            pc = 0xfbb5; continue;
        }
        0xfbb3 => {
            // block fbb3
            #[cfg(feature = "trace")] eprintln!("f_0000fb6a pc fbb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fbb3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xfbb5;
        }
        0xfbb5 => {
            // block fbb5
            #[cfg(feature = "trace")] eprintln!("f_0000fb6a pc fbb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fbb5 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // fbb9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fbba pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // fbbc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fbbd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28c9e size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028c9e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28c9e;
    loop {
        match pc {
        0x28c9e => {
            // block 28c9e
            #[cfg(feature = "trace")] eprintln!("f_00028c9e pc 28c9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28c9e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28c9f mov rbp,rsp
            rbp = rsp;
            // 28ca2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28ca3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 28ca4 mov rbx,rdi
            rbx = rdi;
            // 28ca7 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 28caa mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 28cb2 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 28cba mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 28cc2 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 28cca mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 28cd1 je 0x28ceb
            if fl.e() { pc = 0x28ceb; continue; }
            pc = 0x28cd3;
        }
        0x28cd3 => {
            // block 28cd3
            #[cfg(feature = "trace")] eprintln!("f_00028c9e pc 28cd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28cd3 mov rdi,rbx
            rdi = rbx;
            // 28cd6 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28cdb mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 28cde test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 28ce1 je 0x28ceb
            if fl.e() { pc = 0x28ceb; continue; }
            pc = 0x28ce3;
        }
        0x28ce3 => {
            // block 28ce3
            #[cfg(feature = "trace")] eprintln!("f_00028c9e pc 28ce3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28ce3 mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28ce6 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x28ceb;
        }
        0x28ceb => {
            // block 28ceb
            #[cfg(feature = "trace")] eprintln!("f_00028c9e pc 28ceb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28ceb add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 28cef pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28cf0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28cf1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28fda size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028fda<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28fda;
    loop {
        match pc {
        0x28fda => {
            // block 28fda
            #[cfg(feature = "trace")] eprintln!("f_00028fda pc 28fda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28fda push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28fdb mov rbp,rsp
            rbp = rsp;
            // 28fde push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28fe0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28fe2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28fe3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 28fe4 mov rbx,rsi
            rbx = rsi;
            // 28fe7 mov r14,rdi
            r14 = rdi;
            // 28fea test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 28fec je 0x29011
            if fl.e() { pc = 0x29011; continue; }
            pc = 0x28fee;
        }
        0x28fee => {
            // block 28fee
            #[cfg(feature = "trace")] eprintln!("f_00028fda pc 28fee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28fee test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 28ff1 je 0x29021
            if fl.e() { pc = 0x29021; continue; }
            pc = 0x28ff3;
        }
        0x28ff3 => {
            // block 28ff3
            #[cfg(feature = "trace")] eprintln!("f_00028fda pc 28ff3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28ff3 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 28ff6 mov rdi,r14
            rdi = r14;
            // 28ff9 mov rsi,r15
            rsi = r15;
            // 28ffc call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29001 mov rdi,QWORD PTR [r14]
            rdi = m.r64(r14) as u64;
            // 29004 mov rsi,rbx
            rsi = rbx;
            // 29007 mov rdx,r15
            rdx = r15;
            // 2900a call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2900f jmp 0x2901d
            pc = 0x2901d; continue;
        }
        0x29011 => {
            // block 29011
            #[cfg(feature = "trace")] eprintln!("f_00028fda pc 29011 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29011 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 29013 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 29015 mov rdi,r14
            rdi = r14;
            // 29018 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2901d;
        }
        0x2901d => {
            // block 2901d
            #[cfg(feature = "trace")] eprintln!("f_00028fda pc 2901d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2901d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2901f jmp 0x29023
            pc = 0x29023; continue;
        }
        0x29021 => {
            // block 29021
            #[cfg(feature = "trace")] eprintln!("f_00028fda pc 29021 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29021 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x29023;
        }
        0x29023 => {
            // block 29023
            #[cfg(feature = "trace")] eprintln!("f_00028fda pc 29023 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29023 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 29027 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29028 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2902a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2902c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2902d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43390 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043390<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43390;
    loop {
        match pc {
        0x43390 => {
            // block 43390
            #[cfg(feature = "trace")] eprintln!("f_00043390 pc 43390 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43390 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43391 mov rbp,rsp
            rbp = rsp;
            // 43394 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43396 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43397 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4339b mov r14,rsi
            r14 = rsi;
            // 4339e mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 433a5 mov DWORD PTR [r14],0x2
            m.w32(r14, (0x2u64) as u32);
            // 433ac mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 433af lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 433b3 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 433b5 mov esi,0x7e
            rsi = (0x7eu64) as u32 as u64;
            // 433ba mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 433bf xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 433c2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 433c5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 433c7 je 0x433d9
            if fl.e() { pc = 0x433d9; continue; }
            pc = 0x433c9;
        }
        0x433c9 => {
            // block 433c9
            #[cfg(feature = "trace")] eprintln!("f_00043390 pc 433c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 433c9 mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 433cc shr eax,0xc
            { let (r, f) = shr::<32>((rax as u32 as u64), 0xcu64, fl); rax = (r) as u32 as u64; fl = f; }
            // 433cf not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // 433d1 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 433d4 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 433d7 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x433d9;
        }
        0x433d9 => {
            // block 433d9
            #[cfg(feature = "trace")] eprintln!("f_00043390 pc 433d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 433d9 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 433db add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 433df pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 433e0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 433e2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 433e3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43b62 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043b62<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43b62;
    loop {
        match pc {
        0x43b62 => {
            // block 43b62
            #[cfg(feature = "trace")] eprintln!("f_00043b62 pc 43b62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43b62 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43b63 mov rbp,rsp
            rbp = rsp;
            // 43b66 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43b68 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43b6a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43b6b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43b6c mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 43b6f mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 43b71 mov r15,rdi
            r15 = rdi;
            // 43b74 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43b78 call 0xcbb7b
            { let r = { let t = m.env.code_va(0xcbb7b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43b7d mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 43b7f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43b81 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 43b84 jg 0x43bab
            if fl.g() { pc = 0x43bab; continue; }
            pc = 0x43b86;
        }
        0x43b86 => {
            // block 43b86
            #[cfg(feature = "trace")] eprintln!("f_00043b62 pc 43b86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43b86 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 43b89 jne 0x43bab
            if fl.ne() { pc = 0x43bab; continue; }
            pc = 0x43b8b;
        }
        0x43b8b => {
            // block 43b8b
            #[cfg(feature = "trace")] eprintln!("f_00043b62 pc 43b8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43b8b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43b8e mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 43b90 lea rdx,[rip+0xbca89] # 0x100620
            rdx = m.env.data_va(85485);
            // 43b97 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43b9a mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // 43b9f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 43ba2 mov rdi,r15
            rdi = r15;
            // 43ba5 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 43ba8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43bab;
        }
        0x43bab => {
            // block 43bab
            #[cfg(feature = "trace")] eprintln!("f_00043b62 pc 43bab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43bab add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43baf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43bb0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43bb2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43bb4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43bb5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43bb6 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043bb6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43bb6;
    loop {
        match pc {
        0x43bb6 => {
            // block 43bb6
            #[cfg(feature = "trace")] eprintln!("f_00043bb6 pc 43bb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43bb6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43bb7 mov rbp,rsp
            rbp = rsp;
            // 43bba push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43bbc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43bbe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43bbf push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43bc0 mov r14,rdx
            r14 = rdx;
            // 43bc3 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 43bc5 mov r15,rdi
            r15 = rdi;
            // 43bc8 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43bcc call 0xcbb7b
            { let r = { let t = m.env.code_va(0xcbb7b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43bd1 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 43bd3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43bd5 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 43bd8 jg 0x43bff
            if fl.g() { pc = 0x43bff; continue; }
            pc = 0x43bda;
        }
        0x43bda => {
            // block 43bda
            #[cfg(feature = "trace")] eprintln!("f_00043bb6 pc 43bda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43bda xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 43bdd jne 0x43bff
            if fl.ne() { pc = 0x43bff; continue; }
            pc = 0x43bdf;
        }
        0x43bdf => {
            // block 43bdf
            #[cfg(feature = "trace")] eprintln!("f_00043bb6 pc 43bdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43bdf mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43be2 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 43be4 lea rdx,[rip+0xbca35] # 0x100620
            rdx = m.env.data_va(85485);
            // 43beb mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43bee mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // 43bf3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 43bf6 mov rdi,r15
            rdi = r15;
            // 43bf9 mov rdx,r14
            rdx = r14;
            // 43bfc call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43bff;
        }
        0x43bff => {
            // block 43bff
            #[cfg(feature = "trace")] eprintln!("f_00043bb6 pc 43bff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43bff add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43c03 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43c04 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43c06 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43c08 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43c09 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x46ba0 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00046ba0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x46ba0;
    loop {
        match pc {
        0x46ba0 => {
            // block 46ba0
            #[cfg(feature = "trace")] eprintln!("f_00046ba0 pc 46ba0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46ba0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 46ba1 mov rbp,rsp
            rbp = rsp;
            // 46ba4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 46ba5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 46ba6 mov rbx,rdi
            rbx = rdi;
            // 46ba9 lea rdx,[rbp-0x10]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // 46bad mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 46bb2 call 0x46bf4
            { let r = { let t = m.env.code_va(0x46bf4); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 46bb7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 46bb9 je 0x46bca
            if fl.e() { pc = 0x46bca; continue; }
            pc = 0x46bbb;
        }
        0x46bbb => {
            // block 46bbb
            #[cfg(feature = "trace")] eprintln!("f_00046ba0 pc 46bbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46bbb mov rax,QWORD PTR [rbp-0x10]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64;
            // 46bbf mov QWORD PTR [rbx+0x210],rax
            m.w64(rbx.wrapping_add(0x210), (rax) as u64);
            // 46bc6 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 46bc8 jmp 0x46bed
            pc = 0x46bed; continue;
        }
        0x46bca => {
            // block 46bca
            #[cfg(feature = "trace")] eprintln!("f_00046ba0 pc 46bca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46bca mov QWORD PTR [rbx+0x220],0x0
            m.w64(rbx.wrapping_add(0x220), (0x0u64) as u64);
            // 46bd5 mov QWORD PTR [rbx+0x218],0x0
            m.w64(rbx.wrapping_add(0x218), (0x0u64) as u64);
            // 46be0 mov QWORD PTR [rbx+0x210],0x0
            m.w64(rbx.wrapping_add(0x210), (0x0u64) as u64);
            // 46beb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x46bed;
        }
        0x46bed => {
            // block 46bed
            #[cfg(feature = "trace")] eprintln!("f_00046ba0 pc 46bed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 46bed add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 46bf1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 46bf2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 46bf3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbb044 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bb044<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbb044;
    loop {
        match pc {
        0xbb044 => {
            // block bb044
            #[cfg(feature = "trace")] eprintln!("f_000bb044 pc bb044 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb044 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bb045 mov rbp,rsp
            rbp = rsp;
            // bb048 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bb049 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bb04a mov rbx,rdi
            rbx = rdi;
            // bb04d mov DWORD PTR [rbp-0xc],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rsi as u32 as u64)) as u32);
            // bb050 lea rdi,[rip+0x10d5b9] # 0x1c8610
            rdi = m.env.data_va(904669);
            // bb057 lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // bb05b call 0xebe2e
            { let r = { let t = m.env.stub_va(0x5fb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bb060 lea rcx,[rip+0x10d5b1] # 0x1c8618
            rcx = m.env.data_va(904677);
            // bb067 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bb06a je 0xbb07d
            if fl.e() { pc = 0xbb07d; continue; }
            pc = 0xbb06c;
        }
        0xbb06c => {
            // block bb06c
            #[cfg(feature = "trace")] eprintln!("f_000bb044 pc bb06c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb06c add rax,0x28
            { let (r, f) = add::<64>(rax, 0x28u64);
              rax = r;
              fl = f; }
            // bb070 mov rdi,rbx
            rdi = rbx;
            // bb073 mov rsi,rax
            rsi = rax;
            // bb076 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bb07b jmp 0xbb08e
            pc = 0xbb08e; continue;
        }
        0xbb07d => {
            // block bb07d
            #[cfg(feature = "trace")] eprintln!("f_000bb044 pc bb07d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb07d lea rsi,[rip+0x7dc44] # 0x138cc8
            rsi = m.env.data_va(316565);
            // bb084 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bb086 mov rdi,rbx
            rdi = rbx;
            // bb089 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbb08e;
        }
        0xbb08e => {
            // block bb08e
            #[cfg(feature = "trace")] eprintln!("f_000bb044 pc bb08e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb08e mov rax,rbx
            rax = rbx;
            // bb091 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bb095 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bb096 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bb097 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbb098 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bb098<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbb098;
    loop {
        match pc {
        0xbb098 => {
            // block bb098
            #[cfg(feature = "trace")] eprintln!("f_000bb098 pc bb098 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb098 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bb099 mov rbp,rsp
            rbp = rsp;
            // bb09c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bb09d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bb09e mov rbx,rdi
            rbx = rdi;
            // bb0a1 mov DWORD PTR [rbp-0xc],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rsi as u32 as u64)) as u32);
            // bb0a4 lea rdi,[rip+0x10d5ad] # 0x1c8658
            rdi = m.env.data_va(904741);
            // bb0ab lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // bb0af call 0xebe52
            { let r = { let t = m.env.stub_va(0x61f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bb0b4 lea rcx,[rip+0x10d5a5] # 0x1c8660
            rcx = m.env.data_va(904749);
            // bb0bb cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bb0be je 0xbb0d1
            if fl.e() { pc = 0xbb0d1; continue; }
            pc = 0xbb0c0;
        }
        0xbb0c0 => {
            // block bb0c0
            #[cfg(feature = "trace")] eprintln!("f_000bb098 pc bb0c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb0c0 add rax,0x28
            { let (r, f) = add::<64>(rax, 0x28u64);
              rax = r;
              fl = f; }
            // bb0c4 mov rdi,rbx
            rdi = rbx;
            // bb0c7 mov rsi,rax
            rsi = rax;
            // bb0ca call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bb0cf jmp 0xbb0e2
            pc = 0xbb0e2; continue;
        }
        0xbb0d1 => {
            // block bb0d1
            #[cfg(feature = "trace")] eprintln!("f_000bb098 pc bb0d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb0d1 lea rsi,[rip+0x7dbf0] # 0x138cc8
            rsi = m.env.data_va(316565);
            // bb0d8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bb0da mov rdi,rbx
            rdi = rbx;
            // bb0dd call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbb0e2;
        }
        0xbb0e2 => {
            // block bb0e2
            #[cfg(feature = "trace")] eprintln!("f_000bb098 pc bb0e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bb0e2 mov rax,rbx
            rax = rbx;
            // bb0e5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bb0e9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bb0ea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bb0eb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2f90 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2f90<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2f90;
    loop {
        match pc {
        0xe2f90 => {
            // block e2f90
            #[cfg(feature = "trace")] eprintln!("f_000e2f90 pc e2f90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2f90 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2f91 mov rbp,rsp
            rbp = rsp;
            // e2f94 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2f96 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2f97 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2f9b mov r14,rcx
            r14 = rcx;
            // e2f9e mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e2fa0 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e2fa3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2fa6 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // e2faa test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e2fac je 0xe2fb9
            if fl.e() { pc = 0xe2fb9; continue; }
            pc = 0xe2fae;
        }
        0xe2fae => {
            // block e2fae
            #[cfg(feature = "trace")] eprintln!("f_000e2f90 pc e2fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2fae lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e2fb2 mov esi,0x4001c
            rsi = (0x4001cu64) as u32 as u64;
            // e2fb7 jmp 0xe2fc2
            pc = 0xe2fc2; continue;
        }
        0xe2fb9 => {
            // block e2fb9
            #[cfg(feature = "trace")] eprintln!("f_000e2f90 pc e2fb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2fb9 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e2fbd mov esi,0x4001b
            rsi = (0x4001bu64) as u32 as u64;
            pc = 0xe2fc2;
        }
        0xe2fc2 => {
            // block e2fc2
            #[cfg(feature = "trace")] eprintln!("f_000e2f90 pc e2fc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2fc2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2fc7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2fca call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2fcc shl ebx,0x3
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x3u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e2fcf mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // e2fd2 mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2fd4 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e2fd6 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e2fd9 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e2fdb add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2fdf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2fe0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2fe2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2fe3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe54b0 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e54b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe54b0;
    loop {
        match pc {
        0xe54b0 => {
            // block e54b0
            #[cfg(feature = "trace")] eprintln!("f_000e54b0 pc e54b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e54b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e54b1 mov rbp,rsp
            rbp = rsp;
            // e54b4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e54b5 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e54b9 mov rbx,rdi
            rbx = rdi;
            // e54bc test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e54be je 0xe54c5
            if fl.e() { pc = 0xe54c5; continue; }
            pc = 0xe54c0;
        }
        0xe54c0 => {
            // block e54c0
            #[cfg(feature = "trace")] eprintln!("f_000e54b0 pc e54c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e54c0 mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // e54c3 jmp 0xe54c8
            pc = 0xe54c8; continue;
        }
        0xe54c5 => {
            // block e54c5
            #[cfg(feature = "trace")] eprintln!("f_000e54b0 pc e54c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e54c5 mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe54c8;
        }
        0xe54c8 => {
            // block e54c8
            #[cfg(feature = "trace")] eprintln!("f_000e54b0 pc e54c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e54c8 lea rax,[rbp-0x18]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // e54cc mov QWORD PTR [rsp],rax
            m.w64(rsp, (rax) as u64);
            // e54d0 lea rcx,[rbp-0xc]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // e54d4 lea r8,[rbp-0x10]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // e54d8 lea r9,[rbp-0x14]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e54dc mov rdi,rbx
            rdi = rbx;
            // e54df call 0xe5874
            { let r = { let t = m.env.code_va(0xe5874); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e54e4 mov esi,DWORD PTR [rbp-0x18]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // e54e7 mov edx,DWORD PTR [rbp-0x14]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // e54ea mov ecx,DWORD PTR [rbp-0x10]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64) as u32 as u64;
            // e54ed mov r8d,DWORD PTR [rbp-0xc]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // e54f1 mov r9d,DWORD PTR [rbx+0x8]
            r9 = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // e54f5 mov rdi,rbx
            rdi = rbx;
            // e54f8 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e54fc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e54fd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e54fe jmp 0xe4cd0
            { let r = { let t = m.env.code_va(0xe4cd0); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x17b9c size=85
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00017b9c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x17b9c;
    loop {
        match pc {
        0x17b9c => {
            // block 17b9c
            #[cfg(feature = "trace")] eprintln!("f_00017b9c pc 17b9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17b9c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 17b9d mov rbp,rsp
            rbp = rsp;
            // 17ba0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 17ba2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 17ba4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 17ba6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 17ba7 mov r15,rsi
            r15 = rsi;
            // 17baa mov r14,rdi
            r14 = rdi;
            // 17bad mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 17bb0 cmp rbx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 17bb4 je 0x17be5
            if fl.e() { pc = 0x17be5; continue; }
            pc = 0x17bb6;
        }
        0x17bb6 => {
            // block 17bb6
            #[cfg(feature = "trace")] eprintln!("f_00017b9c pc 17bb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17bb6 lea r12,[rip+0x1126d7] # 0x12a294
            r12 = m.env.data_va(256609);
            pc = 0x17bbd;
        }
        0x17bbd => {
            // block 17bbd
            #[cfg(feature = "trace")] eprintln!("f_00017b9c pc 17bbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17bbd mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 17bc2 mov rdi,r14
            rdi = r14;
            // 17bc5 mov rsi,r12
            rsi = r12;
            // 17bc8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17bcd mov rdi,rax
            rdi = rax;
            // 17bd0 mov rsi,rbx
            rsi = rbx;
            // 17bd3 call 0x17bf1
            { let r = { let t = m.env.code_va(0x17bf1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17bd8 add rbx,0xf8
            { let (r, f) = add::<64>(rbx, 0xf8u64);
              rbx = r;
              fl = f; }
            // 17bdf cmp rbx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 17be3 jne 0x17bbd
            if fl.ne() { pc = 0x17bbd; continue; }
            pc = 0x17be5;
        }
        0x17be5 => {
            // block 17be5
            #[cfg(feature = "trace")] eprintln!("f_00017b9c pc 17be5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17be5 mov rax,r14
            rax = r14;
            // 17be8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 17be9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 17beb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 17bed pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 17bef pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17bf0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a48 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001a48<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a48;
    loop {
        match pc {
        0x1a48 => {
            // block 1a48
            #[cfg(feature = "trace")] eprintln!("f_00001a48 pc 1a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a48 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a49 mov rbp,rsp
            rbp = rsp;
            // 1a4c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a4d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1a4e mov rbx,rsi
            rbx = rsi;
            // 1a51 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 1a58 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1a5b mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // 1a5f cmp edx,0x3
            fl = sub::<32>((rdx as u32 as u64), 0x3u64).1;
            // 1a62 jg 0x1a7a
            if fl.g() { pc = 0x1a7a; continue; }
            pc = 0x1a64;
        }
        0x1a64 => {
            // block 1a64
            #[cfg(feature = "trace")] eprintln!("f_00001a48 pc 1a64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a64 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 1a68 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1a6d mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 1a72 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 1a78 jmp 0x1a8e
            pc = 0x1a8e; continue;
        }
        0x1a7a => {
            // block 1a7a
            #[cfg(feature = "trace")] eprintln!("f_00001a48 pc 1a7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a7a lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 1a7e mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1a83 mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // 1a88 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            pc = 0x1a8e;
        }
        0x1a8e => {
            // block 1a8e
            #[cfg(feature = "trace")] eprintln!("f_00001a48 pc 1a8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a8e call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a90 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 1a94 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            // 1a97 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1a9b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a9c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a9d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5658 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005658<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5658;
    loop {
        match pc {
        0x5658 => {
            // block 5658
            #[cfg(feature = "trace")] eprintln!("f_00005658 pc 5658 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5658 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5659 mov rbp,rsp
            rbp = rsp;
            // 565c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 565e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 565f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5663 mov r14,rdx
            r14 = rdx;
            // 5666 mov rbx,rsi
            rbx = rsi;
            // 5669 mov DWORD PTR [rbp-0x14],0x3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x3u64) as u32);
            // 5670 mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // 5674 mov BYTE PTR [rbx],0x0
            m.w8(rbx, (0x0u64) as u8);
            // 5677 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 567a lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 567e mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 5680 call QWORD PTR [rax+0x5d0]
            { let r = { let t = m.r64(rax.wrapping_add(0x5d0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5686 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5688 je 0x56a3
            if fl.e() { pc = 0x56a3; continue; }
            pc = 0x568a;
        }
        0x568a => {
            // block 568a
            #[cfg(feature = "trace")] eprintln!("f_00005658 pc 568a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 568a mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 568d cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 5690 ja 0x56a3
            if fl.a() { pc = 0x56a3; continue; }
            pc = 0x5692;
        }
        0x5692 => {
            // block 5692
            #[cfg(feature = "trace")] eprintln!("f_00005658 pc 5692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5692 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 5695 sete BYTE PTR [rbx]
            m.w8(rbx, ((fl.e() as u64)) as u8);
            // 5698 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 569b sete BYTE PTR [r14]
            m.w8(r14, ((fl.e() as u64)) as u8);
            // 569f mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 56a1 jmp 0x56a5
            pc = 0x56a5; continue;
        }
        0x56a3 => {
            // block 56a3
            #[cfg(feature = "trace")] eprintln!("f_00005658 pc 56a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56a3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x56a5;
        }
        0x56a5 => {
            // block 56a5
            #[cfg(feature = "trace")] eprintln!("f_00005658 pc 56a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 56a5 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 56a9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 56aa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 56ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 56ad ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc85e size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c85e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc85e;
    loop {
        match pc {
        0xc85e => {
            // block c85e
            #[cfg(feature = "trace")] eprintln!("f_0000c85e pc c85e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c85e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c85f mov rbp,rsp
            rbp = rsp;
            // c862 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c864 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c865 mov r14,rsi
            r14 = rsi;
            // c868 mov rbx,rdi
            rbx = rdi;
            // c86b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c86e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c874 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c876 call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c87b test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c87d je 0xc8a6
            if fl.e() { pc = 0xc8a6; continue; }
            pc = 0xc87f;
        }
        0xc87f => {
            // block c87f
            #[cfg(feature = "trace")] eprintln!("f_0000c85e pc c87f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c87f mov r9,QWORD PTR [rbx]
            r9 = m.r64(rbx) as u64;
            // c882 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // c885 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // c888 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // c88b lea ecx,[rax*8+0x7]
            rcx = (rax.wrapping_mul(8).wrapping_add(0x7)) as u32 as u64;
            // c892 mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // c897 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c89a mov rdi,rbx
            rdi = rbx;
            // c89d mov rdx,r14
            rdx = r14;
            // c8a0 call QWORD PTR [r9+0x38]
            { let r = { let t = m.r64(r9.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c8a4 jmp 0xc8af
            pc = 0xc8af; continue;
        }
        0xc8a6 => {
            // block c8a6
            #[cfg(feature = "trace")] eprintln!("f_0000c85e pc c8a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8a6 mov DWORD PTR [r14],0xc
            m.w32(r14, (0xcu64) as u32);
            // c8ad xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc8af;
        }
        0xc8af => {
            // block c8af
            #[cfg(feature = "trace")] eprintln!("f_0000c85e pc c8af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8af pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c8b0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8b2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8b3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf44a size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f44a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf44a;
    loop {
        match pc {
        0xf44a => {
            // block f44a
            #[cfg(feature = "trace")] eprintln!("f_0000f44a pc f44a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f44a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f44b mov rbp,rsp
            rbp = rsp;
            // f44e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f450 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f451 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f455 mov r14,rsi
            r14 = rsi;
            // f458 mov rbx,rdi
            rbx = rdi;
            // f45b mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f45e call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f463 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f465 je 0xf495
            if fl.e() { pc = 0xf495; continue; }
            pc = 0xf467;
        }
        0xf467 => {
            // block f467
            #[cfg(feature = "trace")] eprintln!("f_0000f44a pc f467 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f467 movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f46b mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f46e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f471 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f475 mov esi,0x14a
            rsi = (0x14au64) as u32 as u64;
            // f47a mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // f47f mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // f485 mov rdi,rbx
            rdi = rbx;
            // f488 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f48b mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f48f mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f493 jmp 0xf497
            pc = 0xf497; continue;
        }
        0xf495 => {
            // block f495
            #[cfg(feature = "trace")] eprintln!("f_0000f44a pc f495 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f495 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf497;
        }
        0xf497 => {
            // block f497
            #[cfg(feature = "trace")] eprintln!("f_0000f44a pc f497 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f497 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f49b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f49c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f49e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f49f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf580 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f580<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf580;
    loop {
        match pc {
        0xf580 => {
            // block f580
            #[cfg(feature = "trace")] eprintln!("f_0000f580 pc f580 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f580 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f581 mov rbp,rsp
            rbp = rsp;
            // f584 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f586 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f587 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f58b mov r14,rsi
            r14 = rsi;
            // f58e mov rbx,rdi
            rbx = rdi;
            // f591 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f594 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f599 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f59b je 0xf5cb
            if fl.e() { pc = 0xf5cb; continue; }
            pc = 0xf59d;
        }
        0xf59d => {
            // block f59d
            #[cfg(feature = "trace")] eprintln!("f_0000f580 pc f59d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f59d movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f5a1 mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f5a4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f5a7 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f5ab mov esi,0x14b
            rsi = (0x14bu64) as u32 as u64;
            // f5b0 mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // f5b5 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // f5bb mov rdi,rbx
            rdi = rbx;
            // f5be call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f5c1 mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f5c5 mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f5c9 jmp 0xf5cd
            pc = 0xf5cd; continue;
        }
        0xf5cb => {
            // block f5cb
            #[cfg(feature = "trace")] eprintln!("f_0000f580 pc f5cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f5cb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf5cd;
        }
        0xf5cd => {
            // block f5cd
            #[cfg(feature = "trace")] eprintln!("f_0000f580 pc f5cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f5cd add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f5d1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f5d2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f5d4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f5d5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf6b6 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f6b6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf6b6;
    loop {
        match pc {
        0xf6b6 => {
            // block f6b6
            #[cfg(feature = "trace")] eprintln!("f_0000f6b6 pc f6b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f6b6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f6b7 mov rbp,rsp
            rbp = rsp;
            // f6ba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f6bc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f6bd sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f6c1 mov r14,rsi
            r14 = rsi;
            // f6c4 mov rbx,rdi
            rbx = rdi;
            // f6c7 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f6ca call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f6cf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f6d1 je 0xf701
            if fl.e() { pc = 0xf701; continue; }
            pc = 0xf6d3;
        }
        0xf6d3 => {
            // block f6d3
            #[cfg(feature = "trace")] eprintln!("f_0000f6b6 pc f6d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f6d3 movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f6d7 mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f6da mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f6dd lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f6e1 mov esi,0x14c
            rsi = (0x14cu64) as u32 as u64;
            // f6e6 mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // f6eb mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // f6f1 mov rdi,rbx
            rdi = rbx;
            // f6f4 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f6f7 mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f6fb mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f6ff jmp 0xf703
            pc = 0xf703; continue;
        }
        0xf701 => {
            // block f701
            #[cfg(feature = "trace")] eprintln!("f_0000f6b6 pc f701 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f701 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf703;
        }
        0xf703 => {
            // block f703
            #[cfg(feature = "trace")] eprintln!("f_0000f6b6 pc f703 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f703 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f707 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f708 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f70a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f70b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf7ec size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f7ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf7ec;
    loop {
        match pc {
        0xf7ec => {
            // block f7ec
            #[cfg(feature = "trace")] eprintln!("f_0000f7ec pc f7ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f7ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f7ed mov rbp,rsp
            rbp = rsp;
            // f7f0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f7f2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f7f3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f7f7 mov r14,rsi
            r14 = rsi;
            // f7fa mov rbx,rdi
            rbx = rdi;
            // f7fd mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f800 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f805 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f807 je 0xf837
            if fl.e() { pc = 0xf837; continue; }
            pc = 0xf809;
        }
        0xf809 => {
            // block f809
            #[cfg(feature = "trace")] eprintln!("f_0000f7ec pc f809 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f809 movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f80d mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f810 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f813 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f817 mov esi,0x14d
            rsi = (0x14du64) as u32 as u64;
            // f81c mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // f821 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // f827 mov rdi,rbx
            rdi = rbx;
            // f82a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f82d mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f831 mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f835 jmp 0xf839
            pc = 0xf839; continue;
        }
        0xf837 => {
            // block f837
            #[cfg(feature = "trace")] eprintln!("f_0000f7ec pc f837 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f837 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf839;
        }
        0xf839 => {
            // block f839
            #[cfg(feature = "trace")] eprintln!("f_0000f7ec pc f839 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f839 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f83d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f83e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f840 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f841 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf8d4 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f8d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf8d4;
    loop {
        match pc {
        0xf8d4 => {
            // block f8d4
            #[cfg(feature = "trace")] eprintln!("f_0000f8d4 pc f8d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f8d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f8d5 mov rbp,rsp
            rbp = rsp;
            // f8d8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f8da push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f8db sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f8df mov r14,rsi
            r14 = rsi;
            // f8e2 mov rbx,rdi
            rbx = rdi;
            // f8e5 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f8e8 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f8ed test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f8ef je 0xf91f
            if fl.e() { pc = 0xf91f; continue; }
            pc = 0xf8f1;
        }
        0xf8f1 => {
            // block f8f1
            #[cfg(feature = "trace")] eprintln!("f_0000f8d4 pc f8f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f8f1 movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f8f5 mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f8f8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f8fb lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f8ff mov esi,0x14e
            rsi = (0x14eu64) as u32 as u64;
            // f904 mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // f909 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // f90f mov rdi,rbx
            rdi = rbx;
            // f912 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f915 mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // f919 mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // f91d jmp 0xf921
            pc = 0xf921; continue;
        }
        0xf91f => {
            // block f91f
            #[cfg(feature = "trace")] eprintln!("f_0000f8d4 pc f91f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f91f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xf921;
        }
        0xf921 => {
            // block f921
            #[cfg(feature = "trace")] eprintln!("f_0000f8d4 pc f921 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f921 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f925 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f926 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f928 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f929 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf9bc size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f9bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf9bc;
    loop {
        match pc {
        0xf9bc => {
            // block f9bc
            #[cfg(feature = "trace")] eprintln!("f_0000f9bc pc f9bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f9bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f9bd mov rbp,rsp
            rbp = rsp;
            // f9c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f9c2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f9c3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // f9c7 mov r14,rsi
            r14 = rsi;
            // f9ca mov rbx,rdi
            rbx = rdi;
            // f9cd mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // f9d0 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f9d5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f9d7 je 0xfa07
            if fl.e() { pc = 0xfa07; continue; }
            pc = 0xf9d9;
        }
        0xf9d9 => {
            // block f9d9
            #[cfg(feature = "trace")] eprintln!("f_0000f9bc pc f9d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f9d9 movzx eax,WORD PTR [r14]
            rax = (m.r16(r14) as u64) as u32 as u64;
            // f9dd mov DWORD PTR [rbp-0x14],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rax as u32 as u64)) as u32);
            // f9e0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f9e3 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // f9e7 mov esi,0x14f
            rsi = (0x14fu64) as u32 as u64;
            // f9ec mov ecx,0xffff0000
            rcx = (0xffff0000u64) as u32 as u64;
            // f9f1 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // f9f7 mov rdi,rbx
            rdi = rbx;
            // f9fa call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f9fd mov cx,WORD PTR [rbp-0x14]
            rcx = (rcx & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u16 as u64);
            // fa01 mov WORD PTR [r14],cx
            m.w16(r14, ((rcx as u16 as u64)) as u16);
            // fa05 jmp 0xfa09
            pc = 0xfa09; continue;
        }
        0xfa07 => {
            // block fa07
            #[cfg(feature = "trace")] eprintln!("f_0000f9bc pc fa07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fa07 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xfa09;
        }
        0xfa09 => {
            // block fa09
            #[cfg(feature = "trace")] eprintln!("f_0000f9bc pc fa09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fa09 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // fa0d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fa0e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // fa10 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fa11 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfbbe size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fbbe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xfbbe;
    loop {
        match pc {
        0xfbbe => {
            // block fbbe
            #[cfg(feature = "trace")] eprintln!("f_0000fbbe pc fbbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fbbe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fbbf mov rbp,rsp
            rbp = rsp;
            // fbc2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // fbc4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fbc5 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // fbc8 mov rbx,rdi
            rbx = rdi;
            // fbcb mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // fbce call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fbd3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // fbd5 je 0xfc09
            if fl.e() { pc = 0xfc09; continue; }
            pc = 0xfbd7;
        }
        0xfbd7 => {
            // block fbd7
            #[cfg(feature = "trace")] eprintln!("f_0000fbbe pc fbd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fbd7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fbda movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // fbde mov esi,0x150
            rsi = (0x150u64) as u32 as u64;
            // fbe3 mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // fbe8 mov r8d,0x7
            r8 = (0x7u64) as u32 as u64;
            // fbee mov rdi,rbx
            rdi = rbx;
            // fbf1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fbf4 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // fbf7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fbfa xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // fbfc xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // fbfe mov rdi,rbx
            rdi = rbx;
            // fc01 call QWORD PTR [rax+0xbd8]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd8)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fc07 jmp 0xfc0c
            pc = 0xfc0c; continue;
        }
        0xfc09 => {
            // block fc09
            #[cfg(feature = "trace")] eprintln!("f_0000fbbe pc fc09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fc09 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xfc0c;
        }
        0xfc0c => {
            // block fc0c
            #[cfg(feature = "trace")] eprintln!("f_0000fbbe pc fc0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fc0c mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // fc0f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fc10 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // fc12 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fc13 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
