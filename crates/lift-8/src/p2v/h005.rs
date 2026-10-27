//! Mechanically lifted functions (scripts/lift_run.py), module h005. Each one is checked against the original
//! in the world harness (tests/world_h005.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

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

/// parity: P2v @0xe59da size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e59da<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe59da;
    loop {
        match pc {
        0xe59da => {
            // block e59da
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e59da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e59da push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e59db mov rbp,rsp
            rbp = rsp;
            // e59de push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e59e0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e59e1 mov r11b,BYTE PTR [rbp+0x18]
            r11 = (r11 & !0xff) | ((m.r8(rbp.wrapping_add(0x18)) as u64) as u8 as u64);
            // e59e5 mov r10b,BYTE PTR [rbp+0x10]
            r10 = (r10 & !0xff) | ((m.r8(rbp.wrapping_add(0x10)) as u64) as u8 as u64);
            // e59e9 lea ebx,[r9-0x1]
            rbx = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // e59ed mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            // e59f2 cmp ebx,0x8
            fl = sub::<32>((rbx as u32 as u64), 0x8u64).1;
            // e59f5 ja 0xe5a04
            if fl.a() { pc = 0xe5a04; continue; }
            pc = 0xe59f7;
        }
        0xe59f7 => {
            // block e59f7
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e59f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e59f7 movsxd rax,ebx
            rax = sx::<32>((rbx as u32 as u64)) as u64;
            // e59fa lea rbx,[rip+0x442af] # 0x129cb0
            rbx = m.env.data_va(255101);
            // e5a01 mov eax,DWORD PTR [rbx+rax*4]
            rax = (m.r32(rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe5a04;
        }
        0xe5a04 => {
            // block e5a04
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a04 add r9d,0xfffffffd
            { let (r, f) = add::<32>((r9 as u32 as u64), 0xfffffffdu64);
              r9 = (r) as u32 as u64;
              fl = f; }
            // e5a08 cmp r9d,0x3
            fl = sub::<32>((r9 as u32 as u64), 0x3u64).1;
            // e5a0c ja 0xe5a38
            if fl.a() { pc = 0xe5a38; continue; }
            pc = 0xe5a0e;
        }
        0xe5a0e => {
            // block e5a0e
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a0e lea r14,[rip+0x4b] # 0xe5a60
            r14 = m.env.code_va(0xe5a60);
            // e5a15 movsxd rbx,DWORD PTR [r14+r9*4]
            rbx = sx::<32>(m.r32(r14.wrapping_add(r9.wrapping_mul(4))) as u64) as u64;
            // e5a19 add rbx,r14
            { let (r, f) = add::<64>(rbx, r14);
              rbx = r;
              fl = f; }
            // e5a1c jmp rbx
            pc = (rbx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xe5a1e => {
            // block e5a1e
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a1e test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // e5a21 je 0xe5a38
            if fl.e() { pc = 0xe5a38; continue; }
            pc = 0xe5a23;
        }
        0xe5a23 => {
            // block e5a23
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a23 jmp 0xe5a3d
            pc = 0xe5a3d; continue;
        }
        0xe5a25 => {
            // block e5a25
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a25 mov r9d,0x6
            r9 = (0x6u64) as u32 as u64;
            // e5a2b jmp 0xe5a33
            pc = 0xe5a33; continue;
        }
        0xe5a2d => {
            // block e5a2d
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a2d mov r9d,0x5
            r9 = (0x5u64) as u32 as u64;
            pc = 0xe5a33;
        }
        0xe5a33 => {
            // block e5a33
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a33 test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // e5a36 jne 0xe5a57
            if fl.ne() { pc = 0xe5a57; continue; }
            pc = 0xe5a38;
        }
        0xe5a38 => {
            // block e5a38
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a38 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // e5a3b jne 0xe5a47
            if fl.ne() { pc = 0xe5a47; continue; }
            pc = 0xe5a3d;
        }
        0xe5a3d => {
            // block e5a3d
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a3d movzx r9d,r10b
            r9 = ((r10 as u8 as u64)) as u32 as u64;
            // e5a41 add r9d,0x7
            { let (r, f) = add::<32>((r9 as u32 as u64), 0x7u64);
              r9 = (r) as u32 as u64;
              fl = f; }
            // e5a45 jmp 0xe5a57
            pc = 0xe5a57; continue;
        }
        0xe5a47 => {
            // block e5a47
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a47 cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e5a4a mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // e5a4d jne 0xe5a57
            if fl.ne() { pc = 0xe5a57; continue; }
            pc = 0xe5a4f;
        }
        0xe5a4f => {
            // block e5a4f
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a4f movzx r9d,r10b
            r9 = ((r10 as u8 as u64)) as u32 as u64;
            // e5a53 add r9d,0x3
            { let (r, f) = add::<32>((r9 as u32 as u64), 0x3u64);
              r9 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe5a57;
        }
        0xe5a57 => {
            // block e5a57
            #[cfg(feature = "trace")] eprintln!("f_000e59da pc e5a57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a57 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e5a58 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e5a5a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5a5b jmp 0xe4cd0
            { let r = { let t = m.env.code_va(0xe4cd0); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1729c size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001729c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1729c;
    loop {
        match pc {
        0x1729c => {
            // block 1729c
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 1729c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1729c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1729d mov rbp,rsp
            rbp = rsp;
            // 172a0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 172a2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 172a3 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 172a7 mov r14,rsi
            r14 = rsi;
            // 172aa mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // 172ac lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 172b0 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 172b5 call 0x16080
            { let r = { let t = m.env.code_va(0x16080); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 172ba mov rax,QWORD PTR [rbp-0x28]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 172be mov rcx,QWORD PTR [rbp-0x20]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 172c2 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 172c5 je 0x172d6
            if fl.e() { pc = 0x172d6; continue; }
            pc = 0x172c7;
        }
        0x172c7 => {
            // block 172c7
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172c7 cmp DWORD PTR [rax],ebx
            fl = sub::<32>(m.r32(rax) as u64, (rbx as u32 as u64)).1;
            // 172c9 je 0x172da
            if fl.e() { pc = 0x172da; continue; }
            pc = 0x172cb;
        }
        0x172cb => {
            // block 172cb
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172cb add rax,0xf8
            { let (r, f) = add::<64>(rax, 0xf8u64);
              rax = r;
              fl = f; }
            // 172d1 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 172d4 jne 0x172c7
            if fl.ne() { pc = 0x172c7; continue; }
            pc = 0x172d6;
        }
        0x172d6 => {
            // block 172d6
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172d6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 172d8 jmp 0x172f3
            pc = 0x172f3; continue;
        }
        0x172da => {
            // block 172da
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172da mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 172dd mov rbx,QWORD PTR [rcx+0x10]
            rbx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 172e1 movzx esi,WORD PTR [rax+0x4]
            rsi = (m.r16(rax.wrapping_add(0x4)) as u64) as u32 as u64;
            // 172e5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 172e7 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 172e9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 172ec mov rdi,r14
            rdi = r14;
            // 172ef call rbx
            { let r = { let t = rbx; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 172f1 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x172f3;
        }
        0x172f3 => {
            // block 172f3
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 172f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 172f3 lea rax,[rip+0x12a11e] # 0x141418
            rax = m.env.data_va(351205);
            // 172fa mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 172fe lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 17302 call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17307 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 17309 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 1730d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1730e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 17310 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17311 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x17312 => {
            // block 17312
            #[cfg(feature = "trace")] eprintln!("f_0001729c pc 17312 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17312 mov rbx,rax
            rbx = rax;
            // 17315 lea rax,[rip+0x12a0fc] # 0x141418
            rax = m.env.data_va(351205);
            // 1731c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 17320 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 17324 call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17329 mov rdi,rbx
            rdi = rbx;
            // 1732c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4b6de size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004b6de<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4b6de;
    loop {
        match pc {
        0x4b6de => {
            // block 4b6de
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b6de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b6de push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4b6df mov rbp,rsp
            rbp = rsp;
            // 4b6e2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 4b6e4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 4b6e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4b6e7 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 4b6eb mov rbx,rdi
            rbx = rdi;
            // 4b6ee mov r15,QWORD PTR [rip+0xf4ce3] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 4b6f5 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 4b6f8 mov QWORD PTR [rbp-0x20],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (r15) as u64);
            // 4b6fc mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 4b704 mov DWORD PTR [rbp-0x34],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0x1u64) as u32);
            // 4b70b movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // 4b70e mov QWORD PTR [rbp-0x28],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rax) as u64);
            // 4b712 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b715 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 4b718 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b71a call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b720 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 4b722 je 0x4b758
            if fl.e() { pc = 0x4b758; continue; }
            pc = 0x4b724;
        }
        0x4b724 => {
            // block 4b724
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b724 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4b727 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 4b729 mov rdi,rbx
            rdi = rbx;
            // 4b72c call QWORD PTR [rax+0x190]
            { let r = { let t = m.r64(rax.wrapping_add(0x190)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b732 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 4b736 lea r8,[rbp-0x30]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 4b73a lea r9,[rbp-0x34]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 4b73e mov esi,0x1a
            rsi = (0x1au64) as u32 as u64;
            // 4b743 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 4b748 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 4b74a call 0xebf66
            { let r = { let t = m.env.stub_va(0x733); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b74f cmp QWORD PTR [rbp-0x30],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x0u64).1;
            // 4b754 setne r14b
            r14 = (r14 & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x4b758;
        }
        0x4b758 => {
            // block 4b758
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b758 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b758 cmp r15,QWORD PTR [rbp-0x20]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // 4b75c jne 0x4b76c
            if fl.ne() { pc = 0x4b76c; continue; }
            pc = 0x4b75e;
        }
        0x4b75e => {
            // block 4b75e
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b75e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b75e mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 4b761 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 4b765 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4b766 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4b768 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4b76a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4b76b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x4b76c => {
            // block 4b76c
            #[cfg(feature = "trace")] eprintln!("f_0004b6de pc 4b76c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b76c call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f1f8 size=138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f1f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2f1f8;
    loop {
        match pc {
        0x2f1f8 => {
            // block 2f1f8
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f1f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f1f9 mov rbp,rsp
            rbp = rsp;
            // 2f1fc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f1fe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f1ff sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f203 mov rbx,rdi
            rbx = rdi;
            // 2f206 lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // 2f20a mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f20e mov rdi,r14
            rdi = r14;
            // 2f211 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f217 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2f219 je 0x2f279
            if fl.e() { pc = 0x2f279; continue; }
            pc = 0x2f21b;
        }
        0x2f21b => {
            // block 2f21b
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f21b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f21b mov rdi,r14
            rdi = r14;
            // 2f21e call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f223 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2f226 cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 2f22b jne 0x2f279
            if fl.ne() { pc = 0x2f279; continue; }
            pc = 0x2f22d;
        }
        0x2f22d => {
            // block 2f22d
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f22d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f22d mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f231 mov esi,DWORD PTR [rbx+0x2b4]
            rsi = (m.r32(rbx.wrapping_add(0x2b4)) as u64) as u32 as u64;
            // 2f237 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2f239 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2f23e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f241 mov rdi,r14
            rdi = r14;
            // 2f244 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f247 mov rdi,rbx
            rdi = rbx;
            // 2f24a call 0x31164
            { let r = { let t = m.env.code_va(0x31164); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f24f mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // 2f253 lea rsi,[rbp-0x11]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffef);
            // 2f257 mov rdi,rbx
            rdi = rbx;
            // 2f25a call 0x311bc
            { let r = { let t = m.env.code_va(0x311bc); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f25f movzx esi,BYTE PTR [rbp-0x11]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u32 as u64;
            // 2f263 or esi,0x80
            { let r = (rsi as u32 as u64) | 0x80u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2f269 mov rdi,rbx
            rdi = rbx;
            // 2f26c add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f270 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f271 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f273 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f274 jmp 0x31368
            { let r = { let t = m.env.code_va(0x31368); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
        0x2f279 => {
            // block 2f279
            #[cfg(feature = "trace")] eprintln!("f_0002f1f8 pc 2f279 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f279 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2f27d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f27e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f280 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f281 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64996 size=129
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064996<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x64996;
    loop {
        match pc {
        0x64996 => {
            // block 64996
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 64996 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64996 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64997 mov rbp,rsp
            rbp = rsp;
            // 6499a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6499c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6499e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 649a0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 649a2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 649a3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 649a4 mov r14,rsi
            r14 = rsi;
            // 649a7 mov r12,rdi
            r12 = rdi;
            // 649aa mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // 649ad add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 649b1 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 649b4 je 0x64a05
            if fl.e() { pc = 0x64a05; continue; }
            pc = 0x649b6;
        }
        0x649b6 => {
            // block 649b6
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649b6 lea r15,[rip+0xc5ae0] # 0x12a49d
            r15 = m.env.data_va(257130);
            // 649bd jmp 0x649d2
            pc = 0x649d2; continue;
        }
        0x649bf => {
            // block 649bf
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649bf mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 649c4 mov rdi,r12
            rdi = r12;
            // 649c7 mov rsi,r15
            rsi = r15;
            // 649ca call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 649cf mov r13,rbx
            r13 = rbx;
            pc = 0x649d2;
        }
        0x649d2 => {
            // block 649d2
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649d2 lea rsi,[r13+0x20]
            rsi = r13.wrapping_add(0x20);
            // 649d6 mov rdi,r12
            rdi = r12;
            // 649d9 call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 649de mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 649e2 test rax,rax
            fl = logic::<64>(rax & rax);
            // 649e5 je 0x649f4
            if fl.e() { pc = 0x649f4; continue; }
            pc = 0x649e7;
        }
        0x649e7 => {
            // block 649e7
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649e7 mov rbx,rax
            rbx = rax;
            // 649ea mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 649ed test rax,rax
            fl = logic::<64>(rax & rax);
            // 649f0 jne 0x649e7
            if fl.ne() { pc = 0x649e7; continue; }
            pc = 0x649f2;
        }
        0x649f2 => {
            // block 649f2
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649f2 jmp 0x64a00
            pc = 0x64a00; continue;
        }
        0x649f4 => {
            // block 649f4
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 649f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 649f4 mov rbx,QWORD PTR [r13+0x10]
            rbx = m.r64(r13.wrapping_add(0x10)) as u64;
            // 649f8 cmp QWORD PTR [rbx],r13
            fl = sub::<64>(m.r64(rbx) as u64, r13).1;
            // 649fb mov r13,rbx
            r13 = rbx;
            // 649fe jne 0x649f4
            if fl.ne() { pc = 0x649f4; continue; }
            pc = 0x64a00;
        }
        0x64a00 => {
            // block 64a00
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 64a00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a00 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 64a03 jne 0x649bf
            if fl.ne() { pc = 0x649bf; continue; }
            pc = 0x64a05;
        }
        0x64a05 => {
            // block 64a05
            #[cfg(feature = "trace")] eprintln!("f_00064996 pc 64a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64a05 mov rax,r12
            rax = r12;
            // 64a08 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 64a0c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64a0d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 64a0f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 64a11 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 64a13 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 64a15 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 64a16 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bb44 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bb44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2bb44;
    loop {
        match pc {
        0x2bb44 => {
            // block 2bb44
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bb45 mov rbp,rsp
            rbp = rsp;
            // 2bb48 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bb4a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bb4c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2bb4e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bb4f mov r12,rsi
            r12 = rsi;
            // 2bb52 mov r14,rdi
            r14 = rdi;
            // 2bb55 lea rbx,[r12+0x8]
            rbx = r12.wrapping_add(0x8);
            // 2bb5a mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2bb5f mov rdi,r12
            rdi = r12;
            // 2bb62 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bb67 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 2bb70 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 2bb74 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 2bb7d mov r15,QWORD PTR [r14+0x50]
            r15 = m.r64(r14.wrapping_add(0x50)) as u64;
            // 2bb81 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2bb84 je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb86;
        }
        0x2bb86 => {
            // block 2bb86
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb86 cmp DWORD PTR [r14+0x58],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x58)) as u64, 0x0u64).1;
            // 2bb8b je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb8d;
        }
        0x2bb8d => {
            // block 2bb8d
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb8d mov eax,DWORD PTR [r14+0x4c]
            rax = (m.r32(r14.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 2bb91 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2bb93 je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb95;
        }
        0x2bb95 => {
            // block 2bb95
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb95 cmp eax,DWORD PTR [r14+0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x20)) as u64).1;
            // 2bb99 jbe 0x2bba6
            if fl.be() { pc = 0x2bba6; continue; }
            pc = 0x2bb9b;
        }
        0x2bb9b => {
            // block 2bb9b
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb9b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bb9d;
        }
        0x2bb9d => {
            // block 2bb9d
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb9d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bb9e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2bba0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bba2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2bba4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bba5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2bba6 => {
            // block 2bba6
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bba6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bba6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bba8;
        }
        0x2bba8 => {
            // block 2bba8
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bba8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bba8 mov esi,DWORD PTR [r15+rbx*4]
            rsi = (m.r32(r15.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2bbac mov rdi,r12
            rdi = r12;
            // 2bbaf call 0x60955
            { let r = { let t = m.env.code_va(0x60955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bbb4 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2bbb7 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2bbb9 cmp ebx,DWORD PTR [r14+0x58]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x58)) as u64).1;
            // 2bbbd jb 0x2bba8
            if fl.b() { pc = 0x2bba8; continue; }
            pc = 0x2bbbf;
        }
        0x2bbbf => {
            // block 2bbbf
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bbbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bbbf jmp 0x2bb9d
            pc = 0x2bb9d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe8294 size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e8294<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe8294;
    loop {
        match pc {
        0xe8294 => {
            // block e8294
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e8294 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8294 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e8295 mov rbp,rsp
            rbp = rsp;
            // e8298 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e829a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e829c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e829d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e829e mov rbx,rsi
            rbx = rsi;
            // e82a1 mov r14,rdi
            r14 = rdi;
            // e82a4 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82a7 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82aa lea r15,[r14+0x8]
            r15 = r14.wrapping_add(0x8);
            // e82ae test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // e82b1 jne 0xe82b8
            if fl.ne() { pc = 0xe82b8; continue; }
            pc = 0xe82b3;
        }
        0xe82b3 => {
            // block e82b3
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82b3 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e82b6 jmp 0xe82bc
            pc = 0xe82bc; continue;
        }
        0xe82b8 => {
            // block e82b8
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82b8 mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xe82bc;
        }
        0xe82bc => {
            // block e82bc
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82bc mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // e82c1 mov rdi,r15
            rdi = r15;
            // e82c4 mov rsi,rbx
            rsi = rbx;
            // e82c7 call 0xeb918
            { let r = { let t = m.env.stub_va(0xe5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82cc mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e82cf mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e82d3 test BYTE PTR [rax+r15*1+0x20],0x5
            fl = logic::<8>(m.r8(rax.wrapping_add(r15).wrapping_add(0x20)) as u64 & 0x5u64);
            // e82d9 je 0xe82df
            if fl.e() { pc = 0xe82df; continue; }
            pc = 0xe82db;
        }
        0xe82db => {
            // block e82db
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82db xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e82dd jmp 0xe8304
            pc = 0xe8304; continue;
        }
        0xe82df => {
            // block e82df
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e82df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e82df mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82e2 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // e82e7 mov rdi,r14
            rdi = r14;
            // e82ea call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82ed mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82f0 mov rdi,r14
            rdi = r14;
            // e82f3 call QWORD PTR [rax+0xa0]
            { let r = { let t = m.r64(rax.wrapping_add(0xa0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e82f9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e82fc mov rdi,r14
            rdi = r14;
            // e82ff call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8302 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe8304;
        }
        0xe8304 => {
            // block e8304
            #[cfg(feature = "trace")] eprintln!("f_000e8294 pc e8304 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8304 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e8308 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e8309 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e830b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e830d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e830e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe494e size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e494e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe494e;
    loop {
        match pc {
        0xe494e => {
            // block e494e
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e494e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e494e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e494f mov rbp,rsp
            rbp = rsp;
            // e4952 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4954 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4956 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4957 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e4958 mov r15,rdi
            r15 = rdi;
            // e495b mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e495e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e4961 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e4965 mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // e496a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e496f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4972 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4975 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e4978 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // e497b test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e497d je 0xe49b6
            if fl.e() { pc = 0xe49b6; continue; }
            pc = 0xe497f;
        }
        0xe497f => {
            // block e497f
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e497f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e497f mov ebx,0xffffffec
            rbx = (0xffffffecu64) as u32 as u64;
            pc = 0xe4984;
        }
        0xe4984 => {
            // block e4984
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e4984 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4984 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e4987 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e498a call QWORD PTR [rax+0x1880]
            { let r = { let t = m.r64(rax.wrapping_add(0x1880)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4990 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e4992 jne 0xe49b6
            if fl.ne() { pc = 0xe49b6; continue; }
            pc = 0xe4994;
        }
        0xe4994 => {
            // block e4994
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e4994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4994 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e4997 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e499a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e499c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // e49a1 call QWORD PTR [rax+0xbd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e49a7 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // e49a9 jne 0xe4984
            if fl.ne() { pc = 0xe4984; continue; }
            pc = 0xe49ab;
        }
        0xe49ab => {
            // block e49ab
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e49ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e49ab mov DWORD PTR [r15+0x8],0x16
            m.w32(r15.wrapping_add(0x8), (0x16u64) as u32);
            // e49b3 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe49b6;
        }
        0xe49b6 => {
            // block e49b6
            #[cfg(feature = "trace")] eprintln!("f_000e494e pc e49b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e49b6 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // e49b9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e49bd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e49be pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e49c0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e49c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e49c3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25d00 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025d00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x25d00;
    loop {
        match pc {
        0x25d00 => {
            // block 25d00
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25d01 mov rbp,rsp
            rbp = rsp;
            // 25d04 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 25d08 test rax,rax
            fl = logic::<64>(rax & rax);
            // 25d0b je 0x25d3e
            if fl.e() { pc = 0x25d3e; continue; }
            pc = 0x25d0d;
        }
        0x25d0d => {
            // block 25d0d
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d0d mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 25d0f jmp 0x25d14
            pc = 0x25d14; continue;
        }
        0x25d11 => {
            // block 25d11
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d11 mov rax,rdx
            rax = rdx;
            pc = 0x25d14;
        }
        0x25d14 => {
            // block 25d14
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d14 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 25d17 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 25d19 jge 0x25d25
            if fl.ge() { pc = 0x25d25; continue; }
            pc = 0x25d1b;
        }
        0x25d1b => {
            // block 25d1b
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d1b mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 25d1e test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 25d21 jne 0x25d11
            if fl.ne() { pc = 0x25d11; continue; }
            pc = 0x25d23;
        }
        0x25d23 => {
            // block 25d23
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d23 jmp 0x25d4a
            pc = 0x25d4a; continue;
        }
        0x25d25 => {
            // block 25d25
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d25 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 25d27 jge 0x25d52
            if fl.ge() { pc = 0x25d52; continue; }
            pc = 0x25d29;
        }
        0x25d29 => {
            // block 25d29
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d29 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 25d2d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 25d30 jne 0x25d11
            if fl.ne() { pc = 0x25d11; continue; }
            pc = 0x25d32;
        }
        0x25d32 => {
            // block 25d32
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d32 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 25d35 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 25d39 mov rsi,rax
            rsi = rax;
            // 25d3c jmp 0x25d55
            pc = 0x25d55; continue;
        }
        0x25d3e => {
            // block 25d3e
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d3e add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 25d42 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 25d45 mov rsi,rdi
            rsi = rdi;
            // 25d48 jmp 0x25d55
            pc = 0x25d55; continue;
        }
        0x25d4a => {
            // block 25d4a
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d4a mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 25d4d mov rsi,rax
            rsi = rax;
            // 25d50 jmp 0x25d55
            pc = 0x25d55; continue;
        }
        0x25d52 => {
            // block 25d52
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d52 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x25d55;
        }
        0x25d55 => {
            // block 25d55
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d55 mov rax,rsi
            rax = rsi;
            // 25d58 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25d59 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3cdc size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003cdc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x3cdc;
    loop {
        match pc {
        0x3cdc => {
            // block 3cdc
            #[cfg(feature = "trace")] eprintln!("f_00003cdc pc 3cdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3cdc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3cdd mov rbp,rsp
            rbp = rsp;
            // 3ce0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3ce2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3ce3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 3ce7 mov rbx,rdi
            rbx = rdi;
            // 3cea mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3ced lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 3cf1 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // 3cf6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3cfb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3cfe call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d01 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 3d08 lea r14,[rbp-0x18]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            pc = 0x3d0c;
        }
        0x3d0c => {
            // block 3d0c
            #[cfg(feature = "trace")] eprintln!("f_00003cdc pc 3d0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d0c mov DWORD PTR [rbp-0x18],0x100
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0x100u64) as u32);
            // 3d13 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3d16 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3d1b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3d20 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3d23 mov rdi,rbx
            rdi = rbx;
            // 3d26 mov rdx,r14
            rdx = r14;
            // 3d29 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d2c test BYTE PTR [rbp-0x17],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe9)) as u64 & 0x1u64);
            // 3d30 jne 0x3d0c
            if fl.ne() { pc = 0x3d0c; continue; }
            pc = 0x3d32;
        }
        0x3d32 => {
            // block 3d32
            #[cfg(feature = "trace")] eprintln!("f_00003cdc pc 3d32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d32 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 3d34 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 3d38 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3d39 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3d3b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3d3c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2db52 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002db52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2db52;
    loop {
        match pc {
        0x2db52 => {
            // block 2db52
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2db53 mov rbp,rsp
            rbp = rsp;
            // 2db56 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2db5a test rax,rax
            fl = logic::<64>(rax & rax);
            // 2db5d je 0x2db90
            if fl.e() { pc = 0x2db90; continue; }
            pc = 0x2db5f;
        }
        0x2db5f => {
            // block 2db5f
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db5f mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 2db61 jmp 0x2db66
            pc = 0x2db66; continue;
        }
        0x2db63 => {
            // block 2db63
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db63 mov rax,rdx
            rax = rdx;
            pc = 0x2db66;
        }
        0x2db66 => {
            // block 2db66
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db66 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 2db69 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 2db6b jge 0x2db77
            if fl.ge() { pc = 0x2db77; continue; }
            pc = 0x2db6d;
        }
        0x2db6d => {
            // block 2db6d
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db6d mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 2db70 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2db73 jne 0x2db63
            if fl.ne() { pc = 0x2db63; continue; }
            pc = 0x2db75;
        }
        0x2db75 => {
            // block 2db75
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db75 jmp 0x2db9c
            pc = 0x2db9c; continue;
        }
        0x2db77 => {
            // block 2db77
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db77 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 2db79 jge 0x2dba4
            if fl.ge() { pc = 0x2dba4; continue; }
            pc = 0x2db7b;
        }
        0x2db7b => {
            // block 2db7b
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db7b mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2db7f test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2db82 jne 0x2db63
            if fl.ne() { pc = 0x2db63; continue; }
            pc = 0x2db84;
        }
        0x2db84 => {
            // block 2db84
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db84 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2db87 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 2db8b mov rsi,rax
            rsi = rax;
            // 2db8e jmp 0x2dba7
            pc = 0x2dba7; continue;
        }
        0x2db90 => {
            // block 2db90
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db90 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 2db94 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 2db97 mov rsi,rdi
            rsi = rdi;
            // 2db9a jmp 0x2dba7
            pc = 0x2dba7; continue;
        }
        0x2db9c => {
            // block 2db9c
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db9c mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2db9f mov rsi,rax
            rsi = rax;
            // 2dba2 jmp 0x2dba7
            pc = 0x2dba7; continue;
        }
        0x2dba4 => {
            // block 2dba4
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2dba4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dba4 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x2dba7;
        }
        0x2dba7 => {
            // block 2dba7
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2dba7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dba7 mov rax,rsi
            rax = rsi;
            // 2dbaa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dbab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4dac size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4dac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc4dac;
    loop {
        match pc {
        0xc4dac => {
            // block c4dac
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4dad mov rbp,rsp
            rbp = rsp;
            // c4db0 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c4db4 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4db7 je 0xc4dea
            if fl.e() { pc = 0xc4dea; continue; }
            pc = 0xc4db9;
        }
        0xc4db9 => {
            // block c4db9
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4db9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4db9 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c4dbb jmp 0xc4dc0
            pc = 0xc4dc0; continue;
        }
        0xc4dbd => {
            // block c4dbd
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dbd mov rax,rdx
            rax = rdx;
            pc = 0xc4dc0;
        }
        0xc4dc0 => {
            // block c4dc0
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dc0 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c4dc3 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c4dc5 jae 0xc4dd1
            if fl.ae() { pc = 0xc4dd1; continue; }
            pc = 0xc4dc7;
        }
        0xc4dc7 => {
            // block c4dc7
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dc7 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c4dca test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4dcd jne 0xc4dbd
            if fl.ne() { pc = 0xc4dbd; continue; }
            pc = 0xc4dcf;
        }
        0xc4dcf => {
            // block c4dcf
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dcf jmp 0xc4df6
            pc = 0xc4df6; continue;
        }
        0xc4dd1 => {
            // block c4dd1
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dd1 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c4dd3 jae 0xc4dfe
            if fl.ae() { pc = 0xc4dfe; continue; }
            pc = 0xc4dd5;
        }
        0xc4dd5 => {
            // block c4dd5
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dd5 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // c4dd9 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4ddc jne 0xc4dbd
            if fl.ne() { pc = 0xc4dbd; continue; }
            pc = 0xc4dde;
        }
        0xc4dde => {
            // block c4dde
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dde mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c4de1 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c4de5 mov rsi,rax
            rsi = rax;
            // c4de8 jmp 0xc4e01
            pc = 0xc4e01; continue;
        }
        0xc4dea => {
            // block c4dea
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dea add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c4dee mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c4df1 mov rsi,rdi
            rsi = rdi;
            // c4df4 jmp 0xc4e01
            pc = 0xc4e01; continue;
        }
        0xc4df6 => {
            // block c4df6
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4df6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4df6 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c4df9 mov rsi,rax
            rsi = rax;
            // c4dfc jmp 0xc4e01
            pc = 0xc4e01; continue;
        }
        0xc4dfe => {
            // block c4dfe
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4dfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4dfe mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc4e01;
        }
        0xc4e01 => {
            // block c4e01
            #[cfg(feature = "trace")] eprintln!("f_000c4dac pc c4e01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4e01 mov rax,rsi
            rax = rsi;
            // c4e04 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4e05 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd49a8 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d49a8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd49a8;
    loop {
        match pc {
        0xd49a8 => {
            // block d49a8
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49a8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d49a9 mov rbp,rsp
            rbp = rsp;
            // d49ac mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // d49b0 test rax,rax
            fl = logic::<64>(rax & rax);
            // d49b3 je 0xd49e6
            if fl.e() { pc = 0xd49e6; continue; }
            pc = 0xd49b5;
        }
        0xd49b5 => {
            // block d49b5
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49b5 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // d49b7 jmp 0xd49bc
            pc = 0xd49bc; continue;
        }
        0xd49b9 => {
            // block d49b9
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49b9 mov rax,rdx
            rax = rdx;
            pc = 0xd49bc;
        }
        0xd49bc => {
            // block d49bc
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49bc mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // d49bf cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // d49c1 jge 0xd49cd
            if fl.ge() { pc = 0xd49cd; continue; }
            pc = 0xd49c3;
        }
        0xd49c3 => {
            // block d49c3
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49c3 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // d49c6 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // d49c9 jne 0xd49b9
            if fl.ne() { pc = 0xd49b9; continue; }
            pc = 0xd49cb;
        }
        0xd49cb => {
            // block d49cb
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49cb jmp 0xd49f2
            pc = 0xd49f2; continue;
        }
        0xd49cd => {
            // block d49cd
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49cd cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // d49cf jge 0xd49fa
            if fl.ge() { pc = 0xd49fa; continue; }
            pc = 0xd49d1;
        }
        0xd49d1 => {
            // block d49d1
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49d1 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // d49d5 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // d49d8 jne 0xd49b9
            if fl.ne() { pc = 0xd49b9; continue; }
            pc = 0xd49da;
        }
        0xd49da => {
            // block d49da
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49da mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // d49dd add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // d49e1 mov rsi,rax
            rsi = rax;
            // d49e4 jmp 0xd49fd
            pc = 0xd49fd; continue;
        }
        0xd49e6 => {
            // block d49e6
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49e6 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // d49ea mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // d49ed mov rsi,rdi
            rsi = rdi;
            // d49f0 jmp 0xd49fd
            pc = 0xd49fd; continue;
        }
        0xd49f2 => {
            // block d49f2
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49f2 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // d49f5 mov rsi,rax
            rsi = rax;
            // d49f8 jmp 0xd49fd
            pc = 0xd49fd; continue;
        }
        0xd49fa => {
            // block d49fa
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49fa mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xd49fd;
        }
        0xd49fd => {
            // block d49fd
            #[cfg(feature = "trace")] eprintln!("f_000d49a8 pc d49fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d49fd mov rax,rsi
            rax = rsi;
            // d4a00 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d4a01 ret
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

/// parity: P2v @0xe0a36 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0a36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0a36;
    loop {
        match pc {
        0xe0a36 => {
            // block e0a36
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0a37 mov rbp,rsp
            rbp = rsp;
            // e0a3a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0a3c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0a3d mov rbx,rdi
            rbx = rdi;
            // e0a40 mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            // e0a43 test r14,r14
            fl = logic::<64>(r14 & r14);
            // e0a46 je 0xe0a77
            if fl.e() { pc = 0xe0a77; continue; }
            pc = 0xe0a48;
        }
        0xe0a48 => {
            // block e0a48
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a48 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e0a4c cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // e0a4f je 0xe0a6b
            if fl.e() { pc = 0xe0a6b; continue; }
            pc = 0xe0a51;
        }
        0xe0a51 => {
            // block e0a51
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a51 lea rdi,[rax-0x38]
            rdi = rax.wrapping_add(0xffff_ffff_ffff_ffc8);
            // e0a55 mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // e0a59 mov rax,QWORD PTR [rax-0x38]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e0a5d call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a5f mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e0a63 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // e0a66 jne 0xe0a51
            if fl.ne() { pc = 0xe0a51; continue; }
            pc = 0xe0a68;
        }
        0xe0a68 => {
            // block e0a68
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a68 mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            pc = 0xe0a6b;
        }
        0xe0a6b => {
            // block e0a6b
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a6b mov rdi,r14
            rdi = r14;
            // e0a6e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0a6f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0a71 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0a72 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xe0a77 => {
            // block e0a77
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a77 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0a78 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0a7a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0a7b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4b10 size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004b10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x4b10;
    loop {
        match pc {
        0x4b10 => {
            // block 4b10
            #[cfg(feature = "trace")] eprintln!("f_00004b10 pc 4b10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4b11 mov rbp,rsp
            rbp = rsp;
            // 4b14 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4b18 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // 4b1f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 4b22 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // 4b26 call QWORD PTR [rax+0x1e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1e0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4b2c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4b2e je 0x4b4b
            if fl.e() { pc = 0x4b4b; continue; }
            pc = 0x4b30;
        }
        0x4b30 => {
            // block 4b30
            #[cfg(feature = "trace")] eprintln!("f_00004b10 pc 4b30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b30 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4b32 mov ecx,DWORD PTR [rbp-0x4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 4b35 lea edx,[rcx-0xeb0c]
            rdx = (rcx.wrapping_add(0xffff_ffff_ffff_14f4)) as u32 as u64;
            // 4b3b cmp edx,0x3
            fl = sub::<32>((rdx as u32 as u64), 0x3u64).1;
            // 4b3e jb 0x4b4d
            if fl.b() { pc = 0x4b4d; continue; }
            pc = 0x4b40;
        }
        0x4b40 => {
            // block 4b40
            #[cfg(feature = "trace")] eprintln!("f_00004b10 pc 4b40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b40 add ecx,0xffff24f9
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xffff24f9u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4b46 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 4b49 jb 0x4b4d
            if fl.b() { pc = 0x4b4d; continue; }
            pc = 0x4b4b;
        }
        0x4b4b => {
            // block 4b4b
            #[cfg(feature = "trace")] eprintln!("f_00004b10 pc 4b4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b4b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4b4d;
        }
        0x4b4d => {
            // block 4b4d
            #[cfg(feature = "trace")] eprintln!("f_00004b10 pc 4b4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b4d add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4b51 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4b52 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc0bc size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc0bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xbc0bc;
    loop {
        match pc {
        0xbc0bc => {
            // block bc0bc
            #[cfg(feature = "trace")] eprintln!("f_000bc0bc pc bc0bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc0bd mov rbp,rsp
            rbp = rsp;
            // bc0c0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc0c1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bc0c2 mov rbx,rsi
            rbx = rsi;
            // bc0c5 mov DWORD PTR [rbp-0xc],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rdi as u32 as u64)) as u32);
            // bc0c8 lea rdi,[rip+0x10c559] # 0x1c8628
            rdi = m.env.data_va(904693);
            // bc0cf lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // bc0d3 call 0xebdf2
            { let r = { let t = m.env.stub_va(0x5bf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc0d8 mov ecx,0x6c
            rcx = (0x6cu64) as u32 as u64;
            // bc0dd lea rdx,[rip+0x10c54c] # 0x1c8630
            rdx = m.env.data_va(904701);
            // bc0e4 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // bc0e7 je 0xbc0ec
            if fl.e() { pc = 0xbc0ec; continue; }
            pc = 0xbc0e9;
        }
        0xbc0e9 => {
            // block bc0e9
            #[cfg(feature = "trace")] eprintln!("f_000bc0bc pc bc0e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0e9 mov ecx,DWORD PTR [rax+0x20]
            rcx = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            pc = 0xbc0ec;
        }
        0xbc0ec => {
            // block bc0ec
            #[cfg(feature = "trace")] eprintln!("f_000bc0bc pc bc0ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0ec mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            // bc0ee cmp ecx,0x6c
            fl = sub::<32>((rcx as u32 as u64), 0x6cu64).1;
            // bc0f1 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc0f4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bc0f8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc0f9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc0fa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
