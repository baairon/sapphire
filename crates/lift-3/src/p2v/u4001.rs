//! Mechanically lifted functions (scripts/lift_run.py), module u4001. Each one is checked against the original
//! in the world harness (tests/world_u4001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x1064 size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001064<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1064;
    loop {
        match pc {
        0x1064 => {
            // block 1064
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 1064 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1064 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1065 mov rbp,rsp
            rbp = rsp;
            // 1068 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 106a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 106c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 106e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 106f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 1073 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 1076 mov r14,rsi
            r14 = rsi;
            // 1079 mov rbx,rdi
            rbx = rdi;
            // 107c mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 1083 mov DWORD PTR [rbp-0x28],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0xcu64) as u32);
            // 108a mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // 108e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1091 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1097 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 109a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 109c cmove r15d,r12d
            if fl.e() { r15 = ((r12 as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // 10a0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 10a3 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 10a7 mov rdi,rbx
            rdi = rbx;
            // 10aa mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 10ad call QWORD PTR [rax+0x3e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x3e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10b3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 10b5 je 0x10f0
            if fl.e() { pc = 0x10f0; continue; }
            pc = 0x10b7;
        }
        0x10b7 => {
            // block 10b7
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10b7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 10ba lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 10be mov rdi,rbx
            rdi = rbx;
            // 10c1 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 10c4 call QWORD PTR [rax+0x440]
            { let r = { let t = m.r64(rax.wrapping_add(0x440)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10ca test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 10cc je 0x10ed
            if fl.e() { pc = 0x10ed; continue; }
            pc = 0x10ce;
        }
        0x10ce => {
            // block 10ce
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10ce cmp DWORD PTR [rbp-0x24],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64, 0x0u64).1;
            // 10d2 jne 0x10e4
            if fl.ne() { pc = 0x10e4; continue; }
            pc = 0x10d4;
        }
        0x10d4 => {
            // block 10d4
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d4 mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 10d7 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 10da je 0x10e4
            if fl.e() { pc = 0x10e4; continue; }
            pc = 0x10dc;
        }
        0x10dc => {
            // block 10dc
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10dc mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // 10df cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 10e2 jne 0x10f0
            if fl.ne() { pc = 0x10f0; continue; }
            pc = 0x10e4;
        }
        0x10e4 => {
            // block 10e4
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e4 mov BYTE PTR [r14],0x1
            m.w8(r14, (0x1u64) as u8);
            // 10e8 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // 10eb jmp 0x10f0
            pc = 0x10f0; continue;
        }
        0x10ed => {
            // block 10ed
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10ed xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x10f0;
        }
        0x10f0 => {
            // block 10f0
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f0 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 10f3 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 10f7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10f8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 10fa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10fc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10fe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10ff ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x89c8 size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000089c8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x89c8;
    loop {
        match pc {
        0x89c8 => {
            // block 89c8
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 89c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 89c8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 89c9 mov rbp,rsp
            rbp = rsp;
            // 89cc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 89ce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 89cf sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 89d3 mov rbx,rdi
            rbx = rdi;
            // 89d6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 89d9 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 89dd call QWORD PTR [rax+0x12b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x12b0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 89e3 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 89e6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 89e8 je 0x8a58
            if fl.e() { pc = 0x8a58; continue; }
            pc = 0x89ea;
        }
        0x89ea => {
            // block 89ea
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 89ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 89ea test BYTE PTR [rbp-0x14],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64 & 0x1u64);
            // 89ee je 0x8a58
            if fl.e() { pc = 0x8a58; continue; }
            pc = 0x89f0;
        }
        0x89f0 => {
            // block 89f0
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 89f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 89f0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 89f3 mov rdi,rbx
            rdi = rbx;
            // 89f6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 89fc mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 89fe call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8a03 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 8a06 jne 0x8a27
            if fl.ne() { pc = 0x8a27; continue; }
            pc = 0x8a08;
        }
        0x8a08 => {
            // block 8a08
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a08 mov edi,DWORD PTR [rbp-0x14]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 8a0b mov esi,edi
            rsi = ((rdi as u32 as u64)) as u32 as u64;
            // 8a0d shr esi,0x18
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x18u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 8a10 and esi,0x7
            { let r = (rsi as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8a13 mov r14d,0x19
            r14 = (0x19u64) as u32 as u64;
            // 8a19 cmp esi,0x5
            fl = sub::<32>((rsi as u32 as u64), 0x5u64).1;
            // 8a1c je 0x8a58
            if fl.e() { pc = 0x8a58; continue; }
            pc = 0x8a1e;
        }
        0x8a1e => {
            // block 8a1e
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a1e shr edi,0x1c
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x1cu64, fl); rdi = (r) as u32 as u64; fl = f; }
            // 8a21 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 8a23 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 8a25 jmp 0x8a4d
            pc = 0x8a4d; continue;
        }
        0x8a27 => {
            // block 8a27
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a27 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 8a2a jb 0x8a58
            if fl.b() { pc = 0x8a58; continue; }
            pc = 0x8a2c;
        }
        0x8a2c => {
            // block 8a2c
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a2c mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 8a2f mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 8a31 shr edi,0x1c
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x1cu64, fl); rdi = (r) as u32 as u64; fl = f; }
            // 8a34 shr eax,0x4
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 8a37 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 8a39 and esi,0xf
            { let r = (rsi as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 8a3c and eax,0xe
            { let r = (rax as u32 as u64) & 0xeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8a3f cmp eax,0x6
            fl = sub::<32>((rax as u32 as u64), 0x6u64).1;
            // 8a42 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 8a45 movzx ecx,al
            rcx = ((rax as u8 as u64)) as u32 as u64;
            // 8a48 shl ecx,0x3
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x3u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 8a4b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x8a4d;
        }
        0x8a4d => {
            // block 8a4d
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a4d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 8a50 call 0x6d8
            { let r = { let t = m.env.code_va(0x6d8); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8a55 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x8a58;
        }
        0x8a58 => {
            // block 8a58
            #[cfg(feature = "trace")] eprintln!("f_000089c8 pc 8a58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8a58 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 8a5b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8a5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8a60 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8a62 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8a63 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11282 size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011282<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x11282;
    loop {
        match pc {
        0x11282 => {
            // block 11282
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 11282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11282 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11283 mov rbp,rsp
            rbp = rsp;
            // 11286 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11288 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1128a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1128b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1128c mov r15,rdi
            r15 = rdi;
            // 1128f lea rax,[rip+0x13000a] # 0x1412a0
            rax = m.env.data_va(350829);
            // 11296 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 11299 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1129d mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 112a1 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 112a4 je 0x112c1
            if fl.e() { pc = 0x112c1; continue; }
            pc = 0x112a6;
        }
        0x112a6 => {
            // block 112a6
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112a6 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 112aa test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 112ad je 0x112b8
            if fl.e() { pc = 0x112b8; continue; }
            pc = 0x112af;
        }
        0x112af => {
            // block 112af
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112af mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 112b2 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 112b6 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x112b8;
        }
        0x112b8 => {
            // block 112b8
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112b8 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 112bc cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 112bf jne 0x112a6
            if fl.ne() { pc = 0x112a6; continue; }
            pc = 0x112c1;
        }
        0x112c1 => {
            // block 112c1
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112c1 mov rdi,QWORD PTR [r15+0x40]
            rdi = m.r64(r15.wrapping_add(0x40)) as u64;
            // 112c5 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 112c8 je 0x112e8
            if fl.e() { pc = 0x112e8; continue; }
            pc = 0x112ca;
        }
        0x112ca => {
            // block 112ca
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112ca mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 112cd mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // 112d1 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112d3 mov rdi,QWORD PTR [r15+0x40]
            rdi = m.r64(r15.wrapping_add(0x40)) as u64;
            // 112d7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 112da mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 112de call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112e0 mov QWORD PTR [r15+0x40],0x0
            m.w64(r15.wrapping_add(0x40), (0x0u64) as u64);
            pc = 0x112e8;
        }
        0x112e8 => {
            // block 112e8
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 112e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 112e8 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 112eb mov rax,QWORD PTR [rax+0x88]
            rax = m.r64(rax.wrapping_add(0x88)) as u64;
            // 112f2 mov rdi,r15
            rdi = r15;
            // 112f5 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112f7 mov rdi,r15
            rdi = r15;
            // 112fa add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 112fe pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 112ff pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11301 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11303 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11304 jmp 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x11309 => {
            // block 11309
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 11309 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11309 jmp 0x1130b
            pc = 0x1130b; continue;
        }
        0x1130b => {
            // block 1130b
            #[cfg(feature = "trace")] eprintln!("f_00011282 pc 1130b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1130b mov r14,rax
            r14 = rax;
            // 1130e mov rdi,r15
            rdi = r15;
            // 11311 call 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11316 mov rdi,r14
            rdi = r14;
            // 11319 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5c8e size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5c8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd5c8e;
    loop {
        match pc {
        0xd5c8e => {
            // block d5c8e
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5c8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5c8f mov rbp,rsp
            rbp = rsp;
            // d5c92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5c93 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d5c94 mov rbx,rdi
            rbx = rdi;
            // d5c97 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // d5c9e mov DWORD PTR [rbp-0x10],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u32);
            // d5ca5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5ca8 lea rdx,[rbp-0x10]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // d5cac mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // d5cb1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5cb6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5cb9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5cbc mov ecx,DWORD PTR [rbp-0x10]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64) as u32 as u64;
            // d5cbf cmp ecx,0x106466ff
            fl = sub::<32>((rcx as u32 as u64), 0x106466ffu64).1;
            // d5cc5 jg 0xd5cfe
            if fl.g() { pc = 0xd5cfe; continue; }
            pc = 0xd5cc7;
        }
        0xd5cc7 => {
            // block d5cc7
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cc7 cmp ecx,0x10518400
            fl = sub::<32>((rcx as u32 as u64), 0x10518400u64).1;
            // d5ccd je 0xd5cd7
            if fl.e() { pc = 0xd5cd7; continue; }
            pc = 0xd5ccf;
        }
        0xd5ccf => {
            // block d5ccf
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5ccf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ccf cmp ecx,0x10518450
            fl = sub::<32>((rcx as u32 as u64), 0x10518450u64).1;
            // d5cd5 jne 0xd5d21
            if fl.ne() { pc = 0xd5d21; continue; }
            pc = 0xd5cd7;
        }
        0xd5cd7 => {
            // block d5cd7
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cd7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5cda lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d5cde mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // d5ce3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5ce8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5ceb mov rdi,rbx
            rdi = rbx;
            // d5cee call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5cf1 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // d5cf4 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d5cf6 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d5cf9 seta al
            rax = (rax & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // d5cfc jmp 0xd5d23
            pc = 0xd5d23; continue;
        }
        0xd5cfe => {
            // block d5cfe
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cfe mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d5d00 lea edx,[rcx-0x10646700]
            rdx = (rcx.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d5d06 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // d5d09 jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d0b;
        }
        0xd5d0b => {
            // block d5d0b
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d0b lea edx,[rcx-0x10646705]
            rdx = (rcx.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d5d11 cmp edx,0x2
            fl = sub::<32>((rdx as u32 as u64), 0x2u64).1;
            // d5d14 jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d16;
        }
        0xd5d16 => {
            // block d5d16
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d16 add ecx,0xef8ef7b0
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xef8ef7b0u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d5d1c cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // d5d1f jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d21;
        }
        0xd5d21 => {
            // block d5d21
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d21 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd5d23;
        }
        0xd5d23 => {
            // block d5d23
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d23 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d5d27 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5d28 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5d29 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4c34 size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4c34<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4c34;
    loop {
        match pc {
        0xe4c34 => {
            // block e4c34
            #[cfg(feature = "trace")] eprintln!("f_000e4c34 pc e4c34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4c34 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4c35 mov rbp,rsp
            rbp = rsp;
            // e4c38 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4c3a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4c3c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4c3d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e4c3e mov rbx,rdi
            rbx = rdi;
            // e4c41 lea rax,[rip+0x61660] # 0x1462a8
            rax = m.env.data_va(371317);
            // e4c48 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e4c4b lea r14,[rbx+0x20]
            r14 = rbx.wrapping_add(0x20);
            // e4c4f mov QWORD PTR [rbx+0x90],0x0
            m.w64(rbx.wrapping_add(0x90), (0x0u64) as u64);
            // e4c5a mov BYTE PTR [rbx+0x8c],0x0
            m.w8(rbx.wrapping_add(0x8c), (0x0u64) as u8);
            // e4c61 mov BYTE PTR [rbx+0xc],0x0
            m.w8(rbx.wrapping_add(0xc), (0x0u64) as u8);
            // e4c65 mov BYTE PTR [rbx+0xd],0x0
            m.w8(rbx.wrapping_add(0xd), (0x0u64) as u8);
            // e4c69 mov DWORD PTR [rbx+0x8],0x0
            m.w32(rbx.wrapping_add(0x8), (0x0u64) as u32);
            // e4c70 mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // e4c78 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // e4c80 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // e4c88 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // e4c90 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // e4c98 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // e4ca0 call 0xe4cd0
            { let r = { let t = m.env.code_va(0xe4cd0); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4ca5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e4ca9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4caa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4cac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4cae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4caf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe4cb0 => {
            // block e4cb0
            #[cfg(feature = "trace")] eprintln!("f_000e4c34 pc e4cb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4cb0 mov r15,rax
            r15 = rax;
            // e4cb3 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // e4cb7 mov rdi,rbx
            rdi = rbx;
            // e4cba call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4cbf mov rdi,r14
            rdi = r14;
            // e4cc2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4cc7 mov rdi,r15
            rdi = r15;
            // e4cca call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4f2a size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4f2a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4f2a;
    loop {
        match pc {
        0xe4f2a => {
            // block e4f2a
            #[cfg(feature = "trace")] eprintln!("f_000e4f2a pc e4f2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4f2a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4f2b mov rbp,rsp
            rbp = rsp;
            // e4f2e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4f30 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4f32 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4f33 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e4f34 mov rbx,rdi
            rbx = rdi;
            // e4f37 lea rax,[rip+0x6136a] # 0x1462a8
            rax = m.env.data_va(371317);
            // e4f3e mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e4f41 lea r14,[rbx+0x20]
            r14 = rbx.wrapping_add(0x20);
            // e4f45 mov QWORD PTR [rbx+0x90],0x0
            m.w64(rbx.wrapping_add(0x90), (0x0u64) as u64);
            // e4f50 mov BYTE PTR [rbx+0x8c],0x0
            m.w8(rbx.wrapping_add(0x8c), (0x0u64) as u8);
            // e4f57 mov BYTE PTR [rbx+0xc],0x0
            m.w8(rbx.wrapping_add(0xc), (0x0u64) as u8);
            // e4f5b mov BYTE PTR [rbx+0xd],0x0
            m.w8(rbx.wrapping_add(0xd), (0x0u64) as u8);
            // e4f5f mov DWORD PTR [rbx+0x8],0x0
            m.w32(rbx.wrapping_add(0x8), (0x0u64) as u32);
            // e4f66 mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // e4f6e mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // e4f76 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // e4f7e mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // e4f86 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // e4f8e mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // e4f96 call 0xe4fc6
            { let r = { let t = m.env.code_va(0xe4fc6); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4f9b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e4f9f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4fa0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4fa2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4fa4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4fa5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe4fa6 => {
            // block e4fa6
            #[cfg(feature = "trace")] eprintln!("f_000e4f2a pc e4fa6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4fa6 mov r15,rax
            r15 = rax;
            // e4fa9 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // e4fad mov rdi,rbx
            rdi = rbx;
            // e4fb0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4fb5 mov rdi,r14
            rdi = r14;
            // e4fb8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4fbd mov rdi,r15
            rdi = r15;
            // e4fc0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe532e size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e532e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe532e;
    loop {
        match pc {
        0xe532e => {
            // block e532e
            #[cfg(feature = "trace")] eprintln!("f_000e532e pc e532e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e532e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e532f mov rbp,rsp
            rbp = rsp;
            // e5332 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e5334 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e5336 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e5337 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e5338 mov rbx,rdi
            rbx = rdi;
            // e533b lea rax,[rip+0x60f66] # 0x1462a8
            rax = m.env.data_va(371317);
            // e5342 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e5345 lea r14,[rbx+0x20]
            r14 = rbx.wrapping_add(0x20);
            // e5349 mov QWORD PTR [rbx+0x90],0x0
            m.w64(rbx.wrapping_add(0x90), (0x0u64) as u64);
            // e5354 mov BYTE PTR [rbx+0x8c],0x0
            m.w8(rbx.wrapping_add(0x8c), (0x0u64) as u8);
            // e535b mov BYTE PTR [rbx+0xc],0x0
            m.w8(rbx.wrapping_add(0xc), (0x0u64) as u8);
            // e535f mov BYTE PTR [rbx+0xd],0x0
            m.w8(rbx.wrapping_add(0xd), (0x0u64) as u8);
            // e5363 mov DWORD PTR [rbx+0x8],0x0
            m.w32(rbx.wrapping_add(0x8), (0x0u64) as u32);
            // e536a mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // e5372 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // e537a mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // e5382 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // e538a mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // e5392 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // e539a call 0xe53ca
            { let r = { let t = m.env.code_va(0xe53ca); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e539f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e53a3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e53a4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e53a6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e53a8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e53a9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe53aa => {
            // block e53aa
            #[cfg(feature = "trace")] eprintln!("f_000e532e pc e53aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e53aa mov r15,rax
            r15 = rax;
            // e53ad add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // e53b1 mov rdi,rbx
            rdi = rbx;
            // e53b4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e53b9 mov rdi,r14
            rdi = r14;
            // e53bc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e53c1 mov rdi,r15
            rdi = r15;
            // e53c4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5414 size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e5414<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe5414;
    loop {
        match pc {
        0xe5414 => {
            // block e5414
            #[cfg(feature = "trace")] eprintln!("f_000e5414 pc e5414 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5414 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5415 mov rbp,rsp
            rbp = rsp;
            // e5418 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e541a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e541c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e541d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e541e mov rbx,rdi
            rbx = rdi;
            // e5421 lea rax,[rip+0x60e80] # 0x1462a8
            rax = m.env.data_va(371317);
            // e5428 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e542b lea r14,[rbx+0x20]
            r14 = rbx.wrapping_add(0x20);
            // e542f mov QWORD PTR [rbx+0x90],0x0
            m.w64(rbx.wrapping_add(0x90), (0x0u64) as u64);
            // e543a mov BYTE PTR [rbx+0x8c],0x0
            m.w8(rbx.wrapping_add(0x8c), (0x0u64) as u8);
            // e5441 mov BYTE PTR [rbx+0xc],0x0
            m.w8(rbx.wrapping_add(0xc), (0x0u64) as u8);
            // e5445 mov BYTE PTR [rbx+0xd],0x0
            m.w8(rbx.wrapping_add(0xd), (0x0u64) as u8);
            // e5449 mov DWORD PTR [rbx+0x8],0x0
            m.w32(rbx.wrapping_add(0x8), (0x0u64) as u32);
            // e5450 mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // e5458 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // e5460 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // e5468 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // e5470 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // e5478 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // e5480 call 0xe54b0
            { let r = { let t = m.env.code_va(0xe54b0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e5485 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e5489 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e548a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e548c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e548e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e548f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe5490 => {
            // block e5490
            #[cfg(feature = "trace")] eprintln!("f_000e5414 pc e5490 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5490 mov r15,rax
            r15 = rax;
            // e5493 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // e5497 mov rdi,rbx
            rdi = rbx;
            // e549a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e549f mov rdi,r14
            rdi = r14;
            // e54a2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e54a7 mov rdi,r15
            rdi = r15;
            // e54aa call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e306 size=158
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e306<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x2e306;
    loop {
        match pc {
        0x2e306 => {
            // block 2e306
            #[cfg(feature = "trace")] eprintln!("f_0002e306 pc 2e306 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e306 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e307 mov rbp,rsp
            rbp = rsp;
            // 2e30a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e30c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e30e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2e310 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e312 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e313 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2e317 movsd QWORD PTR [rbp-0x30],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x1 as u64);
            // 2e31c movsd QWORD PTR [rbp-0x38],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), x0 as u64);
            // 2e321 mov QWORD PTR [rbp-0x40],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r9) as u64);
            // 2e325 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // 2e328 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // 2e32b mov r13d,edx
            r13 = ((rdx as u32 as u64)) as u32 as u64;
            // 2e32e mov r14,rsi
            r14 = rsi;
            // 2e331 mov edi,0x70
            rdi = (0x70u64) as u32 as u64;
            // 2e336 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e33b mov rbx,rax
            rbx = rax;
            // 2e33e movzx eax,BYTE PTR [rbp+0x28]
            rax = (m.r8(rbp.wrapping_add(0x28)) as u64) as u32 as u64;
            // 2e342 mov DWORD PTR [rsp+0x18],eax
            m.w32(rsp.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 2e346 movzx eax,BYTE PTR [rbp+0x20]
            rax = (m.r8(rbp.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2e34a mov DWORD PTR [rsp+0x10],eax
            m.w32(rsp.wrapping_add(0x10), ((rax as u32 as u64)) as u32);
            // 2e34e mov eax,DWORD PTR [rbp+0x18]
            rax = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 2e351 mov DWORD PTR [rsp+0x8],eax
            m.w32(rsp.wrapping_add(0x8), ((rax as u32 as u64)) as u32);
            // 2e355 movzx eax,BYTE PTR [rbp+0x10]
            rax = (m.r8(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // 2e359 mov DWORD PTR [rsp],eax
            m.w32(rsp, ((rax as u32 as u64)) as u32);
            // 2e35c mov rdi,rbx
            rdi = rbx;
            // 2e35f mov rsi,r14
            rsi = r14;
            // 2e362 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // 2e365 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // 2e368 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // 2e36b mov r9,QWORD PTR [rbp-0x40]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2e36f movsd xmm0,QWORD PTR [rbp-0x38]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u128;
            // 2e374 movsd xmm1,QWORD PTR [rbp-0x30]
            x1 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u128;
            // 2e379 call 0x2ec36
            { let r = { let t = m.env.code_va(0x2ec36); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e37e mov rax,rbx
            rax = rbx;
            // 2e381 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2e385 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e386 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e388 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2e38a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e38c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e38e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e38f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2e390 => {
            // block 2e390
            #[cfg(feature = "trace")] eprintln!("f_0002e306 pc 2e390 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e390 mov r14,rax
            r14 = rax;
            // 2e393 mov rdi,rbx
            rdi = rbx;
            // 2e396 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2e39b mov rdi,r14
            rdi = r14;
            // 2e39e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8140 size=158
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d8140<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd8140;
    loop {
        match pc {
        0xd8140 => {
            // block d8140
            #[cfg(feature = "trace")] eprintln!("f_000d8140 pc d8140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8140 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8141 mov rbp,rsp
            rbp = rsp;
            // d8144 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d8146 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d8148 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d8149 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d814a mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // d814d mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d8150 mov rbx,rdi
            rbx = rdi;
            // d8153 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8156 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d815b mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d8160 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8165 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8168 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d816b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d816e mov rdi,rbx
            rdi = rbx;
            // d8171 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8177 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d817a mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d817f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8184 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8187 mov rdi,rbx
            rdi = rbx;
            // d818a mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // d818d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8190 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8193 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d8198 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d819d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d81a0 mov rdi,rbx
            rdi = rbx;
            // d81a3 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d81a6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d81a9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d81ac mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d81b1 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // d81b6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d81bb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d81be mov rdi,rbx
            rdi = rbx;
            // d81c1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d81c4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d81c7 mov rdi,rbx
            rdi = rbx;
            // d81ca call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d81d0 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d81d2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d81d6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d81d7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d81d9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d81db pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d81dc ret
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

/// parity: P2v @0x12785 size=159
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012785<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x12785;
    loop {
        match pc {
        0x12785 => {
            // block 12785
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 12785 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12785 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12786 mov rbp,rsp
            rbp = rsp;
            // 12789 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1278b push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1278d push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1278f push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12791 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12792 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 12793 mov r15,r9
            r15 = r9;
            // 12796 mov r14,rcx
            r14 = rcx;
            // 12799 mov r12,rsi
            r12 = rsi;
            // 1279c cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 1279f je 0x12815
            if fl.e() { pc = 0x12815; continue; }
            pc = 0x127a1;
        }
        0x127a1 => {
            // block 127a1
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127a1 lea r13,[rbp+0x20]
            r13 = rbp.wrapping_add(0x20);
            // 127a5 lea rax,[rbp+0x10]
            rax = rbp.wrapping_add(0x10);
            // 127a9 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 127ad mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            pc = 0x127b1;
        }
        0x127b1 => {
            // block 127b1
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127b1 cmp r15,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 127b5 je 0x127f0
            if fl.e() { pc = 0x127f0; continue; }
            pc = 0x127b7;
        }
        0x127b7 => {
            // block 127b7
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127b7 mov eax,DWORD PTR [r15-0x20]
            rax = (m.r32(r15.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // 127bb mov rbx,QWORD PTR [r13+0x8]
            rbx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 127bf add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            // 127c3 cmp eax,DWORD PTR [r12-0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r12.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // 127c8 jge 0x127d6
            if fl.ge() { pc = 0x127d6; continue; }
            pc = 0x127ca;
        }
        0x127ca => {
            // block 127ca
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127ca add r12,0xffffffffffffffc8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffc8u64);
              r12 = r;
              fl = f; }
            // 127ce mov rdi,rbx
            rdi = rbx;
            // 127d1 mov rsi,r12
            rsi = r12;
            // 127d4 jmp 0x127e0
            pc = 0x127e0; continue;
        }
        0x127d6 => {
            // block 127d6
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127d6 add r15,0xffffffffffffffc8
            { let (r, f) = add::<64>(r15, 0xffff_ffff_ffff_ffc8u64);
              r15 = r;
              fl = f; }
            // 127da mov rdi,rbx
            rdi = rbx;
            // 127dd mov rsi,r15
            rsi = r15;
            pc = 0x127e0;
        }
        0x127e0 => {
            // block 127e0
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127e0 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 127e5 mov QWORD PTR [r13+0x8],rbx
            m.w64(r13.wrapping_add(0x8), (rbx) as u64);
            // 127e9 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 127ec jne 0x127b1
            if fl.ne() { pc = 0x127b1; continue; }
            pc = 0x127ee;
        }
        0x127ee => {
            // block 127ee
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127ee jmp 0x12815
            pc = 0x12815; continue;
        }
        0x127f0 => {
            // block 127f0
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127f0 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 127f3 je 0x12815
            if fl.e() { pc = 0x12815; continue; }
            pc = 0x127f5;
        }
        0x127f5 => {
            // block 127f5
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127f5 mov rbx,QWORD PTR [r13+0x8]
            rbx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 127f9 add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            pc = 0x127fd;
        }
        0x127fd => {
            // block 127fd
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 127fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 127fd add r12,0xffffffffffffffc8
            { let (r, f) = add::<64>(r12, 0xffff_ffff_ffff_ffc8u64);
              r12 = r;
              fl = f; }
            // 12801 mov rdi,rbx
            rdi = rbx;
            // 12804 mov rsi,r12
            rsi = r12;
            // 12807 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1280c add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            // 12810 cmp r14,r12
            fl = sub::<64>(r14, r12).1;
            // 12813 jne 0x127fd
            if fl.ne() { pc = 0x127fd; continue; }
            pc = 0x12815;
        }
        0x12815 => {
            // block 12815
            #[cfg(feature = "trace")] eprintln!("f_00012785 pc 12815 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12815 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 12819 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1281a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1281c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1281e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 12820 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12822 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12823 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x26de size=160
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000026de<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x26de;
    loop {
        match pc {
        0x26de => {
            // block 26de
            #[cfg(feature = "trace")] eprintln!("f_000026de pc 26de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26de push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 26df mov rbp,rsp
            rbp = rsp;
            // 26e2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 26e4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 26e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 26e7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 26e8 mov r14,rdx
            r14 = rdx;
            // 26eb mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 26ee mov rbx,rdi
            rbx = rdi;
            // 26f1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 26f4 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26fa test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 26fc jne 0x2771
            if fl.ne() { pc = 0x2771; continue; }
            pc = 0x26fe;
        }
        0x26fe => {
            // block 26fe
            #[cfg(feature = "trace")] eprintln!("f_000026de pc 26fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26fe mov DWORD PTR [r14],0x5
            m.w32(r14, (0x5u64) as u32);
            // 2705 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 270c mov DWORD PTR [rbp-0x20],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u32);
            // 2713 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2716 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 2719 lea rdx,[rip+0xfc960] # 0xff080
            rdx = m.env.data_va(79949);
            // 2720 mov r15d,DWORD PTR [rdx+rcx*4]
            r15 = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2724 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 2728 mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // 272d mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 2733 mov rdi,rbx
            rdi = rbx;
            // 2736 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2739 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 273c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 273e je 0x2771
            if fl.e() { pc = 0x2771; continue; }
            pc = 0x2740;
        }
        0x2740 => {
            // block 2740
            #[cfg(feature = "trace")] eprintln!("f_000026de pc 2740 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2740 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2743 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // 2747 mov ecx,0x6000000
            rcx = (0x6000000u64) as u32 as u64;
            // 274c mov r8d,0x19
            r8 = (0x19u64) as u32 as u64;
            // 2752 mov rdi,rbx
            rdi = rbx;
            // 2755 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2758 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 275b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 275d je 0x2771
            if fl.e() { pc = 0x2771; continue; }
            pc = 0x275f;
        }
        0x275f => {
            // block 275f
            #[cfg(feature = "trace")] eprintln!("f_000026de pc 275f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 275f mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // 2762 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 2764 and eax,0x6
            { let r = (rax as u32 as u64) & 0x6u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2767 add eax,DWORD PTR [rbp-0x1c]
            { let (r, f) = add::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 276a mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 276d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 276f jmp 0x2773
            pc = 0x2773; continue;
        }
        0x2771 => {
            // block 2771
            #[cfg(feature = "trace")] eprintln!("f_000026de pc 2771 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2771 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2773;
        }
        0x2773 => {
            // block 2773
            #[cfg(feature = "trace")] eprintln!("f_000026de pc 2773 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2773 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2777 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2778 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 277a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 277c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 277d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7432 size=160
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007432<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x7432;
    loop {
        match pc {
        0x7432 => {
            // block 7432
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 7432 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7432 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7433 mov rbp,rsp
            rbp = rsp;
            // 7436 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7438 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 743a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 743b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 743c mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 743f mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 7442 mov rbx,rdi
            rbx = rdi;
            // 7445 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 7448 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 744b call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7451 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7453 jne 0x748c
            if fl.ne() { pc = 0x748c; continue; }
            pc = 0x7455;
        }
        0x7455 => {
            // block 7455
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 7455 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7455 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 7458 call 0xcfa3b
            { let r = { let t = m.env.code_va(0xcfa3b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 745d test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 7460 je 0x748c
            if fl.e() { pc = 0x748c; continue; }
            pc = 0x7462;
        }
        0x7462 => {
            // block 7462
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 7462 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7462 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 7465 call 0xcc106
            { let r = { let t = m.env.code_va(0xcc106); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 746a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 746c je 0x7490
            if fl.e() { pc = 0x7490; continue; }
            pc = 0x746e;
        }
        0x746e => {
            // block 746e
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 746e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 746e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7470 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 7474 je 0x74a4
            if fl.e() { pc = 0x74a4; continue; }
            pc = 0x7476;
        }
        0x7476 => {
            // block 7476
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 7476 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7476 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 747a jne 0x7499
            if fl.ne() { pc = 0x7499; continue; }
            pc = 0x747c;
        }
        0x747c => {
            // block 747c
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 747c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 747c mov eax,0x30000000
            rax = (0x30000000u64) as u32 as u64;
            // 7481 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 7484 or ecx,0x4
            { let r = (rcx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 7487 cmp ecx,0x4
            fl = sub::<32>((rcx as u32 as u64), 0x4u64).1;
            // 748a je 0x74a4
            if fl.e() { pc = 0x74a4; continue; }
            pc = 0x748c;
        }
        0x748c => {
            // block 748c
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 748c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 748c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 748e jmp 0x74c7
            pc = 0x74c7; continue;
        }
        0x7490 => {
            // block 7490
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 7490 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7490 cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 7494 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 7497 jmp 0x74c7
            pc = 0x74c7; continue;
        }
        0x7499 => {
            // block 7499
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 7499 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7499 cmp r15d,0x2
            fl = sub::<32>((r15 as u32 as u64), 0x2u64).1;
            // 749d jne 0x74c7
            if fl.ne() { pc = 0x74c7; continue; }
            pc = 0x749f;
        }
        0x749f => {
            // block 749f
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 749f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 749f mov eax,0x1d
            rax = (0x1du64) as u32 as u64;
            pc = 0x74a4;
        }
        0x74a4 => {
            // block 74a4
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 74a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 74a4 mov r9,QWORD PTR [rbx]
            r9 = m.r64(rbx) as u64;
            // 74a7 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 74aa lea rdx,[rip+0xf7f9f] # 0xff450
            rdx = m.env.data_va(80925);
            // 74b1 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 74b4 mov ecx,0x30000000
            rcx = (0x30000000u64) as u32 as u64;
            // 74b9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 74bc mov rdi,rbx
            rdi = rbx;
            // 74bf mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 74c1 call QWORD PTR [r9+0x30]
            { let r = { let t = m.r64(r9.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 74c5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x74c7;
        }
        0x74c7 => {
            // block 74c7
            #[cfg(feature = "trace")] eprintln!("f_00007432 pc 74c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 74c7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 74cb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 74cc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 74ce pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 74d0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 74d1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf1d4 size=160
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f1d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xf1d4;
    loop {
        match pc {
        0xf1d4 => {
            // block f1d4
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f1d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f1d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f1d5 mov rbp,rsp
            rbp = rsp;
            // f1d8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // f1da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f1dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f1dd sub rsp,0x88
            { let (r, f) = sub::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // f1e4 mov r14,rdx
            r14 = rdx;
            // f1e7 mov rbx,rdi
            rbx = rdi;
            // f1ea lea r15,[rbp-0x98]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // f1f1 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // f1f3 mov rdi,r15
            rdi = r15;
            // f1f6 call 0x2ccf6
            { let r = { let t = m.env.code_va(0x2ccf6); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f1fb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f1fe mov rax,QWORD PTR [rax+0xd0]
            rax = m.r64(rax.wrapping_add(0xd0)) as u64;
            // f205 mov rdi,rbx
            rdi = rbx;
            // f208 mov rsi,r15
            rsi = r15;
            // f20b call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f20d mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // f20f test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // f211 je 0xf235
            if fl.e() { pc = 0xf235; continue; }
            pc = 0xf213;
        }
        0xf213 => {
            // block f213
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f213 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f213 cmp QWORD PTR [rbp-0x4c],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64, 0x0u64).1;
            // f218 je 0xf235
            if fl.e() { pc = 0xf235; continue; }
            pc = 0xf21a;
        }
        0xf21a => {
            // block f21a
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f21a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f21a cmp DWORD PTR [rbp-0x44],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64, 0x0u64).1;
            // f21e je 0xf235
            if fl.e() { pc = 0xf235; continue; }
            pc = 0xf220;
        }
        0xf220 => {
            // block f220
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f220 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f220 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // f227 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // f229 call 0x2cd04
            { let r = { let t = m.env.code_va(0x2cd04); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f22e mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // f231 mov QWORD PTR [r14+0x8],rdx
            m.w64(r14.wrapping_add(0x8), (rdx) as u64);
            pc = 0xf235;
        }
        0xf235 => {
            // block f235
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f235 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f235 lea rdi,[rbp-0x4c]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb4);
            // f239 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f23e lea rdi,[rbp-0x74]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // f242 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f247 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // f249 add rsp,0x88
            { let (r, f) = add::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // f250 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f251 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f253 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // f255 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f256 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xf257 => {
            // block f257
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f257 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f257 mov rbx,rax
            rbx = rax;
            // f25a lea rdi,[rbp-0x4c]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb4);
            // f25e call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f263 lea rdi,[rbp-0x74]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // f267 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f26c mov rdi,rbx
            rdi = rbx;
            // f26f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x270f1 size=160
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000270f1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x270f1;
    loop {
        match pc {
        0x270f1 => {
            // block 270f1
            #[cfg(feature = "trace")] eprintln!("f_000270f1 pc 270f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 270f1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 270f2 mov rbp,rsp
            rbp = rsp;
            // 270f5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 270f7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 270f9 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 270fb push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 270fc mov r15,rsi
            r15 = rsi;
            // 270ff mov r14,rdi
            r14 = rdi;
            // 27102 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 27106 sub rax,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rax, m.r64(r15) as u64);
              rax = r;
              fl = f; }
            // 27109 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 2710d movabs rsi,0xaaaaaaaaaaaaaaab
            rsi = 0xaaaa_aaaa_aaaa_aaabu64;
            // 27117 imul rsi,rax
            { let (r, f) = imul::<64>(rsi, rax); rsi = r; fl = f; }
            // 2711b call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27120 lea rsi,[rip+0x103373] # 0x12a49a
            rsi = m.env.data_va(257127);
            // 27127 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 2712c mov rdi,rax
            rdi = rax;
            // 2712f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27134 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 27137 cmp rbx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 2713b je 0x27175
            if fl.e() { pc = 0x27175; continue; }
            pc = 0x2713d;
        }
        0x2713d => {
            // block 2713d
            #[cfg(feature = "trace")] eprintln!("f_000270f1 pc 2713d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2713d lea r12,[rip+0x103359] # 0x12a49d
            r12 = m.env.data_va(257130);
            pc = 0x27144;
        }
        0x27144 => {
            // block 27144
            #[cfg(feature = "trace")] eprintln!("f_000270f1 pc 27144 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27144 mov rdi,r14
            rdi = r14;
            // 27147 mov rsi,rbx
            rsi = rbx;
            // 2714a call 0x26f9d
            { let r = { let t = m.env.code_va(0x26f9d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2714f add rbx,0xc
            { let (r, f) = add::<64>(rbx, 0xcu64);
              rbx = r;
              fl = f; }
            // 27153 cmp rbx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 27157 mov rax,rbx
            rax = rbx;
            // 2715a je 0x27170
            if fl.e() { pc = 0x27170; continue; }
            pc = 0x2715c;
        }
        0x2715c => {
            // block 2715c
            #[cfg(feature = "trace")] eprintln!("f_000270f1 pc 2715c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2715c mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 27161 mov rdi,r14
            rdi = r14;
            // 27164 mov rsi,r12
            rsi = r12;
            // 27167 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2716c mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x27170;
        }
        0x27170 => {
            // block 27170
            #[cfg(feature = "trace")] eprintln!("f_000270f1 pc 27170 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27170 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 27173 jne 0x27144
            if fl.ne() { pc = 0x27144; continue; }
            pc = 0x27175;
        }
        0x27175 => {
            // block 27175
            #[cfg(feature = "trace")] eprintln!("f_000270f1 pc 27175 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27175 lea rsi,[rip+0x1032d8] # 0x12a454
            rsi = m.env.data_va(257057);
            // 2717c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 27181 mov rdi,r14
            rdi = r14;
            // 27184 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 27185 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 27187 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 27189 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2718b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2718c jmp 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41e06 size=160
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041e06<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x41e06;
    loop {
        match pc {
        0x41e06 => {
            // block 41e06
            #[cfg(feature = "trace")] eprintln!("f_00041e06 pc 41e06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41e06 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41e07 mov rbp,rsp
            rbp = rsp;
            // 41e0a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 41e0c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 41e0e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 41e0f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 41e10 mov r14,rsi
            r14 = rsi;
            // 41e13 mov rbx,rdi
            rbx = rdi;
            // 41e16 cmp edx,0x8
            fl = sub::<32>((rdx as u32 as u64), 0x8u64).1;
            // 41e19 jae 0x41e59
            if fl.ae() { pc = 0x41e59; continue; }
            pc = 0x41e1b;
        }
        0x41e1b => {
            // block 41e1b
            #[cfg(feature = "trace")] eprintln!("f_00041e06 pc 41e1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41e1b mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 41e1d lea rcx,[rip+0xbe59c] # 0x1003c0
            rcx = m.env.data_va(84877);
            // 41e24 mov r15d,DWORD PTR [rcx+rax*4]
            r15 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 41e28 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 41e2f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 41e32 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 41e36 mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 41e3b mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 41e41 mov rdi,rbx
            rdi = rbx;
            // 41e44 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 41e47 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41e4a cmp DWORD PTR [rbp-0x1c],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x1u64).1;
            // 41e4e jne 0x41e64
            if fl.ne() { pc = 0x41e64; continue; }
            pc = 0x41e50;
        }
        0x41e50 => {
            // block 41e50
            #[cfg(feature = "trace")] eprintln!("f_00041e06 pc 41e50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41e50 mov DWORD PTR [r14],0x10
            m.w32(r14, (0x10u64) as u32);
            // 41e57 jmp 0x41e95
            pc = 0x41e95; continue;
        }
        0x41e59 => {
            // block 41e59
            #[cfg(feature = "trace")] eprintln!("f_00041e06 pc 41e59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41e59 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 41e60 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 41e62 jmp 0x41e9a
            pc = 0x41e9a; continue;
        }
        0x41e64 => {
            // block 41e64
            #[cfg(feature = "trace")] eprintln!("f_00041e06 pc 41e64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41e64 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 41e67 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 41e6b mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 41e70 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 41e76 mov rdi,rbx
            rdi = rbx;
            // 41e79 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 41e7c call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 41e7f cmp DWORD PTR [rbp-0x1c],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x1u64).1;
            // 41e83 jne 0x41e8e
            if fl.ne() { pc = 0x41e8e; continue; }
            pc = 0x41e85;
        }
        0x41e85 => {
            // block 41e85
            #[cfg(feature = "trace")] eprintln!("f_00041e06 pc 41e85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41e85 mov DWORD PTR [r14],0x8
            m.w32(r14, (0x8u64) as u32);
            // 41e8c jmp 0x41e95
            pc = 0x41e95; continue;
        }
        0x41e8e => {
            // block 41e8e
            #[cfg(feature = "trace")] eprintln!("f_00041e06 pc 41e8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41e8e mov DWORD PTR [r14],0x6
            m.w32(r14, (0x6u64) as u32);
            pc = 0x41e95;
        }
        0x41e95 => {
            // block 41e95
            #[cfg(feature = "trace")] eprintln!("f_00041e06 pc 41e95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41e95 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 41e97 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x41e9a;
        }
        0x41e9a => {
            // block 41e9a
            #[cfg(feature = "trace")] eprintln!("f_00041e06 pc 41e9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41e9a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 41e9e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 41e9f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 41ea1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 41ea3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41ea4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42140 size=160
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042140<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42140;
    loop {
        match pc {
        0x42140 => {
            // block 42140
            #[cfg(feature = "trace")] eprintln!("f_00042140 pc 42140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42140 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42141 mov rbp,rsp
            rbp = rsp;
            // 42144 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42146 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42148 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4214a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4214c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4214d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4214e mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 42150 mov r15,rdi
            r15 = rdi;
            // 42153 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 42156 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 4215a call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4215f mov r12w,ax
            r12 = (r12 & !0xffff) | (((rax as u16 as u64)) as u16 as u64);
            // 42163 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 42166 ja 0x421cf
            if fl.a() { pc = 0x421cf; continue; }
            pc = 0x42168;
        }
        0x42168 => {
            // block 42168
            #[cfg(feature = "trace")] eprintln!("f_00042140 pc 42168 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42168 lea rax,[rip+0xbe301] # 0x100470
            rax = m.env.data_va(85053);
            // 4216f mov r14d,DWORD PTR [rax+r14*4]
            r14 = (m.r32(rax.wrapping_add(r14.wrapping_mul(4))) as u64) as u32 as u64;
            // 42173 movsxd r13,ebx
            r13 = sx::<32>((rbx as u32 as u64)) as u64;
            // 42176 lea rax,[rip+0xbe543] # 0x1006c0
            rax = m.env.data_va(85645);
            // 4217d mov edx,DWORD PTR [rax+r13*4]
            rdx = (m.r32(rax.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 42181 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 42184 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 42189 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 4218f mov rdi,r15
            rdi = r15;
            // 42192 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 42195 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42198 cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 4219b jg 0x421a6
            if fl.g() { pc = 0x421a6; continue; }
            pc = 0x4219d;
        }
        0x4219d => {
            // block 4219d
            #[cfg(feature = "trace")] eprintln!("f_00042140 pc 4219d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4219d movzx ecx,r12w
            rcx = ((r12 as u16 as u64)) as u32 as u64;
            // 421a1 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 421a4 jb 0x421c8
            if fl.b() { pc = 0x421c8; continue; }
            pc = 0x421a6;
        }
        0x421a6 => {
            // block 421a6
            #[cfg(feature = "trace")] eprintln!("f_00042140 pc 421a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 421a6 lea rax,[rip+0xbe533] # 0x1006e0
            rax = m.env.data_va(85677);
            // 421ad mov edx,DWORD PTR [rax+r13*4]
            rdx = (m.r32(rax.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 421b1 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 421b4 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 421b9 mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            // 421bf mov rdi,r15
            rdi = r15;
            // 421c2 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 421c5 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x421c8;
        }
        0x421c8 => {
            // block 421c8
            #[cfg(feature = "trace")] eprintln!("f_00042140 pc 421c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 421c8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 421ca setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 421cd jmp 0x421d1
            pc = 0x421d1; continue;
        }
        0x421cf => {
            // block 421cf
            #[cfg(feature = "trace")] eprintln!("f_00042140 pc 421cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 421cf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x421d1;
        }
        0x421d1 => {
            // block 421d1
            #[cfg(feature = "trace")] eprintln!("f_00042140 pc 421d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 421d1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 421d5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 421d6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 421d8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 421da pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 421dc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 421de pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 421df ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3a0c size=160
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3a0c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd3a0c;
    loop {
        match pc {
        0xd3a0c => {
            // block d3a0c
            #[cfg(feature = "trace")] eprintln!("f_000d3a0c pc d3a0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3a0c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3a0d mov rbp,rsp
            rbp = rsp;
            // d3a10 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3a12 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d3a14 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d3a16 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d3a18 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d3a19 sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d3a1d mov r14,rcx
            r14 = rcx;
            // d3a20 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // d3a23 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // d3a26 mov rbx,rdi
            rbx = rdi;
            // d3a29 cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // d3a2d je 0xd3a9a
            if fl.e() { pc = 0xd3a9a; continue; }
            pc = 0xd3a2f;
        }
        0xd3a2f => {
            // block d3a2f
            #[cfg(feature = "trace")] eprintln!("f_000d3a0c pc d3a2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3a2f lea r13,[rbp-0x90]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d3a36 mov esi,0x10
            rsi = (0x10u64) as u32 as u64;
            // d3a3b mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // d3a40 mov rdi,r13
            rdi = r13;
            // d3a43 call 0x26090
            { let r = { let t = m.env.code_va(0x26090); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3a48 xorps xmm0,xmm0
            x0 = 0;
            // d3a4b movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d3a4f movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // d3a53 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // d3a57 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // d3a5b movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // d3a5f movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // d3a66 mov DWORD PTR [rbp-0x90],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x5u64) as u32);
            // d3a70 mov DWORD PTR [rbp-0x8c],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff74), ((r12 as u32 as u64)) as u32);
            // d3a77 mov DWORD PTR [rbp-0x88],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff78), ((r15 as u32 as u64)) as u32);
            // d3a7e mov QWORD PTR [rbp-0x50],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r14) as u64);
            // d3a82 mov DWORD PTR [r14],r12d
            m.w32(r14, ((r12 as u32 as u64)) as u32);
            // d3a85 mov DWORD PTR [r14+0xc],r15d
            m.w32(r14.wrapping_add(0xc), ((r15 as u32 as u64)) as u32);
            // d3a89 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3a8c mov rdi,rbx
            rdi = rbx;
            // d3a8f mov rsi,r13
            rsi = r13;
            // d3a92 call QWORD PTR [rax+0xc8]
            { let r = { let t = m.r64(rax.wrapping_add(0xc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3a98 jmp 0xd3a9c
            pc = 0xd3a9c; continue;
        }
        0xd3a9a => {
            // block d3a9a
            #[cfg(feature = "trace")] eprintln!("f_000d3a0c pc d3a9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3a9a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3a9c;
        }
        0xd3a9c => {
            // block d3a9c
            #[cfg(feature = "trace")] eprintln!("f_000d3a0c pc d3a9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3a9c add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // d3aa0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3aa1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d3aa3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d3aa5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3aa7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3aa9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3aaa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x61a4c size=161
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00061a4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x61a4c;
    loop {
        match pc {
        0x61a4c => {
            // block 61a4c
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 61a4d mov rbp,rsp
            rbp = rsp;
            // 61a50 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 61a52 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 61a53 mov r14,rdi
            r14 = rdi;
            // 61a56 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 61a58 je 0x61ab4
            if fl.e() { pc = 0x61ab4; continue; }
            pc = 0x61a5a;
        }
        0x61a5a => {
            // block 61a5a
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a5a cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 61a5d jg 0x61a6a
            if fl.g() { pc = 0x61a6a; continue; }
            pc = 0x61a5f;
        }
        0x61a5f => {
            // block 61a5f
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a5f jne 0x61ab4
            if fl.ne() { pc = 0x61ab4; continue; }
            pc = 0x61a61;
        }
        0x61a61 => {
            // block 61a61
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a61 lea rsi,[rip+0xcdcaf] # 0x12f717
            rsi = m.env.data_va(278244);
            // 61a68 jmp 0x61a97
            pc = 0x61a97; continue;
        }
        0x61a6a => {
            // block 61a6a
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a6a cmp esi,0x1f
            fl = sub::<32>((rsi as u32 as u64), 0x1fu64).1;
            // 61a6d jg 0x61a7d
            if fl.g() { pc = 0x61a7d; continue; }
            pc = 0x61a6f;
        }
        0x61a6f => {
            // block 61a6f
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a6f cmp esi,0x2
            fl = sub::<32>((rsi as u32 as u64), 0x2u64).1;
            // 61a72 jne 0x61a8b
            if fl.ne() { pc = 0x61a8b; continue; }
            pc = 0x61a74;
        }
        0x61a74 => {
            // block 61a74
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a74 lea rsi,[rip+0xcdca8] # 0x12f723
            rsi = m.env.data_va(278256);
            // 61a7b jmp 0x61a97
            pc = 0x61a97; continue;
        }
        0x61a7d => {
            // block 61a7d
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a7d cmp esi,0x20
            fl = sub::<32>((rsi as u32 as u64), 0x20u64).1;
            // 61a80 jne 0x61a9e
            if fl.ne() { pc = 0x61a9e; continue; }
            pc = 0x61a82;
        }
        0x61a82 => {
            // block 61a82
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a82 lea rsi,[rip+0xcdcb2] # 0x12f73b
            rsi = m.env.data_va(278280);
            // 61a89 jmp 0x61aaa
            pc = 0x61aaa; continue;
        }
        0x61a8b => {
            // block 61a8b
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a8b cmp esi,0x3
            fl = sub::<32>((rsi as u32 as u64), 0x3u64).1;
            // 61a8e jne 0x61ab4
            if fl.ne() { pc = 0x61ab4; continue; }
            pc = 0x61a90;
        }
        0x61a90 => {
            // block 61a90
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a90 lea rsi,[rip+0xcdc98] # 0x12f72f
            rsi = m.env.data_va(278268);
            pc = 0x61a97;
        }
        0x61a97 => {
            // block 61a97
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a97 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 61a9c jmp 0x61aaf
            pc = 0x61aaf; continue;
        }
        0x61a9e => {
            // block 61a9e
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61a9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61a9e cmp esi,0x21
            fl = sub::<32>((rsi as u32 as u64), 0x21u64).1;
            // 61aa1 jne 0x61ab4
            if fl.ne() { pc = 0x61ab4; continue; }
            pc = 0x61aa3;
        }
        0x61aa3 => {
            // block 61aa3
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61aa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61aa3 lea rsi,[rip+0xcdc9b] # 0x12f745
            rsi = m.env.data_va(278290);
            pc = 0x61aaa;
        }
        0x61aaa => {
            // block 61aaa
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61aaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61aaa mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            pc = 0x61aaf;
        }
        0x61aaf => {
            // block 61aaf
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61aaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61aaf mov rdi,r14
            rdi = r14;
            // 61ab2 jmp 0x61ae0
            pc = 0x61ae0; continue;
        }
        0x61ab4 => {
            // block 61ab4
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61ab4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61ab4 cmp esi,0x79
            fl = sub::<32>((rsi as u32 as u64), 0x79u64).1;
            // 61ab7 jg 0x61ac8
            if fl.g() { pc = 0x61ac8; continue; }
            pc = 0x61ab9;
        }
        0x61ab9 => {
            // block 61ab9
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61ab9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61ab9 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 61abb lea rcx,[rip+0xdfe0e] # 0x1418d0
            rcx = m.env.data_va(352413);
            // 61ac2 mov rbx,QWORD PTR [rcx+rax*8]
            rbx = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // 61ac6 jmp 0x61acf
            pc = 0x61acf; continue;
        }
        0x61ac8 => {
            // block 61ac8
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61ac8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61ac8 lea rbx,[rip+0xd71f9] # 0x138cc8
            rbx = m.env.data_va(316565);
            pc = 0x61acf;
        }
        0x61acf => {
            // block 61acf
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61acf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61acf mov rdi,rbx
            rdi = rbx;
            // 61ad2 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 61ad7 mov rdi,r14
            rdi = r14;
            // 61ada mov rsi,rbx
            rsi = rbx;
            // 61add mov rdx,rax
            rdx = rax;
            pc = 0x61ae0;
        }
        0x61ae0 => {
            // block 61ae0
            #[cfg(feature = "trace")] eprintln!("f_00061a4c pc 61ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 61ae0 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 61ae5 mov rax,r14
            rax = r14;
            // 61ae8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 61ae9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 61aeb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 61aec ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x50a0 size=162
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000050a0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x50a0;
    loop {
        match pc {
        0x50a0 => {
            // block 50a0
            #[cfg(feature = "trace")] eprintln!("f_000050a0 pc 50a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50a0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 50a1 mov rbp,rsp
            rbp = rsp;
            // 50a4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 50a6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 50a8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 50aa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 50ac push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 50ad push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 50ae mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // 50b1 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 50b4 mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // 50b7 mov rbx,rdi
            rbx = rdi;
            // 50ba mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 50bd call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50c3 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 50c6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 50c8 cmove r14d,r15d
            if fl.e() { r14 = ((r15 as u32 as u64)) as u32 as u64; } else { r14 = r14 as u32 as u64; }
            // 50cc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 50cf mov rdi,rbx
            rdi = rbx;
            // 50d2 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 50d5 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50db test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 50dd jne 0x512f
            if fl.ne() { pc = 0x512f; continue; }
            pc = 0x50df;
        }
        0x50df => {
            // block 50df
            #[cfg(feature = "trace")] eprintln!("f_000050a0 pc 50df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 50df mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 50e2 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 50e6 mov rdi,rbx
            rdi = rbx;
            // 50e9 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 50ec call QWORD PTR [rax+0x3e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x3e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 50f2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 50f5 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 50f9 mov rdi,rbx
            rdi = rbx;
            // 50fc mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 50ff call QWORD PTR [rax+0x380]
            { let r = { let t = m.r64(rax.wrapping_add(0x380)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5105 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5108 mov rax,QWORD PTR [rax+0x5c8]
            rax = m.r64(rax.wrapping_add(0x5c8)) as u64;
            // 510f test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // 5112 je 0x511b
            if fl.e() { pc = 0x511b; continue; }
            pc = 0x5114;
        }
        0x5114 => {
            // block 5114
            #[cfg(feature = "trace")] eprintln!("f_000050a0 pc 5114 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5114 movzx r15d,r12b
            r15 = ((r12 as u8 as u64)) as u32 as u64;
            // 5118 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            pc = 0x511b;
        }
        0x511b => {
            // block 511b
            #[cfg(feature = "trace")] eprintln!("f_000050a0 pc 511b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 511b mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 511e mov ecx,DWORD PTR [rbp-0x30]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 5121 mov rdi,rbx
            rdi = rbx;
            // 5124 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 5127 mov r8d,r14d
            r8 = ((r14 as u32 as u64)) as u32 as u64;
            // 512a call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 512c mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x512f;
        }
        0x512f => {
            // block 512f
            #[cfg(feature = "trace")] eprintln!("f_000050a0 pc 512f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 512f mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 5132 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5136 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5137 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5139 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 513b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 513d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 513f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5140 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4dfc size=164
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004dfc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4dfc;
    loop {
        match pc {
        0x4dfc => {
            // block 4dfc
            #[cfg(feature = "trace")] eprintln!("f_00004dfc pc 4dfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dfc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4dfd mov rbp,rsp
            rbp = rsp;
            // 4e00 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4e02 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4e04 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4e06 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4e07 mov r14,rcx
            r14 = rcx;
            // 4e0a mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 4e0c mov r12,rdi
            r12 = rdi;
            // 4e0f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4e13 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e19 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4e1b je 0x4e22
            if fl.e() { pc = 0x4e22; continue; }
            pc = 0x4e1d;
        }
        0x4e1d => {
            // block 4e1d
            #[cfg(feature = "trace")] eprintln!("f_00004dfc pc 4e1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e1d xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 4e20 jmp 0x4e94
            pc = 0x4e94; continue;
        }
        0x4e22 => {
            // block 4e22
            #[cfg(feature = "trace")] eprintln!("f_00004dfc pc 4e22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e22 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4e26 mov ebx,ebx
            rbx = ((rbx as u32 as u64)) as u32 as u64;
            // 4e28 lea rcx,[rip+0xfa4c1] # 0xff2f0
            rcx = m.env.data_va(80573);
            // 4e2f mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4e32 mov edx,DWORD PTR [r14]
            rdx = (m.r32(r14) as u64) as u32 as u64;
            // 4e35 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 4e38 mov ecx,0xff
            rcx = (0xffu64) as u32 as u64;
            // 4e3d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4e40 mov rdi,r12
            rdi = r12;
            // 4e43 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e46 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4e48 je 0x4e94
            if fl.e() { pc = 0x4e94; continue; }
            pc = 0x4e4a;
        }
        0x4e4a => {
            // block 4e4a
            #[cfg(feature = "trace")] eprintln!("f_00004dfc pc 4e4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e4a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4e4e lea rcx,[rip+0xfa4cb] # 0xff320
            rcx = m.env.data_va(80621);
            // 4e55 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4e58 mov edx,DWORD PTR [r14+0x4]
            rdx = (m.r32(r14.wrapping_add(0x4)) as u64) as u32 as u64;
            // 4e5c xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 4e5f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4e64 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4e67 mov rdi,r12
            rdi = r12;
            // 4e6a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e6d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4e6f je 0x4e94
            if fl.e() { pc = 0x4e94; continue; }
            pc = 0x4e71;
        }
        0x4e71 => {
            // block 4e71
            #[cfg(feature = "trace")] eprintln!("f_00004dfc pc 4e71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e71 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4e75 lea rcx,[rip+0xfa4d4] # 0xff350
            rcx = m.env.data_va(80669);
            // 4e7c mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4e7f mov edx,DWORD PTR [r14+0x8]
            rdx = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // 4e83 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4e88 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4e8b mov rdi,r12
            rdi = r12;
            // 4e8e call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4e91 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x4e94;
        }
        0x4e94 => {
            // block 4e94
            #[cfg(feature = "trace")] eprintln!("f_00004dfc pc 4e94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4e94 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 4e97 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4e98 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4e9a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4e9c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4e9e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4e9f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29d86 size=164
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029d86<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x29d86;
    loop {
        match pc {
        0x29d86 => {
            // block 29d86
            #[cfg(feature = "trace")] eprintln!("f_00029d86 pc 29d86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29d86 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29d87 mov rbp,rsp
            rbp = rsp;
            // 29d8a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 29d8b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 29d8c mov rbx,rdi
            rbx = rdi;
            // 29d8f movabs rax,0x0
            rax = 0x0u64;
            // 29d99 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 29d9c mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 29da4 movabs rax,0x800000050
            rax = 0x0_0008_0000_0050u64;
            // 29dae mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 29db2 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 29db6 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 29dbe mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 29dc6 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 29dce mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 29dd6 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 29dde mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 29de6 mov esi,0xe0
            rsi = (0xe0u64) as u32 as u64;
            // 29deb call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29df0 mov rax,QWORD PTR [rbx+0x20]
            rax = m.r64(rbx.wrapping_add(0x20)) as u64;
            // 29df4 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // 29df6 test rax,rax
            fl = logic::<64>(rax & rax);
            // 29df9 je 0x29e0a
            if fl.e() { pc = 0x29e0a; continue; }
            pc = 0x29dfb;
        }
        0x29dfb => {
            // block 29dfb
            #[cfg(feature = "trace")] eprintln!("f_00029d86 pc 29dfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29dfb mov esi,DWORD PTR [rbx+0x28]
            rsi = (m.r32(rbx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 29dfe mov rdi,rax
            rdi = rax;
            // 29e01 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29e06 mov rdi,QWORD PTR [rbx+0x20]
            rdi = m.r64(rbx.wrapping_add(0x20)) as u64;
            pc = 0x29e0a;
        }
        0x29e0a => {
            // block 29e0a
            #[cfg(feature = "trace")] eprintln!("f_00029d86 pc 29e0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29e0a movabs rax,0x6e74763200000000
            rax = 0x6e74_7632_0000_0000u64;
            // 29e14 mov QWORD PTR [rbx+0x48],rax
            m.w64(rbx.wrapping_add(0x48), (rax) as u64);
            // 29e18 mov esi,0xe0
            rsi = (0xe0u64) as u32 as u64;
            // 29e1d call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29e22 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 29e26 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29e27 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29e28 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb95b4 size=164
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b95b4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb95b4;
    loop {
        match pc {
        0xb95b4 => {
            // block b95b4
            #[cfg(feature = "trace")] eprintln!("f_000b95b4 pc b95b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b95b4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b95b5 mov rbp,rsp
            rbp = rsp;
            // b95b8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b95ba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b95bc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b95be push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b95bf mov rbx,rdx
            rbx = rdx;
            // b95c2 mov r12,rsi
            r12 = rsi;
            // b95c5 mov r14,rdi
            r14 = rdi;
            // b95c8 mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // b95d0 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // b95d8 mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // b95df mov rdi,r12
            rdi = r12;
            // b95e2 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b95e7 movzx r15d,BYTE PTR [rbx]
            r15 = (m.r8(rbx) as u64) as u32 as u64;
            // b95eb test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // b95ef jne 0xb95f6
            if fl.ne() { pc = 0xb95f6; continue; }
            pc = 0xb95f1;
        }
        0xb95f1 => {
            // block b95f1
            #[cfg(feature = "trace")] eprintln!("f_000b95b4 pc b95f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b95f1 shr r15,1
            { let (r, f) = shr::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // b95f4 jmp 0xb95fa
            pc = 0xb95fa; continue;
        }
        0xb95f6 => {
            // block b95f6
            #[cfg(feature = "trace")] eprintln!("f_000b95b4 pc b95f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b95f6 mov r15,QWORD PTR [rbx+0x8]
            r15 = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0xb95fa;
        }
        0xb95fa => {
            // block b95fa
            #[cfg(feature = "trace")] eprintln!("f_000b95b4 pc b95fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b95fa mov rcx,rax
            rcx = rax;
            // b95fd add rcx,r15
            { let (r, f) = add::<64>(rcx, r15);
              rcx = r;
              fl = f; }
            // b9600 mov rdi,r14
            rdi = r14;
            // b9603 mov rsi,r12
            rsi = r12;
            // b9606 mov rdx,rax
            rdx = rax;
            // b9609 call 0xebfea
            { let r = { let t = m.env.stub_va(0x7b7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b960e test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // b9611 jne 0xb9618
            if fl.ne() { pc = 0xb9618; continue; }
            pc = 0xb9613;
        }
        0xb9613 => {
            // block b9613
            #[cfg(feature = "trace")] eprintln!("f_000b95b4 pc b9613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9613 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // b9616 jmp 0xb961c
            pc = 0xb961c; continue;
        }
        0xb9618 => {
            // block b9618
            #[cfg(feature = "trace")] eprintln!("f_000b95b4 pc b9618 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9618 mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xb961c;
        }
        0xb961c => {
            // block b961c
            #[cfg(feature = "trace")] eprintln!("f_000b95b4 pc b961c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b961c mov rdi,r14
            rdi = r14;
            // b961f mov rsi,rbx
            rsi = rbx;
            // b9622 mov rdx,r15
            rdx = r15;
            // b9625 call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b962a mov rax,r14
            rax = r14;
            // b962d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b962e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b9630 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9632 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9634 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9635 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb9636 => {
            // block b9636
            #[cfg(feature = "trace")] eprintln!("f_000b95b4 pc b9636 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9636 mov rbx,rax
            rbx = rax;
            // b9639 mov rdi,r14
            rdi = r14;
            // b963c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9641 mov rdi,rbx
            rdi = rbx;
            // b9644 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2b8dc size=166
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002b8dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2b8dc;
    loop {
        match pc {
        0x2b8dc => {
            // block 2b8dc
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b8dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b8dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b8dd mov rbp,rsp
            rbp = rsp;
            // 2b8e0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2b8e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2b8e3 mov rbx,rsi
            rbx = rsi;
            // 2b8e6 mov r14,rdi
            r14 = rdi;
            // 2b8e9 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 2b8ed mov DWORD PTR [r14+0x20],eax
            m.w32(r14.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // 2b8f1 mov DWORD PTR [r14+0x4c],0x0
            m.w32(r14.wrapping_add(0x4c), (0x0u64) as u32);
            // 2b8f9 lea rdi,[r14+0x24]
            rdi = r14.wrapping_add(0x24);
            // 2b8fd shl rax,0x2
            { let (r, f) = shl::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 2b901 movabs rsi,0x3fffffffc
            rsi = 0x0_0003_ffff_fffcu64;
            // 2b90b and rsi,rax
            { let r = rsi & rax; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 2b90e call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b913 lea rdi,[r14+0x50]
            rdi = r14.wrapping_add(0x50);
            // 2b917 mov esi,DWORD PTR [r14+0x20]
            rsi = (m.r32(r14.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2b91b shl rsi,0x2
            { let (r, f) = shl::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 2b91f call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b924 lea rdi,[r14+0x78]
            rdi = r14.wrapping_add(0x78);
            // 2b928 mov esi,DWORD PTR [r14+0x20]
            rsi = (m.r32(r14.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2b92c shl rsi,0x2
            { let (r, f) = shl::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 2b930 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b935 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2b938 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2b93c cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 2b93f je 0x2b97b
            if fl.e() { pc = 0x2b97b; continue; }
            pc = 0x2b941;
        }
        0x2b941 => {
            // block 2b941
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b941 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b941 mov rcx,QWORD PTR [r14+0x24]
            rcx = m.r64(r14.wrapping_add(0x24)) as u64;
            // 2b945 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x2b947;
        }
        0x2b947 => {
            // block 2b947
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b947 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b947 mov esi,DWORD PTR [rax+0x1c]
            rsi = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 2b94a mov edi,edx
            rdi = ((rdx as u32 as u64)) as u32 as u64;
            // 2b94c mov DWORD PTR [rcx+rdi*4],esi
            m.w32(rcx.wrapping_add(rdi.wrapping_mul(4)), ((rsi as u32 as u64)) as u32);
            // 2b94f mov rsi,QWORD PTR [rax+0x8]
            rsi = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2b953 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2b956 je 0x2b965
            if fl.e() { pc = 0x2b965; continue; }
            pc = 0x2b958;
        }
        0x2b958 => {
            // block 2b958
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b958 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b958 mov rax,rsi
            rax = rsi;
            // 2b95b mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 2b95e test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2b961 jne 0x2b958
            if fl.ne() { pc = 0x2b958; continue; }
            pc = 0x2b963;
        }
        0x2b963 => {
            // block 2b963
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b963 jmp 0x2b974
            pc = 0x2b974; continue;
        }
        0x2b965 => {
            // block 2b965
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b965 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b965 mov rsi,rax
            rsi = rax;
            pc = 0x2b968;
        }
        0x2b968 => {
            // block 2b968
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b968 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // 2b96c cmp QWORD PTR [rax],rsi
            fl = sub::<64>(m.r64(rax) as u64, rsi).1;
            // 2b96f mov rsi,rax
            rsi = rax;
            // 2b972 jne 0x2b968
            if fl.ne() { pc = 0x2b968; continue; }
            pc = 0x2b974;
        }
        0x2b974 => {
            // block 2b974
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b974 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b974 inc edx
            { let (r, f) = inc::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // 2b976 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 2b979 jne 0x2b947
            if fl.ne() { pc = 0x2b947; continue; }
            pc = 0x2b97b;
        }
        0x2b97b => {
            // block 2b97b
            #[cfg(feature = "trace")] eprintln!("f_0002b8dc pc 2b97b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b97b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2b97d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2b97e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2b980 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b981 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x12041 size=167
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012041<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x12041;
    loop {
        match pc {
        0x12041 => {
            // block 12041
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12041 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12041 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12042 mov rbp,rsp
            rbp = rsp;
            // 12045 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12047 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12049 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1204b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1204d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1204e push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1204f mov r15,r8
            r15 = r8;
            // 12052 mov r14,rcx
            r14 = rcx;
            // 12055 mov rbx,rdx
            rbx = rdx;
            // 12058 mov r12,rsi
            r12 = rsi;
            // 1205b mov r13,rdi
            r13 = rdi;
            // 1205e jmp 0x12064
            pc = 0x12064; continue;
        }
        0x12060 => {
            // block 12060
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12060 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12060 add r15,0x38
            { let (r, f) = add::<64>(r15, 0x38u64);
              r15 = r;
              fl = f; }
            pc = 0x12064;
        }
        0x12064 => {
            // block 12064
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12064 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12064 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 12067 je 0x12099
            if fl.e() { pc = 0x12099; continue; }
            pc = 0x12069;
        }
        0x12069 => {
            // block 12069
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12069 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12069 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1206c je 0x120b8
            if fl.e() { pc = 0x120b8; continue; }
            pc = 0x1206e;
        }
        0x1206e => {
            // block 1206e
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 1206e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1206e mov eax,DWORD PTR [rbx+0x18]
            rax = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 12071 cmp eax,DWORD PTR [r13+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(r13.wrapping_add(0x18)) as u64).1;
            // 12075 jge 0x12088
            if fl.ge() { pc = 0x12088; continue; }
            pc = 0x12077;
        }
        0x12077 => {
            // block 12077
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12077 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12077 mov rdi,r15
            rdi = r15;
            // 1207a mov rsi,rbx
            rsi = rbx;
            // 1207d call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12082 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12086 jmp 0x12060
            pc = 0x12060; continue;
        }
        0x12088 => {
            // block 12088
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12088 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12088 mov rdi,r15
            rdi = r15;
            // 1208b mov rsi,r13
            rsi = r13;
            // 1208e call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12093 add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 12097 jmp 0x12060
            pc = 0x12060; continue;
        }
        0x12099 => {
            // block 12099
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12099 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12099 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1209c je 0x120d9
            if fl.e() { pc = 0x120d9; continue; }
            pc = 0x1209e;
        }
        0x1209e => {
            // block 1209e
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 1209e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1209e mov rdi,r15
            rdi = r15;
            // 120a1 mov rsi,rbx
            rsi = rbx;
            // 120a4 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 120a9 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 120ad add r15,0x38
            { let (r, f) = add::<64>(r15, 0x38u64);
              r15 = r;
              fl = f; }
            // 120b1 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 120b4 jne 0x1209e
            if fl.ne() { pc = 0x1209e; continue; }
            pc = 0x120b6;
        }
        0x120b6 => {
            // block 120b6
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120b6 jmp 0x120d9
            pc = 0x120d9; continue;
        }
        0x120b8 => {
            // block 120b8
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120b8 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 120bb je 0x120d9
            if fl.e() { pc = 0x120d9; continue; }
            pc = 0x120bd;
        }
        0x120bd => {
            // block 120bd
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120bd sub r12,r13
            { let (r, f) = sub::<64>(r12, r13);
              r12 = r;
              fl = f; }
            // 120c0 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x120c2;
        }
        0x120c2 => {
            // block 120c2
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120c2 lea rsi,[r13+rbx*1+0x0]
            rsi = r13.wrapping_add(rbx);
            // 120c7 lea rdi,[r15+rbx*1]
            rdi = r15.wrapping_add(rbx);
            // 120cb call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 120d0 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 120d4 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 120d7 jne 0x120c2
            if fl.ne() { pc = 0x120c2; continue; }
            pc = 0x120d9;
        }
        0x120d9 => {
            // block 120d9
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120d9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 120dd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 120de pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 120e0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 120e2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 120e4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 120e6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 120e7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2254 size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002254<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2254;
    loop {
        match pc {
        0x2254 => {
            // block 2254
            #[cfg(feature = "trace")] eprintln!("f_00002254 pc 2254 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2254 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2255 mov rbp,rsp
            rbp = rsp;
            // 2258 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 225a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 225c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 225d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 225e mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 2261 mov rbx,rdi
            rbx = rdi;
            // 2264 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 2267 call 0xce188
            { let r = { let t = m.env.code_va(0xce188); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 226c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 226e je 0x22ea
            if fl.e() { pc = 0x22ea; continue; }
            pc = 0x2270;
        }
        0x2270 => {
            // block 2270
            #[cfg(feature = "trace")] eprintln!("f_00002254 pc 2270 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2270 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2273 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 2277 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 227a mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 227f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2284 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2287 mov rdi,rbx
            rdi = rbx;
            // 228a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 228d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 228f je 0x22ed
            if fl.e() { pc = 0x22ed; continue; }
            pc = 0x2291;
        }
        0x2291 => {
            // block 2291
            #[cfg(feature = "trace")] eprintln!("f_00002254 pc 2291 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2291 mov eax,0xdfcfffff
            rax = (0xdfcfffffu64) as u32 as u64;
            // 2296 and eax,DWORD PTR [rbp-0x1c]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2299 shl r14d,0x14
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x14u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 229d or r14d,eax
            { let r = (r14 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 22a0 or r14d,0x20000000
            { let r = (r14 as u32 as u64) | 0x20000000u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 22a7 mov DWORD PTR [rbp-0x1c],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((r14 as u32 as u64)) as u32);
            // 22ab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 22ae xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 22b1 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 22b6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 22bb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 22be mov rdi,rbx
            rdi = rbx;
            // 22c1 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 22c4 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22c7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 22c9 je 0x22ed
            if fl.e() { pc = 0x22ed; continue; }
            pc = 0x22cb;
        }
        0x22cb => {
            // block 22cb
            #[cfg(feature = "trace")] eprintln!("f_00002254 pc 22cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22cb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 22ce lea rsi,[rbx+0x24c]
            rsi = rbx.wrapping_add(0x24c);
            // 22d5 lea rdx,[rbx+0x248]
            rdx = rbx.wrapping_add(0x248);
            // 22dc mov rdi,rbx
            rdi = rbx;
            // 22df call QWORD PTR [rax+0x1d60]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d60)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22e5 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 22e8 jmp 0x22ed
            pc = 0x22ed; continue;
        }
        0x22ea => {
            // block 22ea
            #[cfg(feature = "trace")] eprintln!("f_00002254 pc 22ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22ea xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x22ed;
        }
        0x22ed => {
            // block 22ed
            #[cfg(feature = "trace")] eprintln!("f_00002254 pc 22ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22ed mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 22f0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 22f4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 22f5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 22f7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 22f9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 22fa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4d54 size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004d54<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4d54;
    loop {
        match pc {
        0x4d54 => {
            // block 4d54
            #[cfg(feature = "trace")] eprintln!("f_00004d54 pc 4d54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d54 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4d55 mov rbp,rsp
            rbp = rsp;
            // 4d58 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4d5a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4d5c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4d5e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4d5f mov r14,rcx
            r14 = rcx;
            // 4d62 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 4d64 mov r12,rdi
            r12 = rdi;
            // 4d67 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4d6b call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d71 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4d73 je 0x4d7a
            if fl.e() { pc = 0x4d7a; continue; }
            pc = 0x4d75;
        }
        0x4d75 => {
            // block 4d75
            #[cfg(feature = "trace")] eprintln!("f_00004d54 pc 4d75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d75 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 4d78 jmp 0x4def
            pc = 0x4def; continue;
        }
        0x4d7a => {
            // block 4d7a
            #[cfg(feature = "trace")] eprintln!("f_00004d54 pc 4d7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4d7a mov ebx,ebx
            rbx = ((rbx as u32 as u64)) as u32 as u64;
            // 4d7c lea rax,[rip+0xfa56d] # 0xff2f0
            rax = m.env.data_va(80573);
            // 4d83 mov esi,DWORD PTR [rax+rbx*4]
            rsi = (m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4d86 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4d8a xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 4d8d mov ecx,0xff
            rcx = (0xffu64) as u32 as u64;
            // 4d92 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4d95 mov rdi,r12
            rdi = r12;
            // 4d98 mov rdx,r14
            rdx = r14;
            // 4d9b call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4d9e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4da0 je 0x4def
            if fl.e() { pc = 0x4def; continue; }
            pc = 0x4da2;
        }
        0x4da2 => {
            // block 4da2
            #[cfg(feature = "trace")] eprintln!("f_00004d54 pc 4da2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4da2 lea rax,[rip+0xfa577] # 0xff320
            rax = m.env.data_va(80621);
            // 4da9 mov esi,DWORD PTR [rax+rbx*4]
            rsi = (m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4dac lea rdx,[r14+0x4]
            rdx = r14.wrapping_add(0x4);
            // 4db0 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4db4 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 4db7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4dbc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4dbf mov rdi,r12
            rdi = r12;
            // 4dc2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dc5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4dc7 je 0x4def
            if fl.e() { pc = 0x4def; continue; }
            pc = 0x4dc9;
        }
        0x4dc9 => {
            // block 4dc9
            #[cfg(feature = "trace")] eprintln!("f_00004d54 pc 4dc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4dc9 lea rax,[rip+0xfa580] # 0xff350
            rax = m.env.data_va(80669);
            // 4dd0 mov esi,DWORD PTR [rax+rbx*4]
            rsi = (m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4dd3 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 4dd7 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 4ddb mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4de0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4de3 mov rdi,r12
            rdi = r12;
            // 4de6 mov rdx,r14
            rdx = r14;
            // 4de9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4dec mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x4def;
        }
        0x4def => {
            // block 4def
            #[cfg(feature = "trace")] eprintln!("f_00004d54 pc 4def rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4def mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 4df2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4df3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4df5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4df7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4df9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4dfa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bef0 size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bef0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2bef0;
    loop {
        match pc {
        0x2bef0 => {
            // block 2bef0
            #[cfg(feature = "trace")] eprintln!("f_0002bef0 pc 2bef0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bef0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bef1 mov rbp,rsp
            rbp = rsp;
            // 2bef4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bef6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bef8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bef9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2befa mov r15,rsi
            r15 = rsi;
            // 2befd mov r14,rdi
            r14 = rdi;
            // 2bf00 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2bf04 sub rax,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rax, m.r64(r15) as u64);
              rax = r;
              fl = f; }
            // 2bf07 movabs rsi,0xffffffff0
            rsi = 0x0_000f_ffff_fff0u64;
            // 2bf11 and rsi,rax
            { let r = rsi & rax; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 2bf14 shr rax,0x4
            { let (r, f) = shr::<64>(rax, 0x4u64, fl); rax = r; fl = f; }
            // 2bf18 mov DWORD PTR [r14+0x20],eax
            m.w32(r14.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // 2bf1c mov DWORD PTR [r14+0x4c],0x0
            m.w32(r14.wrapping_add(0x4c), (0x0u64) as u32);
            // 2bf24 lea rdi,[r14+0x24]
            rdi = r14.wrapping_add(0x24);
            // 2bf28 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bf2d lea rdi,[r14+0x50]
            rdi = r14.wrapping_add(0x50);
            // 2bf31 mov esi,DWORD PTR [r14+0x20]
            rsi = (m.r32(r14.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2bf35 add rsi,rsi
            { let (r, f) = add::<64>(rsi, rsi);
              rsi = r;
              fl = f; }
            // 2bf38 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bf3d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2bf40 mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2bf44 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 2bf47 je 0x2bf8b
            if fl.e() { pc = 0x2bf8b; continue; }
            pc = 0x2bf49;
        }
        0x2bf49 => {
            // block 2bf49
            #[cfg(feature = "trace")] eprintln!("f_0002bef0 pc 2bf49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bf49 mov rcx,QWORD PTR [r14+0x24]
            rcx = m.r64(r14.wrapping_add(0x24)) as u64;
            // 2bf4d mov r8,QWORD PTR [r14+0x50]
            r8 = m.r64(r14.wrapping_add(0x50)) as u64;
            // 2bf51 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bf53;
        }
        0x2bf53 => {
            // block 2bf53
            #[cfg(feature = "trace")] eprintln!("f_0002bef0 pc 2bf53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bf53 test r8,r8
            fl = logic::<64>(r8 & r8);
            // 2bf56 je 0x2bf61
            if fl.e() { pc = 0x2bf61; continue; }
            pc = 0x2bf58;
        }
        0x2bf58 => {
            // block 2bf58
            #[cfg(feature = "trace")] eprintln!("f_0002bef0 pc 2bf58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bf58 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 2bf5a mov WORD PTR [r8+rbx*2],0x0
            m.w16(r8.wrapping_add(rbx.wrapping_mul(2)), (0x0u64) as u16);
            pc = 0x2bf61;
        }
        0x2bf61 => {
            // block 2bf61
            #[cfg(feature = "trace")] eprintln!("f_0002bef0 pc 2bf61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bf61 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 2bf64 je 0x2bf82
            if fl.e() { pc = 0x2bf82; continue; }
            pc = 0x2bf66;
        }
        0x2bf66 => {
            // block 2bf66
            #[cfg(feature = "trace")] eprintln!("f_0002bef0 pc 2bf66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bf66 mov edi,esi
            rdi = ((rsi as u32 as u64)) as u32 as u64;
            // 2bf68 inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 2bf6a shl rdi,0x4
            { let (r, f) = shl::<64>(rdi, 0x4u64, fl); rdi = r; fl = f; }
            // 2bf6e mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            // 2bf71 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2bf75 mov QWORD PTR [rcx+rdi*1+0x8],rdx
            m.w64(rcx.wrapping_add(rdi).wrapping_add(0x8), (rdx) as u64);
            // 2bf7a mov QWORD PTR [rcx+rdi*1],rbx
            m.w64(rcx.wrapping_add(rdi), (rbx) as u64);
            // 2bf7e mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x2bf82;
        }
        0x2bf82 => {
            // block 2bf82
            #[cfg(feature = "trace")] eprintln!("f_0002bef0 pc 2bf82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bf82 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 2bf86 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 2bf89 jne 0x2bf53
            if fl.ne() { pc = 0x2bf53; continue; }
            pc = 0x2bf8b;
        }
        0x2bf8b => {
            // block 2bf8b
            #[cfg(feature = "trace")] eprintln!("f_0002bef0 pc 2bf8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bf8b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2bf8d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2bf91 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bf92 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bf94 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2bf96 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bf97 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5bea size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5bea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5bea;
    loop {
        match pc {
        0xb5bea => {
            // block b5bea
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5bea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5bea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5beb mov rbp,rsp
            rbp = rsp;
            // b5bee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5bf0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5bf2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5bf4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5bf6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5bf7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b5bf8 mov r13,rsi
            r13 = rsi;
            // b5bfb mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b5bff lea r14,[rdi+0x8]
            r14 = rdi.wrapping_add(0x8);
            // b5c03 mov rcx,r14
            rcx = r14;
            // b5c06 call 0xebbd0
            { let r = { let t = m.env.stub_va(0x39d); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5c0b mov r15,rax
            r15 = rax;
            // b5c0e cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // b5c11 je 0xb5c7d
            if fl.e() { pc = 0xb5c7d; continue; }
            pc = 0xb5c13;
        }
        0xb5c13 => {
            // block b5c13
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c13 lea rsi,[r15+0x20]
            rsi = r15.wrapping_add(0x20);
            // b5c17 movzx r12d,BYTE PTR [r13+0x0]
            r12 = (m.r8(r13) as u64) as u32 as u64;
            // b5c1c mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // b5c1f and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5c21 jne 0xb5c28
            if fl.ne() { pc = 0xb5c28; continue; }
            pc = 0xb5c23;
        }
        0xb5c23 => {
            // block b5c23
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c23 shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // b5c26 jmp 0xb5c2c
            pc = 0xb5c2c; continue;
        }
        0xb5c28 => {
            // block b5c28
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c28 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0xb5c2c;
        }
        0xb5c2c => {
            // block b5c2c
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c2c movzx ebx,BYTE PTR [rsi]
            rbx = (m.r8(rsi) as u64) as u32 as u64;
            // b5c2f mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // b5c31 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5c34 jne 0xb5c3b
            if fl.ne() { pc = 0xb5c3b; continue; }
            pc = 0xb5c36;
        }
        0xb5c36 => {
            // block b5c36
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c36 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // b5c39 jmp 0xb5c3f
            pc = 0xb5c3f; continue;
        }
        0xb5c3b => {
            // block b5c3b
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c3b mov rbx,QWORD PTR [r15+0x28]
            rbx = m.r64(r15.wrapping_add(0x28)) as u64;
            pc = 0xb5c3f;
        }
        0xb5c3f => {
            // block b5c3f
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c3f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b5c41 jne 0xb5c48
            if fl.ne() { pc = 0xb5c48; continue; }
            pc = 0xb5c43;
        }
        0xb5c43 => {
            // block b5c43
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c43 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // b5c46 jmp 0xb5c4c
            pc = 0xb5c4c; continue;
        }
        0xb5c48 => {
            // block b5c48
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c48 mov r13,QWORD PTR [r13+0x10]
            r13 = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0xb5c4c;
        }
        0xb5c4c => {
            // block b5c4c
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c4c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // b5c4e jne 0xb5c55
            if fl.ne() { pc = 0xb5c55; continue; }
            pc = 0xb5c50;
        }
        0xb5c50 => {
            // block b5c50
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c50 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // b5c53 jmp 0xb5c59
            pc = 0xb5c59; continue;
        }
        0xb5c55 => {
            // block b5c55
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c55 mov rsi,QWORD PTR [r15+0x30]
            rsi = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xb5c59;
        }
        0xb5c59 => {
            // block b5c59
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c59 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // b5c5c mov rdx,r12
            rdx = r12;
            // b5c5f cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // b5c63 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5c66 je 0xb5c78
            if fl.e() { pc = 0xb5c78; continue; }
            pc = 0xb5c68;
        }
        0xb5c68 => {
            // block b5c68
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c68 mov rdi,r13
            rdi = r13;
            // b5c6b call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5c70 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b5c72 je 0xb5c78
            if fl.e() { pc = 0xb5c78; continue; }
            pc = 0xb5c74;
        }
        0xb5c74 => {
            // block b5c74
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c74 jns 0xb5c80
            if fl.ns() { pc = 0xb5c80; continue; }
            pc = 0xb5c76;
        }
        0xb5c76 => {
            // block b5c76
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c76 jmp 0xb5c7d
            pc = 0xb5c7d; continue;
        }
        0xb5c78 => {
            // block b5c78
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c78 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // b5c7b jae 0xb5c80
            if fl.ae() { pc = 0xb5c80; continue; }
            pc = 0xb5c7d;
        }
        0xb5c7d => {
            // block b5c7d
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c7d mov r15,r14
            r15 = r14;
            pc = 0xb5c80;
        }
        0xb5c80 => {
            // block b5c80
            #[cfg(feature = "trace")] eprintln!("f_000b5bea pc b5c80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5c80 mov rax,r15
            rax = r15;
            // b5c83 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b5c87 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5c88 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b5c8a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b5c8c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5c8e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5c90 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5c91 ret
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

/// parity: P2v @0xc49e6 size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c49e6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc49e6;
    loop {
        match pc {
        0xc49e6 => {
            // block c49e6
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c49e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c49e6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c49e7 mov rbp,rsp
            rbp = rsp;
            // c49ea push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c49ec push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c49ee push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c49f0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c49f2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c49f3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c49f4 mov r13,rsi
            r13 = rsi;
            // c49f7 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c49fb lea r14,[rdi+0x8]
            r14 = rdi.wrapping_add(0x8);
            // c49ff mov rcx,r14
            rcx = r14;
            // c4a02 call 0xebe76
            { let r = { let t = m.env.stub_va(0x643); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4a07 mov r15,rax
            r15 = rax;
            // c4a0a cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // c4a0d je 0xc4a79
            if fl.e() { pc = 0xc4a79; continue; }
            pc = 0xc4a0f;
        }
        0xc4a0f => {
            // block c4a0f
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a0f lea rsi,[r15+0x20]
            rsi = r15.wrapping_add(0x20);
            // c4a13 movzx r12d,BYTE PTR [r13+0x0]
            r12 = (m.r8(r13) as u64) as u32 as u64;
            // c4a18 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // c4a1b and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c4a1d jne 0xc4a24
            if fl.ne() { pc = 0xc4a24; continue; }
            pc = 0xc4a1f;
        }
        0xc4a1f => {
            // block c4a1f
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a1f shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // c4a22 jmp 0xc4a28
            pc = 0xc4a28; continue;
        }
        0xc4a24 => {
            // block c4a24
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a24 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0xc4a28;
        }
        0xc4a28 => {
            // block c4a28
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a28 movzx ebx,BYTE PTR [rsi]
            rbx = (m.r8(rsi) as u64) as u32 as u64;
            // c4a2b mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // c4a2d and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c4a30 jne 0xc4a37
            if fl.ne() { pc = 0xc4a37; continue; }
            pc = 0xc4a32;
        }
        0xc4a32 => {
            // block c4a32
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a32 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // c4a35 jmp 0xc4a3b
            pc = 0xc4a3b; continue;
        }
        0xc4a37 => {
            // block c4a37
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a37 mov rbx,QWORD PTR [r15+0x28]
            rbx = m.r64(r15.wrapping_add(0x28)) as u64;
            pc = 0xc4a3b;
        }
        0xc4a3b => {
            // block c4a3b
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a3b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c4a3d jne 0xc4a44
            if fl.ne() { pc = 0xc4a44; continue; }
            pc = 0xc4a3f;
        }
        0xc4a3f => {
            // block c4a3f
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a3f inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // c4a42 jmp 0xc4a48
            pc = 0xc4a48; continue;
        }
        0xc4a44 => {
            // block c4a44
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a44 mov r13,QWORD PTR [r13+0x10]
            r13 = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0xc4a48;
        }
        0xc4a48 => {
            // block c4a48
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a48 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // c4a4a jne 0xc4a51
            if fl.ne() { pc = 0xc4a51; continue; }
            pc = 0xc4a4c;
        }
        0xc4a4c => {
            // block c4a4c
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a4c inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // c4a4f jmp 0xc4a55
            pc = 0xc4a55; continue;
        }
        0xc4a51 => {
            // block c4a51
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a51 mov rsi,QWORD PTR [r15+0x30]
            rsi = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xc4a55;
        }
        0xc4a55 => {
            // block c4a55
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a55 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // c4a58 mov rdx,r12
            rdx = r12;
            // c4a5b cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // c4a5f test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4a62 je 0xc4a74
            if fl.e() { pc = 0xc4a74; continue; }
            pc = 0xc4a64;
        }
        0xc4a64 => {
            // block c4a64
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a64 mov rdi,r13
            rdi = r13;
            // c4a67 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4a6c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c4a6e je 0xc4a74
            if fl.e() { pc = 0xc4a74; continue; }
            pc = 0xc4a70;
        }
        0xc4a70 => {
            // block c4a70
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a70 jns 0xc4a7c
            if fl.ns() { pc = 0xc4a7c; continue; }
            pc = 0xc4a72;
        }
        0xc4a72 => {
            // block c4a72
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a72 jmp 0xc4a79
            pc = 0xc4a79; continue;
        }
        0xc4a74 => {
            // block c4a74
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a74 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // c4a77 jae 0xc4a7c
            if fl.ae() { pc = 0xc4a7c; continue; }
            pc = 0xc4a79;
        }
        0xc4a79 => {
            // block c4a79
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a79 mov r15,r14
            r15 = r14;
            pc = 0xc4a7c;
        }
        0xc4a7c => {
            // block c4a7c
            #[cfg(feature = "trace")] eprintln!("f_000c49e6 pc c4a7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a7c mov rax,r15
            rax = r15;
            // c4a7f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c4a83 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4a84 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c4a86 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c4a88 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4a8a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c4a8c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4a8d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea4f0 size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ea4f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xea4f0;
    loop {
        match pc {
        0xea4f0 => {
            // block ea4f0
            #[cfg(feature = "trace")] eprintln!("f_000ea4f0 pc ea4f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea4f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea4f1 mov rbp,rsp
            rbp = rsp;
            // ea4f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ea4f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea4f8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea4f9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ea4fa mov r15,rdi
            r15 = rdi;
            // ea4fd lea rax,[rip+0x5c4fc] # 0x146a00
            rax = m.env.data_va(373197);
            // ea504 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // ea507 call 0xeafd6
            { let r = { let t = m.env.code_va(0xeafd6); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea50c mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // ea510 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // ea514 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // ea517 je 0xea52b
            if fl.e() { pc = 0xea52b; continue; }
            pc = 0xea519;
        }
        0xea519 => {
            // block ea519
            #[cfg(feature = "trace")] eprintln!("f_000ea4f0 pc ea519 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea519 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            // ea51d call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea522 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // ea526 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // ea529 jne 0xea519
            if fl.ne() { pc = 0xea519; continue; }
            pc = 0xea52b;
        }
        0xea52b => {
            // block ea52b
            #[cfg(feature = "trace")] eprintln!("f_000ea4f0 pc ea52b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea52b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // ea52e mov rax,QWORD PTR [rax+0x88]
            rax = m.r64(rax.wrapping_add(0x88)) as u64;
            // ea535 mov rdi,r15
            rdi = r15;
            // ea538 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea53a lea rax,[rip+0x59c87] # 0x1441c8
            rax = m.env.data_va(362901);
            // ea541 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // ea545 mov QWORD PTR [r15+0x38],rax
            m.w64(r15.wrapping_add(0x38), (rax) as u64);
            // ea549 lea rdi,[r15+0x40]
            rdi = r15.wrapping_add(0x40);
            // ea54d mov rsi,QWORD PTR [r15+0x48]
            rsi = m.r64(r15.wrapping_add(0x48)) as u64;
            // ea551 call 0xebe22
            { let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea556 mov rdi,r15
            rdi = r15;
            // ea559 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ea55d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ea55e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ea560 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ea562 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea563 jmp 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xea568 => {
            // block ea568
            #[cfg(feature = "trace")] eprintln!("f_000ea4f0 pc ea568 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea568 mov r14,rax
            r14 = rax;
            // ea56b lea rax,[rip+0x59c56] # 0x1441c8
            rax = m.env.data_va(362901);
            // ea572 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // ea576 mov QWORD PTR [r15+0x38],rax
            m.w64(r15.wrapping_add(0x38), (rax) as u64);
            // ea57a lea rdi,[r15+0x40]
            rdi = r15.wrapping_add(0x40);
            // ea57e mov rsi,QWORD PTR [r15+0x48]
            rsi = m.r64(r15.wrapping_add(0x48)) as u64;
            // ea582 call 0xebe22
            { let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea587 mov rdi,r15
            rdi = r15;
            // ea58a call 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea58f mov rdi,r14
            rdi = r14;
            // ea592 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd432 size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d432<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd432;
    loop {
        match pc {
        0xd432 => {
            // block d432
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d432 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d432 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d433 mov rbp,rsp
            rbp = rsp;
            // d436 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d438 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d43a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d43c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d43d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d441 mov r14,rdx
            r14 = rdx;
            // d444 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d446 mov r12,rdi
            r12 = rdi;
            // d449 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d44e call 0xd0ea0
            { let r = { let t = m.env.code_va(0xd0ea0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d453 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d456 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d458 jbe 0xd4cd
            if fl.be() { pc = 0xd4cd; continue; }
            pc = 0xd45a;
        }
        0xd45a => {
            // block d45a
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d45a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d45a mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // d461 mov r15d,0x1
            r15 = (0x1u64) as u32 as u64;
            // d467 test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d46a je 0xd47a
            if fl.e() { pc = 0xd47a; continue; }
            pc = 0xd46c;
        }
        0xd46c => {
            // block d46c
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d46c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d46c movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d46f mov r15d,0x9
            r15 = (0x9u64) as u32 as u64;
            // d475 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d478 jne 0xd4c9
            if fl.ne() { pc = 0xd4c9; continue; }
            pc = 0xd47a;
        }
        0xd47a => {
            // block d47a
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d47a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d47a mov BYTE PTR [rbp-0x25],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), (0x0u64) as u8);
            // d47e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d482 lea rsi,[rbp-0x25]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdb);
            // d486 mov rdi,r12
            rdi = r12;
            // d489 call QWORD PTR [rax+0x1820]
            { let r = { let t = m.r64(rax.wrapping_add(0x1820)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d48f mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d491 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d493 cmp BYTE PTR [rbp-0x25],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb)) as u64, 0x0u64).1;
            // d497 je 0xd4cf
            if fl.e() { pc = 0xd4cf; continue; }
            pc = 0xd499;
        }
        0xd499 => {
            // block d499
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d499 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d499 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d49c jne 0xd4cf
            if fl.ne() { pc = 0xd4cf; continue; }
            pc = 0xd49e;
        }
        0xd49e => {
            // block d49e
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d49e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d4a2 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // d4a6 mov esi,0xe9
            rsi = (0xe9u64) as u32 as u64;
            // d4ab mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // d4b0 mov rdi,r12
            rdi = r12;
            // d4b3 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // d4b6 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4b9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d4bb je 0xd4cd
            if fl.e() { pc = 0xd4cd; continue; }
            pc = 0xd4bd;
        }
        0xd4bd => {
            // block d4bd
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d4bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4bd mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // d4c0 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d4c2 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // d4c5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d4c7 jmp 0xd4cf
            pc = 0xd4cf; continue;
        }
        0xd4c9 => {
            // block d4c9
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d4c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4c9 mov BYTE PTR [rbp-0x25],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), (0x0u64) as u8);
            pc = 0xd4cd;
        }
        0xd4cd => {
            // block d4cd
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d4cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd4cf;
        }
        0xd4cf => {
            // block d4cf
            #[cfg(feature = "trace")] eprintln!("f_0000d432 pc d4cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4cf add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d4d3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d4d4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d4d6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d4d8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d4da pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4db ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd61a size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d61a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd61a;
    loop {
        match pc {
        0xd61a => {
            // block d61a
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d61a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d61a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d61b mov rbp,rsp
            rbp = rsp;
            // d61e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d620 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d622 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d624 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d625 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d629 mov r14,rdx
            r14 = rdx;
            // d62c mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d62e mov r12,rdi
            r12 = rdi;
            // d631 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d636 call 0xd0fb9
            { let r = { let t = m.env.code_va(0xd0fb9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d63b movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d63e cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d640 jbe 0xd6b5
            if fl.be() { pc = 0xd6b5; continue; }
            pc = 0xd642;
        }
        0xd642 => {
            // block d642
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d642 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // d649 mov r15d,0x10
            r15 = (0x10u64) as u32 as u64;
            // d64f test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d652 je 0xd662
            if fl.e() { pc = 0xd662; continue; }
            pc = 0xd654;
        }
        0xd654 => {
            // block d654
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d654 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d654 movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d657 mov r15d,0x14
            r15 = (0x14u64) as u32 as u64;
            // d65d cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d660 jne 0xd6b1
            if fl.ne() { pc = 0xd6b1; continue; }
            pc = 0xd662;
        }
        0xd662 => {
            // block d662
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d662 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d662 mov BYTE PTR [rbp-0x25],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), (0x0u64) as u8);
            // d666 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d66a lea rsi,[rbp-0x25]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdb);
            // d66e mov rdi,r12
            rdi = r12;
            // d671 call QWORD PTR [rax+0x1820]
            { let r = { let t = m.r64(rax.wrapping_add(0x1820)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d677 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d679 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d67b cmp BYTE PTR [rbp-0x25],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb)) as u64, 0x0u64).1;
            // d67f je 0xd6b7
            if fl.e() { pc = 0xd6b7; continue; }
            pc = 0xd681;
        }
        0xd681 => {
            // block d681
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d681 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d681 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d684 jne 0xd6b7
            if fl.ne() { pc = 0xd6b7; continue; }
            pc = 0xd686;
        }
        0xd686 => {
            // block d686
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d686 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d686 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d68a lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // d68e mov esi,0xe9
            rsi = (0xe9u64) as u32 as u64;
            // d693 mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // d698 mov rdi,r12
            rdi = r12;
            // d69b mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // d69e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6a3 je 0xd6b5
            if fl.e() { pc = 0xd6b5; continue; }
            pc = 0xd6a5;
        }
        0xd6a5 => {
            // block d6a5
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a5 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // d6a8 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d6aa mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // d6ad mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d6af jmp 0xd6b7
            pc = 0xd6b7; continue;
        }
        0xd6b1 => {
            // block d6b1
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b1 mov BYTE PTR [rbp-0x25],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), (0x0u64) as u8);
            pc = 0xd6b5;
        }
        0xd6b5 => {
            // block d6b5
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd6b7;
        }
        0xd6b7 => {
            // block d6b7
            #[cfg(feature = "trace")] eprintln!("f_0000d61a pc d6b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d6bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6bc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6be pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6c0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d6c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6c3 ret
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

/// parity: P2v @0x27bf8 size=171
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00027bf8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x27bf8;
    loop {
        match pc {
        0x27bf8 => {
            // block 27bf8
            #[cfg(feature = "trace")] eprintln!("f_00027bf8 pc 27bf8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27bf8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 27bf9 mov rbp,rsp
            rbp = rsp;
            // 27bfc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 27bfe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 27bff mov rbx,rsi
            rbx = rsi;
            // 27c02 mov r14,rdi
            r14 = rdi;
            // 27c05 cmp DWORD PTR [rbx],0x2
            fl = sub::<32>(m.r32(rbx) as u64, 0x2u64).1;
            // 27c08 jb 0x27c87
            if fl.b() { pc = 0x27c87; continue; }
            pc = 0x27c0a;
        }
        0x27c0a => {
            // block 27c0a
            #[cfg(feature = "trace")] eprintln!("f_00027bf8 pc 27c0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27c0a lea rsi,[rip+0x102a1c] # 0x12a62d
            rsi = m.env.data_va(257530);
            // 27c11 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 27c16 mov rdi,r14
            rdi = r14;
            // 27c19 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27c1e mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 27c20 mov rdi,rax
            rdi = rax;
            // 27c23 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27c28 lea rsi,[rip+0x102a04] # 0x12a633
            rsi = m.env.data_va(257536);
            // 27c2f mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 27c34 mov rdi,rax
            rdi = rax;
            // 27c37 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27c3c mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 27c3f mov rdi,rax
            rdi = rax;
            // 27c42 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27c47 lea rsi,[rip+0x1029f1] # 0x12a63f
            rsi = m.env.data_va(257548);
            // 27c4e mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 27c53 mov rdi,rax
            rdi = rax;
            // 27c56 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27c5b mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 27c5e mov rdi,rax
            rdi = rax;
            // 27c61 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27c66 lea rsi,[rip+0x1029e0] # 0x12a64d
            rsi = m.env.data_va(257562);
            // 27c6d mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 27c72 mov rdi,rax
            rdi = rax;
            // 27c75 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27c7a mov esi,DWORD PTR [rbx+0xc]
            rsi = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // 27c7d mov rdi,rax
            rdi = rax;
            // 27c80 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27c85 jmp 0x27c9b
            pc = 0x27c9b; continue;
        }
        0x27c87 => {
            // block 27c87
            #[cfg(feature = "trace")] eprintln!("f_00027bf8 pc 27c87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27c87 lea rsi,[rip+0x1029cc] # 0x12a65a
            rsi = m.env.data_va(257575);
            // 27c8e mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 27c93 mov rdi,r14
            rdi = r14;
            // 27c96 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x27c9b;
        }
        0x27c9b => {
            // block 27c9b
            #[cfg(feature = "trace")] eprintln!("f_00027bf8 pc 27c9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27c9b mov rax,r14
            rax = r14;
            // 27c9e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 27c9f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 27ca1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 27ca2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd386 size=172
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d386<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd386;
    loop {
        match pc {
        0xd386 => {
            // block d386
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d386 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d386 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d387 mov rbp,rsp
            rbp = rsp;
            // d38a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d38c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d38e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d390 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d391 mov r14,rdx
            r14 = rdx;
            // d394 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d396 mov r12,rdi
            r12 = rdi;
            // d399 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d39e call 0xd0ea0
            { let r = { let t = m.env.code_va(0xd0ea0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3a3 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d3a6 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d3a8 jbe 0xd3cb
            if fl.be() { pc = 0xd3cb; continue; }
            pc = 0xd3aa;
        }
        0xd3aa => {
            // block d3aa
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3aa test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d3ad je 0xd3cf
            if fl.e() { pc = 0xd3cf; continue; }
            pc = 0xd3af;
        }
        0xd3af => {
            // block d3af
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3af movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d3b2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3b4 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d3b7 mov esi,0xfe
            rsi = (0xfeu64) as u32 as u64;
            // d3bc cmovne esi,ecx
            if fl.ne() { rsi = ((rcx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // d3bf mov r15d,0xff
            r15 = (0xffu64) as u32 as u64;
            // d3c5 cmovne r15d,ecx
            if fl.ne() { r15 = ((rcx as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // d3c9 jmp 0xd3da
            pc = 0xd3da; continue;
        }
        0xd3cb => {
            // block d3cb
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cb xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3cd jmp 0xd426
            pc = 0xd426; continue;
        }
        0xd3cf => {
            // block d3cf
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cf mov r15d,0x71
            r15 = (0x71u64) as u32 as u64;
            // d3d5 mov esi,0x70
            rsi = (0x70u64) as u32 as u64;
            pc = 0xd3da;
        }
        0xd3da => {
            // block d3da
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3da mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // d3e1 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3e3 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // d3e5 je 0xd426
            if fl.e() { pc = 0xd426; continue; }
            pc = 0xd3e7;
        }
        0xd3e7 => {
            // block d3e7
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e7 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // d3ea je 0xd426
            if fl.e() { pc = 0xd426; continue; }
            pc = 0xd3ec;
        }
        0xd3ec => {
            // block d3ec
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3ec lea rdx,[r14+0x4]
            rdx = r14.wrapping_add(0x4);
            // d3f0 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d3f4 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3f6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d3fb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d3fe mov rdi,r12
            rdi = r12;
            // d401 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d404 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d406 je 0xd426
            if fl.e() { pc = 0xd426; continue; }
            pc = 0xd408;
        }
        0xd408 => {
            // block d408
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d408 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d408 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // d40c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d410 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d415 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d418 mov rdi,r12
            rdi = r12;
            // d41b mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d41e mov rdx,r14
            rdx = r14;
            // d421 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d424 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xd426;
        }
        0xd426 => {
            // block d426
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d426 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d426 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d428 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d429 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d42b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d42d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d42f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d430 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe409f size=173
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e409f<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe409f;
    loop {
        match pc {
        0xe409f => {
            // block e409f
            #[cfg(feature = "trace")] eprintln!("f_000e409f pc e409f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e409f push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e40a0 mov rbp,rsp
            rbp = rsp;
            // e40a3 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e40a5 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e40a7 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e40a9 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e40ab push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e40ac push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e40ad mov r14,rdx
            r14 = rdx;
            // e40b0 mov r12,rsi
            r12 = rsi;
            // e40b3 mov r15,rdi
            r15 = rdi;
            // e40b6 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // e40be mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // e40c6 mov QWORD PTR [r15],0x0
            m.w64(r15, (0x0u64) as u64);
            // e40cd movzx ebx,BYTE PTR [r12]
            rbx = (m.r8(r12) as u64) as u32 as u64;
            // e40d2 test bl,0x1
            fl = logic::<8>((rbx as u8 as u64) & 0x1u64);
            // e40d5 jne 0xe40ea
            if fl.ne() { pc = 0xe40ea; continue; }
            pc = 0xe40d7;
        }
        0xe40d7 => {
            // block e40d7
            #[cfg(feature = "trace")] eprintln!("f_000e409f pc e40d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e40d7 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // e40da mov rdi,r14
            rdi = r14;
            // e40dd call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e40e2 mov r13,rax
            r13 = rax;
            // e40e5 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // e40e8 jmp 0xe40ff
            pc = 0xe40ff; continue;
        }
        0xe40ea => {
            // block e40ea
            #[cfg(feature = "trace")] eprintln!("f_000e409f pc e40ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e40ea mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // e40ef mov rdi,r14
            rdi = r14;
            // e40f2 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e40f7 mov r13,rax
            r13 = rax;
            // e40fa mov r12,QWORD PTR [r12+0x10]
            r12 = m.r64(r12.wrapping_add(0x10)) as u64;
            pc = 0xe40ff;
        }
        0xe40ff => {
            // block e40ff
            #[cfg(feature = "trace")] eprintln!("f_000e409f pc e40ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e40ff lea rcx,[r13+rbx*1+0x0]
            rcx = r13.wrapping_add(rbx);
            // e4104 mov rdi,r15
            rdi = r15;
            // e4107 mov rsi,r12
            rsi = r12;
            // e410a mov rdx,rbx
            rdx = rbx;
            // e410d call 0xebfea
            { let r = { let t = m.env.stub_va(0x7b7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4112 mov rdi,r15
            rdi = r15;
            // e4115 mov rsi,r14
            rsi = r14;
            // e4118 mov rdx,r13
            rdx = r13;
            // e411b call 0xebffc
            { let r = { let t = m.env.stub_va(0x7c9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4120 mov rax,r15
            rax = r15;
            // e4123 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e4127 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4128 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e412a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e412c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e412e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4130 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4131 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe4132 => {
            // block e4132
            #[cfg(feature = "trace")] eprintln!("f_000e409f pc e4132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4132 mov rbx,rax
            rbx = rax;
            // e4135 mov rdi,r15
            rdi = r15;
            // e4138 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e413d mov rdi,rbx
            rdi = rbx;
            // e4140 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7e size=174
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000d7e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd7e;
    loop {
        match pc {
        0xd7e => {
            // block d7e
            #[cfg(feature = "trace")] eprintln!("f_00000d7e pc d7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7f mov rbp,rsp
            rbp = rsp;
            // d82 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d84 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d85 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d89 mov r14,rsi
            r14 = rsi;
            // d8c mov DWORD PTR [rbp-0x14],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0xcu64) as u32);
            // d93 mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // d9a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d9d lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // da1 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // da3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // da5 call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dab test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // dad je 0xe08
            if fl.e() { pc = 0xe08; continue; }
            pc = 0xdaf;
        }
        0xdaf => {
            // block daf
            #[cfg(feature = "trace")] eprintln!("f_00000d7e pc daf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // daf mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // db2 cmp rax,0x5
            fl = sub::<64>(rax, 0x5u64).1;
            // db6 jbe 0xdc3
            if fl.be() { pc = 0xdc3; continue; }
            pc = 0xdb8;
        }
        0xdb8 => {
            // block db8
            #[cfg(feature = "trace")] eprintln!("f_00000d7e pc db8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db8 mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // dbf xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // dc1 jmp 0xe08
            pc = 0xe08; continue;
        }
        0xdc3 => {
            // block dc3
            #[cfg(feature = "trace")] eprintln!("f_00000d7e pc dc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc3 lea rcx,[rip+0x4a] # 0xe14
            rcx = m.env.code_va(0xe14);
            // dca movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // dce add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // dd1 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xdd3 => {
            // block dd3
            #[cfg(feature = "trace")] eprintln!("f_00000d7e pc dd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dd3 mov DWORD PTR [r14],0x438
            m.w32(r14, (0x438u64) as u32);
            // dda mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // ddc jmp 0xe08
            pc = 0xe08; continue;
        }
        0xdde => {
            // block dde
            #[cfg(feature = "trace")] eprintln!("f_00000d7e pc dde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dde mov DWORD PTR [r14],0x2d0
            m.w32(r14, (0x2d0u64) as u32);
            // de5 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // de7 jmp 0xe08
            pc = 0xe08; continue;
        }
        0xde9 => {
            // block de9
            #[cfg(feature = "trace")] eprintln!("f_00000d7e pc de9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // de9 mov DWORD PTR [r14],0x1e6
            m.w32(r14, (0x1e6u64) as u32);
            // df0 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // df2 jmp 0xe08
            pc = 0xe08; continue;
        }
        0xdf4 => {
            // block df4
            #[cfg(feature = "trace")] eprintln!("f_00000d7e pc df4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df4 mov DWORD PTR [r14],0x240
            m.w32(r14, (0x240u64) as u32);
            // dfb mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // dfd jmp 0xe08
            pc = 0xe08; continue;
        }
        0xdff => {
            // block dff
            #[cfg(feature = "trace")] eprintln!("f_00000d7e pc dff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dff mov DWORD PTR [r14],0x614
            m.w32(r14, (0x614u64) as u32);
            // e06 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe08;
        }
        0xe08 => {
            // block e08
            #[cfg(feature = "trace")] eprintln!("f_00000d7e pc e08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e0a add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e11 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e12 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x45160 size=176
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00045160<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x45160;
    loop {
        match pc {
        0x45160 => {
            // block 45160
            #[cfg(feature = "trace")] eprintln!("f_00045160 pc 45160 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 45160 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 45161 mov rbp,rsp
            rbp = rsp;
            // 45164 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 45166 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 45168 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4516a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4516c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4516d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 45171 mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // 45174 mov r13d,ecx
            r13 = ((rcx as u32 as u64)) as u32 as u64;
            // 45177 mov r14,rdx
            r14 = rdx;
            // 4517a mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 4517d mov rbx,rdi
            rbx = rdi;
            // 45180 mov DWORD PTR [rbp-0x2c],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x10u64) as u32);
            // 45187 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4518a call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 45190 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 45192 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 45197 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 45199 je 0x451ff
            if fl.e() { pc = 0x451ff; continue; }
            pc = 0x4519b;
        }
        0x4519b => {
            // block 4519b
            #[cfg(feature = "trace")] eprintln!("f_00045160 pc 4519b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4519b cmp r12d,0x1000
            fl = sub::<32>((r12 as u32 as u64), 0x1000u64).1;
            // 451a2 mov eax,0x1000
            rax = (0x1000u64) as u32 as u64;
            // 451a7 cmova r12d,eax
            if fl.a() { r12 = ((rax as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // 451ab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 451ae lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 451b2 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 451b4 mov rdi,rbx
            rdi = rbx;
            // 451b7 call QWORD PTR [rax+0xcd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xcd0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 451bd mov edi,DWORD PTR [rbp-0x2c]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 451c0 call 0x5c9a4
            { let r = { let t = m.env.code_va(0x5c9a4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 451c5 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 451c8 mov ecx,0x2000
            rcx = (0x2000u64) as u32 as u64;
            // 451cd mov edx,0x1000
            rdx = (0x1000u64) as u32 as u64;
            // 451d2 cmovne ecx,edx
            if fl.ne() { rcx = ((rdx as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 451d5 sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 451d7 mov r10,QWORD PTR [rbx]
            r10 = m.r64(rbx) as u64;
            // 451da mov DWORD PTR [rsp],r12d
            m.w32(rsp, ((r12 as u32 as u64)) as u32);
            // 451de mov DWORD PTR [rsp+0x8],0x1
            m.w32(rsp.wrapping_add(0x8), (0x1u64) as u32);
            // 451e6 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 451eb xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 451ed mov rdi,rbx
            rdi = rbx;
            // 451f0 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 451f3 mov r8,r14
            r8 = r14;
            // 451f6 mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // 451f9 call QWORD PTR [r10+0x58]
            { let r = { let t = m.r64(r10.wrapping_add(0x58)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 451fd jmp 0x45201
            pc = 0x45201; continue;
        }
        0x451ff => {
            // block 451ff
            #[cfg(feature = "trace")] eprintln!("f_00045160 pc 451ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 451ff xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x45201;
        }
        0x45201 => {
            // block 45201
            #[cfg(feature = "trace")] eprintln!("f_00045160 pc 45201 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 45201 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 45205 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 45206 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 45208 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 4520a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4520c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4520e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4520f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd4dc size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d4dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd4dc;
    loop {
        match pc {
        0xd4dc => {
            // block d4dc
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d4dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d4dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d4dd mov rbp,rsp
            rbp = rsp;
            // d4e0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d4e2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d4e4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d4e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d4e7 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d4eb mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d4ee mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d4f0 mov r12,rdi
            r12 = rdi;
            // d4f3 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d4f8 call 0xd0ea0
            { let r = { let t = m.env.code_va(0xd0ea0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d4fd movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d500 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d502 jbe 0xd57f
            if fl.be() { pc = 0xd57f; continue; }
            pc = 0xd504;
        }
        0xd504 => {
            // block d504
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d504 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d504 mov r15d,0x1
            r15 = (0x1u64) as u32 as u64;
            // d50a test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d50d je 0xd51f
            if fl.e() { pc = 0xd51f; continue; }
            pc = 0xd50f;
        }
        0xd50f => {
            // block d50f
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d50f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d50f movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d512 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d515 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d518 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // d51b lea r15d,[rax+rax*8]
            r15 = (rax.wrapping_add(rax.wrapping_mul(8))) as u32 as u64;
            pc = 0xd51f;
        }
        0xd51f => {
            // block d51f
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d51f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d51f mov BYTE PTR [rbp-0x21],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf), (0x0u64) as u8);
            // d523 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d527 mov rdi,r12
            rdi = r12;
            // d52a mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d52d call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d533 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d535 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d537 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // d53a je 0xd581
            if fl.e() { pc = 0xd581; continue; }
            pc = 0xd53c;
        }
        0xd53c => {
            // block d53c
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d53c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d53c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d53e jne 0xd581
            if fl.ne() { pc = 0xd581; continue; }
            pc = 0xd540;
        }
        0xd540 => {
            // block d540
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d540 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d540 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d544 lea rsi,[rbp-0x21]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdf);
            // d548 mov rdi,r12
            rdi = r12;
            // d54b call QWORD PTR [rax+0x1820]
            { let r = { let t = m.r64(rax.wrapping_add(0x1820)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d551 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d553 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d555 cmp BYTE PTR [rbp-0x21],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64, 0x0u64).1;
            // d559 je 0xd581
            if fl.e() { pc = 0xd581; continue; }
            pc = 0xd55b;
        }
        0xd55b => {
            // block d55b
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d55b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d55b xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d55e jne 0xd581
            if fl.ne() { pc = 0xd581; continue; }
            pc = 0xd560;
        }
        0xd560 => {
            // block d560
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d560 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d564 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // d567 mov esi,0xe9
            rsi = (0xe9u64) as u32 as u64;
            // d56c mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // d571 mov rdi,r12
            rdi = r12;
            // d574 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d577 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // d57a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d57d jmp 0xd581
            pc = 0xd581; continue;
        }
        0xd57f => {
            // block d57f
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d57f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d57f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd581;
        }
        0xd581 => {
            // block d581
            #[cfg(feature = "trace")] eprintln!("f_0000d4dc pc d581 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d581 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d585 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d586 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d588 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d58a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d58c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d58d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43d98 size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043d98<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43d98;
    loop {
        match pc {
        0x43d98 => {
            // block 43d98
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43d98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43d98 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43d99 mov rbp,rsp
            rbp = rsp;
            // 43d9c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43d9e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43da0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 43da2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 43da4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43da5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43da6 mov r14,rdx
            r14 = rdx;
            // 43da9 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 43dac mov r13,rdi
            r13 = rdi;
            // 43daf mov edi,DWORD PTR [r13+0x10]
            rdi = (m.r32(r13.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43db3 call 0xcc811
            { let r = { let t = m.env.code_va(0xcc811); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43db8 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 43dba xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43dbc cmp r15d,0x7
            fl = sub::<32>((r15 as u32 as u64), 0x7u64).1;
            // 43dc0 ja 0x43e3b
            if fl.a() { pc = 0x43e3b; continue; }
            pc = 0x43dc2;
        }
        0x43dc2 => {
            // block 43dc2
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43dc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43dc2 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 43dc4 je 0x43e3b
            if fl.e() { pc = 0x43e3b; continue; }
            pc = 0x43dc6;
        }
        0x43dc6 => {
            // block 43dc6
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43dc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43dc6 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 43dcd lea r12,[r14+0x8]
            r12 = r14.wrapping_add(0x8);
            // 43dd1 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 43dd4 lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // 43dd8 shl rax,0x5
            { let (r, f) = shl::<64>(rax, 0x5u64, fl); rax = r; fl = f; }
            // 43ddc lea rcx,[rip+0xbc2dd] # 0x1000c0
            rcx = m.env.data_va(84109);
            // 43de3 lea rbx,[rax+rcx*1+0x8]
            rbx = rax.wrapping_add(rcx).wrapping_add(0x8);
            // 43de8 mov r15d,0x1
            r15 = (0x1u64) as u32 as u64;
            pc = 0x43dee;
        }
        0x43dee => {
            // block 43dee
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43dee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43dee mov rdi,r14
            rdi = r14;
            // 43df1 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 43df5 call 0xebb40
            { let r = { let t = m.env.stub_va(0x30d); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43dfa cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 43dfd setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 43e00 mov r9,QWORD PTR [r13+0x0]
            r9 = m.r64(r13) as u64;
            // 43e04 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 43e07 mov esi,DWORD PTR [rbx-0x8]
            rsi = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fff8)) as u64) as u32 as u64;
            // 43e0a mov ecx,DWORD PTR [rbx-0x4]
            rcx = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 43e0d mov r8d,DWORD PTR [rbx]
            r8 = (m.r32(rbx) as u64) as u32 as u64;
            // 43e10 mov rdi,r13
            rdi = r13;
            // 43e13 call QWORD PTR [r9+0x30]
            { let r = { let t = m.r64(r9.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43e17 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43e19 je 0x43e39
            if fl.e() { pc = 0x43e39; continue; }
            pc = 0x43e1b;
        }
        0x43e1b => {
            // block 43e1b
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e1b mov DWORD PTR [rbp-0x2c],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r15 as u32 as u64)) as u32);
            // 43e1f cmp r15d,0x3f
            fl = sub::<32>((r15 as u32 as u64), 0x3fu64).1;
            // 43e23 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 43e25 ja 0x43e3b
            if fl.a() { pc = 0x43e3b; continue; }
            pc = 0x43e27;
        }
        0x43e27 => {
            // block 43e27
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e27 lea ecx,[r15-0x8]
            rcx = (r15.wrapping_add(0xffff_ffff_ffff_fff8)) as u32 as u64;
            // 43e2b inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 43e2e add rbx,0xc
            { let (r, f) = add::<64>(rbx, 0xcu64);
              rbx = r;
              fl = f; }
            // 43e32 cmp ecx,0x37
            fl = sub::<32>((rcx as u32 as u64), 0x37u64).1;
            // 43e35 ja 0x43dee
            if fl.a() { pc = 0x43dee; continue; }
            pc = 0x43e37;
        }
        0x43e37 => {
            // block 43e37
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e37 jmp 0x43e3b
            pc = 0x43e3b; continue;
        }
        0x43e39 => {
            // block 43e39
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e39 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x43e3b;
        }
        0x43e3b => {
            // block 43e3b
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e3b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43e3f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43e40 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 43e42 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 43e44 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43e46 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43e48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43e49 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44320 size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044320<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44320;
    loop {
        match pc {
        0x44320 => {
            // block 44320
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 44320 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44320 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44321 mov rbp,rsp
            rbp = rsp;
            // 44324 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44326 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44328 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 4432a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4432c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4432d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4432e mov r14,rcx
            r14 = rcx;
            // 44331 mov r12,rdx
            r12 = rdx;
            // 44334 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 44337 mov rbx,rdi
            rbx = rdi;
            // 4433a mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 4433d call 0xcf5fd
            { let r = { let t = m.env.code_va(0xcf5fd); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44342 mov DWORD PTR [r12],0x8
            m.w32(r12, (0x8u64) as u32);
            // 4434a mov DWORD PTR [r14],0x20
            m.w32(r14, (0x20u64) as u32);
            // 44351 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 44354 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 44357 jb 0x44368
            if fl.b() { pc = 0x44368; continue; }
            pc = 0x44359;
        }
        0x44359 => {
            // block 44359
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 44359 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44359 lea ecx,[r15*4+0x4]
            rcx = (r15.wrapping_mul(4).wrapping_add(0x4)) as u32 as u64;
            // 44361 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // 44364 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 44366 jbe 0x4437d
            if fl.be() { pc = 0x4437d; continue; }
            pc = 0x44368;
        }
        0x44368 => {
            // block 44368
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 44368 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44368 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x4436b;
        }
        0x4436b => {
            // block 4436b
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 4436b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4436b mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 4436e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 44372 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44373 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 44375 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 44377 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44379 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4437b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4437c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4437d => {
            // block 4437d
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 4437d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4437d mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 44384 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 44387 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 4438b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 4438e mov esi,0xbe
            rsi = (0xbeu64) as u32 as u64;
            // 44393 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44398 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4439b mov rdi,rbx
            rdi = rbx;
            // 4439e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 443a1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 443a3 je 0x4436b
            if fl.e() { pc = 0x4436b; continue; }
            pc = 0x443a5;
        }
        0x443a5 => {
            // block 443a5
            #[cfg(feature = "trace")] eprintln!("f_00044320 pc 443a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 443a5 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 443a8 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 443ab lea rdx,[rip+0xbc2de] # 0x100690
            rdx = m.env.data_va(85597);
            // 443b2 mov cl,BYTE PTR [rdx+rcx*4]
            rcx = (rcx & !0xff) | ((m.r8(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u8 as u64);
            // 443b5 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 443b7 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 443b9 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 443bc mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // 443bf shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 443c2 mov DWORD PTR [r12],ecx
            m.w32(r12, ((rcx as u32 as u64)) as u32);
            // 443c6 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 443c9 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 443cc mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // 443cf jmp 0x4436b
            pc = 0x4436b; continue;
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

/// parity: P2v @0xea43e size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ea43e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xea43e;
    loop {
        match pc {
        0xea43e => {
            // block ea43e
            #[cfg(feature = "trace")] eprintln!("f_000ea43e pc ea43e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea43e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea43f mov rbp,rsp
            rbp = rsp;
            // ea442 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ea444 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea446 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ea448 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ea44a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea44b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ea44c mov rbx,rdi
            rbx = rdi;
            // ea44f call 0xe0758
            { let r = { let t = m.env.code_va(0xe0758); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea454 lea rax,[rip+0x5c5a5] # 0x146a00
            rax = m.env.data_va(373197);
            // ea45b mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // ea45e mov DWORD PTR [rbx+0x34],0xff
            m.w32(rbx.wrapping_add(0x34), (0xffu64) as u32);
            // ea465 lea rdi,[rbx+0x38]
            rdi = rbx.wrapping_add(0x38);
            // ea469 lea rax,[rip+0x59d58] # 0x1441c8
            rax = m.env.data_va(362901);
            // ea470 lea r12,[rax+0x10]
            r12 = rax.wrapping_add(0x10);
            // ea474 mov QWORD PTR [rbx+0x38],r12
            m.w64(rbx.wrapping_add(0x38), (r12) as u64);
            // ea478 lea r14,[rbx+0x40]
            r14 = rbx.wrapping_add(0x40);
            // ea47c lea r13,[rbx+0x48]
            r13 = rbx.wrapping_add(0x48);
            // ea480 mov QWORD PTR [rbx+0x50],0x0
            m.w64(rbx.wrapping_add(0x50), (0x0u64) as u64);
            // ea488 mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // ea490 mov QWORD PTR [rbx+0x40],r13
            m.w64(rbx.wrapping_add(0x40), (r13) as u64);
            // ea494 call QWORD PTR [rax+0x40]
            { let r = { let t = m.r64(rax.wrapping_add(0x40)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea497 call 0xec1f4
            { let r = { let t = m.env.stub_va(0x9c1); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea49c mov DWORD PTR [rbx+0x58],eax
            m.w32(rbx.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // ea49f mov DWORD PTR [rbx+0x5c],0xffffffff
            m.w32(rbx.wrapping_add(0x5c), (0xffffffffu64) as u32);
            // ea4a6 mov DWORD PTR [rbx+0x60],0x0
            m.w32(rbx.wrapping_add(0x60), (0x0u64) as u32);
            // ea4ad mov DWORD PTR [rbx+0x64],0x3
            m.w32(rbx.wrapping_add(0x64), (0x3u64) as u32);
            // ea4b4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ea4b8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ea4b9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ea4bb pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ea4bd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ea4bf pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ea4c1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea4c2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xea4c3 => {
            // block ea4c3
            #[cfg(feature = "trace")] eprintln!("f_000ea43e pc ea4c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea4c3 mov r15,rax
            r15 = rax;
            // ea4c6 mov rsi,QWORD PTR [r13+0x0]
            rsi = m.r64(r13) as u64;
            // ea4ca jmp 0xea4d7
            pc = 0xea4d7; continue;
        }
        0xea4cc => {
            // block ea4cc
            #[cfg(feature = "trace")] eprintln!("f_000ea43e pc ea4cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea4cc mov r15,rax
            r15 = rax;
            // ea4cf mov QWORD PTR [rbx+0x38],r12
            m.w64(rbx.wrapping_add(0x38), (r12) as u64);
            // ea4d3 mov rsi,QWORD PTR [rbx+0x48]
            rsi = m.r64(rbx.wrapping_add(0x48)) as u64;
            pc = 0xea4d7;
        }
        0xea4d7 => {
            // block ea4d7
            #[cfg(feature = "trace")] eprintln!("f_000ea43e pc ea4d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea4d7 mov rdi,r14
            rdi = r14;
            // ea4da call 0xebe22
            { let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea4df mov rdi,rbx
            rdi = rbx;
            // ea4e2 call 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea4e7 mov rdi,r15
            rdi = r15;
            // ea4ea call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6c4 size=180
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d6c4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd6c4;
    loop {
        match pc {
        0xd6c4 => {
            // block d6c4
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d6c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6c5 mov rbp,rsp
            rbp = rsp;
            // d6c8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d6ca push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d6cc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d6ce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6cf sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d6d3 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d6d6 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d6d8 mov r12,rdi
            r12 = rdi;
            // d6db mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6e0 call 0xd0fb9
            { let r = { let t = m.env.code_va(0xd0fb9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6e5 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d6e8 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d6ea jbe 0xd769
            if fl.be() { pc = 0xd769; continue; }
            pc = 0xd6ec;
        }
        0xd6ec => {
            // block d6ec
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d6ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ec mov r15d,0x10
            r15 = (0x10u64) as u32 as u64;
            // d6f2 test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d6f5 je 0xd709
            if fl.e() { pc = 0xd709; continue; }
            pc = 0xd6f7;
        }
        0xd6f7 => {
            // block d6f7
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d6f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6f7 movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d6fa xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d6fc cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d6ff mov r15d,0x14
            r15 = (0x14u64) as u32 as u64;
            // d705 cmovne r15d,ecx
            if fl.ne() { r15 = ((rcx as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            pc = 0xd709;
        }
        0xd709 => {
            // block d709
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d709 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d709 mov BYTE PTR [rbp-0x21],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf), (0x0u64) as u8);
            // d70d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d711 mov rdi,r12
            rdi = r12;
            // d714 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d717 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d71d mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d71f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d721 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // d724 je 0xd76b
            if fl.e() { pc = 0xd76b; continue; }
            pc = 0xd726;
        }
        0xd726 => {
            // block d726
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d726 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d728 jne 0xd76b
            if fl.ne() { pc = 0xd76b; continue; }
            pc = 0xd72a;
        }
        0xd72a => {
            // block d72a
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d72a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d72a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d72e lea rsi,[rbp-0x21]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdf);
            // d732 mov rdi,r12
            rdi = r12;
            // d735 call QWORD PTR [rax+0x1820]
            { let r = { let t = m.r64(rax.wrapping_add(0x1820)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d73b mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d73d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d73f cmp BYTE PTR [rbp-0x21],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf)) as u64, 0x0u64).1;
            // d743 je 0xd76b
            if fl.e() { pc = 0xd76b; continue; }
            pc = 0xd745;
        }
        0xd745 => {
            // block d745
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d745 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d745 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d748 jne 0xd76b
            if fl.ne() { pc = 0xd76b; continue; }
            pc = 0xd74a;
        }
        0xd74a => {
            // block d74a
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d74a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d74a mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d74e dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // d751 mov esi,0xe9
            rsi = (0xe9u64) as u32 as u64;
            // d756 mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // d75b mov rdi,r12
            rdi = r12;
            // d75e mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d761 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // d764 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d767 jmp 0xd76b
            pc = 0xd76b; continue;
        }
        0xd769 => {
            // block d769
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d769 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d769 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd76b;
        }
        0xd76b => {
            // block d76b
            #[cfg(feature = "trace")] eprintln!("f_0000d6c4 pc d76b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d76b add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d76f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d770 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d772 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d774 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d776 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d777 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

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
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11e9f mov rdi,rbx
            rdi = rbx;
            // 11ea2 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11ed2 mov rdi,rbx
            rdi = rbx;
            // 11ed5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xb6914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x450ac size=180
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000450ac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x450ac;
    loop {
        match pc {
        0x450ac => {
            // block 450ac
            #[cfg(feature = "trace")] eprintln!("f_000450ac pc 450ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 450ac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 450ad mov rbp,rsp
            rbp = rsp;
            // 450b0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 450b2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 450b4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 450b6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 450b8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 450b9 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 450bd mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // 450c0 mov r13d,ecx
            r13 = ((rcx as u32 as u64)) as u32 as u64;
            // 450c3 mov r14,rdx
            r14 = rdx;
            // 450c6 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 450c9 mov rbx,rdi
            rbx = rdi;
            // 450cc mov DWORD PTR [rbp-0x2c],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x10u64) as u32);
            // 450d3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 450d6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 450dc mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 450de call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 450e3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 450e5 je 0x4514e
            if fl.e() { pc = 0x4514e; continue; }
            pc = 0x450e7;
        }
        0x450e7 => {
            // block 450e7
            #[cfg(feature = "trace")] eprintln!("f_000450ac pc 450e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 450e7 cmp r12d,0x1000
            fl = sub::<32>((r12 as u32 as u64), 0x1000u64).1;
            // 450ee mov eax,0x1000
            rax = (0x1000u64) as u32 as u64;
            // 450f3 cmova r12d,eax
            if fl.a() { r12 = ((rax as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // 450f7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 450fa lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 450fe xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 45100 mov rdi,rbx
            rdi = rbx;
            // 45103 call QWORD PTR [rax+0xcd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xcd0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 45109 mov edi,DWORD PTR [rbp-0x2c]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 4510c call 0x5c9a4
            { let r = { let t = m.env.code_va(0x5c9a4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 45111 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 45114 mov ecx,0x2000
            rcx = (0x2000u64) as u32 as u64;
            // 45119 mov edx,0x1000
            rdx = (0x1000u64) as u32 as u64;
            // 4511e cmovne ecx,edx
            if fl.ne() { rcx = ((rdx as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 45121 sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 45123 mov r10,QWORD PTR [rbx]
            r10 = m.r64(rbx) as u64;
            // 45126 mov DWORD PTR [rsp],r12d
            m.w32(rsp, ((r12 as u32 as u64)) as u32);
            // 4512a mov DWORD PTR [rsp+0x8],0x1
            m.w32(rsp.wrapping_add(0x8), (0x1u64) as u32);
            // 45132 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 45137 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 4513c mov rdi,rbx
            rdi = rbx;
            // 4513f mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 45142 mov r8,r14
            r8 = r14;
            // 45145 mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // 45148 call QWORD PTR [r10+0x58]
            { let r = { let t = m.r64(r10.wrapping_add(0x58)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4514c jmp 0x45150
            pc = 0x45150; continue;
        }
        0x4514e => {
            // block 4514e
            #[cfg(feature = "trace")] eprintln!("f_000450ac pc 4514e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4514e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x45150;
        }
        0x45150 => {
            // block 45150
            #[cfg(feature = "trace")] eprintln!("f_000450ac pc 45150 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 45150 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 45154 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 45155 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 45157 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 45159 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4515b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4515d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4515e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x153c size=182
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000153c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x153c;
    loop {
        match pc {
        0x153c => {
            // block 153c
            #[cfg(feature = "trace")] eprintln!("f_0000153c pc 153c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 153c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 153d mov rbp,rsp
            rbp = rsp;
            // 1540 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1542 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1544 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1546 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1547 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 154b mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 154e mov r14,rsi
            r14 = rsi;
            // 1551 mov rbx,rdi
            rbx = rdi;
            // 1554 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 155b mov DWORD PTR [rbp-0x28],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u32);
            // 1562 mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // 1569 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 156c call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1572 mov r9,QWORD PTR [rbx]
            r9 = m.r64(rbx) as u64;
            // 1575 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 1578 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 157b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 157d cmove rcx,r15
            if fl.e() { rcx = r15; }
            // 1581 lea rax,[rip+0xfda68] # 0xfeff0
            rax = m.env.data_va(79805);
            // 1588 mov r12d,DWORD PTR [rax+rcx*4]
            r12 = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 158c lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 1590 mov ecx,0x7
            rcx = (0x7u64) as u32 as u64;
            // 1595 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 1598 mov rdi,rbx
            rdi = rbx;
            // 159b mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 159e call QWORD PTR [r9+0x38]
            { let r = { let t = m.r64(r9.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15a2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 15a4 je 0x15e1
            if fl.e() { pc = 0x15e1; continue; }
            pc = 0x15a6;
        }
        0x15a6 => {
            // block 15a6
            #[cfg(feature = "trace")] eprintln!("f_0000153c pc 15a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 15a9 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 15ad mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 15b2 mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 15b8 mov rdi,rbx
            rdi = rbx;
            // 15bb mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 15be call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15c1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 15c3 je 0x15de
            if fl.e() { pc = 0x15de; continue; }
            pc = 0x15c5;
        }
        0x15c5 => {
            // block 15c5
            #[cfg(feature = "trace")] eprintln!("f_0000153c pc 15c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15c5 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 15c8 and eax,0x7
            { let r = (rax as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 15cb mov ecx,DWORD PTR [rbp-0x28]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 15ce shl ecx,0x3
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x3u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 15d1 and ecx,0x8
            { let r = (rcx as u32 as u64) & 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 15d4 or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 15d6 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            // 15d9 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 15dc jmp 0x15e1
            pc = 0x15e1; continue;
        }
        0x15de => {
            // block 15de
            #[cfg(feature = "trace")] eprintln!("f_0000153c pc 15de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15de xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x15e1;
        }
        0x15e1 => {
            // block 15e1
            #[cfg(feature = "trace")] eprintln!("f_0000153c pc 15e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e1 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 15e4 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 15e8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15e9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15eb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15ed pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15ef pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15f0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7e14 size=182
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7e14<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd7e14;
    loop {
        match pc {
        0xd7e14 => {
            // block d7e14
            #[cfg(feature = "trace")] eprintln!("f_000d7e14 pc d7e14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7e14 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7e15 mov rbp,rsp
            rbp = rsp;
            // d7e18 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d7e1a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d7e1c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d7e1e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d7e1f mov r15,rdx
            r15 = rdx;
            // d7e22 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d7e25 mov r12,rdi
            r12 = rdi;
            // d7e28 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d7e2c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d7e2e mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d7e33 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d7e38 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7e3d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7e40 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7e43 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d7e47 mov rdi,r12
            rdi = r12;
            // d7e4a call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7e50 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d7e54 mov r9,QWORD PTR [rax+0x30]
            r9 = m.r64(rax.wrapping_add(0x30)) as u64;
            pc = 0xd7e58;
        }
        0xd7e58 => {
            // block d7e58
            #[cfg(feature = "trace")] eprintln!("f_000d7e14 pc d7e58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7e58 mov edx,DWORD PTR [r15+rbx*4]
            rdx = (m.r32(r15.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // d7e5c mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d7e61 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7e66 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7e69 mov rdi,r12
            rdi = r12;
            // d7e6c call r9
            { let r = { let t = r9; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7e6f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d7e73 mov r9,QWORD PTR [rax+0x30]
            r9 = m.r64(rax.wrapping_add(0x30)) as u64;
            // d7e77 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // d7e7a cmp ebx,0x40
            fl = sub::<32>((rbx as u32 as u64), 0x40u64).1;
            // d7e7d jne 0xd7e58
            if fl.ne() { pc = 0xd7e58; continue; }
            pc = 0xd7e7f;
        }
        0xd7e7f => {
            // block d7e7f
            #[cfg(feature = "trace")] eprintln!("f_000d7e14 pc d7e7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7e7f mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d7e84 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7e89 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7e8c mov rdi,r12
            rdi = r12;
            // d7e8f mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d7e92 call r9
            { let r = { let t = r9; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7e95 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d7e99 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d7e9e mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // d7ea3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7ea8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7eab mov rdi,r12
            rdi = r12;
            // d7eae call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7eb1 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d7eb5 mov rdi,r12
            rdi = r12;
            // d7eb8 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ebe mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d7ec0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7ec1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d7ec3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7ec5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d7ec7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7ec8 ret
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
            { let r = { let t = m.env.stub_va(0x61); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x3d); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x37); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x277); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
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
            { let r = { let t = m.env.stub_va(0x277); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x37); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
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

/// parity: P2v @0xe523e size=184
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e523e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe523e;
    loop {
        match pc {
        0xe523e => {
            // block e523e
            #[cfg(feature = "trace")] eprintln!("f_000e523e pc e523e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e523e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e523f mov rbp,rsp
            rbp = rsp;
            // e5242 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e5244 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e5246 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e5247 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e524b mov rbx,rdi
            rbx = rdi;
            // e524e lea rax,[rip+0x61053] # 0x1462a8
            rax = m.env.data_va(371317);
            // e5255 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e5258 lea r14,[rbx+0x20]
            r14 = rbx.wrapping_add(0x20);
            // e525c mov QWORD PTR [rbx+0x90],0x0
            m.w64(rbx.wrapping_add(0x90), (0x0u64) as u64);
            // e5267 mov BYTE PTR [rbx+0x8c],0x0
            m.w8(rbx.wrapping_add(0x8c), (0x0u64) as u8);
            // e526e mov BYTE PTR [rbx+0xc],0x0
            m.w8(rbx.wrapping_add(0xc), (0x0u64) as u8);
            // e5272 mov BYTE PTR [rbx+0xd],0x0
            m.w8(rbx.wrapping_add(0xd), (0x0u64) as u8);
            // e5276 mov DWORD PTR [rbx+0x8],0x0
            m.w32(rbx.wrapping_add(0x8), (0x0u64) as u32);
            // e527d mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // e5285 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // e528d mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // e5295 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // e529d mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // e52a5 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // e52ad mov eax,DWORD PTR [rsi]
            rax = (m.r32(rsi) as u64) as u32 as u64;
            // e52af mov ecx,DWORD PTR [rsi+0x4]
            rcx = (m.r32(rsi.wrapping_add(0x4)) as u64) as u32 as u64;
            // e52b2 shl rcx,0x20
            { let (r, f) = shl::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // e52b6 or rcx,rax
            { let r = rcx | rax; let f = logic::<64>(r);
              rcx = r;
              fl = f; }
            // e52b9 mov eax,DWORD PTR [rsi+0x8]
            rax = (m.r32(rsi.wrapping_add(0x8)) as u64) as u32 as u64;
            // e52bc mov QWORD PTR [rbp-0x28],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rcx) as u64);
            // e52c0 mov DWORD PTR [rbp-0x20],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), ((rax as u32 as u64)) as u32);
            // e52c3 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // e52c7 call 0xe4fc6
            { let r = { let t = m.env.code_va(0xe4fc6); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e52cc add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e52d0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e52d1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e52d3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e52d5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e52d6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe52d7 => {
            // block e52d7
            #[cfg(feature = "trace")] eprintln!("f_000e523e pc e52d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e52d7 mov r15,rax
            r15 = rax;
            // e52da add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // e52de mov rdi,rbx
            rdi = rbx;
            // e52e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e52e6 mov rdi,r14
            rdi = r14;
            // e52e9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e52ee mov rdi,r15
            rdi = r15;
            // e52f1 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xcb6fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xcb6a1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x470)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x5a8f7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x811); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x80b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x2bc7a size=186
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bc7a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2bc7a;
    loop {
        match pc {
        0x2bc7a => {
            // block 2bc7a
            #[cfg(feature = "trace")] eprintln!("f_0002bc7a pc 2bc7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc7a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bc7b mov rbp,rsp
            rbp = rsp;
            // 2bc7e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bc80 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bc82 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2bc84 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2bc86 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bc87 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2bc8b mov r14,rsi
            r14 = rsi;
            // 2bc8e lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2bc92 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 2bc9a mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 2bca2 mov QWORD PTR [rbp-0x40],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r15) as u64);
            // 2bca6 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2bcaa call 0x2bbc2
            { let r = { let t = m.env.code_va(0x2bbc2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bcaf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2bcb1 je 0x2bcf9
            if fl.e() { pc = 0x2bcf9; continue; }
            pc = 0x2bcb3;
        }
        0x2bcb3 => {
            // block 2bcb3
            #[cfg(feature = "trace")] eprintln!("f_0002bc7a pc 2bcb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bcb3 mov rbx,QWORD PTR [r14]
            rbx = m.r64(r14) as u64;
            // 2bcb6 mov r12,QWORD PTR [r14+0x8]
            r12 = m.r64(r14.wrapping_add(0x8)) as u64;
            // 2bcba mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 2bcbd cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 2bcc0 je 0x2bcfc
            if fl.e() { pc = 0x2bcfc; continue; }
            pc = 0x2bcc2;
        }
        0x2bcc2 => {
            // block 2bcc2
            #[cfg(feature = "trace")] eprintln!("f_0002bc7a pc 2bcc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bcc2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 2bcc4 lea r14,[rbp-0x40]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            pc = 0x2bcc8;
        }
        0x2bcc8 => {
            // block 2bcc8
            #[cfg(feature = "trace")] eprintln!("f_0002bc7a pc 2bcc8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bcc8 mov r13d,ecx
            r13 = ((rcx as u32 as u64)) as u32 as u64;
            // 2bccb mov rdi,r14
            rdi = r14;
            // 2bcce mov rsi,rbx
            rsi = rbx;
            // 2bcd1 call 0xebaec
            { let r = { let t = m.env.stub_va(0x2b9); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bcd6 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // 2bcd9 sete cl
            rcx = (rcx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 2bcdc movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 2bcdf add ecx,r13d
            { let (r, f) = add::<32>((rcx as u32 as u64), (r13 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // 2bce2 mov eax,DWORD PTR [rax+0x20]
            rax = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            // 2bce5 mov DWORD PTR [rbx+0x4],eax
            m.w32(rbx.wrapping_add(0x4), ((rax as u32 as u64)) as u32);
            // 2bce8 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 2bcec cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 2bcef jne 0x2bcc8
            if fl.ne() { pc = 0x2bcc8; continue; }
            pc = 0x2bcf1;
        }
        0x2bcf1 => {
            // block 2bcf1
            #[cfg(feature = "trace")] eprintln!("f_0002bc7a pc 2bcf1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bcf1 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 2bcf3 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 2bcf7 jmp 0x2bcfc
            pc = 0x2bcfc; continue;
        }
        0x2bcf9 => {
            // block 2bcf9
            #[cfg(feature = "trace")] eprintln!("f_0002bc7a pc 2bcf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bcf9 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bcfc;
        }
        0x2bcfc => {
            // block 2bcfc
            #[cfg(feature = "trace")] eprintln!("f_0002bc7a pc 2bcfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bcfc mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 2bd00 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2bd04 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bd09 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 2bd0c add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2bd10 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bd11 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2bd13 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2bd15 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bd17 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2bd19 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bd1a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2bd1b => {
            // block 2bd1b
            #[cfg(feature = "trace")] eprintln!("f_0002bc7a pc 2bd1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bd1b mov rbx,rax
            rbx = rax;
            // 2bd1e mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 2bd22 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2bd26 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bd2b mov rdi,rbx
            rdi = rbx;
            // 2bd2e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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
            { let r = { let t = m.env.code_va(0xceac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x6858c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x10f8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xc8c60); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x22fc size=192
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000022fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x22fc;
    loop {
        match pc {
        0x22fc => {
            // block 22fc
            #[cfg(feature = "trace")] eprintln!("f_000022fc pc 22fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 22fd mov rbp,rsp
            rbp = rsp;
            // 2300 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2302 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2304 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2306 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2308 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2309 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 230d mov r14,rsi
            r14 = rsi;
            // 2310 mov rbx,rdi
            rbx = rdi;
            // 2313 mov DWORD PTR [rbp-0x30],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x1u64) as u32);
            // 231a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 231d xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 2320 lea r15,[rbp-0x2c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 2324 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2326 mov rdx,r15
            rdx = r15;
            // 2329 call QWORD PTR [rax+0x398]
            { let r = { let t = m.r64(rax.wrapping_add(0x398)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 232f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2331 je 0x23a9
            if fl.e() { pc = 0x23a9; continue; }
            pc = 0x2333;
        }
        0x2333 => {
            // block 2333
            #[cfg(feature = "trace")] eprintln!("f_000022fc pc 2333 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2333 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2336 lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 233a mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 233f mov rdi,rbx
            rdi = rbx;
            // 2342 mov rdx,r12
            rdx = r12;
            // 2345 call QWORD PTR [rax+0x398]
            { let r = { let t = m.r64(rax.wrapping_add(0x398)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 234b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 234d jne 0x2361
            if fl.ne() { pc = 0x2361; continue; }
            pc = 0x234f;
        }
        0x234f => {
            // block 234f
            #[cfg(feature = "trace")] eprintln!("f_000022fc pc 234f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 234f mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 2352 call 0xcf034
            { let r = { let t = m.env.code_va(0xcf034); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2357 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 235a jbe 0x2361
            if fl.be() { pc = 0x2361; continue; }
            pc = 0x235c;
        }
        0x235c => {
            // block 235c
            #[cfg(feature = "trace")] eprintln!("f_000022fc pc 235c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 235c xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 235f jmp 0x23a9
            pc = 0x23a9; continue;
        }
        0x2361 => {
            // block 2361
            #[cfg(feature = "trace")] eprintln!("f_000022fc pc 2361 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2361 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2364 lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 2368 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 236b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 236d mov rdi,rbx
            rdi = rbx;
            // 2370 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2376 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2378 je 0x23a9
            if fl.e() { pc = 0x23a9; continue; }
            pc = 0x237a;
        }
        0x237a => {
            // block 237a
            #[cfg(feature = "trace")] eprintln!("f_000022fc pc 237a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 237a mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 237d mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // 2380 mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 2383 call 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2388 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 238b mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 238e mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // 2391 mov edx,DWORD PTR [rbp-0x30]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 2394 call 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2399 cmp r13d,eax
            fl = sub::<32>((r13 as u32 as u64), (rax as u32 as u64)).1;
            // 239c cmovb r15,r12
            if fl.b() { r15 = r12; }
            // 23a0 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 23a3 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 23a6 mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x23a9;
        }
        0x23a9 => {
            // block 23a9
            #[cfg(feature = "trace")] eprintln!("f_000022fc pc 23a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23a9 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 23ac add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 23b0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 23b1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 23b3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 23b5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 23b7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 23b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 23ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2b562 size=192
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002b562<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2b562;
    loop {
        match pc {
        0x2b562 => {
            // block 2b562
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b562 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b563 mov rbp,rsp
            rbp = rsp;
            // 2b566 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2b568 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2b56a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2b56c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2b56e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2b56f sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2b573 mov r12,rsi
            r12 = rsi;
            // 2b576 mov DWORD PTR [rbp-0x38],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0xffffffffu64) as u32);
            // 2b57d mov DWORD PTR [rbp-0x34],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0xffffffffu64) as u32);
            // 2b584 mov DWORD PTR [rbp-0x30],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0xffffffffu64) as u32);
            // 2b58b mov rbx,QWORD PTR [rdi+0xc0]
            rbx = m.r64(rdi.wrapping_add(0xc0)) as u64;
            // 2b592 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2b595 je 0x2b611
            if fl.e() { pc = 0x2b611; continue; }
            pc = 0x2b597;
        }
        0x2b597 => {
            // block 2b597
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b597 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b597 mov eax,DWORD PTR [rdi+0xc8]
            rax = (m.r32(rdi.wrapping_add(0xc8)) as u64) as u32 as u64;
            // 2b59d mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 2b59f mov edx,0xaaaaaaab
            rdx = (0xaaaaaaabu64) as u32 as u64;
            // 2b5a4 imul rdx,rcx
            { let (r, f) = imul::<64>(rdx, rcx); rdx = r; fl = f; }
            // 2b5a8 shr rdx,0x23
            { let (r, f) = shr::<64>(rdx, 0x23u64, fl); rdx = r; fl = f; }
            // 2b5ac cmp eax,0x14f
            fl = sub::<32>((rax as u32 as u64), 0x14fu64).1;
            // 2b5b1 mov r13d,0x1b
            r13 = (0x1bu64) as u32 as u64;
            // 2b5b7 cmovbe r13d,edx
            if fl.be() { r13 = ((rdx as u32 as u64)) as u32 as u64; } else { r13 = r13 as u32 as u64; }
            // 2b5bb mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2b5bd test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 2b5c0 je 0x2b613
            if fl.e() { pc = 0x2b613; continue; }
            pc = 0x2b5c2;
        }
        0x2b5c2 => {
            // block 2b5c2
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b5c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b5c2 lea r14,[r12+0x8]
            r14 = r12.wrapping_add(0x8);
            // 2b5c7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2b5c9 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x2b5cc;
        }
        0x2b5cc => {
            // block 2b5cc
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b5cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b5cc mov DWORD PTR [rbp-0x3c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((rax as u32 as u64)) as u32);
            // 2b5cf mov rdi,r12
            rdi = r12;
            // 2b5d2 lea rsi,[rbp-0x3c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // 2b5d6 call 0xeba92
            { let r = { let t = m.env.stub_va(0x25f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b5db movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 2b5df lea rcx,[rcx+rcx*2]
            rcx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // 2b5e3 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 2b5e6 lea rax,[rax+0x20]
            rax = rax.wrapping_add(0x20);
            // 2b5ea lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2b5ee cmove rax,rdx
            if fl.e() { rax = rdx; }
            // 2b5f2 mov edx,DWORD PTR [rax+0x8]
            rdx = (m.r32(rax.wrapping_add(0x8)) as u64) as u32 as u64;
            // 2b5f5 mov DWORD PTR [rbx+rcx*4+0x8],edx
            m.w32(rbx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2b5f9 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 2b5fc mov QWORD PTR [rbx+rcx*4],rax
            m.w64(rbx.wrapping_add(rcx.wrapping_mul(4)), (rax) as u64);
            // 2b600 lea r15d,[r15+0x1]
            r15 = (r15.wrapping_add(0x1)) as u32 as u64;
            // 2b604 movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // 2b608 cmp eax,r13d
            fl = sub::<32>((rax as u32 as u64), (r13 as u32 as u64)).1;
            // 2b60b jb 0x2b5cc
            if fl.b() { pc = 0x2b5cc; continue; }
            pc = 0x2b60d;
        }
        0x2b60d => {
            // block 2b60d
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b60d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b60d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2b60f jmp 0x2b613
            pc = 0x2b613; continue;
        }
        0x2b611 => {
            // block 2b611
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b611 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b611 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2b613;
        }
        0x2b613 => {
            // block 2b613
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b613 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2b617 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2b618 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2b61a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2b61c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2b61e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2b620 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b621 ret
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
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0xb6914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x58de4 size=193
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00058de4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x58de4;
    loop {
        match pc {
        0x58de4 => {
            // block 58de4
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58de4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58de4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 58de5 mov rbp,rsp
            rbp = rsp;
            // 58de8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 58dea push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58dec push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 58dee push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 58df0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58df1 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 58df5 mov r13,r8
            r13 = r8;
            // 58df8 mov ebx,ecx
            rbx = ((rcx as u32 as u64)) as u32 as u64;
            // 58dfa mov r9,rdi
            r9 = rdi;
            // 58dfd mov QWORD PTR [rbp-0x38],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r9) as u64);
            // 58e01 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 58e03 shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 58e05 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 58e09 shr edx,1
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x1u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 58e0b mov DWORD PTR [rbp-0x2c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rdx as u32 as u64)) as u32);
            // 58e0e mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 58e10 imul esi,edx
            { let (r, f) = imul::<32>((rsi as u32 as u64), (rdx as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // 58e13 mov DWORD PTR [rbp-0x54],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((rsi as u32 as u64)) as u32);
            // 58e16 mov esi,edx
            rsi = ((rdx as u32 as u64)) as u32 as u64;
            // 58e18 imul esi,ebx
            { let (r, f) = imul::<32>((rsi as u32 as u64), (rbx as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // 58e1b mov QWORD PTR [rbp-0x50],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rsi) as u64);
            // 58e1f lea esi,[rsi+rax*1]
            rsi = (rsi.wrapping_add(rax)) as u32 as u64;
            // 58e22 mov DWORD PTR [rbp-0x58],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((rsi as u32 as u64)) as u32);
            // 58e25 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0x58e27;
        }
        0x58e27 => {
            // block 58e27
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58e27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58e27 cmp edi,0x3
            fl = sub::<32>((rdi as u32 as u64), 0x3u64).1;
            // 58e2a je 0x58e48
            if fl.e() { pc = 0x58e48; continue; }
            pc = 0x58e2c;
        }
        0x58e2c => {
            // block 58e2c
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58e2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58e2c cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // 58e2f mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 58e33 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 58e36 je 0x58e4c
            if fl.e() { pc = 0x58e4c; continue; }
            pc = 0x58e38;
        }
        0x58e38 => {
            // block 58e38
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58e38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58e38 cmp edi,0x1
            fl = sub::<32>((rdi as u32 as u64), 0x1u64).1;
            // 58e3b mov r14d,0x0
            r14 = (0x0u64) as u32 as u64;
            // 58e41 jne 0x58e4c
            if fl.ne() { pc = 0x58e4c; continue; }
            pc = 0x58e43;
        }
        0x58e43 => {
            // block 58e43
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58e43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58e43 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 58e46 jmp 0x58e4c
            pc = 0x58e4c; continue;
        }
        0x58e48 => {
            // block 58e48
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58e48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58e48 mov r14d,DWORD PTR [rbp-0x58]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            pc = 0x58e4c;
        }
        0x58e4c => {
            // block 58e4c
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58e4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58e4c mov DWORD PTR [rbp-0x44],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((rdi as u32 as u64)) as u32);
            // 58e4f test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 58e51 je 0x58e8c
            if fl.e() { pc = 0x58e8c; continue; }
            pc = 0x58e53;
        }
        0x58e53 => {
            // block 58e53
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58e53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58e53 mov r15d,DWORD PTR [rbp-0x54]
            r15 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64) as u32 as u64;
            // 58e57 imul r15d,DWORD PTR [rbp-0x44]
            { let (r, f) = imul::<32>((r15 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64); r15 = (r) as u32 as u64; fl = f; }
            // 58e5c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x58e5f;
        }
        0x58e5f => {
            // block 58e5f
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58e5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58e5f mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // 58e62 add rdi,r13
            { let (r, f) = add::<64>(rdi, r13);
              rdi = r;
              fl = f; }
            // 58e65 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 58e68 add rsi,r9
            { let (r, f) = add::<64>(rsi, r9);
              rsi = r;
              fl = f; }
            // 58e6b mov rdx,rax
            rdx = rax;
            // 58e6e call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58e73 mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 58e77 mov r9,QWORD PTR [rbp-0x38]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 58e7b mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 58e7e add r15d,eax
            { let (r, f) = add::<32>((r15 as u32 as u64), (rax as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // 58e81 add r14d,ebx
            { let (r, f) = add::<32>((r14 as u32 as u64), (rbx as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 58e84 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // 58e87 cmp r12d,edx
            fl = sub::<32>((r12 as u32 as u64), (rdx as u32 as u64)).1;
            // 58e8a jb 0x58e5f
            if fl.b() { pc = 0x58e5f; continue; }
            pc = 0x58e8c;
        }
        0x58e8c => {
            // block 58e8c
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58e8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58e8c mov edi,DWORD PTR [rbp-0x44]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64) as u32 as u64;
            // 58e8f inc edi
            { let (r, f) = inc::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 58e91 cmp edi,0x4
            fl = sub::<32>((rdi as u32 as u64), 0x4u64).1;
            // 58e94 jne 0x58e27
            if fl.ne() { pc = 0x58e27; continue; }
            pc = 0x58e96;
        }
        0x58e96 => {
            // block 58e96
            #[cfg(feature = "trace")] eprintln!("f_00058de4 pc 58e96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58e96 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 58e9a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58e9b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 58e9d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 58e9f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58ea1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58ea3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58ea4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x424f6 size=194
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000424f6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x424f6;
    loop {
        match pc {
        0x424f6 => {
            // block 424f6
            #[cfg(feature = "trace")] eprintln!("f_000424f6 pc 424f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 424f6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 424f7 mov rbp,rsp
            rbp = rsp;
            // 424fa push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 424fc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 424fe push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 42500 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 42502 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42503 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42504 mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // 42507 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 4250a mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 4250d mov rbx,rdi
            rbx = rdi;
            // 42510 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 42513 call 0xcf820
            { let r = { let t = m.env.code_va(0xcf820); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42518 movzx r15d,ax
            r15 = ((rax as u16 as u64)) as u32 as u64;
            // 4251c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4251f mov rdi,rbx
            rdi = rbx;
            // 42522 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42528 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 4252b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 4252e cmp r12d,0x4
            fl = sub::<32>((r12 as u32 as u64), 0x4u64).1;
            // 42532 ja 0x425a5
            if fl.a() { pc = 0x425a5; continue; }
            pc = 0x42534;
        }
        0x42534 => {
            // block 42534
            #[cfg(feature = "trace")] eprintln!("f_000424f6 pc 42534 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42534 movzx ecx,r14w
            rcx = ((r14 as u16 as u64)) as u32 as u64;
            // 42538 lea eax,[r15+rax*2]
            rax = (r15.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // 4253c cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 4253e jae 0x425a5
            if fl.ae() { pc = 0x425a5; continue; }
            pc = 0x42540;
        }
        0x42540 => {
            // block 42540
            #[cfg(feature = "trace")] eprintln!("f_000424f6 pc 42540 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42540 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42543 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 42546 lea rdx,[rip+0xbdf23] # 0x100470
            rdx = m.env.data_va(85053);
            // 4254d mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42550 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // 42553 lea rdx,[rip+0xbdef6] # 0x100450
            rdx = m.env.data_va(85021);
            // 4255a mov edx,DWORD PTR [rdx+rcx*4]
            rdx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4255d mov ecx,0xffff
            rcx = (0xffffu64) as u32 as u64;
            // 42562 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42565 mov rdi,rbx
            rdi = rbx;
            // 42568 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4256b mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 4256e test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // 42571 jne 0x425a5
            if fl.ne() { pc = 0x425a5; continue; }
            pc = 0x42573;
        }
        0x42573 => {
            // block 42573
            #[cfg(feature = "trace")] eprintln!("f_000424f6 pc 42573 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42573 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // 42576 je 0x425a5
            if fl.e() { pc = 0x425a5; continue; }
            pc = 0x42578;
        }
        0x42578 => {
            // block 42578
            #[cfg(feature = "trace")] eprintln!("f_000424f6 pc 42578 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42578 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4257b mov rdi,rbx
            rdi = rbx;
            // 4257e mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 42581 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42584 call QWORD PTR [rax+0x6f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x6f0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4258a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4258c je 0x425a5
            if fl.e() { pc = 0x425a5; continue; }
            pc = 0x4258e;
        }
        0x4258e => {
            // block 4258e
            #[cfg(feature = "trace")] eprintln!("f_000424f6 pc 4258e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4258e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42591 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 42596 mov rdi,rbx
            rdi = rbx;
            // 42599 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 4259c call QWORD PTR [rax+0x700]
            { let r = { let t = m.r64(rax.wrapping_add(0x700)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 425a2 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x425a5;
        }
        0x425a5 => {
            // block 425a5
            #[cfg(feature = "trace")] eprintln!("f_000424f6 pc 425a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 425a5 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 425a8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 425ac pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 425ad pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 425af pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 425b1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 425b3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 425b5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 425b6 ret
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
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 444bb mov rdi,rbx
            rdi = rbx;
            // 444be call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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

/// parity: P2v @0x1153a size=196
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001153a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1153a;
    loop {
        match pc {
        0x1153a => {
            // block 1153a
            #[cfg(feature = "trace")] eprintln!("f_0001153a pc 1153a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1153a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1153b mov rbp,rsp
            rbp = rsp;
            // 1153e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11540 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11542 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 11544 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11545 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 11549 mov rbx,rdi
            rbx = rdi;
            // 1154c mov esi,0x8000100
            rsi = (0x8000100u64) as u32 as u64;
            // 11551 call 0xebf4e
            { let r = { let t = m.env.stub_va(0x71b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11556 test rax,rax
            fl = logic::<64>(rax & rax);
            // 11559 je 0x1156f
            if fl.e() { pc = 0x1156f; continue; }
            pc = 0x1155b;
        }
        0x1155b => {
            // block 1155b
            #[cfg(feature = "trace")] eprintln!("f_0001153a pc 1155b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1155b mov rdi,rax
            rdi = rax;
            // 1155e add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 11562 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11563 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11565 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11567 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11569 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1156a jmp 0xec2ba
            { let r = { let t = m.env.stub_va(0xa87); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x1156f => {
            // block 1156f
            #[cfg(feature = "trace")] eprintln!("f_0001153a pc 1156f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1156f mov rdi,rbx
            rdi = rbx;
            // 11572 call 0xebf54
            { let r = { let t = m.env.stub_va(0x721); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11577 mov r14,rax
            r14 = rax;
            // 1157a mov QWORD PTR [rsp+0x10],0x0
            m.w64(rsp.wrapping_add(0x10), (0x0u64) as u64);
            // 11583 mov QWORD PTR [rsp+0x8],0x0
            m.w64(rsp.wrapping_add(0x8), (0x0u64) as u64);
            // 1158c mov QWORD PTR [rsp],0x0
            m.w64(rsp, (0x0u64) as u64);
            // 11594 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 11596 mov ecx,0x8000100
            rcx = (0x8000100u64) as u32 as u64;
            // 1159b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 1159e xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // 115a1 mov rdi,rbx
            rdi = rbx;
            // 115a4 mov rdx,r14
            rdx = r14;
            // 115a7 call 0xebf48
            { let r = { let t = m.env.stub_va(0x715); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 115ac mov r15,rax
            r15 = rax;
            // 115af lea rdi,[r15+0x1]
            rdi = r15.wrapping_add(0x1);
            // 115b3 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 115b8 call 0xec1ac
            { let r = { let t = m.env.stub_va(0x979); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 115bd mov r12,rax
            r12 = rax;
            // 115c0 mov QWORD PTR [rsp+0x8],r15
            m.w64(rsp.wrapping_add(0x8), (r15) as u64);
            // 115c5 mov QWORD PTR [rsp],r12
            m.w64(rsp, (r12) as u64);
            // 115c9 mov QWORD PTR [rsp+0x10],0x0
            m.w64(rsp.wrapping_add(0x10), (0x0u64) as u64);
            // 115d2 mov esi,0x0
            rsi = (0x0u64) as u32 as u64;
            // 115d7 mov ecx,0x8000100
            rcx = (0x8000100u64) as u32 as u64;
            // 115dc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 115df xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // 115e2 mov rdi,rbx
            rdi = rbx;
            // 115e5 mov rdx,r14
            rdx = r14;
            // 115e8 call 0xebf48
            { let r = { let t = m.env.stub_va(0x715); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 115ed mov rax,r12
            rax = r12;
            // 115f0 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 115f4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 115f5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 115f7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 115f9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 115fb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 115fc ret
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
            { let r = { let t = m.env.stub_va(0x7e7); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0xe2b40 size=198
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2b40<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe2b40;
    loop {
        match pc {
        0xe2b40 => {
            // block e2b40
            #[cfg(feature = "trace")] eprintln!("f_000e2b40 pc e2b40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b40 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2b41 mov rbp,rsp
            rbp = rsp;
            // e2b44 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2b46 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2b48 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e2b4a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2b4c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2b4d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e2b4e mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // e2b51 mov r12,rdi
            r12 = rdi;
            // e2b54 mov al,BYTE PTR [rbp+0x10]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0x10)) as u64) as u8 as u64);
            // e2b57 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2b59 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // e2b5c ja 0xe2b79
            if fl.a() { pc = 0xe2b79; continue; }
            pc = 0xe2b5e;
        }
        0xe2b5e => {
            // block e2b5e
            #[cfg(feature = "trace")] eprintln!("f_000e2b40 pc e2b5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b5e mov BYTE PTR [rbp-0x29],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd7), ((rax as u8 as u64)) as u8);
            // e2b61 mov r14d,r9d
            r14 = ((r9 as u32 as u64)) as u32 as u64;
            // e2b64 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // e2b67 ja 0xe2b7f
            if fl.a() { pc = 0xe2b7f; continue; }
            pc = 0xe2b69;
        }
        0xe2b69 => {
            // block e2b69
            #[cfg(feature = "trace")] eprintln!("f_000e2b40 pc e2b69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b69 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // e2b6c ja 0xe2b7f
            if fl.a() { pc = 0xe2b7f; continue; }
            pc = 0xe2b6e;
        }
        0xe2b6e => {
            // block e2b6e
            #[cfg(feature = "trace")] eprintln!("f_000e2b40 pc e2b6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b6e lea eax,[rdx+rsi*4]
            rax = (rdx.wrapping_add(rsi.wrapping_mul(4))) as u32 as u64;
            // e2b71 lea eax,[rcx+rax*4]
            rax = (rcx.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e2b74 lea ebx,[rax+rax*2]
            rbx = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // e2b77 jmp 0xe2b7f
            pc = 0xe2b7f; continue;
        }
        0xe2b79 => {
            // block e2b79
            #[cfg(feature = "trace")] eprintln!("f_000e2b40 pc e2b79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b79 mov BYTE PTR [rbp-0x29],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd7), ((rax as u8 as u64)) as u8);
            // e2b7c mov r14d,r9d
            r14 = ((r9 as u32 as u64)) as u32 as u64;
            pc = 0xe2b7f;
        }
        0xe2b7f => {
            // block e2b7f
            #[cfg(feature = "trace")] eprintln!("f_000e2b40 pc e2b7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2b7f mov rdi,QWORD PTR [r12]
            rdi = m.r64(r12) as u64;
            // e2b83 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2b86 lea r13d,[rbx+0x43864]
            r13 = (rbx.wrapping_add(0x43864)) as u32 as u64;
            // e2b8d mov edx,0x80000000
            rdx = (0x80000000u64) as u32 as u64;
            // e2b92 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2b97 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2b9a mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // e2b9d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2ba0 mov rdi,QWORD PTR [r12]
            rdi = m.r64(r12) as u64;
            // e2ba4 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2ba7 lea esi,[rbx+0x43865]
            rsi = (rbx.wrapping_add(0x43865)) as u32 as u64;
            // e2bad mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2bb2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2bb5 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // e2bb8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2bbb mov rdi,QWORD PTR [r12]
            rdi = m.r64(r12) as u64;
            // e2bbf mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2bc2 add ebx,0x43866
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x43866u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2bc8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2bcd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2bd0 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e2bd2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // e2bd5 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2bd8 movzx eax,BYTE PTR [rbp-0x29]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd7)) as u64) as u32 as u64;
            // e2bdc lea edx,[rax+rax*1+0x1]
            rdx = (rax.wrapping_add(rax).wrapping_add(0x1)) as u32 as u64;
            // e2be0 mov rdi,QWORD PTR [r12]
            rdi = m.r64(r12) as u64;
            // e2be4 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2be7 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e2beb mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2bf0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2bf3 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // e2bf6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e2bfa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2bfb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2bfd pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e2bff pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2c01 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2c03 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2c04 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x60974); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2731e mov rdi,r15
            rdi = r15;
            // 27321 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2736d mov rdi,rbx
            rdi = rbx;
            // 27370 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2fd10 size=200
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002fd10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2fd10;
    loop {
        match pc {
        0x2fd10 => {
            // block 2fd10
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fd10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fd10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2fd11 mov rbp,rsp
            rbp = rsp;
            // 2fd14 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2fd16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2fd17 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2fd1b mov rbx,rdx
            rbx = rdx;
            // 2fd1e mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 2fd21 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 2fd24 shr eax,0x18
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x18u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 2fd27 mov BYTE PTR [rbp-0x11],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), ((rax as u8 as u64)) as u8);
            // 2fd2a mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2fd2e cmp rcx,QWORD PTR [rbx+0x10]
            fl = sub::<64>(rcx, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // 2fd32 jae 0x2fd43
            if fl.ae() { pc = 0x2fd43; continue; }
            pc = 0x2fd34;
        }
        0x2fd34 => {
            // block 2fd34
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fd34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fd34 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // 2fd36 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2fd3a inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 2fd3d mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 2fd41 jmp 0x2fd53
            pc = 0x2fd53; continue;
        }
        0x2fd43 => {
            // block 2fd43
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fd43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fd43 lea rsi,[rbp-0x11]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffef);
            // 2fd47 mov rdi,rbx
            rdi = rbx;
            // 2fd4a call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fd4f mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0x2fd53;
        }
        0x2fd53 => {
            // block 2fd53
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fd53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fd53 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 2fd56 shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 2fd59 mov BYTE PTR [rbp-0x12],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffee), ((rcx as u8 as u64)) as u8);
            // 2fd5c cmp rax,QWORD PTR [rbx+0x10]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // 2fd60 jae 0x2fd71
            if fl.ae() { pc = 0x2fd71; continue; }
            pc = 0x2fd62;
        }
        0x2fd62 => {
            // block 2fd62
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fd62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fd62 mov BYTE PTR [rax],cl
            m.w8(rax, ((rcx as u8 as u64)) as u8);
            // 2fd64 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2fd68 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 2fd6b mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 2fd6f jmp 0x2fd81
            pc = 0x2fd81; continue;
        }
        0x2fd71 => {
            // block 2fd71
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fd71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fd71 lea rsi,[rbp-0x12]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffee);
            // 2fd75 mov rdi,rbx
            rdi = rbx;
            // 2fd78 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fd7d mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0x2fd81;
        }
        0x2fd81 => {
            // block 2fd81
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fd81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fd81 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 2fd84 shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 2fd87 mov BYTE PTR [rbp-0x13],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffed), ((rcx as u8 as u64)) as u8);
            // 2fd8a cmp rax,QWORD PTR [rbx+0x10]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // 2fd8e jae 0x2fd9f
            if fl.ae() { pc = 0x2fd9f; continue; }
            pc = 0x2fd90;
        }
        0x2fd90 => {
            // block 2fd90
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fd90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fd90 mov BYTE PTR [rax],cl
            m.w8(rax, ((rcx as u8 as u64)) as u8);
            // 2fd92 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2fd96 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 2fd99 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 2fd9d jmp 0x2fdaf
            pc = 0x2fdaf; continue;
        }
        0x2fd9f => {
            // block 2fd9f
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fd9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fd9f lea rsi,[rbp-0x13]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffed);
            // 2fda3 mov rdi,rbx
            rdi = rbx;
            // 2fda6 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2fdab mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0x2fdaf;
        }
        0x2fdaf => {
            // block 2fdaf
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fdaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fdaf mov BYTE PTR [rbp-0x14],r14b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((r14 as u8 as u64)) as u8);
            // 2fdb3 cmp rax,QWORD PTR [rbx+0x10]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // 2fdb7 jae 0x2fdc2
            if fl.ae() { pc = 0x2fdc2; continue; }
            pc = 0x2fdb9;
        }
        0x2fdb9 => {
            // block 2fdb9
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fdb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fdb9 mov BYTE PTR [rax],r14b
            m.w8(rax, ((r14 as u8 as u64)) as u8);
            // 2fdbc inc QWORD PTR [rbx+0x8]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, fl); m.w64(rbx.wrapping_add(0x8), (r) as u64); fl = f; }
            // 2fdc0 jmp 0x2fdce
            pc = 0x2fdce; continue;
        }
        0x2fdc2 => {
            // block 2fdc2
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fdc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fdc2 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 2fdc6 mov rdi,rbx
            rdi = rbx;
            // 2fdc9 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2fdce;
        }
        0x2fdce => {
            // block 2fdce
            #[cfg(feature = "trace")] eprintln!("f_0002fd10 pc 2fdce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2fdce add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2fdd2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2fdd3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2fdd5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2fdd6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6c84 size=202
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6c84<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd6c84;
    loop {
        match pc {
        0xd6c84 => {
            // block d6c84
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6c84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c84 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6c85 mov rbp,rsp
            rbp = rsp;
            // d6c88 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6c89 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d6c8a mov rbx,rdi
            rbx = rdi;
            // d6c8d lea rax,[rip+0x6d77c] # 0x144410
            rax = m.env.data_va(363485);
            // d6c94 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // d6c97 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // d6c9e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6ca1 je 0xd6ca8
            if fl.e() { pc = 0xd6ca8; continue; }
            pc = 0xd6ca3;
        }
        0xd6ca3 => {
            // block d6ca3
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6ca3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ca3 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6ca8;
        }
        0xd6ca8 => {
            // block d6ca8
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6ca8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ca8 mov rdi,QWORD PTR [rbx+0x290]
            rdi = m.r64(rbx.wrapping_add(0x290)) as u64;
            // d6caf test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6cb2 je 0xd6cb9
            if fl.e() { pc = 0xd6cb9; continue; }
            pc = 0xd6cb4;
        }
        0xd6cb4 => {
            // block d6cb4
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6cb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6cb4 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6cb9;
        }
        0xd6cb9 => {
            // block d6cb9
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6cb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6cb9 mov rdi,QWORD PTR [rbx+0x6b8]
            rdi = m.r64(rbx.wrapping_add(0x6b8)) as u64;
            // d6cc0 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6cc3 je 0xd6ccb
            if fl.e() { pc = 0xd6ccb; continue; }
            pc = 0xd6cc5;
        }
        0xd6cc5 => {
            // block d6cc5
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6cc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6cc5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d6cc8 call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6ccb;
        }
        0xd6ccb => {
            // block d6ccb
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6ccb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ccb mov rdi,QWORD PTR [rbx+0x6a0]
            rdi = m.r64(rbx.wrapping_add(0x6a0)) as u64;
            // d6cd2 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6cd5 je 0xd6cec
            if fl.e() { pc = 0xd6cec; continue; }
            pc = 0xd6cd7;
        }
        0xd6cd7 => {
            // block d6cd7
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6cd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6cd7 cmp QWORD PTR [rbx+0x6a8],rdi
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x6a8)) as u64, rdi).1;
            // d6cde je 0xd6ce7
            if fl.e() { pc = 0xd6ce7; continue; }
            pc = 0xd6ce0;
        }
        0xd6ce0 => {
            // block d6ce0
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6ce0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ce0 mov QWORD PTR [rbx+0x6a8],rdi
            m.w64(rbx.wrapping_add(0x6a8), (rdi) as u64);
            pc = 0xd6ce7;
        }
        0xd6ce7 => {
            // block d6ce7
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6ce7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ce7 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6cec;
        }
        0xd6cec => {
            // block d6cec
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6cec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6cec lea rdi,[rbx+0x380]
            rdi = rbx.wrapping_add(0x380);
            // d6cf3 call 0xe807c
            { let r = { let t = m.env.code_va(0xe807c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6cf8 lea rdi,[rbx+0x318]
            rdi = rbx.wrapping_add(0x318);
            // d6cff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d04 lea rdi,[rbx+0x300]
            rdi = rbx.wrapping_add(0x300);
            // d6d0b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d10 lea rdi,[rbx+0x2e8]
            rdi = rbx.wrapping_add(0x2e8);
            // d6d17 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d1c lea rdi,[rbx+0x2d0]
            rdi = rbx.wrapping_add(0x2d0);
            // d6d23 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d28 lea rdi,[rbx+0x2b8]
            rdi = rbx.wrapping_add(0x2b8);
            // d6d2f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d34 lea rdi,[rbx+0x2a0]
            rdi = rbx.wrapping_add(0x2a0);
            // d6d3b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d40 mov rdi,rbx
            rdi = rbx;
            // d6d43 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d6d47 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6d48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6d49 jmp 0xb6894
            { let r = { let t = m.env.code_va(0xb6894); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7a34 size=204
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007a34<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x7a34;
    loop {
        match pc {
        0x7a34 => {
            // block 7a34
            #[cfg(feature = "trace")] eprintln!("f_00007a34 pc 7a34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a34 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7a35 mov rbp,rsp
            rbp = rsp;
            // 7a38 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7a3a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7a3c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7a3e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7a40 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7a41 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 7a42 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // 7a45 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 7a48 mov r15,rsi
            r15 = rsi;
            // 7a4b mov rbx,rdi
            rbx = rdi;
            // 7a4e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 7a51 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 7a54 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a5a mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 7a5c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7a5e cmp r12d,0x1
            fl = sub::<32>((r12 as u32 as u64), 0x1u64).1;
            // 7a62 ja 0x7af1
            if fl.a() { pc = 0x7af1; continue; }
            pc = 0x7a68;
        }
        0x7a68 => {
            // block 7a68
            #[cfg(feature = "trace")] eprintln!("f_00007a34 pc 7a68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a68 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 7a6b je 0x7af1
            if fl.e() { pc = 0x7af1; continue; }
            pc = 0x7a71;
        }
        0x7a71 => {
            // block 7a71
            #[cfg(feature = "trace")] eprintln!("f_00007a34 pc 7a71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a71 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 7a73 jne 0x7af1
            if fl.ne() { pc = 0x7af1; continue; }
            pc = 0x7a75;
        }
        0x7a75 => {
            // block 7a75
            #[cfg(feature = "trace")] eprintln!("f_00007a34 pc 7a75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a75 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 7a78 call 0xd0489
            { let r = { let t = m.env.code_va(0xd0489); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a7d mov cx,ax
            rcx = (rcx & !0xffff) | (((rax as u16 as u64)) as u16 as u64);
            // 7a80 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 7a82 test cx,cx
            fl = logic::<16>((rcx as u16 as u64) & (rcx as u16 as u64));
            // 7a85 je 0x7af1
            if fl.e() { pc = 0x7af1; continue; }
            pc = 0x7a87;
        }
        0x7a87 => {
            // block 7a87
            #[cfg(feature = "trace")] eprintln!("f_00007a34 pc 7a87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 7a8a mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 7a8f mov rdi,rbx
            rdi = rbx;
            // 7a92 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 7a95 call QWORD PTR [rax+0xf08]
            { let r = { let t = m.r64(rax.wrapping_add(0xf08)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7a9b mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 7a9e test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // 7aa1 je 0x7aeb
            if fl.e() { pc = 0x7aeb; continue; }
            pc = 0x7aa3;
        }
        0x7aa3 => {
            // block 7aa3
            #[cfg(feature = "trace")] eprintln!("f_00007a34 pc 7aa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7aa3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 7aa6 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 7aa9 lea rdx,[rip+0xf7a00] # 0xff4b0
            rdx = m.env.data_va(81021);
            // 7ab0 add r12d,DWORD PTR [rdx+rcx*4]
            { let (r, f) = add::<32>((r12 as u32 as u64), m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 7ab4 mov rdi,rbx
            rdi = rbx;
            // 7ab7 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7aba call QWORD PTR [rax+0x1000]
            { let r = { let t = m.r64(rax.wrapping_add(0x1000)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ac0 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 7ac3 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // 7ac6 je 0x7ada
            if fl.e() { pc = 0x7ada; continue; }
            pc = 0x7ac8;
        }
        0x7ac8 => {
            // block 7ac8
            #[cfg(feature = "trace")] eprintln!("f_00007a34 pc 7ac8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ac8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 7acb mov rdi,rbx
            rdi = rbx;
            // 7ace mov rsi,r15
            rsi = r15;
            // 7ad1 call QWORD PTR [rax+0xf10]
            { let r = { let t = m.r64(rax.wrapping_add(0xf10)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7ad7 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x7ada;
        }
        0x7ada => {
            // block 7ada
            #[cfg(feature = "trace")] eprintln!("f_00007a34 pc 7ada rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ada mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 7add xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 7adf mov rdi,rbx
            rdi = rbx;
            // 7ae2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 7ae5 call QWORD PTR [rax+0xf08]
            { let r = { let t = m.r64(rax.wrapping_add(0xf08)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7aeb;
        }
        0x7aeb => {
            // block 7aeb
            #[cfg(feature = "trace")] eprintln!("f_00007a34 pc 7aeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7aeb test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // 7aee setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x7af1;
        }
        0x7af1 => {
            // block 7af1
            #[cfg(feature = "trace")] eprintln!("f_00007a34 pc 7af1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7af1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 7af5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7af6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7af8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7afa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7afc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7afe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7aff ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43602 size=204
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043602<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43602;
    loop {
        match pc {
        0x43602 => {
            // block 43602
            #[cfg(feature = "trace")] eprintln!("f_00043602 pc 43602 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43602 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43603 mov rbp,rsp
            rbp = rsp;
            // 43606 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43608 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4360a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4360b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 4360c mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 4360f mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 43611 mov r15,rdi
            r15 = rdi;
            // 43614 cmp ebx,0xf
            fl = sub::<32>((rbx as u32 as u64), 0xfu64).1;
            // 43617 ja 0x43684
            if fl.a() { pc = 0x43684; continue; }
            pc = 0x43619;
        }
        0x43619 => {
            // block 43619
            #[cfg(feature = "trace")] eprintln!("f_00043602 pc 43619 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43619 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 4361c mov rdi,r15
            rdi = r15;
            // 4361f call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43625 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 43627 call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4362c cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 4362f jb 0x43688
            if fl.b() { pc = 0x43688; continue; }
            pc = 0x43631;
        }
        0x43631 => {
            // block 43631
            #[cfg(feature = "trace")] eprintln!("f_00043602 pc 43631 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43631 cmp ebx,0x1
            fl = sub::<32>((rbx as u32 as u64), 0x1u64).1;
            // 43634 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 43637 movzx ebx,al
            rbx = ((rax as u8 as u64)) as u32 as u64;
            // 4363a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 4363d mov esi,0x7f
            rsi = (0x7fu64) as u32 as u64;
            // 43642 mov ecx,0xf00000
            rcx = (0xf00000u64) as u32 as u64;
            // 43647 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 4364d mov rdi,r15
            rdi = r15;
            // 43650 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 43653 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43656 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43659 mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // 4365e mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // 43663 mov r8d,0x5
            r8 = (0x5u64) as u32 as u64;
            // 43669 mov rdi,r15
            rdi = r15;
            // 4366c mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 4366e call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43671 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 43674 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 43679 mov rdi,r15
            rdi = r15;
            // 4367c call QWORD PTR [rax+0x1350]
            { let r = { let t = m.r64(rax.wrapping_add(0x1350)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43682 jmp 0x436c2
            pc = 0x436c2; continue;
        }
        0x43684 => {
            // block 43684
            #[cfg(feature = "trace")] eprintln!("f_00043602 pc 43684 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43684 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43686 jmp 0x436c2
            pc = 0x436c2; continue;
        }
        0x43688 => {
            // block 43688
            #[cfg(feature = "trace")] eprintln!("f_00043602 pc 43688 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43688 lea eax,[r14*4+0x0]
            rax = (r14.wrapping_mul(4)) as u32 as u64;
            // 43690 shl r14d,0x6
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x6u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 43694 or r14d,eax
            { let r = (r14 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 43697 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 43699 mov eax,0x10
            rax = (0x10u64) as u32 as u64;
            // 4369e mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // 436a3 cmove edx,eax
            if fl.e() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // 436a6 or edx,r14d
            { let r = (rdx as u32 as u64) | (r14 as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 436a9 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 436ac mov esi,0xbe
            rsi = (0xbeu64) as u32 as u64;
            // 436b1 mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // 436b6 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // 436bc mov rdi,r15
            rdi = r15;
            // 436bf call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x436c2;
        }
        0x436c2 => {
            // block 436c2
            #[cfg(feature = "trace")] eprintln!("f_00043602 pc 436c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 436c2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 436c6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 436c7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 436c9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 436cb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 436cc ret
            return Ret { rax, rdx, xmm0: x0 };
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

/// parity: P2v @0xc3901 size=207
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3901<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3901;
    loop {
        match pc {
        0xc3901 => {
            // block c3901
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3901 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3901 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3902 mov rbp,rsp
            rbp = rsp;
            // c3905 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3907 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3909 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c390b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c390d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c390e sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // c3912 mov r14,rsi
            r14 = rsi;
            // c3915 mov rbx,rdi
            rbx = rdi;
            // c3918 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // c391b add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c391f cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // c3922 je 0xc39a9
            if fl.e() { pc = 0xc39a9; continue; }
            pc = 0xc3928;
        }
        0xc3928 => {
            // block c3928
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3928 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3928 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // c392c jmp 0xc3945
            pc = 0xc3945; continue;
        }
        0xc392e => {
            // block c392e
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c392e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c392e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // c3933 mov rdi,rbx
            rdi = rbx;
            // c3936 lea rsi,[rip+0x66e26] # 0x12a763
            rsi = m.env.data_va(257840);
            // c393d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3942 mov r13,r12
            r13 = r12;
            pc = 0xc3945;
        }
        0xc3945 => {
            // block c3945
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3945 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3945 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c3949 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // c394e mov rdi,r15
            rdi = r15;
            // c3951 call 0x5f77c
            { let r = { let t = m.env.code_va(0x5f77c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3956 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // c395a test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // c395d jne 0xc3968
            if fl.ne() { pc = 0xc3968; continue; }
            pc = 0xc395f;
        }
        0xc395f => {
            // block c395f
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c395f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c395f shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // c3962 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // c3966 jmp 0xc3970
            pc = 0xc3970; continue;
        }
        0xc3968 => {
            // block c3968
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3968 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c396c mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xc3970;
        }
        0xc3970 => {
            // block c3970
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3970 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3970 mov rdi,rbx
            rdi = rbx;
            // c3973 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3978 mov rdi,r15
            rdi = r15;
            // c397b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3980 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // c3984 test rax,rax
            fl = logic::<64>(rax & rax);
            // c3987 je 0xc3997
            if fl.e() { pc = 0xc3997; continue; }
            pc = 0xc3989;
        }
        0xc3989 => {
            // block c3989
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3989 mov r12,rax
            r12 = rax;
            // c398c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // c3990 test rax,rax
            fl = logic::<64>(rax & rax);
            // c3993 jne 0xc3989
            if fl.ne() { pc = 0xc3989; continue; }
            pc = 0xc3995;
        }
        0xc3995 => {
            // block c3995
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3995 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3995 jmp 0xc39a4
            pc = 0xc39a4; continue;
        }
        0xc3997 => {
            // block c3997
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3997 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3997 mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // c399b cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // c399f mov r13,r12
            r13 = r12;
            // c39a2 jne 0xc3997
            if fl.ne() { pc = 0xc3997; continue; }
            pc = 0xc39a4;
        }
        0xc39a4 => {
            // block c39a4
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c39a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c39a4 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // c39a7 jne 0xc392e
            if fl.ne() { pc = 0xc392e; continue; }
            pc = 0xc39a9;
        }
        0xc39a9 => {
            // block c39a9
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c39a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c39a9 mov rax,rbx
            rax = rbx;
            // c39ac add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // c39b0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c39b1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c39b3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c39b5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c39b7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c39b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c39ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc39bb => {
            // block c39bb
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c39bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c39bb mov rbx,rax
            rbx = rax;
            // c39be lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // c39c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c39c7 mov rdi,rbx
            rdi = rbx;
            // c39ca call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x74d2 size=208
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000074d2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x74d2;
    loop {
        match pc {
        0x74d2 => {
            // block 74d2
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 74d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 74d2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 74d3 mov rbp,rsp
            rbp = rsp;
            // 74d6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 74d8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 74da push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 74db push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 74dc mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 74df mov rbx,rdi
            rbx = rdi;
            // 74e2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 74e5 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 74eb xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 74ee test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 74f0 jne 0x7594
            if fl.ne() { pc = 0x7594; continue; }
            pc = 0x74f6;
        }
        0x74f6 => {
            // block 74f6
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 74f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 74f6 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 74f9 call 0xcfa3b
            { let r = { let t = m.env.code_va(0xcfa3b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 74fe test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 7501 je 0x7594
            if fl.e() { pc = 0x7594; continue; }
            pc = 0x7507;
        }
        0x7507 => {
            // block 7507
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 7507 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7507 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 750a call 0xcc106
            { let r = { let t = m.env.code_va(0xcc106); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 750f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7511 je 0x758e
            if fl.e() { pc = 0x758e; continue; }
            pc = 0x7513;
        }
        0x7513 => {
            // block 7513
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 7513 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7513 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 751a lea eax,[r14-0x4]
            rax = (r14.wrapping_add(0xffff_ffff_ffff_fffc)) as u32 as u64;
            // 751e cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 7521 jae 0x7531
            if fl.ae() { pc = 0x7531; continue; }
            pc = 0x7523;
        }
        0x7523 => {
            // block 7523
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 7523 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7523 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 7526 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 752a mov esi,0x1500
            rsi = (0x1500u64) as u32 as u64;
            // 752f jmp 0x7543
            pc = 0x7543; continue;
        }
        0x7531 => {
            // block 7531
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 7531 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7531 cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // 7535 ja 0x7594
            if fl.a() { pc = 0x7594; continue; }
            pc = 0x7537;
        }
        0x7537 => {
            // block 7537
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 7537 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7537 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 753a lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 753e mov esi,0x1400
            rsi = (0x1400u64) as u32 as u64;
            pc = 0x7543;
        }
        0x7543 => {
            // block 7543
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 7543 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7543 mov ecx,0x30000000
            rcx = (0x30000000u64) as u32 as u64;
            // 7548 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 754b mov rdi,rbx
            rdi = rbx;
            // 754e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7551 mov r15d,0x3
            r15 = (0x3u64) as u32 as u64;
            // 7557 cmp DWORD PTR [rbp-0x1c],0x30000000
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x30000000u64).1;
            // 755e je 0x7594
            if fl.e() { pc = 0x7594; continue; }
            pc = 0x7560;
        }
        0x7560 => {
            // block 7560
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 7560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7560 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 7563 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 7566 lea rdx,[rip+0xf7ee3] # 0xff450
            rdx = m.env.data_va(80925);
            // 756d mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 7570 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 7574 mov ecx,0x30000000
            rcx = (0x30000000u64) as u32 as u64;
            // 7579 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 757c mov rdi,rbx
            rdi = rbx;
            // 757f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7582 mov r15d,0x2
            r15 = (0x2u64) as u32 as u64;
            // 7588 test BYTE PTR [rbp-0x19],0x20
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe7)) as u64 & 0x20u64);
            // 758c jne 0x7594
            if fl.ne() { pc = 0x7594; continue; }
            pc = 0x758e;
        }
        0x758e => {
            // block 758e
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 758e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 758e mov r15d,0x1
            r15 = (0x1u64) as u32 as u64;
            pc = 0x7594;
        }
        0x7594 => {
            // block 7594
            #[cfg(feature = "trace")] eprintln!("f_000074d2 pc 7594 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7594 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 7597 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 759b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 759c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 759e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 75a0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 75a1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ec36 size=208
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002ec36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x2ec36;
    loop {
        match pc {
        0x2ec36 => {
            // block 2ec36
            #[cfg(feature = "trace")] eprintln!("f_0002ec36 pc 2ec36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ec36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ec37 mov rbp,rsp
            rbp = rsp;
            // 2ec3a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2ec3c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2ec3d mov rbx,rdi
            rbx = rdi;
            // 2ec40 mov r10b,BYTE PTR [rbp+0x28]
            r10 = (r10 & !0xff) | ((m.r8(rbp.wrapping_add(0x28)) as u64) as u8 as u64);
            // 2ec44 mov r14b,BYTE PTR [rbp+0x20]
            r14 = (r14 & !0xff) | ((m.r8(rbp.wrapping_add(0x20)) as u64) as u8 as u64);
            // 2ec48 mov edi,DWORD PTR [rbp+0x18]
            rdi = (m.r32(rbp.wrapping_add(0x18)) as u64) as u32 as u64;
            // 2ec4b mov r11b,BYTE PTR [rbp+0x10]
            r11 = (r11 & !0xff) | ((m.r8(rbp.wrapping_add(0x10)) as u64) as u8 as u64);
            // 2ec4f lea rax,[rip+0x11293a] # 0x141590
            rax = m.env.data_va(351581);
            // 2ec56 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 2ec59 mov QWORD PTR [rbx+0x8],rsi
            m.w64(rbx.wrapping_add(0x8), (rsi) as u64);
            // 2ec5d mov DWORD PTR [rbx+0x10],edx
            m.w32(rbx.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // 2ec60 mov DWORD PTR [rbx+0x14],ecx
            m.w32(rbx.wrapping_add(0x14), ((rcx as u32 as u64)) as u32);
            // 2ec63 mov DWORD PTR [rbx+0x18],r8d
            m.w32(rbx.wrapping_add(0x18), ((r8 as u32 as u64)) as u32);
            // 2ec67 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 2ec69 lea rdx,[rip+0xd10d0] # 0xffd40
            rdx = m.env.data_va(83213);
            // 2ec70 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x2ec72;
        }
        0x2ec72 => {
            // block 2ec72
            #[cfg(feature = "trace")] eprintln!("f_0002ec36 pc 2ec72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ec72 cmp DWORD PTR [rdx+rcx*8],edi
            fl = sub::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(8))) as u64, (rdi as u32 as u64)).1;
            // 2ec75 je 0x2ec81
            if fl.e() { pc = 0x2ec81; continue; }
            pc = 0x2ec77;
        }
        0x2ec77 => {
            // block 2ec77
            #[cfg(feature = "trace")] eprintln!("f_0002ec36 pc 2ec77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ec77 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 2ec7a cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 2ec7d jbe 0x2ec72
            if fl.be() { pc = 0x2ec72; continue; }
            pc = 0x2ec7f;
        }
        0x2ec7f => {
            // block 2ec7f
            #[cfg(feature = "trace")] eprintln!("f_0002ec36 pc 2ec7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ec7f jmp 0x2ec85
            pc = 0x2ec85; continue;
        }
        0x2ec81 => {
            // block 2ec81
            #[cfg(feature = "trace")] eprintln!("f_0002ec36 pc 2ec81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ec81 mov esi,DWORD PTR [rdx+rcx*8+0x4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4)) as u64) as u32 as u64;
            pc = 0x2ec85;
        }
        0x2ec85 => {
            // block 2ec85
            #[cfg(feature = "trace")] eprintln!("f_0002ec36 pc 2ec85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ec85 mov DWORD PTR [rbx+0x1c],esi
            m.w32(rbx.wrapping_add(0x1c), ((rsi as u32 as u64)) as u32);
            // 2ec88 mov DWORD PTR [rbx+0x20],edi
            m.w32(rbx.wrapping_add(0x20), ((rdi as u32 as u64)) as u32);
            // 2ec8b movsd QWORD PTR [rbx+0x28],xmm0
            m.w64(rbx.wrapping_add(0x28), x0 as u64);
            // 2ec90 movsd QWORD PTR [rbx+0x30],xmm1
            m.w64(rbx.wrapping_add(0x30), x1 as u64);
            // 2ec95 mov BYTE PTR [rbx+0x38],r14b
            m.w8(rbx.wrapping_add(0x38), ((r14 as u8 as u64)) as u8);
            // 2ec99 mov BYTE PTR [rbx+0x39],r10b
            m.w8(rbx.wrapping_add(0x39), ((r10 as u8 as u64)) as u8);
            // 2ec9d mov QWORD PTR [rbx+0x58],0x0
            m.w64(rbx.wrapping_add(0x58), (0x0u64) as u64);
            // 2eca5 mov QWORD PTR [rbx+0x50],0x0
            m.w64(rbx.wrapping_add(0x50), (0x0u64) as u64);
            // 2ecad mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // 2ecb5 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 2ecbd mov BYTE PTR [rbx+0x60],r11b
            m.w8(rbx.wrapping_add(0x60), ((r11 as u8 as u64)) as u8);
            // 2ecc1 mov BYTE PTR [rbx+0x61],0x0
            m.w8(rbx.wrapping_add(0x61), (0x0u64) as u8);
            // 2ecc5 mov DWORD PTR [rbx+0x64],0x1
            m.w32(rbx.wrapping_add(0x64), (0x1u64) as u32);
            // 2eccc mov QWORD PTR [rbx+0x68],0x0
            m.w64(rbx.wrapping_add(0x68), (0x0u64) as u64);
            // 2ecd4 test r9,r9
            fl = logic::<64>(r9 & r9);
            // 2ecd7 je 0x2ecee
            if fl.e() { pc = 0x2ecee; continue; }
            pc = 0x2ecd9;
        }
        0x2ecd9 => {
            // block 2ecd9
            #[cfg(feature = "trace")] eprintln!("f_0002ec36 pc 2ecd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ecd9 cmp BYTE PTR [r9],0x0
            fl = sub::<8>(m.r8(r9) as u64, 0x0u64).1;
            // 2ecdd je 0x2ecee
            if fl.e() { pc = 0x2ecee; continue; }
            pc = 0x2ecdf;
        }
        0x2ecdf => {
            // block 2ecdf
            #[cfg(feature = "trace")] eprintln!("f_0002ec36 pc 2ecdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ecdf add rbx,0x48
            { let (r, f) = add::<64>(rbx, 0x48u64);
              rbx = r;
              fl = f; }
            // 2ece3 mov rdi,rbx
            rdi = rbx;
            // 2ece6 mov rsi,r9
            rsi = r9;
            // 2ece9 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x2ecee;
        }
        0x2ecee => {
            // block 2ecee
            #[cfg(feature = "trace")] eprintln!("f_0002ec36 pc 2ecee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ecee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2ecef pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2ecf1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ecf2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2ecf3 => {
            // block 2ecf3
            #[cfg(feature = "trace")] eprintln!("f_0002ec36 pc 2ecf3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ecf3 mov r14,rax
            r14 = rax;
            // 2ecf6 mov rdi,rbx
            rdi = rbx;
            // 2ecf9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 2ecfe mov rdi,r14
            rdi = r14;
            // 2ed01 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x445f0 size=210
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000445f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x445f0;
    loop {
        match pc {
        0x445f0 => {
            // block 445f0
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 445f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 445f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 445f1 mov rbp,rsp
            rbp = rsp;
            // 445f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 445f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 445f8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 445fa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 445fc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 445fd sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 44601 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 44604 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 44606 mov r15,rdi
            r15 = rdi;
            // 44609 lea rax,[r15+0x8]
            rax = r15.wrapping_add(0x8);
            // 4460d mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 44611 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // 44619 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 44621 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 44624 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 44626 je 0x4465e
            if fl.e() { pc = 0x4465e; continue; }
            pc = 0x44628;
        }
        0x44628 => {
            // block 44628
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 44628 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44628 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 4462f mov r13d,0x1
            r13 = (0x1u64) as u32 as u64;
            // 44635 lea r12,[rbp-0x2c]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            pc = 0x44639;
        }
        0x44639 => {
            // block 44639
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 44639 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44639 lea eax,[r13-0x1]
            rax = (r13.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 4463d bt ebx,eax
            fl.cf = bit::<32>((rbx as u32 as u64), (rax as u32 as u64));
            // 44640 jae 0x4464d
            if fl.ae() { pc = 0x4464d; continue; }
            pc = 0x44642;
        }
        0x44642 => {
            // block 44642
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 44642 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44642 mov rdi,r15
            rdi = r15;
            // 44645 mov rsi,r12
            rsi = r12;
            // 44648 call 0xebb52
            { let r = { let t = m.env.stub_va(0x31f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x4464d;
        }
        0x4464d => {
            // block 4464d
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 4464d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4464d mov DWORD PTR [rbp-0x2c],r13d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r13 as u32 as u64)) as u32);
            // 44651 cmp r13d,0x8
            fl = sub::<32>((r13 as u32 as u64), 0x8u64).1;
            // 44655 lea eax,[r13+0x1]
            rax = (r13.wrapping_add(0x1)) as u32 as u64;
            // 44659 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 4465c jl 0x44639
            if fl.l() { pc = 0x44639; continue; }
            pc = 0x4465e;
        }
        0x4465e => {
            // block 4465e
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 4465e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4465e test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // 44661 je 0x44694
            if fl.e() { pc = 0x44694; continue; }
            pc = 0x44663;
        }
        0x44663 => {
            // block 44663
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 44663 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44663 mov DWORD PTR [rbp-0x30],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x8u64) as u32);
            // 4466a mov ebx,0x9
            rbx = (0x9u64) as u32 as u64;
            // 4466f lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0x44673;
        }
        0x44673 => {
            // block 44673
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 44673 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44673 lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 44676 bt r14d,eax
            fl.cf = bit::<32>((r14 as u32 as u64), (rax as u32 as u64));
            // 4467a jae 0x44687
            if fl.ae() { pc = 0x44687; continue; }
            pc = 0x4467c;
        }
        0x4467c => {
            // block 4467c
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 4467c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4467c mov rdi,r15
            rdi = r15;
            // 4467f mov rsi,r12
            rsi = r12;
            // 44682 call 0xebb52
            { let r = { let t = m.env.stub_va(0x31f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x44687;
        }
        0x44687 => {
            // block 44687
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 44687 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44687 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // 4468a cmp ebx,0x40
            fl = sub::<32>((rbx as u32 as u64), 0x40u64).1;
            // 4468d lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 44690 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 44692 jl 0x44673
            if fl.l() { pc = 0x44673; continue; }
            pc = 0x44694;
        }
        0x44694 => {
            // block 44694
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 44694 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44694 mov rax,r15
            rax = r15;
            // 44697 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4469b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4469c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4469e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 446a0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 446a2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 446a4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 446a5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x446a6 => {
            // block 446a6
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 446a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 446a6 jmp 0x446a8
            pc = 0x446a8; continue;
        }
        0x446a8 => {
            // block 446a8
            #[cfg(feature = "trace")] eprintln!("f_000445f0 pc 446a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 446a8 mov rbx,rax
            rbx = rax;
            // 446ab mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 446af mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 446b2 mov rdi,r15
            rdi = r15;
            // 446b5 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 446ba mov rdi,rbx
            rdi = rbx;
            // 446bd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x2847e size=211
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002847e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2847e;
    loop {
        match pc {
        0x2847e => {
            // block 2847e
            #[cfg(feature = "trace")] eprintln!("f_0002847e pc 2847e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2847e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2847f mov rbp,rsp
            rbp = rsp;
            // 28482 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28484 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28485 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 28489 mov rbx,rsi
            rbx = rsi;
            // 2848c lea rsi,[rip+0x102319] # 0x12a7ac
            rsi = m.env.data_va(257913);
            // 28493 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 28498 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2849d mov r14,rax
            r14 = rax;
            // 284a0 mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 284a2 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 284a6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 284a8 call 0x64324
            { let r = { let t = m.env.code_va(0x64324); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 284ad movzx edx,BYTE PTR [rbp-0x28]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 284b1 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 284b4 jne 0x284bf
            if fl.ne() { pc = 0x284bf; continue; }
            pc = 0x284b6;
        }
        0x284b6 => {
            // block 284b6
            #[cfg(feature = "trace")] eprintln!("f_0002847e pc 284b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 284b6 lea rsi,[rbp-0x27]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd9);
            // 284ba shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 284bd jmp 0x284c7
            pc = 0x284c7; continue;
        }
        0x284bf => {
            // block 284bf
            #[cfg(feature = "trace")] eprintln!("f_0002847e pc 284bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 284bf mov rdx,QWORD PTR [rbp-0x20]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 284c3 mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            pc = 0x284c7;
        }
        0x284c7 => {
            // block 284c7
            #[cfg(feature = "trace")] eprintln!("f_0002847e pc 284c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 284c7 mov rdi,r14
            rdi = r14;
            // 284ca call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 284cf lea rsi,[rip+0x1022df] # 0x12a7b5
            rsi = m.env.data_va(257922);
            // 284d6 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 284db mov rdi,rax
            rdi = rax;
            // 284de call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 284e3 mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 284e6 mov rdi,rax
            rdi = rax;
            // 284e9 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 284ee lea rsi,[rip+0x1022cc] # 0x12a7c1
            rsi = m.env.data_va(257934);
            // 284f5 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 284fa mov rdi,rax
            rdi = rax;
            // 284fd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28502 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 28506 mov rdi,rbx
            rdi = rbx;
            // 28509 mov rsi,rax
            rsi = rax;
            // 2850c call 0x26b10
            { let r = { let t = m.env.code_va(0x26b10); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28511 lea rsi,[rip+0x101f76] # 0x12a48e
            rsi = m.env.data_va(257115);
            // 28518 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2851d mov rdi,rax
            rdi = rax;
            // 28520 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28525 mov rbx,rax
            rbx = rax;
            // 28528 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2852c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28531 mov rax,rbx
            rax = rbx;
            // 28534 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 28538 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28539 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2853b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2853c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2853d => {
            // block 2853d
            #[cfg(feature = "trace")] eprintln!("f_0002847e pc 2853d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2853d mov rbx,rax
            rbx = rax;
            // 28540 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 28544 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28549 mov rdi,rbx
            rdi = rbx;
            // 2854c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x647f0 size=211
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000647f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x647f0;
    loop {
        match pc {
        0x647f0 => {
            // block 647f0
            #[cfg(feature = "trace")] eprintln!("f_000647f0 pc 647f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 647f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 647f1 mov rbp,rsp
            rbp = rsp;
            // 647f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 647f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 647f8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 647fa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 647fc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 647fd sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 64801 mov r13,rsi
            r13 = rsi;
            // 64804 mov r14,rdi
            r14 = rdi;
            // 64807 lea rsi,[rip+0xc5c48] # 0x12a456
            rsi = m.env.data_va(257059);
            // 6480e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 64813 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64818 mov rbx,QWORD PTR [r13+0x0]
            rbx = m.r64(r13) as u64;
            // 6481c cmp rbx,QWORD PTR [r13+0x8]
            fl = sub::<64>(rbx, m.r64(r13.wrapping_add(0x8)) as u64).1;
            // 64820 je 0x64889
            if fl.e() { pc = 0x64889; continue; }
            pc = 0x64822;
        }
        0x64822 => {
            // block 64822
            #[cfg(feature = "trace")] eprintln!("f_000647f0 pc 64822 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64822 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 64826 lea r12,[rip+0xc5f36] # 0x12a763
            r12 = m.env.data_va(257840);
            pc = 0x6482d;
        }
        0x6482d => {
            // block 6482d
            #[cfg(feature = "trace")] eprintln!("f_000647f0 pc 6482d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6482d mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 6482f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 64831 mov rdi,r15
            rdi = r15;
            // 64834 call 0x5e61c
            { let r = { let t = m.env.code_va(0x5e61c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64839 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 6483d test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 64840 jne 0x6484b
            if fl.ne() { pc = 0x6484b; continue; }
            pc = 0x64842;
        }
        0x64842 => {
            // block 64842
            #[cfg(feature = "trace")] eprintln!("f_000647f0 pc 64842 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64842 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 64845 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 64849 jmp 0x64853
            pc = 0x64853; continue;
        }
        0x6484b => {
            // block 6484b
            #[cfg(feature = "trace")] eprintln!("f_000647f0 pc 6484b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6484b mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 6484f mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x64853;
        }
        0x64853 => {
            // block 64853
            #[cfg(feature = "trace")] eprintln!("f_000647f0 pc 64853 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64853 mov rdi,r14
            rdi = r14;
            // 64856 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6485b mov rdi,r15
            rdi = r15;
            // 6485e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64863 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 64867 cmp rbx,QWORD PTR [r13+0x8]
            fl = sub::<64>(rbx, m.r64(r13.wrapping_add(0x8)) as u64).1;
            // 6486b mov rax,rbx
            rax = rbx;
            // 6486e je 0x64884
            if fl.e() { pc = 0x64884; continue; }
            pc = 0x64870;
        }
        0x64870 => {
            // block 64870
            #[cfg(feature = "trace")] eprintln!("f_000647f0 pc 64870 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64870 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 64875 mov rdi,r14
            rdi = r14;
            // 64878 mov rsi,r12
            rsi = r12;
            // 6487b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64880 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0x64884;
        }
        0x64884 => {
            // block 64884
            #[cfg(feature = "trace")] eprintln!("f_000647f0 pc 64884 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64884 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 64887 jne 0x6482d
            if fl.ne() { pc = 0x6482d; continue; }
            pc = 0x64889;
        }
        0x64889 => {
            // block 64889
            #[cfg(feature = "trace")] eprintln!("f_000647f0 pc 64889 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64889 lea rsi,[rip+0xc5bc4] # 0x12a454
            rsi = m.env.data_va(257057);
            // 64890 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 64895 mov rdi,r14
            rdi = r14;
            // 64898 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6489d mov rax,r14
            rax = r14;
            // 648a0 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 648a4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 648a5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 648a7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 648a9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 648ab pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 648ad pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 648ae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x648af => {
            // block 648af
            #[cfg(feature = "trace")] eprintln!("f_000647f0 pc 648af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 648af mov rbx,rax
            rbx = rax;
            // 648b2 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 648b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 648bb mov rdi,rbx
            rdi = rbx;
            // 648be call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x648c3 size=211
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000648c3<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x648c3;
    loop {
        match pc {
        0x648c3 => {
            // block 648c3
            #[cfg(feature = "trace")] eprintln!("f_000648c3 pc 648c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 648c3 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 648c4 mov rbp,rsp
            rbp = rsp;
            // 648c7 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 648c9 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 648cb push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 648cd push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 648cf push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 648d0 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 648d4 mov r13,rsi
            r13 = rsi;
            // 648d7 mov r14,rdi
            r14 = rdi;
            // 648da lea rsi,[rip+0xc5b75] # 0x12a456
            rsi = m.env.data_va(257059);
            // 648e1 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 648e6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 648eb mov rbx,QWORD PTR [r13+0x0]
            rbx = m.r64(r13) as u64;
            // 648ef cmp rbx,QWORD PTR [r13+0x8]
            fl = sub::<64>(rbx, m.r64(r13.wrapping_add(0x8)) as u64).1;
            // 648f3 je 0x6495c
            if fl.e() { pc = 0x6495c; continue; }
            pc = 0x648f5;
        }
        0x648f5 => {
            // block 648f5
            #[cfg(feature = "trace")] eprintln!("f_000648c3 pc 648f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 648f5 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 648f9 lea r12,[rip+0xc5e63] # 0x12a763
            r12 = m.env.data_va(257840);
            pc = 0x64900;
        }
        0x64900 => {
            // block 64900
            #[cfg(feature = "trace")] eprintln!("f_000648c3 pc 64900 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64900 mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 64902 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 64904 mov rdi,r15
            rdi = r15;
            // 64907 call 0x5d8db
            { let r = { let t = m.env.code_va(0x5d8db); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6490c movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 64910 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 64913 jne 0x6491e
            if fl.ne() { pc = 0x6491e; continue; }
            pc = 0x64915;
        }
        0x64915 => {
            // block 64915
            #[cfg(feature = "trace")] eprintln!("f_000648c3 pc 64915 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64915 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 64918 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 6491c jmp 0x64926
            pc = 0x64926; continue;
        }
        0x6491e => {
            // block 6491e
            #[cfg(feature = "trace")] eprintln!("f_000648c3 pc 6491e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6491e mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 64922 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x64926;
        }
        0x64926 => {
            // block 64926
            #[cfg(feature = "trace")] eprintln!("f_000648c3 pc 64926 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64926 mov rdi,r14
            rdi = r14;
            // 64929 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6492e mov rdi,r15
            rdi = r15;
            // 64931 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64936 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // 6493a cmp rbx,QWORD PTR [r13+0x8]
            fl = sub::<64>(rbx, m.r64(r13.wrapping_add(0x8)) as u64).1;
            // 6493e mov rax,rbx
            rax = rbx;
            // 64941 je 0x64957
            if fl.e() { pc = 0x64957; continue; }
            pc = 0x64943;
        }
        0x64943 => {
            // block 64943
            #[cfg(feature = "trace")] eprintln!("f_000648c3 pc 64943 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64943 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 64948 mov rdi,r14
            rdi = r14;
            // 6494b mov rsi,r12
            rsi = r12;
            // 6494e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64953 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0x64957;
        }
        0x64957 => {
            // block 64957
            #[cfg(feature = "trace")] eprintln!("f_000648c3 pc 64957 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64957 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 6495a jne 0x64900
            if fl.ne() { pc = 0x64900; continue; }
            pc = 0x6495c;
        }
        0x6495c => {
            // block 6495c
            #[cfg(feature = "trace")] eprintln!("f_000648c3 pc 6495c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6495c lea rsi,[rip+0xc5af1] # 0x12a454
            rsi = m.env.data_va(257057);
            // 64963 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 64968 mov rdi,r14
            rdi = r14;
            // 6496b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64970 mov rax,r14
            rax = r14;
            // 64973 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 64977 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64978 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6497a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 6497c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6497e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 64980 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64981 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x64982 => {
            // block 64982
            #[cfg(feature = "trace")] eprintln!("f_000648c3 pc 64982 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64982 mov rbx,rax
            rbx = rax;
            // 64985 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 64989 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6498e mov rdi,rbx
            rdi = rbx;
            // 64991 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15aa0 size=212
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00015aa0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x15aa0;
    loop {
        match pc {
        0x15aa0 => {
            // block 15aa0
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15aa0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15aa0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15aa1 mov rbp,rsp
            rbp = rsp;
            // 15aa4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15aa6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15aa8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15aaa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15aac push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15aad sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 15ab1 mov rbx,rcx
            rbx = rcx;
            // 15ab4 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 15ab7 je 0x15b61
            if fl.e() { pc = 0x15b61; continue; }
            pc = 0x15abd;
        }
        0x15abd => {
            // block 15abd
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15abd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15abd movzx r14d,BYTE PTR [rsi]
            r14 = (m.r8(rsi) as u64) as u32 as u64;
            // 15ac1 mov r15b,r14b
            r15 = (r15 & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 15ac4 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15ac8 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 15acb mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 15acf mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 15ad3 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // 15ad7 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 15adb lea rax,[rsi+0x1]
            rax = rsi.wrapping_add(0x1);
            // 15adf mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0x15ae3;
        }
        0x15ae3 => {
            // block 15ae3
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15ae3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ae3 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 15ae7 mov rbx,rdx
            rbx = rdx;
            pc = 0x15aea;
        }
        0x15aea => {
            // block 15aea
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15aea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15aea movzx r13d,BYTE PTR [rbx+0x20]
            r13 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 15aef mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 15af2 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15af4 jne 0x15afb
            if fl.ne() { pc = 0x15afb; continue; }
            pc = 0x15af6;
        }
        0x15af6 => {
            // block 15af6
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15af6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15af6 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // 15af9 jmp 0x15aff
            pc = 0x15aff; continue;
        }
        0x15afb => {
            // block 15afb
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15afb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15afb mov r13,QWORD PTR [rbx+0x28]
            r13 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x15aff;
        }
        0x15aff => {
            // block 15aff
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15aff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15aff test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 15b02 mov r12,QWORD PTR [rbp-0x30]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 15b06 cmove r12,r14
            if fl.e() { r12 = r14; }
            // 15b0a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 15b0c jne 0x15b17
            if fl.ne() { pc = 0x15b17; continue; }
            pc = 0x15b0e;
        }
        0x15b0e => {
            // block 15b0e
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b0e lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 15b12 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 15b15 jmp 0x15b1b
            pc = 0x15b1b; continue;
        }
        0x15b17 => {
            // block 15b17
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b17 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x15b1b;
        }
        0x15b1b => {
            // block 15b1b
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b1b cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 15b1e mov rdx,r13
            rdx = r13;
            // 15b21 cmovb rdx,r12
            if fl.b() { rdx = r12; }
            // 15b25 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 15b28 je 0x15b43
            if fl.e() { pc = 0x15b43; continue; }
            pc = 0x15b2a;
        }
        0x15b2a => {
            // block 15b2a
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b2a test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 15b2d mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 15b31 cmove rsi,QWORD PTR [rbp-0x40]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64; }
            // 15b36 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15b3b test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 15b3d je 0x15b43
            if fl.e() { pc = 0x15b43; continue; }
            pc = 0x15b3f;
        }
        0x15b3f => {
            // block 15b3f
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b3f js 0x15b48
            if fl.s() { pc = 0x15b48; continue; }
            pc = 0x15b41;
        }
        0x15b41 => {
            // block 15b41
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b41 jmp 0x15b53
            pc = 0x15b53; continue;
        }
        0x15b43 => {
            // block 15b43
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b43 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 15b46 jae 0x15b53
            if fl.ae() { pc = 0x15b53; continue; }
            pc = 0x15b48;
        }
        0x15b48 => {
            // block 15b48
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b48 mov rbx,QWORD PTR [rbx+0x8]
            rbx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 15b4c test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 15b4f jne 0x15aea
            if fl.ne() { pc = 0x15aea; continue; }
            pc = 0x15b51;
        }
        0x15b51 => {
            // block 15b51
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b51 jmp 0x15b5d
            pc = 0x15b5d; continue;
        }
        0x15b53 => {
            // block 15b53
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b53 mov rdx,QWORD PTR [rbx]
            rdx = m.r64(rbx) as u64;
            // 15b56 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 15b59 jne 0x15ae3
            if fl.ne() { pc = 0x15ae3; continue; }
            pc = 0x15b5b;
        }
        0x15b5b => {
            // block 15b5b
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b5b jmp 0x15b61
            pc = 0x15b61; continue;
        }
        0x15b5d => {
            // block 15b5d
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b5d mov rbx,QWORD PTR [rbp-0x48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0x15b61;
        }
        0x15b61 => {
            // block 15b61
            #[cfg(feature = "trace")] eprintln!("f_00015aa0 pc 15b61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15b61 mov rax,rbx
            rax = rbx;
            // 15b64 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 15b68 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15b69 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15b6b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15b6d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15b6f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15b71 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15b72 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x32c72 size=212
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00032c72<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x32c72;
    loop {
        match pc {
        0x32c72 => {
            // block 32c72
            #[cfg(feature = "trace")] eprintln!("f_00032c72 pc 32c72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c72 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 32c73 mov rbp,rsp
            rbp = rsp;
            // 32c76 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 32c78 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 32c7a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 32c7c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 32c7e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 32c7f sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 32c83 mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // 32c86 mov r13d,ecx
            r13 = ((rcx as u32 as u64)) as u32 as u64;
            // 32c89 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 32c8c mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 32c8f mov rbx,rdi
            rbx = rdi;
            // 32c92 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // 32c95 call 0x5a914
            { let r = { let t = m.env.code_va(0x5a914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32c9a mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            // 32c9c cmp r14d,0x55
            fl = sub::<32>((r14 as u32 as u64), 0x55u64).1;
            // 32ca0 jg 0x32cb5
            if fl.g() { pc = 0x32cb5; continue; }
            pc = 0x32ca2;
        }
        0x32ca2 => {
            // block 32ca2
            #[cfg(feature = "trace")] eprintln!("f_00032c72 pc 32ca2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ca2 cmp r14d,0x1b
            fl = sub::<32>((r14 as u32 as u64), 0x1bu64).1;
            // 32ca6 ja 0x32cce
            if fl.a() { pc = 0x32cce; continue; }
            pc = 0x32ca8;
        }
        0x32ca8 => {
            // block 32ca8
            #[cfg(feature = "trace")] eprintln!("f_00032c72 pc 32ca8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ca8 mov ecx,0xc01e000
            rcx = (0xc01e000u64) as u32 as u64;
            // 32cad bt ecx,r14d
            fl.cf = bit::<32>((rcx as u32 as u64), (r14 as u32 as u64));
            // 32cb1 jb 0x32cd0
            if fl.b() { pc = 0x32cd0; continue; }
            pc = 0x32cb3;
        }
        0x32cb3 => {
            // block 32cb3
            #[cfg(feature = "trace")] eprintln!("f_00032c72 pc 32cb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32cb3 jmp 0x32cce
            pc = 0x32cce; continue;
        }
        0x32cb5 => {
            // block 32cb5
            #[cfg(feature = "trace")] eprintln!("f_00032c72 pc 32cb5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32cb5 lea ecx,[r14-0x56]
            rcx = (r14.wrapping_add(0xffff_ffff_ffff_ffaa)) as u32 as u64;
            // 32cb9 cmp ecx,0x23
            fl = sub::<32>((rcx as u32 as u64), 0x23u64).1;
            // 32cbc ja 0x32cce
            if fl.a() { pc = 0x32cce; continue; }
            pc = 0x32cbe;
        }
        0x32cbe => {
            // block 32cbe
            #[cfg(feature = "trace")] eprintln!("f_00032c72 pc 32cbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32cbe movabs rsi,0xffffe3c3f
            rsi = 0x0_000f_fffe_3c3fu64;
            // 32cc8 bt rsi,rcx
            fl.cf = bit::<64>(rsi, rcx);
            // 32ccc jb 0x32cd0
            if fl.b() { pc = 0x32cd0; continue; }
            pc = 0x32cce;
        }
        0x32cce => {
            // block 32cce
            #[cfg(feature = "trace")] eprintln!("f_00032c72 pc 32cce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32cce xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x32cd0;
        }
        0x32cd0 => {
            // block 32cd0
            #[cfg(feature = "trace")] eprintln!("f_00032c72 pc 32cd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32cd0 movzx ecx,r13b
            rcx = ((r13 as u8 as u64)) as u32 as u64;
            // 32cd4 movzx r8d,dl
            r8 = ((rdx as u8 as u64)) as u32 as u64;
            // 32cd8 movzx r9d,r12b
            r9 = ((r12 as u8 as u64)) as u32 as u64;
            // 32cdc lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 32ce0 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 32ce2 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 32ce5 call 0x32996
            { let r = { let t = m.env.code_va(0x32996); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32cea mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 32ced mov DWORD PTR [rbx+0x38],eax
            m.w32(rbx.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32cf0 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 32cf4 mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // 32cf8 mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 32cfc mov QWORD PTR [rbx+0x28],rax
            m.w64(rbx.wrapping_add(0x28), (rax) as u64);
            // 32d00 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 32d04 mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // 32d08 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 32d0c mov QWORD PTR [rbx+0x18],rax
            m.w64(rbx.wrapping_add(0x18), (rax) as u64);
            // 32d10 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 32d14 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 32d18 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 32d1c mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 32d20 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 32d24 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 32d27 mov DWORD PTR [rbx+0x14],r14d
            m.w32(rbx.wrapping_add(0x14), ((r14 as u32 as u64)) as u32);
            // 32d2b mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // 32d2e call 0x5a935
            { let r = { let t = m.env.code_va(0x5a935); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32d33 mov DWORD PTR [rbx+0x38],eax
            m.w32(rbx.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32d36 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 32d3a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32d3b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 32d3d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 32d3f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 32d41 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 32d43 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32d44 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x436ce size=212
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000436ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x436ce;
    loop {
        match pc {
        0x436ce => {
            // block 436ce
            #[cfg(feature = "trace")] eprintln!("f_000436ce pc 436ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 436ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 436cf mov rbp,rsp
            rbp = rsp;
            // 436d2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 436d4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 436d6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 436d7 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 436db mov r14,rdx
            r14 = rdx;
            // 436de mov r15,rsi
            r15 = rsi;
            // 436e1 mov rbx,rdi
            rbx = rdi;
            // 436e4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 436e7 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 436ed mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 436ef call 0xce64c
            { let r = { let t = m.env.code_va(0xce64c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 436f4 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 436f7 jb 0x43757
            if fl.b() { pc = 0x43757; continue; }
            pc = 0x436f9;
        }
        0x436f9 => {
            // block 436f9
            #[cfg(feature = "trace")] eprintln!("f_000436ce pc 436f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 436f9 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 43700 mov DWORD PTR [rbp-0x20],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u32);
            // 43707 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4370a lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // 4370e mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // 43713 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // 43718 mov r8d,0x5
            r8 = (0x5u64) as u32 as u64;
            // 4371e mov rdi,rbx
            rdi = rbx;
            // 43721 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43724 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43726 je 0x43797
            if fl.e() { pc = 0x43797; continue; }
            pc = 0x43728;
        }
        0x43728 => {
            // block 43728
            #[cfg(feature = "trace")] eprintln!("f_000436ce pc 43728 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43728 cmp DWORD PTR [rbp-0x20],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64, 0x1u64).1;
            // 4372c sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 4372f movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 43732 mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            // 43735 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 43738 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 4373c mov esi,0x7f
            rsi = (0x7fu64) as u32 as u64;
            // 43741 mov ecx,0xf00000
            rcx = (0xf00000u64) as u32 as u64;
            // 43746 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 4374c mov rdi,rbx
            rdi = rbx;
            // 4374f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43752 mov ecx,DWORD PTR [rbp-0x1c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 43755 jmp 0x43794
            pc = 0x43794; continue;
        }
        0x43757 => {
            // block 43757
            #[cfg(feature = "trace")] eprintln!("f_000436ce pc 43757 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43757 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 4375e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 43761 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 43765 mov esi,0xbe
            rsi = (0xbeu64) as u32 as u64;
            // 4376a mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // 4376f mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // 43775 mov rdi,rbx
            rdi = rbx;
            // 43778 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4377b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4377d je 0x43797
            if fl.e() { pc = 0x43797; continue; }
            pc = 0x4377f;
        }
        0x4377f => {
            // block 4377f
            #[cfg(feature = "trace")] eprintln!("f_000436ce pc 4377f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4377f mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 43782 test cl,0x3
            fl = logic::<8>((rcx as u8 as u64) & 0x3u64);
            // 43785 setne dl
            rdx = (rdx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 43788 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // 4378b mov DWORD PTR [r15],edx
            m.w32(r15, ((rdx as u32 as u64)) as u32);
            // 4378e shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 43791 and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0x43794;
        }
        0x43794 => {
            // block 43794
            #[cfg(feature = "trace")] eprintln!("f_000436ce pc 43794 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43794 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            pc = 0x43797;
        }
        0x43797 => {
            // block 43797
            #[cfg(feature = "trace")] eprintln!("f_000436ce pc 43797 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43797 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4379b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4379c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4379e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 437a0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 437a1 ret
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

/// parity: P2v @0xb5e0e size=212
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5e0e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb5e0e;
    loop {
        match pc {
        0xb5e0e => {
            // block b5e0e
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e0e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5e0f mov rbp,rsp
            rbp = rsp;
            // b5e12 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5e14 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5e16 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5e18 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5e1a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5e1b sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b5e1f mov rbx,rcx
            rbx = rcx;
            // b5e22 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5e25 je 0xb5ecf
            if fl.e() { pc = 0xb5ecf; continue; }
            pc = 0xb5e2b;
        }
        0xb5e2b => {
            // block b5e2b
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e2b movzx r14d,BYTE PTR [rsi]
            r14 = (m.r8(rsi) as u64) as u32 as u64;
            // b5e2f mov r15b,r14b
            r15 = (r15 & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b5e32 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5e36 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b5e39 mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // b5e3d mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b5e41 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // b5e45 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b5e49 lea rax,[rsi+0x1]
            rax = rsi.wrapping_add(0x1);
            // b5e4d mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0xb5e51;
        }
        0xb5e51 => {
            // block b5e51
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e51 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // b5e55 mov rbx,rdx
            rbx = rdx;
            pc = 0xb5e58;
        }
        0xb5e58 => {
            // block b5e58
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e58 movzx r13d,BYTE PTR [rbx+0x20]
            r13 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b5e5d mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // b5e60 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5e62 jne 0xb5e69
            if fl.ne() { pc = 0xb5e69; continue; }
            pc = 0xb5e64;
        }
        0xb5e64 => {
            // block b5e64
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e64 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // b5e67 jmp 0xb5e6d
            pc = 0xb5e6d; continue;
        }
        0xb5e69 => {
            // block b5e69
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e69 mov r13,QWORD PTR [rbx+0x28]
            r13 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xb5e6d;
        }
        0xb5e6d => {
            // block b5e6d
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e6d test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b5e70 mov r12,QWORD PTR [rbp-0x30]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b5e74 cmove r12,r14
            if fl.e() { r12 = r14; }
            // b5e78 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b5e7a jne 0xb5e85
            if fl.ne() { pc = 0xb5e85; continue; }
            pc = 0xb5e7c;
        }
        0xb5e7c => {
            // block b5e7c
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e7c lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // b5e80 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // b5e83 jmp 0xb5e89
            pc = 0xb5e89; continue;
        }
        0xb5e85 => {
            // block b5e85
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e85 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xb5e89;
        }
        0xb5e89 => {
            // block b5e89
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e89 cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // b5e8c mov rdx,r13
            rdx = r13;
            // b5e8f cmovb rdx,r12
            if fl.b() { rdx = r12; }
            // b5e93 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5e96 je 0xb5eb1
            if fl.e() { pc = 0xb5eb1; continue; }
            pc = 0xb5e98;
        }
        0xb5e98 => {
            // block b5e98
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5e98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5e98 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b5e9b mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b5e9f cmove rsi,QWORD PTR [rbp-0x40]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64; }
            // b5ea4 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5ea9 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b5eab je 0xb5eb1
            if fl.e() { pc = 0xb5eb1; continue; }
            pc = 0xb5ead;
        }
        0xb5ead => {
            // block b5ead
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5ead rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ead js 0xb5eb6
            if fl.s() { pc = 0xb5eb6; continue; }
            pc = 0xb5eaf;
        }
        0xb5eaf => {
            // block b5eaf
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5eaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5eaf jmp 0xb5ec1
            pc = 0xb5ec1; continue;
        }
        0xb5eb1 => {
            // block b5eb1
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5eb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5eb1 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // b5eb4 jae 0xb5ec1
            if fl.ae() { pc = 0xb5ec1; continue; }
            pc = 0xb5eb6;
        }
        0xb5eb6 => {
            // block b5eb6
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5eb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5eb6 mov rbx,QWORD PTR [rbx+0x8]
            rbx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5eba test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5ebd jne 0xb5e58
            if fl.ne() { pc = 0xb5e58; continue; }
            pc = 0xb5ebf;
        }
        0xb5ebf => {
            // block b5ebf
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5ebf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ebf jmp 0xb5ecb
            pc = 0xb5ecb; continue;
        }
        0xb5ec1 => {
            // block b5ec1
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5ec1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ec1 mov rdx,QWORD PTR [rbx]
            rdx = m.r64(rbx) as u64;
            // b5ec4 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5ec7 jne 0xb5e51
            if fl.ne() { pc = 0xb5e51; continue; }
            pc = 0xb5ec9;
        }
        0xb5ec9 => {
            // block b5ec9
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5ec9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ec9 jmp 0xb5ecf
            pc = 0xb5ecf; continue;
        }
        0xb5ecb => {
            // block b5ecb
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5ecb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ecb mov rbx,QWORD PTR [rbp-0x48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xb5ecf;
        }
        0xb5ecf => {
            // block b5ecf
            #[cfg(feature = "trace")] eprintln!("f_000b5e0e pc b5ecf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ecf mov rax,rbx
            rax = rbx;
            // b5ed2 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b5ed6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5ed7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b5ed9 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b5edb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5edd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5edf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5ee0 ret
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

/// parity: P2v @0xc4a8e size=212
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4a8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4a8e;
    loop {
        match pc {
        0xc4a8e => {
            // block c4a8e
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4a8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4a8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4a8f mov rbp,rsp
            rbp = rsp;
            // c4a92 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c4a94 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4a96 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c4a98 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c4a9a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4a9b sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c4a9f mov rbx,rcx
            rbx = rcx;
            // c4aa2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4aa5 je 0xc4b4f
            if fl.e() { pc = 0xc4b4f; continue; }
            pc = 0xc4aab;
        }
        0xc4aab => {
            // block c4aab
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4aab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4aab movzx r14d,BYTE PTR [rsi]
            r14 = (m.r8(rsi) as u64) as u32 as u64;
            // c4aaf mov r15b,r14b
            r15 = (r15 & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // c4ab2 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c4ab6 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // c4ab9 mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c4abd mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // c4ac1 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // c4ac5 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // c4ac9 lea rax,[rsi+0x1]
            rax = rsi.wrapping_add(0x1);
            // c4acd mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0xc4ad1;
        }
        0xc4ad1 => {
            // block c4ad1
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4ad1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ad1 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // c4ad5 mov rbx,rdx
            rbx = rdx;
            pc = 0xc4ad8;
        }
        0xc4ad8 => {
            // block c4ad8
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4ad8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ad8 movzx r13d,BYTE PTR [rbx+0x20]
            r13 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // c4add mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // c4ae0 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c4ae2 jne 0xc4ae9
            if fl.ne() { pc = 0xc4ae9; continue; }
            pc = 0xc4ae4;
        }
        0xc4ae4 => {
            // block c4ae4
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4ae4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ae4 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // c4ae7 jmp 0xc4aed
            pc = 0xc4aed; continue;
        }
        0xc4ae9 => {
            // block c4ae9
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4ae9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4ae9 mov r13,QWORD PTR [rbx+0x28]
            r13 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xc4aed;
        }
        0xc4aed => {
            // block c4aed
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4aed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4aed test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c4af0 mov r12,QWORD PTR [rbp-0x30]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // c4af4 cmove r12,r14
            if fl.e() { r12 = r14; }
            // c4af8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c4afa jne 0xc4b05
            if fl.ne() { pc = 0xc4b05; continue; }
            pc = 0xc4afc;
        }
        0xc4afc => {
            // block c4afc
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4afc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4afc lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // c4b00 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // c4b03 jmp 0xc4b09
            pc = 0xc4b09; continue;
        }
        0xc4b05 => {
            // block c4b05
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b05 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xc4b09;
        }
        0xc4b09 => {
            // block c4b09
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b09 cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // c4b0c mov rdx,r13
            rdx = r13;
            // c4b0f cmovb rdx,r12
            if fl.b() { rdx = r12; }
            // c4b13 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4b16 je 0xc4b31
            if fl.e() { pc = 0xc4b31; continue; }
            pc = 0xc4b18;
        }
        0xc4b18 => {
            // block c4b18
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b18 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c4b1b mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c4b1f cmove rsi,QWORD PTR [rbp-0x40]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64; }
            // c4b24 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c4b29 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c4b2b je 0xc4b31
            if fl.e() { pc = 0xc4b31; continue; }
            pc = 0xc4b2d;
        }
        0xc4b2d => {
            // block c4b2d
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b2d js 0xc4b36
            if fl.s() { pc = 0xc4b36; continue; }
            pc = 0xc4b2f;
        }
        0xc4b2f => {
            // block c4b2f
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b2f jmp 0xc4b41
            pc = 0xc4b41; continue;
        }
        0xc4b31 => {
            // block c4b31
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b31 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // c4b34 jae 0xc4b41
            if fl.ae() { pc = 0xc4b41; continue; }
            pc = 0xc4b36;
        }
        0xc4b36 => {
            // block c4b36
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b36 mov rbx,QWORD PTR [rbx+0x8]
            rbx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4b3a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c4b3d jne 0xc4ad8
            if fl.ne() { pc = 0xc4ad8; continue; }
            pc = 0xc4b3f;
        }
        0xc4b3f => {
            // block c4b3f
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b3f jmp 0xc4b4b
            pc = 0xc4b4b; continue;
        }
        0xc4b41 => {
            // block c4b41
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b41 mov rdx,QWORD PTR [rbx]
            rdx = m.r64(rbx) as u64;
            // c4b44 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4b47 jne 0xc4ad1
            if fl.ne() { pc = 0xc4ad1; continue; }
            pc = 0xc4b49;
        }
        0xc4b49 => {
            // block c4b49
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b49 jmp 0xc4b4f
            pc = 0xc4b4f; continue;
        }
        0xc4b4b => {
            // block c4b4b
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b4b mov rbx,QWORD PTR [rbp-0x48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xc4b4f;
        }
        0xc4b4f => {
            // block c4b4f
            #[cfg(feature = "trace")] eprintln!("f_000c4a8e pc c4b4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4b4f mov rax,rbx
            rax = rbx;
            // c4b52 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c4b56 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4b57 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c4b59 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c4b5b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c4b5d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c4b5f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4b60 ret
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
            { let r = { let t = m.env.stub_va(0xa51); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0xa4b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x19664 size=215
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00019664<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x19664;
    loop {
        match pc {
        0x19664 => {
            // block 19664
            #[cfg(feature = "trace")] eprintln!("f_00019664 pc 19664 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19664 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19665 mov rbp,rsp
            rbp = rsp;
            // 19668 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1966a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1966c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1966e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 19670 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19671 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 19675 mov r14,rsi
            r14 = rsi;
            // 19678 mov r15d,edi
            r15 = ((rdi as u32 as u64)) as u32 as u64;
            // 1967b lea r13,[r14+0x8]
            r13 = r14.wrapping_add(0x8);
            // 1967f mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 19683 mov rdi,r14
            rdi = r14;
            // 19686 call 0xeb9d2
            { let r = { let t = m.env.stub_va(0x19f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1968b mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 19693 mov QWORD PTR [r14],r13
            m.w64(r14, (r13) as u64);
            // 19696 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 1969e mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 196a3 lea r12,[rbp-0x2c]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            pc = 0x196a7;
        }
        0x196a7 => {
            // block 196a7
            #[cfg(feature = "trace")] eprintln!("f_00019664 pc 196a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 196a7 mov DWORD PTR [rbp-0x2c],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rbx as u32 as u64)) as u32);
            // 196aa mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // 196ad mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 196af call 0xc9e94
            { let r = { let t = m.env.code_va(0xc9e94); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 196b4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 196b6 je 0x196c3
            if fl.e() { pc = 0x196c3; continue; }
            pc = 0x196b8;
        }
        0x196b8 => {
            // block 196b8
            #[cfg(feature = "trace")] eprintln!("f_00019664 pc 196b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 196b8 mov rdi,r14
            rdi = r14;
            // 196bb mov rsi,r12
            rsi = r12;
            // 196be call 0xeb9cc
            { let r = { let t = m.env.stub_va(0x199); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x196c3;
        }
        0x196c3 => {
            // block 196c3
            #[cfg(feature = "trace")] eprintln!("f_00019664 pc 196c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 196c3 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 196c5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 196c7 cmp ebx,0x20
            fl = sub::<32>((rbx as u32 as u64), 0x20u64).1;
            // 196ca jb 0x196a7
            if fl.b() { pc = 0x196a7; continue; }
            pc = 0x196cc;
        }
        0x196cc => {
            // block 196cc
            #[cfg(feature = "trace")] eprintln!("f_00019664 pc 196cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 196cc jmp 0x19724
            pc = 0x19724; continue;
        }
        0x196ce => {
            // block 196ce
            #[cfg(feature = "trace")] eprintln!("f_00019664 pc 196ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 196ce mov rbx,rax
            rbx = rax;
            // 196d1 cmp edx,0x1
            fl = sub::<32>((rdx as u32 as u64), 0x1u64).1;
            // 196d4 jne 0x19733
            if fl.ne() { pc = 0x19733; continue; }
            pc = 0x196d6;
        }
        0x196d6 => {
            // block 196d6
            #[cfg(feature = "trace")] eprintln!("f_00019664 pc 196d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 196d6 mov rdi,rbx
            rdi = rbx;
            // 196d9 call 0xec164
            { let r = { let t = m.env.stub_va(0x931); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 196de mov rax,QWORD PTR [rip+0x126ce3] # 0x1403c8
            rax = m.r64(m.env.data_va(347029)) as u64;
            // 196e5 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 196e9 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 196ed mov rdi,rbx
            rdi = rbx;
            // 196f0 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 196f5 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 196f9 mov rdi,r14
            rdi = r14;
            // 196fc call 0xeb9d2
            { let r = { let t = m.env.stub_va(0x19f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19701 mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 19709 mov QWORD PTR [r14],r13
            m.w64(r14, (r13) as u64);
            // 1970c mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 19714 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 19718 call 0xec128
            { let r = { let t = m.env.stub_va(0x8f5); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1971d call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19722 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x19724;
        }
        0x19724 => {
            // block 19724
            #[cfg(feature = "trace")] eprintln!("f_00019664 pc 19724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19724 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 19728 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19729 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1972b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1972d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1972f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 19731 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19732 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x19733 => {
            // block 19733
            #[cfg(feature = "trace")] eprintln!("f_00019664 pc 19733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19733 mov rdi,rbx
            rdi = rbx;
            // 19736 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
